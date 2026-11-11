//! Finds the code in an image by following it from what is certainly code.
//!
//! Starts at the entry point and the exports, and adds anything that looks like
//! a pointer to code: base relocations, values stored in data sections, and
//! immediates in the code already found. Follows branches and calls, and reads
//! the jump tables behind indirect jumps.

use crate::pe::Image;
use iced_x86::{Code, Decoder, DecoderOptions, FlowControl, Instruction, Mnemonic, OpKind, Register};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Default)]
pub struct Report {
    pub instructions: BTreeMap<u32, Instruction>,
    /// Real function entries: called, exported, or pointed at and not inside other code.
    pub functions: BTreeSet<u32>,
    /// Addresses inside functions that a pointer might reach.
    pub entries: BTreeSet<u32>,
    pub jump_tables: BTreeMap<u32, Vec<u32>>,
    pub unresolved_jumps: Vec<u32>,
    pub invalid: Vec<u32>,
    pub mnemonics: BTreeMap<Mnemonic, usize>,
}

fn decode_at(image: &Image, address: u32) -> Option<Instruction> {
    let offset = address.checked_sub(image.base)? as usize;
    let bytes = image.memory.get(offset..(offset + 16).min(image.memory.len()))?;
    let mut decoder = Decoder::with_ip(32, bytes, address as u64, DecoderOptions::NONE);
    Some(decoder.decode())
}

/// True if code at this address decodes cleanly for a few instructions.
fn plausible_code(image: &Image, mut address: u32) -> bool {
    for _ in 0..4 {
        let Some(instruction) = decode_at(image, address) else { return false };
        if instruction.code() == Code::INVALID || !image.is_code(address) {
            return false;
        }
        match instruction.flow_control() {
            FlowControl::Return | FlowControl::UnconditionalBranch | FlowControl::IndirectBranch => return true,
            _ => address += instruction.len() as u32,
        }
    }
    true
}

fn pointers_in_data(image: &Image) -> Vec<u32> {
    let mut found = Vec::new();
    for section in image.sections.iter().filter(|s| !s.executable) {
        for address in (section.start & !3..section.end.saturating_sub(3)).step_by(4) {
            if let Some(value) = image.u32_at(address) {
                if image.is_code(value) {
                    found.push(value);
                }
            }
        }
    }
    found
}

/// Reads a table behind `jmp [index*4+table]`, which the compiler lays out as
/// consecutive pointers. It ends at the first value that isn't code.
fn read_jump_table(image: &Image, table: u32) -> Vec<u32> {
    let mut targets = Vec::new();
    let mut at = table;
    while let Some(value) = image.u32_at(at) {
        if !image.is_code(value) || targets.len() >= 1024 {
            break;
        }
        targets.push(value);
        at += 4;
    }
    targets
}

struct Walk<'a> {
    image: &'a Image,
    slots: BTreeSet<u32>,
    report: Report,
    work: Vec<u32>,
    candidates: Vec<u32>,
    called: BTreeSet<u32>,
    pointed: BTreeSet<u32>,
}

pub fn discover(image: &Image) -> Report {
    let exports = image.exports.iter().map(|e| e.address).filter(|&a| image.is_code(a));
    let mut work: Vec<u32> = exports.collect();
    if image.entry != 0 {
        work.push(image.entry);
    }
    let mut candidates: Vec<u32> = image.relocated.iter().filter_map(|&at| image.u32_at(at)).filter(|&v| image.is_code(v)).collect();
    candidates.extend(pointers_in_data(image));
    let called = work.iter().copied().collect();
    let slots = image.imports.iter().map(|i| i.slot).collect();
    let mut walk = Walk { image, slots, report: Report::default(), work, candidates, called, pointed: BTreeSet::new() };

    let mut seen: HashSet<u32> = HashSet::new();
    loop {
        while let Some(address) = walk.work.pop() {
            walk.follow(address);
        }
        let Some(candidate) = walk.candidates.pop() else { break };
        if !seen.insert(candidate) {
            continue;
        }
        if walk.report.instructions.contains_key(&candidate) {
            walk.pointed.insert(candidate);
        } else if plausible_code(image, candidate) {
            walk.pointed.insert(candidate);
            walk.work.push(candidate);
        }
    }
    walk.finish()
}

