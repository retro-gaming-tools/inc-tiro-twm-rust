#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_1300a78a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a78a); }
 let mut pc: u32 = 0x1300a78a;
 loop { match pc {
 0x1300a78a => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300a79b; continue; }
  pc = 0x1300a790; continue;
 }
 0x1300a790 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300a79b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a79c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a79c); }
 let mut pc: u32 = 0x1300a79c;
 loop { match pc {
 0x1300a79c => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a7a5); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a7a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a7a9); }
 let mut pc: u32 = 0x1300a7a9;
 loop { match pc {
 0x1300a7a9 => {
  { let v: u32 = 0x1300d660u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a7b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a7b3); }
 let mut pc: u32 = 0x1300a7b3;
 loop { match pc {
 0x1300a7b3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a7c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a7c0); }
 let mut pc: u32 = 0x1300a7c0;
 loop { match pc {
 0x1300a7c0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300a805u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c124u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1300a7f8); f_1300a80f(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a805(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a805); }
 let mut pc: u32 = 0x1300a805;
 loop { match pc {
 0x1300a805 => {
  { let v: u32 = 0x1300d698u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a80f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a80f); }
 let mut pc: u32 = 0x1300a80f;
 loop { match pc {
 0x1300a80f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13005c60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a820); }
 let mut pc: u32 = 0x1300a820;
 loop { match pc {
 0x1300a820 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300a8efu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), cpu.edi); }
  if cpu.zf { pc = 0x1300a861; continue; }
  pc = 0x1300a84c; continue;
 }
 0x1300a84c => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c128u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300a85a); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300a861; continue;
 }
 0x1300a861 => {
  { let v = 0x64u32; cpu.push(v); }
  cpu.push(0x1300a868); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1300a887; continue; }
  pc = 0x1300a880; continue;
 }
 0x1300a880 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x1300a887); f_13009f60(cpu);
  pc = 0x1300a887; continue;
 }
 0x1300a887 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a895); f_13008130(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c134u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a8d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a8d0); }
 let mut pc: u32 = 0x1300a8d0;
 loop { match pc {
 0x1300a8d0 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300a8e1; continue; }
  pc = 0x1300a8d6; continue;
 }
 0x1300a8d6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300a8e1 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a8e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a8e2); }
 let mut pc: u32 = 0x1300a8e2;
 loop { match pc {
 0x1300a8e2 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a8eb); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a8ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a8ef); }
 let mut pc: u32 = 0x1300a8ef;
 loop { match pc {
 0x1300a8ef => {
  { let v: u32 = 0x1300d6c0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a8f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a8f9); }
 let mut pc: u32 = 0x1300a8f9;
 loop { match pc {
 0x1300a8f9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a910); }
 let mut pc: u32 = 0x1300a910;
 loop { match pc {
 0x1300a910 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300aa0fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x1300a959; continue; }
  pc = 0x1300a940; continue;
 }
 0x1300a940 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c128u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300a94e); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300a959; continue;
 }
 0x1300a959 => {
  { let v = 0x64u32; cpu.push(v); }
  cpu.push(0x1300a960); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1300a985; continue; }
  pc = 0x1300a973; continue;
 }
 0x1300a973 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300a983); f_1300a0d0(cpu);
  pc = 0x1300a98a; continue;
 }
 0x1300a985 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x1300a98a; continue;
 }
 0x1300a98a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a998); f_13008130(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0xcu8); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c134u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  if cpu.zf { pc = 0x1300a9d4; continue; }
  pc = 0x1300a9be; continue;
 }
 0x1300a9be => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300a9d4); f_13008760(cpu);
  pc = 0x1300a9d4; continue;
 }
 0x1300a9d4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a9f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a9f0); }
 let mut pc: u32 = 0x1300a9f0;
 loop { match pc {
 0x1300a9f0 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300aa01; continue; }
  pc = 0x1300a9f6; continue;
 }
 0x1300a9f6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300aa01 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300aa02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300aa02); }
 let mut pc: u32 = 0x1300aa02;
 loop { match pc {
 0x1300aa02 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300aa0b); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300aa0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300aa0f); }
 let mut pc: u32 = 0x1300aa0f;
 loop { match pc {
 0x1300aa0f => {
  { let v: u32 = 0x1300d6f8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300aa19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300aa19); }
 let mut pc: u32 = 0x1300aa19;
 loop { match pc {
 0x1300aa19 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300aa30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300aa30); }
 let mut pc: u32 = 0x1300aa30;
 loop { match pc {
 0x1300aa30 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300aa75u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c134u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1300aa68); f_1300aa7f(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300aa75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300aa75); }
 let mut pc: u32 = 0x1300aa75;
 loop { match pc {
 0x1300aa75 => {
  { let v: u32 = 0x1300d730u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300aa7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300aa7f); }
 let mut pc: u32 = 0x1300aa7f;
 loop { match pc {
 0x1300aa7f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13008280(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300aa90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300aa90); }
 let mut pc: u32 = 0x1300aa90;
 loop { match pc {
 0x1300aa90 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300ab70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), cpu.edi); }
  if cpu.zf { pc = 0x1300aad8; continue; }
  pc = 0x1300aabc; continue;
 }
 0x1300aabc => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c140u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c138u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300aad1); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300aad8; continue;
 }
 0x1300aad8 => {
  { let v = 0x64u32; cpu.push(v); }
  cpu.push(0x1300aadf); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1300aafe; continue; }
  pc = 0x1300aaf7; continue;
 }
 0x1300aaf7 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x1300aafe); f_13009f60(cpu);
  pc = 0x1300aafe; continue;
 }
 0x1300aafe => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300ab0c); f_13005590(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c14cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ab51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ab51); }
 let mut pc: u32 = 0x1300ab51;
 loop { match pc {
 0x1300ab51 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300ab62; continue; }
  pc = 0x1300ab57; continue;
 }
 0x1300ab57 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300ab62 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ab63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ab63); }
 let mut pc: u32 = 0x1300ab63;
 loop { match pc {
 0x1300ab63 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300ab6c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ab70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ab70); }
 let mut pc: u32 = 0x1300ab70;
 loop { match pc {
 0x1300ab70 => {
  { let v: u32 = 0x1300d758u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ab7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ab7a); }
 let mut pc: u32 = 0x1300ab7a;
 loop { match pc {
 0x1300ab7a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ab90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ab90); }
 let mut pc: u32 = 0x1300ab90;
 loop { match pc {
 0x1300ab90 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300aca6u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x1300abe0; continue; }
  pc = 0x1300abc0; continue;
 }
 0x1300abc0 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c140u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c138u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300abd5); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300abe0; continue;
 }
 0x1300abe0 => {
  { let v = 0x64u32; cpu.push(v); }
  cpu.push(0x1300abe7); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1300ac0c; continue; }
  pc = 0x1300abfa; continue;
 }
 0x1300abfa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300ac0a); f_1300a0d0(cpu);
  pc = 0x1300ac11; continue;
 }
 0x1300ac0c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x1300ac11; continue;
 }
 0x1300ac11 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300ac1f); f_13005590(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c14cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  if cpu.zf { pc = 0x1300ac6b; continue; }
  pc = 0x1300ac50; continue;
 }
 0x1300ac50 => {
  { cpu.and8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x1300ac6b; continue; }
  pc = 0x1300ac54; continue;
 }
 0x1300ac54 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x1300ac6b); f_13008760(cpu);
  pc = 0x1300ac6b; continue;
 }
 0x1300ac6b => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ac87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ac87); }
 let mut pc: u32 = 0x1300ac87;
 loop { match pc {
 0x1300ac87 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300ac98; continue; }
  pc = 0x1300ac8d; continue;
 }
 0x1300ac8d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300ac98 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ac99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ac99); }
 let mut pc: u32 = 0x1300ac99;
 loop { match pc {
 0x1300ac99 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300aca2); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300aca6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300aca6); }
 let mut pc: u32 = 0x1300aca6;
 loop { match pc {
 0x1300aca6 => {
  { let v: u32 = 0x1300d790u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300acb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300acb0); }
 let mut pc: u32 = 0x1300acb0;
 loop { match pc {
 0x1300acb0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300acc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300acc0); }
 let mut pc: u32 = 0x1300acc0;
 loop { match pc {
 0x1300acc0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300ad05u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c14cu32; cpu.w32(0xffffffecu32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1300acf8); f_1300ad0f(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ad05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ad05); }
 let mut pc: u32 = 0x1300ad05;
 loop { match pc {
 0x1300ad05 => {
  { let v: u32 = 0x1300d7c8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ad0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ad0f); }
 let mut pc: u32 = 0x1300ad0f;
 loop { match pc {
 0x1300ad0f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13005730(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ad18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ad18); }
 let mut pc: u32 = 0x1300ad18;
 loop { match pc {
 0x1300ad18 => {
  pc = cpu.r32(0x130131bcu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ad20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ad20); }
 let mut pc: u32 = 0x1300ad20;
 loop { match pc {
 0x1300ad20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300c350u32; cpu.push(v); }
  { let v = 0x1300b1b0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), cpu.ecx); cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if cpu.sf { pc = 0x1300ad6b; continue; }
  pc = 0x1300ad5a; continue;
 }
 0x1300ad5a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x1300ad66); cpu.call(t);
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1300ad5a; continue; }
  pc = 0x1300ad6b; continue;
 }
 0x1300ad6b => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1300ad7e); f_1300ad90(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ad90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ad90); }
 let mut pc: u32 = 0x1300ad90;
 loop { match pc {
 0x1300ad90 => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1300adab; continue; }
  pc = 0x1300ad96; continue;
 }
 0x1300ad96 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300adab); f_1300adb0(cpu);
  pc = 0x1300adab; continue;
 }
 0x1300adab => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300adb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300adb0); }
 let mut pc: u32 = 0x1300adb0;
 loop { match pc {
 0x1300adb0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300c360u32; cpu.push(v); }
  { let v = 0x1300b1b0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if cpu.sf { pc = 0x1300ae0e; continue; }
  pc = 0x1300ade6; continue;
 }
 0x1300ade6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x1300adec; continue;
 }
 0x1300adec => {
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x1300adf6); cpu.call(t);
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1300adec; continue; }
  pc = 0x1300adfc; continue;
 }
 0x1300adfc => {
  pc = 0x1300ae0e; continue;
 }
 0x1300ae0e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300adfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300adfe); }
 let mut pc: u32 = 0x1300adfe;
 loop { match pc {
 0x1300adfe => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300ae07); f_1300ae30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ae0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ae0b); }
 let mut pc: u32 = 0x1300ae0b;
 loop { match pc {
 0x1300ae0b => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ae30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ae30); }
 let mut pc: u32 = 0x1300ae30;
 loop { match pc {
 0x1300ae30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0xe06d7363u32); }
  if !cpu.zf { pc = 0x1300ae43; continue; }
  pc = 0x1300ae3e; continue;
 }
 0x1300ae3e => {
  cpu.push(0x1300ae43); f_1300b1b6(cpu);
  pc = 0x1300ae43; continue;
 }
 0x1300ae43 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ae46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ae46); }
 let mut pc: u32 = 0x1300ae46;
 loop { match pc {
 0x1300ae46 => {
  pc = cpu.r32(0x130131b8u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ae50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ae50); }
 let mut pc: u32 = 0x1300ae50;
 loop { match pc {
 0x1300ae50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1300ae58); f_1300b1bc(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x1300ae68; continue; }
  pc = 0x1300ae5f; continue;
 }
 0x1300ae5f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300ae65); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300ae68; continue;
 }
 0x1300ae68 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ae80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ae80); }
 let mut pc: u32 = 0x1300ae80;
 loop { match pc {
 0x1300ae80 => {
  pc = cpu.r32(0x130131b0u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300ae90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300ae90); }
 let mut pc: u32 = 0x1300ae90;
 loop { match pc {
 0x1300ae90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x130122a0u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1300aebf; continue; }
  pc = 0x1300ae9b; continue;
 }
 0x1300ae9b => {
  let t = cpu.r32(0x1301310cu32); cpu.push(0x1300aea1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  { let v: u32 = cpu.r32(0x130122a0u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1300aebe; continue; }
  pc = 0x1300aeb4; continue;
 }
 0x1300aeb4 => {
  { cpu.and32(cpu.eax, 0x80000000u32); }
  if cpu.zf { pc = 0x1300aebe; continue; }
  pc = 0x1300aebb; continue;
 }
 0x1300aebb => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1300aebf; continue;
 }
 0x1300aebe => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1300aebf; continue;
 }
 0x1300aebf => {
  { let v: u32 = cpu.ecx; cpu.w32(0x130122a0u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1300aeeb; continue; }
  pc = 0x1300aec9; continue;
 }
 0x1300aec9 => {
  { let v: u32 = cpu.r32(0x130122a4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1300aee5; continue; }
  pc = 0x1300aed2; continue;
 }
 0x1300aed2 => {
  { let v: u32 = cpu.r32(0x13013110u32); cpu.esi = v; }
  pc = 0x1300aed8; continue;
 }
 0x1300aed8 => {
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x1300aedc); cpu.call(t);
  { let v: u32 = cpu.r32(0x130122a4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300aed8; continue; }
  pc = 0x1300aee5; continue;
 }
 0x1300aee5 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x130122a4u32, v); }
  pc = 0x1300aeeb; continue;
 }
 0x1300aeeb => {
  { cpu.sub32(cpu.r32(0x1301245cu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300af04; continue; }
  pc = 0x1300aef4; continue;
 }
 0x1300aef4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1301318cu32); cpu.push(0x1300aeff); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300af1b; continue;
 }
 0x1300af04 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x13012458u32; cpu.push(v); }
  { let v = 0x1301245cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300af18); f_1300b1c2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1300af1b; continue;
 }
 0x1300af1b => {
  { cpu.sub32(cpu.r32(0x130122a0u32), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1300af2a; continue; }
  pc = 0x1300af24; continue;
 }
 0x1300af24 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x130122a4u32)); cpu.w32(0x130122a4u32, v); }
  pc = 0x1300af2a; continue;
 }
 0x1300af2a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300af30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300af30); }
 let mut pc: u32 = 0x1300af30;
 loop { match pc {
 0x1300af30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300af3a); f_1300ae90(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300af50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300af50); }
 let mut pc: u32 = 0x1300af50;
 loop { match pc {
 0x1300af50 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x1000u32); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  if cpu.cf { pc = 0x1300af70; continue; }
  pc = 0x1300af5c; continue;
 }
 0x1300af5c => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x1000u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1000u32); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); }
  { cpu.sub32(cpu.eax, 0x1000u32); }
  if !cpu.cf { pc = 0x1300af5c; continue; }
  pc = 0x1300af70; continue;
 }
 0x1300af70 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); }
  { let v: u32 = cpu.ecx; cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300af80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300af80); }
 let mut pc: u32 = 0x1300af80;
 loop { match pc {
 0x1300af80 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1300af98; continue; }
  pc = 0x1300af89; continue;
 }
 0x1300af89 => {
  { cpu.sub32(cpu.r32(0x130122b0u32), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1300afba; continue; }
  pc = 0x1300af92; continue;
 }
 0x1300af92 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x130122b0u32)); cpu.w32(0x130122b0u32, v); }
  pc = 0x1300af98; continue;
 }
 0x1300af98 => {
  { cpu.sub32(cpu.r32(0x130122a0u32), 0x0u32); }
  if !cpu.zf { pc = 0x1300afc6; continue; }
  pc = 0x1300afa1; continue;
 }
 0x1300afa1 => {
  let t = cpu.r32(0x1301310cu32); cpu.push(0x1300afa7); cpu.call(t);
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if !cpu.zf { pc = 0x1300afc0; continue; }
  pc = 0x1300afab; continue;
 }
 0x1300afab => {
  { cpu.and32(cpu.eax, 0x80000000u32); }
  if cpu.zf { pc = 0x1300afc0; continue; }
  pc = 0x1300afb2; continue;
 }
 0x1300afb2 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x130122a0u32)); cpu.w32(0x130122a0u32, v); }
  pc = 0x1300afc6; continue;
 }
 0x1300afba => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300afc0 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x130122a0u32)); cpu.w32(0x130122a0u32, v); }
  pc = 0x1300afc6; continue;
 }
 0x1300afc6 => {
  { let v: u32 = cpu.r32(0x1301319cu32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x13012450u32, v); }
  if !cpu.zf { pc = 0x1300b076; continue; }
  pc = 0x1300afdc; continue;
 }
 0x1300afdc => {
  { cpu.sub32(cpu.r32(0x130122a0u32), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x1300affb; continue; }
  pc = 0x1300afe5; continue;
 }
 0x1300afe5 => {
  { cpu.sub32(cpu.r32(0x130122b0u32), 0x0u32); }
  if !cpu.zf { pc = 0x1300b0e3; continue; }
  pc = 0x1300aff2; continue;
 }
 0x1300aff2 => {
  { cpu.sub32(cpu.r32(0x130122a0u32), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300b018; continue; }
  pc = 0x1300affb; continue;
 }
 0x1300affb => {
  { let v = 0x80u32; cpu.push(v); }
  let t = cpu.r32(0x13013198u32); cpu.push(0x1300b006); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1301245cu32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1300b040; continue; }
  pc = 0x1300b012; continue;
 }
 0x1300b012 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300b018 => {
  { cpu.sub32(cpu.r32(0x130122b0u32), 0x0u32); }
  if !cpu.zf { pc = 0x1300b040; continue; }
  pc = 0x1300b021; continue;
 }
 0x1300b021 => {
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x2000u32; cpu.push(v); }
  let t = cpu.r32(0x13013104u32); cpu.push(0x1300b031); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1301245cu32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1300b040; continue; }
  pc = 0x1300b03a; continue;
 }
 0x1300b03a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300b040 => {
  { let v: u32 = cpu.r32(0x1301245cu32); cpu.eax = v; }
  { let v = 0x13012024u32; cpu.push(v); }
  { let v = 0x13012000u32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1301245cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x13012458u32, v); }
  cpu.push(0x1300b064); f_1300b1c8(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x130122b0u32)); cpu.w32(0x130122b0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300b076 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1300b0e3; continue; }
  pc = 0x1300b07a; continue;
 }
 0x1300b07a => {
  { cpu.sub32(cpu.r32(0x130122a0u32), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x1300b08c; continue; }
  pc = 0x1300b083; continue;
 }
 0x1300b083 => {
  { cpu.sub32(cpu.r32(0x130122b0u32), 0x0u32); }
  if !cpu.zf { pc = 0x1300b0e3; continue; }
  pc = 0x1300b08c; continue;
 }
 0x1300b08c => {
  { cpu.sub32(cpu.r32(0x1301245cu32), 0x0u32); }
  if cpu.zf { pc = 0x1300b0e3; continue; }
  pc = 0x1300b095; continue;
 }
 0x1300b095 => {
  { let v: u32 = cpu.r32(0x13012458u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1301245cu32), cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1300b0b9; continue; }
  pc = 0x1300b0a6; continue;
 }
 0x1300b0a6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300b0ae; continue; }
  pc = 0x1300b0ac; continue;
 }
 0x1300b0ac => {
  let t = cpu.eax; cpu.push(0x1300b0ae); cpu.call(t);
  pc = 0x1300b0ae; continue;
 }
 0x1300b0ae => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1301245cu32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1300b0a6; continue; }
  pc = 0x1300b0b9; continue;
 }
 0x1300b0b9 => {
  { cpu.sub32(cpu.r32(0x130122a0u32), 0x0u32); }
  { let v: u32 = cpu.r32(0x1301245cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  if (cpu.sf == cpu.of) { pc = 0x1300b0d3; continue; }
  pc = 0x1300b0c8; continue;
 }
 0x1300b0c8 => {
  let t = cpu.r32(0x13013190u32); cpu.push(0x1300b0ce); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300b0d9; continue;
 }
 0x1300b0d3 => {
  let t = cpu.r32(0x13013108u32); cpu.push(0x1300b0d9); cpu.call(t);
  pc = 0x1300b0d9; continue;
 }
 0x1300b0d9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1301245cu32, v); }
  pc = 0x1300b0e3; continue;
 }
 0x1300b0e3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300b0f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300b0f0); }
 let mut pc: u32 = 0x1300b0f0;
 loop { match pc {
 0x1300b0f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  { let v = cpu.ebp; cpu.push(v); }
  if !cpu.zf { pc = 0x1300b113; continue; }
  pc = 0x1300b101; continue;
 }
 0x1300b101 => {
  { cpu.sub32(cpu.r32(0x130122b0u32), 0x0u32); }
  if !cpu.zf { pc = 0x1300b113; continue; }
  pc = 0x1300b10a; continue;
 }
 0x1300b10a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300b113 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x1300b11d; continue; }
  pc = 0x1300b118; continue;
 }
 0x1300b118 => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if !cpu.zf { pc = 0x1300b157; continue; }
  pc = 0x1300b11d; continue;
 }
 0x1300b11d => {
  { let v: u32 = cpu.r32(0x13012454u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1300b136; continue; }
  pc = 0x1300b127; continue;
 }
 0x1300b127 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ecx; cpu.push(0x1300b134); cpu.call(t);
  pc = 0x1300b13e; continue;
 }
 0x1300b136 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x1300b13e; continue;
 }
 0x1300b13e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300b14e; continue; }
  pc = 0x1300b142; continue;
 }
 0x1300b142 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300b14a); f_1300af80(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1300b15f; continue; }
  pc = 0x1300b14e; continue;
 }
 0x1300b14e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300b157 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x1300b15f; continue;
 }
 0x1300b15f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300b167); f_13001000(cpu);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x1300b17b; continue; }
  pc = 0x1300b16e; continue;
 }
 0x1300b16e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x1300b17b; continue; }
  pc = 0x1300b172; continue;
 }
 0x1300b172 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300b17b); f_1300af80(cpu);
  pc = 0x1300b17b; continue;
 }
 0x1300b17b => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1300b184; continue; }
  pc = 0x1300b17f; continue;
 }
 0x1300b17f => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if !cpu.zf { pc = 0x1300b1a6; continue; }
  pc = 0x1300b184; continue;
 }
 0x1300b184 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300b18c); f_1300af80(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1300b192; continue; }
  pc = 0x1300b190; continue;
 }
 0x1300b190 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x1300b192; continue;
 }
 0x1300b192 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1300b1a6; continue; }
  pc = 0x1300b196; continue;
 }
 0x1300b196 => {
  { let v: u32 = cpu.r32(0x13012454u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300b1a6; continue; }
  pc = 0x1300b19f; continue;
 }
 0x1300b19f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.eax; cpu.push(0x1300b1a4); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1300b1a6; continue;
 }
 0x1300b1a6 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300b1b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300b1b0); }
 let mut pc: u32 = 0x1300b1b0;
 loop { match pc {
 0x1300b1b0 => {
  pc = cpu.r32(0x1301317cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300b1b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300b1b6); }
 let mut pc: u32 = 0x1300b1b6;
 loop { match pc {
 0x1300b1b6 => {
  pc = cpu.r32(0x13013180u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300b1bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300b1bc); }
 let mut pc: u32 = 0x1300b1bc;
 loop { match pc {
 0x1300b1bc => {
  pc = cpu.r32(0x13013184u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300b1c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300b1c2); }
 let mut pc: u32 = 0x1300b1c2;
 loop { match pc {
 0x1300b1c2 => {
  pc = cpu.r32(0x13013188u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300b1c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300b1c8); }
 let mut pc: u32 = 0x1300b1c8;
 loop { match pc {
 0x1300b1c8 => {
  pc = cpu.r32(0x13013194u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
