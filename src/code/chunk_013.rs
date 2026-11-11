#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_004a38d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a38d0); }
 let mut pc: u32 = 0x4a38d0;
 loop { match pc {
 0x4a38d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3932u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3901; continue;
 }
 0x4a3901 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a390a); f_004a391b(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3916); f_004a3928(cpu);
  pc = 0x4a393c; continue;
 }
 0x4a393c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a391b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a391b); }
 let mut pc: u32 = 0x4a391b;
 loop { match pc {
 0x4a391b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a3927); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3928(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3928); }
 let mut pc: u32 = 0x4a3928;
 loop { match pc {
 0x4a3928 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a3931); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3932(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3932); }
 let mut pc: u32 = 0x4a3932;
 loop { match pc {
 0x4a3932 => {
  { let v: u32 = 0x4bd098u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3950); }
 let mut pc: u32 = 0x4a3950;
 loop { match pc {
 0x4a3950 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3998u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a397d; continue;
 }
 0x4a397d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3989); f_004a398e(cpu);
  pc = 0x4a39a2; continue;
 }
 0x4a39a2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a398e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a398e); }
 let mut pc: u32 = 0x4a398e;
 loop { match pc {
 0x4a398e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3997); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3998(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3998); }
 let mut pc: u32 = 0x4a3998;
 loop { match pc {
 0x4a3998 => {
  { let v: u32 = 0x4bd0c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a39b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a39b0); }
 let mut pc: u32 = 0x4a39b0;
 loop { match pc {
 0x4a39b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3a47u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a39e9; continue;
 }
 0x4a39e9 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a39f2); f_004a3a15(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a39fb); f_004a3a21(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3a04); f_004a3a2e(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3a10); f_004a3a3b(cpu);
  pc = 0x4a3a51; continue;
 }
 0x4a3a51 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3a15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3a15); }
 let mut pc: u32 = 0x4a3a15;
 loop { match pc {
 0x4a3a15 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a3a20); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3a21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3a21); }
 let mut pc: u32 = 0x4a3a21;
 loop { match pc {
 0x4a3a21 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a3a2d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3a2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3a2e); }
 let mut pc: u32 = 0x4a3a2e;
 loop { match pc {
 0x4a3a2e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a3a3a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3a3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3a3b); }
 let mut pc: u32 = 0x4a3a3b;
 loop { match pc {
 0x4a3a3b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x4a3a46); f_00492f30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3a47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3a47); }
 let mut pc: u32 = 0x4a3a47;
 loop { match pc {
 0x4a3a47 => {
  { let v: u32 = 0x4bd0f0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3a60); }
 let mut pc: u32 = 0x4a3a60;
 loop { match pc {
 0x4a3a60 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3aa8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3a8d; continue;
 }
 0x4a3a8d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3a99); f_004a3a9e(cpu);
  pc = 0x4a3ab2; continue;
 }
 0x4a3ab2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3a9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3a9e); }
 let mut pc: u32 = 0x4a3a9e;
 loop { match pc {
 0x4a3a9e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3aa7); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3aa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3aa8); }
 let mut pc: u32 = 0x4a3aa8;
 loop { match pc {
 0x4a3aa8 => {
  { let v: u32 = 0x4bd130u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3b2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3b2a); }
 let mut pc: u32 = 0x4a3b2a;
 loop { match pc {
 0x4a3b2a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x80u32); cpu.ecx = v; }
  cpu.push(0x4a3b38); f_00492080(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3b39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3b39); }
 let mut pc: u32 = 0x4a3b39;
 loop { match pc {
 0x4a3b39 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  if cpu.zf { pc = 0x4a3b50; continue; }
  pc = 0x4a3b45; continue;
 }
 0x4a3b45 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3b57; continue;
 }
 0x4a3b50 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3b57; continue;
 }
 0x4a3b57 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a3b5f); f_00493da0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3b60); }
 let mut pc: u32 = 0x4a3b60;
 loop { match pc {
 0x4a3b60 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  if cpu.zf { pc = 0x4a3b7a; continue; }
  pc = 0x4a3b6c; continue;
 }
 0x4a3b6c => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3b81; continue;
 }
 0x4a3b7a => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3b81; continue;
 }
 0x4a3b81 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x6cu32); cpu.ecx = v; }
  cpu.push(0x4a3b8c); f_00493d30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3b8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3b8d); }
 let mut pc: u32 = 0x4a3b8d;
 loop { match pc {
 0x4a3b8d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a3b95); f_00492f30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3bdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3bdb); }
 let mut pc: u32 = 0x4a3bdb;
 loop { match pc {
 0x4a3bdb => {
  { let v: u32 = 0x4bd158u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3c00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3c00); }
 let mut pc: u32 = 0x4a3c00;
 loop { match pc {
 0x4a3c00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3c11; continue;
 }
 0x4a3c11 => {
  pc = 0x4a3c16; continue;
 }
 0x4a3c16 => {
  pc = 0x4a3c1b; continue;
 }
 0x4a3c1b => {
  pc = 0x4a3c20; continue;
 }
 0x4a3c20 => {
  pc = 0x4a3c25; continue;
 }
 0x4a3c25 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a3c33; continue;
 }
 0x4a3c33 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a3c4c; continue; }
  pc = 0x4a3c3d; continue;
 }
 0x4a3c3d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a3c49); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a3c4c; continue;
 }
 0x4a3c4c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x4a3c57; continue;
 }
 0x4a3c57 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3c60); }
 let mut pc: u32 = 0x4a3c60;
 loop { match pc {
 0x4a3c60 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3cc5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3c8d; continue;
 }
 0x4a3c8d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3c99); f_004a3c9e(cpu);
  pc = 0x4a3ca7; continue;
 }
 0x4a3ca7 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a3cbd; continue; }
  pc = 0x4a3cb1; continue;
 }
 0x4a3cb1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a3cba); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a3cbd; continue;
 }
 0x4a3cbd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a3ccf; continue;
 }
 0x4a3ccf => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3c9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3c9e); }
 let mut pc: u32 = 0x4a3c9e;
 loop { match pc {
 0x4a3c9e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a3ca6); f_0048a280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3cc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3cc5); }
 let mut pc: u32 = 0x4a3cc5;
 loop { match pc {
 0x4a3cc5 => {
  { let v: u32 = 0x4bd198u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3ce0); }
 let mut pc: u32 = 0x4a3ce0;
 loop { match pc {
 0x4a3ce0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3d28u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3d0d; continue;
 }
 0x4a3d0d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3d19); f_004a3d1e(cpu);
  pc = 0x4a3d32; continue;
 }
 0x4a3d32 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3d1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3d1e); }
 let mut pc: u32 = 0x4a3d1e;
 loop { match pc {
 0x4a3d1e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3d27); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3d28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3d28); }
 let mut pc: u32 = 0x4a3d28;
 loop { match pc {
 0x4a3d28 => {
  { let v: u32 = 0x4bd1c0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3d40); }
 let mut pc: u32 = 0x4a3d40;
 loop { match pc {
 0x4a3d40 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3d88u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3d6d; continue;
 }
 0x4a3d6d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3d79); f_004a3d7e(cpu);
  pc = 0x4a3d92; continue;
 }
 0x4a3d92 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3d7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3d7e); }
 let mut pc: u32 = 0x4a3d7e;
 loop { match pc {
 0x4a3d7e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3d87); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3d88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3d88); }
 let mut pc: u32 = 0x4a3d88;
 loop { match pc {
 0x4a3d88 => {
  { let v: u32 = 0x4bd1e8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3da0); }
 let mut pc: u32 = 0x4a3da0;
 loop { match pc {
 0x4a3da0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3db1; continue;
 }
 0x4a3db1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x4a3dbc); f_0049caae(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a3dcb; continue;
 }
 0x4a3dcb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3dd0); }
 let mut pc: u32 = 0x4a3dd0;
 loop { match pc {
 0x4a3dd0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b386cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a3dea; continue;
 }
 0x4a3dea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3df0); }
 let mut pc: u32 = 0x4a3df0;
 loop { match pc {
 0x4a3df0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b386cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a3e0a; continue;
 }
 0x4a3e0a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a3e20; continue; }
  pc = 0x4a3e14; continue;
 }
 0x4a3e14 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a3e1d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a3e20; continue;
 }
 0x4a3e20 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a3e28; continue;
 }
 0x4a3e28 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3e30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3e30); }
 let mut pc: u32 = 0x4a3e30;
 loop { match pc {
 0x4a3e30 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3e7bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3e5d; continue;
 }
 0x4a3e5d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3e69); f_004a3e6e(cpu);
  pc = 0x4a3e85; continue;
 }
 0x4a3e85 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3e6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3e6e); }
 let mut pc: u32 = 0x4a3e6e;
 loop { match pc {
 0x4a3e6e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a3e7a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3e7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3e7b); }
 let mut pc: u32 = 0x4a3e7b;
 loop { match pc {
 0x4a3e7b => {
  { let v: u32 = 0x4bd210u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3ea0); }
 let mut pc: u32 = 0x4a3ea0;
 loop { match pc {
 0x4a3ea0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3ee7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3ecd; continue;
 }
 0x4a3ecd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3ed9); f_004a3ede(cpu);
  pc = 0x4a3ef1; continue;
 }
 0x4a3ef1 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3ede(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3ede); }
 let mut pc: u32 = 0x4a3ede;
 loop { match pc {
 0x4a3ede => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a3ee6); f_004a3dd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3ee7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3ee7); }
 let mut pc: u32 = 0x4a3ee7;
 loop { match pc {
 0x4a3ee7 => {
  { let v: u32 = 0x4bd238u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3f00); }
 let mut pc: u32 = 0x4a3f00;
 loop { match pc {
 0x4a3f00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3f65u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3f2d; continue;
 }
 0x4a3f2d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3f39); f_004a3f3e(cpu);
  pc = 0x4a3f47; continue;
 }
 0x4a3f47 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a3f5d; continue; }
  pc = 0x4a3f51; continue;
 }
 0x4a3f51 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a3f5a); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a3f5d; continue;
 }
 0x4a3f5d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a3f6f; continue;
 }
 0x4a3f6f => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3f3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3f3e); }
 let mut pc: u32 = 0x4a3f3e;
 loop { match pc {
 0x4a3f3e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a3f46); f_004a3dd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3f65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3f65); }
 let mut pc: u32 = 0x4a3f65;
 loop { match pc {
 0x4a3f65 => {
  { let v: u32 = 0x4bd260u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3f80); }
 let mut pc: u32 = 0x4a3f80;
 loop { match pc {
 0x4a3f80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4017u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3fb9; continue;
 }
 0x4a3fb9 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3fc2); f_004a3fe5(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3fcb); f_004a3ff1(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3fd4); f_004a3ffe(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3fe0); f_004a400b(cpu);
  pc = 0x4a4021; continue;
 }
 0x4a4021 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3fe5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3fe5); }
 let mut pc: u32 = 0x4a3fe5;
 loop { match pc {
 0x4a3fe5 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a3ff0); f_004a4030(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3ff1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3ff1); }
 let mut pc: u32 = 0x4a3ff1;
 loop { match pc {
 0x4a3ff1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a3ffd); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3ffe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3ffe); }
 let mut pc: u32 = 0x4a3ffe;
 loop { match pc {
 0x4a3ffe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a400a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a400b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a400b); }
 let mut pc: u32 = 0x4a400b;
 loop { match pc {
 0x4a400b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  cpu.push(0x4a4016); f_004a39b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4017(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4017); }
 let mut pc: u32 = 0x4a4017;
 loop { match pc {
 0x4a4017 => {
  { let v: u32 = 0x4bd288u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4030); }
 let mut pc: u32 = 0x4a4030;
 loop { match pc {
 0x4a4030 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a408au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3484u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a406a); cpu.call(t);
  pc = 0x4a406f; continue;
 }
 0x4a406f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a407b); f_004a4080(cpu);
  pc = 0x4a4094; continue;
 }
 0x4a4094 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4080); }
 let mut pc: u32 = 0x4a4080;
 loop { match pc {
 0x4a4080 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4089); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a408a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a408a); }
 let mut pc: u32 = 0x4a408a;
 loop { match pc {
 0x4a408a => {
  { let v: u32 = 0x4bd2c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a40b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a40b0); }
 let mut pc: u32 = 0x4a40b0;
 loop { match pc {
 0x4a40b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a41bbu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x18u32; cpu.push(v); }
  cpu.push(0x4a40d8); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a4192; continue; }
  pc = 0x4a40ef; continue;
 }
 0x4a40ef => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a4107); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x4a411d); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x4a4134); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a413d; continue;
 }
 0x4a413d => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4169; continue;
 }
 0x4a4169 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4172; continue;
 }
 0x4a4172 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a418d; continue;
 }
 0x4a418d => {
  pc = 0x4a4199; continue;
 }
 0x4a4192 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4199; continue;
 }
 0x4a4199 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a41c5; continue;
 }
 0x4a41c5 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a414f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a414f); }
 let mut pc: u32 = 0x4a414f;
 loop { match pc {
 0x4a414f => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a415b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a415c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a415c); }
 let mut pc: u32 = 0x4a415c;
 loop { match pc {
 0x4a415c => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a4168); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4181(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4181); }
 let mut pc: u32 = 0x4a4181;
 loop { match pc {
 0x4a4181 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a418c); f_004a38d0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a41ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a41ae); }
 let mut pc: u32 = 0x4a41ae;
 loop { match pc {
 0x4a41ae => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a41b7); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a41bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a41bb); }
 let mut pc: u32 = 0x4a41bb;
 loop { match pc {
 0x4a41bb => {
  { let v: u32 = 0x4bd2f0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a41e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a41e0); }
 let mut pc: u32 = 0x4a41e0;
 loop { match pc {
 0x4a41e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4261u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a425c; continue; }
  pc = 0x4a4217; continue;
 }
 0x4a4217 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4229; continue;
 }
 0x4a4229 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4235); f_004a423a(cpu);
  pc = 0x4a4246; continue;
 }
 0x4a4246 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a424f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a4257; continue;
 }
 0x4a4257 => {
  pc = 0x4a425c; continue;
 }
 0x4a425c => {
  pc = 0x4a426b; continue;
 }
 0x4a426b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a423a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a423a); }
 let mut pc: u32 = 0x4a423a;
 loop { match pc {
 0x4a423a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a4245); f_004a38d0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4261(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4261); }
 let mut pc: u32 = 0x4a4261;
 loop { match pc {
 0x4a4261 => {
  { let v: u32 = 0x4bd330u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4280); }
 let mut pc: u32 = 0x4a4280;
 loop { match pc {
 0x4a4280 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a42dau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a42ba); cpu.call(t);
  pc = 0x4a42bf; continue;
 }
 0x4a42bf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a42cb); f_004a42d0(cpu);
  pc = 0x4a42e4; continue;
 }
 0x4a42e4 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a42d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a42d0); }
 let mut pc: u32 = 0x4a42d0;
 loop { match pc {
 0x4a42d0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a42d9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a42da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a42da); }
 let mut pc: u32 = 0x4a42da;
 loop { match pc {
 0x4a42da => {
  { let v: u32 = 0x4bd358u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4300); }
 let mut pc: u32 = 0x4a4300;
 loop { match pc {
 0x4a4300 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a43f5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x4a4328); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a43cc; continue; }
  pc = 0x4a433f; continue;
 }
 0x4a433f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a4357); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x4a437b); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4384; continue;
 }
 0x4a4384 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a43a3; continue;
 }
 0x4a43a3 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a43ac; continue;
 }
 0x4a43ac => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a43c7; continue;
 }
 0x4a43c7 => {
  pc = 0x4a43d3; continue;
 }
 0x4a43cc => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a43d3; continue;
 }
 0x4a43d3 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a43ff; continue;
 }
 0x4a43ff => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4396(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4396); }
 let mut pc: u32 = 0x4a4396;
 loop { match pc {
 0x4a4396 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a43a2); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a43bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a43bb); }
 let mut pc: u32 = 0x4a43bb;
 loop { match pc {
 0x4a43bb => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a43c6); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a43e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a43e8); }
 let mut pc: u32 = 0x4a43e8;
 loop { match pc {
 0x4a43e8 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a43f1); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a43f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a43f5); }
 let mut pc: u32 = 0x4a43f5;
 loop { match pc {
 0x4a43f5 => {
  { let v: u32 = 0x4bd380u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4410); }
 let mut pc: u32 = 0x4a4410;
 loop { match pc {
 0x4a4410 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4491u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a448c; continue; }
  pc = 0x4a4447; continue;
 }
 0x4a4447 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4459; continue;
 }
 0x4a4459 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4465); f_004a446a(cpu);
  pc = 0x4a4476; continue;
 }
 0x4a4476 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a447f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a4487; continue;
 }
 0x4a4487 => {
  pc = 0x4a448c; continue;
 }
 0x4a448c => {
  pc = 0x4a449b; continue;
 }
 0x4a449b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a446a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a446a); }
 let mut pc: u32 = 0x4a446a;
 loop { match pc {
 0x4a446a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a4475); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4491(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4491); }
 let mut pc: u32 = 0x4a4491;
 loop { match pc {
 0x4a4491 => {
  { let v: u32 = 0x4bd3b8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a44b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a44b0); }
 let mut pc: u32 = 0x4a44b0;
 loop { match pc {
 0x4a44b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a450au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38c4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a44ea); cpu.call(t);
  pc = 0x4a44ef; continue;
 }
 0x4a44ef => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a44fb); f_004a4500(cpu);
  pc = 0x4a4514; continue;
 }
 0x4a4514 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4500); }
 let mut pc: u32 = 0x4a4500;
 loop { match pc {
 0x4a4500 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4509); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a450a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a450a); }
 let mut pc: u32 = 0x4a450a;
 loop { match pc {
 0x4a450a => {
  { let v: u32 = 0x4bd3e0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4530); }
 let mut pc: u32 = 0x4a4530;
 loop { match pc {
 0x4a4530 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a45bdu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a4558); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a4594; continue; }
  pc = 0x4a456f; continue;
 }
 0x4a456f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a4578); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a4589; continue;
 }
 0x4a4589 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a459b; continue;
 }
 0x4a4594 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a459b; continue;
 }
 0x4a459b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a45c7; continue;
 }
 0x4a45c7 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a45b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a45b0); }
 let mut pc: u32 = 0x4a45b0;
 loop { match pc {
 0x4a45b0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a45b9); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a45bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a45bd); }
 let mut pc: u32 = 0x4a45bd;
 loop { match pc {
 0x4a45bd => {
  { let v: u32 = 0x4bd408u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a45e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a45e0); }
 let mut pc: u32 = 0x4a45e0;
 loop { match pc {
 0x4a45e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4601); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a4609; continue;
 }
 0x4a4609 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4610); }
 let mut pc: u32 = 0x4a4610;
 loop { match pc {
 0x4a4610 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a466au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3284u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a464a); cpu.call(t);
  pc = 0x4a464f; continue;
 }
 0x4a464f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a465b); f_004a4660(cpu);
  pc = 0x4a4674; continue;
 }
 0x4a4674 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4660); }
 let mut pc: u32 = 0x4a4660;
 loop { match pc {
 0x4a4660 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4669); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a466a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a466a); }
 let mut pc: u32 = 0x4a466a;
 loop { match pc {
 0x4a466a => {
  { let v: u32 = 0x4bd430u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4690); }
 let mut pc: u32 = 0x4a4690;
 loop { match pc {
 0x4a4690 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a471du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a46b8); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a46f4; continue; }
  pc = 0x4a46cf; continue;
 }
 0x4a46cf => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a46d8); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a46e9; continue;
 }
 0x4a46e9 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a46fb; continue;
 }
 0x4a46f4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a46fb; continue;
 }
 0x4a46fb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4727; continue;
 }
 0x4a4727 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4710); }
 let mut pc: u32 = 0x4a4710;
 loop { match pc {
 0x4a4710 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4719); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a471d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a471d); }
 let mut pc: u32 = 0x4a471d;
 loop { match pc {
 0x4a471d => {
  { let v: u32 = 0x4bd458u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4740); }
 let mut pc: u32 = 0x4a4740;
 loop { match pc {
 0x4a4740 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a479au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3274u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a477a); cpu.call(t);
  pc = 0x4a477f; continue;
 }
 0x4a477f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a478b); f_004a4790(cpu);
  pc = 0x4a47a4; continue;
 }
 0x4a47a4 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4790); }
 let mut pc: u32 = 0x4a4790;
 loop { match pc {
 0x4a4790 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4799); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a479a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a479a); }
 let mut pc: u32 = 0x4a479a;
 loop { match pc {
 0x4a479a => {
  { let v: u32 = 0x4bd480u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a47c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a47c0); }
 let mut pc: u32 = 0x4a47c0;
 loop { match pc {
 0x4a47c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a484du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a47e8); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a4824; continue; }
  pc = 0x4a47ff; continue;
 }
 0x4a47ff => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a4808); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a4819; continue;
 }
 0x4a4819 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a482b; continue;
 }
 0x4a4824 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a482b; continue;
 }
 0x4a482b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4857; continue;
 }
 0x4a4857 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4840); }
 let mut pc: u32 = 0x4a4840;
 loop { match pc {
 0x4a4840 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4849); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a484d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a484d); }
 let mut pc: u32 = 0x4a484d;
 loop { match pc {
 0x4a484d => {
  { let v: u32 = 0x4bd4a8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4870(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4870); }
 let mut pc: u32 = 0x4a4870;
 loop { match pc {
 0x4a4870 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a48cau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3294u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a48aa); cpu.call(t);
  pc = 0x4a48af; continue;
 }
 0x4a48af => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a48bb); f_004a48c0(cpu);
  pc = 0x4a48d4; continue;
 }
 0x4a48d4 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a48c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a48c0); }
 let mut pc: u32 = 0x4a48c0;
 loop { match pc {
 0x4a48c0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a48c9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a48ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a48ca); }
 let mut pc: u32 = 0x4a48ca;
 loop { match pc {
 0x4a48ca => {
  { let v: u32 = 0x4bd4d0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a48f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a48f0); }
 let mut pc: u32 = 0x4a48f0;
 loop { match pc {
 0x4a48f0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a49a8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x4a4918); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a497f; continue; }
  pc = 0x4a492f; continue;
 }
 0x4a492f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a4947); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a4956); f_00483d99(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a495f; continue;
 }
 0x4a495f => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a497a; continue;
 }
 0x4a497a => {
  pc = 0x4a4986; continue;
 }
 0x4a497f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4986; continue;
 }
 0x4a4986 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a49b2; continue;
 }
 0x4a49b2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a496e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a496e); }
 let mut pc: u32 = 0x4a496e;
 loop { match pc {
 0x4a496e => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a4979); f_00483d4c(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a499b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a499b); }
 let mut pc: u32 = 0x4a499b;
 loop { match pc {
 0x4a499b => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a49a4); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a49a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a49a8); }
 let mut pc: u32 = 0x4a49a8;
 loop { match pc {
 0x4a49a8 => {
  { let v: u32 = 0x4bd4f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a49d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a49d0); }
 let mut pc: u32 = 0x4a49d0;
 loop { match pc {
 0x4a49d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4a51u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a4a4c; continue; }
  pc = 0x4a4a07; continue;
 }
 0x4a4a07 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4a19; continue;
 }
 0x4a4a19 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4a25); f_004a4a2a(cpu);
  pc = 0x4a4a36; continue;
 }
 0x4a4a36 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4a3f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a4a47; continue;
 }
 0x4a4a47 => {
  pc = 0x4a4a4c; continue;
 }
 0x4a4a4c => {
  pc = 0x4a4a5b; continue;
 }
 0x4a4a5b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4a2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4a2a); }
 let mut pc: u32 = 0x4a4a2a;
 loop { match pc {
 0x4a4a2a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a4a35); f_00483d4c(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4a51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4a51); }
 let mut pc: u32 = 0x4a4a51;
 loop { match pc {
 0x4a4a51 => {
  { let v: u32 = 0x4bd528u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4a70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4a70); }
 let mut pc: u32 = 0x4a4a70;
 loop { match pc {
 0x4a4a70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4acau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b33a4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a4aaa); cpu.call(t);
  pc = 0x4a4aaf; continue;
 }
 0x4a4aaf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4abb); f_004a4ac0(cpu);
  pc = 0x4a4ad4; continue;
 }
 0x4a4ad4 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4ac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4ac0); }
 let mut pc: u32 = 0x4a4ac0;
 loop { match pc {
 0x4a4ac0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4ac9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4aca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4aca); }
 let mut pc: u32 = 0x4a4aca;
 loop { match pc {
 0x4a4aca => {
  { let v: u32 = 0x4bd550u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4af0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4af0); }
 let mut pc: u32 = 0x4a4af0;
 loop { match pc {
 0x4a4af0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4b7du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a4b18); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a4b54; continue; }
  pc = 0x4a4b2f; continue;
 }
 0x4a4b2f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a4b38); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a4b49; continue;
 }
 0x4a4b49 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4b5b; continue;
 }
 0x4a4b54 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4b5b; continue;
 }
 0x4a4b5b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4b87; continue;
 }
 0x4a4b87 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4b70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4b70); }
 let mut pc: u32 = 0x4a4b70;
 loop { match pc {
 0x4a4b70 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4b79); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4b7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4b7d); }
 let mut pc: u32 = 0x4a4b7d;
 loop { match pc {
 0x4a4b7d => {
  { let v: u32 = 0x4bd578u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4ba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4ba0); }
 let mut pc: u32 = 0x4a4ba0;
 loop { match pc {
 0x4a4ba0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4bfau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3394u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a4bda); cpu.call(t);
  pc = 0x4a4bdf; continue;
 }
 0x4a4bdf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4beb); f_004a4bf0(cpu);
  pc = 0x4a4c04; continue;
 }
 0x4a4c04 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4bf0); }
 let mut pc: u32 = 0x4a4bf0;
 loop { match pc {
 0x4a4bf0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4bf9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4bfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4bfa); }
 let mut pc: u32 = 0x4a4bfa;
 loop { match pc {
 0x4a4bfa => {
  { let v: u32 = 0x4bd5a0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4c20); }
 let mut pc: u32 = 0x4a4c20;
 loop { match pc {
 0x4a4c20 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4cadu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a4c48); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a4c84; continue; }
  pc = 0x4a4c5f; continue;
 }
 0x4a4c5f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a4c68); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a4c79; continue;
 }
 0x4a4c79 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4c8b; continue;
 }
 0x4a4c84 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4c8b; continue;
 }
 0x4a4c8b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4cb7; continue;
 }
 0x4a4cb7 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4ca0); }
 let mut pc: u32 = 0x4a4ca0;
 loop { match pc {
 0x4a4ca0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4ca9); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4cad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4cad); }
 let mut pc: u32 = 0x4a4cad;
 loop { match pc {
 0x4a4cad => {
  { let v: u32 = 0x4bd5c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4cd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4cd0); }
 let mut pc: u32 = 0x4a4cd0;
 loop { match pc {
 0x4a4cd0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4d48u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3484u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a4d0a); cpu.call(t);
  pc = 0x4a4d0f; continue;
 }
 0x4a4d0f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4d1b); f_004a4d20(cpu);
  pc = 0x4a4d2a; continue;
 }
 0x4a4d2a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a4d40; continue; }
  pc = 0x4a4d34; continue;
 }
 0x4a4d34 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4d3d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a4d40; continue;
 }
 0x4a4d40 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4d52; continue;
 }
 0x4a4d52 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4d20); }
 let mut pc: u32 = 0x4a4d20;
 loop { match pc {
 0x4a4d20 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4d29); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4d48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4d48); }
 let mut pc: u32 = 0x4a4d48;
 loop { match pc {
 0x4a4d48 => {
  { let v: u32 = 0x4bd5f0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4d70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4d70); }
 let mut pc: u32 = 0x4a4d70;
 loop { match pc {
 0x4a4d70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4de8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a4daa); cpu.call(t);
  pc = 0x4a4daf; continue;
 }
 0x4a4daf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4dbb); f_004a4dc0(cpu);
  pc = 0x4a4dca; continue;
 }
 0x4a4dca => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a4de0; continue; }
  pc = 0x4a4dd4; continue;
 }
 0x4a4dd4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4ddd); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a4de0; continue;
 }
 0x4a4de0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4df2; continue;
 }
 0x4a4df2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4dc0); }
 let mut pc: u32 = 0x4a4dc0;
 loop { match pc {
 0x4a4dc0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4dc9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4de8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4de8); }
 let mut pc: u32 = 0x4a4de8;
 loop { match pc {
 0x4a4de8 => {
  { let v: u32 = 0x4bd618u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4e10); }
 let mut pc: u32 = 0x4a4e10;
 loop { match pc {
 0x4a4e10 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4e76u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x4a4e4e); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a4e5a; continue;
 }
 0x4a4e5a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4e80; continue;
 }
 0x4a4e80 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4e69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4e69); }
 let mut pc: u32 = 0x4a4e69;
 loop { match pc {
 0x4a4e69 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a4e75); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4e76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4e76); }
 let mut pc: u32 = 0x4a4e76;
 loop { match pc {
 0x4a4e76 => {
  { let v: u32 = 0x4bd640u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4ea0); }
 let mut pc: u32 = 0x4a4ea0;
 loop { match pc {
 0x4a4ea0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4f18u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38c4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a4eda); cpu.call(t);
  pc = 0x4a4edf; continue;
 }
 0x4a4edf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4eeb); f_004a4ef0(cpu);
  pc = 0x4a4efa; continue;
 }
 0x4a4efa => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a4f10; continue; }
  pc = 0x4a4f04; continue;
 }
 0x4a4f04 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4f0d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a4f10; continue;
 }
 0x4a4f10 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4f22; continue;
 }
 0x4a4f22 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4ef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4ef0); }
 let mut pc: u32 = 0x4a4ef0;
 loop { match pc {
 0x4a4ef0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4ef9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4f18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4f18); }
 let mut pc: u32 = 0x4a4f18;
 loop { match pc {
 0x4a4f18 => {
  { let v: u32 = 0x4bd668u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4f40); }
 let mut pc: u32 = 0x4a4f40;
 loop { match pc {
 0x4a4f40 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a4fb8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3284u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a4f7a); cpu.call(t);
  pc = 0x4a4f7f; continue;
 }
 0x4a4f7f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a4f8b); f_004a4f90(cpu);
  pc = 0x4a4f9a; continue;
 }
 0x4a4f9a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a4fb0; continue; }
  pc = 0x4a4fa4; continue;
 }
 0x4a4fa4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a4fad); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a4fb0; continue;
 }
 0x4a4fb0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a4fc2; continue;
 }
 0x4a4fc2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4f90); }
 let mut pc: u32 = 0x4a4f90;
 loop { match pc {
 0x4a4f90 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a4f99); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4fb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4fb8); }
 let mut pc: u32 = 0x4a4fb8;
 loop { match pc {
 0x4a4fb8 => {
  { let v: u32 = 0x4bd690u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a4fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a4fe0); }
 let mut pc: u32 = 0x4a4fe0;
 loop { match pc {
 0x4a4fe0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a5058u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3274u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a501a); cpu.call(t);
  pc = 0x4a501f; continue;
 }
 0x4a501f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a502b); f_004a5030(cpu);
  pc = 0x4a503a; continue;
 }
 0x4a503a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5050; continue; }
  pc = 0x4a5044; continue;
 }
 0x4a5044 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a504d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a5050; continue;
 }
 0x4a5050 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a5062; continue;
 }
 0x4a5062 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5030); }
 let mut pc: u32 = 0x4a5030;
 loop { match pc {
 0x4a5030 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a5039); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5058(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5058); }
 let mut pc: u32 = 0x4a5058;
 loop { match pc {
 0x4a5058 => {
  { let v: u32 = 0x4bd6b8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5080); }
 let mut pc: u32 = 0x4a5080;
 loop { match pc {
 0x4a5080 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a50f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3294u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a50ba); cpu.call(t);
  pc = 0x4a50bf; continue;
 }
 0x4a50bf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a50cb); f_004a50d0(cpu);
  pc = 0x4a50da; continue;
 }
 0x4a50da => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a50f0; continue; }
  pc = 0x4a50e4; continue;
 }
 0x4a50e4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a50ed); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a50f0; continue;
 }
 0x4a50f0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a5102; continue;
 }
 0x4a5102 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a50d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a50d0); }
 let mut pc: u32 = 0x4a50d0;
 loop { match pc {
 0x4a50d0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a50d9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a50f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a50f8); }
 let mut pc: u32 = 0x4a50f8;
 loop { match pc {
 0x4a50f8 => {
  { let v: u32 = 0x4bd6e0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5120); }
 let mut pc: u32 = 0x4a5120;
 loop { match pc {
 0x4a5120 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a5198u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b33a4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a515a); cpu.call(t);
  pc = 0x4a515f; continue;
 }
 0x4a515f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a516b); f_004a5170(cpu);
  pc = 0x4a517a; continue;
 }
 0x4a517a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5190; continue; }
  pc = 0x4a5184; continue;
 }
 0x4a5184 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a518d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a5190; continue;
 }
 0x4a5190 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a51a2; continue;
 }
 0x4a51a2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5170); }
 let mut pc: u32 = 0x4a5170;
 loop { match pc {
 0x4a5170 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a5179); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5198(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5198); }
 let mut pc: u32 = 0x4a5198;
 loop { match pc {
 0x4a5198 => {
  { let v: u32 = 0x4bd708u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a51c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a51c0); }
 let mut pc: u32 = 0x4a51c0;
 loop { match pc {
 0x4a51c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a5238u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3394u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a51fa); cpu.call(t);
  pc = 0x4a51ff; continue;
 }
 0x4a51ff => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a520b); f_004a5210(cpu);
  pc = 0x4a521a; continue;
 }
 0x4a521a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5230; continue; }
  pc = 0x4a5224; continue;
 }
 0x4a5224 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a522d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a5230; continue;
 }
 0x4a5230 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a5242; continue;
 }
 0x4a5242 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5210); }
 let mut pc: u32 = 0x4a5210;
 loop { match pc {
 0x4a5210 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a5219); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5238(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5238); }
 let mut pc: u32 = 0x4a5238;
 loop { match pc {
 0x4a5238 => {
  { let v: u32 = 0x4bd730u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5260); }
 let mut pc: u32 = 0x4a5260;
 loop { match pc {
 0x4a5260 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52576cu32); cpu.push(0x4a5279); cpu.call(t);
  pc = 0x4a527e; continue;
 }
 0x4a527e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a5286; continue;
 }
 0x4a5286 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5290); }
 let mut pc: u32 = 0x4a5290;
 loop { match pc {
 0x4a5290 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4a52b9); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a52c1; continue;
 }
 0x4a52c1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a52d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a52d0); }
 let mut pc: u32 = 0x4a52d0;
 loop { match pc {
 0x4a52d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a5323u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525778u32); cpu.push(0x4a52fe); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a530a; continue;
 }
 0x4a530a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a532d; continue;
 }
 0x4a532d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5319(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5319); }
 let mut pc: u32 = 0x4a5319;
 loop { match pc {
 0x4a5319 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a5322); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5323(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5323); }
 let mut pc: u32 = 0x4a5323;
 loop { match pc {
 0x4a5323 => {
  { let v: u32 = 0x4bd758u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5340); }
 let mut pc: u32 = 0x4a5340;
 loop { match pc {
 0x4a5340 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a5393u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525778u32); cpu.push(0x4a536e); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a537a; continue;
 }
 0x4a537a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a539d; continue;
 }
 0x4a539d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5389(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5389); }
 let mut pc: u32 = 0x4a5389;
 loop { match pc {
 0x4a5389 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a5392); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5393(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5393); }
 let mut pc: u32 = 0x4a5393;
 loop { match pc {
 0x4a5393 => {
  { let v: u32 = 0x4bd780u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a53b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a53b0); }
 let mut pc: u32 = 0x4a53b0;
 loop { match pc {
 0x4a53b0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a53c0; continue;
 }
 0x4a53c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a5450u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a53e6; continue;
 }
 0x4a53e6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a53f2; continue;
 }
 0x4a53f2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a53fe); f_004a5403(cpu);
  pc = 0x4a540f; continue;
 }
 0x4a540f => {
  pc = 0x4a5414; continue;
 }
 0x4a5414 => {
  pc = 0x4a5419; continue;
 }
 0x4a5419 => {
  pc = 0x4a541e; continue;
 }
 0x4a541e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a542c; continue;
 }
 0x4a542c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5445; continue; }
  pc = 0x4a5436; continue;
 }
 0x4a5436 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x6cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5442); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a5445; continue;
 }
 0x4a5445 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x6cu32); cpu.eax = v; }
  pc = 0x4a545a; continue;
 }
 0x4a545a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5403(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5403); }
 let mut pc: u32 = 0x4a5403;
 loop { match pc {
 0x4a5403 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x44u32); cpu.ecx = v; }
  cpu.push(0x4a540e); f_00493cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5450); }
 let mut pc: u32 = 0x4a5450;
 loop { match pc {
 0x4a5450 => {
  { let v: u32 = 0x4bd7a8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5470); }
 let mut pc: u32 = 0x4a5470;
 loop { match pc {
 0x49e4b8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x60u32); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e4dc; continue;
 }
 0x49e4dc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5470 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x49e4b8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5480); }
 let mut pc: u32 = 0x4a5480;
 loop { match pc {
 0x4a5480 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049e4e3(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5490); }
 let mut pc: u32 = 0x4a5490;
 loop { match pc {
 0x4a5490 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a54a0; continue;
 }
 0x4a54a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a5540u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a54c6; continue;
 }
 0x4a54c6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a54d6; continue;
 }
 0x4a54d6 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a54df); f_004a54f0(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a54eb); f_004a54fc(cpu);
  pc = 0x4a5509; continue;
 }
 0x4a5509 => {
  pc = 0x4a550e; continue;
 }
 0x4a550e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a551c; continue;
 }
 0x4a551c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5535; continue; }
  pc = 0x4a5526; continue;
 }
 0x4a5526 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x28u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5532); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a5535; continue;
 }
 0x4a5535 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x28u32); cpu.eax = v; }
  pc = 0x4a554a; continue;
 }
 0x4a554a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a54f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a54f0); }
 let mut pc: u32 = 0x4a54f0;
 loop { match pc {
 0x4a54f0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a54fb); f_00490d80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a54fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a54fc); }
 let mut pc: u32 = 0x4a54fc;
 loop { match pc {
 0x4a54fc => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a5508); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5540); }
 let mut pc: u32 = 0x4a5540;
 loop { match pc {
 0x4a5540 => {
  { let v: u32 = 0x4bd7d0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5560); }
 let mut pc: u32 = 0x4a5560;
 loop { match pc {
 0x4a5560 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049a40c(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5570); }
 let mut pc: u32 = 0x4a5570;
 loop { match pc {
 0x4998ca => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x499d5du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x9cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff58u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2cu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x49990b); cpu.call(t);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x499932; continue; }
  pc = 0x49992d; continue;
 }
 0x49992d => {
  pc = 0x499937; continue;
 }
 0x499932 => {
  pc = 0x499cf6; continue;
 }
 0x499937 => {
  pc = 0x499949; continue;
 }
 0x499949 => {
  pc = 0x49994e; continue;
 }
 0x49994e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x499970); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499979; continue;
 }
 0x499979 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499995; continue;
 }
 0x499995 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499999; continue;
 }
 0x499999 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4999a2); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499c89; continue; }
  pc = 0x4999aa; continue;
 }
 0x4999aa => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x4999c5; continue; }
  pc = 0x4999b9; continue;
 }
 0x4999b9 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4999df; continue;
 }
 0x4999c5 => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4999df; continue;
 }
 0x4999df => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f80d0u32; cpu.push(v); }
  { let v = 0x4c27c8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4999ff); f_004a36a0(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x499a08); f_00405cb0(cpu);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x499a27); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x499a2d); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x499a42; continue; }
  pc = 0x499a3d; continue;
 }
 0x499a3d => {
  pc = 0x499c89; continue;
 }
 0x499a42 => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x499a5d; continue; }
  pc = 0x499a51; continue;
 }
 0x499a51 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499a77; continue;
 }
 0x499a5d => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499a77; continue;
 }
 0x499a77 => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x499a89); f_004a36a0(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x499a92); f_00405cb0(cpu);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x499aa4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499c48; continue; }
  pc = 0x499aac; continue;
 }
 0x499aac => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x499ac2; continue; }
  pc = 0x499ab6; continue;
 }
 0x499ab6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499ad4; continue;
 }
 0x499ac2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499ad4; continue;
 }
 0x499ad4 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x499af1); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c14u32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499b01); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b0a; continue;
 }
 0x499b0a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b18; continue;
 }
 0x499b18 => {
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b25; continue;
 }
 0x499b25 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b33; continue;
 }
 0x499b33 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x499b44); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b4c; continue;
 }
 0x499b4c => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c44u32; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499b5c); cpu.call(t);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499b75); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b80; continue;
 }
 0x499b80 => {
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x499b88); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff74u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499ba3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499bae; continue;
 }
 0x499bae => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c50u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499bbe); cpu.call(t);
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff68u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff74u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499bdd); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499be8; continue;
 }
 0x499be8 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499bfd); cpu.call(t);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499c06); f_00499d1b(cpu);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499c0f); f_00499d25(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x499c1e); cpu.call(t);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x499c29); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x499c31); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499c3a); f_00499d11(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499c43); f_00499d2f(cpu);
  pc = 0x499c89; continue;
 }
 0x499c48 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x499c7b); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x499c84); cpu.call(t);
  pc = 0x499999; continue;
 }
 0x499c89 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x499c92); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499ced; continue; }
  pc = 0x499c9a; continue;
 }
 0x499c9a => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x499cc9); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499cd9); f_00499d4b(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499ce5); f_00499d54(cpu);
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x499d67; continue;
 }
 0x499ced => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499cf6); f_00499d4b(cpu);
  pc = 0x499cf6; continue;
 }
 0x499cf6 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499d09); f_00499d54(cpu);
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x499d67; continue;
 }
 0x499d67 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x4a5570 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4998ca; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5580); }
 let mut pc: u32 = 0x4a5580;
 loop { match pc {
 0x4a5580 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_00499d78(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5590); }
 let mut pc: u32 = 0x4a5590;
 loop { match pc {
 0x499e03 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2cu32); cpu.eax = v; }
  pc = 0x499e1a; continue;
 }
 0x499e1a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a5590 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x499e03; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a55a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a55a0); }
 let mut pc: u32 = 0x4a55a0;
 loop { match pc {
 0x499e1f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x499e4e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x499e60); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499e72; continue; }
  pc = 0x499e68; continue;
 }
 0x499e68 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x499e74; continue;
 }
 0x499e72 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x499e74; continue;
 }
 0x499e74 => {
  pc = 0x499e7c; continue;
 }
 0x499e7c => {
  pc = 0x499e81; continue;
 }
 0x499e81 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a55a0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x499e1f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a55b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a55b0); }
 let mut pc: u32 = 0x4a55b0;
 loop { match pc {
 0x499e88 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.push(0x499eb7); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x499ecb); cpu.call(t);
  pc = 0x499ed0; continue;
 }
 0x499ed0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a55b0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x499e88; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a55c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a55c0); }
 let mut pc: u32 = 0x4a55c0;
 loop { match pc {
 0x499ed5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.push(0x499f04); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x499f18); cpu.call(t);
  pc = 0x499f1d; continue;
 }
 0x499f1d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a55c0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x499ed5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a55d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a55d0); }
 let mut pc: u32 = 0x4a55d0;
 loop { match pc {
 0x499f22 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x499f39; continue;
 }
 0x499f39 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a55d0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x499f22; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a55e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a55e0); }
 let mut pc: u32 = 0x4a55e0;
 loop { match pc {
 0x499f3e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x499f55; continue;
 }
 0x499f55 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a55e0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x499f3e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a55f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a55f0); }
 let mut pc: u32 = 0x4a55f0;
 loop { match pc {
 0x499f5a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49a112u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x499f8b); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499f97; continue;
 }
 0x499f97 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499fb3; continue;
 }
 0x499fb3 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499fba; continue;
 }
 0x499fba => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x499fc3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a07e; continue; }
  pc = 0x499fcb; continue;
 }
 0x499fcb => {
  pc = 0x499fd0; continue;
 }
 0x499fd0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49a016); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x49a022); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x49a034); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a041; continue; }
  pc = 0x49a03c; continue;
 }
 0x49a03c => {
  pc = 0x49a046; continue;
 }
 0x49a041 => {
  pc = 0x49a070; continue;
 }
 0x49a046 => {
  pc = 0x49a055; continue;
 }
 0x49a055 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a068); f_0049a109(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49a11c; continue;
 }
 0x49a070 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49a079); cpu.call(t);
  pc = 0x499fba; continue;
 }
 0x49a07e => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49a085); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49a0bb; continue; }
  pc = 0x49a099; continue;
 }
 0x49a099 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49a0a2); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49a0b0; continue;
 }
 0x49a0b0 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a0c2; continue;
 }
 0x49a0bb => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a0c2; continue;
 }
 0x49a0c2 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49a0dc); cpu.call(t);
  pc = 0x49a0ee; continue;
 }
 0x49a0ee => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a101); f_0049a109(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49a11c; continue;
 }
 0x49a11c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a55f0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x499f5a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5600); }
 let mut pc: u32 = 0x4a5600;
 loop { match pc {
 0x49a12d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49a3d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x78u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49a164); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a170; continue;
 }
 0x49a170 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a18c; continue;
 }
 0x49a18c => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a193; continue;
 }
 0x49a193 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49a19c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a283; continue; }
  pc = 0x49a1a4; continue;
 }
 0x49a1a4 => {
  pc = 0x49a1a9; continue;
 }
 0x49a1a9 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x49a1d5); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x49a1e7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a1f4; continue; }
  pc = 0x49a1ef; continue;
 }
 0x49a1ef => {
  pc = 0x49a1f9; continue;
 }
 0x49a1f4 => {
  pc = 0x49a275; continue;
 }
 0x49a1f9 => {
  pc = 0x49a208; continue;
 }
 0x49a208 => {
  pc = 0x49a20d; continue;
 }
 0x49a20d => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49a22c; continue; }
  pc = 0x49a220; continue;
 }
 0x49a220 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a23e; continue;
 }
 0x49a22c => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a23e; continue;
 }
 0x49a23e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x49a25b); cpu.call(t);
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x49a264); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a270); f_0049a3cf(cpu);
  pc = 0x49a3e2; continue;
 }
 0x49a275 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49a27e); cpu.call(t);
  pc = 0x49a193; continue;
 }
 0x49a283 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c6cu32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49a293); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a29c; continue;
 }
 0x49a29c => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c98u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49a2ac); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49a2c2); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a2ca; continue;
 }
 0x49a2ca => {
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49a2f9); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49a30b); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a313; continue;
 }
 0x49a313 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7cb8u32; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49a323); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49a339); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a341; continue;
 }
 0x49a341 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49a350); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a359); f_0049a3b1(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a362); f_0049a3bb(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49a371); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49a37c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49a384); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a38d); f_0049a3a7(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a396); f_0049a3c5(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a3a2); f_0049a3cf(cpu);
  pc = 0x49a3e2; continue;
 }
 0x49a3e2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5600 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x49a12d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5610); }
 let mut pc: u32 = 0x4a5610;
 loop { match pc {
 0x4a5610 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049a3f2(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5620); }
 let mut pc: u32 = 0x4a5620;
 loop { match pc {
 0x4a3ac0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3bdbu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3ae6; continue;
 }
 0x4a3ae6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3afe; continue;
 }
 0x4a3afe => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3b07); f_004a3b2a(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3b10); f_004a3b39(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3b19); f_004a3b60(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3b25); f_004a3b8d(cpu);
  pc = 0x4a3b96; continue;
 }
 0x4a3b96 => {
  pc = 0x4a3b9b; continue;
 }
 0x4a3b9b => {
  pc = 0x4a3ba0; continue;
 }
 0x4a3ba0 => {
  pc = 0x4a3ba5; continue;
 }
 0x4a3ba5 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x98u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a3bb7; continue;
 }
 0x4a3bb7 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a3bd0; continue; }
  pc = 0x4a3bc1; continue;
 }
 0x4a3bc1 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a3bcd); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a3bd0; continue;
 }
 0x4a3bd0 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x4a3be5; continue;
 }
 0x4a3be5 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5620 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x98u32); cpu.ecx = v; }
  pc = 0x4a3ac0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5630); }
 let mut pc: u32 = 0x4a5630;
 loop { match pc {
 0x49fb98 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x5242b8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x49fbb4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49fbc1; continue; }
  pc = 0x49fbbc; continue;
 }
 0x49fbbc => {
  pc = 0x49fbc6; continue;
 }
 0x49fbc1 => {
  pc = 0x49fbfc; continue;
 }
 0x49fbc6 => {
  pc = 0x49fbd5; continue;
 }
 0x49fbd5 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0xa0u32); cpu.eax = v; }
  if cpu.zf { pc = 0x49fbf0; continue; }
  pc = 0x49fbe3; continue;
 }
 0x49fbe3 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x98u32); cpu.eax = v; }
  pc = 0x49fbf2; continue;
 }
 0x49fbf0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49fbf2; continue;
 }
 0x49fbf2 => {
  pc = 0x49fc10; continue;
 }
 0x49fbfc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x6cu32); cpu.ecx = v; }
  cpu.push(0x49fc0b); f_0049a40c(cpu);
  pc = 0x49fc10; continue;
 }
 0x49fc10 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5630 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x49fb98; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5640); }
 let mut pc: u32 = 0x4a5640;
 loop { match pc {
 0x4998ca => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x499d5du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x9cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff58u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2cu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x49990b); cpu.call(t);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x499932; continue; }
  pc = 0x49992d; continue;
 }
 0x49992d => {
  pc = 0x499937; continue;
 }
 0x499932 => {
  pc = 0x499cf6; continue;
 }
 0x499937 => {
  pc = 0x499949; continue;
 }
 0x499949 => {
  pc = 0x49994e; continue;
 }
 0x49994e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x499970); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499979; continue;
 }
 0x499979 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499995; continue;
 }
 0x499995 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499999; continue;
 }
 0x499999 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4999a2); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499c89; continue; }
  pc = 0x4999aa; continue;
 }
 0x4999aa => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x4999c5; continue; }
  pc = 0x4999b9; continue;
 }
 0x4999b9 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4999df; continue;
 }
 0x4999c5 => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4999df; continue;
 }
 0x4999df => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f80d0u32; cpu.push(v); }
  { let v = 0x4c27c8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4999ff); f_004a36a0(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x499a08); f_00405cb0(cpu);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x499a27); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x499a2d); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x499a42; continue; }
  pc = 0x499a3d; continue;
 }
 0x499a3d => {
  pc = 0x499c89; continue;
 }
 0x499a42 => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x499a5d; continue; }
  pc = 0x499a51; continue;
 }
 0x499a51 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499a77; continue;
 }
 0x499a5d => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499a77; continue;
 }
 0x499a77 => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x499a89); f_004a36a0(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x499a92); f_00405cb0(cpu);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x499aa4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499c48; continue; }
  pc = 0x499aac; continue;
 }
 0x499aac => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x499ac2; continue; }
  pc = 0x499ab6; continue;
 }
 0x499ab6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499ad4; continue;
 }
 0x499ac2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499ad4; continue;
 }
 0x499ad4 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x499af1); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c14u32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499b01); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b0a; continue;
 }
 0x499b0a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b18; continue;
 }
 0x499b18 => {
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b25; continue;
 }
 0x499b25 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b33; continue;
 }
 0x499b33 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x499b44); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b4c; continue;
 }
 0x499b4c => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c44u32; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499b5c); cpu.call(t);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499b75); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499b80; continue;
 }
 0x499b80 => {
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x499b88); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff74u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499ba3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499bae; continue;
 }
 0x499bae => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c50u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499bbe); cpu.call(t);
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff68u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff74u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499bdd); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499be8; continue;
 }
 0x499be8 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499bfd); cpu.call(t);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499c06); f_00499d1b(cpu);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499c0f); f_00499d25(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x499c1e); cpu.call(t);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x499c29); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x499c31); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499c3a); f_00499d11(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499c43); f_00499d2f(cpu);
  pc = 0x499c89; continue;
 }
 0x499c48 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x499c7b); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x499c84); cpu.call(t);
  pc = 0x499999; continue;
 }
 0x499c89 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x499c92); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499ced; continue; }
  pc = 0x499c9a; continue;
 }
 0x499c9a => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x499cc9); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499cd9); f_00499d4b(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499ce5); f_00499d54(cpu);
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x499d67; continue;
 }
 0x499ced => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499cf6); f_00499d4b(cpu);
  pc = 0x499cf6; continue;
 }
 0x499cf6 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499d09); f_00499d54(cpu);
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x499d67; continue;
 }
 0x499d67 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x4a5640 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x4998ca; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5650); }
 let mut pc: u32 = 0x4a5650;
 loop { match pc {
 0x49ee64 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ee70; continue;
 }
 0x49ee70 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x49ee7c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49ef17; continue; }
  pc = 0x49ee84; continue;
 }
 0x49ee84 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49eeb3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49eec3; continue;
 }
 0x49eec3 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49eee5; continue; }
  pc = 0x49eed9; continue;
 }
 0x49eed9 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49eef7; continue;
 }
 0x49eee5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49eef7; continue;
 }
 0x49eef7 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49ef06); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x49ef12); cpu.call(t);
  pc = 0x49ee70; continue;
 }
 0x49ef17 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49ef46); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x49ef6a; continue; }
  pc = 0x49ef5e; continue;
 }
 0x49ef5e => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ef82; continue;
 }
 0x49ef6a => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ef82; continue;
 }
 0x49ef82 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49ef91); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x6cu32); cpu.ecx = v; }
  cpu.push(0x49ef9c); f_00499d78(cpu);
  pc = 0x49efa1; continue;
 }
 0x49efa1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a5650 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x49ee64; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5660); }
 let mut pc: u32 = 0x4a5660;
 loop { match pc {
 0x499e03 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2cu32); cpu.eax = v; }
  pc = 0x499e1a; continue;
 }
 0x499e1a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a5660 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x499e03; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5670); }
 let mut pc: u32 = 0x4a5670;
 loop { match pc {
 0x499e1f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x499e4e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x499e60); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499e72; continue; }
  pc = 0x499e68; continue;
 }
 0x499e68 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x499e74; continue;
 }
 0x499e72 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x499e74; continue;
 }
 0x499e74 => {
  pc = 0x499e7c; continue;
 }
 0x499e7c => {
  pc = 0x499e81; continue;
 }
 0x499e81 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5670 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x499e1f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5680); }
 let mut pc: u32 = 0x4a5680;
 loop { match pc {
 0x499e88 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.push(0x499eb7); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x499ecb); cpu.call(t);
  pc = 0x499ed0; continue;
 }
 0x499ed0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a5680 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x499e88; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5690); }
 let mut pc: u32 = 0x4a5690;
 loop { match pc {
 0x499ed5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.push(0x499f04); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x499f18); cpu.call(t);
  pc = 0x499f1d; continue;
 }
 0x499f1d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a5690 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x499ed5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a56a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a56a0); }
 let mut pc: u32 = 0x4a56a0;
 loop { match pc {
 0x499f22 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x499f39; continue;
 }
 0x499f39 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a56a0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x499f22; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a56b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a56b0); }
 let mut pc: u32 = 0x4a56b0;
 loop { match pc {
 0x499f3e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x499f55; continue;
 }
 0x499f55 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a56b0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x499f3e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a56c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a56c0); }
 let mut pc: u32 = 0x4a56c0;
 loop { match pc {
 0x499f5a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49a112u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x499f8b); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499f97; continue;
 }
 0x499f97 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499fb3; continue;
 }
 0x499fb3 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499fba; continue;
 }
 0x499fba => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x499fc3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a07e; continue; }
  pc = 0x499fcb; continue;
 }
 0x499fcb => {
  pc = 0x499fd0; continue;
 }
 0x499fd0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49a016); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x49a022); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x49a034); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a041; continue; }
  pc = 0x49a03c; continue;
 }
 0x49a03c => {
  pc = 0x49a046; continue;
 }
 0x49a041 => {
  pc = 0x49a070; continue;
 }
 0x49a046 => {
  pc = 0x49a055; continue;
 }
 0x49a055 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a068); f_0049a109(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49a11c; continue;
 }
 0x49a070 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49a079); cpu.call(t);
  pc = 0x499fba; continue;
 }
 0x49a07e => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49a085); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49a0bb; continue; }
  pc = 0x49a099; continue;
 }
 0x49a099 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49a0a2); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49a0b0; continue;
 }
 0x49a0b0 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a0c2; continue;
 }
 0x49a0bb => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a0c2; continue;
 }
 0x49a0c2 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49a0dc); cpu.call(t);
  pc = 0x49a0ee; continue;
 }
 0x49a0ee => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a101); f_0049a109(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49a11c; continue;
 }
 0x49a11c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a56c0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x499f5a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a56d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a56d0); }
 let mut pc: u32 = 0x4a56d0;
 loop { match pc {
 0x49a12d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49a3d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x78u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49a164); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a170; continue;
 }
 0x49a170 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a18c; continue;
 }
 0x49a18c => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a193; continue;
 }
 0x49a193 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49a19c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a283; continue; }
  pc = 0x49a1a4; continue;
 }
 0x49a1a4 => {
  pc = 0x49a1a9; continue;
 }
 0x49a1a9 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x49a1d5); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x49a1e7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a1f4; continue; }
  pc = 0x49a1ef; continue;
 }
 0x49a1ef => {
  pc = 0x49a1f9; continue;
 }
 0x49a1f4 => {
  pc = 0x49a275; continue;
 }
 0x49a1f9 => {
  pc = 0x49a208; continue;
 }
 0x49a208 => {
  pc = 0x49a20d; continue;
 }
 0x49a20d => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49a22c; continue; }
  pc = 0x49a220; continue;
 }
 0x49a220 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a23e; continue;
 }
 0x49a22c => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a23e; continue;
 }
 0x49a23e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x49a25b); cpu.call(t);
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x49a264); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a270); f_0049a3cf(cpu);
  pc = 0x49a3e2; continue;
 }
 0x49a275 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49a27e); cpu.call(t);
  pc = 0x49a193; continue;
 }
 0x49a283 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c6cu32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49a293); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a29c; continue;
 }
 0x49a29c => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7c98u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49a2ac); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49a2c2); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a2ca; continue;
 }
 0x49a2ca => {
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49a2f9); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49a30b); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a313; continue;
 }
 0x49a313 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7cb8u32; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49a323); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49a339); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a341; continue;
 }
 0x49a341 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49a350); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a359); f_0049a3b1(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a362); f_0049a3bb(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49a371); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49a37c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49a384); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a38d); f_0049a3a7(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a396); f_0049a3c5(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a3a2); f_0049a3cf(cpu);
  pc = 0x49a3e2; continue;
 }
 0x49a3e2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a56d0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x49a12d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a56e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a56e0); }
 let mut pc: u32 = 0x4a56e0;
 loop { match pc {
 0x49fa8f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49fb7du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49fac7); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fad3; continue;
 }
 0x49fad3 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49faef; continue;
 }
 0x49faef => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fb04; continue;
 }
 0x49fafb => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49fb04); cpu.call(t);
  pc = 0x49fb04; continue;
 }
 0x49fb04 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49fb0d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49fb5a; continue; }
  pc = 0x49fb15; continue;
 }
 0x49fb15 => {
  pc = 0x49fb1a; continue;
 }
 0x49fb1a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x49fb52); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fafb; continue;
 }
 0x49fb5a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49fb6c); f_0049fb74(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49fb87; continue;
 }
 0x49fb87 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x4a56e0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x49fa8f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a56f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a56f0); }
 let mut pc: u32 = 0x4a56f0;
 loop { match pc {
 0x49ad86 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4b17c8u32); cpu.eax = v; }
  pc = 0x49ad9c; continue;
 }
 0x49ad9c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a56f0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x49ad86; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5700); }
 let mut pc: u32 = 0x4a5700;
 loop { match pc {
 0x49ada3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49adb6; continue;
 }
 0x49adb6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x4a5700 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x49ada3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5710); }
 let mut pc: u32 = 0x4a5710;
 loop { match pc {
 0x49adbd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49adce; continue;
 }
 0x49adce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5710 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x49adbd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5720); }
 let mut pc: u32 = 0x4a5720;
 loop { match pc {
 0x49add5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49ade8; continue;
 }
 0x49ade8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5720 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x49add5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5730); }
 let mut pc: u32 = 0x4a5730;
 loop { match pc {
 0x49adef => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ae00; continue;
 }
 0x49ae00 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5730 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x6cu32); cpu.ecx = v; }
  pc = 0x49adef; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5740); }
 let mut pc: u32 = 0x4a5740;
 loop { match pc {
 0x49e4b8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x60u32); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e4dc; continue;
 }
 0x49e4dc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5740 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x28u32); cpu.ecx = v; }
  pc = 0x49e4b8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5750); }
 let mut pc: u32 = 0x4a5750;
 loop { match pc {
 0x49f916 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.edx)); cpu.push(0x49f937); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x49f946); f_0049e4e3(cpu);
  pc = 0x49f94b; continue;
 }
 0x49f94b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5750 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x49f916; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5760); }
 let mut pc: u32 = 0x4a5760;
 loop { match pc {
 0x49f952 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49fa4eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x49f97f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49f994; continue; }
  pc = 0x49f987; continue;
 }
 0x49f987 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49fa40; continue; }
  pc = 0x49f994; continue;
 }
 0x49f994 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49f9a4); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f9b0; continue;
 }
 0x49f9b0 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f9cc; continue;
 }
 0x49f9cc => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f9d3; continue;
 }
 0x49f9d3 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49f9dc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49fa15; continue; }
  pc = 0x49f9e4; continue;
 }
 0x49f9e4 => {
  pc = 0x49f9e9; continue;
 }
 0x49f9e9 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x49fa07); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49fa10); cpu.call(t);
  pc = 0x49f9d3; continue;
 }
 0x49fa15 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.push(0x49fa2a); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49fa40); f_0049fa45(cpu);
  pc = 0x49fa40; continue;
 }
 0x49fa40 => {
  pc = 0x49fa58; continue;
 }
 0x49fa58 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5760 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x49f952; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5770); }
 let mut pc: u32 = 0x4a5770;
 loop { match pc {
 0x4a5770 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049fa68(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5780); }
 let mut pc: u32 = 0x4a5780;
 loop { match pc {
 0x4a5780 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a5790; continue;
 }
 0x4a5790 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a585eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a57b6; continue;
 }
 0x4a57b6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a57ce; continue;
 }
 0x4a57ce => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a57d7); f_004a57fa(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a57e0); f_004a5806(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a57e9); f_004a5813(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a57f5); f_004a5820(cpu);
  pc = 0x4a582c; continue;
 }
 0x4a582c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a583a; continue;
 }
 0x4a583a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5853; continue; }
  pc = 0x4a5844; continue;
 }
 0x4a5844 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5850); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a5853; continue;
 }
 0x4a5853 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x34u32); cpu.eax = v; }
  pc = 0x4a5868; continue;
 }
 0x4a5868 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a57fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a57fa); }
 let mut pc: u32 = 0x4a57fa;
 loop { match pc {
 0x4a57fa => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a5805); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5806(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5806); }
 let mut pc: u32 = 0x4a5806;
 loop { match pc {
 0x4a5806 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a5812); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5813(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5813); }
 let mut pc: u32 = 0x4a5813;
 loop { match pc {
 0x4a5813 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a581f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5820); }
 let mut pc: u32 = 0x4a5820;
 loop { match pc {
 0x4a5820 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x4a582b); f_00492f30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a585e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a585e); }
 let mut pc: u32 = 0x4a585e;
 loop { match pc {
 0x4a585e => {
  { let v: u32 = 0x4bd800u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5880); }
 let mut pc: u32 = 0x4a5880;
 loop { match pc {
 0x4a5880 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049a9c4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5890); }
 let mut pc: u32 = 0x4a5890;
 loop { match pc {
 0x4a5890 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049ac2e(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a58a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a58a0); }
 let mut pc: u32 = 0x4a58a0;
 loop { match pc {
 0x4a58a0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a58b0; continue;
 }
 0x4a58b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a597eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a58d6; continue;
 }
 0x4a58d6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a58ee; continue;
 }
 0x4a58ee => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a58f7); f_004a591a(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5900); f_004a5926(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5909); f_004a5933(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5915); f_004a5940(cpu);
  pc = 0x4a594c; continue;
 }
 0x4a594c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a595a; continue;
 }
 0x4a595a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5973; continue; }
  pc = 0x4a5964; continue;
 }
 0x4a5964 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x58u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5970); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a5973; continue;
 }
 0x4a5973 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x58u32); cpu.eax = v; }
  pc = 0x4a5988; continue;
 }
 0x4a5988 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a591a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a591a); }
 let mut pc: u32 = 0x4a591a;
 loop { match pc {
 0x4a591a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a5925); f_004a4030(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5926(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5926); }
 let mut pc: u32 = 0x4a5926;
 loop { match pc {
 0x4a5926 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a5932); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5933(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5933); }
 let mut pc: u32 = 0x4a5933;
 loop { match pc {
 0x4a5933 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a593f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5940); }
 let mut pc: u32 = 0x4a5940;
 loop { match pc {
 0x4a5940 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  cpu.push(0x4a594b); f_004a39b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a597e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a597e); }
 let mut pc: u32 = 0x4a597e;
 loop { match pc {
 0x4a597e => {
  { let v: u32 = 0x4bd840u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a59a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a59a0); }
 let mut pc: u32 = 0x4a59a0;
 loop { match pc {
 0x4a59a0 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049b4e7(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a59b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a59b0); }
 let mut pc: u32 = 0x4a59b0;
 loop { match pc {
 0x4a59b0 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049b7f9(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a59c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a59c0); }
 let mut pc: u32 = 0x4a59c0;
 loop { match pc {
 0x4a59c0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a59d0; continue;
 }
 0x4a59d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a59e1; continue;
 }
 0x4a59e1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a59e9); f_0049caae(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a59f7; continue;
 }
 0x4a59f7 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5a10; continue; }
  pc = 0x4a5a01; continue;
 }
 0x4a5a01 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x54u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5a0d); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a5a10; continue;
 }
 0x4a5a10 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x54u32); cpu.eax = v; }
  pc = 0x4a5a1b; continue;
 }
 0x4a5a1b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5a30); }
 let mut pc: u32 = 0x4a5a30;
 loop { match pc {
 0x4a5a30 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049cba6(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5a40); }
 let mut pc: u32 = 0x4a5a40;
 loop { match pc {
 0x4a5a40 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_0049ce47(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5a50); }
 let mut pc: u32 = 0x4a5a50;
 loop { match pc {
 0x4a5a50 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a5a60; continue;
 }
 0x4a5a60 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a5a71; continue;
 }
 0x4a5a71 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a5a79); f_004a0127(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a5a87; continue;
 }
 0x4a5a87 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5aa0; continue; }
  pc = 0x4a5a91; continue;
 }
 0x4a5a91 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x48u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5a9d); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a5aa0; continue;
 }
 0x4a5aa0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x48u32); cpu.eax = v; }
  pc = 0x4a5aab; continue;
 }
 0x4a5aab => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5ac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5ac0); }
 let mut pc: u32 = 0x4a5ac0;
 loop { match pc {
 0x4a5ac0 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_004a021f(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5ad0); }
 let mut pc: u32 = 0x4a5ad0;
 loop { match pc {
 0x4a5ad0 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_004a0482(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5ae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5ae0); }
 let mut pc: u32 = 0x4a5ae0;
 loop { match pc {
 0x4a5ae0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a5af0; continue;
 }
 0x4a5af0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a5b01; continue;
 }
 0x4a5b01 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a5b09); f_004a25b0(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a5b17; continue;
 }
 0x4a5b17 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5b30; continue; }
  pc = 0x4a5b21; continue;
 }
 0x4a5b21 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x78u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5b2d); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a5b30; continue;
 }
 0x4a5b30 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x78u32); cpu.eax = v; }
  pc = 0x4a5b3b; continue;
 }
 0x4a5b3b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5b50); }
 let mut pc: u32 = 0x4a5b50;
 loop { match pc {
 0x4a5b50 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_004a26d5(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5b60); }
 let mut pc: u32 = 0x4a5b60;
 loop { match pc {
 0x4a5b60 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  f_004a2f17(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5b70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5b70); }
 let mut pc: u32 = 0x4a5b70;
 loop { match pc {
 0x4a5b70 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a5b80; continue;
 }
 0x4a5b80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a5c1au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a5ba6; continue;
 }
 0x4a5ba6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a5bb6; continue;
 }
 0x4a5bb6 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5bbf); f_004a5bd0(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5bcb); f_004a5bdc(cpu);
  pc = 0x4a5be8; continue;
 }
 0x4a5be8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a5bf6; continue;
 }
 0x4a5bf6 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a5c0f; continue; }
  pc = 0x4a5c00; continue;
 }
 0x4a5c00 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x64u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5c0c); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a5c0f; continue;
 }
 0x4a5c0f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x64u32); cpu.eax = v; }
  pc = 0x4a5c24; continue;
 }
 0x4a5c24 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5bd0); }
 let mut pc: u32 = 0x4a5bd0;
 loop { match pc {
 0x4a5bd0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a5bdb); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5bdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5bdc); }
 let mut pc: u32 = 0x4a5bdc;
 loop { match pc {
 0x4a5bdc => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  cpu.push(0x4a5be7); f_0049caae(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5c1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5c1a); }
 let mut pc: u32 = 0x4a5c1a;
 loop { match pc {
 0x4a5c1a => {
  { let v: u32 = 0x4bd880u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5c40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5c40); }
 let mut pc: u32 = 0x4a5c40;
 loop { match pc {
 0x4a1dde => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a2466u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xd4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a1e0e); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1fde; continue;
 }
 0x4a1e28 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10u32); cpu.ecx = v; }
  cpu.push(0x4a1e3a); f_0049cba6(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a1e49; continue; }
  pc = 0x4a1e42; continue;
 }
 0x4a1e42 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2470; continue;
 }
 0x4a1e49 => {
  pc = 0x4a1ffd; continue;
 }
 0x4a1e4e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a1e57); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1e6c; continue;
 }
 0x4a1e69 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1e6c; continue;
 }
 0x4a1e6c => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x4a1fa7; continue; }
  pc = 0x4a1e78; continue;
 }
 0x4a1e78 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a1e81); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0xffffffccu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a1e94); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a1eae); f_00483464(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x4a1eb9); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a1f57; continue; }
  pc = 0x4a1ecd; continue;
 }
 0x4a1ecd => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a1ee2); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x4a1f06); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1f0f; continue;
 }
 0x4a1f0f => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1f2e; continue;
 }
 0x4a1f2e => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1f37; continue;
 }
 0x4a1f37 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1f52; continue;
 }
 0x4a1f52 => {
  pc = 0x4a1f5e; continue;
 }
 0x4a1f57 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1f5e; continue;
 }
 0x4a1f5e => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4a1f7b); cpu.call(t);
  pc = 0x4a1f8d; continue;
 }
 0x4a1f8d => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a1f96); f_004a2453(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a1fa2); f_004a245c(cpu);
  pc = 0x4a1e69; continue;
 }
 0x4a1fa7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10u32); cpu.ecx = v; }
  cpu.push(0x4a1fb9); f_0049cba6(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a1fc8; continue; }
  pc = 0x4a1fc1; continue;
 }
 0x4a1fc1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2470; continue;
 }
 0x4a1fc8 => {
  pc = 0x4a1ffd; continue;
 }
 0x4a1fcd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2470; continue;
 }
 0x4a1fde => {
  { cpu.sub32(cpu.r32(0xffffff20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a1e28; continue; }
  pc = 0x4a1feb; continue;
 }
 0x4a1feb => {
  { cpu.sub32(cpu.r32(0xffffff20u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x4a1e4e; continue; }
  pc = 0x4a1ff8; continue;
 }
 0x4a1ff8 => {
  pc = 0x4a1fcd; continue;
 }
 0x4a1ffd => {
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a2010); cpu.call(t);
  { let v: u32 = 0x5u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a201c; continue;
 }
 0x4a201c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2038; continue;
 }
 0x4a2038 => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a203f; continue;
 }
 0x4a203f => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a2048); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a20a8; continue; }
  pc = 0x4a2050; continue;
 }
 0x4a2050 => {
  pc = 0x4a2055; continue;
 }
 0x4a2055 => {
  pc = 0x4a205a; continue;
 }
 0x4a205a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f8210u32; cpu.push(v); }
  { let v = 0x4f6a10u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2074); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a209a; continue; }
  pc = 0x4a2084; continue;
 }
 0x4a2084 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x525794u32); cpu.push(0x4a209a); cpu.call(t);
  pc = 0x4a209a; continue;
 }
 0x4a209a => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a20a3); cpu.call(t);
  pc = 0x4a203f; continue;
 }
 0x4a20a8 => {
  { let v: u32 = 0x4b386cu32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a20b4; continue;
 }
 0x4a20b4 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b387cu32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a20c4; continue;
 }
 0x4a20c4 => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a20dc; continue;
 }
 0x4a20dc => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff74u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff74u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a2108); cpu.call(t);
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2111; continue;
 }
 0x4a2111 => {
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a212d; continue;
 }
 0x4a212d => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a2153); cpu.call(t);
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a215c; continue;
 }
 0x4a215c => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a217e; continue;
 }
 0x4a217e => {
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525770u32); cpu.push(0x4a218e); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525774u32); cpu.push(0x4a21ab); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a21bb; continue; }
  pc = 0x4a21b6; continue;
 }
 0x4a21b6 => {
  pc = 0x4a23f1; continue;
 }
 0x4a21bb => {
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a21c4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a21d1; continue; }
  pc = 0x4a21cc; continue;
 }
 0x4a21cc => {
  pc = 0x4a23f1; continue;
 }
 0x4a21d1 => {
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff48u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a21e0); f_004a5340(cpu);
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a21f3); f_004a5340(cpu);
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a2203); cpu.call(t);
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2212); f_004a5340(cpu);
  { let v: u8 = 0xfu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2225); f_004a5340(cpu);
  { let v: u8 = 0x10u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2229; continue;
 }
 0x4a2229 => {
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525784u32); cpu.push(0x4a223d); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a238c; continue; }
  pc = 0x4a2248; continue;
 }
 0x4a2248 => {
  pc = 0x4a224d; continue;
 }
 0x4a224d => {
  pc = 0x4a2252; continue;
 }
 0x4a2252 => {
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a2277); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a236f; continue; }
  pc = 0x4a227f; continue;
 }
 0x4a227f => {
  pc = 0x4a2284; continue;
 }
 0x4a2284 => {
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff30u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2299); f_004a4e10(cpu);
  { let v: u8 = 0x11u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff48u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52577cu32); cpu.push(0x4a22b0); cpu.call(t);
  pc = 0x4a22b5; continue;
 }
 0x4a22b5 => {
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a22c7); f_004a5260(cpu);
  pc = 0x4a22cc; continue;
 }
 0x4a22cc => {
  pc = 0x4a22d1; continue;
 }
 0x4a22d1 => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a22e9); f_004a5290(cpu);
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525780u32); cpu.push(0x4a22f5); cpu.call(t);
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525780u32); cpu.push(0x4a2301); cpu.call(t);
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525784u32); cpu.push(0x4a2312); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a234c; continue; }
  pc = 0x4a231d; continue;
 }
 0x4a231d => {
  pc = 0x4a2322; continue;
 }
 0x4a2322 => {
  { let v: u32 = 0xffffff30u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a2344); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a22d1; continue; }
  pc = 0x4a234c; continue;
 }
 0x4a234c => {
  pc = 0x4a2351; continue;
 }
 0x4a2351 => {
  { let v: u32 = 0xffffff30u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a2366); f_004a5290(cpu);
  { let v: u8 = 0x10u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a236f); f_004a23b5(cpu);
  pc = 0x4a236f; continue;
 }
 0x4a236f => {
  { let v: u32 = 0xffffff48u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a237b); cpu.call(t);
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a2387); cpu.call(t);
  pc = 0x4a2229; continue;
 }
 0x4a238c => {
  { let v: u8 = 0xfu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2395); f_004a23c1(cpu);
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a239e); f_004a23cd(cpu);
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a23a7); f_004a23d9(cpu);
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a23b0); f_004a23e5(cpu);
  pc = 0x4a23f1; continue;
 }
 0x4a23f1 => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a23fa); f_004a2408(cpu);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2403); f_004a2414(cpu);
  pc = 0x4a241d; continue;
 }
 0x4a241d => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2426); f_004a2441(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2439); f_004a244a(cpu);
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a2470; continue;
 }
 0x4a2470 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5c40 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a1dde; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5c50); }
 let mut pc: u32 = 0x4a5c50;
 loop { match pc {
 0x4a2481 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a2595u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a24ad); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4a24bc); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a24c6); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a24d9); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a24e5; continue;
 }
 0x4a24e5 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2501; continue;
 }
 0x4a2501 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2508; continue;
 }
 0x4a2508 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a2511); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a2566; continue; }
  pc = 0x4a2519; continue;
 }
 0x4a2519 => {
  pc = 0x4a251e; continue;
 }
 0x4a251e => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a2529); f_004834e0(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2538); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4a2540; continue;
 }
 0x4a2540 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a254b); f_004834fb(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2555); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a2561); cpu.call(t);
  pc = 0x4a2508; continue;
 }
 0x4a2566 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  cpu.push(0x4a2575); f_0049ce47(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2584); f_004a258c(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a259f; continue;
 }
 0x4a259f => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4a5c50 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a2481; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5c60); }
 let mut pc: u32 = 0x4a5c60;
 loop { match pc {
 0x4a5c60 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4a5c6b); f_004a5c7a(cpu);
  cpu.push(0x4a5c70); f_004a5c9c(cpu);
  pc = 0x4a5c75; continue;
 }
 0x4a5c75 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5c7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5c7a); }
 let mut pc: u32 = 0x4a5c7a;
 loop { match pc {
 0x4a5c7a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f827cu32; cpu.push(v); }
  { let v: u32 = 0x5242d0u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a5c92); cpu.call(t);
  pc = 0x4a5c97; continue;
 }
 0x4a5c97 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5c9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5c9c); }
 let mut pc: u32 = 0x4a5c9c;
 loop { match pc {
 0x4a5c9c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4a5cb9u32; cpu.push(v); }
  cpu.push(0x4a5cac); f_004ae7f0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a5cb4; continue;
 }
 0x4a5cb4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5cb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5cb9); }
 let mut pc: u32 = 0x4a5cb9;
 loop { match pc {
 0x4a5cb9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5242d0u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a5cca); cpu.call(t);
  pc = 0x4a5ccf; continue;
 }
 0x4a5ccf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5cd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5cd4); }
 let mut pc: u32 = 0x4a5cd4;
 loop { match pc {
 0x4a5cd4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4a5cdf); f_004a5cee(cpu);
  cpu.push(0x4a5ce4); f_004a5d36(cpu);
  pc = 0x4a5ce9; continue;
 }
 0x4a5ce9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5cee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5cee); }
 let mut pc: u32 = 0x4a5cee;
 loop { match pc {
 0x4a5cee => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f828cu32; cpu.push(v); }
  { let v: u32 = 0x5242e0u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a5d06); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f8298u32; cpu.push(v); }
  { let v: u32 = 0x5242e8u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a5d19); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f82a8u32; cpu.push(v); }
  { let v: u32 = 0x5242f0u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a5d2c); cpu.call(t);
  pc = 0x4a5d31; continue;
 }
 0x4a5d31 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5d36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5d36); }
 let mut pc: u32 = 0x4a5d36;
 loop { match pc {
 0x4a5d36 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4a5d53u32; cpu.push(v); }
  cpu.push(0x4a5d46); f_004ae7f0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a5d4e; continue;
 }
 0x4a5d4e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5d53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5d53); }
 let mut pc: u32 = 0x4a5d53;
 loop { match pc {
 0x4a5d53 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x52571cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x5242e0u32; cpu.push(v); }
  cpu.push(0x4a5d6d); f_004aeb10(cpu);
  pc = 0x4a5d72; continue;
 }
 0x4a5d72 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5d77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5d77); }
 let mut pc: u32 = 0x4a5d77;
 loop { match pc {
 0x4a5d77 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4a5d82); f_004a5d91(cpu);
  cpu.push(0x4a5d87); f_004a5db3(cpu);
  pc = 0x4a5d8c; continue;
 }
 0x4a5d8c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5d91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5d91); }
 let mut pc: u32 = 0x4a5d91;
 loop { match pc {
 0x4a5d91 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f82b4u32; cpu.push(v); }
  { let v: u32 = 0x5242c8u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a5da9); cpu.call(t);
  pc = 0x4a5dae; continue;
 }
 0x4a5dae => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5db3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5db3); }
 let mut pc: u32 = 0x4a5db3;
 loop { match pc {
 0x4a5db3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4a5dd0u32; cpu.push(v); }
  cpu.push(0x4a5dc3); f_004ae7f0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a5dcb; continue;
 }
 0x4a5dcb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5dd0); }
 let mut pc: u32 = 0x4a5dd0;
 loop { match pc {
 0x4a5dd0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5242d8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x4a5dfe; continue; }
  pc = 0x4a5de5; continue;
 }
 0x4a5de5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5242d8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5242d8u32, v); }
  { let v: u32 = 0x5242c8u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a5dfe); cpu.call(t);
  pc = 0x4a5dfe; continue;
 }
 0x4a5dfe => {
  pc = 0x4a5e03; continue;
 }
 0x4a5e03 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5e08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5e08); }
 let mut pc: u32 = 0x4a5e08;
 loop { match pc {
 0x4a5e08 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5242c8u32; cpu.eax = v; }
  pc = 0x4a5e1e; continue;
 }
 0x4a5e1e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a5e23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a5e23); }
 let mut pc: u32 = 0x4a5e23;
 loop { match pc {
 0x4a5e23 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a6084u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xffffff6cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x90u32); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1du32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a5e88); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x84u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5e9e); f_0049cfb4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a5ec1; continue; }
  pc = 0x4a5ea6; continue;
 }
 0x4a5ea6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5eb9); f_004a607b(cpu);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a608e; continue;
 }
 0x4a5ec1 => {
  pc = 0x4a5ec6; continue;
 }
 0x4a5ec6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x525768u32); cpu.push(0x4a5ed8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a5ef2; continue; }
  pc = 0x4a5ee0; continue;
 }
 0x4a5ee0 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x8fffffffu32; cpu.w32(0xffffff78u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a600a; continue;
 }
 0x4a5ef2 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x525724u32); cpu.push(0x4a5efe); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4fa488u32); cpu.push(0x4a5f0b); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x4a5fe6; continue; }
  pc = 0x4a5f1b; continue;
 }
 0x4a5f1b => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f82c4u32; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a5f2b); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a5f34; continue;
 }
 0x4a5f34 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f82ecu32; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a5f44); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x4a5f5d); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a5f65; continue;
 }
 0x4a5f65 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x4a5f74); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5f7d); f_004a605e(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a5f88); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a5f90; continue;
 }
 0x4a5f90 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f82f8u32; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a5fa0); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x4a5fb6); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a5fbe; continue;
 }
 0x4a5fbe => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x4a5fcd); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5fd6); f_004a6054(cpu);
  { let v: u32 = 0x8fffffffu32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a5fe6); f_004a6068(cpu);
  pc = 0x4a5fe6; continue;
 }
 0x4a5fe6 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a5ffb); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a600a; continue;
 }
 0x4a600a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6019; continue;
 }
 0x4a6016 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6019; continue;
 }
 0x4a6019 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3u32); }
  if !cpu.cf { pc = 0x4a6039; continue; }
  pc = 0x4a6023; continue;
 }
 0x4a6023 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffff60u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  pc = 0x4a6016; continue;
 }
 0x4a6039 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a604c); f_004a607b(cpu);
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a608e; continue;
 }
 0x4a608e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6054); }
 let mut pc: u32 = 0x4a6054;
 loop { match pc {
 0x4a6054 => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a605d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a605e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a605e); }
 let mut pc: u32 = 0x4a605e;
 loop { match pc {
 0x4a605e => {
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a6067); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6068(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6068); }
 let mut pc: u32 = 0x4a6068;
 loop { match pc {
 0x4a6068 => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a6071); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6072(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6072); }
 let mut pc: u32 = 0x4a6072;
 loop { match pc {
 0x4a6072 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a607a); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a607b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a607b); }
 let mut pc: u32 = 0x4a607b;
 loop { match pc {
 0x4a607b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a6083); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6084(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6084); }
 let mut pc: u32 = 0x4a6084;
 loop { match pc {
 0x4a6084 => {
  { let v: u32 = 0x4bd8b0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6114(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6114); }
 let mut pc: u32 = 0x4a6114;
 loop { match pc {
 0x4a6114 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74c8u32; cpu.push(v); }
  { let v = 0x4f74f8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a6137); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a6162; continue; }
  pc = 0x4a6147; continue;
 }
 0x4a6147 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xd0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x90u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4fa504u32); cpu.push(0x4a615f); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4a6162; continue;
 }
 0x4a6162 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x84u32); cpu.ecx = v; }
  cpu.push(0x4a6174); f_0049d426(cpu);
  pc = 0x4a6179; continue;
 }
 0x4a6179 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6180); }
 let mut pc: u32 = 0x4a6180;
 loop { match pc {
 0x4a6180 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74c8u32; cpu.push(v); }
  { let v = 0x4f74f8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a61a3); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a61ce; continue; }
  pc = 0x4a61b3; continue;
 }
 0x4a61b3 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xd0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x90u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4f8528u32); cpu.push(0x4a61cb); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4a61ce; continue;
 }
 0x4a61ce => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x84u32); cpu.ecx = v; }
  cpu.push(0x4a61e0); f_0049d4f3(cpu);
  pc = 0x4a61e5; continue;
 }
 0x4a61e5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a61ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a61ec); }
 let mut pc: u32 = 0x4a61ec;
 loop { match pc {
 0x4a61ec => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffff78u32.wrapping_add(cpu.eax)), 0x8fffffffu32); }
  if cpu.zf { pc = 0x4a6244; continue; }
  pc = 0x4a620b; continue;
 }
 0x4a620b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffff6cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a6232; continue; }
  pc = 0x4a621b; continue;
 }
 0x4a621b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x90u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4fa4a8u32); cpu.push(0x4a622a); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a6244; continue;
 }
 0x4a6232 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x90u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4fa4ccu32); cpu.push(0x4a6241); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a6244; continue;
 }
 0x4a6244 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x84u32); cpu.ecx = v; }
  cpu.push(0x4a6252); f_0049da33(cpu);
  pc = 0x4a6257; continue;
 }
 0x4a6257 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a625c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a625c); }
 let mut pc: u32 = 0x4a625c;
 loop { match pc {
 0x4a625c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x84u32); cpu.ecx = v; }
  cpu.push(0x4a627a); f_0049db5c(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffea4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffea4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffea4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xfffffea4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x4a62a9); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a6430; continue; }
  pc = 0x4a62b1; continue;
 }
 0x4a62b1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a62bd); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a62c8); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a62da); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a62e5); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a62f7); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a6302); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6314); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a631f); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6331); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a633c); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ecx), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a634e); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a6359); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a636b); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a6376); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6388); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a6393); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a63a5); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a63b0); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ecx), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a63c2); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a63cd); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a63df); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a63ea); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff84u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a63fc); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a6407); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a658c; continue;
 }
 0x4a6430 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a643c); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a6447); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6459); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a6464); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6476); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a6481); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6493); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a649e); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a64b0); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a64bb); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ecx), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a64cd); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a64d8); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a64ea); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a64f5); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6507); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a6512); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6524); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a652f); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ecx), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a6541); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a654c); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a655e); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a6569); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a657b); f_00482a83(cpu);
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a6586); f_004ae7b6(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a658c; continue;
 }
 0x4a658c => {
  pc = 0x4a6591; continue;
 }
 0x4a6591 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6598(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6598); }
 let mut pc: u32 = 0x4a6598;
 loop { match pc {
 0x4a6598 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xfffffeacu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x4a65d3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a65e5; continue; }
  pc = 0x4a65db; continue;
 }
 0x4a65db => {
  { let v: u32 = cpu.r32(0x4b17c8u32); cpu.eax = v; }
  pc = 0x4a664c; continue;
 }
 0x4a65e5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a65f4; continue;
 }
 0x4a65f1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a65f4; continue;
 }
 0x4a65f4 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x3u32); }
  if !cpu.cf { pc = 0x4a6642; continue; }
  pc = 0x4a65fe; continue;
 }
 0x4a65fe => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x5242e0u32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4a6614); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a6621; continue; }
  pc = 0x4a661c; continue;
 }
 0x4a661c => {
  pc = 0x4a6626; continue;
 }
 0x4a6621 => {
  pc = 0x4a663d; continue;
 }
 0x4a6626 => {
  pc = 0x4a6635; continue;
 }
 0x4a6635 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a664c; continue;
 }
 0x4a663d => {
  pc = 0x4a65f1; continue;
 }
 0x4a6642 => {
  { let v: u32 = cpu.r32(0x4b17c8u32); cpu.eax = v; }
  pc = 0x4a664c; continue;
 }
 0x4a664c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6653(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6653); }
 let mut pc: u32 = 0x4a6653;
 loop { match pc {
 0x4a6653 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xfffffeacu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x4a668e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a669d; continue; }
  pc = 0x4a6696; continue;
 }
 0x4a6696 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a6753; continue;
 }
 0x4a669d => {
  { let v: u32 = cpu.r32(0x4b17c8u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.zf { pc = 0x4a66b2; continue; }
  pc = 0x4a66ab; continue;
 }
 0x4a66ab => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a6753; continue;
 }
 0x4a66b2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xffffff68u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x4a6739; continue; }
  pc = 0x4a66c6; continue;
 }
 0x4a66c6 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a66e5); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4a66f2); cpu.call(t);
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.cf { pc = 0x4a6706; continue; }
  pc = 0x4a66fa; continue;
 }
 0x4a66fa => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a6753; continue;
 }
 0x4a6706 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xfffffeacu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x4a6739); cpu.call(t);
  pc = 0x4a6739; continue;
 }
 0x4a6739 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff68u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a6753; continue;
 }
 0x4a6753 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a675a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a675a); }
 let mut pc: u32 = 0x4a675a;
 loop { match pc {
 0x4a675a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4b17c8u32), cpu.eax); }
  if cpu.zf { pc = 0x4a67bb; continue; }
  pc = 0x4a6775; continue;
 }
 0x4a6775 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xffffff68u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x4a67bb; continue; }
  pc = 0x4a6789; continue;
 }
 0x4a6789 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.push(0x4a67aa); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffff68u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  pc = 0x4a67bb; continue;
 }
 0x4a67bb => {
  pc = 0x4a67c0; continue;
 }
 0x4a67c0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a67c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a67c7); }
 let mut pc: u32 = 0x4a67c7;
 loop { match pc {
 0x4a67c7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4b17c8u32), cpu.eax); }
  if cpu.zf { pc = 0x4a67f9; continue; }
  pc = 0x4a67e2; continue;
 }
 0x4a67e2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  pc = 0x4a6800; continue;
 }
 0x4a67f9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a6800; continue;
 }
 0x4a6800 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6807(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6807); }
 let mut pc: u32 = 0x4a6807;
 loop { match pc {
 0x4a6807 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a6f1du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffee8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xfffffeacu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x4a6869); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a6876; continue; }
  pc = 0x4a6871; continue;
 }
 0x4a6871 => {
  pc = 0x4a6f27; continue;
 }
 0x4a6876 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xfffffeacu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x4a68b1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a68d4); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a68f4; continue;
 }
 0x4a68f4 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffff68u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a6c4e; continue; }
  pc = 0x4a6907; continue;
 }
 0x4a6907 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.push(0x4a6952); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x158u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x4a6972; continue; }
  pc = 0x4a6963; continue;
 }
 0x4a6963 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6998; continue;
 }
 0x4a6972 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x158u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6998; continue;
 }
 0x4a6998 => {
  { let v = 0x5242e0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.edx)); cpu.push(0x4a69c5); cpu.call(t);
  { cpu.sub32(cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a6bfa; continue; }
  pc = 0x4a69d2; continue;
 }
 0x4a69d2 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xfffffeacu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.push(0x4a6a0d); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff44u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff44u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a6a2f; continue; }
  pc = 0x4a6a20; continue;
 }
 0x4a6a20 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff10u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6a4a; continue;
 }
 0x4a6a2f => {
  { let v: u32 = cpu.r32(0xffffff44u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffff44u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff10u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6a4a; continue;
 }
 0x4a6a4a => {
  { let v: u32 = cpu.r32(0xffffff10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff58u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a6a9b; continue; }
  pc = 0x4a6a63; continue;
 }
 0x4a6a63 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4c0288u32; cpu.push(v); }
  { let v = 0x4c0208u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5242b8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a6a87); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a6a8d); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6aa5; continue;
 }
 0x4a6a9b => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6aa5; continue;
 }
 0x4a6aa5 => {
  pc = 0x4a6aaa; continue;
 }
 0x4a6aaa => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a6ac3; continue; }
  pc = 0x4a6abe; continue;
 }
 0x4a6abe => {
  pc = 0x4a6ac8; continue;
 }
 0x4a6ac3 => {
  pc = 0x4a6b77; continue;
 }
 0x4a6ac8 => {
  pc = 0x4a6ada; continue;
 }
 0x4a6ada => {
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff0cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6aeb; continue;
 }
 0x4a6aeb => {
  { let v: u32 = cpu.r32(0xffffff0cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff08u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff08u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff0cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a6b08); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a6b1e; continue; }
  pc = 0x4a6b10; continue;
 }
 0x4a6b10 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6b1e; continue;
 }
 0x4a6b1e => {
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff04u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6b2f; continue;
 }
 0x4a6b2f => {
  { let v: u32 = cpu.r32(0xffffff04u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff00u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff4cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff00u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff04u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4a6b53); cpu.call(t);
  { let v = cpu.rf32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffff50u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffff54u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4a6b77; continue;
 }
 0x4a6b77 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.push(0x4a6b94); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff38u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.push(0x4a6bb2); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffef0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffef0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffef0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffef0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4a6be9; continue;
 }
 0x4a6be9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a6bf5); f_004a6f11(cpu);
  pc = 0x4a6c49; continue;
 }
 0x4a6bfa => {
  { let v: u32 = 0xffffff2cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x128u32); cpu.ecx = v; }
  cpu.push(0x4a6c12); f_004828cd(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffeecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffeecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffeecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffeecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4a6c49; continue;
 }
 0x4a6c49 => {
  pc = 0x4a6c79; continue;
 }
 0x4a6c4e => {
  { let v: u32 = 0xffffff20u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x128u32); cpu.ecx = v; }
  cpu.push(0x4a6c66); f_004828cd(cpu);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a6c79; continue;
 }
 0x4a6c79 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a6c86); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffff6cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a6d63; continue; }
  pc = 0x4a6c99; continue;
 }
 0x4a6c99 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.push(0x4a6ce4); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x158u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x4a6d04; continue; }
  pc = 0x4a6cf5; continue;
 }
 0x4a6cf5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffefcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6d2a; continue;
 }
 0x4a6d04 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x158u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffefcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6d2a; continue;
 }
 0x4a6d2a => {
  { let v: u32 = 0x5242e0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffefcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.edx)); cpu.push(0x4a6d5e); cpu.call(t);
  pc = 0x4a6d87; continue;
 }
 0x4a6d63 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x128u32); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a6d87); f_004828c1(cpu);
  pc = 0x4a6d87; continue;
 }
 0x4a6d87 => {
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a6da3); f_004828ff(cpu);
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xfffffeacu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.edx)); cpu.push(0x4a6de2); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffff70u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a6efc; continue; }
  pc = 0x4a6df5; continue;
 }
 0x4a6df5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.push(0x4a6e40); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x158u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x4a6e60; continue; }
  pc = 0x4a6e51; continue;
 }
 0x4a6e51 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffef8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6e86; continue;
 }
 0x4a6e60 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffeacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x158u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffef8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6e86; continue;
 }
 0x4a6e86 => {
  { let v: u32 = 0x5242e0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffef8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff48u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.edx)); cpu.push(0x4a6eba); cpu.call(t);
  { let v = cpu.rf32(0xffffff48u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a6ee7; continue; }
  pc = 0x4a6ed2; continue;
 }
 0x4a6ed2 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xffffff74u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a6ef7; continue;
 }
 0x4a6ee7 => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffff74u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a6ef7; continue;
 }
 0x4a6ef7 => {
  pc = 0x4a6f0c; continue;
 }
 0x4a6efc => {
  { let v: u32 = cpu.r32(0xfffffee8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xffffff74u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a6f0c; continue;
 }
 0x4a6f0c => {
  pc = 0x4a6f27; continue;
 }
 0x4a6f27 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6f11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6f11); }
 let mut pc: u32 = 0x4a6f11;
 loop { match pc {
 0x4a6f11 => {
  { let v: u32 = 0xffffff58u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a6f1c); f_0042e680(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6f1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6f1d); }
 let mut pc: u32 = 0x4a6f1d;
 loop { match pc {
 0x4a6f1d => {
  { let v: u32 = 0x4bd8f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6fa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6fa4); }
 let mut pc: u32 = 0x4a6fa4;
 loop { match pc {
 0x4a6fa4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a6fb5; continue;
 }
 0x4a6fb5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6fba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6fba); }
 let mut pc: u32 = 0x4a6fba;
 loop { match pc {
 0x4a6fba => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xd0u32); cpu.eax = v; }
  pc = 0x4a6fd3; continue;
 }
 0x4a6fd3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a6fd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a6fd8); }
 let mut pc: u32 = 0x4a6fd8;
 loop { match pc {
 0x4a6fd8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a70e2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a702e; continue; }
  pc = 0x4a700a; continue;
 }
 0x4a700a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3900u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x4a7023; continue;
 }
 0x4a7023 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a702e; continue;
 }
 0x4a702e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7038); f_0049a526(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x4a704b); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3924u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b390cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x38u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v = 0x4f8304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4a70a2); cpu.call(t);
  pc = 0x4a70a7; continue;
 }
 0x4a70a7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a70ec; continue;
 }
 0x4a70ec => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a70b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a70b6); }
 let mut pc: u32 = 0x4a70b6;
 loop { match pc {
 0x4a70b6 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a70c2); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a70c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a70c3); }
 let mut pc: u32 = 0x4a70c3;
 loop { match pc {
 0x4a70c3 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a70cb); f_004a3800(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a70cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a70cc); }
 let mut pc: u32 = 0x4a70cc;
 loop { match pc {
 0x4a70cc => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a70e1; continue; }
  pc = 0x4a70d6; continue;
 }
 0x4a70d6 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x3cu32); cpu.ecx = v; }
  cpu.push(0x4a70e1); f_00485370(cpu);
  pc = 0x4a70e1; continue;
 }
 0x4a70e1 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a70e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a70e2); }
 let mut pc: u32 = 0x4a70e2;
 loop { match pc {
 0x4a70e2 => {
  { let v: u32 = 0x4bd920u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a71bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a71bb); }
 let mut pc: u32 = 0x4a71bb;
 loop { match pc {
 0x4a71bb => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a71c4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a71c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a71c5); }
 let mut pc: u32 = 0x4a71c5;
 loop { match pc {
 0x4a71c5 => {
  { let v: u32 = 0x4bd958u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7228(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7228); }
 let mut pc: u32 = 0x4a7228;
 loop { match pc {
 0x4a7228 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a7523u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x48u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9cu32.wrapping_add(cpu.eax), v); }
  { let v = 0x40u32; cpu.push(v); }
  cpu.push(0x4a7264); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a728a; continue; }
  pc = 0x4a7278; continue;
 }
 0x4a7278 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7282); f_004a6fd8(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7291; continue;
 }
 0x4a728a => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7291; continue;
 }
 0x4a7291 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524080u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4a72ac); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a72c2; continue; }
  pc = 0x4a72bc; continue;
 }
 0x4a72bc => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a72c2; continue;
 }
 0x4a72c2 => {
  pc = 0x4a72c7; continue;
 }
 0x4a72c7 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x4a72ee); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a730f; continue; }
  pc = 0x4a7303; continue;
 }
 0x4a7303 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7321; continue;
 }
 0x4a730f => {
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7321; continue;
 }
 0x4a7321 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74f8u32; cpu.push(v); }
  { let v = 0x4c27c8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7345); f_00405cb0(cpu);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4a7356); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a735c); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x98u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a739c; continue; }
  pc = 0x4a7378; continue;
 }
 0x4a7378 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a7388); f_004a7504(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a7394); f_004a751a(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a752d; continue;
 }
 0x4a739c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a73b6); f_00405cb0(cpu);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x4a73d4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a7477; continue; }
  pc = 0x4a73dc; continue;
 }
 0x4a73dc => {
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x4a73ff); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x98u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a7426; continue; }
  pc = 0x4a741a; continue;
 }
 0x4a741a => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7444; continue;
 }
 0x4a7426 => {
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x98u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7444; continue;
 }
 0x4a7444 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x4a7453); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a7463); f_004a7504(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a746f); f_004a751a(cpu);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a752d; continue;
 }
 0x4a7477 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7489); f_00405cb0(cpu);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a7499); f_0049ed2c(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a74c5; continue; }
  pc = 0x4a74a1; continue;
 }
 0x4a74a1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a74b1); f_004a7504(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a74bd); f_004a751a(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a752d; continue;
 }
 0x4a74c5 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a74d5); f_004a7504(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a74e1); f_004a751a(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a752d; continue;
 }
 0x4a752d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a74e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a74e9); }
 let mut pc: u32 = 0x4a74e9;
 loop { match pc {
 0x4a74e9 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a74f1); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a74f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a74f2); }
 let mut pc: u32 = 0x4a74f2;
 loop { match pc {
 0x4a74f2 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a74fa); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a74fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a74fb); }
 let mut pc: u32 = 0x4a74fb;
 loop { match pc {
 0x4a74fb => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7503); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7504(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7504); }
 let mut pc: u32 = 0x4a7504;
 loop { match pc {
 0x4a7504 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a750c); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a750d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a750d); }
 let mut pc: u32 = 0x4a750d;
 loop { match pc {
 0x4a750d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a7516); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a751a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a751a); }
 let mut pc: u32 = 0x4a751a;
 loop { match pc {
 0x4a751a => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a7522); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7523(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7523); }
 let mut pc: u32 = 0x4a7523;
 loop { match pc {
 0x4a7523 => {
  { let v: u32 = 0x4bd980u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a76bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a76bf); }
 let mut pc: u32 = 0x4a76bf;
 loop { match pc {
 0x4a76bf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a77a2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a7715; continue; }
  pc = 0x4a76f1; continue;
 }
 0x4a76f1 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3938u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a770a; continue;
 }
 0x4a770a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7715; continue;
 }
 0x4a7715 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a771f); f_0049ffd8(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b395cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3944u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v = 0x4f8310u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4a776f); cpu.call(t);
  pc = 0x4a7774; continue;
 }
 0x4a7774 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a77ac; continue;
 }
 0x4a77ac => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7783(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7783); }
 let mut pc: u32 = 0x4a7783;
 loop { match pc {
 0x4a7783 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a778b); f_004ad990(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a778c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a778c); }
 let mut pc: u32 = 0x4a778c;
 loop { match pc {
 0x4a778c => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a77a1; continue; }
  pc = 0x4a7796; continue;
 }
 0x4a7796 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x48u32); cpu.ecx = v; }
  cpu.push(0x4a77a1); f_00485370(cpu);
  pc = 0x4a77a1; continue;
 }
 0x4a77a1 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a77a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a77a2); }
 let mut pc: u32 = 0x4a77a2;
 loop { match pc {
 0x4a77a2 => {
  { let v: u32 = 0x4bd9d0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a786c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a786c); }
 let mut pc: u32 = 0x4a786c;
 loop { match pc {
 0x4a786c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xb4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a788f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a789c; continue; }
  pc = 0x4a7897; continue;
 }
 0x4a7897 => {
  pc = 0x4a7a4e; continue;
 }
 0x4a789c => {
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x30u32); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a78b7); f_004828c1(cpu);
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff50u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a78c7); f_00482b36(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a78e2); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a78ef); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a78fe); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a790b); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a791a); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a7927); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a7936); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a7943); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a7952); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a795f); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a796e); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a797b); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a798a); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a7997); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a79a6); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a79b3); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a79c2); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38e8u32); cpu.set_st(0, v); }
  cpu.push(0x4a79cf); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  cpu.push(0x4a79e4); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a79f1); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  cpu.push(0x4a7a06); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a7a13); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  cpu.push(0x4a7a28); f_00482a72(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38ecu32); cpu.set_st(0, v); }
  cpu.push(0x4a7a35); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4fa4b0u32); cpu.push(0x4a7a46); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x4a7a4e; continue;
 }
 0x4a7a4e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7a53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7a53); }
 let mut pc: u32 = 0x4a7a53;
 loop { match pc {
 0x4a7a53 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7a64; continue;
 }
 0x4a7a64 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xfffffffdu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x78u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = 0xfffffffdu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7a98); cpu.call(t);
  pc = 0x4a7a9d; continue;
 }
 0x4a7a9d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7aa2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7aa2); }
 let mut pc: u32 = 0x4a7aa2;
 loop { match pc {
 0x4a7aa2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a7b6bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7ae0); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a7af0); f_0049ae07(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a7b13; continue; }
  pc = 0x4a7af8; continue;
 }
 0x4a7af8 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a7b0b); f_004a7b62(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a7b75; continue;
 }
 0x4a7b13 => {
  pc = 0x4a7b18; continue;
 }
 0x4a7b18 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4a7b2c); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a7b51); f_004a7b62(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a7b75; continue;
 }
 0x4a7b75 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7b59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7b59); }
 let mut pc: u32 = 0x4a7b59;
 loop { match pc {
 0x4a7b59 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7b61); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7b62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7b62); }
 let mut pc: u32 = 0x4a7b62;
 loop { match pc {
 0x4a7b62 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a7b6a); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7b6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7b6b); }
 let mut pc: u32 = 0x4a7b6b;
 loop { match pc {
 0x4a7b6b => {
  { let v: u32 = 0x4bda00u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7b86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7b86); }
 let mut pc: u32 = 0x4a7b86;
 loop { match pc {
 0x4a7b86 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74a0u32; cpu.push(v); }
  { let v = 0x4c27e8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x60u32); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a7baf); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a7bb5); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4a7bca); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7bde); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a7c2d; continue; }
  pc = 0x4a7bed; continue;
 }
 0x4a7bed => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7bfe); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a7c19; continue; }
  pc = 0x4a7c0d; continue;
 }
 0x4a7c0d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)).wrapping_sub(0xcu32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a7bed; continue;
 }
 0x4a7c19 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a7c88; continue;
 }
 0x4a7c2d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7c3e); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a7c88; continue; }
  pc = 0x4a7c4d; continue;
 }
 0x4a7c4d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7c5e); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a7c79; continue; }
  pc = 0x4a7c6d; continue;
 }
 0x4a7c6d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)).wrapping_add(0xcu32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a7c4d; continue;
 }
 0x4a7c79 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a7c88; continue;
 }
 0x4a7c88 => {
  pc = 0x4a7c8d; continue;
 }
 0x4a7c8d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7c94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7c94); }
 let mut pc: u32 = 0x4a7c94;
 loop { match pc {
 0x4a7c94 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7cb1); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7cc5); cpu.call(t);
  { let v = cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74a0u32; cpu.push(v); }
  { let v = 0x4c27e8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a7ce4); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a7cea); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4a7cff); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7d13); cpu.call(t);
  { let v = cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x4a7d39; continue; }
  pc = 0x4a7d2e; continue;
 }
 0x4a7d2e => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  pc = 0x4a7d3f; continue;
 }
 0x4a7d39 => {
  { let v = cpu.rf32(0x4b38f0u32); cpu.fpush(v); }
  pc = 0x4a7d3f; continue;
 }
 0x4a7d3f => {
  pc = 0x4a7d44; continue;
 }
 0x4a7d44 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7d49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7d49); }
 let mut pc: u32 = 0x4a7d49;
 loop { match pc {
 0x4a7d49 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7d5d); f_004a7c94(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7d71); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a7d85); cpu.call(t);
  { let v = cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38e0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a7dc8; continue; }
  pc = 0x4a7d9a; continue;
 }
 0x4a7d9a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a7dfe; continue;
 }
 0x4a7dc8 => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f4u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x4a7df0; continue; }
  pc = 0x4a7ddd; continue;
 }
 0x4a7ddd => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a7dfe; continue;
 }
 0x4a7df0 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a7dfe; continue;
 }
 0x4a7dfe => {
  pc = 0x4a7e03; continue;
 }
 0x4a7e03 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7e0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7e0a); }
 let mut pc: u32 = 0x4a7e0a;
 loop { match pc {
 0x4a7e0a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a8036u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a7e63; continue; }
  pc = 0x4a7e3c; continue;
 }
 0x4a7e3c => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3970u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a7e58; continue;
 }
 0x4a7e58 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7e63; continue;
 }
 0x4a7e63 => {
  pc = 0x4a7e8c; continue;
 }
 0x4a7e8c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7e96); f_0049b35e(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a7ea9); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a7ebc; continue;
 }
 0x4a7ebc => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7edb; continue;
 }
 0x4a7edb => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a7eeb); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38c4u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a7efe; continue;
 }
 0x4a7efe => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7f1d; continue;
 }
 0x4a7f1d => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38a4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b388cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x4a7f56; continue;
 }
 0x4a7f56 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a7f9c; continue;
 }
 0x4a7f9c => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3994u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b397cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x74u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x78u32.wrapping_add(cpu.eax), v); }
  { let v = 0x4f831cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4a8000); cpu.call(t);
  pc = 0x4a8005; continue;
 }
 0x4a8005 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a8040; continue;
 }
 0x4a8040 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7ece(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7ece); }
 let mut pc: u32 = 0x4a7ece;
 loop { match pc {
 0x4a7ece => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a7eda); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7f10); }
 let mut pc: u32 = 0x4a7f10;
 loop { match pc {
 0x4a7f10 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a7f1c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7f65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7f65); }
 let mut pc: u32 = 0x4a7f65;
 loop { match pc {
 0x4a7f65 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x4a7f70); f_004a44b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7f71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7f71); }
 let mut pc: u32 = 0x4a7f71;
 loop { match pc {
 0x4a7f71 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x4a7f7c); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7f7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7f7d); }
 let mut pc: u32 = 0x4a7f7d;
 loop { match pc {
 0x4a7f7d => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7f85); f_004ad9f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a7f86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a7f86); }
 let mut pc: u32 = 0x4a7f86;
 loop { match pc {
 0x4a7f86 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4a7f9b; continue; }
  pc = 0x4a7f90; continue;
 }
 0x4a7f90 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x78u32); cpu.ecx = v; }
  cpu.push(0x4a7f9b); f_00485370(cpu);
  pc = 0x4a7f9b; continue;
 }
 0x4a7f9b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8014(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8014); }
 let mut pc: u32 = 0x4a8014;
 loop { match pc {
 0x4a8014 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a801c); f_004ad9c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a801d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a801d); }
 let mut pc: u32 = 0x4a801d;
 loop { match pc {
 0x4a801d => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a8035; continue; }
  pc = 0x4a8027; continue;
 }
 0x4a8027 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x80u32); cpu.ecx = v; }
  cpu.push(0x4a8035); f_00485370(cpu);
  pc = 0x4a8035; continue;
 }
 0x4a8035 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8036(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8036); }
 let mut pc: u32 = 0x4a8036;
 loop { match pc {
 0x4a8036 => {
  { let v: u32 = 0x4bda30u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8051(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8051); }
 let mut pc: u32 = 0x4a8051;
 loop { match pc {
 0x4a8051 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a810cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3994u32; cpu.w32(0xffffff80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b397cu32; cpu.w32(0xffffff84u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a80ea; continue; }
  pc = 0x4a80c0; continue;
 }
 0x4a80c0 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a80ea; continue; }
  pc = 0x4a80d9; continue;
 }
 0x4a80d9 => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a80e5); cpu.call(t);
  pc = 0x4a80ea; continue;
 }
 0x4a80ea => {
  pc = 0x4a80ef; continue;
 }
 0x4a80ef => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a80fb); f_004a8100(cpu);
  pc = 0x4a8116; continue;
 }
 0x4a8116 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8100); }
 let mut pc: u32 = 0x4a8100;
 loop { match pc {
 0x4a8100 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a810b); f_004a25b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a810c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a810c); }
 let mut pc: u32 = 0x4a810c;
 loop { match pc {
 0x4a810c => {
  { let v: u32 = 0x4bda90u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8124(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8124); }
 let mut pc: u32 = 0x4a8124;
 loop { match pc {
 0x4a8124 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a829fu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4a814e); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a8288; continue; }
  pc = 0x4a8161; continue;
 }
 0x4a8161 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a817c); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a81c7; continue; }
  pc = 0x4a8193; continue;
 }
 0x4a8193 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4adb80u32; cpu.push(v); }
  { let v = 0x4a86c7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a81b9); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a81ce; continue;
 }
 0x4a81c7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a81ce; continue;
 }
 0x4a81ce => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x78u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a81f2; continue; }
  pc = 0x4a81eb; continue;
 }
 0x4a81eb => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a82a9; continue;
 }
 0x4a81f2 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a81fb; continue;
 }
 0x4a81fb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4a8204); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a8288; continue; }
  pc = 0x4a820c; continue;
 }
 0x4a820c => {
  pc = 0x4a8211; continue;
 }
 0x4a8211 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8222; continue;
 }
 0x4a8222 => {
  pc = 0x4a8227; continue;
 }
 0x4a8227 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a8244; continue;
 }
 0x4a8244 => {
  pc = 0x4a8249; continue;
 }
 0x4a8249 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a8276; continue; }
  pc = 0x4a8265; continue;
 }
 0x4a8265 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a8271); cpu.call(t);
  pc = 0x4a8276; continue;
 }
 0x4a8276 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x4a827f); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0xcu32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a81fb; continue;
 }
 0x4a8288 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a82a9; continue;
 }
 0x4a82a9 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8292(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8292); }
 let mut pc: u32 = 0x4a8292;
 loop { match pc {
 0x4a8292 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a829b); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a829f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a829f); }
 let mut pc: u32 = 0x4a829f;
 loop { match pc {
 0x4a829f => {
  { let v: u32 = 0x4bdab8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a82ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a82ba); }
 let mut pc: u32 = 0x4a82ba;
 loop { match pc {
 0x4a82ba => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a82d1; continue;
 }
 0x4a82d1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
