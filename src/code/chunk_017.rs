#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_11001000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001000); }
 let mut pc: u32 = 0x11001000;
 loop { match pc {
 0x11001000 => {
  pc = 0x110037e2; continue;
 }
 0x110037e2 => {
  { let v: u32 = 0x11003857u32; cpu.eax = v; }
  cpu.push(0x110037ec); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x1100380e; continue; }
  pc = 0x110037f9; continue;
 }
 0x110037f9 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x11001055u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v = 0x18u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1100380b); f_11005cc0(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11003830; continue;
 }
 0x1100380e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100381b); f_1100384c(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003827); f_11003861(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11003838; continue; }
  pc = 0x1100382d; continue;
 }
 0x1100382d => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  pc = 0x11003830; continue;
 }
 0x11003830 => {
  cpu.push(0x11003835); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11003838; continue;
 }
 0x11003838 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_11001005(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001005); }
 let mut pc: u32 = 0x11001005;
 loop { match pc {
 0x11001005 => {
  pc = 0x110039fc; continue;
 }
 0x110039fc => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100100a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100100a); }
 let mut pc: u32 = 0x1100100a;
 loop { match pc {
 0x1100100a => {
  pc = 0x11004753; continue;
 }
 0x11004753 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), cpu.esi); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x1100476b; continue; }
  pc = 0x11004762; continue;
 }
 0x11004762 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1100476b); f_1100143d(cpu);
  pc = 0x1100476b; continue;
 }
 0x1100476b => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.div32(v); }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { pc = 0x110047ae; continue; }
  pc = 0x11004784; continue;
 }
 0x11004784 => {
  if (cpu.cf || cpu.zf) { pc = 0x1100479b; continue; }
  pc = 0x11004786; continue;
 }
 0x11004786 => {
  if (cpu.cf || cpu.zf) { pc = 0x110047ae; continue; }
  pc = 0x11004788; continue;
 }
 0x11004788 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1100478c; continue;
 }
 0x1100478c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  cpu.push(0x11004796); f_110011f4(cpu);
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1100478c; continue; }
  pc = 0x11004799; continue;
 }
 0x11004799 => {
  pc = 0x110047ae; continue;
 }
 0x1100479b => {
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.cf { pc = 0x110047ae; continue; }
  pc = 0x1100479f; continue;
 }
 0x1100479f => {
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  pc = 0x110047a1; continue;
 }
 0x110047a1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  cpu.push(0x110047ab); f_11001406(cpu);
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x110047a1; continue; }
  pc = 0x110047ae; continue;
 }
 0x110047ae => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100100f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100100f); }
 let mut pc: u32 = 0x1100100f;
 loop { match pc {
 0x1100100f => {
  pc = 0x11001b2e; continue;
 }
 0x11001b2e => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x11001b4e; continue; }
  pc = 0x11001b39; continue;
 }
 0x11001b39 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x1100119fu32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11001b4b); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x11001b5c; continue;
 }
 0x11001b4e => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  { let v: u32 = 0x1100705cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x11001b64; continue; }
  pc = 0x11001b5b; continue;
 }
 0x11001b5b => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11001b5c; continue;
 }
 0x11001b5c => {
  cpu.push(0x11001b61); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11001b64; continue;
 }
 0x11001b64 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001014(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001014); }
 let mut pc: u32 = 0x11001014;
 loop { match pc {
 0x11001014 => {
  pc = 0x110024fa; continue;
 }
 0x110024fa => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  cpu.push(0x11002504); f_110010a5(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11002515); f_1100104b(cpu);
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1100252c; continue; }
  pc = 0x1100251b; continue;
 }
 0x1100251b => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1100251f; continue;
 }
 0x1100251f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1100251f; continue; }
  pc = 0x1100252c; continue;
 }
 0x1100252c => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001019(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001019); }
 let mut pc: u32 = 0x11001019;
 loop { match pc {
 0x11001019 => {
  pc = 0x11002046; continue;
 }
 0x11002046 => {
  cpu.push(0x1100204b); f_110010a5(cpu);
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100101e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100101e); }
 let mut pc: u32 = 0x1100101e;
 loop { match pc {
 0x1100101e => {
  pc = 0x11002336; continue;
 }
 0x11002336 => {
  { let v: u32 = 0x11002387u32; cpu.eax = v; }
  cpu.push(0x11002340); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11002352); f_110010a5(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11002360); f_1100120d(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001023(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001023); }
 let mut pc: u32 = 0x11001023;
 loop { match pc {
 0x11001023 => {
  pc = 0x11005b02; continue;
 }
 0x11005b02 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x11005b09); f_11001393(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001028(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001028); }
 let mut pc: u32 = 0x11001028;
 loop { match pc {
 0x11001028 => {
  pc = 0x11003a73; continue;
 }
 0x11003a73 => {
  { let v: u32 = 0x11003ab0u32; cpu.eax = v; }
  cpu.push(0x11003a7d); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11003a8b); f_11001046(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x110070a4u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100102d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100102d); }
 let mut pc: u32 = 0x1100102d;
 loop { match pc {
 0x1100102d => {
  pc = 0x11001cee; continue;
 }
 0x11001cee => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), cpu.edx); }
  if !cpu.zf { pc = 0x11001d02; continue; }
  pc = 0x11001cfa; continue;
 }
 0x11001cfa => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11001d23; continue;
 }
 0x11001d02 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x11001d08; continue;
 }
 0x11001d08 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x11001d17; continue; }
  pc = 0x11001d0f; continue;
 }
 0x11001d0f => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x11001d08; continue; }
  pc = 0x11001d15; continue;
 }
 0x11001d15 => {
  pc = 0x11001d23; continue;
 }
 0x11001d17 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); }
  if !cpu.zf { pc = 0x11001d23; continue; }
  pc = 0x11001d21; continue;
 }
 0x11001d21 => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11001d23; continue;
 }
 0x11001d23 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001032(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001032); }
 let mut pc: u32 = 0x11001032;
 loop { match pc {
 0x11001032 => {
  pc = 0x11003b5c; continue;
 }
 0x11003b5c => {
  { let v: u32 = 0x11003b85u32; cpu.eax = v; }
  cpu.push(0x11003b66); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003b78); f_11003b8f(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001037(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001037); }
 let mut pc: u32 = 0x11001037;
 loop { match pc {
 0x11001037 => {
  pc = 0x11004f24; continue;
 }
 0x11004f24 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x11004f45; continue; }
  pc = 0x11004f2d; continue;
 }
 0x11004f2d => {
  cpu.push(0x11004f32); f_1100110e(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11004f45; continue; }
  pc = 0x11004f36; continue;
 }
 0x11004f36 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if cpu.zf { pc = 0x11004f47; continue; }
  pc = 0x11004f45; continue;
 }
 0x11004f45 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x11004f47; continue;
 }
 0x11004f47 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100103c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100103c); }
 let mut pc: u32 = 0x1100103c;
 loop { match pc {
 0x1100103c => {
  pc = 0x110019ce; continue;
 }
 0x110019ce => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001041(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001041); }
 let mut pc: u32 = 0x11001041;
 loop { match pc {
 0x11001041 => {
  pc = 0x110050c2; continue;
 }
 0x110050c2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x110050d6; continue; }
  pc = 0x110050c8; continue;
 }
 0x110050c8 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x110050d6); f_11001262(cpu);
  pc = 0x110050d6; continue;
 }
 0x110050d6 => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001046(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001046); }
 let mut pc: u32 = 0x11001046;
 loop { match pc {
 0x11001046 => {
  pc = 0x11003294; continue;
 }
 0x11003294 => {
  { let v: u32 = 0x110032dfu32; cpu.eax = v; }
  cpu.push(0x1100329e); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11007084u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x11007094u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x110032c0); f_11001244(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x110032c7); f_1100114f(cpu);
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
pub fn f_1100104b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100104b); }
 let mut pc: u32 = 0x1100104b;
 loop { match pc {
 0x1100104b => {
  pc = 0x11002056; continue;
 }
 0x11002056 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x11002061); f_110010a5(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.zf { pc = 0x110020cc; continue; }
  pc = 0x1100206b; continue;
 }
 0x1100206b => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11002074); f_11005de6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x11002091; continue; }
  pc = 0x1100207d; continue;
 }
 0x1100207d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x110020ba; continue; }
  pc = 0x11002081; continue;
 }
 0x11002081 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x1100e268u32); cpu.push(0x1100208c); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x110020ba; continue;
 }
 0x11002091 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x1100e268u32); cpu.push(0x1100209c); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edi); cpu.edx = v; }
  if cpu.zf { pc = 0x110020ba; continue; }
  pc = 0x110020a5; continue;
 }
 0x110020a5 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x20202020u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  pc = 0x110020ba; continue;
 }
 0x110020ba => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(v); }
  cpu.push(0x110020c6); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x110020cc; continue;
 }
 0x110020cc => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001050); }
 let mut pc: u32 = 0x11001050;
 loop { match pc {
 0x11001050 => {
  pc = 0x11004a55; continue;
 }
 0x11004a55 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  cpu.push(0x11004a5d); f_1100128a(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001055(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001055); }
 let mut pc: u32 = 0x11001055;
 loop { match pc {
 0x11001055 => {
  pc = 0x110036ef; continue;
 }
 0x110036ef => {
  { let v: u32 = 0x11003730u32; cpu.eax = v; }
  cpu.push(0x110036f9); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100370c); f_11003725(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003718); f_1100373a(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100105a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100105a); }
 let mut pc: u32 = 0x1100105a;
 loop { match pc {
 0x1100105a => {
  pc = 0x11005394; continue;
 }
 0x11005394 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1df5e0du32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  cpu.push(0x110053a4); f_11001069(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x5f5e100u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100105f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100105f); }
 let mut pc: u32 = 0x1100105f;
 loop { match pc {
 0x1100105f => {
  pc = 0x11004e2b; continue;
 }
 0x11004e2b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.ecx); }
  if !cpu.zf { pc = 0x11004e46; continue; }
  pc = 0x11004e39; continue;
 }
 0x11004e39 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if cpu.zf { pc = 0x11004e48; continue; }
  pc = 0x11004e46; continue;
 }
 0x11004e46 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x11004e48; continue;
 }
 0x11004e48 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001064(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001064); }
 let mut pc: u32 = 0x11001064;
 loop { match pc {
 0x11001064 => {
  pc = 0x11004d39; continue;
 }
 0x11004d39 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004d6c; continue; }
  pc = 0x11004d43; continue;
 }
 0x11004d43 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x11004d47; continue;
 }
 0x11004d47 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x11004d56; continue; }
  pc = 0x11004d4b; continue;
 }
 0x11004d4b => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11004d47; continue; }
  pc = 0x11004d54; continue;
 }
 0x11004d54 => {
  pc = 0x11004d6c; continue;
 }
 0x11004d56 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x11004d62; continue; }
  pc = 0x11004d5d; continue;
 }
 0x11004d5d => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x11004d65; continue;
 }
 0x11004d62 => {
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x11004d65; continue;
 }
 0x11004d65 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x11004d6c; continue;
 }
 0x11004d6c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001069(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001069); }
 let mut pc: u32 = 0x11001069;
 loop { match pc {
 0x11001069 => {
  pc = 0x110052a0; continue;
 }
 0x110052a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x2710u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = 0x5f5e100u32; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100106e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100106e); }
 let mut pc: u32 = 0x1100106e;
 loop { match pc {
 0x1100106e => {
  pc = 0x1100216f; continue;
 }
 0x1100216f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1100e264u32); cpu.push(0x1100218c); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1100219e; continue; }
  pc = 0x11002196; continue;
 }
 0x11002196 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1100219e; continue;
 }
 0x1100219e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001073(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001073); }
 let mut pc: u32 = 0x11001073;
 loop { match pc {
 0x11001073 => {
  pc = 0x110020f4; continue;
 }
 0x110020f4 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x110020fb); f_1100104b(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001078); }
 let mut pc: u32 = 0x11001078;
 loop { match pc {
 0x11001078 => {
  pc = 0x1100375a; continue;
 }
 0x1100375a => {
  { let v: u32 = 0x110037b2u32; cpu.eax = v; }
  cpu.push(0x11003764); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003777); f_110037a7(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003783); f_110037bc(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11003794; continue; }
  pc = 0x11003789; continue;
 }
 0x11003789 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11003791); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11003794; continue;
 }
 0x11003794 => {
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
pub fn f_1100107d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100107d); }
 let mut pc: u32 = 0x1100107d;
 loop { match pc {
 0x1100107d => {
  pc = 0x110031f5; continue;
 }
 0x110031f5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001082(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001082); }
 let mut pc: u32 = 0x11001082;
 loop { match pc {
 0x11001082 => {
  pc = 0x11003ad5; continue;
 }
 0x11003ad5 => {
  { let v: u32 = 0x11003b2bu32; cpu.eax = v; }
  cpu.push(0x11003adf); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11007084u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x11007094u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = 0x110070a4u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001087(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001087); }
 let mut pc: u32 = 0x11001087;
 loop { match pc {
 0x11001087 => {
  pc = 0x11001f8d; continue;
 }
 0x11001f8d => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { let v: u32 = 0x11007074u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x11001f9b); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100108c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100108c); }
 let mut pc: u32 = 0x1100108c;
 loop { match pc {
 0x1100108c => {
  pc = 0x110026e6; continue;
 }
 0x110026e6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  if cpu.zf { pc = 0x110026fe; continue; }
  pc = 0x110026f7; continue;
 }
 0x110026f7 => {
  cpu.push(0x110026fc); f_11005e44(cpu);
  pc = 0x11002704; continue;
 }
 0x110026fe => {
  let t = cpu.r32(0x1100e220u32); cpu.push(0x11002704); cpu.call(t);
  pc = 0x11002704; continue;
 }
 0x11002704 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001091(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001091); }
 let mut pc: u32 = 0x11001091;
 loop { match pc {
 0x11001091 => {
  pc = 0x11001ae2; continue;
 }
 0x11001ae2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x11001afd; continue; }
  pc = 0x11001afa; continue;
 }
 0x11001afa => {
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11001afd; continue;
 }
 0x11001afd => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001096(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001096); }
 let mut pc: u32 = 0x11001096;
 loop { match pc {
 0x11001096 => {
  pc = 0x11004492; continue;
 }
 0x11004492 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1100449a); f_11001456(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x110044aa; continue; }
  pc = 0x110044a1; continue;
 }
 0x110044a1 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110044a7); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110044aa; continue;
 }
 0x110044aa => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100109b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100109b); }
 let mut pc: u32 = 0x1100109b;
 loop { match pc {
 0x1100109b => {
  pc = 0x110030ab; continue;
 }
 0x110030ab => {
  { let v: u32 = 0x1100310fu32; cpu.eax = v; }
  cpu.push(0x110030b5); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x110030c9); f_110011ea(cpu);
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x110030db); f_110011ea(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010a0); }
 let mut pc: u32 = 0x110010a0;
 loop { match pc {
 0x110010a0 => {
  pc = 0x11004cbe; continue;
 }
 0x11004cbe => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11004cc8); f_1100110e(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004ce2; continue; }
  pc = 0x11004ccc; continue;
 }
 0x11004ccc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004d0b; continue; }
  pc = 0x11004cd3; continue;
 }
 0x11004cd3 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x11004cd6; continue;
 }
 0x11004cd6 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11004cd6; continue; }
  pc = 0x11004ce0; continue;
 }
 0x11004ce0 => {
  pc = 0x11004d0b; continue;
 }
 0x11004ce2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x11004d0b; continue; }
  pc = 0x11004ce9; continue;
 }
 0x11004ce9 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x11004ced; continue;
 }
 0x11004ced => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.edx); }
  if !cpu.zf { pc = 0x11004d04; continue; }
  pc = 0x11004cf4; continue;
 }
 0x11004cf4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { pc = 0x11004d02; continue; }
  pc = 0x11004d00; continue;
 }
 0x11004d00 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x11004d02; continue;
 }
 0x11004d02 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x11004d04; continue;
 }
 0x11004d04 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x11004ced; continue; }
  pc = 0x11004d0b; continue;
 }
 0x11004d0b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010a5); }
 let mut pc: u32 = 0x110010a5;
 loop { match pc {
 0x110010a5 => {
  pc = 0x11002037; continue;
 }
 0x11002037 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  cpu.push(0x1100203f); f_11005dec(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010aa); }
 let mut pc: u32 = 0x110010aa;
 loop { match pc {
 0x110010aa => {
  pc = 0x1100577c; continue;
 }
 0x1100577c => {
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010af); }
 let mut pc: u32 = 0x110010af;
 loop { match pc {
 0x110010af => {
  pc = 0x11001a87; continue;
 }
 0x11001a87 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010b4); }
 let mut pc: u32 = 0x110010b4;
 loop { match pc {
 0x110010b4 => {
  pc = 0x11001a5f; continue;
 }
 0x11001a5f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11001a67); f_110010af(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11001a7d; continue; }
  pc = 0x11001a6b; continue;
 }
 0x11001a6b => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11001a72); f_110010d2(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11001a79); f_110010af(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11001a6b; continue; }
  pc = 0x11001a7d; continue;
 }
 0x11001a7d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010b9); }
 let mut pc: u32 = 0x110010b9;
 loop { match pc {
 0x110010b9 => {
  pc = 0x11004ec6; continue;
 }
 0x11004ec6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004ed2; continue; }
  pc = 0x11004ecc; continue;
 }
 0x11004ecc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11004ed2; continue;
 }
 0x11004ed2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010be); }
 let mut pc: u32 = 0x110010be;
 loop { match pc {
 0x110010be => {
  pc = 0x11005490; continue;
 }
 0x11005490 => {
  { let v: u32 = 0x110054d1u32; cpu.eax = v; }
  cpu.push(0x1100549a); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x11007184u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110054b5); f_11005cb6(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x110054c4); f_110054db(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010c3); }
 let mut pc: u32 = 0x110010c3;
 loop { match pc {
 0x110010c3 => {
  pc = 0x11003556; continue;
 }
 0x11003556 => {
  { let v: u32 = 0x110035dcu32; cpu.eax = v; }
  cpu.push(0x11003560); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  cpu.push(0x11003572); f_110010dc(cpu);
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x1100e1d4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  let t = cpu.r32(0x1100e1dcu32); cpu.push(0x11003583); cpu.call(t);
  { let v = 0x1100d604u32; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  let t = cpu.r32(0x1100e1dcu32); cpu.push(0x11003590); cpu.call(t);
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1100359c); f_1100147e(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x1100e1d4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x1100e1dcu32); cpu.push(0x110035b4); cpu.call(t);
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x1100e1d8u32); cpu.push(0x110035bb); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x110035ca); f_110035e6(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010c8); }
 let mut pc: u32 = 0x110010c8;
 loop { match pc {
 0x110010c8 => {
  pc = 0x110058e1; continue;
 }
 0x110058e1 => {
  { let v: u32 = 0x1100590au32; cpu.eax = v; }
  cpu.push(0x110058eb); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110058fd); f_11005914(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010cd); }
 let mut pc: u32 = 0x110010cd;
 loop { match pc {
 0x110010cd => {
  pc = 0x11005bfc; continue;
 }
 0x11005bfc => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x11005c03); f_11001343(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010d2); }
 let mut pc: u32 = 0x110010d2;
 loop { match pc {
 0x110010d2 => {
  pc = 0x11001a53; continue;
 }
 0x11001a53 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11001a5c); f_11001442(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010d7); }
 let mut pc: u32 = 0x110010d7;
 loop { match pc {
 0x110010d7 => {
  pc = 0x11004917; continue;
 }
 0x11004917 => {
  { let v: u32 = 0x11004969u32; cpu.eax = v; }
  cpu.push(0x11004921); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x110070b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004939); f_11001172(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004945); f_11004973(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11004956; continue; }
  pc = 0x1100494b; continue;
 }
 0x1100494b => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11004953); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11004956; continue;
 }
 0x11004956 => {
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
pub fn f_110010dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010dc); }
 let mut pc: u32 = 0x110010dc;
 loop { match pc {
 0x110010dc => {
  pc = 0x11003387; continue;
 }
 0x11003387 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x1100d8d0u32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010e1); }
 let mut pc: u32 = 0x110010e1;
 loop { match pc {
 0x110010e1 => {
  pc = 0x11002fcf; continue;
 }
 0x11002fcf => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x11002fef; continue; }
  pc = 0x11002fda; continue;
 }
 0x11002fda => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x11001087u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11002fec); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x11002ffe; continue;
 }
 0x11002fef => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11002ff6); f_11001087(cpu);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11003006; continue; }
  pc = 0x11002ffd; continue;
 }
 0x11002ffd => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11002ffe; continue;
 }
 0x11002ffe => {
  cpu.push(0x11003003); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11003006; continue;
 }
 0x11003006 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010e6); }
 let mut pc: u32 = 0x110010e6;
 loop { match pc {
 0x110010e6 => {
  pc = 0x11004604; continue;
 }
 0x11004604 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11004610); f_110013a7(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010eb); }
 let mut pc: u32 = 0x110010eb;
 loop { match pc {
 0x110010eb => {
  pc = 0x11002682; continue;
 }
 0x11002682 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1100268b); f_110010a5(cpu);
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x110026ae; continue; }
  pc = 0x11002692; continue;
 }
 0x11002692 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.push(v); }
  let t = cpu.r32(0x1100e218u32); cpu.push(0x110026a4); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x110026ae; continue; }
  pc = 0x110026ab; continue;
 }
 0x110026ab => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if !cpu.sf { pc = 0x11002692; continue; }
  pc = 0x110026ae; continue;
 }
 0x110026ae => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x110026b7); f_1100104b(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010f0); }
 let mut pc: u32 = 0x110010f0;
 loop { match pc {
 0x110010f0 => {
  pc = 0x110049fe; continue;
 }
 0x110049fe => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x8u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x11004a17; continue; }
  pc = 0x11004a07; continue;
 }
 0x11004a07 => {
  cpu.push(0x11004a0c); f_11001172(cpu);
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11004a17); f_1100148d(cpu);
  pc = 0x11004a17; continue;
 }
 0x11004a17 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010f5); }
 let mut pc: u32 = 0x110010f5;
 loop { match pc {
 0x110010f5 => {
  pc = 0x11002208; continue;
 }
 0x11002208 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010fa); }
 let mut pc: u32 = 0x110010fa;
 loop { match pc {
 0x110010fa => {
  pc = 0x11003c95; continue;
 }
 0x11003c95 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11003cb2); f_11005de6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110010ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110010ff); }
 let mut pc: u32 = 0x110010ff;
 loop { match pc {
 0x110010ff => {
  pc = 0x11002115; continue;
 }
 0x11002115 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  let t = cpu.r32(0x1100e26cu32); cpu.push(0x11002121); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001109(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001109); }
 let mut pc: u32 = 0x11001109;
 loop { match pc {
 0x11001109 => {
  pc = 0x11005990; continue;
 }
 0x11005990 => {
  { let v: u32 = 0x110059edu32; cpu.eax = v; }
  cpu.push(0x1100599a); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x110059bc; continue; }
  pc = 0x110059a7; continue;
 }
 0x110059a7 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x110010c8u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x110059b9); f_11005cc0(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x110059d1; continue;
 }
 0x110059bc => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110059c8); f_110059f7(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x110059d9; continue; }
  pc = 0x110059ce; continue;
 }
 0x110059ce => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  pc = 0x110059d1; continue;
 }
 0x110059d1 => {
  cpu.push(0x110059d6); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110059d9; continue;
 }
 0x110059d9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_1100110e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100110e); }
 let mut pc: u32 = 0x1100110e;
 loop { match pc {
 0x1100110e => {
  pc = 0x11004a88; continue;
 }
 0x11004a88 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001113(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001113); }
 let mut pc: u32 = 0x11001113;
 loop { match pc {
 0x11001113 => {
  pc = 0x11003900; continue;
 }
 0x11003900 => {
  { let v: u32 = 0x11007084u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001118(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001118); }
 let mut pc: u32 = 0x11001118;
 loop { match pc {
 0x11001118 => {
  pc = 0x11001f08; continue;
 }
 0x11001f08 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11007074u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x11001f27; continue; }
  pc = 0x11001f1a; continue;
 }
 0x11001f1a => {
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11001f22); f_1100146a(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11001f2a; continue;
 }
 0x11001f27 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x11001f2a; continue;
 }
 0x11001f2a => {
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100111d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100111d); }
 let mut pc: u32 = 0x1100111d;
 loop { match pc {
 0x1100111d => {
  pc = 0x11002be4; continue;
 }
 0x11002be4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11002bf3); f_1100108c(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001122(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001122); }
 let mut pc: u32 = 0x11001122;
 loop { match pc {
 0x11001122 => {
  pc = 0x11001ac6; continue;
 }
 0x11001ac6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x11001ad1); f_1100103c(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1100705cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001127(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001127); }
 let mut pc: u32 = 0x11001127;
 loop { match pc {
 0x11001127 => {
  pc = 0x110031e5; continue;
 }
 0x110031e5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100112c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100112c); }
 let mut pc: u32 = 0x1100112c;
 loop { match pc {
 0x1100112c => {
  pc = 0x11003ba5; continue;
 }
 0x11003ba5 => {
  { let v: u32 = 0x11003be5u32; cpu.eax = v; }
  cpu.push(0x11003baf); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003bc1); f_11003bef(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11003bd2; continue; }
  pc = 0x11003bc7; continue;
 }
 0x11003bc7 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11003bcf); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11003bd2; continue;
 }
 0x11003bd2 => {
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
pub fn f_11001131(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001131); }
 let mut pc: u32 = 0x11001131;
 loop { match pc {
 0x11001131 => {
  pc = 0x11004e50; continue;
 }
 0x11004e50 => {
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11004e5d); f_1100105f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001136(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001136); }
 let mut pc: u32 = 0x11001136;
 loop { match pc {
 0x11001136 => {
  pc = 0x11002b1f; continue;
 }
 0x11002b1f => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11002b2a); f_11001145(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100113b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100113b); }
 let mut pc: u32 = 0x1100113b;
 loop { match pc {
 0x1100113b => {
  pc = 0x110031ff; continue;
 }
 0x110031ff => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001140); }
 let mut pc: u32 = 0x11001140;
 loop { match pc {
 0x11001140 => {
  pc = 0x11002f28; continue;
 }
 0x11002f28 => {
  { let v: u32 = 0x11002f86u32; cpu.eax = v; }
  cpu.push(0x11002f32); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11002f49); f_110011e5(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11002f5b); f_11001145(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002f6a); f_11002f7e(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_11001145(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001145); }
 let mut pc: u32 = 0x11001145;
 loop { match pc {
 0x11001145 => {
  pc = 0x110024d7; continue;
 }
 0x110024d7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x110024e7); f_110010a5(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110024ef); f_1100142e(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100114a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100114a); }
 let mut pc: u32 = 0x1100114a;
 loop { match pc {
 0x1100114a => {
  pc = 0x110047eb; continue;
 }
 0x110047eb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), cpu.eax); }
  if cpu.cf { pc = 0x110047f8; continue; }
  pc = 0x110047f4; continue;
 }
 0x110047f4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x11004810; continue;
 }
 0x110047f8 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x1100480d; continue; }
  pc = 0x11004801; continue;
 }
 0x11004801 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x11004810; continue;
 }
 0x1100480d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x11004810; continue;
 }
 0x11004810 => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100114f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100114f); }
 let mut pc: u32 = 0x1100114f;
 loop { match pc {
 0x1100114f => {
  pc = 0x110031e9; continue;
 }
 0x110031e9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001154(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001154); }
 let mut pc: u32 = 0x11001154;
 loop { match pc {
 0x11001154 => {
  pc = 0x11004a24; continue;
 }
 0x11004a24 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11004a31); f_11001262(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001159(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001159); }
 let mut pc: u32 = 0x11001159;
 loop { match pc {
 0x11001159 => {
  pc = 0x11005715; continue;
 }
 0x11005715 => {
  { cpu.and8(cpu.r8(0x1100d908u32), 0x1u8); }
  if !cpu.zf { pc = 0x1100573e; continue; }
  pc = 0x1100571e; continue;
 }
 0x1100571e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1100d910u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1100d908u32) | 0x1u8; cpu.w8(0x1100d908u32, v); }
  cpu.push(0x11005731); f_11001343(cpu);
  { let v = 0x11005770u32; cpu.push(v); }
  cpu.push(0x1100573b); f_11005f00(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1100573e; continue;
 }
 0x1100573e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11005753; continue; }
  pc = 0x11005746; continue;
 }
 0x11005746 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1100d910u32; cpu.ecx = v; }
  cpu.push(0x11005751); f_11001186(cpu);
  pc = 0x1100575d; continue;
 }
 0x11005753 => {
  { let v: u32 = 0x1100d910u32; cpu.ecx = v; }
  f_1100118b(cpu); return;
 }
 0x1100575d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100115e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100115e); }
 let mut pc: u32 = 0x1100115e;
 loop { match pc {
 0x1100115e => {
  pc = 0x11005ab9; continue;
 }
 0x11005ab9 => {
  { let v: u32 = 0x11005ae2u32; cpu.eax = v; }
  cpu.push(0x11005ac3); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11005ad5); f_11005aec(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001163(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001163); }
 let mut pc: u32 = 0x11001163;
 loop { match pc {
 0x11001163 => {
  pc = 0x11003912; continue;
 }
 0x11003912 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x11007084u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001168(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001168); }
 let mut pc: u32 = 0x11001168;
 loop { match pc {
 0x11001168 => {
  pc = 0x11004545; continue;
 }
 0x11004545 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x11004565; continue; }
  pc = 0x11004550; continue;
 }
 0x11004550 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x1100132au32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11004562); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x11004574; continue;
 }
 0x11004565 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1100456c); f_1100132a(cpu);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x1100457c; continue; }
  pc = 0x11004573; continue;
 }
 0x11004573 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11004574; continue;
 }
 0x11004574 => {
  cpu.push(0x11004579); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1100457c; continue;
 }
 0x1100457c => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100116d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100116d); }
 let mut pc: u32 = 0x1100116d;
 loop { match pc {
 0x1100116d => {
  pc = 0x11004ac1; continue;
 }
 0x11004ac1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11004acf); f_1100110e(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11004b09; continue; }
  pc = 0x11004ad3; continue;
 }
 0x11004ad3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11004b09); f_11001217(cpu);
  pc = 0x11004b09; continue;
 }
 0x11004b09 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001172(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001172); }
 let mut pc: u32 = 0x11001172;
 loop { match pc {
 0x11001172 => {
  pc = 0x11004a60; continue;
 }
 0x11004a60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11004a68); f_1100110e(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11004a7e; continue; }
  pc = 0x11004a6c; continue;
 }
 0x11004a6c => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11004a73); f_11001424(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11004a7a); f_1100110e(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004a6c; continue; }
  pc = 0x11004a7e; continue;
 }
 0x11004a7e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001177(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001177); }
 let mut pc: u32 = 0x11001177;
 loop { match pc {
 0x11001177 => {
  pc = 0x11004d7d; continue;
 }
 0x11004d7d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100117c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100117c); }
 let mut pc: u32 = 0x1100117c;
 loop { match pc {
 0x1100117c => {
  pc = 0x11003996; continue;
 }
 0x11003996 => {
  { let v: u32 = 0x110039d6u32; cpu.eax = v; }
  cpu.push(0x110039a0); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11007084u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x11007094u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
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
pub fn f_11001181(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001181); }
 let mut pc: u32 = 0x11001181;
 loop { match pc {
 0x11001181 => {
  pc = 0x11005123; continue;
 }
 0x11005123 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xau32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x11005144; continue;
 }
 0x11005141 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x11005144; continue;
 }
 0x11005144 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x1100514e); cpu.call(t);
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xau32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1100515b); f_11001460(cpu);
  { let v = cpu.st(0) + cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x11005141; continue; }
  pc = 0x11005164; continue;
 }
 0x11005164 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = (cpu.r64(0xfffffff0u32.wrapping_add(cpu.ebp)) as i64 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x11007118u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  cpu.push(0x11005188); f_11005f20(cpu);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1100518e); f_11005f1a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = (cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  cpu.push(0x110051a5); f_11005f14(cpu);
  { let v = cpu.st(0) * cpu.rf64(0x11007120u32); cpu.set_st(0, v); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x110051c1; continue; }
  pc = 0x110051bf; continue;
 }
 0x110051bf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x110051c1; continue;
 }
 0x110051c1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001186(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001186); }
 let mut pc: u32 = 0x11001186;
 loop { match pc {
 0x11001186 => {
  pc = 0x11005552; continue;
 }
 0x11005552 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x11005557); cpu.call(t);
  { let v: u32 = 0x2710u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100118b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100118b); }
 let mut pc: u32 = 0x1100118b;
 loop { match pc {
 0x1100118b => {
  pc = 0x110054f7; continue;
 }
 0x110054f7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x37u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x36u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = 0x17u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = 0x5f5e100u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(4)))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001190); }
 let mut pc: u32 = 0x11001190;
 loop { match pc {
 0x11001190 => {
  pc = 0x110045b7; continue;
 }
 0x110045b7 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x110045d7; continue; }
  pc = 0x110045c2; continue;
 }
 0x110045c2 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x11001221u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110045d4); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x110045e6; continue;
 }
 0x110045d7 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110045de); f_11001221(cpu);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x110045ee; continue; }
  pc = 0x110045e5; continue;
 }
 0x110045e5 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x110045e6; continue;
 }
 0x110045e6 => {
  cpu.push(0x110045eb); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110045ee; continue;
 }
 0x110045ee => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001195(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001195); }
 let mut pc: u32 = 0x11001195;
 loop { match pc {
 0x11001195 => {
  pc = 0x110053df; continue;
 }
 0x110053df => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100119a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100119a); }
 let mut pc: u32 = 0x1100119a;
 loop { match pc {
 0x1100119a => {
  pc = 0x11005812; continue;
 }
 0x11005812 => {
  { let v: u32 = 0x11005850u32; cpu.eax = v; }
  cpu.push(0x1100581c); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x11007154u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100119f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100119f); }
 let mut pc: u32 = 0x1100119f;
 loop { match pc {
 0x1100119f => {
  pc = 0x11001abf; continue;
 }
 0x11001abf => {
  { let v: u32 = 0x1100705cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011a4); }
 let mut pc: u32 = 0x110011a4;
 loop { match pc {
 0x110011a4 => {
  pc = 0x11005b72; continue;
 }
 0x11005b72 => {
  { let v: u32 = 0x11005bcfu32; cpu.eax = v; }
  cpu.push(0x11005b7c); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x11005b9e; continue; }
  pc = 0x11005b89; continue;
 }
 0x11005b89 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x1100115eu32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x11005b9b); f_11005cc0(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11005bb3; continue;
 }
 0x11005b9e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11005baa); f_11005bd9(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11005bbb; continue; }
  pc = 0x11005bb0; continue;
 }
 0x11005bb0 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  pc = 0x11005bb3; continue;
 }
 0x11005bb3 => {
  cpu.push(0x11005bb8); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11005bbb; continue;
 }
 0x11005bbb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_110011a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011a9); }
 let mut pc: u32 = 0x110011a9;
 loop { match pc {
 0x110011a9 => {
  pc = 0x11004ff3; continue;
 }
 0x11004ff3 => {
  { let v: u32 = 0x11005028u32; cpu.eax = v; }
  cpu.push(0x11004ffd); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1100500e); f_11001271(cpu);
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
pub fn f_110011ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011ae); }
 let mut pc: u32 = 0x110011ae;
 loop { match pc {
 0x110011ae => {
  pc = 0x11004a38; continue;
 }
 0x11004a38 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11004a44); f_11001262(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011b3); }
 let mut pc: u32 = 0x110011b3;
 loop { match pc {
 0x110011b3 => {
  pc = 0x11004a9c; continue;
 }
 0x11004a9c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.esi); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xfffffffcu32); }
  if cpu.zf { pc = 0x11004ab8; continue; }
  pc = 0x11004aab; continue;
 }
 0x11004aab => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.esi); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xfffffffcu32); }
  if !cpu.zf { pc = 0x11004aab; continue; }
  pc = 0x11004ab8; continue;
 }
 0x11004ab8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011b8); }
 let mut pc: u32 = 0x110011b8;
 loop { match pc {
 0x110011b8 => {
  pc = 0x11002542; continue;
 }
 0x11002542 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1100e210u32); cpu.push(0x1100255a); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11002566; continue; }
  pc = 0x11002561; continue;
 }
 0x11002561 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  pc = 0x1100256b; continue;
 }
 0x11002566 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1100256b; continue;
 }
 0x1100256b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011bd); }
 let mut pc: u32 = 0x110011bd;
 loop { match pc {
 0x110011bd => {
  pc = 0x1100301c; continue;
 }
 0x1100301c => {
  { let v: u32 = 0x1100307au32; cpu.eax = v; }
  cpu.push(0x11003026); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.push(0x1100303d); f_110011ea(cpu);
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003048); f_1100132f(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
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
pub fn f_110011c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011c2); }
 let mut pc: u32 = 0x110011c2;
 loop { match pc {
 0x110011c2 => {
  pc = 0x11002713; continue;
 }
 0x11002713 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x11002732; continue; }
  pc = 0x1100272a; continue;
 }
 0x1100272a => {
  let t = cpu.r32(0x1100e22cu32); cpu.push(0x11002730); cpu.call(t);
  pc = 0x11002738; continue;
 }
 0x11002732 => {
  let t = cpu.r32(0x1100e228u32); cpu.push(0x11002738); cpu.call(t);
  pc = 0x11002738; continue;
 }
 0x11002738 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011cc); }
 let mut pc: u32 = 0x110011cc;
 loop { match pc {
 0x110011cc => {
  pc = 0x11001a9b; continue;
 }
 0x11001a9b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.esi); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xfffffffcu32); }
  if cpu.zf { pc = 0x11001ab6; continue; }
  pc = 0x11001aaa; continue;
 }
 0x11001aaa => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.esi); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xfffffffcu32); }
  if !cpu.zf { pc = 0x11001aaa; continue; }
  pc = 0x11001ab6; continue;
 }
 0x11001ab6 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011d1); }
 let mut pc: u32 = 0x110011d1;
 loop { match pc {
 0x110011d1 => {
  pc = 0x11004994; continue;
 }
 0x11004994 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011d6); }
 let mut pc: u32 = 0x110011d6;
 loop { match pc {
 0x110011d6 => {
  pc = 0x110027e0; continue;
 }
 0x110027e0 => {
  { let v: u32 = 0x11002857u32; cpu.eax = v; }
  cpu.push(0x110027ea); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110027fd); f_110012e4(cpu);
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = 0x1100d56cu32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x11002815); f_1100123f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11002821); f_1100104b(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x1100282a); f_110011ea(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002836); f_11002861(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011e0); }
 let mut pc: u32 = 0x110011e0;
 loop { match pc {
 0x110011e0 => {
  pc = 0x11005aa7; continue;
 }
 0x11005aa7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011e5); }
 let mut pc: u32 = 0x110011e5;
 loop { match pc {
 0x110011e5 => {
  pc = 0x11002936; continue;
 }
 0x11002936 => {
  { let v: u32 = 0x110029b0u32; cpu.eax = v; }
  cpu.push(0x11002940); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x28u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002953); f_110012e4(cpu);
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = 0x1100d574u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1100296e); f_1100123f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1100297a); f_1100104b(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x11002983); f_110011ea(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100298f); f_110029ba(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011ea); }
 let mut pc: u32 = 0x110011ea;
 loop { match pc {
 0x110011ea => {
  pc = 0x11001fa3; continue;
 }
 0x11001fa3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11007074u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x11001fb8); f_1100146a(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011ef); }
 let mut pc: u32 = 0x110011ef;
 loop { match pc {
 0x110011ef => {
  pc = 0x11004d25; continue;
 }
 0x11004d25 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011f4); }
 let mut pc: u32 = 0x110011f4;
 loop { match pc {
 0x110011f4 => {
  pc = 0x11004f89; continue;
 }
 0x11004f89 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x11004f9f; continue; }
  pc = 0x11004f8f; continue;
 }
 0x11004f8f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xfffffffcu32); }
  if cpu.zf { pc = 0x11004f9f; continue; }
  pc = 0x11004f99; continue;
 }
 0x11004f99 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11004f9f; continue;
 }
 0x11004f9f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011f9); }
 let mut pc: u32 = 0x110011f9;
 loop { match pc {
 0x110011f9 => {
  pc = 0x110044b7; continue;
 }
 0x110044b7 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x110044d7; continue; }
  pc = 0x110044c2; continue;
 }
 0x110044c2 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x11001456u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0x18u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110044d4); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x110044e6; continue;
 }
 0x110044d7 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110044de); f_11001456(cpu);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x110044ee; continue; }
  pc = 0x110044e5; continue;
 }
 0x110044e5 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x110044e6; continue;
 }
 0x110044e6 => {
  cpu.push(0x110044eb); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110044ee; continue;
 }
 0x110044ee => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110011fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110011fe); }
 let mut pc: u32 = 0x110011fe;
 loop { match pc {
 0x110011fe => {
  pc = 0x1100560e; continue;
 }
 0x1100560e => {
  { let v: u32 = 0x1100566eu32; cpu.eax = v; }
  cpu.push(0x11005618); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x11007154u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0x11007184u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1100564f); f_1100124e(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_11001203(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001203); }
 let mut pc: u32 = 0x11001203;
 loop { match pc {
 0x11001203 => {
  pc = 0x110020fe; continue;
 }
 0x110020fe => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  let t = cpu.r32(0x1100e270u32); cpu.push(0x1100210a); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001208(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001208); }
 let mut pc: u32 = 0x11001208;
 loop { match pc {
 0x11001208 => {
  pc = 0x11005783; continue;
 }
 0x11005783 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100120d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100120d); }
 let mut pc: u32 = 0x1100120d;
 loop { match pc {
 0x1100120d => {
  pc = 0x1100220c; continue;
 }
 0x1100220c => {
  { let v: u32 = 0x110022a3u32; cpu.eax = v; }
  cpu.push(0x11002216); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1100222a); f_110010a5(cpu);
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if cpu.cf { pc = 0x11002236; continue; }
  pc = 0x11002234; continue;
 }
 0x11002234 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x11002236; continue;
 }
 0x11002236 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1100223d); f_110010a5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11002263; continue; }
  pc = 0x11002241; continue;
 }
 0x11002241 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x11002249); f_1100132f(cpu);
  pc = 0x11002249; continue;
 }
 0x11002249 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x11002263 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1100226c); f_11005de6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x1100e268u32); cpu.push(0x11002280); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x11002292); f_11001118(cpu);
  pc = 0x11002249; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001212(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001212); }
 let mut pc: u32 = 0x11001212;
 loop { match pc {
 0x11001212 => {
  pc = 0x11001e05; continue;
 }
 0x11001e05 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001217(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001217); }
 let mut pc: u32 = 0x11001217;
 loop { match pc {
 0x11001217 => {
  pc = 0x11004ca4; continue;
 }
 0x11004ca4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004cb8; continue; }
  pc = 0x11004cab; continue;
 }
 0x11004cab => {
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  pc = 0x11004cae; continue;
 }
 0x11004cae => {
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11004cae; continue; }
  pc = 0x11004cb8; continue;
 }
 0x11004cb8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100121c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100121c); }
 let mut pc: u32 = 0x1100121c;
 loop { match pc {
 0x1100121c => {
  pc = 0x11002bf8; continue;
 }
 0x11002bf8 => {
  { let v: u32 = 0x11002c59u32; cpu.eax = v; }
  cpu.push(0x11002c02); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x11002c17); f_11001118(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002c27); f_1100108c(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.adc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xffffffffu32); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002c40); f_11002c51(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001221(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001221); }
 let mut pc: u32 = 0x11001221;
 loop { match pc {
 0x11001221 => {
  pc = 0x11004daa; continue;
 }
 0x11004daa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004db8; continue; }
  pc = 0x11004db0; continue;
 }
 0x11004db0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x11004db8); f_11001064(cpu);
  pc = 0x11004db8; continue;
 }
 0x11004db8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001226(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001226); }
 let mut pc: u32 = 0x11001226;
 loop { match pc {
 0x11001226 => {
  pc = 0x11001c26; continue;
 }
 0x11001c26 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if cpu.zf { pc = 0x11001c57; continue; }
  pc = 0x11001c3f; continue;
 }
 0x11001c3f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x11001c47); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if !cpu.zf { pc = 0x11001c3f; continue; }
  pc = 0x11001c57; continue;
 }
 0x11001c57 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100122b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100122b); }
 let mut pc: u32 = 0x1100122b;
 loop { match pc {
 0x1100122b => {
  pc = 0x11001e83; continue;
 }
 0x11001e83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001230); }
 let mut pc: u32 = 0x11001230;
 loop { match pc {
 0x11001230 => {
  pc = 0x11002775; continue;
 }
 0x11002775 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11002785); f_110010a5(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(v); }
  let t = cpu.r32(0x1100e230u32); cpu.push(0x1100278f); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001235(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001235); }
 let mut pc: u32 = 0x11001235;
 loop { match pc {
 0x11001235 => {
  pc = 0x11005c06; continue;
 }
 0x11005c06 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11005c0e); f_110010be(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11005c1e; continue; }
  pc = 0x11005c15; continue;
 }
 0x11005c15 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11005c1b); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11005c1e; continue;
 }
 0x11005c1e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100123a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100123a); }
 let mut pc: u32 = 0x1100123a;
 loop { match pc {
 0x1100123a => {
  pc = 0x11002d81; continue;
 }
 0x11002d81 => {
  { let v: u32 = 0x11002dddu32; cpu.eax = v; }
  cpu.push(0x11002d8b); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x11002da0); f_11001118(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002db0); f_1100108c(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002dc4); f_11002dd5(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100123f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100123f); }
 let mut pc: u32 = 0x1100123f;
 loop { match pc {
 0x1100123f => {
  pc = 0x11002748; continue;
 }
 0x11002748 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1100275d); f_110010a5(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(v); }
  let t = cpu.r32(0x1100e230u32); cpu.push(0x11002767); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001244); }
 let mut pc: u32 = 0x11001244;
 loop { match pc {
 0x11001244 => {
  pc = 0x11003158; continue;
 }
 0x11003158 => {
  { cpu.and8(cpu.r8(0x1100d8ecu32), 0x1u8); }
  if !cpu.zf { pc = 0x1100317f; continue; }
  pc = 0x11003161; continue;
 }
 0x11003161 => {
  { let v: u8 = cpu.r8(0x1100d8ecu32) | 0x1u8; cpu.w8(0x1100d8ecu32, v); }
  { let v: u32 = 0x1100d8f0u32; cpu.ecx = v; }
  cpu.push(0x11003172); f_11001348(cpu);
  { let v = 0x11003190u32; cpu.push(v); }
  cpu.push(0x1100317c); f_11005f00(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1100317f; continue;
 }
 0x1100317f => {
  { let v: u32 = 0x1100d8f0u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001249(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001249); }
 let mut pc: u32 = 0x11001249;
 loop { match pc {
 0x11001249 => {
  pc = 0x11001c76; continue;
 }
 0x11001c76 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x11001cb3; continue; }
  pc = 0x11001c82; continue;
 }
 0x11001c82 => {
  cpu.push(0x11001c87); f_110010af(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11001c9c; continue; }
  pc = 0x11001c8b; continue;
 }
 0x11001c8b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x11001c90; continue;
 }
 0x11001c90 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); }
  if !cpu.zf { pc = 0x11001c90; continue; }
  pc = 0x11001c9a; continue;
 }
 0x11001c9a => {
  pc = 0x11001cb3; continue;
 }
 0x11001c9c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x11001ca2; continue;
 }
 0x11001ca2 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), cpu.eax); }
  if !cpu.zf { pc = 0x11001cac; continue; }
  pc = 0x11001ca7; continue;
 }
 0x11001ca7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11001cac; continue;
 }
 0x11001cac => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.ecx); }
  if !cpu.zf { pc = 0x11001ca2; continue; }
  pc = 0x11001cb3; continue;
 }
 0x11001cb3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100124e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100124e); }
 let mut pc: u32 = 0x1100124e;
 loop { match pc {
 0x1100124e => {
  pc = 0x110056ca; continue;
 }
 0x110056ca => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = 0xdcu32; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x110056da); f_11005de6(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x110056e4; continue;
 }
 0x110056e4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xdcu32); }
  if (cpu.sf != cpu.of) { pc = 0x110056e4; continue; }
  pc = 0x110056fb; continue;
 }
 0x110056fb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001253(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001253); }
 let mut pc: u32 = 0x11001253;
 loop { match pc {
 0x11001253 => {
  pc = 0x110022d5; continue;
 }
 0x110022d5 => {
  { let v: u32 = 0x11002319u32; cpu.eax = v; }
  cpu.push(0x110022df); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x110022f3); f_1100120d(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001258(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001258); }
 let mut pc: u32 = 0x11001258;
 loop { match pc {
 0x11001258 => {
  pc = 0x1100212c; continue;
 }
 0x1100212c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1100e234u32); cpu.push(0x11002149); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1100215b; continue; }
  pc = 0x11002153; continue;
 }
 0x11002153 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1100215b; continue;
 }
 0x1100215b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100125d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100125d); }
 let mut pc: u32 = 0x1100125d;
 loop { match pc {
 0x1100125d => {
  pc = 0x11004e84; continue;
 }
 0x11004e84 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001262(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001262); }
 let mut pc: u32 = 0x11001262;
 loop { match pc {
 0x11001262 => {
  pc = 0x11004c03; continue;
 }
 0x11004c03 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001267(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001267); }
 let mut pc: u32 = 0x11001267;
 loop { match pc {
 0x11001267 => {
  pc = 0x110029e5; continue;
 }
 0x110029e5 => {
  { let v: u32 = 0x11002a50u32; cpu.eax = v; }
  cpu.push(0x110029ef); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002a02); f_110012e4(cpu);
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002a12); f_110013cf(cpu);
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x11002a23); f_110011ea(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002a2f); f_11002a5a(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100126c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100126c); }
 let mut pc: u32 = 0x1100126c;
 loop { match pc {
 0x1100126c => {
  pc = 0x11004e98; continue;
 }
 0x11004e98 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x11004ea7; continue; }
  pc = 0x11004ea1; continue;
 }
 0x11004ea1 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11004ea7); f_11001064(cpu);
  pc = 0x11004ea7; continue;
 }
 0x11004ea7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x11004eb9); f_110011ef(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001271(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001271); }
 let mut pc: u32 = 0x11001271;
 loop { match pc {
 0x11001271 => {
  pc = 0x11004d89; continue;
 }
 0x11004d89 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x11004d9e); f_110011ef(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001276(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001276); }
 let mut pc: u32 = 0x11001276;
 loop { match pc {
 0x11001276 => {
  pc = 0x1100461a; continue;
 }
 0x1100461a => {
  { let v: u32 = 0x11004643u32; cpu.eax = v; }
  cpu.push(0x11004624); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004636); f_1100464d(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100127b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100127b); }
 let mut pc: u32 = 0x1100127b;
 loop { match pc {
 0x1100127b => {
  pc = 0x1100579b; continue;
 }
 0x1100579b => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001280); }
 let mut pc: u32 = 0x11001280;
 loop { match pc {
 0x11001280 => {
  pc = 0x11005572; continue;
 }
 0x11005572 => {
  { let v: u32 = 0x110055ddu32; cpu.eax = v; }
  cpu.push(0x1100557c); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x1100558f); f_11001393(cpu);
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x110055a0; continue;
 }
 0x110055a0 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x110055a8); f_1100105a(cpu);
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xdcu32); }
  if (cpu.sf != cpu.of) { pc = 0x110055a0; continue; }
  pc = 0x110055b9; continue;
 }
 0x110055b9 => {
  { let v: u32 = 0x36u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110055cc); f_110055e7(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001285(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001285); }
 let mut pc: u32 = 0x11001285;
 loop { match pc {
 0x11001285 => {
  pc = 0x110021b2; continue;
 }
 0x110021b2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1100e238u32); cpu.push(0x110021cc); cpu.call(t);
  { let v = cpu.st(0); cpu.wf64(0xfffffff4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x110021e1; continue; }
  pc = 0x110021d9; continue;
 }
 0x110021d9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x110021e1; continue;
 }
 0x110021e1 => {
  { let v = cpu.rf64(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100128a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100128a); }
 let mut pc: u32 = 0x1100128a;
 loop { match pc {
 0x1100128a => {
  pc = 0x11004c27; continue;
 }
 0x11004c27 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x11004c44); f_110010a0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x11004c4c); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100128f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100128f); }
 let mut pc: u32 = 0x1100128f;
 loop { match pc {
 0x1100128f => {
  pc = 0x11003a26; continue;
 }
 0x11003a26 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x11003a46; continue; }
  pc = 0x11003a31; continue;
 }
 0x11003a31 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x1100131bu32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11003a43); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x11003a55; continue;
 }
 0x11003a46 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11003a4d); f_1100131b(cpu);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11003a5d; continue; }
  pc = 0x11003a54; continue;
 }
 0x11003a54 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11003a55; continue;
 }
 0x11003a55 => {
  cpu.push(0x11003a5a); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11003a5d; continue;
 }
 0x11003a5d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001294(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001294); }
 let mut pc: u32 = 0x11001294;
 loop { match pc {
 0x11001294 => {
  pc = 0x11001dc2; continue;
 }
 0x11001dc2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if !cpu.zf { pc = 0x11001ddd; continue; }
  pc = 0x11001dd0; continue;
 }
 0x11001dd0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if cpu.zf { pc = 0x11001ddf; continue; }
  pc = 0x11001ddd; continue;
 }
 0x11001ddd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x11001ddf; continue;
 }
 0x11001ddf => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001299(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001299); }
 let mut pc: u32 = 0x11001299;
 loop { match pc {
 0x11001299 => {
  pc = 0x11001d62; continue;
 }
 0x11001d62 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x11001d74); f_1100137f(cpu);
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100129e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100129e); }
 let mut pc: u32 = 0x1100129e;
 loop { match pc {
 0x1100129e => {
  pc = 0x1100504b; continue;
 }
 0x1100504b => {
  { let v: u32 = 0x11005080u32; cpu.eax = v; }
  cpu.push(0x11005055); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11005066); f_1100139d(cpu);
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
pub fn f_110012a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012a3); }
 let mut pc: u32 = 0x110012a3;
 loop { match pc {
 0x110012a3 => {
  pc = 0x11004592; continue;
 }
 0x11004592 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1100459a); f_11001221(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x110045aa; continue; }
  pc = 0x110045a1; continue;
 }
 0x110045a1 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110045a7); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110045aa; continue;
 }
 0x110045aa => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012a8); }
 let mut pc: u32 = 0x110012a8;
 loop { match pc {
 0x110012a8 => {
  pc = 0x11002eaa; continue;
 }
 0x11002eaa => {
  { let v: u32 = 0x11002f05u32; cpu.eax = v; }
  cpu.push(0x11002eb4); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x11002ec8); f_110014bf(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11002eda); f_11001145(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002ee9); f_11002efd(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_110012b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012b2); }
 let mut pc: u32 = 0x110012b2;
 loop { match pc {
 0x110012b2 => {
  pc = 0x11002bc8; continue;
 }
 0x11002bc8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11002bd7); f_1100108c(cpu);
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012b7); }
 let mut pc: u32 = 0x110012b7;
 loop { match pc {
 0x110012b7 => {
  pc = 0x1100257a; continue;
 }
 0x1100257a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x110025e7; continue; }
  pc = 0x1100258b; continue;
 }
 0x1100258b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  if cpu.zf { pc = 0x110025e7; continue; }
  pc = 0x1100259b; continue;
 }
 0x1100259b => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110025a2); f_110010a5(cpu);
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x110025e7; continue; }
  pc = 0x110025a6; continue;
 }
 0x110025a6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x110025b0); f_110010a5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x110025dc; continue; }
  pc = 0x110025b4; continue;
 }
 0x110025b4 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110025bb); f_110010a5(cpu);
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x110025dc; continue; }
  pc = 0x110025bf; continue;
 }
 0x110025bf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x110025d8); f_110010a5(cpu);
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x110025b4; continue; }
  pc = 0x110025dc; continue;
 }
 0x110025dc => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1100259b; continue; }
  pc = 0x110025e7; continue;
 }
 0x110025e7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012bc); }
 let mut pc: u32 = 0x110012bc;
 loop { match pc {
 0x110012bc => {
  pc = 0x11003614; continue;
 }
 0x11003614 => {
  { let v: u32 = 0x1100367eu32; cpu.eax = v; }
  cpu.push(0x1100361e); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1100363b); f_110011ea(cpu);
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1100364e); f_110011ea(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
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
pub fn f_110012c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012c1); }
 let mut pc: u32 = 0x110012c1;
 loop { match pc {
 0x110012c1 => {
  pc = 0x11001b08; continue;
 }
 0x11001b08 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x1u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1100705cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x11001b21; continue; }
  pc = 0x11001b18; continue;
 }
 0x11001b18 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11001b1e); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11001b21; continue;
 }
 0x11001b21 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012c6); }
 let mut pc: u32 = 0x110012c6;
 loop { match pc {
 0x110012c6 => {
  pc = 0x110038b3; continue;
 }
 0x110038b3 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x110038d3; continue; }
  pc = 0x110038be; continue;
 }
 0x110038be => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x11001433u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110038d0); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x110038e2; continue;
 }
 0x110038d3 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110038da); f_11001433(cpu);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x110038ea; continue; }
  pc = 0x110038e1; continue;
 }
 0x110038e1 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x110038e2; continue;
 }
 0x110038e2 => {
  cpu.push(0x110038e7); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110038ea; continue;
 }
 0x110038ea => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012cb); }
 let mut pc: u32 = 0x110012cb;
 loop { match pc {
 0x110012cb => {
  pc = 0x11002610; continue;
 }
 0x11002610 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  let t = cpu.r32(0x1100e214u32); cpu.push(0x1100261c); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012d0); }
 let mut pc: u32 = 0x110012d0;
 loop { match pc {
 0x110012d0 => {
  pc = 0x110053bf; continue;
 }
 0x110053bf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x110053c4); cpu.call(t);
  { let v: u32 = 0x2710u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012da); }
 let mut pc: u32 = 0x110012da;
 loop { match pc {
 0x110012da => {
  pc = 0x11001a2d; continue;
 }
 0x11001a2d => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11001a3a); f_11001091(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012df); }
 let mut pc: u32 = 0x110012df;
 loop { match pc {
 0x110012df => {
  pc = 0x1100319c; continue;
 }
 0x1100319c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x110031bc; continue; }
  pc = 0x110031a4; continue;
 }
 0x110031a4 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x110031bc; continue; }
  pc = 0x110031aa; continue;
 }
 0x110031aa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x110031bc; continue; }
  pc = 0x110031b4; continue;
 }
 0x110031b4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x110031bc); cpu.call(t);
  pc = 0x110031bc; continue;
 }
 0x110031bc => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012e4); }
 let mut pc: u32 = 0x110012e4;
 loop { match pc {
 0x110012e4 => {
  pc = 0x11001f3f; continue;
 }
 0x11001f3f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x11007074u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11001f5a); f_11005de6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x11001f70; continue; }
  pc = 0x11001f65; continue;
 }
 0x11001f65 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.cf { pc = 0x11001f65; continue; }
  pc = 0x11001f70; continue;
 }
 0x11001f70 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012e9); }
 let mut pc: u32 = 0x110012e9;
 loop { match pc {
 0x110012e9 => {
  pc = 0x11001a07; continue;
 }
 0x11001a07 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x8u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x11001a20; continue; }
  pc = 0x11001a10; continue;
 }
 0x11001a10 => {
  cpu.push(0x11001a15); f_110010b4(cpu);
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11001a20); f_11001226(cpu);
  pc = 0x11001a20; continue;
 }
 0x11001a20 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012ee); }
 let mut pc: u32 = 0x110012ee;
 loop { match pc {
 0x110012ee => {
  pc = 0x1100391f; continue;
 }
 0x1100391f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012f3); }
 let mut pc: u32 = 0x110012f3;
 loop { match pc {
 0x110012f3 => {
  pc = 0x11002a81; continue;
 }
 0x11002a81 => {
  { let v: u32 = 0x11002aeeu32; cpu.eax = v; }
  cpu.push(0x11002a8b); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002a9f); f_110012e4(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11002ab5); f_11001230(cpu);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x11002ac1); f_110011ea(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002acd); f_11002af8(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012f8); }
 let mut pc: u32 = 0x110012f8;
 loop { match pc {
 0x110012f8 => {
  pc = 0x11001e78; continue;
 }
 0x11001e78 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110012fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110012fd); }
 let mut pc: u32 = 0x110012fd;
 loop { match pc {
 0x110012fd => {
  pc = 0x11002e1b; continue;
 }
 0x11002e1b => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11002e26); f_11001145(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001302(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001302); }
 let mut pc: u32 = 0x11001302;
 loop { match pc {
 0x11001302 => {
  pc = 0x11001e8e; continue;
 }
 0x11001e8e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  cpu.push(0x11001e9b); f_110010af(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11001eae; continue; }
  pc = 0x11001e9f; continue;
 }
 0x11001e9f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if !cpu.zf { pc = 0x11001eae; continue; }
  pc = 0x11001ea9; continue;
 }
 0x11001ea9 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x11001eae; continue;
 }
 0x11001eae => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001307(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001307); }
 let mut pc: u32 = 0x11001307;
 loop { match pc {
 0x11001307 => {
  pc = 0x110026ca; continue;
 }
 0x110026ca => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100130c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100130c); }
 let mut pc: u32 = 0x1100130c;
 loop { match pc {
 0x1100130c => {
  pc = 0x11002445; continue;
 }
 0x11002445 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x11002450); f_110010a5(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x11002460; continue; }
  pc = 0x1100245e; continue;
 }
 0x1100245e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x11002460; continue;
 }
 0x11002460 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x11002469); f_110010a5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11002471; continue; }
  pc = 0x1100246d; continue;
 }
 0x1100246d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x110024b3; continue;
 }
 0x11002471 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x1100247a); f_110010a5(cpu);
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.esi); cpu.eax = v; }
  if cpu.zf { pc = 0x110024a2; continue; }
  pc = 0x11002480; continue;
 }
 0x11002480 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x1100249a); f_110010a5(cpu);
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.cf && !cpu.zf) { pc = 0x11002480; continue; }
  pc = 0x110024a2; continue;
 }
 0x110024a2 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x110024a9); f_110010a5(cpu);
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x110024b3); f_1100104b(cpu);
  pc = 0x110024b3; continue;
 }
 0x110024b3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001311(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001311); }
 let mut pc: u32 = 0x11001311;
 loop { match pc {
 0x11001311 => {
  pc = 0x11005876; continue;
 }
 0x11005876 => {
  { let v: u32 = 0x110058b6u32; cpu.eax = v; }
  cpu.push(0x11005880); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x11007154u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
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
pub fn f_11001316(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001316); }
 let mut pc: u32 = 0x11001316;
 loop { match pc {
 0x11001316 => {
  pc = 0x11001bb4; continue;
 }
 0x11001bb4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11001bbc); f_11001325(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11001bcc; continue; }
  pc = 0x11001bc3; continue;
 }
 0x11001bc3 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11001bc9); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11001bcc; continue;
 }
 0x11001bcc => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100131b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100131b); }
 let mut pc: u32 = 0x1100131b;
 loop { match pc {
 0x1100131b => {
  pc = 0x11003308; continue;
 }
 0x11003308 => {
  { let v: u32 = 0x1100335cu32; cpu.eax = v; }
  cpu.push(0x11003312); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x11007094u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003329); f_11001244(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x11003330); f_11001127(cpu);
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x11003342; continue; }
  pc = 0x11003335; continue;
 }
 0x11003335 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1100333b); f_11001244(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x11003342); f_1100114f(cpu);
  pc = 0x11003342; continue;
 }
 0x11003342 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100334e); f_11003366(cpu);
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
pub fn f_11001320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001320); }
 let mut pc: u32 = 0x11001320;
 loop { match pc {
 0x11001320 => {
  pc = 0x11004ef6; continue;
 }
 0x11004ef6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x11004f17; continue; }
  pc = 0x11004eff; continue;
 }
 0x11004eff => {
  cpu.push(0x11004f04); f_1100110e(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11004f17; continue; }
  pc = 0x11004f08; continue;
 }
 0x11004f08 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.ecx); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if cpu.zf { pc = 0x11004f19; continue; }
  pc = 0x11004f17; continue;
 }
 0x11004f17 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x11004f19; continue;
 }
 0x11004f19 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001325(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001325); }
 let mut pc: u32 = 0x11001325;
 loop { match pc {
 0x11001325 => {
  pc = 0x11001d57; continue;
 }
 0x11001d57 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  cpu.push(0x11001d5f); f_1100102d(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100132a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100132a); }
 let mut pc: u32 = 0x1100132a;
 loop { match pc {
 0x1100132a => {
  pc = 0x110049ce; continue;
 }
 0x110049ce => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x110070e4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x110049ec; continue; }
  pc = 0x110049dc; continue;
 }
 0x110049dc => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x110049de; continue;
 }
 0x110049de => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  if !cpu.zf { pc = 0x110049de; continue; }
  pc = 0x110049ec; continue;
 }
 0x110049ec => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100132f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100132f); }
 let mut pc: u32 = 0x1100132f;
 loop { match pc {
 0x1100132f => {
  pc = 0x11001ee4; continue;
 }
 0x11001ee4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x11007074u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11001ef4); f_11005de6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001334(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001334); }
 let mut pc: u32 = 0x11001334;
 loop { match pc {
 0x11001334 => {
  pc = 0x11004504; continue;
 }
 0x11004504 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x1100450f); f_110011d1(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x110070e4u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001339(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001339); }
 let mut pc: u32 = 0x11001339;
 loop { match pc {
 0x11001339 => {
  pc = 0x11001bd9; continue;
 }
 0x11001bd9 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x11001bf9; continue; }
  pc = 0x11001be4; continue;
 }
 0x11001be4 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x11001325u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11001bf6); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x11001c08; continue;
 }
 0x11001bf9 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11001c00); f_11001325(cpu);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11001c10; continue; }
  pc = 0x11001c07; continue;
 }
 0x11001c07 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11001c08; continue;
 }
 0x11001c08 => {
  cpu.push(0x11001c0d); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11001c10; continue;
 }
 0x11001c10 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100133e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100133e); }
 let mut pc: u32 = 0x1100133e;
 loop { match pc {
 0x1100133e => {
  pc = 0x1100487d; continue;
 }
 0x1100487d => {
  { let v: u32 = 0x110048d4u32; cpu.eax = v; }
  cpu.push(0x11004887); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x11004892); f_11005de6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x110048bc; continue; }
  pc = 0x110048aa; continue;
 }
 0x110048aa => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110048b1); f_110011d1(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x110048bc; continue;
 }
 0x110048bc => {
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
pub fn f_11001343(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001343); }
 let mut pc: u32 = 0x11001343;
 loop { match pc {
 0x11001343 => {
  pc = 0x110053eb; continue;
 }
 0x110053eb => {
  { let v: u32 = 0x1100545du32; cpu.eax = v; }
  cpu.push(0x110053f5); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x11007154u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v = 0xdcu32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11007184u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1100542e); f_11005de6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1100543e); f_11001280(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_11001348(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001348); }
 let mut pc: u32 = 0x11001348;
 loop { match pc {
 0x11001348 => {
  pc = 0x11003142; continue;
 }
 0x11003142 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100134d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100134d); }
 let mut pc: u32 = 0x1100134d;
 loop { match pc {
 0x1100134d => {
  pc = 0x11002b30; continue;
 }
 0x11002b30 => {
  { let v: u32 = 0x11002b98u32; cpu.eax = v; }
  cpu.push(0x11002b3a); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002b4e); f_110011ea(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002b62); f_11001145(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x11002b6b); f_110011ea(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002b77); f_11002ba2(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001352(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001352); }
 let mut pc: u32 = 0x11001352;
 loop { match pc {
 0x11001352 => {
  pc = 0x11003e6e; continue;
 }
 0x11003e6e => {
  { let v: u32 = 0x11003eecu32; cpu.eax = v; }
  cpu.push(0x11003e78); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11003e88); f_110014d3(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x110070b4u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x11003ea7); f_1100148d(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003eb6); f_110011a9(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001357(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001357); }
 let mut pc: u32 = 0x11001357;
 loop { match pc {
 0x11001357 => {
  pc = 0x11001e44; continue;
 }
 0x11001e44 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11001e54); f_11001091(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100135c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100135c); }
 let mut pc: u32 = 0x1100135c;
 loop { match pc {
 0x1100135c => {
  pc = 0x11003c0b; continue;
 }
 0x11003c0b => {
  { let v: u32 = 0x11003c68u32; cpu.eax = v; }
  cpu.push(0x11003c15); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x11003c37; continue; }
  pc = 0x11003c22; continue;
 }
 0x11003c22 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x11001032u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x11003c34); f_11005cc0(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11003c4c; continue;
 }
 0x11003c37 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003c43); f_11003c72(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11003c54; continue; }
  pc = 0x11003c49; continue;
 }
 0x11003c49 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  pc = 0x11003c4c; continue;
 }
 0x11003c4c => {
  cpu.push(0x11003c51); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11003c54; continue;
 }
 0x11003c54 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_11001361(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001361); }
 let mut pc: u32 = 0x11001361;
 loop { match pc {
 0x11001361 => {
  pc = 0x11004136; continue;
 }
 0x11004136 => {
  { let v: u32 = 0x110041afu32; cpu.eax = v; }
  cpu.push(0x11004140); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if cpu.zf { pc = 0x1100419a; continue; }
  pc = 0x1100414f; continue;
 }
 0x1100414f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v = 0x1cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x1100416e); f_11005de6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x11004190; continue; }
  pc = 0x11004186; continue;
 }
 0x11004186 => {
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x11004190); f_11001352(cpu);
  pc = 0x11004190; continue;
 }
 0x11004190 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1100419a; continue;
 }
 0x1100419a => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001366(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001366); }
 let mut pc: u32 = 0x11001366;
 loop { match pc {
 0x11001366 => {
  pc = 0x11002e2c; continue;
 }
 0x11002e2c => {
  { let v: u32 = 0x11002e87u32; cpu.eax = v; }
  cpu.push(0x11002e36); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x11002e4a); f_110011d6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11002e5c); f_11001145(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002e6b); f_11002e7f(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_1100136b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100136b); }
 let mut pc: u32 = 0x1100136b;
 loop { match pc {
 0x1100136b => {
  pc = 0x11004e6e; continue;
 }
 0x11004e6e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001375(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001375); }
 let mut pc: u32 = 0x11001375;
 loop { match pc {
 0x11001375 => {
  pc = 0x11003924; continue;
 }
 0x11003924 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x1u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11007084u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x1100393d; continue; }
  pc = 0x11003934; continue;
 }
 0x11003934 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1100393a); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1100393d; continue;
 }
 0x1100393d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100137a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100137a); }
 let mut pc: u32 = 0x1100137a;
 loop { match pc {
 0x1100137a => {
  pc = 0x110050de; continue;
 }
 0x110050de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x110050f0; continue; }
  pc = 0x110050e4; continue;
 }
 0x110050e4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x110050f0); f_1100128a(cpu);
  pc = 0x110050f0; continue;
 }
 0x110050f0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100137f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100137f); }
 let mut pc: u32 = 0x1100137f;
 loop { match pc {
 0x1100137f => {
  pc = 0x11001cc8; continue;
 }
 0x11001cc8 => {
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11001cdf; continue; }
  pc = 0x11001cd5; continue;
 }
 0x11001cd5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x11001ce4; continue;
 }
 0x11001cdf => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x11001ce4; continue;
 }
 0x11001ce4 => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001384(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001384); }
 let mut pc: u32 = 0x11001384;
 loop { match pc {
 0x11001384 => {
  pc = 0x110019d9; continue;
 }
 0x110019d9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110019e7); f_1100103c(cpu);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x1100705cu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001389(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001389); }
 let mut pc: u32 = 0x11001389;
 loop { match pc {
 0x11001389 => {
  pc = 0x11001e5b; continue;
 }
 0x11001e5b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x11001e67); f_11001442(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100138e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100138e); }
 let mut pc: u32 = 0x1100138e;
 loop { match pc {
 0x1100138e => {
  pc = 0x11002d02; continue;
 }
 0x11002d02 => {
  { let v: u32 = 0x11002d5eu32; cpu.eax = v; }
  cpu.push(0x11002d0c); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x11002d24); f_11001118(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002d30); f_1100108c(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.esi, cpu.eax); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002d44); f_11002d56(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_11001393(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001393); }
 let mut pc: u32 = 0x11001393;
 loop { match pc {
 0x11001393 => {
  pc = 0x11005306; continue;
 }
 0x11005306 => {
  { let v: u32 = 0x11005366u32; cpu.eax = v; }
  cpu.push(0x11005310); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x11007154u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0x1100713cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x11005347); f_11001195(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_11001398(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001398); }
 let mut pc: u32 = 0x11001398;
 loop { match pc {
 0x11001398 => {
  pc = 0x1100569c; continue;
 }
 0x1100569c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x8u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x110056bb; continue; }
  pc = 0x110056a5; continue;
 }
 0x110056a5 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(v); }
  cpu.push(0x110056ad); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x110056bb); f_1100124e(cpu);
  pc = 0x110056bb; continue;
 }
 0x110056bb => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100139d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100139d); }
 let mut pc: u32 = 0x1100139d;
 loop { match pc {
 0x1100139d => {
  pc = 0x11004dbc; continue;
 }
 0x11004dbc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x11004dd2; continue; }
  pc = 0x11004dcc; continue;
 }
 0x11004dcc => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x11004dd2); f_110011ef(cpu);
  pc = 0x11004dd2; continue;
 }
 0x11004dd2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013a2); }
 let mut pc: u32 = 0x110013a2;
 loop { match pc {
 0x110013a2 => {
  pc = 0x11001fcc; continue;
 }
 0x11001fcc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.zf { pc = 0x11001ff1; continue; }
  pc = 0x11001fd8; continue;
 }
 0x11001fd8 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.push(v); }
  cpu.push(0x11001fe0); f_11005cb6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(v); }
  cpu.push(0x11001feb); f_1100146a(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x11001ff1; continue;
 }
 0x11001ff1 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013a7); }
 let mut pc: u32 = 0x110013a7;
 loop { match pc {
 0x110013a7 => {
  pc = 0x11004de7; continue;
 }
 0x11004de7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.zf { pc = 0x11004e17; continue; }
  pc = 0x11004df3; continue;
 }
 0x11004df3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.ecx); }
  if cpu.zf { pc = 0x11004e11; continue; }
  pc = 0x11004df9; continue;
 }
 0x11004df9 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x11004e03; continue; }
  pc = 0x11004dfd; continue;
 }
 0x11004dfd => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x11004e03); f_11001064(cpu);
  pc = 0x11004e03; continue;
 }
 0x11004e03 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x11004e11; continue; }
  pc = 0x11004e0b; continue;
 }
 0x11004e0b => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x11004e11); f_110011ef(cpu);
  pc = 0x11004e11; continue;
 }
 0x11004e11 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x11004e17; continue;
 }
 0x11004e17 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013ac); }
 let mut pc: u32 = 0x110013ac;
 loop { match pc {
 0x110013ac => {
  pc = 0x11003907; continue;
 }
 0x11003907 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x11007084u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013b1); }
 let mut pc: u32 = 0x110013b1;
 loop { match pc {
 0x110013b1 => {
  pc = 0x11002c7d; continue;
 }
 0x11002c7d => {
  { let v: u32 = 0x11002cdeu32; cpu.eax = v; }
  cpu.push(0x11002c87); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x11002c9f); f_11001118(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002cab); f_1100108c(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.esi, cpu.eax); }
  { let v: u32 = cpu.adc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xffffffffu32); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11002cc4); f_11002cd6(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_110013b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013b6); }
 let mut pc: u32 = 0x110013b6;
 loop { match pc {
 0x110013b6 => {
  pc = 0x110057c6; continue;
 }
 0x110057c6 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x110057e6; continue; }
  pc = 0x110057d1; continue;
 }
 0x110057d1 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x110010aau32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110057e3); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x110057f4; continue;
 }
 0x110057e6 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x110057fc; continue; }
  pc = 0x110057f3; continue;
 }
 0x110057f3 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x110057f4; continue;
 }
 0x110057f4 => {
  cpu.push(0x110057f9); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110057fc; continue;
 }
 0x110057fc => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013bb); }
 let mut pc: u32 = 0x110013bb;
 loop { match pc {
 0x110013bb => {
  pc = 0x11001e1b; continue;
 }
 0x11001e1b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013c0); }
 let mut pc: u32 = 0x110013c0;
 loop { match pc {
 0x110013c0 => {
  pc = 0x11001de7; continue;
 }
 0x11001de7 => {
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11001df4); f_11001294(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013c5); }
 let mut pc: u32 = 0x110013c5;
 loop { match pc {
 0x110013c5 => {
  pc = 0x110058dc; continue;
 }
 0x110058dc => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013ca); }
 let mut pc: u32 = 0x110013ca;
 loop { match pc {
 0x110013ca => {
  pc = 0x11004b22; continue;
 }
 0x11004b22 => {
  { let v: u32 = 0x11004b7cu32; cpu.eax = v; }
  cpu.push(0x11004b2c); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x11004b3b); f_11001271(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x11004b4a); f_1100145b(cpu);
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11004b58); f_1100116d(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004b64); f_11004b74(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013cf); }
 let mut pc: u32 = 0x110013cf;
 loop { match pc {
 0x110013cf => {
  pc = 0x110026da; continue;
 }
 0x110026da => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013d4); }
 let mut pc: u32 = 0x110013d4;
 loop { match pc {
 0x110013d4 => {
  pc = 0x11001e2f; continue;
 }
 0x11001e2f => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  cpu.push(0x11001e3d); f_11001091(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013d9); }
 let mut pc: u32 = 0x110013d9;
 loop { match pc {
 0x110013d9 => {
  pc = 0x110050f5; continue;
 }
 0x110050f5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x11005108; continue; }
  pc = 0x110050fb; continue;
 }
 0x110050fb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x11005108); f_1100128a(cpu);
  pc = 0x11005108; continue;
 }
 0x11005108 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013de); }
 let mut pc: u32 = 0x110013de;
 loop { match pc {
 0x110013de => {
  pc = 0x11004520; continue;
 }
 0x11004520 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11004528); f_1100132a(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11004538; continue; }
  pc = 0x1100452f; continue;
 }
 0x1100452f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11004535); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11004538; continue;
 }
 0x11004538 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013e3); }
 let mut pc: u32 = 0x110013e3;
 loop { match pc {
 0x110013e3 => {
  pc = 0x11001ebc; continue;
 }
 0x11001ebc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013e8); }
 let mut pc: u32 = 0x110013e8;
 loop { match pc {
 0x110013e8 => {
  pc = 0x110036b2; continue;
 }
 0x110036b2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x110036ca); f_110013a2(cpu);
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x110036d6); f_110013a2(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013ed); }
 let mut pc: u32 = 0x110013ed;
 loop { match pc {
 0x110013ed => {
  pc = 0x110046c9; continue;
 }
 0x110046c9 => {
  { let v: u32 = 0x11004726u32; cpu.eax = v; }
  cpu.push(0x110046d3); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x110046f5; continue; }
  pc = 0x110046e0; continue;
 }
 0x110046e0 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x11001276u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x110046f2); f_11005cc0(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x1100470a; continue;
 }
 0x110046f5 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004701); f_11004730(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11004712; continue; }
  pc = 0x11004707; continue;
 }
 0x11004707 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  pc = 0x1100470a; continue;
 }
 0x1100470a => {
  cpu.push(0x1100470f); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11004712; continue;
 }
 0x11004712 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_110013f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013f2); }
 let mut pc: u32 = 0x110013f2;
 loop { match pc {
 0x110013f2 => {
  pc = 0x11005c2b; continue;
 }
 0x11005c2b => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x11005c4b; continue; }
  pc = 0x11005c36; continue;
 }
 0x11005c36 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x110010beu32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11005c48); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x11005c5a; continue;
 }
 0x11005c4b => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11005c52); f_110010be(cpu);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11005c62; continue; }
  pc = 0x11005c59; continue;
 }
 0x11005c59 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11005c5a; continue;
 }
 0x11005c5a => {
  cpu.push(0x11005c5f); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11005c62; continue;
 }
 0x11005c62 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013f7); }
 let mut pc: u32 = 0x110013f7;
 loop { match pc {
 0x110013f7 => {
  pc = 0x1100394a; continue;
 }
 0x1100394a => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x1100396a; continue; }
  pc = 0x11003955; continue;
 }
 0x11003955 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x11001113u32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11003967); f_11005cc0(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x11003978; continue;
 }
 0x1100396a => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x1u8); }
  { let v: u32 = 0x11007084u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x11003980; continue; }
  pc = 0x11003977; continue;
 }
 0x11003977 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x11003978; continue;
 }
 0x11003978 => {
  cpu.push(0x1100397d); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11003980; continue;
 }
 0x11003980 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110013fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110013fc); }
 let mut pc: u32 = 0x110013fc;
 loop { match pc {
 0x110013fc => {
  pc = 0x110041e8; continue;
 }
 0x110041e8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001401(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001401); }
 let mut pc: u32 = 0x11001401;
 loop { match pc {
 0x11001401 => {
  pc = 0x11005b0c; continue;
 }
 0x11005b0c => {
  { let v: u32 = 0x11005b4cu32; cpu.eax = v; }
  cpu.push(0x11005b16); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11005b28); f_11005b56(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11005b39; continue; }
  pc = 0x11005b2e; continue;
 }
 0x11005b2e => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11005b36); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11005b39; continue;
 }
 0x11005b39 => {
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
pub fn f_11001406(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001406); }
 let mut pc: u32 = 0x11001406;
 loop { match pc {
 0x11001406 => {
  pc = 0x11004f70; continue;
 }
 0x11004f70 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x11004f83; continue; }
  pc = 0x11004f76; continue;
 }
 0x11004f76 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), cpu.eax); }
  if cpu.zf { pc = 0x11004f83; continue; }
  pc = 0x11004f7e; continue;
 }
 0x11004f7e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11004f83; continue;
 }
 0x11004f83 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100140b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100140b); }
 let mut pc: u32 = 0x1100140b;
 loop { match pc {
 0x1100140b => {
  pc = 0x110031cc; continue;
 }
 0x110031cc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if (cpu.sf != cpu.of) { pc = 0x110031dd; continue; }
  pc = 0x110031d7; continue;
 }
 0x110031d7 => {
  { let v: u32 = 0x3u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x110031dd; continue;
 }
 0x110031dd => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001410); }
 let mut pc: u32 = 0x11001410;
 loop { match pc {
 0x11001410 => {
  pc = 0x11002faa; continue;
 }
 0x11002faa => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11002fb2); f_11001087(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11002fc2; continue; }
  pc = 0x11002fb9; continue;
 }
 0x11002fb9 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11002fbf); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11002fc2; continue;
 }
 0x11002fc2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001415(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001415); }
 let mut pc: u32 = 0x11001415;
 loop { match pc {
 0x11001415 => {
  pc = 0x11001d36; continue;
 }
 0x11001d36 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x11001d4b); f_1100137f(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100141a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100141a); }
 let mut pc: u32 = 0x1100141a;
 loop { match pc {
 0x1100141a => {
  pc = 0x11004b9f; continue;
 }
 0x11004b9f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x11004baf); f_1100125d(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004bea; continue; }
  pc = 0x11004bb3; continue;
 }
 0x11004bb3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11004bbf); f_1100125d(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004bea; continue; }
  pc = 0x11004bc3; continue;
 }
 0x11004bc3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11004bea; continue;
 }
 0x11004bea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100141f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100141f); }
 let mut pc: u32 = 0x1100141f;
 loop { match pc {
 0x1100141f => {
  pc = 0x11003f4a; continue;
 }
 0x11003f4a => {
  { let v: u32 = 0x11003fc1u32; cpu.eax = v; }
  cpu.push(0x11003f54); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x110070ccu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = 0x1cu32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x11003f7d); f_11005de6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x11003f98; continue; }
  pc = 0x11003f91; continue;
 }
 0x11003f91 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x11003f98); f_110014ce(cpu);
  pc = 0x11003f98; continue;
 }
 0x11003f98 => {
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x11003fac); f_1100143d(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001424(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001424); }
 let mut pc: u32 = 0x11001424;
 loop { match pc {
 0x11001424 => {
  pc = 0x11004a4a; continue;
 }
 0x11004a4a => {
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  cpu.push(0x11004a52); f_1100128a(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001429(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001429); }
 let mut pc: u32 = 0x11001429;
 loop { match pc {
 0x11001429 => {
  pc = 0x11005a1a; continue;
 }
 0x11005a1a => {
  { let v: u32 = 0x11005a79u32; cpu.eax = v; }
  cpu.push(0x11005a24); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x11007154u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1100713cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_1100142e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100142e); }
 let mut pc: u32 = 0x1100142e;
 loop { match pc {
 0x1100142e => {
  pc = 0x110023a7; continue;
 }
 0x110023a7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if !cpu.zf { pc = 0x110023bc; continue; }
  pc = 0x110023b8; continue;
 }
 0x110023b8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x1100241d; continue;
 }
 0x110023bc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x110023c4); f_110010a5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x110023cc; continue; }
  pc = 0x110023c8; continue;
 }
 0x110023c8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x1100241d; continue;
 }
 0x110023cc => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110023d3); f_110010a5(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x110023dd); f_110010a5(cpu);
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x110023ec); f_1100104b(cpu);
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x1100240d; continue; }
  pc = 0x110023fa; continue;
 }
 0x110023fa => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  if cpu.cf { pc = 0x110023fa; continue; }
  pc = 0x1100240d; continue;
 }
 0x1100240d => {
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1100241d); f_110012b7(cpu);
  pc = 0x1100241d; continue;
 }
 0x1100241d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001433(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001433); }
 let mut pc: u32 = 0x11001433;
 loop { match pc {
 0x11001433 => {
  pc = 0x11003157; continue;
 }
 0x11003157 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001438(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001438); }
 let mut pc: u32 = 0x11001438;
 loop { match pc {
 0x11001438 => {
  pc = 0x11004663; continue;
 }
 0x11004663 => {
  { let v: u32 = 0x110046a3u32; cpu.eax = v; }
  cpu.push(0x1100466d); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100467f); f_110046ad(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11004690; continue; }
  pc = 0x11004685; continue;
 }
 0x11004685 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1100468d); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11004690; continue;
 }
 0x11004690 => {
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
pub fn f_1100143d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100143d); }
 let mut pc: u32 = 0x1100143d;
 loop { match pc {
 0x1100143d => {
  pc = 0x110041ec; continue;
 }
 0x110041ec => {
  { let v: u32 = 0x1100438cu32; cpu.eax = v; }
  cpu.push(0x110041f6); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x8u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x110043f9; continue; }
  pc = 0x1100420a; continue;
 }
 0x1100420a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x110043a2; continue; }
  pc = 0x11004226; continue;
 }
 0x11004226 => {
  if (!cpu.cf && !cpu.zf) { pc = 0x110042ac; continue; }
  pc = 0x1100422c; continue;
 }
 0x1100422c => {
  if !cpu.cf { pc = 0x1100427b; continue; }
  pc = 0x1100422e; continue;
 }
 0x1100422e => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x11004231; continue;
 }
 0x11004231 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.edi); }
  if cpu.zf { pc = 0x1100424a; continue; }
  pc = 0x11004236; continue;
 }
 0x11004236 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(v); }
  pc = 0x11004261; continue;
 }
 0x1100424a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.div32(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(v); }
  pc = 0x11004261; continue;
 }
 0x11004261 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x11004268); cpu.call(t);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  cpu.push(0x1100426f); f_11001050(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if cpu.cf { pc = 0x11004231; continue; }
  pc = 0x1100427b; continue;
 }
 0x1100427b => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x11004373; continue; }
  pc = 0x11004285; continue;
 }
 0x11004285 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x110042a7); cpu.call(t);
  pc = 0x11004373; continue;
 }
 0x110042ac => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x11004354; continue; }
  pc = 0x110042b5; continue;
 }
 0x110042b5 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x110042b7; continue;
 }
 0x110042b7 => {
  { let v = 0xcu32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  cpu.push(0x110042c3); f_11005de6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x110042dd; continue; }
  pc = 0x110042d0; continue;
 }
 0x110042d0 => {
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x110042db); f_110010fa(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x110042dd; continue;
 }
 0x110042dd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110042f1); f_11005de6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x11004311; continue; }
  pc = 0x11004302; continue;
 }
 0x11004302 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x11004309); f_110011d1(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x11004313; continue;
 }
 0x11004311 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x11004313; continue;
 }
 0x11004313 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004323); f_110011ae(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x11004348; continue; }
  pc = 0x1100432f; continue;
 }
 0x1100432f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(v); }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x11004348); cpu.call(t);
  pc = 0x11004348; continue;
 }
 0x11004348 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.cf { pc = 0x110042b7; continue; }
  pc = 0x11004354; continue;
 }
 0x11004354 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.div32(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(v); }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x11004373); cpu.call(t);
  pc = 0x11004373; continue;
 }
 0x11004373 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  cpu.push(0x1100437e); f_1100144c(cpu);
  pc = 0x110043f3; continue;
 }
 0x110043a2 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x8u32.wrapping_add(cpu.ebp))); }
  if cpu.cf { pc = 0x110043d0; continue; }
  pc = 0x110043a7; continue;
 }
 0x110043a7 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x110043f3; continue; }
  pc = 0x110043ab; continue;
 }
 0x110043ab => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.push(0x110043ce); cpu.call(t);
  pc = 0x110043f3; continue;
 }
 0x110043d0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(v); }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x110043f3); cpu.call(t);
  pc = 0x110043f3; continue;
 }
 0x110043f3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x110043f9; continue;
 }
 0x110043f9 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001442(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001442); }
 let mut pc: u32 = 0x11001442;
 loop { match pc {
 0x11001442 => {
  pc = 0x11001b7a; continue;
 }
 0x11001b7a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x11001b94; continue; }
  pc = 0x11001b91; continue;
 }
 0x11001b91 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x11001b94; continue;
 }
 0x11001b94 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11001b9c); f_11001249(cpu);
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x11001ba4); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100144c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100144c); }
 let mut pc: u32 = 0x1100144c;
 loop { match pc {
 0x1100144c => {
  pc = 0x11004ed6; continue;
 }
 0x11004ed6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004ee2; continue; }
  pc = 0x11004edc; continue;
 }
 0x11004edc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11004ee2; continue;
 }
 0x11004ee2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001451(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001451); }
 let mut pc: u32 = 0x11001451;
 loop { match pc {
 0x11001451 => {
  pc = 0x11002e00; continue;
 }
 0x11002e00 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  let t = cpu.r32(0x1100e1dcu32); cpu.push(0x11002e11); cpu.call(t);
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001456(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001456); }
 let mut pc: u32 = 0x11001456;
 loop { match pc {
 0x11001456 => {
  pc = 0x11003ffa; continue;
 }
 0x11003ffa => {
  { let v: u32 = 0x11004054u32; cpu.eax = v; }
  cpu.push(0x11004004); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x110070ccu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if cpu.zf { pc = 0x1100403c; continue; }
  pc = 0x1100401a; continue;
 }
 0x1100401a => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004025); f_11004049(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004031); f_1100405e(cpu);
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11004039); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1100403c; continue;
 }
 0x1100403c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100145b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100145b); }
 let mut pc: u32 = 0x1100145b;
 loop { match pc {
 0x1100145b => {
  pc = 0x11004ee6; continue;
 }
 0x11004ee6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11004ef2; continue; }
  pc = 0x11004eec; continue;
 }
 0x11004eec => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11004ef2; continue;
 }
 0x11004ef2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001460); }
 let mut pc: u32 = 0x11001460;
 loop { match pc {
 0x11001460 => {
  pc = 0x110051f1; continue;
 }
 0x110051f1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1100520a); f_11005de6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1100521b; continue; }
  pc = 0x11005213; continue;
 }
 0x11005213 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { cpu.stos(4, true); }
  pc = 0x1100521b; continue;
 }
 0x1100521b => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1100523c; continue; }
  pc = 0x11005221; continue;
 }
 0x11005221 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1100522f; continue;
 }
 0x1100522f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(0x11005236); cpu.call(t);
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x1100522f; continue; }
  pc = 0x1100523c; continue;
 }
 0x1100523c => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x11005253; continue; }
  pc = 0x11005242; continue;
 }
 0x11005242 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  pc = 0x11005246; continue;
 }
 0x11005246 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x11005246; continue; }
  pc = 0x11005253; continue;
 }
 0x11005253 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x11005259); f_11005cb6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.div32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = (cpu.r64(0xfffffff8u32.wrapping_add(cpu.ebp)) as i64 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001465(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001465); }
 let mut pc: u32 = 0x11001465;
 loop { match pc {
 0x11001465 => {
  pc = 0x110050a3; continue;
 }
 0x110050a3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x110050b9; continue; }
  pc = 0x110050a9; continue;
 }
 0x110050a9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x110050b9); f_11001262(cpu);
  pc = 0x110050b9; continue;
 }
 0x110050b9 => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100146a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100146a); }
 let mut pc: u32 = 0x1100146a;
 loop { match pc {
 0x1100146a => {
  pc = 0x1100279e; continue;
 }
 0x1100279e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110027aa); f_11005dec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x110027b8); f_11005de6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x110027cc; continue; }
  pc = 0x110027c1; continue;
 }
 0x110027c1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.edi); }
  if cpu.cf { pc = 0x110027c1; continue; }
  pc = 0x110027cc; continue;
 }
 0x110027cc => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100146f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100146f); }
 let mut pc: u32 = 0x1100146f;
 loop { match pc {
 0x1100146f => {
  pc = 0x11001ed0; continue;
 }
 0x11001ed0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), cpu.eax); }
  if cpu.zf { pc = 0x11001edf; continue; }
  pc = 0x11001eda; continue;
 }
 0x11001eda => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x11001edf; continue;
 }
 0x11001edf => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001474); }
 let mut pc: u32 = 0x11001474;
 loop { match pc {
 0x11001474 => {
  pc = 0x11001a41; continue;
 }
 0x11001a41 => {
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11001a4d); f_11001091(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001479(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001479); }
 let mut pc: u32 = 0x11001479;
 loop { match pc {
 0x11001479 => {
  pc = 0x11001e6b; continue;
 }
 0x11001e6b => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  cpu.push(0x11001e75); f_11001442(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100147e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100147e); }
 let mut pc: u32 = 0x1100147e;
 loop { match pc {
 0x1100147e => {
  pc = 0x11003398; continue;
 }
 0x11003398 => {
  { let v: u32 = 0x110034ebu32; cpu.eax = v; }
  cpu.push(0x110033a2); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x110033bd); f_110011ea(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  cpu.push(0x110033cf); f_11001019(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1100348e; continue; }
  pc = 0x110033d7; continue;
 }
 0x110033d7 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = 0x1100d5f8u32; cpu.push(v); }
  cpu.push(0x110033e6); f_11001118(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x110033fb); f_11001145(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11003409); f_110011d6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11003418); f_11001145(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003424); f_110034bc(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = 0x1100d5ecu32; cpu.push(v); }
  cpu.push(0x11003433); f_11001118(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11003442); f_11001145(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1100d5e8u32; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x11003453); f_11001118(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11003469); f_11001145(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x11003473); f_11001145(cpu);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100347c); f_110034d4(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003485); f_110034cc(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100348e); f_110034c4(cpu);
  pc = 0x1100348e; continue;
 }
 0x1100348e => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1100349a); f_110011ea(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110034a7); f_110034f5(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001483(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001483); }
 let mut pc: u32 = 0x11001483;
 loop { match pc {
 0x11001483 => {
  pc = 0x1100510e; continue;
 }
 0x1100510e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1100511e; continue; }
  pc = 0x11005114; continue;
 }
 0x11005114 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x1100511e); f_1100128a(cpu);
  pc = 0x1100511e; continue;
 }
 0x1100511e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001488(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001488); }
 let mut pc: u32 = 0x11001488;
 loop { match pc {
 0x11001488 => {
  pc = 0x11001d88; continue;
 }
 0x11001d88 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x11001db0; continue; }
  pc = 0x11001d94; continue;
 }
 0x11001d94 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.ecx); }
  if cpu.zf { pc = 0x11001daa; continue; }
  pc = 0x11001d9a; continue;
 }
 0x11001d9a => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x11001da0); f_1100102d(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x11001daa); f_1100137f(cpu);
  pc = 0x11001daa; continue;
 }
 0x11001daa => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x11001db0; continue;
 }
 0x11001db0 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100148d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100148d); }
 let mut pc: u32 = 0x1100148d;
 loop { match pc {
 0x1100148d => {
  pc = 0x11004c5b; continue;
 }
 0x11004c5b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if cpu.zf { pc = 0x11004c8f; continue; }
  pc = 0x11004c71; continue;
 }
 0x11004c71 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  pc = 0x11004c76; continue;
 }
 0x11004c76 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  let t = cpu.ebp; cpu.push(0x11004c7b); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x11004c83); f_110011ae(cpu);
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if !cpu.zf { pc = 0x11004c76; continue; }
  pc = 0x11004c8f; continue;
 }
 0x11004c8f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001492(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001492); }
 let mut pc: u32 = 0x11001492;
 loop { match pc {
 0x11001492 => {
  pc = 0x1100388e; continue;
 }
 0x1100388e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11003896); f_11001433(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x110038a6; continue; }
  pc = 0x1100389d; continue;
 }
 0x1100389d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110038a3); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110038a6; continue;
 }
 0x110038a6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11001497(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11001497); }
 let mut pc: u32 = 0x11001497;
 loop { match pc {
 0x11001497 => {
  pc = 0x11004905; continue;
 }
 0x11004905 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x1100490e); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100149c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100149c); }
 let mut pc: u32 = 0x1100149c;
 loop { match pc {
 0x1100149c => {
  pc = 0x1100592a; continue;
 }
 0x1100592a => {
  { let v: u32 = 0x1100596au32; cpu.eax = v; }
  cpu.push(0x11005934); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11005946); f_11005974(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11005957; continue; }
  pc = 0x1100594c; continue;
 }
 0x1100594c => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11005954); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11005957; continue;
 }
 0x11005957 => {
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
pub fn f_110014a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014a1); }
 let mut pc: u32 = 0x110014a1;
 loop { match pc {
 0x110014a1 => {
  pc = 0x110031c9; continue;
 }
 0x110031c9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014a6); }
 let mut pc: u32 = 0x110014a6;
 loop { match pc {
 0x110014a6 => {
  pc = 0x11004081; continue;
 }
 0x11004081 => {
  { let v: u32 = 0x110040fcu32; cpu.eax = v; }
  cpu.push(0x1100408b); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x110070ccu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v = 0x1cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x110040bb); f_11005de6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x110040dd; continue; }
  pc = 0x110040d3; continue;
 }
 0x110040d3 => {
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x110040dd); f_11001352(cpu);
  pc = 0x110040dd; continue;
 }
 0x110040dd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014ab); }
 let mut pc: u32 = 0x110014ab;
 loop { match pc {
 0x110014ab => {
  pc = 0x11003b57; continue;
 }
 0x11003b57 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014b0); }
 let mut pc: u32 = 0x110014b0;
 loop { match pc {
 0x110014b0 => {
  pc = 0x110021f9; continue;
 }
 0x110021f9 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.push(v); }
  cpu.push(0x11002201); f_1100146a(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014b5); }
 let mut pc: u32 = 0x110014b5;
 loop { match pc {
 0x110014b5 => {
  pc = 0x11004f52; continue;
 }
 0x11004f52 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11004f5c; continue; }
  pc = 0x11004f58; continue;
 }
 0x11004f58 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x11004f69; continue;
 }
 0x11004f5c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x11004f69; continue;
 }
 0x11004f69 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014ba); }
 let mut pc: u32 = 0x110014ba;
 loop { match pc {
 0x110014ba => {
  pc = 0x11002003; continue;
 }
 0x11002003 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), cpu.eax); }
  if cpu.zf { pc = 0x11002027; continue; }
  pc = 0x1100200f; continue;
 }
 0x1100200f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11002015); f_11005cb6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x11002021); f_1100146a(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x11002027; continue;
 }
 0x11002027 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014bf); }
 let mut pc: u32 = 0x110014bf;
 loop { match pc {
 0x110014bf => {
  pc = 0x1100288b; continue;
 }
 0x1100288b => {
  { let v: u32 = 0x11002902u32; cpu.eax = v; }
  cpu.push(0x11002895); f_11005e26(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110028a8); f_110012e4(cpu);
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = 0x1100d570u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x110028c0); f_1100123f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x110028cc); f_1100104b(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x110028d5); f_110011ea(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x110028e1); f_1100290c(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014c4); }
 let mut pc: u32 = 0x110014c4;
 loop { match pc {
 0x110014c4 => {
  pc = 0x11003a01; continue;
 }
 0x11003a01 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11003a09); f_1100131b(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11003a19; continue; }
  pc = 0x11003a10; continue;
 }
 0x11003a10 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11003a16); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11003a19; continue;
 }
 0x11003a19 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014c9); }
 let mut pc: u32 = 0x110014c9;
 loop { match pc {
 0x110014c9 => {
  pc = 0x1100578e; continue;
 }
 0x1100578e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014ce); }
 let mut pc: u32 = 0x110014ce;
 loop { match pc {
 0x110014ce => {
  pc = 0x11003d80; continue;
 }
 0x11003d80 => {
  { let v: u32 = 0x11003df2u32; cpu.eax = v; }
  cpu.push(0x11003d8a); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  cpu.push(0x11003da2); f_110014d3(cpu);
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x110070b4u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x11003dbe); f_110011a9(cpu);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014d3); }
 let mut pc: u32 = 0x110014d3;
 loop { match pc {
 0x110014d3 => {
  pc = 0x110049a0; continue;
 }
 0x110049a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x110049ae); f_110011d1(cpu);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x110070e4u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014d8); }
 let mut pc: u32 = 0x110014d8;
 loop { match pc {
 0x110014d8 => {
  pc = 0x110057a0; continue;
 }
 0x110057a0 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x1u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1100716cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if cpu.zf { pc = 0x110057b9; continue; }
  pc = 0x110057b0; continue;
 }
 0x110057b0 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110057b6); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x110057b9; continue;
 }
 0x110057b9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014dd); }
 let mut pc: u32 = 0x110014dd;
 loop { match pc {
 0x110014dd => {
  pc = 0x11004fa5; continue;
 }
 0x11004fa5 => {
  { let v: u32 = 0x11004fd2u32; cpu.eax = v; }
  cpu.push(0x11004faf); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11004fba); f_11001177(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110014e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110014e2); }
 let mut pc: u32 = 0x110014e2;
 loop { match pc {
 0x110014e2 => {
  pc = 0x11002627; continue;
 }
 0x11002627 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x11002632); f_110010a5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1100265b; continue; }
  pc = 0x11002636; continue;
 }
 0x11002636 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.push(v); }
  let t = cpu.r32(0x1100e218u32); cpu.push(0x11002648); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1100265b; continue; }
  pc = 0x1100264f; continue;
 }
 0x1100264f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x11002657); f_110010a5(cpu);
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x11002636; continue; }
  pc = 0x1100265b; continue;
 }
 0x1100265b => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x11002669; continue; }
  pc = 0x1100265f; continue;
 }
 0x1100265f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x11002669); f_1100130c(cpu);
  pc = 0x11002669; continue;
 }
 0x11002669 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002294(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002294); }
 let mut pc: u32 = 0x11002294;
 loop { match pc {
 0x11002294 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x110022a2; continue; }
  pc = 0x1100229a; continue;
 }
 0x1100229a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x110022a2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110022a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110022a3); }
 let mut pc: u32 = 0x110022a3;
 loop { match pc {
 0x110022a3 => {
  { let v: u32 = 0x110078b8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100230a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100230a); }
 let mut pc: u32 = 0x1100230a;
 loop { match pc {
 0x1100230a => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11002318; continue; }
  pc = 0x11002310; continue;
 }
 0x11002310 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x11002318 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002319(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002319); }
 let mut pc: u32 = 0x11002319;
 loop { match pc {
 0x11002319 => {
  { let v: u32 = 0x110078e8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002378(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002378); }
 let mut pc: u32 = 0x11002378;
 loop { match pc {
 0x11002378 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11002386; continue; }
  pc = 0x1100237e; continue;
 }
 0x1100237e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x11002386 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002387(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002387); }
 let mut pc: u32 = 0x11002387;
 loop { match pc {
 0x11002387 => {
  { let v: u32 = 0x11007918u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002848(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002848); }
 let mut pc: u32 = 0x11002848;
 loop { match pc {
 0x11002848 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11002856; continue; }
  pc = 0x1100284e; continue;
 }
 0x1100284e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x11002856 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002857(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002857); }
 let mut pc: u32 = 0x11002857;
 loop { match pc {
 0x11002857 => {
  { let v: u32 = 0x11007948u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002861(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002861); }
 let mut pc: u32 = 0x11002861;
 loop { match pc {
 0x11002861 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110028f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110028f3); }
 let mut pc: u32 = 0x110028f3;
 loop { match pc {
 0x110028f3 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11002901; continue; }
  pc = 0x110028f9; continue;
 }
 0x110028f9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x11002901 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002902(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002902); }
 let mut pc: u32 = 0x11002902;
 loop { match pc {
 0x11002902 => {
  { let v: u32 = 0x11007980u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100290c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100290c); }
 let mut pc: u32 = 0x1100290c;
 loop { match pc {
 0x1100290c => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110029a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110029a1); }
 let mut pc: u32 = 0x110029a1;
 loop { match pc {
 0x110029a1 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x110029af; continue; }
  pc = 0x110029a7; continue;
 }
 0x110029a7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x110029af => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110029b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110029b0); }
 let mut pc: u32 = 0x110029b0;
 loop { match pc {
 0x110029b0 => {
  { let v: u32 = 0x110079b8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110029ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110029ba); }
 let mut pc: u32 = 0x110029ba;
 loop { match pc {
 0x110029ba => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002a41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002a41); }
 let mut pc: u32 = 0x11002a41;
 loop { match pc {
 0x11002a41 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11002a4f; continue; }
  pc = 0x11002a47; continue;
 }
 0x11002a47 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x11002a4f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002a50); }
 let mut pc: u32 = 0x11002a50;
 loop { match pc {
 0x11002a50 => {
  { let v: u32 = 0x110079f0u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002a5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002a5a); }
 let mut pc: u32 = 0x11002a5a;
 loop { match pc {
 0x11002a5a => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002adf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002adf); }
 let mut pc: u32 = 0x11002adf;
 loop { match pc {
 0x11002adf => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11002aed; continue; }
  pc = 0x11002ae5; continue;
 }
 0x11002ae5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x11002aed => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002aee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002aee); }
 let mut pc: u32 = 0x11002aee;
 loop { match pc {
 0x11002aee => {
  { let v: u32 = 0x11007a28u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002af8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002af8); }
 let mut pc: u32 = 0x11002af8;
 loop { match pc {
 0x11002af8 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002b89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002b89); }
 let mut pc: u32 = 0x11002b89;
 loop { match pc {
 0x11002b89 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x11002b97; continue; }
  pc = 0x11002b8f; continue;
 }
 0x11002b8f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x11002b97 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002b98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002b98); }
 let mut pc: u32 = 0x11002b98;
 loop { match pc {
 0x11002b98 => {
  { let v: u32 = 0x11007a60u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002ba2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002ba2); }
 let mut pc: u32 = 0x11002ba2;
 loop { match pc {
 0x11002ba2 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002c51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002c51); }
 let mut pc: u32 = 0x11002c51;
 loop { match pc {
 0x11002c51 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002c59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002c59); }
 let mut pc: u32 = 0x11002c59;
 loop { match pc {
 0x11002c59 => {
  { let v: u32 = 0x11007a98u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002cd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002cd6); }
 let mut pc: u32 = 0x11002cd6;
 loop { match pc {
 0x11002cd6 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002cde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002cde); }
 let mut pc: u32 = 0x11002cde;
 loop { match pc {
 0x11002cde => {
  { let v: u32 = 0x11007ac8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002d56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002d56); }
 let mut pc: u32 = 0x11002d56;
 loop { match pc {
 0x11002d56 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002d5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002d5e); }
 let mut pc: u32 = 0x11002d5e;
 loop { match pc {
 0x11002d5e => {
  { let v: u32 = 0x11007af8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002dd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002dd5); }
 let mut pc: u32 = 0x11002dd5;
 loop { match pc {
 0x11002dd5 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002ddd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002ddd); }
 let mut pc: u32 = 0x11002ddd;
 loop { match pc {
 0x11002ddd => {
  { let v: u32 = 0x11007b28u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002e7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002e7f); }
 let mut pc: u32 = 0x11002e7f;
 loop { match pc {
 0x11002e7f => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002e87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002e87); }
 let mut pc: u32 = 0x11002e87;
 loop { match pc {
 0x11002e87 => {
  { let v: u32 = 0x11007b58u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002efd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002efd); }
 let mut pc: u32 = 0x11002efd;
 loop { match pc {
 0x11002efd => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002f05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002f05); }
 let mut pc: u32 = 0x11002f05;
 loop { match pc {
 0x11002f05 => {
  { let v: u32 = 0x11007b88u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002f7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002f7e); }
 let mut pc: u32 = 0x11002f7e;
 loop { match pc {
 0x11002f7e => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11002f86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11002f86); }
 let mut pc: u32 = 0x11002f86;
 loop { match pc {
 0x11002f86 => {
  { let v: u32 = 0x11007bb8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100306f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100306f); }
 let mut pc: u32 = 0x1100306f;
 loop { match pc {
 0x1100306f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100307a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100307a); }
 let mut pc: u32 = 0x1100307a;
 loop { match pc {
 0x1100307a => {
  { let v: u32 = 0x11007be8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003084(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003084); }
 let mut pc: u32 = 0x11003084;
 loop { match pc {
 0x11003084 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003104(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003104); }
 let mut pc: u32 = 0x11003104;
 loop { match pc {
 0x11003104 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100310f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100310f); }
 let mut pc: u32 = 0x1100310f;
 loop { match pc {
 0x1100310f => {
  { let v: u32 = 0x11007c20u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003119(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003119); }
 let mut pc: u32 = 0x11003119;
 loop { match pc {
 0x11003119 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003190); }
 let mut pc: u32 = 0x11003190;
 loop { match pc {
 0x11003190 => {
  { let v: u32 = 0x1100d8f0u32; cpu.ecx = v; }
  f_11001433(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003209(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003209); }
 let mut pc: u32 = 0x11003209;
 loop { match pc {
 0x11003209 => {
  cpu.push(0x1100320e); f_11003215(cpu);
  pc = 0x11003256; continue;
 }
 0x11003256 => {
  { let v = 0x11003267u32; cpu.push(v); }
  cpu.push(0x11003260); f_11005f00(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003215(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003215); }
 let mut pc: u32 = 0x11003215;
 loop { match pc {
 0x11003215 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x1100d8d0u32; cpu.ecx = v; }
  { let v = 0x1100d58cu32; cpu.push(v); }
  cpu.push(0x11003226); f_11001118(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x1100d8d8u32; cpu.ecx = v; }
  { let v = 0x1100d580u32; cpu.push(v); }
  cpu.push(0x11003237); f_11001118(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x1100d8e0u32; cpu.ecx = v; }
  { let v = 0x1100d578u32; cpu.push(v); }
  cpu.push(0x11003248); f_11001118(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003267(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003267); }
 let mut pc: u32 = 0x11003267;
 loop { match pc {
 0x11003267 => {
  { cpu.and8(cpu.r8(0x1100d8e8u32), 0x1u8); }
  if !cpu.zf { pc = 0x1100328a; continue; }
  pc = 0x11003270; continue;
 }
 0x11003270 => {
  { let v: u8 = cpu.or8(cpu.r8(0x1100d8e8u32), 0x1u8); cpu.w8(0x1100d8e8u32, v); }
  { let v = 0x11001087u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x1100d8d0u32; cpu.push(v); }
  cpu.push(0x1100328a); f_11005cc0(cpu);
  pc = 0x1100328a; continue;
 }
 0x1100328a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110032df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110032df); }
 let mut pc: u32 = 0x110032df;
 loop { match pc {
 0x110032df => {
  { let v: u32 = 0x11007c58u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110032e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110032e9); }
 let mut pc: u32 = 0x110032e9;
 loop { match pc {
 0x110032e9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001113(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100335c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100335c); }
 let mut pc: u32 = 0x1100335c;
 loop { match pc {
 0x1100335c => {
  { let v: u32 = 0x11007c88u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003366(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003366); }
 let mut pc: u32 = 0x11003366;
 loop { match pc {
 0x11003366 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001113(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110034bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110034bc); }
 let mut pc: u32 = 0x110034bc;
 loop { match pc {
 0x110034bc => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110034c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110034c4); }
 let mut pc: u32 = 0x110034c4;
 loop { match pc {
 0x110034c4 => {
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110034cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110034cc); }
 let mut pc: u32 = 0x110034cc;
 loop { match pc {
 0x110034cc => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110034d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110034d4); }
 let mut pc: u32 = 0x110034d4;
 loop { match pc {
 0x110034d4 => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
