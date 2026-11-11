//! Sound: DirectSound buffers and CD music, mixed in software and sent to SDL3.
//! Time decides how much to mix, so buffers finish with or without a device.

use crate::cpu::{Cpu, GuestFn};
use crate::host::{ddraw, table};
use crate::sys;
use lewton::inside_ogg::OggStreamReader;
use std::collections::{HashMap, VecDeque};
use std::ffi::{c_int, c_void};
use std::time::Instant;

const DS_OK: u32 = 0;
const MIX_RATE: f64 = 44_100.0;
const MAX_MIX_SECONDS: f64 = 0.25;
const FORMAT_F32: u32 = 0x8120;
const PLAYBACK_DEVICE: u32 = 0xffff_ffff;
const INIT_AUDIO: u32 = 0x10;
const MUSIC_LEVEL: f32 = 0.7;
const MIN_VOLUME: f64 = -10_000.0;
const STATUS_PLAYING: u32 = 1;
const STATUS_LOOPING: u32 = 4;
const PLAY_LOOPING: u32 = 1;
const BUFFER_PRIMARY: u32 = 1;
const MCI_OPEN: u32 = 0x803;
const MCI_PLAY: u32 = 0x806;
const MCI_STOP: u32 = 0x808;
const MCI_STATUS: u32 = 0x814;
const MCI_DEVICE: u32 = 1;
const MM_MCINOTIFY: u32 = 0x3b9;
const MCI_NOTIFY_SUCCESSFUL: u32 = 1;

type Pointer = *mut c_void;

#[derive(Clone, Copy, Default)]
struct Buffer {
    data: u32,
    size: u32,
    channels: u32,
    bits: u32,
    frequency: f64,
    volume: f64,
    pan: f64,
    playing: bool,
    looping: bool,
    position: f64,
}

struct Music {
    reader: OggStreamReader<std::fs::File>,
    samples: VecDeque<f32>,
    channels: usize,
    rate: f64,
    fraction: f64,
    track: u32,
    last_track: u32,
}

pub struct Audio {
    buffers: HashMap<u32, Buffer>,
    device_vtable: u32,
    buffer_vtable: u32,
    stream: Pointer,
    put_data: Option<unsafe extern "C" fn(Pointer, *const c_void, c_int) -> bool>,
    last_mix: Instant,
    music: Option<Music>,
    /// The loudest sample since the last `audio` script step, for headless checks.
    pub peak: f32,
    pub peak_music: f32,
    /// The CD line's volume in the mixer, from 0 to `MIXER_MAXIMUM`, which
    /// scales the music as it did the CD's output.
    music_volume: u32,
}

impl Default for Audio {
    fn default() -> Audio {
        Audio {
            buffers: HashMap::new(),
            device_vtable: 0,
            buffer_vtable: 0,
            stream: std::ptr::null_mut(),
            put_data: None,
            last_mix: Instant::now(),
            music: None,
            peak: 0.0,
            peak_music: 0.0,
            // A game that never sets it plays its music as loud as the CD was.
            music_volume: MIXER_MAXIMUM,
        }
    }
}

macro_rules! method {
    ($offset:expr, $arguments:expr, |$cpu:ident| $body:block) => {
        ($offset, $arguments, (|$cpu: &mut Cpu| { let result: u32 = $body; $cpu.finish(result, $arguments); }) as GuestFn)
    };
}

fn buffer_mut<'a>(cpu: &'a mut Cpu, at: u32) -> &'a mut Buffer {
    cpu.host.audio.buffers.entry(at).or_default()
}

