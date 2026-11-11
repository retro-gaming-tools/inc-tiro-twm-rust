//! Reads what the recompiler and the runtime need from a 32-bit PE file.

pub struct Section {
    pub start: u32,
    pub end: u32,
    pub executable: bool,
}

pub struct Import {
    pub slot: u32,
    pub dll: String,
    pub name: Option<String>,
    pub ordinal: u32,
}

pub struct Export {
    pub address: u32,
    pub name: Option<String>,
    pub ordinal: u32,
}

pub struct Image {
    pub base: u32,
    pub entry: u32,
    pub is_dll: bool,
    pub memory: Vec<u8>,
    pub sections: Vec<Section>,
    pub exports: Vec<Export>,
    pub imports: Vec<Import>,
    /// Addresses named by base relocations, whose contents are pointers.
    pub relocated: Vec<u32>,
}

fn u16_at(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset + 2)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]) as u32)
}

fn u32_at(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset + 4)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn text_at(data: &[u8], offset: usize) -> String {
    let bytes = data.get(offset..).unwrap_or(&[]);
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).to_string()
}

impl Image {
    pub fn parse(file: &[u8]) -> Option<Image> {
        let header = u32_at(file, 0x3c)? as usize;
        if file.get(header..header + 4)? != b"PE\0\0" || u16_at(file, header + 4)? != 0x14c {
            return None;
        }
        let section_count = u16_at(file, header + 6)? as usize;
        let optional_size = u16_at(file, header + 20)? as usize;
        let is_dll = u16_at(file, header + 22)? & 0x2000 != 0;
        let optional = header + 24;
        let entry_rva = u32_at(file, optional + 16)?;
        let base = u32_at(file, optional + 28)?;
        let image_size = u32_at(file, optional + 56)? as usize;
        let header_size = u32_at(file, optional + 60)? as usize;
        let directory = |index: usize| -> Option<(u32, u32)> {
            Some((u32_at(file, optional + 96 + index * 8)?, u32_at(file, optional + 100 + index * 8)?))
        };

        let mut memory = vec![0u8; image_size];
        memory.get_mut(..header_size)?.copy_from_slice(file.get(..header_size)?);
        let mut sections = Vec::new();
        for index in 0..section_count {
            let at = optional + optional_size + index * 40;
            let virtual_size = u32_at(file, at + 8)?;
            let rva = u32_at(file, at + 12)?;
            let raw_size = u32_at(file, at + 16)?;
            let raw_at = u32_at(file, at + 20)? as usize;
            let flags = u32_at(file, at + 36)?;
            let copied = (raw_size as usize).min(image_size.saturating_sub(rva as usize));
            if copied > 0 && raw_at > 0 {
                let source = file.get(raw_at..raw_at + copied.min(file.len().saturating_sub(raw_at)))?;
                memory.get_mut(rva as usize..rva as usize + source.len())?.copy_from_slice(source);
            }
            sections.push(Section {
                start: base + rva,
                end: base + rva + virtual_size.max(raw_size),
                executable: flags & 0x2000_0000 != 0,
            });
        }

        let mut image = Image {
            base,
            entry: if entry_rva != 0 { base + entry_rva } else { 0 },
            is_dll,
            memory,
            sections,
            exports: Vec::new(),
            imports: Vec::new(),
            relocated: Vec::new(),
        };
        image.read_exports(directory(0)?);
        image.read_imports(directory(1)?);
        image.read_relocations(directory(5)?);
        Some(image)
    }

