//! Scripted input and screenshots, so a game can be driven with no display.
//!
//! `RECOMP_SCRIPT` holds steps separated by spaces: `wait:SECONDS`, `shot:NAME`
//! (written to `RECOMP_OUT`), `move:X,Y`, `click:X,Y`, `key:CODE` (a virtual
//! key), `type:TEXT` (characters, as typed), `fps` (the
//! frame rate since the last `fps`), `stick:X,Y` (the left stick, from -1 to 1),
//! `pad:south`, `pad:east` or `pad:start` (a controller button), `trigger:rt` or
//! `trigger:lt` (a trigger pressed and released), `peek:ADDRESS` (prints the
//! 32-bit value there, in hex), `poke:ADDRESS,VALUE` (writes one, both in hex), `answer:yes` or `answer:no` (how questions a
//! mod asks are answered, since there's no window to click) and `quit`.
//! With a display, as under `RECOMP_DISPLAY=1`, `stats` shows or hides the
//! frame counter, and `window:NAME` saves what the window shows, with the
//! counter or a mod's question over the picture, where `shot` saves the
//! game's frame alone.

use crate::cpu::Cpu;
use crate::host::{ddraw, gamepad};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const WM_MOUSEMOVE: u32 = 0x200;
const WM_LBUTTONDOWN: u32 = 0x201;
const WM_LBUTTONUP: u32 = 0x202;
const WM_KEYDOWN: u32 = 0x100;
const WM_KEYUP: u32 = 0x101;
const WM_CHAR: u32 = 0x102;
const MK_LBUTTON: u32 = 1;

pub struct Script {
    pub steps: VecDeque<String>,
    pub next_step: Instant,
    pub output: String,
    pub fps_mark: (u64, Instant),
    /// What `host::ask` answers with no window: yes unless `answer:no` said otherwise.
    pub answer: bool,
    /// Whether the run has a script at all.
    pub scripted: bool,
}

impl Script {
    pub fn from_environment() -> Script {
        let text = std::env::var("RECOMP_SCRIPT").unwrap_or_default();
        Script {
            steps: text.split_whitespace().map(str::to_string).collect(),
            next_step: Instant::now(),
            output: std::env::var("RECOMP_OUT").unwrap_or_else(|_| ".".to_string()),
            fps_mark: (0, Instant::now()),
            answer: true,
            scripted: !text.trim().is_empty(),
        }
    }
}

fn pair(value: &str) -> (u32, u32) {
    let (x, y) = value.split_once(',').unwrap_or((value, "0"));
    (x.parse().unwrap_or(0), y.parse().unwrap_or(0))
}

/// How many different colours an RGB picture has, counting up to 256.
pub(crate) fn colours(pixels: &[u8]) -> usize {
    let mut seen = std::collections::HashSet::new();
    for pixel in pixels.chunks_exact(3) {
        seen.insert((pixel[0], pixel[1], pixel[2]));
        if seen.len() >= 256 {
            break;
        }
    }
    seen.len()
}

thread_local! {
    /// Every `shot` of the run by name, for `changed:` to compare with.
    static SHOTS: std::cell::RefCell<std::collections::HashMap<String, (u32, u32, Vec<u8>)>> = Default::default();
}

fn remember(name: &str, width: u32, height: u32, pixels: &[u8]) {
    SHOTS.with(|shots| shots.borrow_mut().insert(name.to_string(), (width, height, pixels.to_vec())));
}

/// The share of pixels, from 0 to 1, that differ between two RGB pictures.
/// Pictures of different sizes differ entirely.
pub(crate) fn changed_share(a: (u32, u32, &[u8]), b: (u32, u32, &[u8])) -> f64 {
    if a.0 != b.0 || a.1 != b.1 || a.2.len() != b.2.len() || a.2.is_empty() {
        return 1.0;
    }
    let differing = a.2.chunks_exact(3).zip(b.2.chunks_exact(3)).filter(|(x, y)| x != y).count();
    differing as f64 / (a.2.len() / 3) as f64
}