fn open_stream(audio: &mut Audio) {
    if std::env::var_os("RECOMP_HEADLESS").is_some() {
        return;
    }
    unsafe {
        let library = sys::open_sdl();
        if library.is_null() {
            return;
        }
        let find = |name: &str| sys::symbol(library, name);
        let init: unsafe extern "C" fn(u32) -> bool = std::mem::transmute(find("SDL_InitSubSystem"));
        if !init(INIT_AUDIO) {
            return;
        }
        let open: unsafe extern "C" fn(u32, *const [c_int; 3], Pointer, Pointer) -> Pointer = std::mem::transmute(find("SDL_OpenAudioDeviceStream"));
        let resume: unsafe extern "C" fn(Pointer) -> bool = std::mem::transmute(find("SDL_ResumeAudioStreamDevice"));
        let spec = [FORMAT_F32 as c_int, 2, MIX_RATE as c_int];
        let stream = open(PLAYBACK_DEVICE, &spec, std::ptr::null_mut(), std::ptr::null_mut());
        if !stream.is_null() {
            resume(stream);
            audio.stream = stream;
            audio.put_data = Some(std::mem::transmute(find("SDL_PutAudioStreamData")));
        }
    }
}

fn create_buffer(cpu: &mut Cpu) -> u32 {
    let (description, out) = (cpu.arg(1), cpu.arg(2));
    let flags = cpu.r32(description + 4);
    let size = cpu.r32(description + 8);
    let format = cpu.r32(description + 16);
    let vtable = cpu.host.audio.buffer_vtable;
    let at = ddraw::object(cpu, vtable);
    let (channels, rate, bits) = if format != 0 {
        (cpu.r16(format + 2) as u32, cpu.r32(format + 4), cpu.r16(format + 14) as u32)
    } else {
        (2, 22_050, 16)
    };
    let data = if flags & BUFFER_PRIMARY != 0 { 0 } else { cpu.allocate(size.max(1)) };
    if data != 0 {
        cpu.fill(data, size, 0);
    }
    *buffer_mut(cpu, at) = Buffer { data, size, channels, bits, frequency: rate as f64, ..Buffer::default() };
    cpu.w32(out, at);
    DS_OK
}

fn lock(cpu: &mut Cpu) -> u32 {
    let (this, offset, wanted, first, first_size, second, second_size) =
        (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5), cpu.arg(6));
    let buffer = *buffer_mut(cpu, this);
    let wanted = wanted.min(buffer.size);
    let offset = offset % buffer.size.max(1);
    let run = wanted.min(buffer.size - offset);
    cpu.w32(first, buffer.data + offset);
    cpu.w32(first_size, run);
    if second != 0 {
        cpu.w32(second, if wanted > run { buffer.data } else { 0 });
    }
    if second_size != 0 {
        cpu.w32(second_size, wanted - run);
    }
    DS_OK
}