impl Walk<'_> {
    /// Case labels and addresses inside functions also look like pointers, so a
    /// pointed-at address only counts as an entry if nothing jumps or falls into it.
    fn finish(mut self) -> Report {
        let mut inside: HashSet<u32> = HashSet::new();
        for (&address, instruction) in &self.report.instructions {
            let next = address + instruction.len() as u32;
            match instruction.flow_control() {
                FlowControl::UnconditionalBranch => {
                    inside.insert(instruction.near_branch_target() as u32);
                }
                FlowControl::ConditionalBranch => {
                    inside.insert(instruction.near_branch_target() as u32);
                    inside.insert(next);
                }
                FlowControl::Next | FlowControl::Call | FlowControl::IndirectCall | FlowControl::Interrupt => {
                    inside.insert(next);
                }
                _ => {}
            }
        }
        let cases: HashSet<u32> = self.report.jump_tables.values().flatten().copied().collect();
        inside.extend(&cases);
        let pointed = self.pointed.iter().copied().filter(|address| !inside.contains(address));
        let functions: BTreeSet<u32> = self.called.iter().copied().chain(pointed).collect();
        // Anything else pointed at may still be reached through a pointer, so it
        // gets an entry for indirect calls, without splitting the code around it.
        self.report.entries = self.pointed.iter().copied().filter(|a| !functions.contains(a) && !cases.contains(a)).collect();
        self.report.functions = functions;
        self.report
    }

    fn is_import(&self, instruction: &Instruction) -> bool {
        instruction.memory_base() == Register::None
            && instruction.memory_index() == Register::None
            && self.slots.contains(&instruction.memory_displacement32())
    }

    fn follow(&mut self, start: u32) {
        let image = self.image;
        let mut address = start;
        loop {
            if self.report.instructions.contains_key(&address) || !image.is_code(address) {
                return;
            }
            let Some(instruction) = decode_at(image, address) else { return };
            if instruction.code() == Code::INVALID {
                self.report.invalid.push(address);
                return;
            }
            self.report.instructions.insert(address, instruction);
            *self.report.mnemonics.entry(instruction.mnemonic()).or_default() += 1;
            for operand in 0..instruction.op_count() {
                if instruction.op_kind(operand) == OpKind::Immediate32 && image.is_code(instruction.immediate32()) {
                    self.candidates.push(instruction.immediate32());
                }
            }
            let next = address + instruction.len() as u32;
            match instruction.flow_control() {
                FlowControl::Next | FlowControl::Interrupt | FlowControl::IndirectCall | FlowControl::Exception | FlowControl::XbeginXabortXend => address = next,
                FlowControl::ConditionalBranch => {
                    self.work.push(instruction.near_branch_target() as u32);
                    address = next;
                }
                FlowControl::UnconditionalBranch => address = instruction.near_branch_target() as u32,
                FlowControl::Call => {
                    let target = instruction.near_branch_target() as u32;
                    if image.is_code(target) {
                        self.called.insert(target);
                        self.work.push(target);
                    }
                    address = next;
                }
                FlowControl::IndirectBranch => {
                    if !self.is_import(&instruction) {
                        self.jump_table(address, &instruction);
                    }
                    return;
                }
                _ => return,
            }
        }
    }

    fn jump_table(&mut self, address: u32, instruction: &Instruction) {
        let targets = if instruction.op0_kind() == OpKind::Memory && instruction.memory_index_scale() == 4 {
            read_jump_table(self.image, instruction.memory_displacement32())
        } else {
            Vec::new()
        };
        if targets.is_empty() {
            self.report.unresolved_jumps.push(address);
        } else {
            self.work.extend(&targets);
            self.report.jump_tables.insert(address, targets);
        }
    }
}
