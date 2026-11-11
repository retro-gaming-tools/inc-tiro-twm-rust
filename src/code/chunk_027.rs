#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_14001000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001000); }
 let mut pc: u32 = 0x14001000;
 loop { match pc {
 0x14001000 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x14001020; continue; }
  pc = 0x14001007; continue;
 }
 0x14001007 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x14005034u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x140081c0u32); cpu.push(0x14001019); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x140081c4u32); cpu.push(0x14001020); cpu.call(t);
  pc = 0x14001020; continue;
 }
 0x14001020 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001030); }
 let mut pc: u32 = 0x14001030;
 loop { match pc {
 0x14001030 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x14008168u32); cpu.push(0x14001044); cpu.call(t);
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001050); }
 let mut pc: u32 = 0x14001050;
 loop { match pc {
 0x14001050 => {
  { let v: u32 = cpu.r32(0x14005030u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001060); }
 let mut pc: u32 = 0x14001060;
 loop { match pc {
 0x14001060 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008108u32); cpu.push(0x1400106b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001070); }
 let mut pc: u32 = 0x14001070;
 loop { match pc {
 0x14001070 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14005030u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400810cu32); cpu.push(0x14001099); cpu.call(t);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14005030u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140010b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140010b0); }
 let mut pc: u32 = 0x140010b0;
 loop { match pc {
 0x140010b0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140010d8); f_14001160(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x14008110u32); cpu.push(0x140010e9); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140010fe; continue; }
  pc = 0x140010ed; continue;
 }
 0x140010ed => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x140010f3); f_14001060(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x34u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140010fe => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x14001126; continue; }
  pc = 0x14001102; continue;
 }
 0x14001102 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1400110c); f_14001180(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x14001126; continue; }
  pc = 0x14001115; continue;
 }
 0x14001115 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1400111b); f_14001060(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x34u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001126 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x14001132); f_14001070(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x1400114a; continue; }
  pc = 0x1400113a; continue;
 }
 0x1400113a => {
  cpu.push(0x1400113f); f_14001060(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x34u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400114a => {
  cpu.push(0x1400114f); f_14001060(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x34u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001160); }
 let mut pc: u32 = 0x14001160;
 loop { match pc {
 0x14001160 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008114u32); cpu.push(0x1400117a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001180); }
 let mut pc: u32 = 0x14001180;
 loop { match pc {
 0x14001180 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1400118a); f_140012c0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001190); }
 let mut pc: u32 = 0x14001190;
 loop { match pc {
 0x14001190 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1400119a); f_14001390(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140011a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140011a0); }
 let mut pc: u32 = 0x140011a0;
 loop { match pc {
 0x140011a0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140011b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140011b0); }
 let mut pc: u32 = 0x140011b0;
 loop { match pc {
 0x140011b0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140011f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140011f0); }
 let mut pc: u32 = 0x140011f0;
 loop { match pc {
 0x140011f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x140011f8; continue;
 }
 0x140011f8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x14001217; continue; }
  pc = 0x140011fe; continue;
 }
 0x140011fe => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x14001214; continue; }
  pc = 0x14001202; continue;
 }
 0x14001202 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x14001217; continue; }
  pc = 0x1400120a; continue;
 }
 0x1400120a => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x140011f8; continue; }
  pc = 0x14001214; continue;
 }
 0x14001214 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001217 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001220); }
 let mut pc: u32 = 0x14001220;
 loop { match pc {
 0x14001220 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001250); }
 let mut pc: u32 = 0x14001250;
 loop { match pc {
 0x14001250 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001290); }
 let mut pc: u32 = 0x14001290;
 loop { match pc {
 0x14001290 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x140012ae); f_14001400(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140012c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140012c0); }
 let mut pc: u32 = 0x140012c0;
 loop { match pc {
 0x140012c0 => {
  { let v: u32 = cpu.r32(0x1400506cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x140012d0); f_140012e0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140012e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140012e0); }
 let mut pc: u32 = 0x140012e0;
 loop { match pc {
 0x140012e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffe0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x140012f0; continue; }
  pc = 0x140012eb; continue;
 }
 0x140012eb => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140012f0 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x140012f9; continue; }
  pc = 0x140012f4; continue;
 }
 0x140012f4 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x140012f9; continue;
 }
 0x140012f9 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x140012fd; continue;
 }
 0x140012fd => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0xffffffe0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14001310; continue; }
  pc = 0x14001307; continue;
 }
 0x14001307 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1400130d); f_14001330(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14001310; continue;
 }
 0x14001310 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14001327; continue; }
  pc = 0x14001314; continue;
 }
 0x14001314 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x14001327; continue; }
  pc = 0x14001318; continue;
 }
 0x14001318 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1400131e); f_14001700(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140012fd; continue; }
  pc = 0x14001325; continue;
 }
 0x14001325 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x14001327; continue;
 }
 0x14001327 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001330); }
 let mut pc: u32 = 0x14001330;
 loop { match pc {
 0x14001330 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xfu32); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xfffffff0u32; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14005954u32), cpu.edi); }
  if cpu.cf { pc = 0x14001371; continue; }
  pc = 0x14001344; continue;
 }
 0x14001344 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x1400134b); f_14001860(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001359); f_14001d30(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x14001365); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x14001371; continue; }
  pc = 0x1400136c; continue;
 }
 0x1400136c => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001371 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x140073c0u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008124u32); cpu.push(0x14001380); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001390); }
 let mut pc: u32 = 0x14001390;
 loop { match pc {
 0x14001390 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x140013f8; continue; }
  pc = 0x1400139c; continue;
 }
 0x1400139c => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x140013a3); f_14001860(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x140013b6); f_14001c80(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140013df; continue; }
  pc = 0x140013bd; continue;
 }
 0x140013bd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x140013cd); f_14001ce0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x140013d7); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140013df => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x140013e6); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x140073c0u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008128u32); cpu.push(0x140013f8); cpu.call(t);
  pc = 0x140013f8; continue;
 }
 0x140013f8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001400); }
 let mut pc: u32 = 0x14001400;
 loop { match pc {
 0x14001400 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400141c; continue; }
  pc = 0x14001409; continue;
 }
 0x14001409 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x14001420; continue; }
  pc = 0x14001413; continue;
 }
 0x14001413 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x14001420; continue;
 }
 0x1400141c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x14001420; continue;
 }
 0x14001420 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14001452; continue; }
  pc = 0x1400142a; continue;
 }
 0x1400142a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x14001452; continue; }
  pc = 0x1400142f; continue;
 }
 0x1400142f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x1400142f; continue; }
  pc = 0x1400143a; continue;
 }
 0x1400143a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001441); f_14002130(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if cpu.zf { pc = 0x14001452; continue; }
  pc = 0x1400144a; continue;
 }
 0x1400144a => {
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x14001452; continue; }
  pc = 0x1400144e; continue;
 }
 0x1400144e => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x14001452; continue;
 }
 0x14001452 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400146a; continue; }
  pc = 0x1400145a; continue;
 }
 0x1400145a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1400146a; continue; }
  pc = 0x1400145f; continue;
 }
 0x1400145f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1400145f; continue; }
  pc = 0x1400146a; continue;
 }
 0x1400146a => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1400149a; continue; }
  pc = 0x14001472; continue;
 }
 0x14001472 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x14001480; continue; }
  pc = 0x14001478; continue;
 }
 0x14001478 => {
  { cpu.sub8((cpu.eax as u8), 0x2eu8); }
  if cpu.zf { pc = 0x14001480; continue; }
  pc = 0x1400147c; continue;
 }
 0x1400147c => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x14001480; continue;
 }
 0x14001480 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x1400149d; continue; }
  pc = 0x1400148c; continue;
 }
 0x1400148c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x1400148c; continue; }
  pc = 0x14001498; continue;
 }
 0x14001498 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400149a => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1400149d; continue;
 }
 0x1400149d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140014a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140014a0); }
 let mut pc: u32 = 0x140014a0;
 loop { match pc {
 0x140014a0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x140015a7; continue; }
  pc = 0x140014ad; continue;
 }
 0x140014ad => {
  let t = cpu.r32(0x14008138u32); cpu.push(0x140014b3); cpu.call(t);
  { cpu.sub32(cpu.r32(0x14005068u32), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x1400595cu32, v); }
  if !cpu.zf { pc = 0x140014ff; continue; }
  pc = 0x140014c1; continue;
 }
 0x140014c1 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if !cpu.zf { pc = 0x140014d6; continue; }
  pc = 0x140014c5; continue;
 }
 0x140014c5 => {
  { cpu.and32(cpu.eax, 0x80000000u32); }
  if cpu.zf { pc = 0x140014d6; continue; }
  pc = 0x140014cc; continue;
 }
 0x140014cc => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x140014d3); f_14002f30(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x140014d6; continue;
 }
 0x140014d6 => {
  { let v = 0x14004008u32; cpu.push(v); }
  let t = cpu.r32(0x14008134u32); cpu.push(0x140014e1); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140014ff; continue; }
  pc = 0x140014e5; continue;
 }
 0x140014e5 => {
  { let v = 0x14004000u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008130u32); cpu.push(0x140014f1); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140014ff; continue; }
  pc = 0x140014f5; continue;
 }
 0x140014f5 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x140014fc); f_14002f30(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x140014ff; continue;
 }
 0x140014ff => {
  cpu.push(0x14001504); f_14001740(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1400150d; continue; }
  pc = 0x14001508; continue;
 }
 0x14001508 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1400150d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1400595cu32); cpu.ecx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14005968u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14005964u32, v); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1400595cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14005960u32, v); }
  cpu.push(0x14001541); f_14002310(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1400154f; continue; }
  pc = 0x14001545; continue;
 }
 0x14001545 => {
  cpu.push(0x1400154a); f_14001780(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1400154f => {
  let t = cpu.r32(0x1400812cu32); cpu.push(0x14001555); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x140073c4u32, v); }
  cpu.push(0x1400155f); f_14002da0(cpu);
  { cpu.sub32(cpu.r32(0x140073c4u32), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x14005058u32, v); }
  if cpu.zf { pc = 0x14001598; continue; }
  pc = 0x1400156d; continue;
 }
 0x1400156d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14001598; continue; }
  pc = 0x14001571; continue;
 }
 0x14001571 => {
  cpu.push(0x14001576); f_140024e0(cpu);
  cpu.push(0x1400157b); f_14002d90(cpu);
  cpu.push(0x14001580); f_14002810(cpu);
  cpu.push(0x14001585); f_14002720(cpu);
  cpu.push(0x1400158a); f_140021b0(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x14005054u32)); cpu.w32(0x14005054u32, v); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x14001598 => {
  cpu.push(0x1400159d); f_14002370(cpu);
  cpu.push(0x140015a2); f_14001780(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x140015a7 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140015e4; continue; }
  pc = 0x140015ab; continue;
 }
 0x140015ab => {
  { let v: u32 = cpu.r32(0x14005054u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x140015df; continue; }
  pc = 0x140015b4; continue;
 }
 0x140015b4 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14005994u32), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x14005054u32, v); }
  if !cpu.zf { pc = 0x140015c8; continue; }
  pc = 0x140015c3; continue;
 }
 0x140015c3 => {
  cpu.push(0x140015c8); f_14002200(cpu);
  pc = 0x140015c8; continue;
 }
 0x140015c8 => {
  cpu.push(0x140015cd); f_140026c0(cpu);
  cpu.push(0x140015d2); f_14002370(cpu);
  cpu.push(0x140015d7); f_14001780(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x140015df => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x140015e4 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x140015f3; continue; }
  pc = 0x140015e9; continue;
 }
 0x140015e9 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x140015f0); f_14002430(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x140015f3; continue;
 }
 0x140015f3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001600); }
 let mut pc: u32 = 0x14001600;
 loop { match pc {
 0x14001600 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  { let v = cpu.ebp; cpu.push(v); }
  if !cpu.zf { pc = 0x14001623; continue; }
  pc = 0x14001611; continue;
 }
 0x14001611 => {
  { cpu.sub32(cpu.r32(0x14005054u32), 0x0u32); }
  if !cpu.zf { pc = 0x14001623; continue; }
  pc = 0x1400161a; continue;
 }
 0x1400161a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x14001623 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x1400162d; continue; }
  pc = 0x14001628; continue;
 }
 0x14001628 => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if !cpu.zf { pc = 0x14001667; continue; }
  pc = 0x1400162d; continue;
 }
 0x1400162d => {
  { let v: u32 = cpu.r32(0x140073c8u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x14001646; continue; }
  pc = 0x14001637; continue;
 }
 0x14001637 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ecx; cpu.push(0x14001644); cpu.call(t);
  pc = 0x1400164e; continue;
 }
 0x14001646 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x1400164e; continue;
 }
 0x1400164e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400165e; continue; }
  pc = 0x14001652; continue;
 }
 0x14001652 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1400165a); f_140014a0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1400166f; continue; }
  pc = 0x1400165e; continue;
 }
 0x1400165e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x14001667 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x1400166f; continue;
 }
 0x1400166f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14001677); f_14002f40(cpu);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x1400168b; continue; }
  pc = 0x1400167e; continue;
 }
 0x1400167e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x1400168b; continue; }
  pc = 0x14001682; continue;
 }
 0x14001682 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1400168b); f_140014a0(cpu);
  pc = 0x1400168b; continue;
 }
 0x1400168b => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x14001694; continue; }
  pc = 0x1400168f; continue;
 }
 0x1400168f => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if !cpu.zf { pc = 0x140016b6; continue; }
  pc = 0x14001694; continue;
 }
 0x14001694 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1400169c); f_140014a0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140016a2; continue; }
  pc = 0x140016a0; continue;
 }
 0x140016a0 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x140016a2; continue;
 }
 0x140016a2 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x140016b6; continue; }
  pc = 0x140016a6; continue;
 }
 0x140016a6 => {
  { let v: u32 = cpu.r32(0x140073c8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140016b6; continue; }
  pc = 0x140016af; continue;
 }
 0x140016af => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.eax; cpu.push(0x140016b4); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x140016b6; continue;
 }
 0x140016b6 => {
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
pub fn f_140016c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140016c0); }
 let mut pc: u32 = 0x140016c0;
 loop { match pc {
 0x140016c0 => {
  { let v: u32 = cpu.r32(0x14005064u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x140016d7; continue; }
  pc = 0x140016ca; continue;
 }
 0x140016ca => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140016dc; continue; }
  pc = 0x140016ce; continue;
 }
 0x140016ce => {
  { cpu.sub32(cpu.r32(0x14005068u32), 0x1u32); }
  if !cpu.zf { pc = 0x140016dc; continue; }
  pc = 0x140016d7; continue;
 }
 0x140016d7 => {
  cpu.push(0x140016dc); f_14002f50(cpu);
  pc = 0x140016dc; continue;
 }
 0x140016dc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140016e6); f_14002f90(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xffu32; cpu.push(v); }
  let t = cpu.r32(0x14005060u32); cpu.push(0x140016f4); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001700); }
 let mut pc: u32 = 0x14001700;
 loop { match pc {
 0x14001700 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x14001707); f_14001860(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1400612cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14001731; continue; }
  pc = 0x14001713; continue;
 }
 0x14001713 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.eax; cpu.push(0x1400171a); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14001731; continue; }
  pc = 0x14001721; continue;
 }
 0x14001721 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x14001728); f_140018d0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001731 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x14001738); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001740); }
 let mut pc: u32 = 0x14001740;
 loop { match pc {
 0x14001740 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x14008140u32); cpu.push(0x1400174f); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x140073c0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1400175b; continue; }
  pc = 0x14001758; continue;
 }
 0x14001758 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400175b => {
  cpu.push(0x14001760); f_140019d0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14001773; continue; }
  pc = 0x14001764; continue;
 }
 0x14001764 => {
  { let v: u32 = cpu.r32(0x140073c0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400813cu32); cpu.push(0x14001770); cpu.call(t);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001773 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001780); }
 let mut pc: u32 = 0x14001780;
 loop { match pc {
 0x14001780 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14005138u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14008144u32); cpu.edi = v; }
  pc = 0x1400178d; continue;
 }
 0x1400178d => {
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140017a1; continue; }
  pc = 0x14001797; continue;
 }
 0x14001797 => {
  { let v = 0x8000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x140017a1); cpu.call(t);
  pc = 0x140017a1; continue;
 }
 0x140017a1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x14005138u32); }
  if !cpu.zf { pc = 0x1400178d; continue; }
  pc = 0x140017ab; continue;
 }
 0x140017ab => {
  { let v: u32 = cpu.r32(0x140073c0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400813cu32); cpu.push(0x140017b7); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140017c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140017c0); }
 let mut pc: u32 = 0x140017c0;
 loop { match pc {
 0x140017c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x140050bcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14008148u32); cpu.esi = v; }
  let t = cpu.esi; cpu.push(0x140017cf); cpu.call(t);
  { let v: u32 = cpu.r32(0x140050acu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x140017d8); cpu.call(t);
  { let v: u32 = cpu.r32(0x1400509cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x140017e0); cpu.call(t);
  { let v: u32 = cpu.r32(0x1400507cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x140017e8); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140017f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140017f0); }
 let mut pc: u32 = 0x140017f0;
 loop { match pc {
 0x140017f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1400814cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x140017fb; continue;
 }
 0x140017fb => {
  { let v: u32 = 0x14005078u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400182a; continue; }
  pc = 0x14001808; continue;
 }
 0x14001808 => {
  { cpu.sub32(cpu.edi, 0x11u32); }
  if cpu.zf { pc = 0x1400182a; continue; }
  pc = 0x1400180d; continue;
 }
 0x1400180d => {
  { cpu.sub32(cpu.edi, 0xdu32); }
  if cpu.zf { pc = 0x1400182a; continue; }
  pc = 0x14001812; continue;
 }
 0x14001812 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x1400182a; continue; }
  pc = 0x14001817; continue;
 }
 0x14001817 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x1400182a; continue; }
  pc = 0x1400181c; continue;
 }
 0x1400181c => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x1400181f); cpu.call(t);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001827); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1400182a; continue;
 }
 0x1400182a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x140017fb; continue; }
  pc = 0x14001830; continue;
 }
 0x14001830 => {
  { let v: u32 = cpu.r32(0x1400509cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x14001838); cpu.call(t);
  { let v: u32 = cpu.r32(0x140050acu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x14001840); cpu.call(t);
  { let v: u32 = cpu.r32(0x140050bcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x14001848); cpu.call(t);
  { let v: u32 = cpu.r32(0x1400507cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x14001850); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001860); }
 let mut pc: u32 = 0x14001860;
 loop { match pc {
 0x14001860 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14005078u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x140018bb; continue; }
  pc = 0x14001875; continue;
 }
 0x14001875 => {
  { let v = 0x18u32; cpu.push(v); }
  cpu.push(0x1400187c); f_140012c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1400188f; continue; }
  pc = 0x14001885; continue;
 }
 0x14001885 => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x1400188c); f_140016c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1400188f; continue;
 }
 0x1400188f => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x14001896); f_14001860(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x140018a9; continue; }
  pc = 0x1400189f; continue;
 }
 0x1400189f => {
  let t = cpu.r32(0x14008148u32); cpu.push(0x140018a5); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x140018b1; continue;
 }
 0x140018a9 => {
  cpu.push(0x140018ae); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x140018b1; continue;
 }
 0x140018b1 => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x140018b8); f_140018d0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x140018bb; continue;
 }
 0x140018bb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008150u32); cpu.push(0x140018c4); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140018d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140018d0); }
 let mut pc: u32 = 0x140018d0;
 loop { match pc {
 0x140018d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14005078u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x14008154u32); cpu.push(0x140018e5); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140018f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140018f0); }
 let mut pc: u32 = 0x140018f0;
 loop { match pc {
 0x140018f0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x14005c80u32); }
  if cpu.cf { pc = 0x1400191a; continue; }
  pc = 0x140018fe; continue;
 }
 0x140018fe => {
  { cpu.sub32(cpu.eax, 0x14005ee0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1400191a; continue; }
  pc = 0x14001905; continue;
 }
 0x14001905 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x14005c80u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001916); f_14001860(cpu);
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400191a => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008150u32); cpu.push(0x14001924); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001930); }
 let mut pc: u32 = 0x14001930;
 loop { match pc {
 0x14001930 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x14u32); }
  if (cpu.sf == cpu.of) { pc = 0x14001949; continue; }
  pc = 0x1400193c; continue;
 }
 0x1400193c => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001945); f_14001860(cpu);
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001949 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008150u32); cpu.push(0x14001956); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001960); }
 let mut pc: u32 = 0x14001960;
 loop { match pc {
 0x14001960 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x14005c80u32); }
  if cpu.cf { pc = 0x1400198a; continue; }
  pc = 0x1400196e; continue;
 }
 0x1400196e => {
  { cpu.sub32(cpu.eax, 0x14005ee0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1400198a; continue; }
  pc = 0x14001975; continue;
 }
 0x14001975 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x14005c80u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001986); f_140018d0(cpu);
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400198a => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008154u32); cpu.push(0x14001994); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140019a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140019a0); }
 let mut pc: u32 = 0x140019a0;
 loop { match pc {
 0x140019a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x14u32); }
  if (cpu.sf == cpu.of) { pc = 0x140019b9; continue; }
  pc = 0x140019ac; continue;
 }
 0x140019ac => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140019b5); f_140018d0(cpu);
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140019b9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008154u32); cpu.push(0x140019c6); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140019d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140019d0); }
 let mut pc: u32 = 0x140019d0;
 loop { match pc {
 0x140019d0 => {
  { cpu.sub32(cpu.r32(0x14005948u32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if !cpu.zf { pc = 0x14001a49; continue; }
  pc = 0x140019dd; continue;
 }
 0x140019dd => {
  { let v: u32 = 0x14005138u32; cpu.esi = v; }
  pc = 0x140019e2; continue;
 }
 0x140019e2 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1400815cu32); cpu.edi = v; }
  { let v = 0x2000u32; cpu.push(v); }
  { let v = 0x400000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x140019f8); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x14001b22; continue; }
  pc = 0x14001a02; continue;
 }
 0x14001a02 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1000u32; cpu.push(v); }
  { let v = 0x10000u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x14001a11); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14001b14; continue; }
  pc = 0x14001a19; continue;
 }
 0x14001a19 => {
  { cpu.sub32(cpu.esi, 0x14005138u32); }
  if !cpu.zf { pc = 0x14001a6d; continue; }
  pc = 0x14001a21; continue;
 }
 0x14001a21 => {
  { cpu.sub32(cpu.r32(0x14005138u32), 0x0u32); }
  if !cpu.zf { pc = 0x14001a34; continue; }
  pc = 0x14001a2a; continue;
 }
 0x14001a2a => {
  { let v: u32 = 0x14005138u32; cpu.w32(0x14005138u32, v); }
  pc = 0x14001a34; continue;
 }
 0x14001a34 => {
  { cpu.sub32(cpu.r32(0x1400513cu32), 0x0u32); }
  if !cpu.zf { pc = 0x14001a86; continue; }
  pc = 0x14001a3d; continue;
 }
 0x14001a3d => {
  { let v: u32 = 0x14005138u32; cpu.w32(0x1400513cu32, v); }
  pc = 0x14001a86; continue;
 }
 0x14001a49 => {
  { let v = 0x814u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x140073c0u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008124u32); cpu.push(0x14001a5c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x140019e2; continue; }
  pc = 0x14001a66; continue;
 }
 0x14001a66 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001a6d => {
  { let v: u32 = 0x14005138u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x1400513cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1400513cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x14001a86; continue;
 }
 0x14001a86 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.ecx = v; }
  { let v: u32 = 0xf1u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x810u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x10u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x14001aa7; continue;
 }
 0x14001aa7 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.ebp, 0x10u32); }
  if (cpu.sf == cpu.of) { pc = 0x14001ab4; continue; }
  pc = 0x14001aaf; continue;
 }
 0x14001aaf => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x14001ab7; continue;
 }
 0x14001ab4 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x14001ab7; continue;
 }
 0x14001ab7 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x410u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.ebp, 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x14001aa7; continue; }
  pc = 0x14001ac6; continue;
 }
 0x14001ac6 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10000u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x14001b0d; continue; }
  pc = 0x14001ae0; continue;
 }
 0x14001ae0 => {
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.ecx = v; }
  pc = 0x14001aea; continue;
 }
 0x14001aea => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xf8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x1000u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10000u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14001aea; continue; }
  pc = 0x14001b0d; continue;
 }
 0x14001b0d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001b14 => {
  { let v = 0x8000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x14008144u32); cpu.push(0x14001b22); cpu.call(t);
  pc = 0x14001b22; continue;
 }
 0x14001b22 => {
  { cpu.sub32(cpu.esi, 0x14005138u32); }
  if cpu.zf { pc = 0x14001b39; continue; }
  pc = 0x14001b2a; continue;
 }
 0x14001b2a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x140073c0u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008128u32); cpu.push(0x14001b39); cpu.call(t);
  pc = 0x14001b39; continue;
 }
 0x14001b39 => {
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
pub fn f_14001b40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001b40); }
 let mut pc: u32 = 0x14001b40;
 loop { match pc {
 0x14001b40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x8000u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008144u32); cpu.push(0x14001b59); cpu.call(t);
  { cpu.sub32(cpu.r32(0x1400594cu32), cpu.esi); }
  if !cpu.zf { pc = 0x14001b69; continue; }
  pc = 0x14001b61; continue;
 }
 0x14001b61 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1400594cu32, v); }
  pc = 0x14001b69; continue;
 }
 0x14001b69 => {
  { cpu.sub32(cpu.esi, 0x14005138u32); }
  if cpu.zf { pc = 0x14001b92; continue; }
  pc = 0x14001b71; continue;
 }
 0x14001b71 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x140073c0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x14008128u32); cpu.push(0x14001b90); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001b92 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14005948u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001ba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001ba0); }
 let mut pc: u32 = 0x14001ba0;
 loop { match pc {
 0x14001ba0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1400513cu32); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x14001bad; continue;
 }
 0x14001bad => {
  { cpu.sub32(cpu.r32(0x810u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x14001c5c; continue; }
  pc = 0x14001bba; continue;
 }
 0x14001bba => {
  { let v: u32 = 0x3ffu32; cpu.ebp = v; }
  { let v: u32 = 0x40fu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x3ff000u32; cpu.edi = v; }
  pc = 0x14001bd2; continue;
 }
 0x14001bd2 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xf0u8); }
  if !cpu.zf { pc = 0x14001c16; continue; }
  pc = 0x14001bd7; continue;
 }
 0x14001bd7 => {
  { let v = 0x4000u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x1000u32; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008144u32); cpu.push(0x14001bf0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14001c16; continue; }
  pc = 0x14001bf4; continue;
 }
 0x14001bf4 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x14005950u32)); cpu.w32(0x14005950u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x14001c09; continue; }
  pc = 0x14001c05; continue;
 }
 0x14001c05 => {
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x14001c0c; continue; }
  pc = 0x14001c09; continue;
 }
 0x14001c09 => {
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x14001c0c; continue;
 }
 0x14001c0c => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x14001c22; continue; }
  pc = 0x14001c16; continue;
 }
 0x14001c16 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x1000u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x14001bd2; continue; }
  pc = 0x14001c22; continue;
 }
 0x14001c22 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  if cpu.zf { pc = 0x14001c5c; continue; }
  pc = 0x14001c2e; continue;
 }
 0x14001c2e => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x14001c5c; continue; }
  pc = 0x14001c34; continue;
 }
 0x14001c34 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x11u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x14001c3c; continue;
 }
 0x14001c3c => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if !cpu.zf { pc = 0x14001c4b; continue; }
  pc = 0x14001c41; continue;
 }
 0x14001c41 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x14001c3c; continue; }
  pc = 0x14001c4b; continue;
 }
 0x14001c4b => {
  { cpu.sub32(cpu.edx, 0x400u32); }
  if !cpu.zf { pc = 0x14001c5c; continue; }
  pc = 0x14001c53; continue;
 }
 0x14001c53 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001c59); f_14001b40(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14001c5c; continue;
 }
 0x14001c5c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1400513cu32)); }
  if cpu.zf { pc = 0x14001c6f; continue; }
  pc = 0x14001c64; continue;
 }
 0x14001c64 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x14001bad; continue; }
  pc = 0x14001c6f; continue;
 }
 0x14001c6f => {
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
pub fn f_14001c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001c80); }
 let mut pc: u32 = 0x14001c80;
 loop { match pc {
 0x14001c80 => {
  { let v: u32 = 0x14005138u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x14001c89; continue;
 }
 0x14001c89 => {
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14001ca0; continue; }
  pc = 0x14001c93; continue;
 }
 0x14001c93 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x14001ca0; continue; }
  pc = 0x14001c97; continue;
 }
 0x14001c97 => {
  { let v: u32 = cpu.eax.wrapping_add(0x400000u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14001cad; continue; }
  pc = 0x14001ca0; continue;
 }
 0x14001ca0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x14005138u32); }
  if !cpu.zf { pc = 0x14001c89; continue; }
  pc = 0x14001caa; continue;
 }
 0x14001caa => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001cad => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffff000u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x100u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001ce0); }
 let mut pc: u32 = 0x14001ce0;
 loop { match pc {
 0x14001ce0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x810u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 12u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0xf1u8; cpu.w8(0x410u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0xf0u8); }
  if !cpu.zf { pc = 0x14001d25; continue; }
  pc = 0x14001d0c; continue;
 }
 0x14001d0c => {
  { let v: u32 = cpu.inc32(cpu.r32(0x14005950u32)); cpu.w32(0x14005950u32, v); }
  { cpu.sub32(cpu.r32(0x14005950u32), 0x20u32); }
  if !cpu.zf { pc = 0x14001d25; continue; }
  pc = 0x14001d1b; continue;
 }
 0x14001d1b => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x14001d22); f_14001ba0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x14001d25; continue;
 }
 0x14001d25 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14001d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001d30); }
 let mut pc: u32 = 0x14001d30;
 loop { match pc {
 0x14001d30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1400594cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x14001d3e; continue;
 }
 0x14001d3e => {
  { cpu.sub32(cpu.r32(0x810u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x14001dff; continue; }
  pc = 0x14001d4b; continue;
 }
 0x14001d4b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x400u32); }
  if (cpu.sf == cpu.of) { pc = 0x14001daa; continue; }
  pc = 0x14001d56; continue;
 }
 0x14001d56 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 12u32); cpu.ebp = v; }
  pc = 0x14001d5b; continue;
 }
 0x14001d5b => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14001d9b; continue; }
  pc = 0x14001d67; continue;
 }
 0x14001d67 => {
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x14001d9b; continue; }
  pc = 0x14001d6c; continue;
 }
 0x14001d6c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x410u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x14001d9b; continue; }
  pc = 0x14001d79; continue;
 }
 0x14001d79 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001d89); f_14001fb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14001e70; continue; }
  pc = 0x14001d94; continue;
 }
 0x14001d94 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x410u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x14001d9b; continue;
 }
 0x14001d9b => {
  { let v: u32 = cpu.ebp.wrapping_add(0x1000u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.ebp, 0x400000u32); }
  if (cpu.sf != cpu.of) { pc = 0x14001d5b; continue; }
  pc = 0x14001daa; continue;
 }
 0x14001daa => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x14001dff; continue; }
  pc = 0x14001db3; continue;
 }
 0x14001db3 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14001df3; continue; }
  pc = 0x14001dbf; continue;
 }
 0x14001dbf => {
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x14001df3; continue; }
  pc = 0x14001dc4; continue;
 }
 0x14001dc4 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x410u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x14001df3; continue; }
  pc = 0x14001dd1; continue;
 }
 0x14001dd1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14001de1); f_14001fb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14001e82; continue; }
  pc = 0x14001dec; continue;
 }
 0x14001dec => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x410u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x14001df3; continue;
 }
 0x14001df3 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x1000u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x14001db3; continue; }
  pc = 0x14001dff; continue;
 }
 0x14001dff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1400594cu32)); }
  if !cpu.zf { pc = 0x14001d3e; continue; }
  pc = 0x14001e0d; continue;
 }
 0x14001e0d => {
  { let v: u32 = 0x14005138u32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  pc = 0x14001e19; continue;
 }
 0x14001e19 => {
  { cpu.sub32(cpu.r32(0x810u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.zf { pc = 0x14001e26; continue; }
  pc = 0x14001e21; continue;
 }
 0x14001e21 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), cpu.ecx); }
  if !cpu.zf { pc = 0x14001e94; continue; }
  pc = 0x14001e26; continue;
 }
 0x14001e26 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x14005138u32); }
  if !cpu.zf { pc = 0x14001e19; continue; }
  pc = 0x14001e30; continue;
 }
 0x14001e30 => {
  cpu.push(0x14001e35); f_140019d0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14001fa3; continue; }
  pc = 0x14001e3d; continue;
 }
 0x14001e3d => {
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1400594cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)).wrapping_sub((cpu.ebx as u8)); cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x100u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001e70 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1400594cu32, v); }
  { let v: u8 = cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))), (cpu.ebx as u8)); cpu.w8(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001e82 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1400594cu32, v); }
  { let v: u8 = cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))), (cpu.ebx as u8)); cpu.w8(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001e94 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x14001ea9; continue; }
  pc = 0x14001ea4; continue;
 }
 0x14001ea4 => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  pc = 0x14001ea9; continue;
 }
 0x14001ea9 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x14001ebe; continue; }
  pc = 0x14001eae; continue;
 }
 0x14001eae => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x14001eb3; continue;
 }
 0x14001eb3 => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x14001ebe; continue; }
  pc = 0x14001eb9; continue;
 }
 0x14001eb9 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x14001eb3; continue; }
  pc = 0x14001ebe; continue;
 }
 0x14001ebe => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.ebp, 12u32); cpu.ebp = v; }
  { let v = 0x1000u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 12u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400815cu32); cpu.push(0x14001ee1); cpu.call(t);
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x810u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.zf { pc = 0x14001f9c; continue; }
  pc = 0x14001eef; continue;
 }
 0x14001eef => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 12u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x810u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x14001f31; continue; }
  pc = 0x14001f01; continue;
 }
 0x14001f01 => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  pc = 0x14001f06; continue;
 }
 0x14001f06 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x1000u32); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffff004u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xfffff0f8u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0xf1u8; cpu.w8(0x410u32.wrapping_add(cpu.ecx), v); }
  if (cpu.sf != cpu.of) { pc = 0x14001f06; continue; }
  pc = 0x14001f31; continue;
 }
 0x14001f31 => {
  { let v: u32 = cpu.esi; cpu.w32(0x1400594cu32, v); }
  { cpu.sub32(cpu.edi, 0x400u32); }
  if (cpu.sf == cpu.of) { pc = 0x14001f53; continue; }
  pc = 0x14001f3f; continue;
 }
 0x14001f3f => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x14001f44; continue;
 }
 0x14001f44 => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x14001f53; continue; }
  pc = 0x14001f4a; continue;
 }
 0x14001f4a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x14001f44; continue; }
  pc = 0x14001f53; continue;
 }
 0x14001f53 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x400u32); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  if (cpu.sf == cpu.of) { pc = 0x14001f68; continue; }
  pc = 0x14001f65; continue;
 }
 0x14001f65 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x14001f68; continue;
 }
 0x14001f68 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 12u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))).wrapping_sub((cpu.ebx as u8)); cpu.w8(0x10u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), cpu.ebx); cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001f9c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001fa3 => {
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
pub fn f_14001fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14001fb0); }
 let mut pc: u32 = 0x14001fb0;
 loop { match pc {
 0x14001fb0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if cpu.cf { pc = 0x14001ffa; continue; }
  pc = 0x14001fc7; continue;
 }
 0x14001fc7 => {
  { let v: u32 = 0xf8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if !cpu.cf { pc = 0x14001fdd; continue; }
  pc = 0x14001fd6; continue;
 }
 0x14001fd6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)).wrapping_sub(cpu.edx); cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x14001fe9; continue;
 }
 0x14001fdd => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x14001fe9; continue;
 }
 0x14001fe9 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14001ffa => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x14002003; continue; }
  pc = 0x14002001; continue;
 }
 0x14002001 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x14002003; continue;
 }
 0x14002003 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xf8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  if !cpu.cf { pc = 0x1400209e; continue; }
  pc = 0x14002018; continue;
 }
 0x14002018 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x1400204c; continue; }
  pc = 0x1400201f; continue;
 }
 0x1400201f => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x14002033; continue; }
  pc = 0x1400202c; continue;
 }
 0x1400202c => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x1400202c; continue; }
  pc = 0x14002033; continue;
 }
 0x14002033 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.cf { pc = 0x14002068; continue; }
  pc = 0x14002037; continue;
 }
 0x14002037 => {
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if !cpu.zf { pc = 0x14002042; continue; }
  pc = 0x1400203b; continue;
 }
 0x1400203b => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x14002052; continue;
 }
 0x14002042 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14002061; continue; }
  pc = 0x14002048; continue;
 }
 0x14002048 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x14002052; continue;
 }
 0x1400204c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  pc = 0x14002052; continue;
 }
 0x14002052 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xf8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if cpu.cf { pc = 0x14002018; continue; }
  pc = 0x1400205f; continue;
 }
 0x1400205f => {
  pc = 0x1400209e; continue;
 }
 0x14002061 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002068 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xf8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if !cpu.cf { pc = 0x1400207e; continue; }
  pc = 0x14002075; continue;
 }
 0x14002075 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x1400208a; continue;
 }
 0x1400207e => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1400208a; continue;
 }
 0x1400208a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400209e => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x140020e8; continue; }
  pc = 0x140020a8; continue;
 }
 0x140020a8 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xf7u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x140020e8; continue; }
  pc = 0x140020b5; continue;
 }
 0x140020b5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x140020de; continue; }
  pc = 0x140020bc; continue;
 }
 0x140020bc => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { pc = 0x140020d0; continue; }
  pc = 0x140020c9; continue;
 }
 0x140020c9 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x140020c9; continue; }
  pc = 0x140020d0; continue;
 }
 0x140020d0 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.cf { pc = 0x140020f6; continue; }
  pc = 0x140020d4; continue;
 }
 0x140020d4 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x140020ef; continue; }
  pc = 0x140020da; continue;
 }
 0x140020da => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x140020e4; continue;
 }
 0x140020de => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  pc = 0x140020e4; continue;
 }
 0x140020e4 => {
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (!cpu.cf && !cpu.zf) { pc = 0x140020a8; continue; }
  pc = 0x140020e8; continue;
 }
 0x140020e8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140020ef => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140020f6 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xf8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if !cpu.cf { pc = 0x1400210c; continue; }
  pc = 0x14002103; continue;
 }
 0x14002103 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x14002118; continue;
 }
 0x1400210c => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x14002118; continue;
 }
 0x14002118 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002130); }
 let mut pc: u32 = 0x14002130;
 loop { match pc {
 0x14002130 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14002145; continue; }
  pc = 0x1400213f; continue;
 }
 0x1400213f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002145 => {
  { cpu.sub32(cpu.r32(0x14005accu32), 0x0u32); }
  if !cpu.zf { pc = 0x14002155; continue; }
  pc = 0x1400214e; continue;
 }
 0x1400214e => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002155 => {
  { let v = 0x19u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.ebx = v; }
  cpu.push(0x1400215f); f_14001860(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x140059c9u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x14002180; continue; }
  pc = 0x1400216f; continue;
 }
 0x1400216f => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x14002176); f_140018d0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002180 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x14002185; continue;
 }
 0x14002185 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if cpu.cf { pc = 0x14002196; continue; }
  pc = 0x1400218a; continue;
 }
 0x1400218a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x140059c9u32.wrapping_add(cpu.eax)), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x14002185; continue; }
  pc = 0x14002196; continue;
 }
 0x14002196 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x1400219d); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140021b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140021b0); }
 let mut pc: u32 = 0x140021b0;
 loop { match pc {
 0x140021b0 => {
  { let v: u32 = cpu.r32(0x140073bcu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140021bb; continue; }
  pc = 0x140021b9; continue;
 }
 0x140021b9 => {
  let t = cpu.eax; cpu.push(0x140021bb); cpu.call(t);
  pc = 0x140021bb; continue;
 }
 0x140021bb => {
  { let v = 0x14005010u32; cpu.push(v); }
  { let v = 0x14005008u32; cpu.push(v); }
  cpu.push(0x140021ca); f_140022f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x14005004u32; cpu.push(v); }
  { let v = 0x14005000u32; cpu.push(v); }
  cpu.push(0x140021dc); f_140022f0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140021e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140021e0); }
 let mut pc: u32 = 0x140021e0;
 loop { match pc {
 0x140021e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140021ee); f_14002210(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002200); }
 let mut pc: u32 = 0x14002200;
 loop { match pc {
 0x14002200 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1400220b); f_14002210(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002210); }
 let mut pc: u32 = 0x14002210;
 loop { match pc {
 0x14002210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x14002218); f_140022d0(cpu);
  { cpu.sub32(cpu.r32(0x14005998u32), 0x1u32); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  if !cpu.zf { pc = 0x14002233; continue; }
  pc = 0x14002225; continue;
 }
 0x14002225 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x14008164u32); cpu.push(0x1400222c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008160u32); cpu.push(0x14002233); cpu.call(t);
  pc = 0x14002233; continue;
 }
 0x14002233 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14005994u32, v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14005990u32, v); }
  if !cpu.zf { pc = 0x1400228d; continue; }
  pc = 0x1400224e; continue;
 }
 0x1400224e => {
  { cpu.sub32(cpu.r32(0x140073b8u32), 0x0u32); }
  if cpu.zf { pc = 0x1400227b; continue; }
  pc = 0x14002257; continue;
 }
 0x14002257 => {
  { let v: u32 = cpu.r32(0x140073b4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x140073b8u32)); }
  if cpu.cf { pc = 0x1400227b; continue; }
  pc = 0x14002268; continue;
 }
 0x14002268 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002270; continue; }
  pc = 0x1400226e; continue;
 }
 0x1400226e => {
  let t = cpu.eax; cpu.push(0x14002270); cpu.call(t);
  pc = 0x14002270; continue;
 }
 0x14002270 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x140073b8u32)); }
  if !cpu.cf { pc = 0x14002268; continue; }
  pc = 0x1400227b; continue;
 }
 0x1400227b => {
  { let v = 0x1400501cu32; cpu.push(v); }
  { let v = 0x14005014u32; cpu.push(v); }
  cpu.push(0x1400228a); f_140022f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1400228d; continue;
 }
 0x1400228d => {
  { let v = 0x14005024u32; cpu.push(v); }
  { let v = 0x14005020u32; cpu.push(v); }
  cpu.push(0x1400229c); f_140022f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x140022ac; continue; }
  pc = 0x140022a3; continue;
 }
 0x140022a3 => {
  cpu.push(0x140022a8); f_140022e0(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140022ac => {
  { let v: u32 = 0x1u32; cpu.w32(0x14005998u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x14008158u32); cpu.push(0x140022bd); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140022d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140022d0); }
 let mut pc: u32 = 0x140022d0;
 loop { match pc {
 0x140022d0 => {
  { let v = 0xdu32; cpu.push(v); }
  cpu.push(0x140022d7); f_14001860(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140022e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140022e0); }
 let mut pc: u32 = 0x140022e0;
 loop { match pc {
 0x140022e0 => {
  { let v = 0xdu32; cpu.push(v); }
  cpu.push(0x140022e7); f_140018d0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140022f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140022f0); }
 let mut pc: u32 = 0x140022f0;
 loop { match pc {
 0x140022f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1400230d; continue; }
  pc = 0x140022fe; continue;
 }
 0x140022fe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002306; continue; }
  pc = 0x14002304; continue;
 }
 0x14002304 => {
  let t = cpu.eax; cpu.push(0x14002306); cpu.call(t);
  pc = 0x14002306; continue;
 }
 0x14002306 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x140022fe; continue; }
  pc = 0x1400230d; continue;
 }
 0x1400230d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002310); }
 let mut pc: u32 = 0x14002310;
 loop { match pc {
 0x14002310 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14002316); f_140017c0(cpu);
  let t = cpu.r32(0x1400811cu32); cpu.push(0x1400231c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1400599cu32, v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1400232a; continue; }
  pc = 0x14002326; continue;
 }
 0x14002326 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400232a => {
  { let v = 0x74u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x14002333); f_14003280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1400236c; continue; }
  pc = 0x1400233c; continue;
 }
 0x1400233c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1400599cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400816cu32); cpu.push(0x14002349); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400236c; continue; }
  pc = 0x1400234d; continue;
 }
 0x1400234d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14002353); f_140023a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  let t = cpu.r32(0x14008104u32); cpu.push(0x1400235c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400236c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002370); }
 let mut pc: u32 = 0x14002370;
 loop { match pc {
 0x14002370 => {
  cpu.push(0x14002375); f_140017f0(cpu);
  { let v: u32 = cpu.r32(0x1400599cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x14002390; continue; }
  pc = 0x1400237f; continue;
 }
 0x1400237f => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008118u32); cpu.push(0x14002386); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1400599cu32, v); }
  pc = 0x14002390; continue;
 }
 0x14002390 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140023a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140023a0); }
 let mut pc: u32 = 0x140023a0;
 loop { match pc {
 0x140023a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x14005f08u32; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140023c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140023c0); }
 let mut pc: u32 = 0x140023c0;
 loop { match pc {
 0x140023c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x14008180u32); cpu.push(0x140023c8); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1400599cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400817cu32); cpu.push(0x140023d6); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x14002423; continue; }
  pc = 0x140023dc; continue;
 }
 0x140023dc => {
  { let v = 0x74u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x140023e5); f_14003280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x14002419; continue; }
  pc = 0x140023ee; continue;
 }
 0x140023ee => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1400599cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400816cu32); cpu.push(0x140023fb); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002419; continue; }
  pc = 0x140023ff; continue;
 }
 0x140023ff => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x14002405); f_140023a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  let t = cpu.r32(0x14008104u32); cpu.push(0x1400240e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x14002423; continue;
 }
 0x14002419 => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x14002420); f_140016c0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x14002423; continue;
 }
 0x14002423 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x14008178u32); cpu.push(0x1400242a); cpu.call(t);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002430); }
 let mut pc: u32 = 0x14002430;
 loop { match pc {
 0x14002430 => {
  { cpu.sub32(cpu.r32(0x1400599cu32), 0xffffffffu32); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x140024cf; continue; }
  pc = 0x1400243e; continue;
 }
 0x1400243e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x14002458; continue; }
  pc = 0x14002446; continue;
 }
 0x14002446 => {
  { let v: u32 = cpu.r32(0x1400599cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400817cu32); cpu.push(0x14002452); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x140024c1; continue; }
  pc = 0x14002458; continue;
 }
 0x14002458 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002468; continue; }
  pc = 0x1400245f; continue;
 }
 0x1400245f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14002465); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002468; continue;
 }
 0x14002468 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002478; continue; }
  pc = 0x1400246f; continue;
 }
 0x1400246f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14002475); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002478; continue;
 }
 0x14002478 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002488; continue; }
  pc = 0x1400247f; continue;
 }
 0x1400247f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14002485); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002488; continue;
 }
 0x14002488 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002498; continue; }
  pc = 0x1400248f; continue;
 }
 0x1400248f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14002495); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002498; continue;
 }
 0x14002498 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140024a8; continue; }
  pc = 0x1400249f; continue;
 }
 0x1400249f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140024a5); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x140024a8; continue;
 }
 0x140024a8 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140024b8; continue; }
  pc = 0x140024af; continue;
 }
 0x140024af => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140024b5); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x140024b8; continue;
 }
 0x140024b8 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x140024be); f_14001390(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x140024c1; continue;
 }
 0x140024c1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1400599cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400816cu32); cpu.push(0x140024cf); cpu.call(t);
  pc = 0x140024cf; continue;
 }
 0x140024cf => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140024e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140024e0); }
 let mut pc: u32 = 0x140024e0;
 loop { match pc {
 0x140024e0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x480u32; cpu.push(v); }
  cpu.push(0x140024f1); f_140012c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x14002504; continue; }
  pc = 0x140024fa; continue;
 }
 0x140024fa => {
  { let v = 0x1bu32; cpu.push(v); }
  cpu.push(0x14002501); f_140016c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002504; continue;
 }
 0x14002504 => {
  { let v: u32 = 0x480u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x140072b0u32, v); }
  { let v: u32 = 0x20u32; cpu.w32(0x140073b0u32, v); }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x14002546; continue; }
  pc = 0x1400251e; continue;
 }
 0x1400251e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.edx = v; }
  pc = 0x14002525; continue;
 }
 0x14002525 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x24u32); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffe1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x140072b0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x480u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14002525; continue; }
  pc = 0x14002546; continue;
 }
 0x14002546 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008190u32); cpu.push(0x14002551); cpu.call(t);
  { cpu.sub16(cpu.r16(0x42u32.wrapping_add(cpu.esp)), 0x0u16); }
  if cpu.zf { pc = 0x14002637; continue; }
  pc = 0x1400255d; continue;
 }
 0x1400255d => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x14002637; continue; }
  pc = 0x14002568; continue;
 }
 0x14002568 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, 0x800u32); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x14002582; continue; }
  pc = 0x1400257d; continue;
 }
 0x1400257d => {
  { let v: u32 = 0x800u32; cpu.ebp = v; }
  pc = 0x14002582; continue;
 }
 0x14002582 => {
  { cpu.sub32(cpu.r32(0x140073b0u32), cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x140025e8; continue; }
  pc = 0x1400258a; continue;
 }
 0x1400258a => {
  { let v: u32 = 0x140072b4u32; cpu.edi = v; }
  pc = 0x1400258f; continue;
 }
 0x1400258f => {
  { let v = 0x480u32; cpu.push(v); }
  cpu.push(0x14002599); f_140012c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140025e2; continue; }
  pc = 0x140025a0; continue;
 }
 0x140025a0 => {
  { let v: u32 = 0x480u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x140073b0u32).wrapping_add(0x20u32); cpu.w32(0x140073b0u32, v); }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x140025d5; continue; }
  pc = 0x140025b3; continue;
 }
 0x140025b3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x140025b5; continue;
 }
 0x140025b5 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x24u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xau8; cpu.w8(0xffffffe1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x480u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x140025b5; continue; }
  pc = 0x140025d5; continue;
 }
 0x140025d5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x140073b0u32), cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x1400258f; continue; }
  pc = 0x140025e0; continue;
 }
 0x140025e0 => {
  pc = 0x140025e8; continue;
 }
 0x140025e2 => {
  { let v: u32 = cpu.r32(0x140073b0u32); cpu.ebp = v; }
  pc = 0x140025e8; continue;
 }
 0x140025e8 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x14002637; continue; }
  pc = 0x140025ee; continue;
 }
 0x140025ee => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1400262e; continue; }
  pc = 0x140025f5; continue;
 }
 0x140025f5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1400262e; continue; }
  pc = 0x140025fb; continue;
 }
 0x140025fb => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x1400260a; continue; }
  pc = 0x140025ff; continue;
 }
 0x140025ff => {
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x14008188u32); cpu.push(0x14002606); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400262e; continue; }
  pc = 0x1400260a; continue;
 }
 0x1400260a => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x140072b0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1400262e; continue;
 }
 0x1400262e => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x140025ee; continue; }
  pc = 0x14002637; continue;
 }
 0x14002637 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x1400263b; continue;
 }
 0x1400263b => {
  { let v: u32 = cpu.r32(0x140072b0u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1400269f; continue; }
  pc = 0x14002648; continue;
 }
 0x14002648 => {
  { let v: u32 = 0xfffffff6u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  { let v: u8 = 0x81u8; cpu.w8(0x4u32.wrapping_add(cpu.ebx), v); }
  if cpu.zf { pc = 0x14002663; continue; }
  pc = 0x14002655; continue;
 }
 0x14002655 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = 0xfffffff5u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x14002663; continue;
 }
 0x14002663 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1400818cu32); cpu.push(0x1400266a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  if cpu.zf { pc = 0x14002699; continue; }
  pc = 0x14002671; continue;
 }
 0x14002671 => {
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x14008188u32); cpu.push(0x14002678); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002699; continue; }
  pc = 0x1400267c; continue;
 }
 0x1400267c => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x1400268e; continue; }
  pc = 0x14002688; continue;
 }
 0x14002688 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)) | 0x40u8; cpu.w8(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x140026a3; continue;
 }
 0x1400268e => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x140026a3; continue; }
  pc = 0x14002693; continue;
 }
 0x14002693 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)) | 0x8u8; cpu.w8(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x140026a3; continue;
 }
 0x14002699 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)) | 0x40u8; cpu.w8(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x140026a3; continue;
 }
 0x1400269f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)) | 0x80u8; cpu.w8(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x140026a3; continue;
 }
 0x140026a3 => {
  { let v: u32 = cpu.esi.wrapping_add(0x24u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0x6cu32); }
  if (cpu.sf != cpu.of) { pc = 0x1400263b; continue; }
  pc = 0x140026ac; continue;
 }
 0x140026ac => {
  { let v: u32 = cpu.r32(0x140073b0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008184u32); cpu.push(0x140026b8); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140026c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140026c0); }
 let mut pc: u32 = 0x140026c0;
 loop { match pc {
 0x140026c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x140072b0u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1400814cu32); cpu.edi = v; }
  pc = 0x140026cf; continue;
 }
 0x140026cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x14002705; continue; }
  pc = 0x140026d5; continue;
 }
 0x140026d5 => {
  { let v: u32 = 0x480u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x140026fa; continue; }
  pc = 0x140026df; continue;
 }
 0x140026df => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x140026e1; continue;
 }
 0x140026e1 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), cpu.ebp); }
  if cpu.zf { pc = 0x140026ec; continue; }
  pc = 0x140026e6; continue;
 }
 0x140026e6 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x140026ec); cpu.call(t);
  pc = 0x140026ec; continue;
 }
 0x140026ec => {
  { let v: u32 = cpu.ebx.wrapping_add(0x24u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x480u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x140026e1; continue; }
  pc = 0x140026fa; continue;
 }
 0x140026fa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14002702); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002705; continue;
 }
 0x14002705 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x140073b0u32); }
  if cpu.cf { pc = 0x140026cf; continue; }
  pc = 0x14002710; continue;
 }
 0x14002710 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002720); }
 let mut pc: u32 = 0x14002720;
 loop { match pc {
 0x14002720 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14005058u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x1400274e; continue; }
  pc = 0x14002734; continue;
 }
 0x14002734 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x3du8); }
  if cpu.zf { pc = 0x1400273a; continue; }
  pc = 0x14002739; continue;
 }
 0x14002739 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1400273a; continue;
 }
 0x1400273a => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x14002734; continue; }
  pc = 0x1400274e; continue;
 }
 0x1400274e => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1400275b); f_140012c0(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14005978u32, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x14002773; continue; }
  pc = 0x14002769; continue;
 }
 0x14002769 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x14002770); f_140016c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002773; continue;
 }
 0x14002773 => {
  { let v: u32 = cpu.r32(0x14005058u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x140027df; continue; }
  pc = 0x14002781; continue;
 }
 0x14002781 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x3du8); }
  if cpu.zf { pc = 0x140027d5; continue; }
  pc = 0x14002798; continue;
 }
 0x14002798 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1400279e); f_140012c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140027b1; continue; }
  pc = 0x140027a7; continue;
 }
 0x140027a7 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x140027ae); f_140016c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x140027b1; continue;
 }
 0x140027b1 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x140027d5; continue;
 }
 0x140027d5 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x14002781; continue; }
  pc = 0x140027df; continue;
 }
 0x140027df => {
  { let v: u32 = cpu.r32(0x14005058u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140027ea); f_14001390(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x14005058u32, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
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
pub fn f_14002810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002810); }
 let mut pc: u32 = 0x14002810;
 loop { match pc {
 0x14002810 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  { let v: u32 = 0x14006190u32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x14008194u32); cpu.push(0x14002828); cpu.call(t);
  { let v: u32 = cpu.r32(0x140073c4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x14005988u32, v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1400283e; continue; }
  pc = 0x14002838; continue;
 }
 0x14002838 => {
  { let v: u32 = cpu.r32(0x140073c4u32); cpu.esi = v; }
  pc = 0x1400283e; continue;
 }
 0x1400283e => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14002852); f_140028b0(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14002866); f_140012c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x14002879; continue; }
  pc = 0x1400286f; continue;
 }
 0x1400286f => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x14002876); f_140016c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002879; continue;
 }
 0x14002879 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14002892); f_140028b0(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14005970u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1400596cu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140028b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140028b0); }
 let mut pc: u32 = 0x140028b0;
 loop { match pc {
 0x140028b0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  if cpu.zf { pc = 0x140028e2; continue; }
  pc = 0x140028d7; continue;
 }
 0x140028d7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x140028e2; continue;
 }
 0x140028e2 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if cpu.zf { pc = 0x1400292d; continue; }
  pc = 0x140028e7; continue;
 }
 0x140028e7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140028f2; continue; }
  pc = 0x140028ed; continue;
 }
 0x140028ed => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x140028f2; continue;
 }
 0x140028f2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x140059c9u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if cpu.zf { pc = 0x1400290e; continue; }
  pc = 0x14002902; continue;
 }
 0x14002902 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400290d; continue; }
  pc = 0x14002908; continue;
 }
 0x14002908 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1400290d; continue;
 }
 0x1400290d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1400290e; continue;
 }
 0x1400290e => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x1400291c; continue; }
  pc = 0x14002913; continue;
 }
 0x14002913 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x14002920; continue; }
  pc = 0x14002917; continue;
 }
 0x14002917 => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if !cpu.zf { pc = 0x140028e7; continue; }
  pc = 0x1400291c; continue;
 }
 0x1400291c => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x14002923; continue; }
  pc = 0x14002920; continue;
 }
 0x14002920 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  pc = 0x14002973; continue;
 }
 0x14002923 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002973; continue; }
  pc = 0x14002927; continue;
 }
 0x14002927 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  pc = 0x14002973; continue;
 }
 0x1400292d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if cpu.zf { pc = 0x14002963; continue; }
  pc = 0x14002933; continue;
 }
 0x14002933 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x14002963; continue; }
  pc = 0x14002939; continue;
 }
 0x14002939 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x140059c9u32.wrapping_add(cpu.edx)), 0x4u8); }
  if cpu.zf { pc = 0x14002952; continue; }
  pc = 0x14002946; continue;
 }
 0x14002946 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002952; continue; }
  pc = 0x1400294c; continue;
 }
 0x1400294c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x14002952; continue;
 }
 0x14002952 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400295d; continue; }
  pc = 0x14002958; continue;
 }
 0x14002958 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1400295d; continue;
 }
 0x1400295d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if !cpu.zf { pc = 0x14002933; continue; }
  pc = 0x14002963; continue;
 }
 0x14002963 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400296d; continue; }
  pc = 0x14002969; continue;
 }
 0x14002969 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1400296d; continue;
 }
 0x1400296d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if !cpu.zf { pc = 0x14002973; continue; }
  pc = 0x14002972; continue;
 }
 0x14002972 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x14002973; continue;
 }
 0x14002973 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x14002975; continue;
 }
 0x14002975 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x14002a64; continue; }
  pc = 0x1400297e; continue;
 }
 0x1400297e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x1400298a; continue; }
  pc = 0x14002985; continue;
 }
 0x14002985 => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if !cpu.zf { pc = 0x1400298d; continue; }
  pc = 0x1400298a; continue;
 }
 0x1400298a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1400297e; continue;
 }
 0x1400298d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x14002a64; continue; }
  pc = 0x14002996; continue;
 }
 0x14002996 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x140029a8; continue; }
  pc = 0x1400299d; continue;
 }
 0x1400299d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x140029a8; continue;
 }
 0x140029a8 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x140029ae; continue;
 }
 0x140029ae => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x5cu8); }
  if !cpu.zf { pc = 0x140029c1; continue; }
  pc = 0x140029ba; continue;
 }
 0x140029ba => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x5cu8); }
  if cpu.zf { pc = 0x140029ba; continue; }
  pc = 0x140029c1; continue;
 }
 0x140029c1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if !cpu.zf { pc = 0x140029e8; continue; }
  pc = 0x140029c6; continue;
 }
 0x140029c6 => {
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x140029e5; continue; }
  pc = 0x140029cb; continue;
 }
 0x140029cb => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x140029dc; continue; }
  pc = 0x140029cf; continue;
 }
 0x140029cf => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if !cpu.zf { pc = 0x140029dc; continue; }
  pc = 0x140029d8; continue;
 }
 0x140029d8 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x140029de; continue;
 }
 0x140029dc => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x140029de; continue;
 }
 0x140029de => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x140029e5; continue;
 }
 0x140029e5 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x140029e8; continue;
 }
 0x140029e8 => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x14002a00; continue; }
  pc = 0x140029ef; continue;
 }
 0x140029ef => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140029f7; continue; }
  pc = 0x140029f3; continue;
 }
 0x140029f3 => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x140029f7; continue;
 }
 0x140029f7 => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x140029ef; continue; }
  pc = 0x14002a00; continue;
 }
 0x14002a00 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x14002a55; continue; }
  pc = 0x14002a06; continue;
 }
 0x14002a06 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x14002a14; continue; }
  pc = 0x14002a0a; continue;
 }
 0x14002a0a => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x14002a55; continue; }
  pc = 0x14002a0f; continue;
 }
 0x14002a0f => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if cpu.zf { pc = 0x14002a55; continue; }
  pc = 0x14002a14; continue;
 }
 0x14002a14 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x14002a4f; continue; }
  pc = 0x14002a18; continue;
 }
 0x14002a18 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002a3d; continue; }
  pc = 0x14002a1c; continue;
 }
 0x14002a1c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x140059c9u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if cpu.zf { pc = 0x14002a2f; continue; }
  pc = 0x14002a29; continue;
 }
 0x14002a29 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x14002a2f; continue;
 }
 0x14002a2f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x140029ae; continue;
 }
 0x14002a3d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x140059c9u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if cpu.zf { pc = 0x14002a4d; continue; }
  pc = 0x14002a4a; continue;
 }
 0x14002a4a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x14002a4d; continue;
 }
 0x14002a4d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x14002a4f; continue;
 }
 0x14002a4f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x140029ae; continue;
 }
 0x14002a55 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002a5d; continue; }
  pc = 0x14002a59; continue;
 }
 0x14002a59 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x14002a5d; continue;
 }
 0x14002a5d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x14002975; continue;
 }
 0x14002a64 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x14002a75; continue; }
  pc = 0x14002a6b; continue;
 }
 0x14002a6b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x14002a75; continue;
 }
 0x14002a75 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002a80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002a80); }
 let mut pc: u32 = 0x14002a80;
 loop { match pc {
 0x14002a80 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x14002a8e); f_14001860(cpu);
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14002a9b); f_14002cb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x14005accu32), cpu.ebp); }
  if !cpu.zf { pc = 0x14002abc; continue; }
  pc = 0x14002aa8; continue;
 }
 0x14002aa8 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x14002aaf); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002abc => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x14002ad9; continue; }
  pc = 0x14002ac0; continue;
 }
 0x14002ac0 => {
  cpu.push(0x14002ac5); f_14002d60(cpu);
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x14002acc); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002ad9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x14005af0u32; cpu.eax = v; }
  pc = 0x14002ae6; continue;
 }
 0x14002ae6 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ebp); }
  if cpu.zf { pc = 0x14002b89; continue; }
  pc = 0x14002aee; continue;
 }
 0x14002aee => {
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x14005be0u32); }
  if cpu.cf { pc = 0x14002ae6; continue; }
  pc = 0x14002afc; continue;
 }
 0x14002afc => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x14008198u32); cpu.push(0x14002b08); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x14002c6a; continue; }
  pc = 0x14002b11; continue;
 }
 0x14002b11 => {
  { let v: u32 = 0x140059c8u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.stos(1, false); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x14002c3b; continue; }
  pc = 0x14002b2b; continue;
 }
 0x14002b2b => {
  { let v: u32 = 0x1au32.wrapping_add(cpu.esp); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1au32.wrapping_add(cpu.esp)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x14002b61; continue; }
  pc = 0x14002b35; continue;
 }
 0x14002b35 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x14002b61; continue; }
  pc = 0x14002b3c; continue;
 }
 0x14002b3c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x14002b59; continue; }
  pc = 0x14002b48; continue;
 }
 0x14002b48 => {
  { let v: u8 = cpu.r8(0x140059c9u32.wrapping_add(cpu.eax)) | 0x4u8; cpu.w8(0x140059c9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.cf { pc = 0x14002b48; continue; }
  pc = 0x14002b59; continue;
 }
 0x14002b59 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x14002b35; continue; }
  pc = 0x14002b61; continue;
 }
 0x14002b61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x14002b66; continue;
 }
 0x14002b66 => {
  { let v: u8 = cpu.r8(0x140059c9u32.wrapping_add(cpu.eax)) | 0x8u8; cpu.w8(0x140059c9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.cf { pc = 0x14002b66; continue; }
  pc = 0x14002b75; continue;
 }
 0x14002b75 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x14005accu32, v); }
  cpu.push(0x14002b81); f_14002d00(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14002c42; continue;
 }
 0x14002b89 => {
  { let v: u32 = 0x140059c8u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(2)); cpu.edi = v; }
  pc = 0x14002ba6; continue;
 }
 0x14002ba6 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0x14005b00u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x14002be6; continue; }
  pc = 0x14002bb5; continue;
 }
 0x14002bb5 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x14002be6; continue; }
  pc = 0x14002bbc; continue;
 }
 0x14002bbc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { pc = 0x14002bde; continue; }
  pc = 0x14002bc8; continue;
 }
 0x14002bc8 => {
  { let v: u8 = cpu.r8(0x14005ae8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x14002bce; continue;
 }
 0x14002bce => {
  { let v: u8 = cpu.r8(0x140059c9u32.wrapping_add(cpu.edx)) | (cpu.ecx as u8); cpu.w8(0x140059c9u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.cf { pc = 0x14002bce; continue; }
  pc = 0x14002bde; continue;
 }
 0x14002bde => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x14002bb5; continue; }
  pc = 0x14002be6; continue;
 }
 0x14002be6 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.cf { pc = 0x14002ba6; continue; }
  pc = 0x14002bec; continue;
 }
 0x14002bec => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x14005accu32, v); }
  cpu.push(0x14002bf8); f_14002d00(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x14005ad8u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14005ad0u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14005af8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ebx = v; }
  { let v = 0x19u32; cpu.push(v); }
  { let v: u32 = 0x14005af4u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x14002c2e); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002c3b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14005accu32, v); }
  pc = 0x14002c42; continue;
 }
 0x14002c42 => {
  { let v: u32 = 0x14005ad8u32; cpu.ecx = v; }
  { let v = 0x19u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14005ad0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x14002c5d); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002c6a => {
  { cpu.sub32(cpu.r32(0x14005ae4u32), 0x0u32); }
  if cpu.zf { pc = 0x14002c8c; continue; }
  pc = 0x14002c73; continue;
 }
 0x14002c73 => {
  cpu.push(0x14002c78); f_14002d60(cpu);
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x14002c7f); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002c8c => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x14002c93); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002cb0); }
 let mut pc: u32 = 0x14002cb0;
 loop { match pc {
 0x14002cb0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14005ae4u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffeu32); }
  if !cpu.zf { pc = 0x14002cd3; continue; }
  pc = 0x14002cc3; continue;
 }
 0x14002cc3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14005ae4u32, v); }
  pc = cpu.r32(0x140081a0u32); continue;
 }
 0x14002cd3 => {
  { cpu.sub32(cpu.eax, 0xfffffffdu32); }
  if !cpu.zf { pc = 0x14002ce8; continue; }
  pc = 0x14002cd8; continue;
 }
 0x14002cd8 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14005ae4u32, v); }
  pc = cpu.r32(0x14008174u32); continue;
 }
 0x14002ce8 => {
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if !cpu.zf { pc = 0x14002cfc; continue; }
  pc = 0x14002ced; continue;
 }
 0x14002ced => {
  { let v: u32 = 0x1u32; cpu.w32(0x14005ae4u32, v); }
  { let v: u32 = cpu.r32(0x14005fa8u32); cpu.eax = v; }
  pc = 0x14002cfc; continue;
 }
 0x14002cfc => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002d00); }
 let mut pc: u32 = 0x14002d00;
 loop { match pc {
 0x14002d00 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x3a4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x12u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14002d1d; continue; }
  pc = 0x14002d0e; continue;
 }
 0x14002d0e => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x14002d4cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x14002d38u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x14002d1d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002d20 => {
  { let v: u32 = 0x411u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002d26 => {
  { let v: u32 = 0x804u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002d2c => {
  { let v: u32 = 0x412u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002d32 => {
  { let v: u32 = 0x404u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002d38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002d38); }
 let mut pc: u32 = 0x14002d38;
 loop { match pc {
 0x14002d38 => {
  { let v: u8 = cpu.r8(0x2d261400u32) & ((cpu.ecx >> 8) as u8); cpu.w8(0x2d261400u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebp.wrapping_mul(1))).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x2d321400u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x14002du32.wrapping_add(cpu.ebx.wrapping_mul(1))).wrapping_add((cpu.edx as u8)); cpu.w8(0x14002du32.wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x14002d5f, "Int3");
  f_14002d60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002d4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002d4c); }
 let mut pc: u32 = 0x14002d4c;
 loop { match pc {
 0x14002d4c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x14002d5f, "Int3");
  f_14002d60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002d60); }
 let mut pc: u32 = 0x14002d60;
 loop { match pc {
 0x14002d60 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x140059c8u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x14005ad8u32, v); }
  { let v: u32 = 0x14005ad8u32; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14005accu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14005ad0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002d90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002d90); }
 let mut pc: u32 = 0x14002d90;
 loop { match pc {
 0x14002d90 => {
  { let v = 0xfffffffdu32; cpu.push(v); }
  cpu.push(0x14002d97); f_14002a80(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002da0); }
 let mut pc: u32 = 0x14002da0;
 loop { match pc {
 0x14002da0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x140081acu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x14005be4u32), cpu.ebx); }
  if !cpu.zf { pc = 0x14002def; continue; }
  pc = 0x14002db9; continue;
 }
 0x14002db9 => {
  let t = cpu.esi; cpu.push(0x14002dbb); cpu.call(t);
  { cpu.sub32(cpu.eax, cpu.ebx); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if cpu.zf { pc = 0x14002dcd; continue; }
  pc = 0x14002dc1; continue;
 }
 0x14002dc1 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14005be4u32, v); }
  pc = 0x14002def; continue;
 }
 0x14002dcd => {
  let t = cpu.r32(0x140081a8u32); cpu.push(0x14002dd3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x14002de5; continue; }
  pc = 0x14002dd9; continue;
 }
 0x14002dd9 => {
  { let v: u32 = 0x2u32; cpu.w32(0x14005be4u32, v); }
  pc = 0x14002def; continue;
 }
 0x14002de5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002def => {
  { cpu.sub32(cpu.r32(0x14005be4u32), 0x1u32); }
  if !cpu.zf { pc = 0x14002e9e; continue; }
  pc = 0x14002dfc; continue;
 }
 0x14002dfc => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x14002e12; continue; }
  pc = 0x14002e00; continue;
 }
 0x14002e00 => {
  let t = cpu.esi; cpu.push(0x14002e02); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x14002e12; continue; }
  pc = 0x14002e08; continue;
 }
 0x14002e08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002e12 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  if cpu.zf { pc = 0x14002e2c; continue; }
  pc = 0x14002e1a; continue;
 }
 0x14002e1a => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x14002e1a; continue; }
  pc = 0x14002e23; continue;
 }
 0x14002e23 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x14002e1a; continue; }
  pc = 0x14002e2c; continue;
 }
 0x14002e2c => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edi); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.sar32(cpu.esi, 1u32); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x140081b0u32); cpu.push(0x14002e46); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x14002e8d; continue; }
  pc = 0x14002e4c; continue;
 }
 0x14002e4c => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x14002e52); f_140012c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x14002e8d; continue; }
  pc = 0x14002e5b; continue;
 }
 0x14002e5b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x140081b0u32); cpu.push(0x14002e6d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14002e7c; continue; }
  pc = 0x14002e71; continue;
 }
 0x14002e71 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x14002e77); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x14002e7c; continue;
 }
 0x14002e7c => {
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x1400819cu32); cpu.push(0x14002e83); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002e8d => {
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x1400819cu32); cpu.push(0x14002e94); cpu.call(t);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002e9e => {
  { cpu.sub32(cpu.r32(0x14005be4u32), 0x2u32); }
  if !cpu.zf { pc = 0x14002f22; continue; }
  pc = 0x14002ea7; continue;
 }
 0x14002ea7 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x14002ec1; continue; }
  pc = 0x14002eab; continue;
 }
 0x14002eab => {
  let t = cpu.r32(0x140081a8u32); cpu.push(0x14002eb1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x14002ec1; continue; }
  pc = 0x14002eb7; continue;
 }
 0x14002eb7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002ec1 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x14002ed6; continue; }
  pc = 0x14002ec8; continue;
 }
 0x14002ec8 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x14002ec8; continue; }
  pc = 0x14002ecf; continue;
 }
 0x14002ecf => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x14002ec8; continue; }
  pc = 0x14002ed6; continue;
 }
 0x14002ed6 => {
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x14002edf); f_140012c0(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14002efb; continue; }
  pc = 0x14002eea; continue;
 }
 0x14002eea => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x140081a4u32); cpu.push(0x14002ef1); cpu.call(t);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002efb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  let t = cpu.r32(0x140081a4u32); cpu.push(0x14002f16); cpu.call(t);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14002f22 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
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
pub fn f_14002f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002f30); }
 let mut pc: u32 = 0x14002f30;
 loop { match pc {
 0x14002f30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14005068u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002f40); }
 let mut pc: u32 = 0x14002f40;
 loop { match pc {
 0x14002f40 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002f50); }
 let mut pc: u32 = 0x14002f50;
 loop { match pc {
 0x14002f50 => {
  { let v: u32 = cpu.r32(0x14005064u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x14002f67; continue; }
  pc = 0x14002f5a; continue;
 }
 0x14002f5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14002f8c; continue; }
  pc = 0x14002f5e; continue;
 }
 0x14002f5e => {
  { cpu.sub32(cpu.r32(0x14005068u32), 0x1u32); }
  if !cpu.zf { pc = 0x14002f8c; continue; }
  pc = 0x14002f67; continue;
 }
 0x14002f67 => {
  { let v = 0xfcu32; cpu.push(v); }
  cpu.push(0x14002f71); f_14002f90(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14005c78u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14002f7f; continue; }
  pc = 0x14002f7d; continue;
 }
 0x14002f7d => {
  let t = cpu.eax; cpu.push(0x14002f7f); cpu.call(t);
  pc = 0x14002f7f; continue;
 }
 0x14002f7f => {
  { let v = 0xffu32; cpu.push(v); }
  cpu.push(0x14002f89); f_14002f90(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x14002f8c; continue;
 }
 0x14002f8c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14002f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14002f90); }
 let mut pc: u32 = 0x14002f90;
 loop { match pc {
 0x14002f90 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x1a8u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x14005be8u32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x14002fa8; continue;
 }
 0x14002fa8 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); }
  if cpu.zf { pc = 0x14002fb7; continue; }
  pc = 0x14002fac; continue;
 }
 0x14002fac => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x14005c78u32); }
  if cpu.cf { pc = 0x14002fa8; continue; }
  pc = 0x14002fb7; continue;
 }
 0x14002fb7 => {
  { cpu.sub32(cpu.r32(0x14005be8u32.wrapping_add(cpu.ecx.wrapping_mul(8))), cpu.edx); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.ebx = v; }
  if !cpu.zf { pc = 0x1400317b; continue; }
  pc = 0x14002fcb; continue;
 }
 0x14002fcb => {
  { cpu.sub32(cpu.r32(0x14005064u32), 0x1u32); }
  if cpu.zf { pc = 0x14003137; continue; }
  pc = 0x14002fd8; continue;
 }
 0x14002fd8 => {
  { cpu.sub32(cpu.r32(0x14005064u32), 0x0u32); }
  if !cpu.zf { pc = 0x14002fee; continue; }
  pc = 0x14002fe1; continue;
 }
 0x14002fe1 => {
  { cpu.sub32(cpu.r32(0x14005068u32), 0x1u32); }
  if cpu.zf { pc = 0x14003137; continue; }
  pc = 0x14002fee; continue;
 }
 0x14002fee => {
  { cpu.sub32(cpu.edx, 0xfcu32); }
  if cpu.zf { pc = 0x1400317b; continue; }
  pc = 0x14002ffa; continue;
 }
 0x14002ffa => {
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14008194u32); cpu.ebp = v; }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x14003011); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1400302b; continue; }
  pc = 0x14003015; continue;
 }
 0x14003015 => {
  { let v: u32 = 0x140042f0u32; cpu.esi = v; }
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  pc = 0x1400302b; continue;
 }
 0x1400302b => {
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.ebp = v; }
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3cu32); }
  if (cpu.cf || cpu.zf) { pc = 0x1400306f; continue; }
  pc = 0x14003049; continue;
 }
 0x14003049 => {
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = 0x3u32; cpu.push(v); }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x7cu32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ebp = v; }
  { let v = 0x140042ecu32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1400306c); f_140033d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1400306f; continue;
 }
 0x1400306f => {
  { let v: u32 = 0x140042d0u32; cpu.esi = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x140042ccu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x14005becu32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = 0x12010u32; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v = 0x140042a4u32; cpu.push(v); }
  { cpu.movs(1, true); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14003129); f_14003330(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1a8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003137 => {
  { cpu.sub32(cpu.r32(0x140072b0u32), 0x0u32); }
  if cpu.zf { pc = 0x1400314d; continue; }
  pc = 0x14003140; continue;
 }
 0x14003140 => {
  { let v: u32 = cpu.r32(0x140072b0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x14003157; continue; }
  pc = 0x1400314d; continue;
 }
 0x1400314d => {
  { let v = 0xfffffff4u32; cpu.push(v); }
  let t = cpu.r32(0x1400818cu32); cpu.push(0x14003155); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x14003157; continue;
 }
 0x14003157 => {
  { let v: u32 = cpu.r32(0x14005becu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x140081b4u32); cpu.push(0x1400317b); cpu.call(t);
  pc = 0x1400317b; continue;
 }
 0x1400317b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1a8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003190); }
 let mut pc: u32 = 0x14003190;
 loop { match pc {
 0x14003190 => {
  { cpu.sub32(cpu.r32(0x140072a0u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x140031a6; continue; }
  pc = 0x1400319a; continue;
 }
 0x1400319a => {
  { let v: u32 = 0x200u32; cpu.w32(0x140072a0u32, v); }
  pc = 0x140031b9; continue;
 }
 0x140031a6 => {
  { cpu.sub32(cpu.r32(0x140072a0u32), 0x14u32); }
  if (cpu.sf == cpu.of) { pc = 0x140031b9; continue; }
  pc = 0x140031af; continue;
 }
 0x140031af => {
  { let v: u32 = 0x14u32; cpu.w32(0x140072a0u32, v); }
  pc = 0x140031b9; continue;
 }
 0x140031b9 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x140072a0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140031c6); f_14003280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14006294u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140031fb; continue; }
  pc = 0x140031d2; continue;
 }
 0x140031d2 => {
  { let v: u32 = 0x14u32; cpu.w32(0x140072a0u32, v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x140031e5); f_14003280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14006294u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140031fb; continue; }
  pc = 0x140031f1; continue;
 }
 0x140031f1 => {
  { let v = 0x1au32; cpu.push(v); }
  cpu.push(0x140031f8); f_140016c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x140031fb; continue;
 }
 0x140031fb => {
  { let v: u32 = 0x14005c80u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x14003202; continue;
 }
 0x14003202 => {
  { let v: u32 = cpu.r32(0x14006294u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x20u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x50u32); }
  if (cpu.sf != cpu.of) { pc = 0x14003202; continue; }
  pc = 0x14003217; continue;
 }
 0x14003217 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x14005c90u32; cpu.esi = v; }
  pc = 0x1400321e; continue;
 }
 0x1400321e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x140072b0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x14003243; continue; }
  pc = 0x1400323f; continue;
 }
 0x1400323f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14003249; continue; }
  pc = 0x14003243; continue;
 }
 0x14003243 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x14003249; continue;
 }
 0x14003249 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.esi, 0x14005cf0u32); }
  if cpu.cf { pc = 0x1400321e; continue; }
  pc = 0x14003255; continue;
 }
 0x14003255 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003260); }
 let mut pc: u32 = 0x14003260;
 loop { match pc {
 0x14003260 => {
  cpu.push(0x14003265); f_14003620(cpu);
  { cpu.sub8(cpu.r8(0x14005990u32), 0x0u8); }
  if cpu.zf { pc = 0x14003273; continue; }
  pc = 0x1400326e; continue;
 }
 0x1400326e => {
  pc = 0x140034d0; continue;
 }
 0x14003273 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140034d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x140034e2); f_14001860(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x140072a0u32), cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x14003557; continue; }
  pc = 0x140034ed; continue;
 }
 0x140034ed => {
  { let v: u32 = 0xcu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1400814cu32); cpu.esi = v; }
  pc = 0x140034f8; continue;
 }
 0x140034f8 => {
  { let v: u32 = cpu.r32(0x14006294u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400354b; continue; }
  pc = 0x14003504; continue;
 }
 0x14003504 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.eax)), 0x83u8); }
  if cpu.zf { pc = 0x14003519; continue; }
  pc = 0x1400350a; continue;
 }
 0x1400350a => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14003510); f_14003700(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x14003519; continue; }
  pc = 0x14003518; continue;
 }
 0x14003518 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x14003519; continue;
 }
 0x14003519 => {
  { cpu.sub32(cpu.edi, 0x50u32); }
  if (cpu.sf != cpu.of) { pc = 0x1400354b; continue; }
  pc = 0x1400351e; continue;
 }
 0x1400351e => {
  { let v: u32 = cpu.r32(0x14006294u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x1400352c); cpu.call(t);
  { let v: u32 = cpu.r32(0x14006294u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1400353b); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14006294u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x1400354b; continue;
 }
 0x1400354b => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x140072a0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x140034f8; continue; }
  pc = 0x14003557; continue;
 }
 0x14003557 => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1400355e); f_140018d0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003280); }
 let mut pc: u32 = 0x14003280;
 loop { match pc {
 0x14003280 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0xffffffe0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x140032a3; continue; }
  pc = 0x14003292; continue;
 }
 0x14003292 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1400329e; continue; }
  pc = 0x14003296; continue;
 }
 0x14003296 => {
  { let v: u32 = cpu.esi.wrapping_add(0xfu32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xfffffff0u32; cpu.esi = v; }
  pc = 0x140032a3; continue;
 }
 0x1400329e => {
  { let v: u32 = 0x10u32; cpu.esi = v; }
  pc = 0x140032a3; continue;
 }
 0x140032a3 => {
  { let v: u32 = cpu.r32(0x14008124u32); cpu.ebx = v; }
  pc = 0x140032a9; continue;
 }
 0x140032a9 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0xffffffe0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14003303; continue; }
  pc = 0x140032b0; continue;
 }
 0x140032b0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14005954u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x140032f2; continue; }
  pc = 0x140032b8; continue;
 }
 0x140032b8 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x140032bf); f_14001860(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140032cd); f_14001d30(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x140032d9); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x140032f6; continue; }
  pc = 0x140032e0; continue;
 }
 0x140032e0 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  pc = 0x140032f2; continue;
 }
 0x140032f2 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x14003324; continue; }
  pc = 0x140032f6; continue;
 }
 0x140032f6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x140073c0u32); cpu.eax = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x14003301); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x14003303; continue;
 }
 0x14003303 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x14003324; continue; }
  pc = 0x14003307; continue;
 }
 0x14003307 => {
  { cpu.sub32(cpu.r32(0x1400506cu32), 0x0u32); }
  if cpu.zf { pc = 0x14003324; continue; }
  pc = 0x14003310; continue;
 }
 0x14003310 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003316); f_14001700(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x140032a9; continue; }
  pc = 0x1400331d; continue;
 }
 0x1400331d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003324 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003330); }
 let mut pc: u32 = 0x14003330;
 loop { match pc {
 0x14003330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x14005facu32), cpu.esi); }
  if !cpu.zf { pc = 0x1400337f; continue; }
  pc = 0x1400333d; continue;
 }
 0x1400333d => {
  { let v = 0x14004338u32; cpu.push(v); }
  let t = cpu.r32(0x140081b8u32); cpu.push(0x14003348); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x140033bc; continue; }
  pc = 0x1400334e; continue;
 }
 0x1400334e => {
  { let v = 0x1400432cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14008130u32); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x1400335c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x14005facu32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140033bc; continue; }
  pc = 0x14003365; continue;
 }
 0x14003365 => {
  { let v = 0x1400431cu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x1400336d); cpu.call(t);
  { let v = 0x14004308u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14005fb0u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x1400337a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x14005fb4u32, v); }
  pc = 0x1400337f; continue;
 }
 0x1400337f => {
  { let v: u32 = cpu.r32(0x14005fb0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1400338c; continue; }
  pc = 0x14003388; continue;
 }
 0x14003388 => {
  let t = cpu.eax; cpu.push(0x1400338a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x1400338c; continue;
 }
 0x1400338c => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x140033a2; continue; }
  pc = 0x14003390; continue;
 }
 0x14003390 => {
  { cpu.sub32(cpu.r32(0x14005fb4u32), 0x0u32); }
  if cpu.zf { pc = 0x140033a2; continue; }
  pc = 0x14003399; continue;
 }
 0x14003399 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x14005fb4u32); cpu.push(0x140033a0); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x140033a2; continue;
 }
 0x140033a2 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x14005facu32); cpu.push(0x140033b8); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140033bc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140033d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140033d0); }
 let mut pc: u32 = 0x140033d0;
 loop { match pc {
 0x140033d0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x14003453; continue; }
  pc = 0x140033d9; continue;
 }
 0x140033d9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x3u32); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  if !cpu.zf { pc = 0x140033f4; continue; }
  pc = 0x140033ed; continue;
 }
 0x140033ed => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x14003461; continue; }
  pc = 0x140033f2; continue;
 }
 0x140033f2 => {
  pc = 0x14003415; continue;
 }
 0x140033f4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x14003422; continue; }
  pc = 0x140033fd; continue;
 }
 0x140033fd => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1400342a; continue; }
  pc = 0x14003401; continue;
 }
 0x14003401 => {
  { cpu.and32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x140033f4; continue; }
  pc = 0x14003409; continue;
 }
 0x14003409 => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x14003461; continue; }
  pc = 0x14003410; continue;
 }
 0x14003410 => {
  { let v: u32 = cpu.and32(cpu.ebx, 0x3u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x14003422; continue; }
  pc = 0x14003415; continue;
 }
 0x14003415 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1400344e; continue; }
  pc = 0x1400341f; continue;
 }
 0x1400341f => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x14003415; continue; }
  pc = 0x14003422; continue;
 }
 0x14003422 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400342a => {
  { cpu.and32(cpu.edi, 0x3u32); }
  if cpu.zf { pc = 0x14003444; continue; }
  pc = 0x14003432; continue;
 }
 0x14003432 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x140034c6; continue; }
  pc = 0x1400343c; continue;
 }
 0x1400343c => {
  { cpu.and32(cpu.edi, 0x3u32); }
  if !cpu.zf { pc = 0x14003432; continue; }
  pc = 0x14003444; continue;
 }
 0x14003444 => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x140034b7; continue; }
  pc = 0x1400344b; continue;
 }
 0x1400344b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1400344e; continue;
 }
 0x1400344e => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x1400344b; continue; }
  pc = 0x14003451; continue;
 }
 0x14003451 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x14003453; continue;
 }
 0x14003453 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003459 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x14003410; continue; }
  pc = 0x14003461; continue;
 }
 0x14003461 => {
  { let v: u32 = 0x7efefeffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, 0x81010100u32); }
  if cpu.zf { pc = 0x14003459; continue; }
  pc = 0x1400347b; continue;
 }
 0x1400347b => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x140034ab; continue; }
  pc = 0x1400347f; continue;
 }
 0x1400347f => {
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x140034a1; continue; }
  pc = 0x14003483; continue;
 }
 0x14003483 => {
  { cpu.and32(cpu.edx, 0xff0000u32); }
  if cpu.zf { pc = 0x14003497; continue; }
  pc = 0x1400348b; continue;
 }
 0x1400348b => {
  { cpu.and32(cpu.edx, 0xff000000u32); }
  if !cpu.zf { pc = 0x14003459; continue; }
  pc = 0x14003493; continue;
 }
 0x14003493 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x140034af; continue;
 }
 0x14003497 => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x140034af; continue;
 }
 0x140034a1 => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x140034af; continue;
 }
 0x140034ab => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x140034af; continue;
 }
 0x140034af => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x140034c1; continue; }
  pc = 0x140034b7; continue;
 }
 0x140034b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x140034b9; continue;
 }
 0x140034b9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x140034b9; continue; }
  pc = 0x140034c1; continue;
 }
 0x140034c1 => {
  { let v: u32 = cpu.and32(cpu.ebx, 0x3u32); cpu.ebx = v; }
  if !cpu.zf { pc = 0x1400344b; continue; }
  pc = 0x140034c6; continue;
 }
 0x140034c6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003570); }
 let mut pc: u32 = 0x14003570;
 loop { match pc {
 0x14003570 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1400357b); f_140035b0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14003589; continue; }
  pc = 0x14003582; continue;
 }
 0x14003582 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003589 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x140035a8; continue; }
  pc = 0x1400358f; continue;
 }
 0x1400358f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14003598); f_140037a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140035a8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140035b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140035b0); }
 let mut pc: u32 = 0x140035b0;
 loop { match pc {
 0x140035b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x14003602; continue; }
  pc = 0x140035c6; continue;
 }
 0x140035c6 => {
  { cpu.and32(cpu.eax, 0x108u32); }
  if cpu.zf { pc = 0x14003602; continue; }
  pc = 0x140035cd; continue;
 }
 0x140035cd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x14003602; continue; }
  pc = 0x140035d8; continue;
 }
 0x140035d8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x140035e3); f_14003850(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x140035f9; continue; }
  pc = 0x140035ea; continue;
 }
 0x140035ea => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x14003602; continue; }
  pc = 0x140035f1; continue;
 }
 0x140035f1 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffffffdu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x14003602; continue;
 }
 0x140035f9 => {
  { let v: u32 = cpu.or32(cpu.r32(0xcu32.wrapping_add(cpu.edi)), 0x20u32); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  pc = 0x14003602; continue;
 }
 0x14003602 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003620); }
 let mut pc: u32 = 0x14003620;
 loop { match pc {
 0x14003620 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x14003627); f_14003630(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003630); }
 let mut pc: u32 = 0x14003630;
 loop { match pc {
 0x14003630 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x14003646); f_14001860(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x140072a0u32), cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x140036da; continue; }
  pc = 0x14003655; continue;
 }
 0x14003655 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x1400365b; continue;
 }
 0x1400365b => {
  { let v: u32 = cpu.r32(0x14006294u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x140036cc; continue; }
  pc = 0x14003667; continue;
 }
 0x14003667 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.eax)), 0x83u8); }
  if cpu.zf { pc = 0x140036cc; continue; }
  pc = 0x1400366d; continue;
 }
 0x1400366d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x14003674); f_14001930(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14006294u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x83u8); }
  if cpu.zf { pc = 0x140036ba; continue; }
  pc = 0x14003686; continue;
 }
 0x14003686 => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x1400369c; continue; }
  pc = 0x1400368b; continue;
 }
 0x1400368b => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x14003691); f_14003570(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x140036ba; continue; }
  pc = 0x14003699; continue;
 }
 0x14003699 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x140036ba; continue;
 }
 0x1400369c => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x140036ba; continue; }
  pc = 0x140036a0; continue;
 }
 0x140036a0 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x140036ba; continue; }
  pc = 0x140036a4; continue;
 }
 0x140036a4 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x140036aa); f_14003570(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x140036ba; continue; }
  pc = 0x140036b2; continue;
 }
 0x140036b2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x140036ba; continue;
 }
 0x140036ba => {
  { let v: u32 = cpu.r32(0x14006294u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x140036c9); f_140019a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x140036cc; continue;
 }
 0x140036cc => {
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x140072a0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1400365b; continue; }
  pc = 0x140036d8; continue;
 }
 0x140036d8 => {
  pc = 0x140036de; continue;
 }
 0x140036da => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x140036de; continue;
 }
 0x140036de => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x140036e5); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x140036f3; continue; }
  pc = 0x140036ef; continue;
 }
 0x140036ef => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x140036f3; continue;
 }
 0x140036f3 => {
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
pub fn f_14003700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003700); }
 let mut pc: u32 = 0x14003700;
 loop { match pc {
 0x14003700 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x1400371d; continue; }
  pc = 0x14003711; continue;
 }
 0x14003711 => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400371d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003723); f_140018f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1400372c); f_14003740(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003737); f_14001960(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003740); }
 let mut pc: u32 = 0x14003740;
 loop { match pc {
 0x14003740 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x83u8); }
  if cpu.zf { pc = 0x14003793; continue; }
  pc = 0x14003751; continue;
 }
 0x14003751 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003757); f_140035b0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003762); f_14003bd0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1400376e); f_14003ad0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1400377c; continue; }
  pc = 0x14003775; continue;
 }
 0x14003775 => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x14003793; continue;
 }
 0x1400377c => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14003793; continue; }
  pc = 0x14003783; continue;
 }
 0x14003783 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14003789); f_14001390(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x14003793; continue;
 }
 0x14003793 => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140037a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140037a0); }
 let mut pc: u32 = 0x140037a0;
 loop { match pc {
 0x140037a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.r32(0x140073b0u32)); }
  if !cpu.cf { pc = 0x1400382e; continue; }
  pc = 0x140037af; continue;
 }
 0x140037af => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x140072b0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1400382e; continue; }
  pc = 0x140037d1; continue;
 }
 0x140037d1 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x140037d7); f_14003da0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1400380f; continue; }
  pc = 0x140037e3; continue;
 }
 0x140037e3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  cpu.push(0x140037ee); f_14003d50(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008170u32); cpu.push(0x140037f8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14003804; continue; }
  pc = 0x140037fc; continue;
 }
 0x140037fc => {
  let t = cpu.r32(0x14008180u32); cpu.push(0x14003802); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x14003804; continue;
 }
 0x14003804 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1400381f; continue; }
  pc = 0x14003808; continue;
 }
 0x14003808 => {
  cpu.push(0x1400380d); f_14003ca0(cpu);
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1400380f; continue;
 }
 0x1400380f => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  cpu.push(0x14003819); f_14003c90(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1400381f; continue;
 }
 0x1400381f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003825); f_14003e10(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1400382e => {
  cpu.push(0x14003833); f_14003c90(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003850); }
 let mut pc: u32 = 0x14003850;
 loop { match pc {
 0x14003850 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x140073b0u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x140038aa; continue; }
  pc = 0x1400385e; continue;
 }
 0x1400385e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x140072b0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x140038aa; continue; }
  pc = 0x1400387e; continue;
 }
 0x1400387e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003884); f_14003da0(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003897); f_140038d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x140038a2); f_14003e10(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140038aa => {
  cpu.push(0x140038af); f_14003c90(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x140038ba); f_14003ca0(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_140038d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x140038d0); }
 let mut pc: u32 = 0x140038d0;
 loop { match pc {
 0x140038d0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x418u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x434u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  if !cpu.zf { pc = 0x140038f8; continue; }
  pc = 0x140038eb; continue;
 }
 0x140038eb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x140038f8 => {
  { let v: u32 = cpu.r32(0x42cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x140072b0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x20u8); }
  if cpu.zf { pc = 0x14003939; continue; }
  pc = 0x1400392c; continue;
 }
 0x1400392c => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x14003936); f_14003e40(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x14003939; continue;
 }
 0x14003939 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ecx)), 0x80u8); }
  if cpu.zf { pc = 0x140039d5; continue; }
  pc = 0x1400394d; continue;
 }
 0x1400394d => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x430u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x1400395c; continue;
 }
 0x1400395c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x430u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.cf { pc = 0x14003a0e; continue; }
  pc = 0x1400396d; continue;
 }
 0x1400396d => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.esi = v; }
  pc = 0x14003971; continue;
 }
 0x14003971 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x430u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.cf { pc = 0x1400399c; continue; }
  pc = 0x1400397e; continue;
 }
 0x1400397e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if !cpu.zf { pc = 0x1400398a; continue; }
  pc = 0x14003985; continue;
 }
 0x14003985 => {
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1400398a; continue;
 }
 0x1400398a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x14003971; continue; }
  pc = 0x1400399c; continue;
 }
 0x1400399c => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x140081b4u32); cpu.push(0x140039c3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14003a04; continue; }
  pc = 0x140039c7; continue;
 }
 0x140039c7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)).wrapping_add(cpu.eax); cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x1400395c; continue; }
  pc = 0x140039d3; continue;
 }
 0x140039d3 => {
  pc = 0x14003a0e; continue;
 }
 0x140039d5 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x434u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x140081b4u32); cpu.push(0x140039ee); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14003a04; continue; }
  pc = 0x140039f2; continue;
 }
 0x140039f2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  pc = 0x14003a0e; continue;
 }
 0x14003a04 => {
  let t = cpu.r32(0x14008180u32); cpu.push(0x14003a0a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x14003a0e; continue;
 }
 0x14003a0e => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x14003aba; continue; }
  pc = 0x14003a19; continue;
 }
 0x14003a19 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x14003a6a; continue; }
  pc = 0x14003a20; continue;
 }
 0x14003a20 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x5u32); }
  if !cpu.zf { pc = 0x14003a4d; continue; }
  pc = 0x14003a27; continue;
 }
 0x14003a27 => {
  cpu.push(0x14003a2c); f_14003c90(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x14003a37); f_14003ca0(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003a4d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x14003a57); f_14003c10(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003a6a => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x40u8); }
  if cpu.zf { pc = 0x14003a94; continue; }
  pc = 0x14003a7b; continue;
 }
 0x14003a7b => {
  { let v: u32 = cpu.r32(0x430u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1au8); }
  if !cpu.zf { pc = 0x14003a94; continue; }
  pc = 0x14003a87; continue;
 }
 0x14003a87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003a94 => {
  cpu.push(0x14003a99); f_14003c90(cpu);
  { let v: u32 = 0x1cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x14003aa4); f_14003ca0(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003aba => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003ad0); }
 let mut pc: u32 = 0x14003ad0;
 loop { match pc {
 0x14003ad0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x140073b0u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x14003b20; continue; }
  pc = 0x14003ade; continue;
 }
 0x14003ade => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x140072b0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x14003b20; continue; }
  pc = 0x14003afe; continue;
 }
 0x14003afe => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003b04); f_14003da0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003b0d); f_14003b40(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003b18); f_14003e10(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003b20 => {
  cpu.push(0x14003b25); f_14003c90(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x14003b30); f_14003ca0(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003b40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003b40); }
 let mut pc: u32 = 0x14003b40;
 loop { match pc {
 0x14003b40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x14003b50; continue; }
  pc = 0x14003b4b; continue;
 }
 0x14003b4b => {
  { cpu.sub32(cpu.esi, 0x2u32); }
  if !cpu.zf { pc = 0x14003b6a; continue; }
  pc = 0x14003b50; continue;
 }
 0x14003b50 => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x14003b57); f_14003d50(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x14003b63); f_14003d50(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if cpu.zf { pc = 0x14003b88; continue; }
  pc = 0x14003b6a; continue;
 }
 0x14003b6a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003b70); f_14003d50(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008108u32); cpu.push(0x14003b7a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x14003b88; continue; }
  pc = 0x14003b7e; continue;
 }
 0x14003b7e => {
  let t = cpu.r32(0x14008180u32); cpu.push(0x14003b84); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x14003b8a; continue;
 }
 0x14003b88 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x14003b8a; continue;
 }
 0x14003b8a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003b90); f_14003cb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x14003ba8; continue; }
  pc = 0x14003b97; continue;
 }
 0x14003b97 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x14003b9d); f_14003c10(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003ba8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi & 0x1fu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x140072b0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003bd0); }
 let mut pc: u32 = 0x14003bd0;
 loop { match pc {
 0x14003bd0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x83u8); }
  if cpu.zf { pc = 0x14003c07; continue; }
  pc = 0x14003bdc; continue;
 }
 0x14003bdc => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x14003c07; continue; }
  pc = 0x14003be0; continue;
 }
 0x14003be0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14003be9); f_14001390(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xfffffbf7u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x14003c07; continue;
 }
 0x14003c07 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003c10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003c10); }
 let mut pc: u32 = 0x14003c10;
 loop { match pc {
 0x14003c10 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003c16); f_14003ca0(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x14005fc0u32; cpu.eax = v; }
  pc = 0x14003c23; continue;
 }
 0x14003c23 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if cpu.zf { pc = 0x14003c49; continue; }
  pc = 0x14003c27; continue;
 }
 0x14003c27 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x14006128u32); }
  if cpu.cf { pc = 0x14003c23; continue; }
  pc = 0x14003c32; continue;
 }
 0x14003c32 => {
  { cpu.sub32(cpu.ecx, 0x13u32); }
  if cpu.cf { pc = 0x14003c59; continue; }
  pc = 0x14003c37; continue;
 }
 0x14003c37 => {
  { cpu.sub32(cpu.ecx, 0x24u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14003c59; continue; }
  pc = 0x14003c3c; continue;
 }
 0x14003c3c => {
  cpu.push(0x14003c41); f_14003c90(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003c49 => {
  cpu.push(0x14003c4e); f_14003c90(cpu);
  { let v: u32 = cpu.r32(0x14005fc4u32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003c59 => {
  { cpu.sub32(cpu.ecx, 0xbcu32); }
  if cpu.cf { pc = 0x14003c76; continue; }
  pc = 0x14003c61; continue;
 }
 0x14003c61 => {
  { cpu.sub32(cpu.ecx, 0xcau32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x14003c76; continue; }
  pc = 0x14003c69; continue;
 }
 0x14003c69 => {
  cpu.push(0x14003c6e); f_14003c90(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003c76 => {
  cpu.push(0x14003c7b); f_14003c90(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003c90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003c90); }
 let mut pc: u32 = 0x14003c90;
 loop { match pc {
 0x14003c90 => {
  cpu.push(0x14003c95); f_140023c0(cpu);
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003ca0); }
 let mut pc: u32 = 0x14003ca0;
 loop { match pc {
 0x14003ca0 => {
  cpu.push(0x14003ca5); f_140023c0(cpu);
  { let v: u32 = cpu.add32(cpu.eax, 0xcu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003cb0); }
 let mut pc: u32 = 0x14003cb0;
 loop { match pc {
 0x14003cb0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x140073b0u32), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x14003d23; continue; }
  pc = 0x14003cbe; continue;
 }
 0x14003cbe => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x140072b0u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x14003d23; continue; }
  pc = 0x14003ce1; continue;
 }
 0x14003ce1 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0xffffffffu32); }
  if cpu.zf { pc = 0x14003d23; continue; }
  pc = 0x14003ce6; continue;
 }
 0x14003ce6 => {
  { cpu.sub32(cpu.r32(0x14005068u32), 0x1u32); }
  if !cpu.zf { pc = 0x14003d15; continue; }
  pc = 0x14003cef; continue;
 }
 0x14003cef => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x14003cff; continue; }
  pc = 0x14003cf3; continue;
 }
 0x14003cf3 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x14003d05; continue; }
  pc = 0x14003cf8; continue;
 }
 0x14003cf8 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x14003d0b; continue; }
  pc = 0x14003cfd; continue;
 }
 0x14003cfd => {
  pc = 0x14003d15; continue;
 }
 0x14003cff => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfffffff6u32; cpu.push(v); }
  pc = 0x14003d0f; continue;
 }
 0x14003d05 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfffffff5u32; cpu.push(v); }
  pc = 0x14003d0f; continue;
 }
 0x14003d0b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfffffff4u32; cpu.push(v); }
  pc = 0x14003d0f; continue;
 }
 0x14003d0f => {
  let t = cpu.r32(0x14008120u32); cpu.push(0x14003d15); cpu.call(t);
  pc = 0x14003d15; continue;
 }
 0x14003d15 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003d23 => {
  cpu.push(0x14003d28); f_14003c90(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x14003d33); f_14003ca0(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003d50); }
 let mut pc: u32 = 0x14003d50;
 loop { match pc {
 0x14003d50 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x140073b0u32)); }
  if !cpu.cf { pc = 0x14003d7e; continue; }
  pc = 0x14003d5c; continue;
 }
 0x14003d5c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x140072b0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x14003d7e; continue; }
  pc = 0x14003d7b; continue;
 }
 0x14003d7b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003d7e => {
  cpu.push(0x14003d83); f_14003c90(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x14003d8e); f_14003ca0(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003da0); }
 let mut pc: u32 = 0x14003da0;
 loop { match pc {
 0x14003da0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffffffe7u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x140072b0u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x14003def; continue; }
  pc = 0x14003dc8; continue;
 }
 0x14003dc8 => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x14003dcf); f_14001860(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x14003de5; continue; }
  pc = 0x14003dd8; continue;
 }
 0x14003dd8 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008148u32); cpu.push(0x14003de2); cpu.call(t);
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.ebx))); cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x14003de5; continue;
 }
 0x14003de5 => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x14003dec); f_140018d0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x14003def; continue;
 }
 0x14003def => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008150u32); cpu.push(0x14003dfd); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003e10); }
 let mut pc: u32 = 0x14003e10;
 loop { match pc {
 0x14003e10 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x140072b0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008154u32); cpu.push(0x14003e37); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_14003e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x14003e40); }
 let mut pc: u32 = 0x14003e40;
 loop { match pc {
 0x14003e40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x14003e4c); f_14003d50(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x14003e67; continue; }
  pc = 0x14003e54; continue;
 }
 0x14003e54 => {
  cpu.push(0x14003e59); f_14003c90(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003e67 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x14008168u32); cpu.push(0x14003e7a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x14003e8c; continue; }
  pc = 0x14003e86; continue;
 }
 0x14003e86 => {
  let t = cpu.r32(0x14008180u32); cpu.push(0x14003e8c); cpu.call(t);
  pc = 0x14003e8c; continue;
 }
 0x14003e8c => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x14003ea1; continue; }
  pc = 0x14003e90; continue;
 }
 0x14003e90 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x14003e96); f_14003c10(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x14003ea1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi & 0x1fu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x140072b0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1))), 0xfdu8); cpu.w8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