fn build_tables(cpu: &mut Cpu) {
    if cpu.host.audio.device_vtable != 0 {
        return;
    }
    let common: [ddraw::Method; 2] = [method!(0x04, 1, |_cpu| { 1 }), method!(0x08, 1, |_cpu| { 0 })];
    let mut device = common.to_vec();
    device.extend([
        method!(0x0c, 4, |cpu| { create_buffer(cpu) }),
        method!(0x18, 3, |_cpu| { DS_OK }),
    ]);
    let mut buffer = common.to_vec();
    buffer.extend([
        method!(0x10, 3, |cpu| {
            let (this, play, write) = (cpu.arg(0), cpu.arg(1), cpu.arg(2));
            let at = buffer_mut(cpu, this).position as u32;
            if play != 0 { cpu.w32(play, at); }
            if write != 0 { cpu.w32(write, at); }
            DS_OK
        }),
        method!(0x18, 2, |cpu| { let (this, at) = (cpu.arg(0), cpu.arg(1)); let value = buffer_mut(cpu, this).volume as i32 as u32; cpu.w32(at, value); DS_OK }),
        method!(0x1c, 2, |cpu| { let (this, at) = (cpu.arg(0), cpu.arg(1)); let value = buffer_mut(cpu, this).pan as i32 as u32; cpu.w32(at, value); DS_OK }),
        method!(0x20, 2, |cpu| { let (this, at) = (cpu.arg(0), cpu.arg(1)); let value = buffer_mut(cpu, this).frequency as u32; cpu.w32(at, value); DS_OK }),
        method!(0x24, 2, |cpu| {
            let (this, at) = (cpu.arg(0), cpu.arg(1));
            let buffer = *buffer_mut(cpu, this);
            let status = if buffer.playing { STATUS_PLAYING | if buffer.looping { STATUS_LOOPING } else { 0 } } else { 0 };
            cpu.w32(at, status);
            DS_OK
        }),
        method!(0x2c, 8, |cpu| { lock(cpu) }),
        method!(0x30, 4, |cpu| {
            let (this, flags) = (cpu.arg(0), cpu.arg(3));
            let buffer = buffer_mut(cpu, this);
            buffer.playing = true;
            buffer.looping = flags & PLAY_LOOPING != 0;
            DS_OK
        }),
        method!(0x34, 2, |cpu| { let (this, at) = (cpu.arg(0), cpu.arg(1)); buffer_mut(cpu, this).position = at as f64; DS_OK }),
        method!(0x3c, 2, |cpu| { let (this, value) = (cpu.arg(0), cpu.arg(1) as i32); buffer_mut(cpu, this).volume = value as f64; DS_OK }),
        method!(0x40, 2, |cpu| { let (this, value) = (cpu.arg(0), cpu.arg(1) as i32); buffer_mut(cpu, this).pan = value as f64; DS_OK }),
        method!(0x44, 2, |cpu| { let (this, value) = (cpu.arg(0), cpu.arg(1)); if value != 0 { buffer_mut(cpu, this).frequency = value as f64; } DS_OK }),
        method!(0x48, 1, |cpu| { let this = cpu.arg(0); buffer_mut(cpu, this).playing = false; DS_OK }),
        method!(0x4c, 5, |_cpu| { DS_OK }),
        method!(0x50, 1, |_cpu| { DS_OK }),
    ]);
    cpu.host.audio.device_vtable = ddraw::build_vtable(cpu, "IDirectSound", 11, &device);
    cpu.host.audio.buffer_vtable = ddraw::build_vtable(cpu, "IDirectSoundBuffer", 21, &buffer);
    open_stream(&mut cpu.host.audio);
}

fn sample_at(cpu: &Cpu, buffer: &Buffer, frame: u32, channel: u32) -> f32 {
    let channel = channel.min(buffer.channels.max(1) - 1);
    let width = buffer.bits / 8;
    let at = buffer.data + (frame * buffer.channels + channel) * width;
    if at + width > buffer.data + buffer.size {
        return 0.0;
    }
    if width == 1 {
        (cpu.r8(at) as f32 - 128.0) / 128.0
    } else {
        cpu.r16(at) as i16 as f32 / 32768.0
    }
}

/// Adds one playing buffer to the mix, and moves its position on.
fn mix_buffer(cpu: &Cpu, buffer: &mut Buffer, mix: &mut [f32], peak: &mut f32) {
    let gain = 10f64.powf(buffer.volume.max(MIN_VOLUME) / 2000.0) as f32;
    // Panning quietens one side only.
    let left_gain = gain * 10f64.powf(-buffer.pan.max(0.0) / 2000.0) as f32;
    let right_gain = gain * 10f64.powf(buffer.pan.min(0.0) / 2000.0) as f32;
    let frame_size = (buffer.channels * buffer.bits / 8).max(1) as f64;
    let step = buffer.frequency / MIX_RATE;
    let frames = buffer.size as f64 / frame_size;
    let mut frame = buffer.position / frame_size;
    for output in mix.chunks_exact_mut(2) {
        if frame >= frames {
            if !buffer.looping {
                buffer.playing = false;
                frame = 0.0;
                break;
            }
            frame -= frames;
        }
        let (left, right) = (sample_at(cpu, buffer, frame as u32, 0), sample_at(cpu, buffer, frame as u32, 1));
        output[0] += left * left_gain;
        output[1] += right * right_gain;
        *peak = peak.max(left.abs());
        frame += step;
    }
    buffer.position = frame * frame_size;
}