#[cfg(test)]
mod tests {
    #[test]
    fn changed_share_counts_differing_pixels() {
        let a = [0u8; 12];
        let mut b = a;
        b[3] = 9;
        assert_eq!(super::changed_share((2, 2, &a), (2, 2, &a)), 0.0);
        assert_eq!(super::changed_share((2, 2, &a), (2, 2, &b)), 0.25);
        assert_eq!(super::changed_share((2, 2, &a), (4, 1, &b)), 1.0);
    }

    #[test]
    fn colours_counts_distinct_pixels() {
        assert_eq!(super::colours(&[0; 30]), 1);
        assert_eq!(super::colours(&[0, 0, 0, 1, 0, 0, 0, 0, 0]), 2);
        let many: Vec<u8> = (0..1000u32).flat_map(|i| [i as u8, (i >> 8) as u8, 0]).collect();
        assert_eq!(super::colours(&many), 256);
    }
}

/// Writes a PNG with stored blocks: it needs no compression library.
pub(crate) fn write_png(path: &str, width: u32, height: u32, pixels: &[u8]) {
    fn crc(data: &[u8]) -> u32 {
        let mut value = 0xffff_ffffu32;
        for &byte in data {
            value ^= byte as u32;
            for _ in 0..8 {
                value = if value & 1 != 0 { (value >> 1) ^ 0xedb8_8320 } else { value >> 1 };
            }
        }
        !value
    }
    fn chunk(file: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
        file.extend((data.len() as u32).to_be_bytes());
        let start = file.len();
        file.extend(kind);
        file.extend(data);
        let check = crc(&file[start..]);
        file.extend(check.to_be_bytes());
    }
    let mut raw = Vec::new();
    for row in pixels.chunks((width * 3) as usize) {
        raw.push(0);
        raw.extend(row);
    }
    let mut stream = vec![0x78, 1];
    let blocks: Vec<&[u8]> = raw.chunks(65535).collect();
    for (index, block) in blocks.iter().enumerate() {
        stream.push(u8::from(index + 1 == blocks.len()));
        stream.extend((block.len() as u16).to_le_bytes());
        stream.extend((!(block.len() as u16)).to_le_bytes());
        stream.extend(*block);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &byte in &raw {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    stream.extend(((b << 16) | a).to_be_bytes());
    let mut file = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut header = Vec::new();
    header.extend(width.to_be_bytes());
    header.extend(height.to_be_bytes());
    header.extend([8, 2, 0, 0, 0]);
    chunk(&mut file, b"IHDR", &header);
    chunk(&mut file, b"IDAT", &stream);
    chunk(&mut file, b"IEND", &[]);
    if let Err(error) = std::fs::write(path, file) {
        println!("couldn't write {path}: {error}");
    }
}

/// Runs every step that's due. Called each time the game looks for messages.
pub fn advance(cpu: &mut Cpu) {
    while Instant::now() >= cpu.host.script.next_step {
        let Some(step) = cpu.host.script.steps.pop_front() else { return };
        let (name, value) = step.split_once(':').unwrap_or((&step, ""));
        match name {
            // Taken before the game starts: see `launcher::snapshot`.
            "launcher" => {}
            "wait" => {
                let seconds: f64 = value.parse().unwrap_or(0.0);
                cpu.host.script.next_step = Instant::now() + Duration::from_secs_f64(seconds);
            }
            "shot" => match ddraw::current_frame(cpu) {
                Some((width, height, pixels)) => {
                    let path = format!("{}/{value}.png", cpu.host.script.output);
                    write_png(&path, width, height, &pixels);
                    // Checks fail a black or flat frame by this count.
                    println!("frame {} saved as {value}.png, {} colours", cpu.host.draw.frames, colours(&pixels));
                    remember(value, width, height, &pixels);
                }
                None => println!("no frame yet for {value}"),
            },
            "window" => match cpu.host.video.as_mut().and_then(|video| video.window_picture()) {
                Some((width, height, pixels)) => {
                    let path = format!("{}/{value}.png", cpu.host.script.output);
                    write_png(&path, width, height, &pixels);
                    println!("window saved as {value}.png");
                }
                None => println!("no window drawn with OpenGL for {value}"),
            },
            "stats" => crate::host::video::toggle_stats(cpu),
            "move" | "click" => {
                let (x, y) = pair(value);
                cpu.host.pointer = (x, y);
                let position = x | (y << 16);
                cpu.host.post((WM_MOUSEMOVE, 0, position));
                if name == "click" {
                    cpu.host.post((WM_LBUTTONDOWN, MK_LBUTTON, position));
                    cpu.host.post((WM_LBUTTONUP, 0, position));
                }
            }
            "key" => {
                let code = value.parse().unwrap_or(0);
                cpu.host.post((WM_KEYDOWN, code, 0));
                // As the keyboard does: these keys are characters as well.
                if matches!(code, 8 | 9 | 13 | 27) {
                    cpu.host.post((WM_CHAR, code, 0));
                }
                cpu.host.post((WM_KEYUP, code, 0));
            }
            "type" => {
                for character in value.chars().filter(char::is_ascii) {
                    let code = character.to_ascii_uppercase() as u32;
                    cpu.host.post((WM_KEYDOWN, code, 0));
                    cpu.host.post((WM_CHAR, character as u32, 0));
                    cpu.host.post((WM_KEYUP, code, 0));
                }
            }
            "stick" => {
                let (x, y) = value.split_once(',').unwrap_or((value, "0"));
                gamepad::set_axis(cpu, 0, x.parse().unwrap_or(0.0));
                gamepad::set_axis(cpu, 1, y.parse().unwrap_or(0.0));
            }
            "peek" => {
                let at = u32::from_str_radix(value, 16).unwrap_or(0);
                println!("peek {at:x} = {:#x}", cpu.r32(at));
            }
            // `expect:ADDRESS,VALUE[,MASK]`, all in hex: ends the run with an
            // error when the 32-bit value there, masked, differs, so a check
            // can prove the game reached the level or loaded the save it should.
            "expect" => {
                let mut parts = value.split(',').map(|part| u32::from_str_radix(part, 16).unwrap_or(0));
                let (at, wanted, mask) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0), parts.next().unwrap_or(u32::MAX));
                let found = cpu.r32(at) & mask;
                if found != wanted & mask {
                    println!("expect failed: {at:x} holds {found:#x}, not {wanted:#x}");
                    std::process::exit(3);
                }
                println!("expect {at:x} = {found:#x}: ok");
            }
            // `changed:NAME,PERCENT`: ends the run with an error unless at
            // least PERCENT of the pixels differ from the `shot:NAME` taken
            // earlier, so an animated menu doesn't pass for a new level.
            "changed" => {
                let (name, percent) = value.split_once(',').unwrap_or((value, "20"));
                let percent: f64 = percent.parse().unwrap_or(20.0);
                let earlier = SHOTS.with(|shots| shots.borrow().get(name).cloned());
                let share = match (earlier, ddraw::current_frame(cpu)) {
                    (Some((w, h, old)), Some((width, height, now))) => changed_share((w, h, &old), (width, height, &now)) * 100.0,
                    _ => 0.0,
                };
                if share < percent {
                    println!("expect failed: only {share:.1}% of the screen changed from {name}, not {percent}%");
                    std::process::exit(3);
                }
                println!("changed {share:.1}% from {name}: ok");
            }
            // Writes a 32-bit value, both in hex, such as a level number, so a
            // check can start where the game would otherwise take hours to reach.
            "poke" => {
                let (at, wanted) = value.split_once(',').unwrap_or((value, "0"));
                let (at, wanted) = (u32::from_str_radix(at, 16).unwrap_or(0), u32::from_str_radix(wanted, 16).unwrap_or(0));
                cpu.w32(at, wanted);
                println!("poke {at:x} = {wanted:#x}");
            }
            "trigger" => {
                let axis = if value == "lt" { gamepad::AXIS_LEFT_TRIGGER } else { gamepad::AXIS_RIGHT_TRIGGER };
                println!("trigger {value} at {:?}", cpu.host.pointer);
                gamepad::set_axis(cpu, axis, 1.0);
                gamepad::set_axis(cpu, axis, 0.0);
            }
            "pad" => {
                let button = match value {
                    "east" => gamepad::BUTTON_EAST,
                    "start" => gamepad::BUTTON_START,
                    _ => gamepad::BUTTON_SOUTH,
                };
                println!("pad {value} at {:?}", cpu.host.pointer);
                gamepad::set_button(cpu, button, true);
                gamepad::set_button(cpu, button, false);
            }
            "f9" => {
                cpu.host.f9_presses += 1;
            }
            "audio" => {
                println!("sound peak {:.3}, music peak {:.3}", cpu.host.audio.peak, cpu.host.audio.peak_music);
                cpu.host.audio.peak = 0.0;
                cpu.host.audio.peak_music = 0.0;
            }
            "fps" => {
                let frames = cpu.host.draw.frames + cpu.host.glide.as_ref().map_or(0, |glide| glide.frames);
                let (last_frames, last_time) = std::mem::replace(&mut cpu.host.script.fps_mark, (frames, Instant::now()));
                let seconds = last_time.elapsed().as_secs_f64();
                println!("{:.1} frames a second", (frames - last_frames) as f64 / seconds);
                if let Some(glide) = cpu.host.glide.as_mut() {
                    let [drawing, flat, locks] = std::mem::take(&mut glide.spent);
                    println!("{:.0}% of the time drawing, {:.0}% on 2D, {:.1} 2D locks a frame", drawing / seconds * 100.0, flat / seconds * 100.0, locks / (frames - last_frames).max(1) as f64);
                    println!("longest frame {:.0} ms", std::mem::take(&mut glide.longest_frame) * 1000.0);
                    if glide.gpu.is_some() {
                        let counts = std::mem::take(&mut glide.counts);
                        let per_frame = |value: f64| value / (frames - last_frames).max(1) as f64;
                        println!(
                            "a frame on the card: {:.0} draws ({:.0} with the same settings as the one before), {:.0} triangles, {:.1} textures sent, {:.0} KB sent, {:.2} reads",
                            per_frame(counts.draws as f64),
                            per_frame(counts.same_state as f64),
                            per_frame(counts.triangles as f64),
                            per_frame(counts.texture_uploads as f64),
                            per_frame(counts.bytes as f64 / 1024.0),
                            per_frame(counts.reads as f64)
                        );
                    }
                }
                let waited = std::mem::take(&mut cpu.host.waited);
                let presented = std::mem::take(&mut cpu.host.presented);
                println!("{:.0}% asleep, {:.0}% putting frames on screen", waited / seconds * 100.0, presented / seconds * 100.0);
                // How evenly frames reached the screen, which an average hides.
                let mut shown = std::mem::take(&mut cpu.host.shown_frames);
                if !shown.is_empty() {
                    shown.sort_by(f32::total_cmp);
                    let at = |share: f32| shown[((shown.len() - 1) as f32 * share) as usize];
                    println!(
                        "frames shown {:.1} ms apart in the middle, 10% to 90% {:.1} to {:.1}, 1% {:.1}, longest {:.1}",
                        at(0.5), at(0.1), at(0.9), at(0.99), shown[shown.len() - 1]
                    );
                }
            }
            "profile" => {
                cpu.hits.clear();
                cpu.profiling = true;
            }
            "hits" => {
                cpu.profiling = false;
                let low: u32 = u32::from_str_radix(value, 16).unwrap_or(0);
                let mut hits: Vec<(u32, u64)> = cpu.hits.iter().map(|(a, n)| (*a, *n)).filter(|(a, _)| *a >= low && *a < low + 0x0100_0000).collect();
                hits.sort();
                for (address, count) in hits {
                    println!("hit {:06x} {count}", address - low + 0x40_0000);
                }
            }
            "answer" => cpu.host.script.answer = value != "no",
            "quit" => {
                println!("the script ended the run");
                std::process::exit(0);
            }
            _ => println!("unknown script step: {step}"),
        }
    }
}
