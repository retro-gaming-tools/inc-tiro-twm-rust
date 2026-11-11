//! advapi32: a small registry. Games of the time kept their install paths and
//! options there. Values start from the port's `recompRegistry`, and what the
//! game writes is kept in `recomp-registry.txt` in the game folder, so options
//! and progress survive.

use super::table;
use crate::cpu::{Cpu, GuestFn};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_MORE_DATA: u32 = 234;
const TEXT: u32 = 1;
const FILE: &str = "recomp-registry.txt";

#[derive(Default)]
pub struct Registry {
    /// A value's kind and bytes, by its key and name in lower case, such as
    /// `software\maker\game\paths\install`. The root key is left out.
    values: BTreeMap<String, (u32, Vec<u8>)>,
    /// Keys the game created that hold no value yet.
    created: Vec<String>,
    /// Open keys, by handle.
    open: HashMap<u32, String>,
    file: PathBuf,
}

impl Registry {
    pub fn load(folder: &Path, defaults: &[(&str, &str)]) -> Registry {
        let mut registry = Registry { file: folder.join(FILE), ..Registry::default() };
        for (name, text) in defaults {
            let mut bytes = text.as_bytes().to_vec();
            bytes.push(0);
            registry.values.insert(name.to_lowercase(), (TEXT, bytes));
        }
        // Each line is `key\name=KIND:HEX`. Anything else is skipped.
        for line in std::fs::read_to_string(&registry.file).unwrap_or_default().lines() {
            let Some((name, value)) = line.split_once('=') else { continue };
            let Some((kind, hex)) = value.split_once(':') else { continue };
            let bytes: Option<Vec<u8>> = (0..hex.len() / 2).map(|i| u8::from_str_radix(hex.get(i * 2..i * 2 + 2)?, 16).ok()).collect();
            if let (Ok(kind), Some(bytes)) = (kind.parse(), bytes) {
                registry.values.insert(name.to_lowercase(), (kind, bytes));
            }
        }
        registry
    }

    /// Fails quietly when the game folder can't be written: the game then
    /// starts from its defaults next time.
    fn save(&self) {
        let mut text = String::new();
        for (name, (kind, bytes)) in &self.values {
            let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            text.push_str(&format!("{name}={kind}:{hex}\n"));
        }
        let _ = std::fs::write(&self.file, text);
    }

    fn exists(&self, key: &str) -> bool {
        let prefix = format!("{key}\\");
        self.created.iter().any(|created| created == key) || self.values.keys().any(|name| name.starts_with(&prefix))
    }
}

/// The path of a key under an open key or a root such as HKEY_LOCAL_MACHINE.
fn key_path(cpu: &Cpu, parent: u32, sub: u32) -> String {
    let sub = cpu.string(sub).trim_matches('\\').to_lowercase();
    match cpu.host.registry.open.get(&parent) {
        Some(path) if sub.is_empty() => path.clone(),
        Some(path) => format!("{path}\\{sub}"),
        None => sub,
    }
}

fn value_name(cpu: &Cpu, key: u32, name: u32) -> String {
    let path = cpu.host.registry.open.get(&key).cloned().unwrap_or_default();
    format!("{path}\\{}", cpu.string(name).to_lowercase())
}

fn open(cpu: &mut Cpu, path: String, out: u32) -> u32 {
    let handle = cpu.host.handle();
    cpu.host.registry.open.insert(handle, path);
    cpu.w32(out, handle);
    0
}

pub fn find(name: &str) -> Option<GuestFn> {
    table! { name;
        "RegOpenKeyExA" 5 |cpu| {
            let (path, out) = (key_path(cpu, cpu.arg(0), cpu.arg(1)), cpu.arg(4));
            if !cpu.host.registry.exists(&path) {
                return ERROR_FILE_NOT_FOUND;
            }
            open(cpu, path, out)
        }
        "RegCreateKeyExA" 9 |cpu| {
            const CREATED: u32 = 1;
            const OPENED: u32 = 2;
            let (path, out, disposition) = (key_path(cpu, cpu.arg(0), cpu.arg(1)), cpu.arg(7), cpu.arg(8));
            let existed = cpu.host.registry.exists(&path);
            if !existed {
                cpu.host.registry.created.push(path.clone());
            }
            if disposition != 0 {
                cpu.w32(disposition, if existed { OPENED } else { CREATED });
            }
            open(cpu, path, out)
        }
        "RegQueryValueExA" 6 |cpu| {
            let (name, kind_out, data, size) = (value_name(cpu, cpu.arg(0), cpu.arg(1)), cpu.arg(3), cpu.arg(4), cpu.arg(5));
            let Some((kind, bytes)) = cpu.host.registry.values.get(&name).cloned() else { return ERROR_FILE_NOT_FOUND };
            if kind_out != 0 {
                cpu.w32(kind_out, kind);
            }
            let room = if size != 0 { cpu.r32(size) } else { 0 };
            if size != 0 {
                cpu.w32(size, bytes.len() as u32);
            }
            if data == 0 {
                return 0;
            }
            if (room as usize) < bytes.len() {
                return ERROR_MORE_DATA;
            }
            cpu.bytes_mut(data, bytes.len() as u32).copy_from_slice(&bytes);
            0
        }
        "RegSetValueExA" 6 |cpu| {
            let (name, kind, data, size) = (value_name(cpu, cpu.arg(0), cpu.arg(1)), cpu.arg(3), cpu.arg(4), cpu.arg(5));
            let bytes = cpu.bytes(data, size).to_vec();
            cpu.host.registry.values.insert(name, (kind, bytes));
            cpu.host.registry.save();
            0
        }
        "RegCloseKey" 1 |cpu| { let key = cpu.arg(0); cpu.host.registry.open.remove(&key); 0 }
    }
}