fn mix_music(music: &mut Music, volume: u32, mix: &mut [f32], peak: &mut f32) -> bool {
    let step = music.rate / MIX_RATE;
    let level = MUSIC_LEVEL * music_gain(volume);
    for output in mix.chunks_exact_mut(2) {
        while music.samples.len() < music.channels * 2 {
            match music.reader.read_dec_packet_itl() {
                Ok(Some(packet)) => music.samples.extend(packet.iter().map(|&s| s as f32 / 32768.0)),
                _ => return false,
            }
        }
        let left = music.samples[0];
        let right = music.samples[music.channels - 1];
        output[0] += left * level;
        output[1] += right * level;
        *peak = peak.max((left * level / MUSIC_LEVEL).abs());
        music.fraction += step;
        while music.fraction >= 1.0 {
            music.fraction -= 1.0;
            for _ in 0..music.channels {
                music.samples.pop_front();
            }
        }
    }
    true
}

/// Mixes what time has passed, and sends it to the device if there is one.
pub fn pump(cpu: &mut Cpu) {
    let now = Instant::now();
    let seconds = now.duration_since(cpu.host.audio.last_mix).as_secs_f64().min(MAX_MIX_SECONDS);
    let frames = (seconds * MIX_RATE) as usize;
    if frames == 0 {
        return;
    }
    cpu.host.audio.last_mix = now;
    let mut mix = vec![0f32; frames * 2];
    let mut peak = 0f32;
    let keys: Vec<u32> = cpu.host.audio.buffers.iter().filter(|(_, b)| b.playing && b.data != 0).map(|(k, _)| *k).collect();
    for key in keys {
        let mut buffer = cpu.host.audio.buffers[&key];
        mix_buffer(cpu, &mut buffer, &mut mix, &mut peak);
        cpu.host.audio.buffers.insert(key, buffer);
    }
    cpu.host.audio.peak = cpu.host.audio.peak.max(peak);
    let mut music_peak = 0f32;
    if let Some(mut music) = cpu.host.audio.music.take() {
        if mix_music(&mut music, cpu.host.audio.music_volume, &mut mix, &mut music_peak) {
            cpu.host.audio.music = Some(music);
        } else if music.track + 1 < music.last_track {
            let (next, last) = (music.track + 1, music.last_track);
            start_track(cpu, next, last);
        } else {
            cpu.host.messages.push_back((MM_MCINOTIFY, MCI_NOTIFY_SUCCESSFUL, MCI_DEVICE));
        }
    }
    cpu.host.audio.peak_music = cpu.host.audio.peak_music.max(music_peak);
    if let Some(put) = cpu.host.audio.put_data {
        for sample in &mut mix {
            *sample = sample.clamp(-1.0, 1.0);
        }
        unsafe { put(cpu.host.audio.stream, mix.as_ptr() as *const c_void, (mix.len() * 4) as c_int) };
    }
}

/// The prefix for CD tracks now, from the game's `recompMusic`: the first whose
/// condition holds, or else the one without a condition. Incubation's cutscenes
/// use `music/Track`, whose files hold their speech and sound, and the rest of
/// the game `music/Arack`, as GOG's own CD shim does.
fn music_prefix(cpu: &Cpu) -> Option<&'static str> {
    let music = cpu.host.music;
    let matching = music.iter().find(|(_, address, value)| *address != 0 && cpu.r32(*address) == *value);
    matching.or_else(|| music.iter().find(|(_, address, _)| *address == 0)).map(|(prefix, _, _)| *prefix)
}

/// The prefix's folder and the start of its file names.
fn music_place(cpu: &Cpu, prefix: &str) -> (std::path::PathBuf, String) {
    let (folder, start) = prefix.rsplit_once('/').unwrap_or(("", prefix));
    (cpu.host.folder.join(folder), start.to_lowercase())
}

/// The highest track number any prefix has a file for: the "disc's" track count.
fn track_count(cpu: &Cpu) -> u32 {
    let mut highest = 0;
    for (prefix, _, _) in cpu.host.music {
        let (folder, start) = music_place(cpu, prefix);
        for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            let number = name.strip_prefix(&start).and_then(|rest| rest.strip_suffix(".ogg")).and_then(|digits| digits.parse::<u32>().ok());
            highest = highest.max(number.unwrap_or(0));
        }
    }
    highest
}

