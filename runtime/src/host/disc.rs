//! Reads the files of a CD image, so a player can hand the port the image of
//! their disc instead of installing the game first.
//!
//! Plain ISO 9660 images have 2048 bytes a sector. Raw ones, such as the
//! `.bin` of a `.bin` and `.cue` pair or a CloneCD `.img`, have 2352 with the
//! data inside. Only the data track is read: audio tracks aren't files.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const LOGICAL: u64 = 2048;
/// The volume descriptor is in sector 16, and names the root directory here.
const DESCRIPTOR_SECTOR: u64 = 16;
const ROOT_RECORD: usize = 156;
const FOLDER_FLAG: u8 = 2;
/// Deeper than any real disc, against images whose folders loop.
const DEEPEST: usize = 16;

pub struct Disc {
    file: File,
    sector: u64,
    skip: u64,
}

#[derive(Clone)]
pub struct Entry {
    /// The path on the disc, with `/` between its parts, as the disc spells it.
    pub path: String,
    sector: u64,
    pub size: u64,
}

/// Whether a file's name says it's a disc image or the sheet that goes with one.
pub fn is_image(path: &Path) -> bool {
    let extension = path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    path.is_file() && matches!(extension.as_str(), "iso" | "bin" | "img" | "mdf" | "cue" | "ccd")
}

/// The file that holds the data: a `.cue` names it in its first FILE line, and
/// a `.ccd` sits beside an `.img` of the same name.
fn data_file(path: &Path) -> PathBuf {
    let extension = path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    match extension.as_str() {
        "cue" => {
            let sheet = std::fs::read(path).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default();
            let named = sheet.lines().find_map(|line| line.trim().strip_prefix("FILE")?.split('"').nth(1).map(str::to_string));
            named.map_or_else(|| path.with_extension("bin"), |name| path.with_file_name(name))
        }
        "ccd" => path.with_extension("img"),
        _ => path.to_path_buf(),
    }
}

impl Disc {
    pub fn open(path: &Path) -> Result<Disc, String> {
        let data = data_file(path);
        let mut file = File::open(&data).map_err(|error| format!("{} can't be opened: {error}", data.display()))?;
        // The descriptor starts with a type byte and "CD001", wherever the
        // sector's data begins.
        for (sector, skip) in [(2048, 0), (2352, 16), (2352, 24), (2336, 8)] {
            let mut mark = [0u8; 5];
            let found = file.seek(SeekFrom::Start(DESCRIPTOR_SECTOR * sector + skip + 1)).is_ok() && file.read_exact(&mut mark).is_ok();
            if found && &mark == b"CD001" {
                return Ok(Disc { file, sector, skip });
            }
        }
        Err(format!("{} isn't a CD image this can read", data.display()))
    }

    fn read_sectors(&mut self, mut sector: u64, size: u64) -> std::io::Result<Vec<u8>> {
        let mut data = vec![0u8; size as usize];
        for part in data.chunks_mut(LOGICAL as usize) {
            self.file.seek(SeekFrom::Start(sector * self.sector + self.skip))?;
            self.file.read_exact(part)?;
            sector += 1;
        }
        Ok(data)
    }

    pub fn read(&mut self, entry: &Entry) -> std::io::Result<Vec<u8>> {
        self.read_sectors(entry.sector, entry.size)
    }

    /// Every file on the disc.
    pub fn files(&mut self) -> Vec<Entry> {
        let mut found = Vec::new();
        if let Ok(descriptor) = self.read_sectors(DESCRIPTOR_SECTOR, LOGICAL) {
            let (sector, size) = extent(&descriptor[ROOT_RECORD..]);
            self.walk(sector, size, String::new(), 0, &mut found);
        }
        found
    }

    fn walk(&mut self, sector: u64, size: u64, folder: String, depth: usize, found: &mut Vec<Entry>) {
        let Ok(data) = self.read_sectors(sector, size) else { return };
        let mut at = 0;
        while at + 34 <= data.len() {
            let length = data[at] as usize;
            if length == 0 {
                // Records never cross a sector: the rest of this one is padding.
                at = (at / LOGICAL as usize + 1) * LOGICAL as usize;
                continue;
            }
            let record = &data[at..(at + length).min(data.len())];
            at += length;
            if record.len() < 34 || record.len() < 33 + record[32] as usize {
                continue;
            }
            let name = &record[33..33 + record[32] as usize];
            if name == [0] || name == [1] {
                continue; // The folder itself, and the one above it.
            }
            let text: String = name.iter().map(|&byte| byte as char).collect();
            let text = text.split(';').next().unwrap_or_default().trim_end_matches('.').to_string();
            // Names come from the image, and mustn't lead out of a folder.
            if text.is_empty() || text == ".." || text.contains(['/', '\\']) {
                continue;
            }
            let (sector, size) = extent(record);
            let path = if folder.is_empty() { text } else { format!("{folder}/{text}") };
            if record[25] & FOLDER_FLAG != 0 {
                if depth < DEEPEST {
                    self.walk(sector, size, path, depth + 1, found);
                }
            } else {
                found.push(Entry { path, sector, size });
            }
        }
    }
}

/// Where a directory record's data starts, and its size.
fn extent(record: &[u8]) -> (u64, u64) {
    let number = |at: usize| u32::from_le_bytes([record[at], record[at + 1], record[at + 2], record[at + 3]]) as u64;
    (number(2) + record[1] as u64, number(10))
}

/// Copies every file of an image into a folder. `progress` gets the bytes
/// written so far and the total. The files go to a folder beside the target
/// first, which takes the target's name only when all are there, so a copy
/// that was interrupted is never taken for a whole game.
pub fn unpack(image: &Path, target: &Path, progress: &mut dyn FnMut(u64, u64)) -> Result<(), String> {
    let mut disc = Disc::open(image)?;
    let files = disc.files();
    let total: u64 = files.iter().map(|entry| entry.size).sum();
    let partial = target.with_extension("unpacking");
    let failed = |error: std::io::Error| format!("The disc couldn't be unpacked into {}: {error}", target.display());
    let _ = std::fs::remove_dir_all(&partial);
    let mut written = 0;
    for entry in &files {
        let path = partial.join(&entry.path);
        std::fs::create_dir_all(path.parent().unwrap_or(&partial)).map_err(failed)?;
        let data = disc.read(entry).map_err(|error| format!("The image ends early or can't be read at {}: {error}", entry.path))?;
        File::create(&path).and_then(|mut file| file.write_all(&data)).map_err(failed)?;
        written += entry.size;
        progress(written, total);
    }
    std::fs::rename(&partial, target).map_err(failed)
}