    fn read_exports(&mut self, (rva, size): (u32, u32)) {
        if size == 0 {
            return;
        }
        let at = rva as usize;
        let ordinal_base = u32_at(&self.memory, at + 16).unwrap_or(1);
        let count = u32_at(&self.memory, at + 20).unwrap_or(0) as usize;
        let name_count = u32_at(&self.memory, at + 24).unwrap_or(0) as usize;
        let table = u32_at(&self.memory, at + 28).unwrap_or(0) as usize;
        let names = u32_at(&self.memory, at + 32).unwrap_or(0) as usize;
        let ordinals = u32_at(&self.memory, at + 36).unwrap_or(0) as usize;
        for index in 0..count {
            let Some(function) = u32_at(&self.memory, table + index * 4) else { continue };
            // An address inside the directory is a forwarder, not code or data.
            if function == 0 || (rva..rva + size).contains(&function) {
                continue;
            }
            let name = (0..name_count)
                .find(|&n| u16_at(&self.memory, ordinals + n * 2) == Some(index as u32))
                .and_then(|n| u32_at(&self.memory, names + n * 4))
                .map(|offset| text_at(&self.memory, offset as usize));
            self.exports.push(Export { address: self.base + function, name, ordinal: ordinal_base + index as u32 });
        }
    }

    fn read_imports(&mut self, (rva, size): (u32, u32)) {
        if size == 0 {
            return;
        }
        let mut at = rva as usize;
        while let (Some(names), Some(slots)) = (u32_at(&self.memory, at), u32_at(&self.memory, at + 16)) {
            if slots == 0 {
                break;
            }
            let dll = text_at(&self.memory, u32_at(&self.memory, at + 12).unwrap_or(0) as usize).to_lowercase();
            let lookup = if names != 0 { names } else { slots } as usize;
            let mut index = 0;
            loop {
                let entry = u32_at(&self.memory, lookup + index * 4).unwrap_or(0);
                if entry == 0 {
                    break;
                }
                let (name, ordinal) = if entry & 0x8000_0000 != 0 {
                    (None, entry & 0xffff)
                } else {
                    (Some(text_at(&self.memory, entry as usize + 2)), 0)
                };
                self.imports.push(Import { slot: self.base + slots + index as u32 * 4, dll: dll.clone(), name, ordinal });
                index += 1;
            }
            at += 20;
        }
    }

    fn read_relocations(&mut self, (rva, size): (u32, u32)) {
        let mut at = rva as usize;
        let end = at + size as usize;
        while at + 8 <= end {
            let page = u32_at(&self.memory, at).unwrap_or(0);
            let block = u32_at(&self.memory, at + 4).unwrap_or(0) as usize;
            if block < 8 {
                break;
            }
            for entry in (at + 8..at + block).step_by(2) {
                let value = u16_at(&self.memory, entry).unwrap_or(0);
                if value >> 12 == 3 {
                    self.relocated.push(self.base + page + (value & 0xfff));
                }
            }
            at += block;
        }
    }

    /// Moves the image, because several of a game's DLLs want the same base.
    pub fn rebase(&mut self, base: u32) {
        let delta = base.wrapping_sub(self.base);
        let old = self.base;
        for &at in &self.relocated {
            let offset = (at - old) as usize;
            if let Some(value) = u32_at(&self.memory, offset) {
                self.memory[offset..offset + 4].copy_from_slice(&value.wrapping_add(delta).to_le_bytes());
            }
        }
        let moved = |address: u32| address.wrapping_add(delta);
        self.base = base;
        if self.entry != 0 {
            self.entry = moved(self.entry);
        }
        self.relocated.iter_mut().for_each(|at| *at = moved(*at));
        self.imports.iter_mut().for_each(|import| import.slot = moved(import.slot));
        self.exports.iter_mut().for_each(|export| export.address = moved(export.address));
        for section in &mut self.sections {
            section.start = moved(section.start);
            section.end = moved(section.end);
        }
    }

    pub fn is_code(&self, address: u32) -> bool {
        self.sections.iter().any(|s| s.executable && (s.start..s.end).contains(&address))
    }

    pub fn u32_at(&self, address: u32) -> Option<u32> {
        u32_at(&self.memory, address.checked_sub(self.base)? as usize)
    }

    pub fn code_bytes(&self) -> usize {
        self.sections.iter().filter(|s| s.executable).map(|s| (s.end - s.start) as usize).sum()
    }
}