/// CD track N is `<prefix>NN.ogg` in the game's folder.
fn start_track(cpu: &mut Cpu, track: u32, last_track: u32) {
    let file = music_prefix(cpu).and_then(|prefix| {
        let (folder, start) = music_place(cpu, prefix);
        let name = format!("{start}{track:02}.ogg");
        if std::env::var_os("RECOMP_TRACE").is_some() {
            println!("  CD track {track} from {prefix}");
        }
        let entries = std::fs::read_dir(&folder).ok()?;
        entries.flatten().find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(&name)).map(|e| e.path())
    });
    cpu.host.audio.music = file.and_then(|path| {
        let reader = OggStreamReader::new(std::fs::File::open(path).ok()?).ok()?;
        let (channels, rate) = (reader.ident_hdr.audio_channels as usize, reader.ident_hdr.audio_sample_rate as f64);
        Some(Music { reader, samples: VecDeque::new(), channels, rate, fraction: 0.0, track, last_track })
    });
    if cpu.host.audio.music.is_none() {
        cpu.host.messages.push_back((MM_MCINOTIFY, MCI_NOTIFY_SUCCESSFUL, MCI_DEVICE));
    }
}

fn send_command(cpu: &mut Cpu) -> u32 {
    const STATUS_ITEM_MODE: u32 = 4;
    const STATUS_NUMBER_OF_TRACKS: u32 = 3;
    const STATUS_ITEM_FLAG: u32 = 0x100;
    const MODE_STOP: u32 = 526;
    let (command, flags, parameters) = (cpu.arg(1), cpu.arg(2), cpu.arg(3));
    match command {
        MCI_OPEN => cpu.w32(parameters + 4, MCI_DEVICE),
        MCI_PLAY => {
            let (first, last) = (cpu.r32(parameters + 4), cpu.r32(parameters + 8));
            if std::env::var_os("RECOMP_TRACE").is_some() {
                println!("  CD play {first} to {last}");
            }
            start_track(cpu, first, last.max(first + 1));
        }
        MCI_STOP => cpu.host.audio.music = None,
        MCI_STATUS if flags & STATUS_ITEM_FLAG != 0 => {
            let item = cpu.r32(parameters + 8);
            let value = match item {
                STATUS_NUMBER_OF_TRACKS => track_count(cpu),
                STATUS_ITEM_MODE => MODE_STOP,
                _ => 0,
            };
            cpu.w32(parameters + 4, value);
        }
        _ => {}
    }
    0
}

pub fn find_dsound(name: &str) -> Option<GuestFn> {
    table! { name;
        "DirectSoundCreate" 3 |cpu| {
            build_tables(cpu);
            let vtable = cpu.host.audio.device_vtable;
            let at = ddraw::object(cpu, vtable);
            let out = cpu.arg(1);
            cpu.w32(out, at);
            DS_OK
        }
    }
}

/// The mixer's volume, as Windows' volume control applied it: in proportion.
pub(crate) fn music_gain(volume: u32) -> f32 {
    volume.min(MIXER_MAXIMUM) as f32 / MIXER_MAXIMUM as f32
}

const MIXER_MAXIMUM: u32 = 0xffff;
const MIXER_HANDLE: u32 = 0x4d58;
const MIXERR_INVALLINE: u32 = 1024;
const MIXERR_INVALCONTROL: u32 = 1025;
const MMSYSERR_INVALPARAM: u32 = 11;
const COMPONENTTYPE_SRC_COMPACTDISC: u32 = 0x1005;
const CONTROLTYPE_VOLUME: u32 = 0x5003_0001;
const GETLINEINFOF_COMPONENTTYPE: u32 = 3;
const GETLINECONTROLSF_ONEBYTYPE: u32 = 2;
const CD_LINE: u32 = 0x0001_0000;
const CD_VOLUME: u32 = 1;

