#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_13004b20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004b20); }
 let mut pc: u32 = 0x13004b20;
 loop { match pc {
 0x13004b20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13004bc3u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.ecx); }
  if cpu.zf { pc = 0x13004b76; continue; }
  pc = 0x13004b4c; continue;
 }
 0x13004b4c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c118u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13004b60; continue; }
  pc = 0x13004b59; continue;
 }
 0x13004b59 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13004b60; continue;
 }
 0x13004b60 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13004b69); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13004b79; continue;
 }
 0x13004b76 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13004b79; continue;
 }
 0x13004b79 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004b83); f_13005ba0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c124u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004bb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004bb1); }
 let mut pc: u32 = 0x13004bb1;
 loop { match pc {
 0x13004bb1 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13004bc2; continue; }
  pc = 0x13004bb7; continue;
 }
 0x13004bb7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13004bc2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004bc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004bc3); }
 let mut pc: u32 = 0x13004bc3;
 loop { match pc {
 0x13004bc3 => {
  { let v: u32 = 0x1300ce68u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004bcd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004bcd); }
 let mut pc: u32 = 0x13004bcd;
 loop { match pc {
 0x13004bcd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004be0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004be0); }
 let mut pc: u32 = 0x13004be0;
 loop { match pc {
 0x13004be0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c118u32); }
  if !cpu.zf { pc = 0x13004c0d; continue; }
  pc = 0x13004bf2; continue;
 }
 0x13004bf2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13004c02; continue; }
  pc = 0x13004bfb; continue;
 }
 0x13004bfb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x13004c02; continue;
 }
 0x13004c02 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  cpu.push(0x13004c0d); f_13004860(cpu);
  pc = 0x13004c0d; continue;
 }
 0x13004c0d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13004c1e); f_13005c70(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004c30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004c30); }
 let mut pc: u32 = 0x13004c30;
 loop { match pc {
 0x13004c30 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13004c38); f_1300a7c0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004c3f); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13004c52; continue; }
  pc = 0x13004c46; continue;
 }
 0x13004c46 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004c4f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004c52; continue;
 }
 0x13004c52 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004c60); }
 let mut pc: u32 = 0x13004c60;
 loop { match pc {
 0x13004c60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13004c97; continue; }
  pc = 0x13004c70; continue;
 }
 0x13004c70 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13004cd0u32; cpu.push(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13004c86); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004c8c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13004c97 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004c9e); f_1300a7c0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004ca5); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13004cb9; continue; }
  pc = 0x13004cad; continue;
 }
 0x13004cad => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004cb6); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004cb9; continue;
 }
 0x13004cb9 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004cd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004cd0); }
 let mut pc: u32 = 0x13004cd0;
 loop { match pc {
 0x13004cd0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004cdb); f_1300a7c0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004ce2); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004cf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004cf0); }
 let mut pc: u32 = 0x13004cf0;
 loop { match pc {
 0x13004cf0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x13004d06; continue; }
  pc = 0x13004d03; continue;
 }
 0x13004d03 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13004d06 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004d10); }
 let mut pc: u32 = 0x13004d10;
 loop { match pc {
 0x13004d10 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004d20); }
 let mut pc: u32 = 0x13004d20;
 loop { match pc {
 0x13004d20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  f_1300a250(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004d30); }
 let mut pc: u32 = 0x13004d30;
 loop { match pc {
 0x13004d30 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13004dd3u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.ecx); }
  if cpu.zf { pc = 0x13004d86; continue; }
  pc = 0x13004d5c; continue;
 }
 0x13004d5c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c128u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13004d70; continue; }
  pc = 0x13004d69; continue;
 }
 0x13004d69 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13004d70; continue;
 }
 0x13004d70 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13004d79); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13004d89; continue;
 }
 0x13004d86 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13004d89; continue;
 }
 0x13004d89 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004d93); f_130081d0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c134u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004dc1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004dc1); }
 let mut pc: u32 = 0x13004dc1;
 loop { match pc {
 0x13004dc1 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13004dd2; continue; }
  pc = 0x13004dc7; continue;
 }
 0x13004dc7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13004dd2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004dd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004dd3); }
 let mut pc: u32 = 0x13004dd3;
 loop { match pc {
 0x13004dd3 => {
  { let v: u32 = 0x1300ce98u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004ddd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004ddd); }
 let mut pc: u32 = 0x13004ddd;
 loop { match pc {
 0x13004ddd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004df0); }
 let mut pc: u32 = 0x13004df0;
 loop { match pc {
 0x13004df0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c128u32); }
  if !cpu.zf { pc = 0x13004e1d; continue; }
  pc = 0x13004e02; continue;
 }
 0x13004e02 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13004e12; continue; }
  pc = 0x13004e0b; continue;
 }
 0x13004e0b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x13004e12; continue;
 }
 0x13004e12 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  cpu.push(0x13004e1d); f_13004860(cpu);
  pc = 0x13004e1d; continue;
 }
 0x13004e1d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13004e2e); f_13008290(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004e40); }
 let mut pc: u32 = 0x13004e40;
 loop { match pc {
 0x13004e40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13004e48); f_1300aa30(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004e4f); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13004e62; continue; }
  pc = 0x13004e56; continue;
 }
 0x13004e56 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004e5f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004e62; continue;
 }
 0x13004e62 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004e70); }
 let mut pc: u32 = 0x13004e70;
 loop { match pc {
 0x13004e70 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13004ea7; continue; }
  pc = 0x13004e80; continue;
 }
 0x13004e80 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13004ee0u32; cpu.push(v); }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x58u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13004e96); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004e9c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13004ea7 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004eae); f_1300aa30(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004eb5); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13004ec9; continue; }
  pc = 0x13004ebd; continue;
 }
 0x13004ebd => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004ec6); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004ec9; continue;
 }
 0x13004ec9 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004ee0); }
 let mut pc: u32 = 0x13004ee0;
 loop { match pc {
 0x13004ee0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004eeb); f_1300aa30(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13004ef2); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004f00); }
 let mut pc: u32 = 0x13004f00;
 loop { match pc {
 0x13004f00 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x13004f16; continue; }
  pc = 0x13004f13; continue;
 }
 0x13004f13 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13004f16 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004f20); }
 let mut pc: u32 = 0x13004f20;
 loop { match pc {
 0x13004f20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004f30); }
 let mut pc: u32 = 0x13004f30;
 loop { match pc {
 0x13004f30 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  f_1300a250(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004f40); }
 let mut pc: u32 = 0x13004f40;
 loop { match pc {
 0x13004f40 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13004feau32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.ecx); }
  if cpu.zf { pc = 0x13004f9d; continue; }
  pc = 0x13004f6c; continue;
 }
 0x13004f6c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c140u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c138u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13004f87; continue; }
  pc = 0x13004f80; continue;
 }
 0x13004f80 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13004f87; continue;
 }
 0x13004f87 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13004f90); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13004fa0; continue;
 }
 0x13004f9d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13004fa0; continue;
 }
 0x13004fa0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004faa); f_13005660(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c14cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004fd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004fd8); }
 let mut pc: u32 = 0x13004fd8;
 loop { match pc {
 0x13004fd8 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13004fe9; continue; }
  pc = 0x13004fde; continue;
 }
 0x13004fde => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13004fe9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004fea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004fea); }
 let mut pc: u32 = 0x13004fea;
 loop { match pc {
 0x13004fea => {
  { let v: u32 = 0x1300cec8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004ff4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004ff4); }
 let mut pc: u32 = 0x13004ff4;
 loop { match pc {
 0x13004ff4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005000); }
 let mut pc: u32 = 0x13005000;
 loop { match pc {
 0x13005000 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c140u32); }
  if !cpu.zf { pc = 0x1300502d; continue; }
  pc = 0x13005012; continue;
 }
 0x13005012 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13005022; continue; }
  pc = 0x1300501b; continue;
 }
 0x1300501b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x13005022; continue;
 }
 0x13005022 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.edi); cpu.ecx = v; }
  cpu.push(0x1300502d); f_13004860(cpu);
  pc = 0x1300502d; continue;
 }
 0x1300502d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300503e); f_13005c70(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  cpu.push(0x13005047); f_13008290(cpu);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005050); }
 let mut pc: u32 = 0x13005050;
 loop { match pc {
 0x13005050 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13005058); f_1300acc0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300505f); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13005072; continue; }
  pc = 0x13005066; continue;
 }
 0x13005066 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300506f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13005072; continue;
 }
 0x13005072 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005080); }
 let mut pc: u32 = 0x13005080;
 loop { match pc {
 0x13005080 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x130050b7; continue; }
  pc = 0x13005090; continue;
 }
 0x13005090 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x130050f0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x130050a6); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130050ac); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130050b7 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130050be); f_1300acc0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130050c5); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x130050d9; continue; }
  pc = 0x130050cd; continue;
 }
 0x130050cd => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130050d6); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130050d9; continue;
 }
 0x130050d9 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130050f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130050f0); }
 let mut pc: u32 = 0x130050f0;
 loop { match pc {
 0x130050f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130050fb); f_1300acc0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13005102); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005110); }
 let mut pc: u32 = 0x13005110;
 loop { match pc {
 0x13005110 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005120); }
 let mut pc: u32 = 0x13005120;
 loop { match pc {
 0x13005120 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130051d0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x1300c004u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x1300c154u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130051d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130051d0); }
 let mut pc: u32 = 0x130051d0;
 loop { match pc {
 0x130051d0 => {
  { let v: u32 = 0x1300cef8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130051da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130051da); }
 let mut pc: u32 = 0x130051da;
 loop { match pc {
 0x130051da => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130051f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130051f0); }
 let mut pc: u32 = 0x130051f0;
 loop { match pc {
 0x130051f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005260); }
 let mut pc: u32 = 0x13005260;
 loop { match pc {
 0x13005260 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13005268); f_13009110(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13005278; continue; }
  pc = 0x1300526f; continue;
 }
 0x1300526f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13005275); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13005278; continue;
 }
 0x13005278 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005280); }
 let mut pc: u32 = 0x13005280;
 loop { match pc {
 0x13005280 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x130052b3; continue; }
  pc = 0x13005290; continue;
 }
 0x13005290 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v = 0x13009110u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x50u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130052a3); f_1300ad20(cpu);
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130052a9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130052b3 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x130052ba); f_13009110(cpu);
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x130052cb; continue; }
  pc = 0x130052c2; continue;
 }
 0x130052c2 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130052c8); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130052cb; continue;
 }
 0x130052cb => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130052e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130052e0); }
 let mut pc: u32 = 0x130052e0;
 loop { match pc {
 0x130052e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130052f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130052f0); }
 let mut pc: u32 = 0x130052f0;
 loop { match pc {
 0x130052f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300539au32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.ecx); }
  if cpu.zf { pc = 0x1300534d; continue; }
  pc = 0x1300531c; continue;
 }
 0x1300531c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c188u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c180u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13005337; continue; }
  pc = 0x13005330; continue;
 }
 0x13005330 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13005337; continue;
 }
 0x13005337 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13005340); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13005350; continue;
 }
 0x1300534d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13005350; continue;
 }
 0x13005350 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300535a); f_13005660(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c194u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005388(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005388); }
 let mut pc: u32 = 0x13005388;
 loop { match pc {
 0x13005388 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13005399; continue; }
  pc = 0x1300538e; continue;
 }
 0x1300538e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13005399 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300539a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300539a); }
 let mut pc: u32 = 0x1300539a;
 loop { match pc {
 0x1300539a => {
  { let v: u32 = 0x1300cf20u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130053a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130053a4); }
 let mut pc: u32 = 0x130053a4;
 loop { match pc {
 0x130053a4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130053b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130053b0); }
 let mut pc: u32 = 0x130053b0;
 loop { match pc {
 0x130053b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c188u32); }
  if !cpu.zf { pc = 0x130053dd; continue; }
  pc = 0x130053c2; continue;
 }
 0x130053c2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x130053d2; continue; }
  pc = 0x130053cb; continue;
 }
 0x130053cb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x130053d2; continue;
 }
 0x130053d2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.edi); cpu.ecx = v; }
  cpu.push(0x130053dd); f_13004860(cpu);
  pc = 0x130053dd; continue;
 }
 0x130053dd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130053ee); f_13005c70(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  cpu.push(0x130053f7); f_13008290(cpu);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005400); }
 let mut pc: u32 = 0x13005400;
 loop { match pc {
 0x13005400 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13005408); f_13009690(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300540f); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13005422; continue; }
  pc = 0x13005416; continue;
 }
 0x13005416 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300541f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13005422; continue;
 }
 0x13005422 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005430); }
 let mut pc: u32 = 0x13005430;
 loop { match pc {
 0x13005430 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13005467; continue; }
  pc = 0x13005440; continue;
 }
 0x13005440 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x130054a0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13005456); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300545c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13005467 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300546e); f_13009690(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13005475); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13005489; continue; }
  pc = 0x1300547d; continue;
 }
 0x1300547d => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005486); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13005489; continue;
 }
 0x13005489 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130054a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130054a0); }
 let mut pc: u32 = 0x130054a0;
 loop { match pc {
 0x130054a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130054ab); f_13009690(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130054b2); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130054c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130054c0); }
 let mut pc: u32 = 0x130054c0;
 loop { match pc {
 0x130054c0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300556fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x1300550e; continue; }
  pc = 0x130054ee; continue;
 }
 0x130054ee => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c1a0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c198u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13005503); f_13004670(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300550e; continue;
 }
 0x1300550e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13005517); f_13005a50(cpu);
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13005528); f_130080a0(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c1acu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_13005552(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005552); }
 let mut pc: u32 = 0x13005552;
 loop { match pc {
 0x13005552 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13005563; continue; }
  pc = 0x13005558; continue;
 }
 0x13005558 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13005563 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005564(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005564); }
 let mut pc: u32 = 0x13005564;
 loop { match pc {
 0x13005564 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300556f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300556f); }
 let mut pc: u32 = 0x1300556f;
 loop { match pc {
 0x1300556f => {
  { let v: u32 = 0x1300cf50u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005579(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005579); }
 let mut pc: u32 = 0x13005579;
 loop { match pc {
 0x13005579 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005590); }
 let mut pc: u32 = 0x13005590;
 loop { match pc {
 0x13005590 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300563fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.edi); }
  if cpu.zf { pc = 0x130055d8; continue; }
  pc = 0x130055bc; continue;
 }
 0x130055bc => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c1a0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c198u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130055d1); f_13004670(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130055d8; continue;
 }
 0x130055d8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130055e5); f_13005af0(cpu);
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130055f7); f_13008130(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c1acu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005622(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005622); }
 let mut pc: u32 = 0x13005622;
 loop { match pc {
 0x13005622 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13005633; continue; }
  pc = 0x13005628; continue;
 }
 0x13005628 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13005633 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005634(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005634); }
 let mut pc: u32 = 0x13005634;
 loop { match pc {
 0x13005634 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300563f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300563f); }
 let mut pc: u32 = 0x1300563f;
 loop { match pc {
 0x1300563f => {
  { let v: u32 = 0x1300cf88u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005649(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005649); }
 let mut pc: u32 = 0x13005649;
 loop { match pc {
 0x13005649 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005660); }
 let mut pc: u32 = 0x13005660;
 loop { match pc {
 0x13005660 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300571au32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.edi); }
  if cpu.zf { pc = 0x130056a8; continue; }
  pc = 0x1300568c; continue;
 }
 0x1300568c => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c1a0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c198u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130056a1); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130056a8; continue;
 }
 0x130056a8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130056b5); f_13005ba0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x130056c5; continue; }
  pc = 0x130056c0; continue;
 }
 0x130056c0 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  pc = 0x130056c7; continue;
 }
 0x130056c5 => {
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  pc = 0x130056c7; continue;
 }
 0x130056c7 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130056d2); f_130081d0(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c1acu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130056fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130056fd); }
 let mut pc: u32 = 0x130056fd;
 loop { match pc {
 0x130056fd => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300570e; continue; }
  pc = 0x13005703; continue;
 }
 0x13005703 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300570e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300570f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300570f); }
 let mut pc: u32 = 0x1300570f;
 loop { match pc {
 0x1300570f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300571a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300571a); }
 let mut pc: u32 = 0x1300571a;
 loop { match pc {
 0x1300571a => {
  { let v: u32 = 0x1300cfc0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005724(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005724); }
 let mut pc: u32 = 0x13005724;
 loop { match pc {
 0x13005724 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005730); }
 let mut pc: u32 = 0x13005730;
 loop { match pc {
 0x13005730 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130057a5u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c1acu32; cpu.w32(0xffffffecu32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13005769); f_13005782(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13005775); f_130057af(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005782(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005782); }
 let mut pc: u32 = 0x13005782;
 loop { match pc {
 0x13005782 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x14u32); }
  if cpu.zf { pc = 0x13005793; continue; }
  pc = 0x13005788; continue;
 }
 0x13005788 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300579a; continue;
 }
 0x13005793 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300579a; continue;
 }
 0x1300579a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_13008280(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130057a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130057a5); }
 let mut pc: u32 = 0x130057a5;
 loop { match pc {
 0x130057a5 => {
  { let v: u32 = 0x1300cff8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130057af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130057af); }
 let mut pc: u32 = 0x130057af;
 loop { match pc {
 0x130057af => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_13005c60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130057c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130057c0); }
 let mut pc: u32 = 0x130057c0;
 loop { match pc {
 0x130057c0 => {
  cpu.push(0x130057c5); f_130057d0(cpu);
  pc = 0x13005850; continue;
 }
 0x13005850 => {
  { let v = 0x13005860u32; cpu.push(v); }
  cpu.push(0x1300585a); f_1300af30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130057d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130057d0); }
 let mut pc: u32 = 0x130057d0;
 loop { match pc {
 0x130057d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13005836u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x130057f3); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13005814; continue; }
  pc = 0x1300580b; continue;
 }
 0x1300580b => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13005814); f_130030f0(cpu);
  pc = 0x13005814; continue;
 }
 0x13005814 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x130123f8u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005828); f_13008400(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005836(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005836); }
 let mut pc: u32 = 0x13005836;
 loop { match pc {
 0x13005836 => {
  { let v: u32 = 0x1300d028u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005840); }
 let mut pc: u32 = 0x13005840;
 loop { match pc {
 0x13005840 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005849); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005860); }
 let mut pc: u32 = 0x13005860;
 loop { match pc {
 0x13005860 => {
  { let v: u32 = 0x13012400u32; cpu.ecx = v; }
  cpu.push(0x1300586a); f_130084b0(cpu);
  { let v: u32 = 0x13012400u32; cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005880); }
 let mut pc: u32 = 0x13005880;
 loop { match pc {
 0x13005880 => {
  cpu.push(0x13005885); f_13005890(cpu);
  pc = 0x130058c0; continue;
 }
 0x130058c0 => {
  { let v = 0x130058d0u32; cpu.push(v); }
  cpu.push(0x130058ca); f_1300af30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005890); }
 let mut pc: u32 = 0x13005890;
 loop { match pc {
 0x13005890 => {
  { let v: u32 = 0x130123f8u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if cpu.zf { pc = 0x130058ab; continue; }
  pc = 0x1300589e; continue;
 }
 0x1300589e => {
  { let v: u32 = cpu.r32(0x130123f8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x130123f8u32); cpu.eax = v; }
  pc = 0x130058ab; continue;
 }
 0x130058ab => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x130123f0u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130058b8); f_130058f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130058d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130058d0); }
 let mut pc: u32 = 0x130058d0;
 loop { match pc {
 0x130058d0 => {
  { let v: u32 = 0x130123f0u32; cpu.ecx = v; }
  f_13005950(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130058e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130058e0); }
 let mut pc: u32 = 0x130058e0;
 loop { match pc {
 0x130058e0 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130058f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130058f0); }
 let mut pc: u32 = 0x130058f0;
 loop { match pc {
 0x130058f0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  if (cpu.sf != cpu.of) { pc = 0x1300593d; continue; }
  pc = 0x13005907; continue;
 }
 0x13005907 => {
  { let v: u32 = 0x130123f8u32; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1300593d; continue; }
  pc = 0x13005912; continue;
 }
 0x13005912 => {
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005924; continue; }
  pc = 0x13005918; continue;
 }
 0x13005918 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005921); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005924; continue;
 }
 0x13005924 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)) | 0x2000u32; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300593d; continue; }
  pc = 0x13005931; continue;
 }
 0x13005931 => {
  { let v: u32 = cpu.esi.wrapping_add(0x38u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300593a); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300593d; continue;
 }
 0x1300593d => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005950); }
 let mut pc: u32 = 0x13005950;
 loop { match pc {
 0x13005950 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005960); }
 let mut pc: u32 = 0x13005960;
 loop { match pc {
 0x13005960 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1300596a); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130059c7; continue; }
  pc = 0x1300596e; continue;
 }
 0x1300596e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x1300597c); f_13009e20(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300598d; continue; }
  pc = 0x13005981; continue;
 }
 0x13005981 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x5u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13005993; continue;
 }
 0x1300598d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x13005993; continue;
 }
 0x13005993 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130059ae; continue; }
  pc = 0x130059a2; continue;
 }
 0x130059a2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130059ab); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130059ae; continue;
 }
 0x130059ae => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130059c7; continue; }
  pc = 0x130059bb; continue;
 }
 0x130059bb => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130059c4); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130059c7; continue;
 }
 0x130059c7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130059d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130059d0); }
 let mut pc: u32 = 0x130059d0;
 loop { match pc {
 0x130059d0 => {
  { let v: u32 = 0x1cu32; cpu.eax = v; }
  cpu.push(0x130059da); f_1300af50(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130059e4); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13005a44; continue; }
  pc = 0x130059e8; continue;
 }
 0x130059e8 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x1cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130059f6); f_13006810(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13005a10; continue; }
  pc = 0x130059fa; continue;
 }
 0x130059fa => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013134u32); cpu.push(0x13005a07); cpu.call(t);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ecx), v); cpu.fpop(); }
  pc = 0x13005a10; continue;
 }
 0x13005a10 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005a2b; continue; }
  pc = 0x13005a1f; continue;
 }
 0x13005a1f => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005a28); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005a2b; continue;
 }
 0x13005a2b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005a44; continue; }
  pc = 0x13005a38; continue;
 }
 0x13005a38 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005a41); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005a44; continue;
 }
 0x13005a44 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005a50); }
 let mut pc: u32 = 0x13005a50;
 loop { match pc {
 0x13005a50 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13005acfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x13005a97; continue; }
  pc = 0x13005a7e; continue;
 }
 0x13005a7e => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c1b0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13005a8c); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13005a97; continue;
 }
 0x13005a97 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c1bcu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x24u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x1u32); cpu.w32(0x24u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005acf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005acf); }
 let mut pc: u32 = 0x13005acf;
 loop { match pc {
 0x13005acf => {
  { let v: u32 = 0x1300d050u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005ad9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005ad9); }
 let mut pc: u32 = 0x13005ad9;
 loop { match pc {
 0x13005ad9 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13005aea; continue; }
  pc = 0x13005adf; continue;
 }
 0x13005adf => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13005aea => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005af0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005af0); }
 let mut pc: u32 = 0x13005af0;
 loop { match pc {
 0x13005af0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13005b7fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x13005b37; continue; }
  pc = 0x13005b1e; continue;
 }
 0x13005b1e => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c1b0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13005b2c); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13005b37; continue;
 }
 0x13005b37 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c1bcu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13005b53); f_13004820(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x24u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x1u32); cpu.w32(0x24u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005b7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005b7f); }
 let mut pc: u32 = 0x13005b7f;
 loop { match pc {
 0x13005b7f => {
  { let v: u32 = 0x1300d078u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005b89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005b89); }
 let mut pc: u32 = 0x13005b89;
 loop { match pc {
 0x13005b89 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13005b9a; continue; }
  pc = 0x13005b8f; continue;
 }
 0x13005b8f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13005b9a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005ba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005ba0); }
 let mut pc: u32 = 0x13005ba0;
 loop { match pc {
 0x13005ba0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13005c38u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x13005be7; continue; }
  pc = 0x13005bce; continue;
 }
 0x13005bce => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c1b0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13005bdc); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13005be7; continue;
 }
 0x13005be7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c1bcu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13005c0c); f_13004820(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x24u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x1u32); cpu.w32(0x24u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005c38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005c38); }
 let mut pc: u32 = 0x13005c38;
 loop { match pc {
 0x13005c38 => {
  { let v: u32 = 0x1300d0a0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005c42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005c42); }
 let mut pc: u32 = 0x13005c42;
 loop { match pc {
 0x13005c42 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13005c53; continue; }
  pc = 0x13005c48; continue;
 }
 0x13005c48 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13005c53 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005c60); }
 let mut pc: u32 = 0x13005c60;
 loop { match pc {
 0x13005c60 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x1300c1bcu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005c70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005c70); }
 let mut pc: u32 = 0x13005c70;
 loop { match pc {
 0x13005c70 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13005d2bu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x13005ca8; continue; }
  pc = 0x13005c9b; continue;
 }
 0x13005c9b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13005ca8; continue; }
  pc = 0x13005ca2; continue;
 }
 0x13005ca2 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x13005ca8); cpu.call(t);
  pc = 0x13005ca8; continue;
 }
 0x13005ca8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x13005cc0); f_13004670(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  cpu.push(0x13005cd8); f_13004860(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13005ce4); f_13005d35(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  cpu.push(0x13005d05); f_13004820(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x1u32); cpu.w32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005d2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005d2b); }
 let mut pc: u32 = 0x13005d2b;
 loop { match pc {
 0x13005d2b => {
  { let v: u32 = 0x1300d0c8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005d35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005d35); }
 let mut pc: u32 = 0x13005d35;
 loop { match pc {
 0x13005d35 => {
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005d40); }
 let mut pc: u32 = 0x13005d40;
 loop { match pc {
 0x13005d40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005d5d; continue; }
  pc = 0x13005d51; continue;
 }
 0x13005d51 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005d5a); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005d5d; continue;
 }
 0x13005d5d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13005d6c; continue; }
  pc = 0x13005d65; continue;
 }
 0x13005d65 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x13005d6c; continue;
 }
 0x13005d6c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13005da0; continue; }
  pc = 0x13005d7a; continue;
 }
 0x13005d7a => {
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005d99; continue; }
  pc = 0x13005d8d; continue;
 }
 0x13005d8d => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005d96); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005d99; continue;
 }
 0x13005d99 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13005da0 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13005dc9; continue; }
  pc = 0x13005da7; continue;
 }
 0x13005da7 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13005dc4; continue; }
  pc = 0x13005dab; continue;
 }
 0x13005dab => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x13005dbe; continue; }
  pc = 0x13005db8; continue;
 }
 0x13005db8 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13005dc9; continue; }
  pc = 0x13005dbc; continue;
 }
 0x13005dbc => {
  pc = 0x13005dc4; continue;
 }
 0x13005dbe => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x13005dc9; continue; }
  pc = 0x13005dc4; continue;
 }
 0x13005dc4 => {
  cpu.push(0x13005dc9); f_13008010(cpu);
  pc = 0x13005dc9; continue;
 }
 0x13005dc9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005de4; continue; }
  pc = 0x13005dd8; continue;
 }
 0x13005dd8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005de1); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005de4; continue;
 }
 0x13005de4 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x13005e4a; continue; }
  pc = 0x13005de8; continue;
 }
 0x13005de8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x13005e4a; continue; }
  pc = 0x13005df4; continue;
 }
 0x13005df4 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13005dfb); f_13006130(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13005e4a; continue; }
  pc = 0x13005e0a; continue;
 }
 0x13005e0a => {
  { let v: u32 = cpu.ecx | 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005e2a; continue; }
  pc = 0x13005e1e; continue;
 }
 0x13005e1e => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005e27); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005e2a; continue;
 }
 0x13005e2a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005e43; continue; }
  pc = 0x13005e37; continue;
 }
 0x13005e37 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005e40); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005e43; continue;
 }
 0x13005e43 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13005e4a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005e60); }
 let mut pc: u32 = 0x13005e60;
 loop { match pc {
 0x13005e60 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x13005e70); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13005f47; continue; }
  pc = 0x13005e78; continue;
 }
 0x13005e78 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x30u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x13005ea2; continue; }
  pc = 0x13005e96; continue;
 }
 0x13005e96 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13005f13; continue;
 }
 0x13005ea2 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), cpu.ebp); }
  if cpu.zf { pc = 0x13005eff; continue; }
  pc = 0x13005eaa; continue;
 }
 0x13005eaa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13005eb8); f_13009ea0(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  if cpu.zf { pc = 0x13005ee7; continue; }
  pc = 0x13005ebf; continue;
 }
 0x13005ebf => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x13013138u32); cpu.push(0x13005ec6); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13005eff; continue; }
  pc = 0x13005ecd; continue;
 }
 0x13005ecd => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13005edf); f_13009e60(cpu);
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), cpu.ebp); }
  if (!cpu.cf && !cpu.zf) { pc = 0x13005eaa; continue; }
  pc = 0x13005ee5; continue;
 }
 0x13005ee5 => {
  pc = 0x13005eff; continue;
 }
 0x13005ee7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x13005f0f; continue; }
  pc = 0x13005ef5; continue;
 }
 0x13005ef5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13005eff; continue;
 }
 0x13005eff => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x13005f0f; continue; }
  pc = 0x13005f03; continue;
 }
 0x13005f03 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13005f13; continue;
 }
 0x13005f0f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x13005f13; continue;
 }
 0x13005f13 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005f2e; continue; }
  pc = 0x13005f22; continue;
 }
 0x13005f22 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005f2b); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005f2e; continue;
 }
 0x13005f2e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005f47; continue; }
  pc = 0x13005f3b; continue;
 }
 0x13005f3b => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005f44); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005f47; continue;
 }
 0x13005f47 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005f60); }
 let mut pc: u32 = 0x13005f60;
 loop { match pc {
 0x13005f60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x13005f6b); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13005fb8; continue; }
  pc = 0x13005f6f; continue;
 }
 0x13005f6f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13005f7d); f_13009ea0(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005f9a; continue; }
  pc = 0x13005f8e; continue;
 }
 0x13005f8e => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005f97); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005f9a; continue;
 }
 0x13005f9a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005fbd; continue; }
  pc = 0x13005fa7; continue;
 }
 0x13005fa7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005fb0); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13005fb8 => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x13005fbd; continue;
 }
 0x13005fbd => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13005fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13005fd0); }
 let mut pc: u32 = 0x13005fd0;
 loop { match pc {
 0x13005fd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x13006081; continue; }
  pc = 0x13005fe7; continue;
 }
 0x13005fe7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13005ffc; continue; }
  pc = 0x13005ff0; continue;
 }
 0x13005ff0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13005ff9); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13005ffc; continue;
 }
 0x13005ffc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ecx)), cpu.eax); }
  if cpu.cf { pc = 0x1300601f; continue; }
  pc = 0x1300600d; continue;
 }
 0x1300600d => {
  { let v: u32 = cpu.r8(0x14u32.wrapping_add(cpu.esp)) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.push(0x13006018); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13006066; continue; }
  pc = 0x1300601d; continue;
 }
 0x1300601d => {
  pc = 0x13006031; continue;
 }
 0x1300601f => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x13006066; continue; }
  pc = 0x13006031; continue;
 }
 0x13006031 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | 0x2u32; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006052; continue; }
  pc = 0x13006046; continue;
 }
 0x13006046 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300604f); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006052; continue;
 }
 0x13006052 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006066; continue; }
  pc = 0x1300605a; continue;
 }
 0x1300605a => {
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x13006063); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006066; continue;
 }
 0x13006066 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006081; continue; }
  pc = 0x13006075; continue;
 }
 0x13006075 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300607e); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13006081; continue;
 }
 0x13006081 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006090); }
 let mut pc: u32 = 0x13006090;
 loop { match pc {
 0x13006090 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130060b4; continue; }
  pc = 0x130060a8; continue;
 }
 0x130060a8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130060b1); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130060b4; continue;
 }
 0x130060b4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x130060c2); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13006100; continue; }
  pc = 0x130060cb; continue;
 }
 0x130060cb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi | 0x6u32; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130060ec; continue; }
  pc = 0x130060e0; continue;
 }
 0x130060e0 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130060e9); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130060ec; continue;
 }
 0x130060ec => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006100; continue; }
  pc = 0x130060f4; continue;
 }
 0x130060f4 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x38u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x130060fd); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006100; continue;
 }
 0x13006100 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300611b; continue; }
  pc = 0x1300610f; continue;
 }
 0x1300610f => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006118); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300611b; continue;
 }
 0x1300611b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006130); }
 let mut pc: u32 = 0x13006130;
 loop { match pc {
 0x13006130 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006151; continue; }
  pc = 0x13006145; continue;
 }
 0x13006145 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300614e); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006151; continue;
 }
 0x13006151 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x1300615f); f_13009ea0(cpu);
  { let v: u32 = cpu.r32(0x13013138u32); cpu.edi = v; }
  pc = 0x13006165; continue;
 }
 0x13006165 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x13006184; continue; }
  pc = 0x1300616a; continue;
 }
 0x1300616a => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x1300616d); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130061bb; continue; }
  pc = 0x13006174; continue;
 }
 0x13006174 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006182); f_13009dd0(cpu);
  pc = 0x13006165; continue;
 }
 0x13006184 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x1u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130061a6; continue; }
  pc = 0x1300619a; continue;
 }
 0x1300619a => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130061a3); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130061a6; continue;
 }
 0x130061a6 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130061bb; continue; }
  pc = 0x130061af; continue;
 }
 0x130061af => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130061b8); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130061bb; continue;
 }
 0x130061bb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130061d6; continue; }
  pc = 0x130061ca; continue;
 }
 0x130061ca => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130061d3); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130061d6; continue;
 }
 0x130061d6 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130061e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130061e0); }
 let mut pc: u32 = 0x130061e0;
 loop { match pc {
 0x130061e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300626fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x13006227; continue; }
  pc = 0x1300620e; continue;
 }
 0x1300620e => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c030u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300621c); f_13004670(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13006227; continue;
 }
 0x13006227 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13006230); f_13005a50(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c03cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_1300625d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300625d); }
 let mut pc: u32 = 0x1300625d;
 loop { match pc {
 0x1300625d => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300626e; continue; }
  pc = 0x13006263; continue;
 }
 0x13006263 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300626e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300626f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300626f); }
 let mut pc: u32 = 0x1300626f;
 loop { match pc {
 0x1300626f => {
  { let v: u32 = 0x1300d0f0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006279(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006279); }
 let mut pc: u32 = 0x13006279;
 loop { match pc {
 0x13006279 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006290); }
 let mut pc: u32 = 0x13006290;
 loop { match pc {
 0x13006290 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13006323u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x130062d7; continue; }
  pc = 0x130062be; continue;
 }
 0x130062be => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c030u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130062cc); f_13004670(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130062d7; continue;
 }
 0x130062d7 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130062e4); f_13005af0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c03cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006311(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006311); }
 let mut pc: u32 = 0x13006311;
 loop { match pc {
 0x13006311 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13006322; continue; }
  pc = 0x13006317; continue;
 }
 0x13006317 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13006322 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006323(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006323); }
 let mut pc: u32 = 0x13006323;
 loop { match pc {
 0x13006323 => {
  { let v: u32 = 0x1300d120u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300632d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300632d); }
 let mut pc: u32 = 0x1300632d;
 loop { match pc {
 0x1300632d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006340); }
 let mut pc: u32 = 0x13006340;
 loop { match pc {
 0x13006340 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13006385u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c03cu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13006378); f_1300638f(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006385(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006385); }
 let mut pc: u32 = 0x13006385;
 loop { match pc {
 0x13006385 => {
  { let v: u32 = 0x1300d150u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300638f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300638f); }
 let mut pc: u32 = 0x1300638f;
 loop { match pc {
 0x1300638f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13005c60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130063a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130063a0); }
 let mut pc: u32 = 0x130063a0;
 loop { match pc {
 0x130063a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x130063ac); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006437; continue; }
  pc = 0x130063b4; continue;
 }
 0x130063b4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  pc = 0x130063bd; continue;
 }
 0x130063bd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x130063cb); f_13009e20(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x13006403; continue; }
  pc = 0x130063d0; continue;
 }
 0x130063d0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edi)), cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x130063e7; continue; }
  pc = 0x130063d8; continue;
 }
 0x130063d8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x130063e0); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130063bd; continue; }
  pc = 0x130063e5; continue;
 }
 0x130063e5 => {
  pc = 0x130063f8; continue;
 }
 0x130063e7 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130063bd; continue; }
  pc = 0x130063f8; continue;
 }
 0x130063f8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x130063bd; continue;
 }
 0x13006403 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300641e; continue; }
  pc = 0x13006412; continue;
 }
 0x13006412 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300641b); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300641e; continue;
 }
 0x1300641e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006437; continue; }
  pc = 0x1300642b; continue;
 }
 0x1300642b => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006434); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13006437; continue;
 }
 0x13006437 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006440); }
 let mut pc: u32 = 0x13006440;
 loop { match pc {
 0x13006440 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1300644d); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130064f8; continue; }
  pc = 0x13006455; continue;
 }
 0x13006455 => {
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.esp)) as i8 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x1300645e; continue;
 }
 0x1300645e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x1300646c); f_13009ea0(cpu);
  { cpu.sub32(cpu.eax, cpu.ebp); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  if cpu.zf { pc = 0x130064c4; continue; }
  pc = 0x13006472; continue;
 }
 0x13006472 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x130064bc; continue; }
  pc = 0x13006479; continue;
 }
 0x13006479 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006485); f_13009e60(cpu);
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.edi))); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1300649f; continue; }
  pc = 0x13006490; continue;
 }
 0x13006490 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x13006498); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300645e; continue; }
  pc = 0x1300649d; continue;
 }
 0x1300649d => {
  pc = 0x130064b0; continue;
 }
 0x1300649f => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300645e; continue; }
  pc = 0x130064b0; continue;
 }
 0x130064b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x1300645e; continue;
 }
 0x130064bc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))) | 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x130064c4; continue;
 }
 0x130064c4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130064df; continue; }
  pc = 0x130064d3; continue;
 }
 0x130064d3 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130064dc); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130064df; continue;
 }
 0x130064df => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130064f8; continue; }
  pc = 0x130064ec; continue;
 }
 0x130064ec => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130064f5); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130064f8; continue;
 }
 0x130064f8 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006510); }
 let mut pc: u32 = 0x13006510;
 loop { match pc {
 0x13006510 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006531; continue; }
  pc = 0x13006525; continue;
 }
 0x13006525 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300652e); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006531; continue;
 }
 0x13006531 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.push(0x13006546); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13006582; continue; }
  pc = 0x1300654b; continue;
 }
 0x1300654b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300656d; continue; }
  pc = 0x13006561; continue;
 }
 0x13006561 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300656a); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300656d; continue;
 }
 0x1300656d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006582; continue; }
  pc = 0x13006576; continue;
 }
 0x13006576 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300657f); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006582; continue;
 }
 0x13006582 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300659d; continue; }
  pc = 0x13006591; continue;
 }
 0x13006591 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300659a); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300659d; continue;
 }
 0x1300659d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130065b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130065b0); }
 let mut pc: u32 = 0x130065b0;
 loop { match pc {
 0x130065b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130065d1; continue; }
  pc = 0x130065c5; continue;
 }
 0x130065c5 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130065ce); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130065d1; continue;
 }
 0x130065d1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.push(0x130065eb); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13006627; continue; }
  pc = 0x130065f0; continue;
 }
 0x130065f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006612; continue; }
  pc = 0x13006606; continue;
 }
 0x13006606 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300660f); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006612; continue;
 }
 0x13006612 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006627; continue; }
  pc = 0x1300661b; continue;
 }
 0x1300661b => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13006624); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006627; continue;
 }
 0x13006627 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006642; continue; }
  pc = 0x13006636; continue;
 }
 0x13006636 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300663f); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13006642; continue;
 }
 0x13006642 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006650); }
 let mut pc: u32 = 0x13006650;
 loop { match pc {
 0x13006650 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006674; continue; }
  pc = 0x13006668; continue;
 }
 0x13006668 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006671); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006674; continue;
 }
 0x13006674 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x13006688); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130066c6; continue; }
  pc = 0x13006691; continue;
 }
 0x13006691 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | 0x2u32; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130066b2; continue; }
  pc = 0x130066a6; continue;
 }
 0x130066a6 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130066af); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130066b2; continue;
 }
 0x130066b2 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130066c6; continue; }
  pc = 0x130066ba; continue;
 }
 0x130066ba => {
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x130066c3); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130066c6; continue;
 }
 0x130066c6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130066e1; continue; }
  pc = 0x130066d5; continue;
 }
 0x130066d5 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130066de); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130066e1; continue;
 }
 0x130066e1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130066f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130066f0); }
 let mut pc: u32 = 0x130066f0;
 loop { match pc {
 0x130066f0 => {
  { let v: u32 = 0x1cu32; cpu.eax = v; }
  cpu.push(0x130066fa); f_1300af50(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13006704); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130067fe; continue; }
  pc = 0x1300670c; continue;
 }
 0x1300670c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x14u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300671a); f_13006810(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x130067ca; continue; }
  pc = 0x13006722; continue;
 }
 0x13006722 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013134u32); cpu.push(0x1300672f); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf64(0x1300c1c0u32); cpu.fcom(a, b);  }
  { let v = cpu.st(0); cpu.wf64(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x1300674f; continue; }
  pc = 0x13006743; continue;
 }
 0x13006743 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x7f7fffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x130067ca; continue;
 }
 0x1300674f => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x1300c1d0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x1300676c; continue; }
  pc = 0x13006760; continue;
 }
 0x13006760 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xff7fffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x130067ca; continue;
 }
 0x1300676c => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x13006796; continue; }
  pc = 0x13006779; continue;
 }
 0x13006779 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x1300c1e8u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x13006796; continue; }
  pc = 0x1300678a; continue;
 }
 0x1300678a => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x800000u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x130067ca; continue;
 }
 0x13006796 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x130067c0; continue; }
  pc = 0x130067a3; continue;
 }
 0x130067a3 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x1300c1f8u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x130067c0; continue; }
  pc = 0x130067b4; continue;
 }
 0x130067b4 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x80800000u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x130067ca; continue;
 }
 0x130067c0 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x130067ca; continue;
 }
 0x130067ca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130067e5; continue; }
  pc = 0x130067d9; continue;
 }
 0x130067d9 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130067e2); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130067e5; continue;
 }
 0x130067e5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130067fe; continue; }
  pc = 0x130067f2; continue;
 }
 0x130067f2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130067fb); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130067fe; continue;
 }
 0x130067fe => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006810); }
 let mut pc: u32 = 0x13006810;
 loop { match pc {
 0x13006810 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1300682b); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006979; continue; }
  pc = 0x13006833; continue;
 }
 0x13006833 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006841); f_13009ea0(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x130068d4; continue; }
  pc = 0x1300684d; continue;
 }
 0x1300684d => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x130068ca; continue; }
  pc = 0x13006852; continue;
 }
 0x13006852 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1300685b; continue; }
  pc = 0x13006856; continue;
 }
 0x13006856 => {
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if !cpu.zf { pc = 0x13006865; continue; }
  pc = 0x1300685b; continue;
 }
 0x1300685b => {
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if cpu.zf { pc = 0x130068a9; continue; }
  pc = 0x13006860; continue;
 }
 0x13006860 => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if cpu.zf { pc = 0x130068a9; continue; }
  pc = 0x13006865; continue;
 }
 0x13006865 => {
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if !cpu.zf { pc = 0x1300687b; continue; }
  pc = 0x1300686a; continue;
 }
 0x1300686a => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x1300687b; continue; }
  pc = 0x1300686e; continue;
 }
 0x1300686e => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1300687b; continue; }
  pc = 0x13006875; continue;
 }
 0x13006875 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x130068a9; continue;
 }
 0x1300687b => {
  { cpu.sub32(cpu.ebx, 0x45u32); }
  if cpu.zf { pc = 0x13006885; continue; }
  pc = 0x13006880; continue;
 }
 0x13006880 => {
  { cpu.sub32(cpu.ebx, 0x65u32); }
  if !cpu.zf { pc = 0x13006890; continue; }
  pc = 0x13006885; continue;
 }
 0x13006885 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x13006890; continue; }
  pc = 0x13006889; continue;
 }
 0x13006889 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x130068a9; continue;
 }
 0x13006890 => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x1301313cu32); cpu.push(0x13006897); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130068d4; continue; }
  pc = 0x1300689e; continue;
 }
 0x1300689e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x130068a5; continue; }
  pc = 0x130068a2; continue;
 }
 0x130068a2 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x130068a9; continue;
 }
 0x130068a5 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x130068a9; continue;
 }
 0x130068a9 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x130068c0); f_13009dd0(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x20u32.wrapping_add(cpu.esp))); }
  if (cpu.sf != cpu.of) { pc = 0x1300684d; continue; }
  pc = 0x130068c8; continue;
 }
 0x130068c8 => {
  pc = 0x130068d4; continue;
 }
 0x130068ca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))) | 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x130068d4; continue;
 }
 0x130068d4 => {
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if !cpu.zf { pc = 0x13006926; continue; }
  pc = 0x130068d9; continue;
 }
 0x130068d9 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if cpu.cf { pc = 0x13006901; continue; }
  pc = 0x130068f1; continue;
 }
 0x130068f1 => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.push(0x130068fa); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1300691c; continue; }
  pc = 0x130068ff; continue;
 }
 0x130068ff => {
  pc = 0x1300690f; continue;
 }
 0x13006901 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0xffu8); }
  if cpu.zf { pc = 0x1300691c; continue; }
  pc = 0x1300690f; continue;
 }
 0x1300690f => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))) & 0xfffffffeu32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x13006926; continue;
 }
 0x1300691c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x13006926; continue;
 }
 0x13006926 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x13006933; continue; }
  pc = 0x1300692d; continue;
 }
 0x1300692d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x20u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x1300693d; continue; }
  pc = 0x13006933; continue;
 }
 0x13006933 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x1300693d; continue;
 }
 0x1300693d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006960; continue; }
  pc = 0x13006954; continue;
 }
 0x13006954 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300695d); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006960; continue;
 }
 0x13006960 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006979; continue; }
  pc = 0x1300696d; continue;
 }
 0x1300696d => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006976); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006979; continue;
 }
 0x13006979 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006990); }
 let mut pc: u32 = 0x13006990;
 loop { match pc {
 0x13006990 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1300699b); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130069fc; continue; }
  pc = 0x1300699f; continue;
 }
 0x1300699f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x130069ad); f_13009e20(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x130069c0; continue; }
  pc = 0x130069b4; continue;
 }
 0x130069b4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x130069c3; continue;
 }
 0x130069c0 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x130069c3; continue;
 }
 0x130069c3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130069de; continue; }
  pc = 0x130069d2; continue;
 }
 0x130069d2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130069db); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130069de; continue;
 }
 0x130069de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130069f7; continue; }
  pc = 0x130069eb; continue;
 }
 0x130069eb => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130069f4); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130069f7; continue;
 }
 0x130069f7 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130069fc => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006a10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006a10); }
 let mut pc: u32 = 0x13006a10;
 loop { match pc {
 0x13006a10 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13006a1a); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006a7a; continue; }
  pc = 0x13006a1e; continue;
 }
 0x13006a1e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006a2c); f_13009e20(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13006a3d; continue; }
  pc = 0x13006a31; continue;
 }
 0x13006a31 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x3u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13006a40; continue;
 }
 0x13006a3d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x13006a40; continue;
 }
 0x13006a40 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006a61; continue; }
  pc = 0x13006a55; continue;
 }
 0x13006a55 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006a5e); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006a61; continue;
 }
 0x13006a61 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006a7a; continue; }
  pc = 0x13006a6e; continue;
 }
 0x13006a6e => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006a77); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13006a7a; continue;
 }
 0x13006a7a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006a80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006a80); }
 let mut pc: u32 = 0x13006a80;
 loop { match pc {
 0x13006a80 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x13006a8b); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006aec; continue; }
  pc = 0x13006a8f; continue;
 }
 0x13006a8f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x13006aa7); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.cf { pc = 0x13006ab8; continue; }
  pc = 0x13006aae; continue;
 }
 0x13006aae => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x3u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13006ab8; continue;
 }
 0x13006ab8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006ad3; continue; }
  pc = 0x13006ac7; continue;
 }
 0x13006ac7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006ad0); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006ad3; continue;
 }
 0x13006ad3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006aec; continue; }
  pc = 0x13006ae0; continue;
 }
 0x13006ae0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006ae9); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13006aec; continue;
 }
 0x13006aec => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006b00); }
 let mut pc: u32 = 0x13006b00;
 loop { match pc {
 0x13006b00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x13006b0f); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006bdc; continue; }
  pc = 0x13006b17; continue;
 }
 0x13006b17 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006ba1; continue; }
  pc = 0x13006b22; continue;
 }
 0x13006b22 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13006b9c; continue; }
  pc = 0x13006b2a; continue;
 }
 0x13006b2a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006b38); f_13009ea0(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x13006b63; continue; }
  pc = 0x13006b3d; continue;
 }
 0x13006b3d => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1cu32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x13006b80; continue; }
  pc = 0x13006b43; continue;
 }
 0x13006b43 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x13006b4b; continue; }
  pc = 0x13006b47; continue;
 }
 0x13006b47 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x13006b4b; continue;
 }
 0x13006b4b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006b5a); f_13009e60(cpu);
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if cpu.cf { pc = 0x13006b2a; continue; }
  pc = 0x13006b5e; continue;
 }
 0x13006b5e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebx); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x13006ba5; continue;
 }
 0x13006b63 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))) | 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x13006b9c; continue; }
  pc = 0x13006b71; continue;
 }
 0x13006b71 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebx); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x13006ba5; continue;
 }
 0x13006b80 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); }
  if cpu.zf { pc = 0x13006b9c; continue; }
  pc = 0x13006b86; continue;
 }
 0x13006b86 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.edi))); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006b97); f_13009e60(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebx); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x13006ba5; continue;
 }
 0x13006b9c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebx); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x13006ba5; continue;
 }
 0x13006ba1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x13006ba5; continue;
 }
 0x13006ba5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006bc0; continue; }
  pc = 0x13006bb4; continue;
 }
 0x13006bb4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006bbd); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006bc0; continue;
 }
 0x13006bc0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006bd9; continue; }
  pc = 0x13006bcd; continue;
 }
 0x13006bcd => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006bd6); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006bd9; continue;
 }
 0x13006bd9 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x13006be4; continue;
 }
 0x13006bdc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x13006be4; continue;
 }
 0x13006be4 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x13006bf1; continue; }
  pc = 0x13006be8; continue;
 }
 0x13006be8 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13006bf1; continue; }
  pc = 0x13006bec; continue;
 }
 0x13006bec => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x13006bf1; continue;
 }
 0x13006bf1 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006c10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006c10); }
 let mut pc: u32 = 0x13006c10;
 loop { match pc {
 0x13006c10 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x10u32); }
  if cpu.zf { pc = 0x13006c41; continue; }
  pc = 0x13006c3a; continue;
 }
 0x13006c3a => {
  { let v: u32 = 0xau32; cpu.edi = v; }
  pc = 0x13006c56; continue;
 }
 0x13006c41 => {
  { cpu.and32(cpu.edi, 0x40u32); }
  if cpu.zf { pc = 0x13006c50; continue; }
  pc = 0x13006c49; continue;
 }
 0x13006c49 => {
  { let v: u32 = 0x10u32; cpu.edi = v; }
  pc = 0x13006c56; continue;
 }
 0x13006c50 => {
  { let v: u32 = cpu.edi & 0x20u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 2u32); cpu.edi = v; }
  pc = 0x13006c56; continue;
 }
 0x13006c56 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13006c5f); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006de1; continue; }
  pc = 0x13006c67; continue;
 }
 0x13006c67 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006c77); f_13009ea0(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x13006c79; continue;
 }
 0x13006c79 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x13006d26; continue; }
  pc = 0x13006c82; continue;
 }
 0x13006c82 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x13006c96; continue; }
  pc = 0x13006c86; continue;
 }
 0x13006c86 => {
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if cpu.zf { pc = 0x13006c90; continue; }
  pc = 0x13006c8b; continue;
 }
 0x13006c8b => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if !cpu.zf { pc = 0x13006c96; continue; }
  pc = 0x13006c90; continue;
 }
 0x13006c90 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x13006d02; continue;
 }
 0x13006c96 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), cpu.ebp); }
  if !cpu.zf { pc = 0x13006cc3; continue; }
  pc = 0x13006c9c; continue;
 }
 0x13006c9c => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x30u8); }
  if !cpu.zf { pc = 0x13006cc3; continue; }
  pc = 0x13006ca7; continue;
 }
 0x13006ca7 => {
  { cpu.sub32(cpu.ebx, 0x78u32); }
  if cpu.zf { pc = 0x13006cb1; continue; }
  pc = 0x13006cac; continue;
 }
 0x13006cac => {
  { cpu.sub32(cpu.ebx, 0x58u32); }
  if !cpu.zf { pc = 0x13006cba; continue; }
  pc = 0x13006cb1; continue;
 }
 0x13006cb1 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13006cd8; continue; }
  pc = 0x13006cb5; continue;
 }
 0x13006cb5 => {
  { cpu.sub32(cpu.edi, 0x10u32); }
  if cpu.zf { pc = 0x13006cd8; continue; }
  pc = 0x13006cba; continue;
 }
 0x13006cba => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x13006cc3; continue; }
  pc = 0x13006cbe; continue;
 }
 0x13006cbe => {
  { let v: u32 = 0x8u32; cpu.edi = v; }
  pc = 0x13006cc3; continue;
 }
 0x13006cc3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.edi, 0x10u32); }
  if !cpu.zf { pc = 0x13006ce7; continue; }
  pc = 0x13006cc9; continue;
 }
 0x13006cc9 => {
  let t = cpu.r32(0x13013140u32); cpu.push(0x13006ccf); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13006cfe; continue; }
  pc = 0x13006cd6; continue;
 }
 0x13006cd6 => {
  pc = 0x13006d30; continue;
 }
 0x13006cd8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32; cpu.edi = v; }
  pc = 0x13006d02; continue;
 }
 0x13006ce7 => {
  let t = cpu.r32(0x1301313cu32); cpu.push(0x13006ced); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006d30; continue; }
  pc = 0x13006cf4; continue;
 }
 0x13006cf4 => {
  { cpu.sub32(cpu.edi, 0x8u32); }
  if !cpu.zf { pc = 0x13006cfe; continue; }
  pc = 0x13006cf9; continue;
 }
 0x13006cf9 => {
  { cpu.sub32(cpu.ebx, 0x37u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x13006d30; continue; }
  pc = 0x13006cfe; continue;
 }
 0x13006cfe => {
  { let v: u32 = cpu.inc32(cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x13006d02; continue;
 }
 0x13006d02 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13006d19); f_13009dd0(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, 0xfu32); }
  if (cpu.sf != cpu.of) { pc = 0x13006c79; continue; }
  pc = 0x13006d24; continue;
 }
 0x13006d24 => {
  pc = 0x13006d30; continue;
 }
 0x13006d26 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13006d30; continue;
 }
 0x13006d30 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x13006d7f; continue; }
  pc = 0x13006d37; continue;
 }
 0x13006d37 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x13006d7d; continue; }
  pc = 0x13006d48; continue;
 }
 0x13006d48 => {
  { let v: u32 = 0xfffffffeu32; cpu.ebx = v; }
  pc = 0x13006d4d; continue;
 }
 0x13006d4d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if cpu.cf { pc = 0x13006dbd; continue; }
  pc = 0x13006d65; continue;
 }
 0x13006d65 => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.push(0x13006d6e); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13006dcb; continue; }
  pc = 0x13006d73; continue;
 }
 0x13006d73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13006d7d; continue;
 }
 0x13006d7d => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x13006d7f; continue;
 }
 0x13006d7f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006da2; continue; }
  pc = 0x13006d96; continue;
 }
 0x13006d96 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006d9f); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006da2; continue;
 }
 0x13006da2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006de5; continue; }
  pc = 0x13006daf; continue;
 }
 0x13006daf => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006db8); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006de5; continue;
 }
 0x13006dbd => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0xffu8); }
  if cpu.zf { pc = 0x13006d73; continue; }
  pc = 0x13006dcb; continue;
 }
 0x13006dcb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) & cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x13006d4d; continue; }
  pc = 0x13006ddf; continue;
 }
 0x13006ddf => {
  pc = 0x13006d7d; continue;
 }
 0x13006de1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x13006de5; continue;
 }
 0x13006de5 => {
  { cpu.sub32(cpu.ebp, 0x10u32); }
  if !cpu.zf { pc = 0x13006df4; continue; }
  pc = 0x13006dea; continue;
 }
 0x13006dea => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13006df4; continue;
 }
 0x13006df4 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006e00); }
 let mut pc: u32 = 0x13006e00;
 loop { match pc {
 0x13006e00 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13006e0d); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006ea2; continue; }
  pc = 0x13006e15; continue;
 }
 0x13006e15 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006e21); f_13006c10(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013144u32); cpu.push(0x13006e2f); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x7fffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13006e4f; continue; }
  pc = 0x13006e39; continue;
 }
 0x13006e39 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x7fffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13006e6e; continue;
 }
 0x13006e4f => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006e6c; continue; }
  pc = 0x13006e5a; continue;
 }
 0x13006e5a => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13006e6e; continue;
 }
 0x13006e6c => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x13006e6e; continue;
 }
 0x13006e6e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006e89; continue; }
  pc = 0x13006e7d; continue;
 }
 0x13006e7d => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006e86); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006e89; continue;
 }
 0x13006e89 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006ea2; continue; }
  pc = 0x13006e96; continue;
 }
 0x13006e96 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006e9f); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006ea2; continue;
 }
 0x13006ea2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006eb0); }
 let mut pc: u32 = 0x13006eb0;
 loop { match pc {
 0x13006eb0 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x13006eba); f_1300af50(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13006ec4); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006f24; continue; }
  pc = 0x13006ec8; continue;
 }
 0x13006ec8 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x20u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13006ed6); f_13006810(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13006ef0; continue; }
  pc = 0x13006eda; continue;
 }
 0x13006eda => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013134u32); cpu.push(0x13006ee7); cpu.call(t);
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ecx), v); cpu.fpop(); }
  pc = 0x13006ef0; continue;
 }
 0x13006ef0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006f0b; continue; }
  pc = 0x13006eff; continue;
 }
 0x13006eff => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006f08); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006f0b; continue;
 }
 0x13006f0b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006f24; continue; }
  pc = 0x13006f18; continue;
 }
 0x13006f18 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006f21); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006f24; continue;
 }
 0x13006f24 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006f30); }
 let mut pc: u32 = 0x13006f30;
 loop { match pc {
 0x13006f30 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13006f3d); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13006fad; continue; }
  pc = 0x13006f41; continue;
 }
 0x13006f41 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006f4d); f_13006c10(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013144u32); cpu.push(0x13006f5b); cpu.call(t);
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  let t = cpu.r32(0x13013148u32); cpu.push(0x13006f6a); cpu.call(t);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x22u32); }
  if !cpu.zf { pc = 0x13006f79; continue; }
  pc = 0x13006f6f; continue;
 }
 0x13006f6f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13006f79; continue;
 }
 0x13006f79 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006f94; continue; }
  pc = 0x13006f88; continue;
 }
 0x13006f88 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006f91); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006f94; continue;
 }
 0x13006f94 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13006fad; continue; }
  pc = 0x13006fa1; continue;
 }
 0x13006fa1 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006faa); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13006fad; continue;
 }
 0x13006fad => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13006fc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13006fc0); }
 let mut pc: u32 = 0x13006fc0;
 loop { match pc {
 0x13006fc0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13006fcd); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13007061; continue; }
  pc = 0x13006fd5; continue;
 }
 0x13006fd5 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13006fe1); f_13006c10(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013144u32); cpu.push(0x13006fef); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1300700e; continue; }
  pc = 0x13006ff9; continue;
 }
 0x13006ff9 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = 0x7fffu16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x1300702d; continue;
 }
 0x1300700e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300702a; continue; }
  pc = 0x13007019; continue;
 }
 0x13007019 => {
  { let v: u16 = 0x8000u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x1300702d; continue;
 }
 0x1300702a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1300702d; continue;
 }
 0x1300702d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007048; continue; }
  pc = 0x1300703c; continue;
 }
 0x1300703c => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007045); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007048; continue;
 }
 0x13007048 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007061; continue; }
  pc = 0x13007055; continue;
 }
 0x13007055 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300705e); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007061; continue;
 }
 0x13007061 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007070); }
 let mut pc: u32 = 0x13007070;
 loop { match pc {
 0x13007070 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1300707e); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13007112; continue; }
  pc = 0x13007086; continue;
 }
 0x13007086 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007092); f_13006c10(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1301314cu32); cpu.push(0x130070a0); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x130070b5; continue; }
  pc = 0x130070aa; continue;
 }
 0x130070aa => {
  { cpu.sub32(cpu.edi, 0x7fffffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x130070c2; continue; }
  pc = 0x130070b2; continue;
 }
 0x130070b2 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  pc = 0x130070b5; continue;
 }
 0x130070b5 => {
  if !cpu.zf { pc = 0x130070d8; continue; }
  pc = 0x130070b7; continue;
 }
 0x130070b7 => {
  let t = cpu.r32(0x13013148u32); cpu.push(0x130070bd); cpu.call(t);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x22u32); }
  if !cpu.zf { pc = 0x130070d8; continue; }
  pc = 0x130070c2; continue;
 }
 0x130070c2 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x130070de; continue;
 }
 0x130070d8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x130070de; continue;
 }
 0x130070de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130070f9; continue; }
  pc = 0x130070ed; continue;
 }
 0x130070ed => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130070f6); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130070f9; continue;
 }
 0x130070f9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007112; continue; }
  pc = 0x13007106; continue;
 }
 0x13007106 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300710f); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007112; continue;
 }
 0x13007112 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007120); }
 let mut pc: u32 = 0x13007120;
 loop { match pc {
 0x13007120 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1300712d); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130071a2; continue; }
  pc = 0x13007131; continue;
 }
 0x13007131 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300713d); f_13006c10(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1301314cu32); cpu.push(0x1300714b); cpu.call(t);
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x1300716e; continue; }
  pc = 0x13007159; continue;
 }
 0x13007159 => {
  let t = cpu.r32(0x13013148u32); cpu.push(0x1300715f); cpu.call(t);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x22u32); }
  if !cpu.zf { pc = 0x1300716e; continue; }
  pc = 0x13007164; continue;
 }
 0x13007164 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x1300716e; continue;
 }
 0x1300716e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007189; continue; }
  pc = 0x1300717d; continue;
 }
 0x1300717d => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007186); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007189; continue;
 }
 0x13007189 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130071a2; continue; }
  pc = 0x13007196; continue;
 }
 0x13007196 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300719f); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130071a2; continue;
 }
 0x130071a2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130071b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130071b0); }
 let mut pc: u32 = 0x130071b0;
 loop { match pc {
 0x130071b0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x130071be); f_13005d40(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13007255; continue; }
  pc = 0x130071c6; continue;
 }
 0x130071c6 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130071d2); f_13006c10(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1301314cu32); cpu.push(0x130071e0); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x130071f5; continue; }
  pc = 0x130071ed; continue;
 }
 0x130071ed => {
  { cpu.sub32(cpu.edi, 0xffff7fffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x13007205; continue; }
  pc = 0x130071f5; continue;
 }
 0x130071f5 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300721a; continue; }
  pc = 0x130071fa; continue;
 }
 0x130071fa => {
  let t = cpu.r32(0x13013148u32); cpu.push(0x13007200); cpu.call(t);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x22u32); }
  if !cpu.zf { pc = 0x1300721a; continue; }
  pc = 0x13007205; continue;
 }
 0x13007205 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13007221; continue;
 }
 0x1300721a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x13007221; continue;
 }
 0x13007221 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300723c; continue; }
  pc = 0x13007230; continue;
 }
 0x13007230 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007239); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300723c; continue;
 }
 0x1300723c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007255; continue; }
  pc = 0x13007249; continue;
 }
 0x13007249 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007252); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007255; continue;
 }
 0x13007255 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007260); }
 let mut pc: u32 = 0x13007260;
 loop { match pc {
 0x13007260 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013120u32); cpu.push(0x1300726b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007270); }
 let mut pc: u32 = 0x13007270;
 loop { match pc {
 0x13007270 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1301311cu32); cpu.push(0x1300727b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007280); }
 let mut pc: u32 = 0x13007280;
 loop { match pc {
 0x13007280 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013118u32); cpu.push(0x1300728b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007290); }
 let mut pc: u32 = 0x13007290;
 loop { match pc {
 0x13007290 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013114u32); cpu.push(0x1300729b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130072a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130072a0); }
 let mut pc: u32 = 0x130072a0;
 loop { match pc {
 0x130072a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130072a8); f_13007690(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x130072b8; continue; }
  pc = 0x130072af; continue;
 }
 0x130072af => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130072b5); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130072b8; continue;
 }
 0x130072b8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130072c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130072c0); }
 let mut pc: u32 = 0x130072c0;
 loop { match pc {
 0x130072c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x130072f3; continue; }
  pc = 0x130072d0; continue;
 }
 0x130072d0 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v = 0x13007690u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130072e3); f_1300ad20(cpu);
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130072e9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130072f3 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x130072fa); f_13007690(cpu);
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x1300730b; continue; }
  pc = 0x13007302; continue;
 }
 0x13007302 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13007308); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300730b; continue;
 }
 0x1300730b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007320); }
 let mut pc: u32 = 0x13007320;
 loop { match pc {
 0x13007320 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13007371u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1300734a); f_13007590(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c20cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_13007371(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007371); }
 let mut pc: u32 = 0x13007371;
 loop { match pc {
 0x13007371 => {
  { let v: u32 = 0x1300d178u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300737b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300737b); }
 let mut pc: u32 = 0x1300737b;
 loop { match pc {
 0x1300737b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13007690(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007390); }
 let mut pc: u32 = 0x13007390;
 loop { match pc {
 0x13007390 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130073e1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130073ba); f_130075f0(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c20cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_130073e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130073e1); }
 let mut pc: u32 = 0x130073e1;
 loop { match pc {
 0x130073e1 => {
  { let v: u32 = 0x1300d1a0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130073eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130073eb); }
 let mut pc: u32 = 0x130073eb;
 loop { match pc {
 0x130073eb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13007690(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007400); }
 let mut pc: u32 = 0x13007400;
 loop { match pc {
 0x13007400 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300740d); f_13007670(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007420); }
 let mut pc: u32 = 0x13007420;
 loop { match pc {
 0x13007420 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13007457u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1300744a); f_13007461(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007457(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007457); }
 let mut pc: u32 = 0x13007457;
 loop { match pc {
 0x13007457 => {
  { let v: u32 = 0x1300d1c8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007461(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007461); }
 let mut pc: u32 = 0x13007461;
 loop { match pc {
 0x13007461 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13007690(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007470); }
 let mut pc: u32 = 0x13007470;
 loop { match pc {
 0x13007470 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130074bfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1300749a); f_130074c9(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x130074ac; continue; }
  pc = 0x130074a0; continue;
 }
 0x130074a0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130074a9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130074ac; continue;
 }
 0x130074ac => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130074bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130074bf); }
 let mut pc: u32 = 0x130074bf;
 loop { match pc {
 0x130074bf => {
  { let v: u32 = 0x1300d1f0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130074c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130074c9); }
 let mut pc: u32 = 0x130074c9;
 loop { match pc {
 0x130074c9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13007690(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130074e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130074e0); }
 let mut pc: u32 = 0x130074e0;
 loop { match pc {
 0x130074e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13007550u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x13007538; continue; }
  pc = 0x13007505; continue;
 }
 0x13007505 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x13007420u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300751b); f_1300ad20(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x1300751c; continue;
 }
 0x1300751c => {
  cpu.push(0x13007521); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13007524; continue;
 }
 0x13007524 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13007538 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13007544); f_1300755a(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13007524; continue; }
  pc = 0x1300754a; continue;
 }
 0x1300754a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x1300751c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007550); }
 let mut pc: u32 = 0x13007550;
 loop { match pc {
 0x13007550 => {
  { let v: u32 = 0x1300d218u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300755a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300755a); }
 let mut pc: u32 = 0x1300755a;
 loop { match pc {
 0x1300755a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13007690(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007570); }
 let mut pc: u32 = 0x13007570;
 loop { match pc {
 0x13007570 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1300c21cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007590); }
 let mut pc: u32 = 0x13007590;
 loop { match pc {
 0x13007590 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1300c21cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x130075b2); f_1300ae80(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x130075df; continue; }
  pc = 0x130075be; continue;
 }
 0x130075be => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x130075df; continue;
 }
 0x130075df => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130075f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130075f0); }
 let mut pc: u32 = 0x130075f0;
 loop { match pc {
 0x130075f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1300c21cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13007653; continue; }
  pc = 0x13007609; continue;
 }
 0x13007609 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1300761d); f_1300ae80(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x13007659; continue; }
  pc = 0x13007629; continue;
 }
 0x13007629 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13007653 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x13007659; continue;
 }
 0x13007659 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007670); }
 let mut pc: u32 = 0x13007670;
 loop { match pc {
 0x13007670 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.zf { pc = 0x13007689; continue; }
  pc = 0x1300767c; continue;
 }
 0x1300767c => {
  cpu.push(0x13007681); f_13007690(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x13007689); f_130075f0(cpu);
  pc = 0x13007689; continue;
 }
 0x13007689 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007690); }
 let mut pc: u32 = 0x13007690;
 loop { match pc {
 0x13007690 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = 0x1300c21cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if cpu.zf { pc = 0x130076a8; continue; }
  pc = 0x1300769c; continue;
 }
 0x1300769c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130076a5); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130076a8; continue;
 }
 0x130076a8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130076b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130076b0); }
 let mut pc: u32 = 0x130076b0;
 loop { match pc {
 0x130076b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x130076bc; continue; }
  pc = 0x130076b7; continue;
 }
 0x130076b7 => {
  { let v: u32 = 0x1300c224u32; cpu.eax = v; }
  pc = 0x130076bc; continue;
 }
 0x130076bc => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130076c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130076c0); }
 let mut pc: u32 = 0x130076c0;
 loop { match pc {
 0x130076c0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300778fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), cpu.esi); }
  if cpu.zf { pc = 0x13007701; continue; }
  pc = 0x130076ec; continue;
 }
 0x130076ec => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = 0x1300c090u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x130076fa); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13007701; continue;
 }
 0x13007701 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x13007708); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13007727; continue; }
  pc = 0x13007720; continue;
 }
 0x13007720 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13007727); f_13003080(cpu);
  pc = 0x13007727; continue;
 }
 0x13007727 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007735); f_13008130(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c09cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007770); }
 let mut pc: u32 = 0x13007770;
 loop { match pc {
 0x13007770 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13007781; continue; }
  pc = 0x13007776; continue;
 }
 0x13007776 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13007781 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007782(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007782); }
 let mut pc: u32 = 0x13007782;
 loop { match pc {
 0x13007782 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300778b); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300778f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300778f); }
 let mut pc: u32 = 0x1300778f;
 loop { match pc {
 0x1300778f => {
  { let v: u32 = 0x1300d240u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007799(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007799); }
 let mut pc: u32 = 0x13007799;
 loop { match pc {
 0x13007799 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130077b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130077b0); }
 let mut pc: u32 = 0x130077b0;
 loop { match pc {
 0x130077b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130078aau32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), cpu.edi); }
  if cpu.zf { pc = 0x130077f1; continue; }
  pc = 0x130077dc; continue;
 }
 0x130077dc => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c090u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130077ea); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130077f1; continue;
 }
 0x130077f1 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x130077f8); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13007817; continue; }
  pc = 0x13007810; continue;
 }
 0x13007810 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x13007817); f_13003080(cpu);
  pc = 0x13007817; continue;
 }
 0x13007817 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007825); f_13008130(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c09cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13007862); f_130036d0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13007870; continue; }
  pc = 0x13007866; continue;
 }
 0x13007866 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x2u32); cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13007870; continue;
 }
 0x13007870 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300788b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300788b); }
 let mut pc: u32 = 0x1300788b;
 loop { match pc {
 0x1300788b => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300789c; continue; }
  pc = 0x13007891; continue;
 }
 0x13007891 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300789c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300789d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300789d); }
 let mut pc: u32 = 0x1300789d;
 loop { match pc {
 0x1300789d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130078a6); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130078aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130078aa); }
 let mut pc: u32 = 0x130078aa;
 loop { match pc {
 0x130078aa => {
  { let v: u32 = 0x1300d278u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130078b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130078b4); }
 let mut pc: u32 = 0x130078b4;
 loop { match pc {
 0x130078b4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130078c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130078c0); }
 let mut pc: u32 = 0x130078c0;
 loop { match pc {
 0x130078c0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13007993u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.edi); }
  if cpu.zf { pc = 0x13007901; continue; }
  pc = 0x130078ec; continue;
 }
 0x130078ec => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c090u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130078fa); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13007901; continue;
 }
 0x13007901 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x13007908); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1300792b; continue; }
  pc = 0x13007920; continue;
 }
 0x13007920 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300792b); f_130030f0(cpu);
  pc = 0x1300792b; continue;
 }
 0x1300792b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007939); f_13008130(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c09cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007974(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007974); }
 let mut pc: u32 = 0x13007974;
 loop { match pc {
 0x13007974 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13007985; continue; }
  pc = 0x1300797a; continue;
 }
 0x1300797a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13007985 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007986(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007986); }
 let mut pc: u32 = 0x13007986;
 loop { match pc {
 0x13007986 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300798f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007993(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007993); }
 let mut pc: u32 = 0x13007993;
 loop { match pc {
 0x13007993 => {
  { let v: u32 = 0x1300d2b0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300799d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300799d); }
 let mut pc: u32 = 0x1300799d;
 loop { match pc {
 0x1300799d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130079b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130079b0); }
 let mut pc: u32 = 0x130079b0;
 loop { match pc {
 0x130079b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13007a8bu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), cpu.esi); }
  if cpu.zf { pc = 0x130079f1; continue; }
  pc = 0x130079dc; continue;
 }
 0x130079dc => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = 0x1300c090u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x130079ea); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130079f1; continue;
 }
 0x130079f1 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x130079f8); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13007a23; continue; }
  pc = 0x13007a10; continue;
 }
 0x13007a10 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13007a23); f_13003160(cpu);
  pc = 0x13007a23; continue;
 }
 0x13007a23 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007a31); f_13008130(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c09cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007a6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007a6c); }
 let mut pc: u32 = 0x13007a6c;
 loop { match pc {
 0x13007a6c => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13007a7d; continue; }
  pc = 0x13007a72; continue;
 }
 0x13007a72 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13007a7d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007a7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007a7e); }
 let mut pc: u32 = 0x13007a7e;
 loop { match pc {
 0x13007a7e => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007a87); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007a8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007a8b); }
 let mut pc: u32 = 0x13007a8b;
 loop { match pc {
 0x13007a8b => {
  { let v: u32 = 0x1300d2e8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007a95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007a95); }
 let mut pc: u32 = 0x13007a95;
 loop { match pc {
 0x13007a95 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007aa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007aa0); }
 let mut pc: u32 = 0x13007aa0;
 loop { match pc {
 0x13007aa0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13007ae5u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c09cu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13007ad8); f_13007aef(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007ae5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007ae5); }
 let mut pc: u32 = 0x13007ae5;
 loop { match pc {
 0x13007ae5 => {
  { let v: u32 = 0x1300d320u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007aef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007aef); }
 let mut pc: u32 = 0x13007aef;
 loop { match pc {
 0x13007aef => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13008280(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007b00); }
 let mut pc: u32 = 0x13007b00;
 loop { match pc {
 0x13007b00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x13007b36; continue; }
  pc = 0x13007b14; continue;
 }
 0x13007b14 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.push(0x13007b23); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13007b36; continue; }
  pc = 0x13007b27; continue;
 }
 0x13007b27 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x13007b36 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007b57; continue; }
  pc = 0x13007b4b; continue;
 }
 0x13007b4b => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007b54); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007b57; continue;
 }
 0x13007b57 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007b6b; continue; }
  pc = 0x13007b5f; continue;
 }
 0x13007b5f => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13007b68); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007b6b; continue;
 }
 0x13007b6b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007b80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007b80); }
 let mut pc: u32 = 0x13007b80;
 loop { match pc {
 0x13007b80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13007b98); f_13003650(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13007bd1; continue; }
  pc = 0x13007b9c; continue;
 }
 0x13007b9c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007bbd; continue; }
  pc = 0x13007bb1; continue;
 }
 0x13007bb1 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007bba); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007bbd; continue;
 }
 0x13007bbd => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007bd1; continue; }
  pc = 0x13007bc5; continue;
 }
 0x13007bc5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13007bce); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13007bd1; continue;
 }
 0x13007bd1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007be0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007be0); }
 let mut pc: u32 = 0x13007be0;
 loop { match pc {
 0x13007be0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x13007c0f; continue; }
  pc = 0x13007bf4; continue;
 }
 0x13007bf4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13007c0b); f_130036d0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13007c44; continue; }
  pc = 0x13007c0f; continue;
 }
 0x13007c0f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007c30; continue; }
  pc = 0x13007c24; continue;
 }
 0x13007c24 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007c2d); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007c30; continue;
 }
 0x13007c30 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007c44; continue; }
  pc = 0x13007c38; continue;
 }
 0x13007c38 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13007c41); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13007c44; continue;
 }
 0x13007c44 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007c50); }
 let mut pc: u32 = 0x13007c50;
 loop { match pc {
 0x13007c50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13007c67); f_13003270(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13007c77; continue; }
  pc = 0x13007c6b; continue;
 }
 0x13007c6b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.edi = v; }
  { let v: u32 = cpu.edi | 0x2u32; cpu.edi = v; }
  pc = 0x13007c77; continue;
 }
 0x13007c77 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007c8e; continue; }
  pc = 0x13007c82; continue;
 }
 0x13007c82 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007c8b); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007c8e; continue;
 }
 0x13007c8e => {
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007ca3; continue; }
  pc = 0x13007c97; continue;
 }
 0x13007c97 => {
  { let v: u32 = cpu.esi.wrapping_add(0x38u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13007ca0); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13007ca3; continue;
 }
 0x13007ca3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007cb0); }
 let mut pc: u32 = 0x13007cb0;
 loop { match pc {
 0x13007cb0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13007cbc); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13007d4f; continue; }
  pc = 0x13007cc4; continue;
 }
 0x13007cc4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x13007cf1; continue; }
  pc = 0x13007cd1; continue;
 }
 0x13007cd1 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xbu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xau32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x1300c238u32; cpu.push(v); }
  cpu.push(0x13007cef); f_13008510(cpu);
  pc = 0x13007d48; continue;
 }
 0x13007cf1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x13007d11; continue; }
  pc = 0x13007cfc; continue;
 }
 0x13007cfc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x13007d0a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13007d48; continue; }
  pc = 0x13007d0f; continue;
 }
 0x13007d0f => {
  pc = 0x13007d26; continue;
 }
 0x13007d11 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13007d48; continue; }
  pc = 0x13007d26; continue;
 }
 0x13007d26 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.push(0x13007d39); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13007d48; continue; }
  pc = 0x13007d3e; continue;
 }
 0x13007d3e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13007d48; continue;
 }
 0x13007d48 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13007d4f); f_13007f20(cpu);
  pc = 0x13007d4f; continue;
 }
 0x13007d4f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007d60); }
 let mut pc: u32 = 0x13007d60;
 loop { match pc {
 0x13007d60 => {
  { let v: u32 = 0x24u32; cpu.eax = v; }
  cpu.push(0x13007d6a); f_1300af50(cpu);
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx & 0x9u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x6u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if cpu.cf { pc = 0x13007d94; continue; }
  pc = 0x13007d92; continue;
 }
 0x13007d92 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x13007d94; continue;
 }
 0x13007d94 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13007d9b); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13007e8d; continue; }
  pc = 0x13007da3; continue;
 }
 0x13007da3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x4u8); }
  if cpu.zf { pc = 0x13007dbc; continue; }
  pc = 0x13007db2; continue;
 }
 0x13007db2 => {
  { let v: u8 = 0x2bu8; cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x13007dbc; continue;
 }
 0x13007dbc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x13007dcb; continue; }
  pc = 0x13007dc5; continue;
 }
 0x13007dc5 => {
  { let v: u8 = 0x23u8; cpu.w8(0x10u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x13007dcb; continue;
 }
 0x13007dcb => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v = 0x1300c23cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x13013154u32); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x13007de8); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300c0d8u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & cpu.ebp; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if !cpu.zf { pc = 0x13007e0f; continue; }
  pc = 0x13007e08; continue;
 }
 0x13007e08 => {
  { let v: u8 = 0x66u8; cpu.w8(0x14u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x13007e3a; continue;
 }
 0x13007e0f => {
  { cpu.sub32(cpu.ecx, 0x800u32); }
  if !cpu.zf { pc = 0x13007e1c; continue; }
  pc = 0x13007e17; continue;
 }
 0x13007e17 => {
  { let v: u8 = 0x65u8; cpu.w8(0x14u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x13007e1c; continue;
 }
 0x13007e1c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x2u8); }
  if cpu.zf { pc = 0x13007e3a; continue; }
  pc = 0x13007e25; continue;
 }
 0x13007e25 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013150u32); cpu.push(0x13007e34); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x13007e3a; continue;
 }
 0x13007e3a => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x13007e50); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1cu32.wrapping_add(cpu.esp)), 0x2bu8); }
  if cpu.zf { pc = 0x13007e63; continue; }
  pc = 0x13007e5c; continue;
 }
 0x13007e5c => {
  { cpu.sub8(cpu.r8(0x1cu32.wrapping_add(cpu.esp)), 0x2du8); }
  if !cpu.zf { pc = 0x13007e74; continue; }
  pc = 0x13007e63; continue;
 }
 0x13007e63 => {
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1du32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x13007e74; continue;
 }
 0x13007e74 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007e86); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13007e8d); f_13007f20(cpu);
  pc = 0x13007e8d; continue;
 }
 0x13007e8d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007ea0); }
 let mut pc: u32 = 0x13007ea0;
 loop { match pc {
 0x13007ea0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007ebc; continue; }
  pc = 0x13007eb0; continue;
 }
 0x13007eb0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007eb9); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007ebc; continue;
 }
 0x13007ebc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13007eed; continue; }
  pc = 0x13007eca; continue;
 }
 0x13007eca => {
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007ee9; continue; }
  pc = 0x13007edd; continue;
 }
 0x13007edd => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007ee6); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007ee9; continue;
 }
 0x13007ee9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13007eed => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13007ef9; continue; }
  pc = 0x13007ef4; continue;
 }
 0x13007ef4 => {
  cpu.push(0x13007ef9); f_13008010(cpu);
  pc = 0x13007ef9; continue;
 }
 0x13007ef9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007f14; continue; }
  pc = 0x13007f08; continue;
 }
 0x13007f08 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007f11); f_13007280(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13007f14; continue;
 }
 0x13007f14 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007f20); }
 let mut pc: u32 = 0x13007f20;
 loop { match pc {
 0x13007f20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x30u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x13007f59; continue; }
  pc = 0x13007f3f; continue;
 }
 0x13007f3f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x13007f47); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13007f59; continue; }
  pc = 0x13007f4c; continue;
 }
 0x13007f4c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13007f59; continue;
 }
 0x13007f59 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x40u8); }
  if cpu.zf { pc = 0x13007fa5; continue; }
  pc = 0x13007f65; continue;
 }
 0x13007f65 => {
  { let v: u32 = cpu.r32(0x1301315cu32); cpu.ebx = v; }
  let t = cpu.ebx; cpu.push(0x13007f6d); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x13013158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x13007f79); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13007f8b; continue; }
  pc = 0x13007f81; continue;
 }
 0x13007f81 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13007f8b; continue;
 }
 0x13007f8b => {
  let t = cpu.ebx; cpu.push(0x13007f8d); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x13007f93); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13007fa5; continue; }
  pc = 0x13007f9b; continue;
 }
 0x13007f9b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13007fa5; continue;
 }
 0x13007fa5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007fc0; continue; }
  pc = 0x13007fb4; continue;
 }
 0x13007fb4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007fbd); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13007fc0; continue;
 }
 0x13007fc0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13007fd9; continue; }
  pc = 0x13007fcd; continue;
 }
 0x13007fcd => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13007fd6); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13007fd9; continue;
 }
 0x13007fd9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13007fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13007fe0); }
 let mut pc: u32 = 0x13007fe0;
 loop { match pc {
 0x13007fe0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13007fe8); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008004; continue; }
  pc = 0x13007fec; continue;
 }
 0x13007fec => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1300c238u32; cpu.push(v); }
  cpu.push(0x13007ffd); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008004); f_13007f20(cpu);
  pc = 0x13008004; continue;
 }
 0x13008004 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008010); }
 let mut pc: u32 = 0x13008010;
 loop { match pc {
 0x13008010 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300802c; continue; }
  pc = 0x13008020; continue;
 }
 0x13008020 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13008029); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300802c; continue;
 }
 0x1300802c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13008047; continue; }
  pc = 0x1300803b; continue;
 }
 0x1300803b => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13008044); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13008047; continue;
 }
 0x13008047 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x13008055); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13008064; continue; }
  pc = 0x1300805a; continue;
 }
 0x1300805a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))) | 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13008064; continue;
 }
 0x13008064 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300807f; continue; }
  pc = 0x13008073; continue;
 }
 0x13008073 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300807c); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300807f; continue;
 }
 0x1300807f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13008098; continue; }
  pc = 0x1300808c; continue;
 }
 0x1300808c => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13008095); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13008098; continue;
 }
 0x13008098 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130080a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130080a0); }
 let mut pc: u32 = 0x130080a0;
 loop { match pc {
 0x130080a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13008114u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x130080e7; continue; }
  pc = 0x130080ce; continue;
 }
 0x130080ce => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c248u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130080dc); f_13004670(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130080e7; continue;
 }
 0x130080e7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c254u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008114(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008114); }
 let mut pc: u32 = 0x13008114;
 loop { match pc {
 0x13008114 => {
  { let v: u32 = 0x1300d348u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300811e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300811e); }
 let mut pc: u32 = 0x1300811e;
 loop { match pc {
 0x1300811e => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300812f; continue; }
  pc = 0x13008124; continue;
 }
 0x13008124 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300812f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008130); }
 let mut pc: u32 = 0x13008130;
 loop { match pc {
 0x13008130 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130081b4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x13008177; continue; }
  pc = 0x1300815e; continue;
 }
 0x1300815e => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c248u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300816c); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13008177; continue;
 }
 0x13008177 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c254u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  cpu.push(0x13008193); f_13004820(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130081b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130081b4); }
 let mut pc: u32 = 0x130081b4;
 loop { match pc {
 0x130081b4 => {
  { let v: u32 = 0x1300d370u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130081be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130081be); }
 let mut pc: u32 = 0x130081be;
 loop { match pc {
 0x130081be => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x130081cf; continue; }
  pc = 0x130081c4; continue;
 }
 0x130081c4 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x130081cf => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130081d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130081d0); }
 let mut pc: u32 = 0x130081d0;
 loop { match pc {
 0x130081d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300825du32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x13008217; continue; }
  pc = 0x130081fe; continue;
 }
 0x130081fe => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c248u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300820c); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13008217; continue;
 }
 0x13008217 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c254u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  cpu.push(0x1300823c); f_13004820(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300825d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300825d); }
 let mut pc: u32 = 0x1300825d;
 loop { match pc {
 0x1300825d => {
  { let v: u32 = 0x1300d398u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008267(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008267); }
 let mut pc: u32 = 0x13008267;
 loop { match pc {
 0x13008267 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13008278; continue; }
  pc = 0x1300826d; continue;
 }
 0x1300826d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13008278 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008280); }
 let mut pc: u32 = 0x13008280;
 loop { match pc {
 0x13008280 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x1300c254u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008290); }
 let mut pc: u32 = 0x13008290;
 loop { match pc {
 0x13008290 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13008338u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x130082c8; continue; }
  pc = 0x130082bb; continue;
 }
 0x130082bb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x130082c8; continue; }
  pc = 0x130082c2; continue;
 }
 0x130082c2 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x130082c8); cpu.call(t);
  pc = 0x130082c8; continue;
 }
 0x130082c8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x130082e0); f_13004670(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  cpu.push(0x130082f8); f_13004860(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13008304); f_13008342(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  cpu.push(0x13008325); f_13004820(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008338); }
 let mut pc: u32 = 0x13008338;
 loop { match pc {
 0x13008338 => {
  { let v: u32 = 0x1300d3c0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008342(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008342); }
 let mut pc: u32 = 0x13008342;
 loop { match pc {
 0x13008342 => {
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008350); }
 let mut pc: u32 = 0x13008350;
 loop { match pc {
 0x13008350 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130083dfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x13008397; continue; }
  pc = 0x1300837e; continue;
 }
 0x1300837e => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c040u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300838c); f_13004670(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13008397; continue;
 }
 0x13008397 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130083a0); f_130080a0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c04cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_130083cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130083cd); }
 let mut pc: u32 = 0x130083cd;
 loop { match pc {
 0x130083cd => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x130083de; continue; }
  pc = 0x130083d3; continue;
 }
 0x130083d3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x130083de => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130083df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130083df); }
 let mut pc: u32 = 0x130083df;
 loop { match pc {
 0x130083df => {
  { let v: u32 = 0x1300d3e8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130083e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130083e9); }
 let mut pc: u32 = 0x130083e9;
 loop { match pc {
 0x130083e9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008400); }
 let mut pc: u32 = 0x13008400;
 loop { match pc {
 0x13008400 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13008493u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x13008447; continue; }
  pc = 0x1300842e; continue;
 }
 0x1300842e => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c040u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1300843c); f_13004670(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13008447; continue;
 }
 0x13008447 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008454); f_13008130(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c04cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008481(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008481); }
 let mut pc: u32 = 0x13008481;
 loop { match pc {
 0x13008481 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13008492; continue; }
  pc = 0x13008487; continue;
 }
 0x13008487 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13008492 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008493(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008493); }
 let mut pc: u32 = 0x13008493;
 loop { match pc {
 0x13008493 => {
  { let v: u32 = 0x1300d418u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300849d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300849d); }
 let mut pc: u32 = 0x1300849d;
 loop { match pc {
 0x1300849d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130084b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130084b0); }
 let mut pc: u32 = 0x130084b0;
 loop { match pc {
 0x130084b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130084f5u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c04cu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x130084e8); f_130084ff(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130084f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130084f5); }
 let mut pc: u32 = 0x130084f5;
 loop { match pc {
 0x130084f5 => {
  { let v: u32 = 0x1300d448u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130084ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130084ff); }
 let mut pc: u32 = 0x130084ff;
 loop { match pc {
 0x130084ff => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13008280(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008510); }
 let mut pc: u32 = 0x13008510;
 loop { match pc {
 0x13008510 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.cf { pc = 0x13008557; continue; }
  pc = 0x1300854f; continue;
 }
 0x1300854f => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.esi = v; }
  pc = 0x13008559; continue;
 }
 0x13008557 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x13008559; continue;
 }
 0x13008559 => {
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.eax)), 0xau8); }
  if !cpu.zf { pc = 0x130085ac; continue; }
  pc = 0x1300855f; continue;
 }
 0x1300855f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x130085ac; continue; }
  pc = 0x13008566; continue;
 }
 0x13008566 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1300858a; continue; }
  pc = 0x1300857d; continue;
 }
 0x1300857d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x13008583); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130085a5; continue; }
  pc = 0x13008588; continue;
 }
 0x13008588 => {
  pc = 0x1300859b; continue;
 }
 0x1300858a => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130085a5; continue; }
  pc = 0x1300859b; continue;
 }
 0x1300859b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))) | 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x130085a5; continue;
 }
 0x130085a5 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x13008566; continue; }
  pc = 0x130085ac; continue;
 }
 0x130085ac => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x130085d2; continue; }
  pc = 0x130085b0; continue;
 }
 0x130085b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.push(0x130085c4); cpu.call(t);
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { pc = 0x130085d2; continue; }
  pc = 0x130085c8; continue;
 }
 0x130085c8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))) | 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x130085d2; continue;
 }
 0x130085d2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x8u8); }
  if cpu.zf { pc = 0x1300862f; continue; }
  pc = 0x130085de; continue;
 }
 0x130085de => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1300862f; continue; }
  pc = 0x130085e5; continue;
 }
 0x130085e5 => {
  { let v: u32 = 0x6u32; cpu.edi = v; }
  pc = 0x130085ea; continue;
 }
 0x130085ea => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1300860e; continue; }
  pc = 0x13008601; continue;
 }
 0x13008601 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x13008607); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13008628; continue; }
  pc = 0x1300860c; continue;
 }
 0x1300860c => {
  pc = 0x1300861f; continue;
 }
 0x1300860e => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13008628; continue; }
  pc = 0x1300861f; continue;
 }
 0x1300861f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))) | cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x13008628; continue;
 }
 0x13008628 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x130085ea; continue; }
  pc = 0x1300862f; continue;
 }
 0x1300862f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x13008647); cpu.call(t);
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x13008657; continue; }
  pc = 0x1300864d; continue;
 }
 0x1300864d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))) | 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x13008657; continue;
 }
 0x13008657 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x2u8); }
  if cpu.zf { pc = 0x130086b4; continue; }
  pc = 0x13008663; continue;
 }
 0x13008663 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x130086b4; continue; }
  pc = 0x1300866a; continue;
 }
 0x1300866a => {
  { let v: u32 = 0x6u32; cpu.edi = v; }
  pc = 0x1300866f; continue;
 }
 0x1300866f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x13008693; continue; }
  pc = 0x13008686; continue;
 }
 0x13008686 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x1300868c); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130086ad; continue; }
  pc = 0x13008691; continue;
 }
 0x13008691 => {
  pc = 0x130086a4; continue;
 }
 0x13008693 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130086ad; continue; }
  pc = 0x130086a4; continue;
 }
 0x130086a4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))) | cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x130086ad; continue;
 }
 0x130086ad => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300866f; continue; }
  pc = 0x130086b4; continue;
 }
 0x130086b4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130086c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130086c0); }
 let mut pc: u32 = 0x130086c0;
 loop { match pc {
 0x130086c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130086e1; continue; }
  pc = 0x130086d5; continue;
 }
 0x130086d5 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130086de); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130086e1; continue;
 }
 0x130086e1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.push(0x130086f6); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13008732; continue; }
  pc = 0x130086fb; continue;
 }
 0x130086fb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300871d; continue; }
  pc = 0x13008711; continue;
 }
 0x13008711 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300871a); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300871d; continue;
 }
 0x1300871d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13008732; continue; }
  pc = 0x13008726; continue;
 }
 0x13008726 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300872f); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13008732; continue;
 }
 0x13008732 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300874d; continue; }
  pc = 0x13008741; continue;
 }
 0x13008741 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300874a); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300874d; continue;
 }
 0x1300874d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008760); }
 let mut pc: u32 = 0x13008760;
 loop { match pc {
 0x13008760 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13008781; continue; }
  pc = 0x13008775; continue;
 }
 0x13008775 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300877e); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13008781; continue;
 }
 0x13008781 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.push(0x1300879b); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130087d7; continue; }
  pc = 0x130087a0; continue;
 }
 0x130087a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130087c2; continue; }
  pc = 0x130087b6; continue;
 }
 0x130087b6 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130087bf); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130087c2; continue;
 }
 0x130087c2 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130087d7; continue; }
  pc = 0x130087cb; continue;
 }
 0x130087cb => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130087d4); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130087d7; continue;
 }
 0x130087d7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130087f2; continue; }
  pc = 0x130087e6; continue;
 }
 0x130087e6 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130087ef); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130087f2; continue;
 }
 0x130087f2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008800); }
 let mut pc: u32 = 0x13008800;
 loop { match pc {
 0x13008800 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13008824; continue; }
  pc = 0x13008818; continue;
 }
 0x13008818 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13008821); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13008824; continue;
 }
 0x13008824 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x13008838); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13008876; continue; }
  pc = 0x13008841; continue;
 }
 0x13008841 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi | 0x2u32; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13008862; continue; }
  pc = 0x13008856; continue;
 }
 0x13008856 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300885f); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13008862; continue;
 }
 0x13008862 => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13008876; continue; }
  pc = 0x1300886a; continue;
 }
 0x1300886a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x38u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x13008873); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13008876; continue;
 }
 0x13008876 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13008891; continue; }
  pc = 0x13008885; continue;
 }
 0x13008885 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300888e); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13008891; continue;
 }
 0x13008891 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130088a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130088a0); }
 let mut pc: u32 = 0x130088a0;
 loop { match pc {
 0x130088a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130088aa); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130088fe; continue; }
  pc = 0x130088ae; continue;
 }
 0x130088ae => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x130088b2; continue;
 }
 0x130088b2 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x130088b9); f_13009e20(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x130088f7; continue; }
  pc = 0x130088be; continue;
 }
 0x130088be => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x130088dc; continue; }
  pc = 0x130088cf; continue;
 }
 0x130088cf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x130088d5); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130088b2; continue; }
  pc = 0x130088da; continue;
 }
 0x130088da => {
  pc = 0x130088ed; continue;
 }
 0x130088dc => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130088b2; continue; }
  pc = 0x130088ed; continue;
 }
 0x130088ed => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x2u32); cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x130088f7; continue;
 }
 0x130088f7 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130088fe); f_13007f20(cpu);
  pc = 0x130088fe; continue;
 }
 0x130088fe => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008910); }
 let mut pc: u32 = 0x13008910;
 loop { match pc {
 0x13008910 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0x1300c25cu32; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x1300c258u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1300c258u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x13008955); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130089e1; continue; }
  pc = 0x1300895d; continue;
 }
 0x1300895d => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x130089b5; continue; }
  pc = 0x13008965; continue;
 }
 0x13008965 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x60u8); }
  if cpu.zf { pc = 0x130089a7; continue; }
  pc = 0x13008975; continue;
 }
 0x13008975 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x13008992; continue; }
  pc = 0x13008979; continue;
 }
 0x13008979 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = 0x58u8; cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x13008988; continue; }
  pc = 0x13008983; continue;
 }
 0x13008983 => {
  { let v: u8 = 0x78u8; cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008988; continue;
 }
 0x13008988 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.esp), v); }
  pc = 0x13008997; continue;
 }
 0x13008992 => {
  { let v: u8 = 0x6fu8; cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008997; continue;
 }
 0x13008997 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))), 0x80u8); }
  if cpu.zf { pc = 0x130089b5; continue; }
  pc = 0x130089a0; continue;
 }
 0x130089a0 => {
  { let v: u8 = 0x30u8; cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x130089b5; continue;
 }
 0x130089a7 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x130089b5; continue; }
  pc = 0x130089ab; continue;
 }
 0x130089ab => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x130089b5; continue; }
  pc = 0x130089b0; continue;
 }
 0x130089b0 => {
  { let v: u8 = 0x2bu8; cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x130089b5; continue;
 }
 0x130089b5 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x130089c6); cpu.call(t);
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130089da); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130089e1); f_13007f20(cpu);
  pc = 0x130089e1; continue;
 }
 0x130089e1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130089f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130089f0); }
 let mut pc: u32 = 0x130089f0;
 loop { match pc {
 0x130089f0 => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  cpu.push(0x130089fa); f_1300af50(cpu);
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xfu32); }
  if cpu.cf { pc = 0x13008a19; continue; }
  pc = 0x13008a14; continue;
 }
 0x13008a14 => {
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  pc = 0x13008a19; continue;
 }
 0x13008a19 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008a20); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008b10; continue; }
  pc = 0x13008a28; continue;
 }
 0x13008a28 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x4u8); }
  if cpu.zf { pc = 0x13008a41; continue; }
  pc = 0x13008a37; continue;
 }
 0x13008a37 => {
  { let v: u8 = 0x2bu8; cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x13008a41; continue;
 }
 0x13008a41 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x13008a50; continue; }
  pc = 0x13008a4a; continue;
 }
 0x13008a4a => {
  { let v: u8 = 0x23u8; cpu.w8(0x10u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x13008a50; continue;
 }
 0x13008a50 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v = 0x1300c260u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x13013154u32); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x13008a6d); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp & cpu.r32(0x1300c0d8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1000u32); }
  if !cpu.zf { pc = 0x13008a92; continue; }
  pc = 0x13008a8b; continue;
 }
 0x13008a8b => {
  { let v: u8 = 0x66u8; cpu.w8(0x14u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x13008abd; continue;
 }
 0x13008a92 => {
  { cpu.sub32(cpu.ebp, 0x800u32); }
  if !cpu.zf { pc = 0x13008a9f; continue; }
  pc = 0x13008a9a; continue;
 }
 0x13008a9a => {
  { let v: u8 = 0x65u8; cpu.w8(0x14u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x13008a9f; continue;
 }
 0x13008a9f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x2u8); }
  if cpu.zf { pc = 0x13008abd; continue; }
  pc = 0x13008aa8; continue;
 }
 0x13008aa8 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013150u32); cpu.push(0x13008ab7); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x13008abd; continue;
 }
 0x13008abd => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x13008ad3); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.esp)), 0x2bu8); }
  if cpu.zf { pc = 0x13008ae6; continue; }
  pc = 0x13008adf; continue;
 }
 0x13008adf => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.esp)), 0x2du8); }
  if !cpu.zf { pc = 0x13008af7; continue; }
  pc = 0x13008ae6; continue;
 }
 0x13008ae6 => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x21u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x13008af7; continue;
 }
 0x13008af7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13008b09); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008b10); f_13007f20(cpu);
  pc = 0x13008b10; continue;
 }
 0x13008b10 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008b20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008b20); }
 let mut pc: u32 = 0x13008b20;
 loop { match pc {
 0x13008b20 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1300c26cu32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1300c258u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1300c258u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1300c25au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x13008b55); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008be1; continue; }
  pc = 0x13008b5d; continue;
 }
 0x13008b5d => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x13008bb5; continue; }
  pc = 0x13008b65; continue;
 }
 0x13008b65 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x60u8); }
  if cpu.zf { pc = 0x13008ba7; continue; }
  pc = 0x13008b75; continue;
 }
 0x13008b75 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x13008b92; continue; }
  pc = 0x13008b79; continue;
 }
 0x13008b79 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = 0x58u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x13008b88; continue; }
  pc = 0x13008b83; continue;
 }
 0x13008b83 => {
  { let v: u8 = 0x78u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x13008b88; continue;
 }
 0x13008b88 => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008b97; continue;
 }
 0x13008b92 => {
  { let v: u8 = 0x6fu8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x13008b97; continue;
 }
 0x13008b97 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))), 0x80u8); }
  if cpu.zf { pc = 0x13008bb5; continue; }
  pc = 0x13008ba0; continue;
 }
 0x13008ba0 => {
  { let v: u8 = 0x30u8; cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008bb5; continue;
 }
 0x13008ba7 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13008bb5; continue; }
  pc = 0x13008bab; continue;
 }
 0x13008bab => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x13008bb5; continue; }
  pc = 0x13008bb0; continue;
 }
 0x13008bb0 => {
  { let v: u8 = 0x2bu8; cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008bb5; continue;
 }
 0x13008bb5 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x13008bc6); cpu.call(t);
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008bda); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008be1); f_13007f20(cpu);
  pc = 0x13008be1; continue;
 }
 0x13008be1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008bf0); }
 let mut pc: u32 = 0x13008bf0;
 loop { match pc {
 0x13008bf0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0x1300c274u32; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x1300c270u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1300c270u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x13008c35); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008c7e; continue; }
  pc = 0x13008c39; continue;
 }
 0x13008c39 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13008c52; continue; }
  pc = 0x13008c41; continue;
 }
 0x13008c41 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x25u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x2u8); }
  if cpu.zf { pc = 0x13008c52; continue; }
  pc = 0x13008c4d; continue;
 }
 0x13008c4d => {
  { let v: u8 = 0x58u8; cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008c52; continue;
 }
 0x13008c52 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x13008c63); cpu.call(t);
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008c77); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008c7e); f_13007f20(cpu);
  pc = 0x13008c7e; continue;
 }
 0x13008c7e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008c90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008c90); }
 let mut pc: u32 = 0x13008c90;
 loop { match pc {
 0x13008c90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13008c98); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008ce6; continue; }
  pc = 0x13008c9c; continue;
 }
 0x13008c9c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x13008cc0; continue; }
  pc = 0x13008cad; continue;
 }
 0x13008cad => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x13008cb9); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13008cdf; continue; }
  pc = 0x13008cbe; continue;
 }
 0x13008cbe => {
  pc = 0x13008cd5; continue;
 }
 0x13008cc0 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13008cdf; continue; }
  pc = 0x13008cd5; continue;
 }
 0x13008cd5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x6u32); cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13008cdf; continue;
 }
 0x13008cdf => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008ce6); f_13007f20(cpu);
  pc = 0x13008ce6; continue;
 }
 0x13008ce6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008cf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008cf0); }
 let mut pc: u32 = 0x13008cf0;
 loop { match pc {
 0x13008cf0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13008cf9); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008d2a; continue; }
  pc = 0x13008cfd; continue;
 }
 0x13008cfd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x13008d15); cpu.call(t);
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x13008d23; continue; }
  pc = 0x13008d19; continue;
 }
 0x13008d19 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x6u32); cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13008d23; continue;
 }
 0x13008d23 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008d2a); f_13007f20(cpu);
  pc = 0x13008d2a; continue;
 }
 0x13008d2a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008d40); }
 let mut pc: u32 = 0x13008d40;
 loop { match pc {
 0x13008d40 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1300c278u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1300c258u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1300c258u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1300c25au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x13008d75); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008e07; continue; }
  pc = 0x13008d7d; continue;
 }
 0x13008d7d => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x13008dd8; continue; }
  pc = 0x13008d87; continue;
 }
 0x13008d87 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x60u8); }
  if cpu.zf { pc = 0x13008dc9; continue; }
  pc = 0x13008d97; continue;
 }
 0x13008d97 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x13008db4; continue; }
  pc = 0x13008d9b; continue;
 }
 0x13008d9b => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = 0x58u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x13008daa; continue; }
  pc = 0x13008da5; continue;
 }
 0x13008da5 => {
  { let v: u8 = 0x78u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x13008daa; continue;
 }
 0x13008daa => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008db9; continue;
 }
 0x13008db4 => {
  { let v: u8 = 0x6fu8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x13008db9; continue;
 }
 0x13008db9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))), 0x80u8); }
  if cpu.zf { pc = 0x13008dd8; continue; }
  pc = 0x13008dc2; continue;
 }
 0x13008dc2 => {
  { let v: u8 = 0x30u8; cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008dd8; continue;
 }
 0x13008dc9 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13008dd8; continue; }
  pc = 0x13008dce; continue;
 }
 0x13008dce => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x13008dd8; continue; }
  pc = 0x13008dd3; continue;
 }
 0x13008dd3 => {
  { let v: u8 = 0x2bu8; cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008dd8; continue;
 }
 0x13008dd8 => {
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x13008dec); cpu.call(t);
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008e00); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008e07); f_13007f20(cpu);
  pc = 0x13008e07; continue;
 }
 0x13008e07 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008e20); }
 let mut pc: u32 = 0x13008e20;
 loop { match pc {
 0x13008e20 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0x1300c27cu32; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x1300c258u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1300c258u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x13008e65); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008eed; continue; }
  pc = 0x13008e6d; continue;
 }
 0x13008e6d => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x13008ec1; continue; }
  pc = 0x13008e75; continue;
 }
 0x13008e75 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x60u8); }
  if cpu.zf { pc = 0x13008eb7; continue; }
  pc = 0x13008e85; continue;
 }
 0x13008e85 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x13008ea2; continue; }
  pc = 0x13008e89; continue;
 }
 0x13008e89 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = 0x58u8; cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x13008e98; continue; }
  pc = 0x13008e93; continue;
 }
 0x13008e93 => {
  { let v: u8 = 0x78u8; cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008e98; continue;
 }
 0x13008e98 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.esp), v); }
  pc = 0x13008ea7; continue;
 }
 0x13008ea2 => {
  { let v: u8 = 0x6fu8; cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008ea7; continue;
 }
 0x13008ea7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))), 0x80u8); }
  if cpu.zf { pc = 0x13008ec1; continue; }
  pc = 0x13008eb0; continue;
 }
 0x13008eb0 => {
  { let v: u8 = 0x30u8; cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x13008ec1; continue;
 }
 0x13008eb7 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x13008ec1; continue; }
  pc = 0x13008ebc; continue;
 }
 0x13008ebc => {
  { let v: u8 = 0x2bu8; cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x13008ec1; continue;
 }
 0x13008ec1 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x13008ed2); cpu.call(t);
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008ee6); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008eed); f_13007f20(cpu);
  pc = 0x13008eed; continue;
 }
 0x13008eed => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008f00); }
 let mut pc: u32 = 0x13008f00;
 loop { match pc {
 0x13008f00 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1300c280u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1300c258u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1300c258u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1300c25au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x13008f35); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13008fbd; continue; }
  pc = 0x13008f3d; continue;
 }
 0x13008f3d => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x13008f91; continue; }
  pc = 0x13008f45; continue;
 }
 0x13008f45 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x60u8); }
  if cpu.zf { pc = 0x13008f87; continue; }
  pc = 0x13008f55; continue;
 }
 0x13008f55 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x13008f72; continue; }
  pc = 0x13008f59; continue;
 }
 0x13008f59 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = 0x58u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x13008f68; continue; }
  pc = 0x13008f63; continue;
 }
 0x13008f63 => {
  { let v: u8 = 0x78u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x13008f68; continue;
 }
 0x13008f68 => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008f77; continue;
 }
 0x13008f72 => {
  { let v: u8 = 0x6fu8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x13008f77; continue;
 }
 0x13008f77 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))), 0x80u8); }
  if cpu.zf { pc = 0x13008f91; continue; }
  pc = 0x13008f80; continue;
 }
 0x13008f80 => {
  { let v: u8 = 0x30u8; cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008f91; continue;
 }
 0x13008f87 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x13008f91; continue; }
  pc = 0x13008f8c; continue;
 }
 0x13008f8c => {
  { let v: u8 = 0x2bu8; cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x13008f91; continue;
 }
 0x13008f91 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x13008fa2); cpu.call(t);
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008fb6); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13008fbd); f_13007f20(cpu);
  pc = 0x13008fbd; continue;
 }
 0x13008fbd => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13008fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13008fd0); }
 let mut pc: u32 = 0x13008fd0;
 loop { match pc {
 0x13008fd0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1300c284u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1300c258u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1300c258u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1300c25au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x13009005); f_13007ea0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009092; continue; }
  pc = 0x1300900d; continue;
 }
 0x1300900d => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x13009063; continue; }
  pc = 0x13009017; continue;
 }
 0x13009017 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x60u8); }
  if cpu.zf { pc = 0x13009059; continue; }
  pc = 0x13009027; continue;
 }
 0x13009027 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x13009044; continue; }
  pc = 0x1300902b; continue;
 }
 0x1300902b => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = 0x58u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x1300903a; continue; }
  pc = 0x13009035; continue;
 }
 0x13009035 => {
  { let v: u8 = 0x78u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x1300903a; continue;
 }
 0x1300903a => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esp), v); }
  pc = 0x13009049; continue;
 }
 0x13009044 => {
  { let v: u8 = 0x6fu8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x13009049; continue;
 }
 0x13009049 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))), 0x80u8); }
  if cpu.zf { pc = 0x13009063; continue; }
  pc = 0x13009052; continue;
 }
 0x13009052 => {
  { let v: u8 = 0x30u8; cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x13009063; continue;
 }
 0x13009059 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x13009063; continue; }
  pc = 0x1300905e; continue;
 }
 0x1300905e => {
  { let v: u8 = 0x2bu8; cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x13009063; continue;
 }
 0x13009063 => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x13009077); cpu.call(t);
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300908b); f_13008510(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13009092); f_13007f20(cpu);
  pc = 0x13009092; continue;
 }
 0x13009092 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130090a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130090a0); }
 let mut pc: u32 = 0x130090a0;
 loop { match pc {
 0x130090a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130090fau32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130090c6); f_130099f0(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1300c154u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130090fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130090fa); }
 let mut pc: u32 = 0x130090fa;
 loop { match pc {
 0x130090fa => {
  { let v: u32 = 0x1300d470u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009104(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009104); }
 let mut pc: u32 = 0x13009104;
 loop { match pc {
 0x13009104 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009110); }
 let mut pc: u32 = 0x13009110;
 loop { match pc {
 0x13009110 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13009159u32; cpu.push(v); }
  { let v: u32 = 0x1300c154u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13009140); f_130094d0(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1300914c); f_13009163(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009159(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009159); }
 let mut pc: u32 = 0x13009159;
 loop { match pc {
 0x13009159 => {
  { let v: u32 = 0x1300d498u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009163(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009163); }
 let mut pc: u32 = 0x13009163;
 loop { match pc {
 0x13009163 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009170); }
 let mut pc: u32 = 0x13009170;
 loop { match pc {
 0x13009170 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009196; continue; }
  pc = 0x1300918e; continue;
 }
 0x1300918e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x13009196 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1300919c); f_1300ae80(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x130091ae; continue; }
  pc = 0x130091a3; continue;
 }
 0x130091a3 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x130091ae => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130091be); f_13009ce0(cpu);
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x130091cc; continue; }
  pc = 0x130091c2; continue;
 }
 0x130091c2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x130091d1; continue;
 }
 0x130091cc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x130091d1; continue;
 }
 0x130091d1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x130091fe; continue; }
  pc = 0x130091e2; continue;
 }
 0x130091e2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x130091fe => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009220); }
 let mut pc: u32 = 0x13009220;
 loop { match pc {
 0x13009220 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13009229); f_13009c80(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009238; continue; }
  pc = 0x1300922e; continue;
 }
 0x1300922e => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13009238 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1300928b; continue; }
  pc = 0x1300923e; continue;
 }
 0x1300923e => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x1300928b; continue; }
  pc = 0x13009244; continue;
 }
 0x13009244 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1300928b; continue; }
  pc = 0x13009250; continue;
 }
 0x13009250 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x130131c0u32); cpu.push(0x1300925e); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x1300926a; continue; }
  pc = 0x13009267; continue;
 }
 0x13009267 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x1300926a; continue;
 }
 0x1300926a => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x1300928b; continue; }
  pc = 0x1300926e; continue;
 }
 0x1300926e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013164u32); cpu.push(0x1300927e); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x1300928b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x130092b9; continue; }
  pc = 0x13009292; continue;
 }
 0x13009292 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x130092b9; continue; }
  pc = 0x13009298; continue;
 }
 0x13009298 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x130092ab; continue; }
  pc = 0x130092a7; continue;
 }
 0x130092a7 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  pc = 0x130092ab; continue;
 }
 0x130092ab => {
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x130092b9; continue;
 }
 0x130092b9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x130092d8; continue; }
  pc = 0x130092c2; continue;
 }
 0x130092c2 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x130092e2; continue; }
  pc = 0x130092c6; continue;
 }
 0x130092c6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x130092e2; continue; }
  pc = 0x130092d0; continue;
 }
 0x130092d0 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x130092d8; continue;
 }
 0x130092d8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130092e2 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013160u32); cpu.push(0x130092ed); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009300); }
 let mut pc: u32 = 0x13009300;
 loop { match pc {
 0x13009300 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13009309); f_13009c80(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009316; continue; }
  pc = 0x1300930e; continue;
 }
 0x1300930e => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009316 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x130093bc; continue; }
  pc = 0x13009321; continue;
 }
 0x13009321 => {
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1300935e; continue; }
  pc = 0x13009327; continue;
 }
 0x13009327 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x1300933a; continue; }
  pc = 0x13009336; continue;
 }
 0x13009336 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  pc = 0x1300933a; continue;
 }
 0x1300933a => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x13009342; continue; }
  pc = 0x1300933e; continue;
 }
 0x1300933e => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  pc = 0x13009344; continue;
 }
 0x13009342 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x13009344; continue;
 }
 0x13009344 => {
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  pc = 0x1300935e; continue;
 }
 0x1300935e => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x130093bc; continue; }
  pc = 0x13009362; continue;
 }
 0x13009362 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130093bc; continue; }
  pc = 0x13009369; continue;
 }
 0x13009369 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.cf { pc = 0x130093b2; continue; }
  pc = 0x1300936e; continue;
 }
 0x1300936e => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1301316cu32); cpu.push(0x13009381); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13009390; continue; }
  pc = 0x13009388; continue;
 }
 0x13009388 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009390 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.zf { pc = 0x130093b2; continue; }
  pc = 0x130093a6; continue;
 }
 0x130093a6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013164u32); cpu.push(0x130093af); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x130093b2; continue;
 }
 0x130093b2 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130093b9); f_13009e20(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130093bc => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x130131c4u32); cpu.push(0x130093c6); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130093d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130093d0); }
 let mut pc: u32 = 0x130093d0;
 loop { match pc {
 0x130093d0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130093f0; continue; }
  pc = 0x130093dc; continue;
 }
 0x130093dc => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x130093f4; continue; }
  pc = 0x130093e1; continue;
 }
 0x130093e1 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x130093fb; continue; }
  pc = 0x130093e6; continue;
 }
 0x130093e6 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x130093f0 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x13009400; continue;
 }
 0x130093f4 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x13009400; continue;
 }
 0x130093fb => {
  { let v: u32 = 0x2u32; cpu.edi = v; }
  pc = 0x13009400; continue;
 }
 0x13009400 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13009409); f_13009220(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013174u32); cpu.push(0x13009419); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300942a; continue; }
  pc = 0x13009420; continue;
 }
 0x13009420 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300942a => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013170u32); cpu.push(0x13009434); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x13009443; continue; }
  pc = 0x13009441; continue;
 }
 0x13009441 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x13009443; continue;
 }
 0x13009443 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009450); }
 let mut pc: u32 = 0x13009450;
 loop { match pc {
 0x13009450 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x130094b5; continue; }
  pc = 0x1300945d; continue;
 }
 0x1300945d => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  cpu.push(0x1300946a); f_130093d0(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009478; continue; }
  pc = 0x1300946f; continue;
 }
 0x1300946f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13009478 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x130094a5; continue; }
  pc = 0x1300947e; continue;
 }
 0x1300947e => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x130094ad; continue; }
  pc = 0x13009485; continue;
 }
 0x13009485 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013164u32); cpu.push(0x13009495); cpu.call(t);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130094a5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130094ad => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130094b5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130094d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130094d0); }
 let mut pc: u32 = 0x130094d0;
 loop { match pc {
 0x130094d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x13009584; continue; }
  pc = 0x130094e0; continue;
 }
 0x130094e0 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  cpu.push(0x130094e7); f_13009220(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130094f6; continue; }
  pc = 0x130094ec; continue;
 }
 0x130094ec => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130094f6 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if !cpu.cf { pc = 0x13009509; continue; }
  pc = 0x13009505; continue;
 }
 0x13009505 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  pc = 0x13009509; continue;
 }
 0x13009509 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13009584; continue; }
  pc = 0x1300950d; continue;
 }
 0x1300950d => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x13009536; continue; }
  pc = 0x13009518; continue;
 }
 0x13009518 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1301312cu32); cpu.ebp = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1fu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  pc = 0x1300953b; continue;
 }
 0x13009536 => {
  { let v: u32 = cpu.r32(0x13013178u32); cpu.eax = v; }
  pc = 0x1300953b; continue;
 }
 0x1300953b => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax)), 0x80u8); }
  if cpu.zf { pc = 0x1300955a; continue; }
  pc = 0x13009541; continue;
 }
 0x13009541 => {
  { cpu.sub32(cpu.edi, cpu.edx); }
  if !cpu.cf { pc = 0x13009550; continue; }
  pc = 0x13009545; continue;
 }
 0x13009545 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0xau8); }
  if !cpu.zf { pc = 0x1300954b; continue; }
  pc = 0x1300954a; continue;
 }
 0x1300954a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1300954b; continue;
 }
 0x1300954b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if cpu.cf { pc = 0x13009545; continue; }
  pc = 0x13009550; continue;
 }
 0x13009550 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x1300955a; continue; }
  pc = 0x13009559; continue;
 }
 0x13009559 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1300955a; continue;
 }
 0x1300955a => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13009568); f_130093d0(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009577; continue; }
  pc = 0x1300956d; continue;
 }
 0x1300956d => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009577 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x13009584; continue;
 }
 0x13009584 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009590); }
 let mut pc: u32 = 0x13009590;
 loop { match pc {
 0x13009590 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13009674u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.edi); }
  if cpu.zf { pc = 0x130095d8; continue; }
  pc = 0x130095bc; continue;
 }
 0x130095bc => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c188u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c180u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130095d1); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130095d8; continue;
 }
 0x130095d8 => {
  { let v = 0x50u32; cpu.push(v); }
  cpu.push(0x130095df); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13009602; continue; }
  pc = 0x130095f7; continue;
 }
 0x130095f7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009602); f_130090a0(cpu);
  pc = 0x13009602; continue;
 }
 0x13009602 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009610); f_13005590(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c194u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009655(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009655); }
 let mut pc: u32 = 0x13009655;
 loop { match pc {
 0x13009655 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13009666; continue; }
  pc = 0x1300965b; continue;
 }
 0x1300965b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13009666 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009667(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009667); }
 let mut pc: u32 = 0x13009667;
 loop { match pc {
 0x13009667 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009670); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009674); }
 let mut pc: u32 = 0x13009674;
 loop { match pc {
 0x13009674 => {
  { let v: u32 = 0x1300d4c0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300967e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300967e); }
 let mut pc: u32 = 0x1300967e;
 loop { match pc {
 0x1300967e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009690); }
 let mut pc: u32 = 0x13009690;
 loop { match pc {
 0x13009690 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130096d5u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c194u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x130096c8); f_130096df(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130096d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130096d5); }
 let mut pc: u32 = 0x130096d5;
 loop { match pc {
 0x130096d5 => {
  { let v: u32 = 0x1300d4f8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130096df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130096df); }
 let mut pc: u32 = 0x130096df;
 loop { match pc {
 0x130096df => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13005730(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130096f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130096f0); }
 let mut pc: u32 = 0x130096f0;
 loop { match pc {
 0x130096f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130099cau32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x13012278u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x13009994; continue; }
  pc = 0x1300971a; continue;
 }
 0x1300971a => {
  { let v = 0x50u32; cpu.push(v); }
  cpu.push(0x13009721); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13009746; continue; }
  pc = 0x13009734; continue;
 }
 0x13009734 => {
  { let v: u32 = cpu.r32(0x1301315cu32); cpu.esi = v; }
  let t = cpu.esi; cpu.push(0x1300973c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x13009744); f_130090a0(cpu);
  pc = 0x1300974e; continue;
 }
 0x13009746 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1301315cu32); cpu.esi = v; }
  pc = 0x1300974e; continue;
 }
 0x1300974e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x13012318u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13009760); f_13005c70(cpu);
  { let v: u32 = cpu.r32(0x13012318u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x13012334u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x13012318u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x13012318u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13009792; continue; }
  pc = 0x13009786; continue;
 }
 0x13009786 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300978f); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13009792; continue;
 }
 0x13009792 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edi)) | 0x4000u32; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130097ab; continue; }
  pc = 0x1300979f; continue;
 }
 0x1300979f => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130097a8); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130097ab; continue;
 }
 0x130097ab => {
  { let v = 0x50u32; cpu.push(v); }
  cpu.push(0x130097b2); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x130097d7; continue; }
  pc = 0x130097ca; continue;
 }
 0x130097ca => {
  let t = cpu.esi; cpu.push(0x130097cc); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130097d7); f_130090a0(cpu);
  pc = 0x130097d7; continue;
 }
 0x130097d7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x130123f8u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x130097e9); f_13008290(cpu);
  { let v: u32 = cpu.r32(0x130123f8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x13012414u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x130123f8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x130123f8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300981b; continue; }
  pc = 0x1300980f; continue;
 }
 0x1300980f => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009818); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300981b; continue;
 }
 0x1300981b => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edi)) | 0x6000u32; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13009834; continue; }
  pc = 0x13009828; continue;
 }
 0x13009828 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13009831); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13009834; continue;
 }
 0x13009834 => {
  { let v = 0x50u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x130123f8u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x130123fcu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x1300984b); f_13009170(cpu);
  { let v = 0x50u32; cpu.push(v); }
  cpu.push(0x13009852); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13009877; continue; }
  pc = 0x1300986a; continue;
 }
 0x1300986a => {
  let t = cpu.esi; cpu.push(0x1300986c); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009877); f_130090a0(cpu);
  pc = 0x13009877; continue;
 }
 0x13009877 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x130122c0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13009889); f_13008290(cpu);
  { let v: u32 = cpu.r32(0x130122c0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x130122dcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x130122c0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x130122c0u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130098bb; continue; }
  pc = 0x130098af; continue;
 }
 0x130098af => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130098b8); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130098bb; continue;
 }
 0x130098bb => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edi)) | 0x6000u32; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130098d4; continue; }
  pc = 0x130098c8; continue;
 }
 0x130098c8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130098d1); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130098d4; continue;
 }
 0x130098d4 => {
  { let v = 0x50u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x130122c0u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x130122c4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x130098eb); f_13009170(cpu);
  { let v = 0x50u32; cpu.push(v); }
  cpu.push(0x130098f2); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13009917; continue; }
  pc = 0x1300990a; continue;
 }
 0x1300990a => {
  let t = cpu.esi; cpu.push(0x1300990c); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009917); f_130090a0(cpu);
  pc = 0x13009917; continue;
 }
 0x13009917 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x13012380u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13009929); f_13008290(cpu);
  { let v: u32 = cpu.r32(0x13012380u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1301239cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x13012380u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x13012380u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300995b; continue; }
  pc = 0x1300994f; continue;
 }
 0x1300994f => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009958); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300995b; continue;
 }
 0x1300995b => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)) | 0x4000u32; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13009974; continue; }
  pc = 0x13009968; continue;
 }
 0x13009968 => {
  { let v: u32 = cpu.esi.wrapping_add(0x38u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13009971); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13009974; continue;
 }
 0x13009974 => {
  { let v = 0x200u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x13012380u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x13012384u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x1300998e); f_13009170(cpu);
  { let v: u32 = cpu.inc32(cpu.r32(0x13012278u32)); cpu.w32(0x13012278u32, v); }
  pc = 0x13009994; continue;
 }
 0x13009994 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130099a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130099a3); }
 let mut pc: u32 = 0x130099a3;
 loop { match pc {
 0x130099a3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130099ac); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130099b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130099b0); }
 let mut pc: u32 = 0x130099b0;
 loop { match pc {
 0x130099b0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130099b9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130099bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130099bd); }
 let mut pc: u32 = 0x130099bd;
 loop { match pc {
 0x130099bd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130099c6); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130099ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130099ca); }
 let mut pc: u32 = 0x130099ca;
 loop { match pc {
 0x130099ca => {
  { let v: u32 = 0x1300d520u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130099d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130099d4); }
 let mut pc: u32 = 0x130099d4;
 loop { match pc {
 0x130099d4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130099dd); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130099f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130099f0); }
 let mut pc: u32 = 0x130099f0;
 loop { match pc {
 0x130099f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1300c004u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13009a2d); f_13007260(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009a40); }
 let mut pc: u32 = 0x13009a40;
 loop { match pc {
 0x13009a40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x1300c004u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009a84; continue; }
  pc = 0x13009a80; continue;
 }
 0x13009a80 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x13009a99; continue; }
  pc = 0x13009a84; continue;
 }
 0x13009a84 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x13009a99; continue;
 }
 0x13009a99 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009aa9); f_13007260(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009ac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009ac0); }
 let mut pc: u32 = 0x13009ac0;
 loop { match pc {
 0x13009ac0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x1300c004u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009ad2); f_13007270(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13009adc); f_13009c60(cpu);
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x13009af2; continue; }
  pc = 0x13009ae2; continue;
 }
 0x13009ae2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009af2; continue; }
  pc = 0x13009ae9; continue;
 }
 0x13009ae9 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009aef); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13009af2; continue;
 }
 0x13009af2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009b00); }
 let mut pc: u32 = 0x13009b00;
 loop { match pc {
 0x13009b00 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x13009b36; continue; }
  pc = 0x13009b06; continue;
 }
 0x13009b06 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009b2a; continue; }
  pc = 0x13009b0e; continue;
 }
 0x13009b0e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x13009b2a; continue; }
  pc = 0x13009b16; continue;
 }
 0x13009b16 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x13009b2a => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x13009b36 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009b40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009b40); }
 let mut pc: u32 = 0x13009b40;
 loop { match pc {
 0x13009b40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009b88; continue; }
  pc = 0x13009b53; continue;
 }
 0x13009b53 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x13009b57; continue;
 }
 0x13009b57 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x13009b6e; continue; }
  pc = 0x13009b5d; continue;
 }
 0x13009b5d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x13009b6e; continue; }
  pc = 0x13009b65; continue;
 }
 0x13009b65 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x13009b7f; continue;
 }
 0x13009b6e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x13009b7a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x13009b88; continue; }
  pc = 0x13009b7f; continue;
 }
 0x13009b7f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13009b57; continue; }
  pc = 0x13009b88; continue;
 }
 0x13009b88 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009ba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009ba0); }
 let mut pc: u32 = 0x13009ba0;
 loop { match pc {
 0x13009ba0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  if cpu.zf { pc = 0x13009bfb; continue; }
  pc = 0x13009bb7; continue;
 }
 0x13009bb7 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009bc6; continue; }
  pc = 0x13009bbd; continue;
 }
 0x13009bbd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009bc3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13009bc6; continue;
 }
 0x13009bc6 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009c4f; continue; }
  pc = 0x13009bd1; continue;
 }
 0x13009bd1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x13009c4f; continue; }
  pc = 0x13009bd9; continue;
 }
 0x13009bd9 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.esp))); cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009bef); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x13009bd1; continue; }
  pc = 0x13009bf9; continue;
 }
 0x13009bf9 => {
  pc = 0x13009c4f; continue;
 }
 0x13009bfb => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x13009c4f; continue; }
  pc = 0x13009c03; continue;
 }
 0x13009c03 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x13009c0d; continue;
 }
 0x13009c0d => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.push(0x13009c13); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x13009c4f; continue; }
  pc = 0x13009c18; continue;
 }
 0x13009c18 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13009c28; continue; }
  pc = 0x13009c26; continue;
 }
 0x13009c26 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x13009c28; continue;
 }
 0x13009c28 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13009c4b; continue; }
  pc = 0x13009c2c; continue;
 }
 0x13009c2c => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)).wrapping_add(cpu.edx); cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.edx); cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)).wrapping_add(cpu.edx); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x13009c4b; continue;
 }
 0x13009c4b => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x13009c0d; continue; }
  pc = 0x13009c4f; continue;
 }
 0x13009c4f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009c60); }
 let mut pc: u32 = 0x13009c60;
 loop { match pc {
 0x13009c60 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ecx)), cpu.eax); }
  if cpu.cf { pc = 0x13009c73; continue; }
  pc = 0x13009c68; continue;
 }
 0x13009c68 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ecx)), cpu.eax); }
  if cpu.cf { pc = 0x13009c73; continue; }
  pc = 0x13009c70; continue;
 }
 0x13009c70 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009c73 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009c80); }
 let mut pc: u32 = 0x13009c80;
 loop { match pc {
 0x13009c80 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x13009c9c; continue; }
  pc = 0x13009c86; continue;
 }
 0x13009c86 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x13009c9c; continue; }
  pc = 0x13009c8c; continue;
 }
 0x13009c8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x13009c91); cpu.call(t);
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfffffffeu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009c9c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009ca0); }
 let mut pc: u32 = 0x13009ca0;
 loop { match pc {
 0x13009ca0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x200u32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13009cad); f_1300ae80(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13009cbb; continue; }
  pc = 0x13009cb4; continue;
 }
 0x13009cb4 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009cbb => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x200u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13009ccc); f_13009ce0(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009ce0); }
 let mut pc: u32 = 0x13009ce0;
 loop { match pc {
 0x13009ce0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x13009cf9; continue; }
  pc = 0x13009ce9; continue;
 }
 0x13009ce9 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009cf9; continue; }
  pc = 0x13009cf0; continue;
 }
 0x13009cf0 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13009cf6); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13009cf9; continue;
 }
 0x13009cf9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009d20); }
 let mut pc: u32 = 0x13009d20;
 loop { match pc {
 0x13009d20 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009d30); }
 let mut pc: u32 = 0x13009d30;
 loop { match pc {
 0x13009d30 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x13009d41); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009d50); }
 let mut pc: u32 = 0x13009d50;
 loop { match pc {
 0x13009d50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x13009db5; continue; }
  pc = 0x13009d5d; continue;
 }
 0x13009d5d => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x13009d6a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009d78; continue; }
  pc = 0x13009d6f; continue;
 }
 0x13009d6f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13009d78 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x13009da5; continue; }
  pc = 0x13009d7e; continue;
 }
 0x13009d7e => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13009dad; continue; }
  pc = 0x13009d85; continue;
 }
 0x13009d85 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013164u32); cpu.push(0x13009d95); cpu.call(t);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13009da5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13009dad => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13009db5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009dd0); }
 let mut pc: u32 = 0x13009dd0;
 loop { match pc {
 0x13009dd0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x13009df0; continue; }
  pc = 0x13009dd9; continue;
 }
 0x13009dd9 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009de4; continue; }
  pc = 0x13009ddf; continue;
 }
 0x13009ddf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009de4); cpu.call(t);
  pc = 0x13009de4; continue;
 }
 0x13009de4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009deb); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009df0 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13009dfc; continue; }
  pc = 0x13009df7; continue;
 }
 0x13009df7 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.cf { pc = 0x13009e03; continue; }
  pc = 0x13009dfc; continue;
 }
 0x13009dfc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009e03); cpu.call(t);
  pc = 0x13009e03; continue;
 }
 0x13009e03 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esi)), cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x13009e18; continue; }
  pc = 0x13009e0f; continue;
 }
 0x13009e0f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009e16); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009e18 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009e20); }
 let mut pc: u32 = 0x13009e20;
 loop { match pc {
 0x13009e20 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x13009e41; continue; }
  pc = 0x13009e29; continue;
 }
 0x13009e29 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009e38; continue; }
  pc = 0x13009e31; continue;
 }
 0x13009e31 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009e36); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009e38 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009e41 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esi)), cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x13009e55; continue; }
  pc = 0x13009e49; continue;
 }
 0x13009e49 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009e50); cpu.call(t);
  { let v: u32 = cpu.inc32(cpu.r32(0x28u32.wrapping_add(cpu.esi))); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009e55 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x28u32.wrapping_add(cpu.esi))); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009e60); }
 let mut pc: u32 = 0x13009e60;
 loop { match pc {
 0x13009e60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x13009e7f; continue; }
  pc = 0x13009e69; continue;
 }
 0x13009e69 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009e76; continue; }
  pc = 0x13009e6f; continue;
 }
 0x13009e6f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009e74); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009e76 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009e7f => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.cf { pc = 0x13009e8e; continue; }
  pc = 0x13009e87; continue;
 }
 0x13009e87 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009e8e); cpu.call(t);
  pc = 0x13009e8e; continue;
 }
 0x13009e8e => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esi)), cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x13009e9a; continue; }
  pc = 0x13009e96; continue;
 }
 0x13009e96 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x13009e9a; continue;
 }
 0x13009e9a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009ea0); }
 let mut pc: u32 = 0x13009ea0;
 loop { match pc {
 0x13009ea0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x13009ebc; continue; }
  pc = 0x13009ea9; continue;
 }
 0x13009ea9 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x13009eb7; continue; }
  pc = 0x13009eaf; continue;
 }
 0x13009eaf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009eb4); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x13009eb7; continue;
 }
 0x13009eb7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13009ebc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x13009ec3); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009ed0); }
 let mut pc: u32 = 0x13009ed0;
 loop { match pc {
 0x13009ed0 => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  cpu.push(0x13009eda); f_1300af50(cpu);
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x13009ef8; continue; }
  pc = 0x13009ee2; continue;
 }
 0x13009ee2 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1300c320u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x13009ef3); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x13009f3e; continue;
 }
 0x13009ef8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x13009f0b; continue; }
  pc = 0x13009f07; continue;
 }
 0x13009f07 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  pc = 0x13009f0b; continue;
 }
 0x13009f0b => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1300c288u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013154u32); cpu.push(0x13009f3b); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x34u32); cpu.esp = v; }
  pc = 0x13009f3e; continue;
 }
 0x13009f3e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  let t = cpu.r32(0x130131a8u32); cpu.push(0x13009f4c); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009f60); }
 let mut pc: u32 = 0x13009f60;
 loop { match pc {
 0x13009f60 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13009fbdu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13009f86); f_130099f0(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c0ecu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009fbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009fbd); }
 let mut pc: u32 = 0x13009fbd;
 loop { match pc {
 0x13009fbd => {
  { let v: u32 = 0x1300d560u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009fc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009fc7); }
 let mut pc: u32 = 0x13009fc7;
 loop { match pc {
 0x13009fc7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13009fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13009fd0); }
 let mut pc: u32 = 0x13009fd0;
 loop { match pc {
 0x13009fd0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300a037u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13009ff6); f_130099f0(cpu);
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c0ecu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300a01d); f_1300a3a0(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
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
pub fn f_1300a037(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a037); }
 let mut pc: u32 = 0x1300a037;
 loop { match pc {
 0x1300a037 => {
  { let v: u32 = 0x1300d588u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a041(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a041); }
 let mut pc: u32 = 0x1300a041;
 loop { match pc {
 0x1300a041 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a050); }
 let mut pc: u32 = 0x1300a050;
 loop { match pc {
 0x1300a050 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300a0b5u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1300a076); f_130099f0(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c0ecu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a0b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a0b5); }
 let mut pc: u32 = 0x1300a0b5;
 loop { match pc {
 0x1300a0b5 => {
  { let v: u32 = 0x1300d5b0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a0bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a0bf); }
 let mut pc: u32 = 0x1300a0bf;
 loop { match pc {
 0x1300a0bf => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a0d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a0d0); }
 let mut pc: u32 = 0x1300a0d0;
 loop { match pc {
 0x1300a0d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300a19cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1300a0f8); f_130099f0(cpu);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  { let v: u32 = 0x1300c0ecu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x1300a12a; continue; }
  pc = 0x1300a114; continue;
 }
 0x1300a114 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  pc = 0x1300a13e; continue;
 }
 0x1300a12a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x1300a138; continue; }
  pc = 0x1300a12e; continue;
 }
 0x1300a12e => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  pc = 0x1300a13e; continue;
 }
 0x1300a138 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  pc = 0x1300a13e; continue;
 }
 0x1300a13e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1300a149); f_13009ce0(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.ebx; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x1300a16b; continue; }
  pc = 0x1300a156; continue;
 }
 0x1300a156 => {
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x1300a180; continue;
 }
 0x1300a16b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x1300a180; continue;
 }
 0x1300a180 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a19c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a19c); }
 let mut pc: u32 = 0x1300a19c;
 loop { match pc {
 0x1300a19c => {
  { let v: u32 = 0x1300d5d8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a1a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a1a6); }
 let mut pc: u32 = 0x1300a1a6;
 loop { match pc {
 0x1300a1a6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a1b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a1b0); }
 let mut pc: u32 = 0x1300a1b0;
 loop { match pc {
 0x1300a1b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300a218u32; cpu.push(v); }
  { let v: u32 = 0x1300c0ecu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x1300a1ff; continue; }
  pc = 0x1300a1e1; continue;
 }
 0x1300a1e1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1300a1ff; continue; }
  pc = 0x1300a1e8; continue;
 }
 0x1300a1e8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1300a1fa; continue; }
  pc = 0x1300a1f3; continue;
 }
 0x1300a1f3 => {
  cpu.push(0x1300a1f8); f_1300ad18(cpu);
  pc = 0x1300a1fc; continue;
 }
 0x1300a1fa => {
  let t = cpu.eax; cpu.push(0x1300a1fc); cpu.call(t);
  pc = 0x1300a1fc; continue;
 }
 0x1300a1fc => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300a1ff; continue;
 }
 0x1300a1ff => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1300a20b); f_1300a222(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a218(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a218); }
 let mut pc: u32 = 0x1300a218;
 loop { match pc {
 0x1300a218 => {
  { let v: u32 = 0x1300d600u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a222(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a222); }
 let mut pc: u32 = 0x1300a222;
 loop { match pc {
 0x1300a222 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a230); }
 let mut pc: u32 = 0x1300a230;
 loop { match pc {
 0x1300a230 => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1300a242; continue; }
  pc = 0x1300a236; continue;
 }
 0x1300a236 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x1300a242; continue;
 }
 0x1300a242 => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a250); }
 let mut pc: u32 = 0x1300a250;
 loop { match pc {
 0x1300a250 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a260); }
 let mut pc: u32 = 0x1300a260;
 loop { match pc {
 0x1300a260 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.esi); }
  { let v: u32 = 0x50u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  if (cpu.cf || cpu.zf) { pc = 0x1300a290; continue; }
  pc = 0x1300a27a; continue;
 }
 0x1300a27a => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300a284; continue; }
  pc = 0x1300a27f; continue;
 }
 0x1300a27f => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x1300a284; continue;
 }
 0x1300a284 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1300a2a2; continue; }
  pc = 0x1300a28c; continue;
 }
 0x1300a28c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  pc = 0x1300a2c5; continue;
 }
 0x1300a290 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300a29a; continue; }
  pc = 0x1300a295; continue;
 }
 0x1300a295 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x1300a29a; continue;
 }
 0x1300a29a => {
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1300a2a2; continue; }
  pc = 0x1300a29e; continue;
 }
 0x1300a29e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  pc = 0x1300a2c5; continue;
 }
 0x1300a2a2 => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1300a2b9; continue; }
  pc = 0x1300a2a6; continue;
 }
 0x1300a2a6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300a2b2; continue; }
  pc = 0x1300a2ad; continue;
 }
 0x1300a2ad => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1300a2b2; continue;
 }
 0x1300a2b2 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebp = v; }
  pc = 0x1300a2c5; continue;
 }
 0x1300a2b9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300a2c5; continue; }
  pc = 0x1300a2c0; continue;
 }
 0x1300a2c0 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x1300a2c5; continue;
 }
 0x1300a2c5 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300a2d3; continue; }
  pc = 0x1300a2cf; continue;
 }
 0x1300a2cf => {
  let t = cpu.eax; cpu.push(0x1300a2d1); cpu.call(t);
  pc = 0x1300a2d8; continue;
 }
 0x1300a2d3 => {
  cpu.push(0x1300a2d8); f_1300ae80(cpu);
  pc = 0x1300a2d8; continue;
 }
 0x1300a2d8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1300a2f3; continue; }
  pc = 0x1300a2e6; continue;
 }
 0x1300a2e6 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1300a2f3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1300a318; continue; }
  pc = 0x1300a2fd; continue;
 }
 0x1300a2fd => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  pc = 0x1300a318; continue;
 }
 0x1300a318 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1300a32a; continue; }
  pc = 0x1300a31f; continue;
 }
 0x1300a31f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a328); f_1300ad18(cpu);
  pc = 0x1300a330; continue;
 }
 0x1300a32a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.eax; cpu.push(0x1300a330); cpu.call(t);
  pc = 0x1300a330; continue;
 }
 0x1300a330 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a348); f_13009ce0(cpu);
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1300a365; continue; }
  pc = 0x1300a34c; continue;
 }
 0x1300a34c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300a365; continue; }
  pc = 0x1300a353; continue;
 }
 0x1300a353 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edi); cpu.w32(0x24u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edi); cpu.w32(0x28u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x1300a365; continue;
 }
 0x1300a365 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300a388; continue; }
  pc = 0x1300a36c; continue;
 }
 0x1300a36c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebx), v); }
  if cpu.zf { pc = 0x1300a388; continue; }
  pc = 0x1300a383; continue;
 }
 0x1300a383 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x1300a388; continue;
 }
 0x1300a388 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a3a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a3a0); }
 let mut pc: u32 = 0x1300a3a0;
 loop { match pc {
 0x1300a3a0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300a3ab; continue; }
  pc = 0x1300a3a8; continue;
 }
 0x1300a3a8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1300a3ab; continue;
 }
 0x1300a3ab => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a3b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a3b0); }
 let mut pc: u32 = 0x1300a3b0;
 loop { match pc {
 0x1300a3b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ecx)), cpu.eax); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.cf { pc = 0x1300a416; continue; }
  pc = 0x1300a3bb; continue;
 }
 0x1300a3bb => {
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1300a3ca; continue; }
  pc = 0x1300a3c1; continue;
 }
 0x1300a3c1 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x1300a3ca => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300a3d1); f_1300a260(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300a3df; continue; }
  pc = 0x1300a3d6; continue;
 }
 0x1300a3d6 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x1300a3df => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1300a3fc; continue; }
  pc = 0x1300a3e5; continue;
 }
 0x1300a3e5 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x24u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x1300a416; continue;
 }
 0x1300a3fc => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300a416; continue; }
  pc = 0x1300a411; continue;
 }
 0x1300a411 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x1300a416; continue;
 }
 0x1300a416 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1300a42d; continue; }
  pc = 0x1300a41f; continue;
 }
 0x1300a41f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x1300a42d; continue; }
  pc = 0x1300a42a; continue;
 }
 0x1300a42a => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.esi))); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x1300a42d; continue;
 }
 0x1300a42d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a440); }
 let mut pc: u32 = 0x1300a440;
 loop { match pc {
 0x1300a440 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1300a479; continue; }
  pc = 0x1300a44b; continue;
 }
 0x1300a44b => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.cf { pc = 0x1300a46a; continue; }
  pc = 0x1300a452; continue;
 }
 0x1300a452 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x24u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x24u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x2cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x1300a46a; continue;
 }
 0x1300a46a => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.ecx)), cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1300a479; continue; }
  pc = 0x1300a472; continue;
 }
 0x1300a472 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1300a479 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a490); }
 let mut pc: u32 = 0x1300a490;
 loop { match pc {
 0x1300a490 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a4a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a4a0); }
 let mut pc: u32 = 0x1300a4a0;
 loop { match pc {
 0x1300a4a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  if cpu.zf { pc = 0x1300a515; continue; }
  pc = 0x1300a4b4; continue;
 }
 0x1300a4b4 => {
  cpu.push(0x1300a4b9); f_1300a440(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300a4d4; continue; }
  pc = 0x1300a4c1; continue;
 }
 0x1300a4c1 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1300a4d9; continue; }
  pc = 0x1300a4c6; continue;
 }
 0x1300a4c6 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x1300a4de; continue; }
  pc = 0x1300a4cb; continue;
 }
 0x1300a4cb => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300a4d4 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  pc = 0x1300a4e1; continue;
 }
 0x1300a4d9 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  pc = 0x1300a4e1; continue;
 }
 0x1300a4de => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  pc = 0x1300a4e1; continue;
 }
 0x1300a4e1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1300a509; continue; }
  pc = 0x1300a4f2; continue;
 }
 0x1300a4f2 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { pc = 0x1300a509; continue; }
  pc = 0x1300a4f9; continue;
 }
 0x1300a4f9 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1300a519; continue; }
  pc = 0x1300a502; continue;
 }
 0x1300a502 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.edi), v); }
  pc = 0x1300a519; continue;
 }
 0x1300a509 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300a515 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x1300a519; continue;
 }
 0x1300a519 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x1300a5cd; continue; }
  pc = 0x1300a522; continue;
 }
 0x1300a522 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x1300a542; continue; }
  pc = 0x1300a528; continue;
 }
 0x1300a528 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x1300a531); f_1300a3b0(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300a542; continue; }
  pc = 0x1300a536; continue;
 }
 0x1300a536 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300a542 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1300a560; continue; }
  pc = 0x1300a54a; continue;
 }
 0x1300a54a => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1300a565; continue; }
  pc = 0x1300a54f; continue;
 }
 0x1300a54f => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x1300a56a; continue; }
  pc = 0x1300a554; continue;
 }
 0x1300a554 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300a560 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  pc = 0x1300a56d; continue;
 }
 0x1300a565 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  pc = 0x1300a56d; continue;
 }
 0x1300a56a => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  pc = 0x1300a56d; continue;
 }
 0x1300a56d => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x1300a586; continue; }
  pc = 0x1300a57a; continue;
 }
 0x1300a57a => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300a586 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edi)), cpu.ebx); }
  if !cpu.cf { pc = 0x1300a5bd; continue; }
  pc = 0x1300a58b; continue;
 }
 0x1300a58b => {
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x1300a59d; continue; }
  pc = 0x1300a591; continue;
 }
 0x1300a591 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300a59d => {
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.edi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300a5a5; continue; }
  pc = 0x1300a5a2; continue;
 }
 0x1300a5a2 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x50u32.wrapping_add(cpu.edi), v); }
  pc = 0x1300a5a5; continue;
 }
 0x1300a5a5 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x1300a5ac); f_1300a260(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300a5bd; continue; }
  pc = 0x1300a5b1; continue;
 }
 0x1300a5b1 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300a5bd => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edi)), 0x0u32); }
  if cpu.zf { pc = 0x1300a5cd; continue; }
  pc = 0x1300a5c8; continue;
 }
 0x1300a5c8 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x1300a5cd; continue;
 }
 0x1300a5cd => {
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
pub fn f_1300a5e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a5e0); }
 let mut pc: u32 = 0x1300a5e0;
 loop { match pc {
 0x1300a5e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300a6b7u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.esi); }
  if cpu.zf { pc = 0x1300a621; continue; }
  pc = 0x1300a60c; continue;
 }
 0x1300a60c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = 0x1300c118u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x1300a61a); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300a621; continue;
 }
 0x1300a621 => {
  { let v = 0x64u32; cpu.push(v); }
  cpu.push(0x1300a628); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1300a64f; continue; }
  pc = 0x1300a640; continue;
 }
 0x1300a640 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a64f); f_1300a0d0(cpu);
  pc = 0x1300a64f; continue;
 }
 0x1300a64f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a65d); f_13005af0(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c124u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a698(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a698); }
 let mut pc: u32 = 0x1300a698;
 loop { match pc {
 0x1300a698 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300a6a9; continue; }
  pc = 0x1300a69e; continue;
 }
 0x1300a69e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300a6a9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a6aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a6aa); }
 let mut pc: u32 = 0x1300a6aa;
 loop { match pc {
 0x1300a6aa => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a6b3); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a6b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a6b7); }
 let mut pc: u32 = 0x1300a6b7;
 loop { match pc {
 0x1300a6b7 => {
  { let v: u32 = 0x1300d628u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a6c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a6c1); }
 let mut pc: u32 = 0x1300a6c1;
 loop { match pc {
 0x1300a6c1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300a6d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300a6d0); }
 let mut pc: u32 = 0x1300a6d0;
 loop { match pc {
 0x1300a6d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300a7a9u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.esi); }
  if cpu.zf { pc = 0x1300a711; continue; }
  pc = 0x1300a6fc; continue;
 }
 0x1300a6fc => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = 0x1300c118u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x1300a70a); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1300a711; continue;
 }
 0x1300a711 => {
  { let v = 0x64u32; cpu.push(v); }
  cpu.push(0x1300a718); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1300a741; continue; }
  pc = 0x1300a730; continue;
 }
 0x1300a730 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300a741); f_1300a0d0(cpu);
  pc = 0x1300a741; continue;
 }
 0x1300a741 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300a74f); f_13005af0(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c124u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