/// One mixer with a CD audio line, whose volume control sets how loud the
/// music plays. Games such as Incubation set their music volume there, since
/// CD audio went from the drive to the sound card by cable, not through the
/// program. Other lines don't exist, and the game falls back or gives up.
fn mixer_line_info(cpu: &mut Cpu) -> u32 {
    let (line, flags) = (cpu.arg(1), cpu.arg(2));
    if flags & 0xf != GETLINEINFOF_COMPONENTTYPE || cpu.r32(line + 24) != COMPONENTTYPE_SRC_COMPACTDISC {
        return MIXERR_INVALLINE;
    }
    cpu.w32(line + 12, CD_LINE);
    cpu.w32(line + 28, 2); // Channels.
    cpu.w32(line + 32, 0); // Connections.
    cpu.w32(line + 36, 1); // Controls.
    0
}

fn mixer_line_controls(cpu: &mut Cpu) -> u32 {
    const CONTROL_SIZE: u32 = 0x94;
    let (controls, flags) = (cpu.arg(1), cpu.arg(2));
    let wanted = cpu.r32(controls + 8);
    if flags & 0xf != GETLINECONTROLSF_ONEBYTYPE || cpu.r32(controls + 4) != CD_LINE || wanted != CONTROLTYPE_VOLUME {
        return MIXERR_INVALCONTROL;
    }
    if cpu.r32(controls + 16) < CONTROL_SIZE {
        return MMSYSERR_INVALPARAM;
    }
    let control = cpu.r32(controls + 20);
    for offset in (0..CONTROL_SIZE).step_by(4) {
        cpu.w32(control + offset, 0);
    }
    cpu.w32(control, CONTROL_SIZE);
    cpu.w32(control + 4, CD_VOLUME);
    cpu.w32(control + 8, CONTROLTYPE_VOLUME);
    for (offset, text) in [(20u32, &b"CD Volume"[..]), (36, &b"CD Audio Volume"[..])] {
        for (index, byte) in text.iter().enumerate() {
            cpu.w8(control + offset + index as u32, *byte);
        }
    }
    cpu.w32(control + 104, MIXER_MAXIMUM); // Bounds: 0 to the maximum.
    cpu.w32(control + 124, MIXER_MAXIMUM); // Steps.
    0
}

/// Reads or writes the CD volume, the same for every channel asked for.
fn mixer_control_details(cpu: &mut Cpu, set: bool) -> u32 {
    let details = cpu.arg(1);
    if cpu.r32(details + 4) != CD_VOLUME {
        return MIXERR_INVALCONTROL;
    }
    let (channels, values) = (cpu.r32(details + 8).max(1), cpu.r32(details + 20));
    if set {
        let volume = cpu.r32(values).min(MIXER_MAXIMUM);
        cpu.host.audio.music_volume = volume;
        println!("  music volume {volume}");
    } else {
        let volume = cpu.host.audio.music_volume;
        for channel in 0..channels {
            cpu.w32(values + channel * 4, volume);
        }
    }
    0
}

pub fn find_mci(name: &str) -> Option<GuestFn> {
    table! { name;
        "mciSendCommandA" 4 |cpu| { send_command(cpu) }
        "mixerGetNumDevs" 0 |_cpu| { 1 }
        "mixerOpen" 5 |cpu| {
            let (out, device) = (cpu.arg(0), cpu.arg(1));
            if device != 0 { MMSYSERR_INVALPARAM } else { cpu.w32(out, MIXER_HANDLE); 0 }
        }
        "mixerClose" 1 |_cpu| { 0 }
        "mixerGetLineInfoA" 3 |cpu| { mixer_line_info(cpu) }
        "mixerGetLineControlsA" 3 |cpu| { mixer_line_controls(cpu) }
        "mixerGetControlDetailsA" 3 |cpu| { mixer_control_details(cpu, false) }
        "mixerSetControlDetails" 3 |cpu| { mixer_control_details(cpu, true) }
    }
}

#[cfg(test)]
mod tests {
    use super::music_gain;

    #[test]
    fn mixer_volume_scales_music_in_proportion() {
        assert_eq!(music_gain(0), 0.0);
        assert_eq!(music_gain(0xffff), 1.0);
        assert_eq!(music_gain(0x1_0000), 1.0);
        assert!((music_gain(25000) - 0.381).abs() < 0.001);
    }
}
