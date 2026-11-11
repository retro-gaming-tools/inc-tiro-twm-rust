#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_13001000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001000); }
 let mut pc: u32 = 0x13001000;
 loop { match pc {
 0x13001000 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x13001012; continue; }
  pc = 0x13001007; continue;
 }
 0x13001007 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013124u32); cpu.push(0x13001012); cpu.call(t);
  pc = 0x13001012; continue;
 }
 0x13001012 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001020); }
 let mut pc: u32 = 0x13001020;
 loop { match pc {
 0x13001020 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x6u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001030); }
 let mut pc: u32 = 0x13001030;
 loop { match pc {
 0x13001030 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001046; continue; }
  pc = 0x1300103a; continue;
 }
 0x1300103a => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001043); f_13007280(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001046; continue;
 }
 0x13001046 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001050); }
 let mut pc: u32 = 0x13001050;
 loop { match pc {
 0x13001050 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001066; continue; }
  pc = 0x1300105a; continue;
 }
 0x1300105a => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001063); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001066; continue;
 }
 0x13001066 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001070); }
 let mut pc: u32 = 0x13001070;
 loop { match pc {
 0x13001070 => {
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x1300107a); f_13007280(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001080); }
 let mut pc: u32 = 0x13001080;
 loop { match pc {
 0x13001080 => {
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x1300108a); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001090); }
 let mut pc: u32 = 0x13001090;
 loop { match pc {
 0x13001090 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130010a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130010a0); }
 let mut pc: u32 = 0x130010a0;
 loop { match pc {
 0x130010a0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130010b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130010b0); }
 let mut pc: u32 = 0x130010b0;
 loop { match pc {
 0x130010b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130010c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130010c0); }
 let mut pc: u32 = 0x130010c0;
 loop { match pc {
 0x130010c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130010c8); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x130010d8; continue; }
  pc = 0x130010cf; continue;
 }
 0x130010cf => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130010d5); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130010d8; continue;
 }
 0x130010d8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130010e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130010e0); }
 let mut pc: u32 = 0x130010e0;
 loop { match pc {
 0x130010e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x13001113; continue; }
  pc = 0x130010f0; continue;
 }
 0x130010f0 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v = 0x130047c0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x50u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13001103); f_1300ad20(cpu);
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13001109); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13001113 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x1300111a); f_130047c0(cpu);
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x1300112b; continue; }
  pc = 0x13001122; continue;
 }
 0x13001122 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13001128); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300112b; continue;
 }
 0x1300112b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001140); }
 let mut pc: u32 = 0x13001140;
 loop { match pc {
 0x13001140 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x30u32.wrapping_add(cpu.ecx))); cpu.w32(0x30u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001150); }
 let mut pc: u32 = 0x13001150;
 loop { match pc {
 0x13001150 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300115b; continue; }
  pc = 0x13001157; continue;
 }
 0x13001157 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1300115b; continue;
 }
 0x1300115b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001160); }
 let mut pc: u32 = 0x13001160;
 loop { match pc {
 0x13001160 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001172; continue; }
  pc = 0x13001166; continue;
 }
 0x13001166 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1300116f); f_13007280(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001172; continue;
 }
 0x13001172 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001180); }
 let mut pc: u32 = 0x13001180;
 loop { match pc {
 0x13001180 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001192; continue; }
  pc = 0x13001186; continue;
 }
 0x13001186 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1300118f); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001192; continue;
 }
 0x13001192 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130011a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130011a0); }
 let mut pc: u32 = 0x130011a0;
 loop { match pc {
 0x130011a0 => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130011b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130011b0); }
 let mut pc: u32 = 0x130011b0;
 loop { match pc {
 0x130011b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x34u32); cpu.esi = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1300c004u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001220); }
 let mut pc: u32 = 0x13001220;
 loop { match pc {
 0x13001220 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x34u32); cpu.esi = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001290); }
 let mut pc: u32 = 0x13001290;
 loop { match pc {
 0x13001290 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13001298); f_13009ac0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x130012a8; continue; }
  pc = 0x1300129f; continue;
 }
 0x1300129f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130012a5); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130012a8; continue;
 }
 0x130012a8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130012b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130012b0); }
 let mut pc: u32 = 0x130012b0;
 loop { match pc {
 0x130012b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x130012e3; continue; }
  pc = 0x130012c0; continue;
 }
 0x130012c0 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v = 0x13009ac0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4cu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130012d3); f_1300ad20(cpu);
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130012d9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130012e3 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x130012ea); f_13009ac0(cpu);
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x130012fb; continue; }
  pc = 0x130012f2; continue;
 }
 0x130012f2 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130012f8); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130012fb; continue;
 }
 0x130012fb => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001310); }
 let mut pc: u32 = 0x13001310;
 loop { match pc {
 0x13001310 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x1300131d; continue; }
  pc = 0x1300131a; continue;
 }
 0x1300131a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1300131d => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001320); }
 let mut pc: u32 = 0x13001320;
 loop { match pc {
 0x13001320 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1300132d; continue; }
  pc = 0x1300132a; continue;
 }
 0x1300132a => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1300132d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001330); }
 let mut pc: u32 = 0x13001330;
 loop { match pc {
 0x13001330 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ecx)), cpu.eax); }
  if cpu.cf { pc = 0x13001346; continue; }
  pc = 0x13001338; continue;
 }
 0x13001338 => {
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.esp)) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.push(0x13001343); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13001346 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001360); }
 let mut pc: u32 = 0x13001360;
 loop { match pc {
 0x13001360 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x13001375; continue; }
  pc = 0x13001368; continue;
 }
 0x13001368 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x13001372); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13001375 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001390); }
 let mut pc: u32 = 0x13001390;
 loop { match pc {
 0x13001390 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x1300139f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130013b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130013b0); }
 let mut pc: u32 = 0x130013b0;
 loop { match pc {
 0x130013b0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x130013bf); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130013d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130013d0); }
 let mut pc: u32 = 0x130013d0;
 loop { match pc {
 0x130013d0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130013e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130013e0); }
 let mut pc: u32 = 0x130013e0;
 loop { match pc {
 0x130013e0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130013f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130013f0); }
 let mut pc: u32 = 0x130013f0;
 loop { match pc {
 0x130013f0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x130013fd; continue; }
  pc = 0x130013fa; continue;
 }
 0x130013fa => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130013fd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001400); }
 let mut pc: u32 = 0x13001400;
 loop { match pc {
 0x13001400 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001410); }
 let mut pc: u32 = 0x13001410;
 loop { match pc {
 0x13001410 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001420); }
 let mut pc: u32 = 0x13001420;
 loop { match pc {
 0x13001420 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001430); }
 let mut pc: u32 = 0x13001430;
 loop { match pc {
 0x13001430 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001440); }
 let mut pc: u32 = 0x13001440;
 loop { match pc {
 0x13001440 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001450); }
 let mut pc: u32 = 0x13001450;
 loop { match pc {
 0x13001450 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001460); }
 let mut pc: u32 = 0x13001460;
 loop { match pc {
 0x13001460 => {
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x1300146d; continue; }
  pc = 0x13001466; continue;
 }
 0x13001466 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x28u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1300146d; continue;
 }
 0x1300146d => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001470); }
 let mut pc: u32 = 0x13001470;
 loop { match pc {
 0x13001470 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x1300147d; continue; }
  pc = 0x13001476; continue;
 }
 0x13001476 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x1cu32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x1300147d; continue;
 }
 0x1300147d => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001480); }
 let mut pc: u32 = 0x13001480;
 loop { match pc {
 0x13001480 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130014a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130014a0); }
 let mut pc: u32 = 0x130014a0;
 loop { match pc {
 0x130014a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130014c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130014c0); }
 let mut pc: u32 = 0x130014c0;
 loop { match pc {
 0x130014c0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130014d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130014d0); }
 let mut pc: u32 = 0x130014d0;
 loop { match pc {
 0x130014d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130014e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130014e0); }
 let mut pc: u32 = 0x130014e0;
 loop { match pc {
 0x130014e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130014f7; continue; }
  pc = 0x130014eb; continue;
 }
 0x130014eb => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130014f4); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130014f7; continue;
 }
 0x130014f7 => {
  { let v: u32 = cpu.r32(0x1300c0d0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  if (cpu.sf == cpu.of) { pc = 0x1300151d; continue; }
  pc = 0x13001511; continue;
 }
 0x13001511 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300151a); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300151d; continue;
 }
 0x1300151d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001530); }
 let mut pc: u32 = 0x13001530;
 loop { match pc {
 0x13001530 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001547; continue; }
  pc = 0x1300153b; continue;
 }
 0x1300153b => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001544); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001547; continue;
 }
 0x13001547 => {
  { let v: u32 = cpu.r32(0x1300c0d0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x40u32; cpu.eax = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  if (cpu.sf == cpu.of) { pc = 0x1300156d; continue; }
  pc = 0x13001561; continue;
 }
 0x13001561 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300156a); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300156d; continue;
 }
 0x1300156d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001580); }
 let mut pc: u32 = 0x13001580;
 loop { match pc {
 0x13001580 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001597; continue; }
  pc = 0x1300158b; continue;
 }
 0x1300158b => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001594); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001597; continue;
 }
 0x13001597 => {
  { let v: u32 = cpu.r32(0x1300c0d0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  if (cpu.sf == cpu.of) { pc = 0x130015bd; continue; }
  pc = 0x130015b1; continue;
 }
 0x130015b1 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130015ba); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130015bd; continue;
 }
 0x130015bd => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130015d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130015d0); }
 let mut pc: u32 = 0x130015d0;
 loop { match pc {
 0x130015d0 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130015e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130015e0); }
 let mut pc: u32 = 0x130015e0;
 loop { match pc {
 0x130015e0 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x24u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130015f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130015f0); }
 let mut pc: u32 = 0x130015f0;
 loop { match pc {
 0x130015f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if (cpu.sf == cpu.of) { pc = 0x13001606; continue; }
  pc = 0x130015fa; continue;
 }
 0x130015fa => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001603); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001606; continue;
 }
 0x13001606 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ecx & cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  { let v: u32 = cpu.edx; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  if (cpu.sf == cpu.of) { pc = 0x1300162e; continue; }
  pc = 0x13001622; continue;
 }
 0x13001622 => {
  { let v: u32 = cpu.esi.wrapping_add(0x38u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300162b); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300162e; continue;
 }
 0x1300162e => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001640); }
 let mut pc: u32 = 0x13001640;
 loop { match pc {
 0x13001640 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x13001656; continue; }
  pc = 0x1300164a; continue;
 }
 0x1300164a => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001653); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001656; continue;
 }
 0x13001656 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.esi; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  if (cpu.sf == cpu.of) { pc = 0x13001674; continue; }
  pc = 0x13001668; continue;
 }
 0x13001668 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13001671); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001674; continue;
 }
 0x13001674 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001680); }
 let mut pc: u32 = 0x13001680;
 loop { match pc {
 0x13001680 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x13001696; continue; }
  pc = 0x1300168a; continue;
 }
 0x1300168a => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001693); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001696; continue;
 }
 0x13001696 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = !cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  if (cpu.sf == cpu.of) { pc = 0x130016b6; continue; }
  pc = 0x130016aa; continue;
 }
 0x130016aa => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130016b3); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130016b6; continue;
 }
 0x130016b6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130016c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130016c0); }
 let mut pc: u32 = 0x130016c0;
 loop { match pc {
 0x130016c0 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130016d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130016d0); }
 let mut pc: u32 = 0x130016d0;
 loop { match pc {
 0x130016d0 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130016e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130016e0); }
 let mut pc: u32 = 0x130016e0;
 loop { match pc {
 0x130016e0 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130016f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130016f0); }
 let mut pc: u32 = 0x130016f0;
 loop { match pc {
 0x130016f0 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001700); }
 let mut pc: u32 = 0x13001700;
 loop { match pc {
 0x13001700 => {
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001710); }
 let mut pc: u32 = 0x13001710;
 loop { match pc {
 0x13001710 => {
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2cu32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001720); }
 let mut pc: u32 = 0x13001720;
 loop { match pc {
 0x13001720 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001730); }
 let mut pc: u32 = 0x13001730;
 loop { match pc {
 0x13001730 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001740); }
 let mut pc: u32 = 0x13001740;
 loop { match pc {
 0x13001740 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001750); }
 let mut pc: u32 = 0x13001750;
 loop { match pc {
 0x13001750 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x6u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001760); }
 let mut pc: u32 = 0x13001760;
 loop { match pc {
 0x13001760 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x4u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001770); }
 let mut pc: u32 = 0x13001770;
 loop { match pc {
 0x13001770 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001785; continue; }
  pc = 0x13001779; continue;
 }
 0x13001779 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001782); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001785; continue;
 }
 0x13001785 => {
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  if (cpu.sf == cpu.of) { pc = 0x1300179e; continue; }
  pc = 0x13001792; continue;
 }
 0x13001792 => {
  { let v: u32 = cpu.esi.wrapping_add(0x38u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300179b); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300179e; continue;
 }
 0x1300179e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130017b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130017b0); }
 let mut pc: u32 = 0x130017b0;
 loop { match pc {
 0x130017b0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130017c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130017c0); }
 let mut pc: u32 = 0x130017c0;
 loop { match pc {
 0x130017c0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x6u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130017d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130017d0); }
 let mut pc: u32 = 0x130017d0;
 loop { match pc {
 0x130017d0 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130017e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130017e0); }
 let mut pc: u32 = 0x130017e0;
 loop { match pc {
 0x130017e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130017f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130017f0); }
 let mut pc: u32 = 0x130017f0;
 loop { match pc {
 0x130017f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x13012168u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001800); }
 let mut pc: u32 = 0x13001800;
 loop { match pc {
 0x13001800 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x13012168u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001810); }
 let mut pc: u32 = 0x13001810;
 loop { match pc {
 0x13001810 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x34u32.wrapping_add(cpu.eax))); cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13001821; continue; }
  pc = 0x1300181e; continue;
 }
 0x1300181e => {
  { let v: u32 = cpu.dec32(cpu.r32(0x30u32.wrapping_add(cpu.eax))); cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x13001821; continue;
 }
 0x13001821 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001830(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001830); }
 let mut pc: u32 = 0x13001830;
 loop { match pc {
 0x13001830 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1300183f; continue; }
  pc = 0x1300183b; continue;
 }
 0x1300183b => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x1300183f; continue;
 }
 0x1300183f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13001851; continue; }
  pc = 0x13001846; continue;
 }
 0x13001846 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x13001851; continue; }
  pc = 0x1300184d; continue;
 }
 0x1300184d => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.ecx), v); }
  pc = 0x13001851; continue;
 }
 0x13001851 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001860); }
 let mut pc: u32 = 0x13001860;
 loop { match pc {
 0x13001860 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001879; continue; }
  pc = 0x1300186d; continue;
 }
 0x1300186d => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001876); f_13007280(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001879; continue;
 }
 0x13001879 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001880); }
 let mut pc: u32 = 0x13001880;
 loop { match pc {
 0x13001880 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001899; continue; }
  pc = 0x1300188d; continue;
 }
 0x1300188d => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001896); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001899; continue;
 }
 0x13001899 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130018a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130018a0); }
 let mut pc: u32 = 0x130018a0;
 loop { match pc {
 0x130018a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130018be; continue; }
  pc = 0x130018b2; continue;
 }
 0x130018b2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130018bb); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130018be; continue;
 }
 0x130018be => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130018d7; continue; }
  pc = 0x130018cb; continue;
 }
 0x130018cb => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130018d4); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130018d7; continue;
 }
 0x130018d7 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130018e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130018e0); }
 let mut pc: u32 = 0x130018e0;
 loop { match pc {
 0x130018e0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130018f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130018f0); }
 let mut pc: u32 = 0x130018f0;
 loop { match pc {
 0x130018f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13001903); f_13005c70(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001910); }
 let mut pc: u32 = 0x13001910;
 loop { match pc {
 0x13001910 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13001918); f_13005c60(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300191f); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13001932; continue; }
  pc = 0x13001926; continue;
 }
 0x13001926 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300192f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001932; continue;
 }
 0x13001932 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001940); }
 let mut pc: u32 = 0x13001940;
 loop { match pc {
 0x13001940 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13001977; continue; }
  pc = 0x13001950; continue;
 }
 0x13001950 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x130019b0u32; cpu.push(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13001966); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300196c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13001977 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300197e); f_13005c60(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001985); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13001999; continue; }
  pc = 0x1300198d; continue;
 }
 0x1300198d => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001996); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001999; continue;
 }
 0x13001999 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130019b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130019b0); }
 let mut pc: u32 = 0x130019b0;
 loop { match pc {
 0x130019b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130019bb); f_13005c60(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130019c2); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130019d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130019d0); }
 let mut pc: u32 = 0x130019d0;
 loop { match pc {
 0x130019d0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(0x130019d8); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130019f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130019f0); }
 let mut pc: u32 = 0x130019f0;
 loop { match pc {
 0x130019f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13001a03; continue; }
  pc = 0x130019fc; continue;
 }
 0x130019fc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x13001a03; continue;
 }
 0x13001a03 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(0x13001a08); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001a20); }
 let mut pc: u32 = 0x13001a20;
 loop { match pc {
 0x13001a20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001a2a); f_13005e60(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001a30); }
 let mut pc: u32 = 0x13001a30;
 loop { match pc {
 0x13001a30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001a3a); f_13005e60(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001a40); }
 let mut pc: u32 = 0x13001a40;
 loop { match pc {
 0x13001a40 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001a4a); f_13005960(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001a50); }
 let mut pc: u32 = 0x13001a50;
 loop { match pc {
 0x13001a50 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001a5a); f_13005960(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001a60); }
 let mut pc: u32 = 0x13001a60;
 loop { match pc {
 0x13001a60 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001a76); f_13006b00(cpu);
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001a80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001a80); }
 let mut pc: u32 = 0x13001a80;
 loop { match pc {
 0x13001a80 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001a96); f_13006b00(cpu);
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001aa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001aa0); }
 let mut pc: u32 = 0x13001aa0;
 loop { match pc {
 0x13001aa0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001ab6); f_13006b00(cpu);
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001ac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001ac0); }
 let mut pc: u32 = 0x13001ac0;
 loop { match pc {
 0x13001ac0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001aca); f_13006a10(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001ad0); }
 let mut pc: u32 = 0x13001ad0;
 loop { match pc {
 0x13001ad0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001ada); f_13006a10(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001ae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001ae0); }
 let mut pc: u32 = 0x13001ae0;
 loop { match pc {
 0x13001ae0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001afc; continue; }
  pc = 0x13001af0; continue;
 }
 0x13001af0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001af9); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001afc; continue;
 }
 0x13001afc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001b17); f_13006b00(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001b30; continue; }
  pc = 0x13001b24; continue;
 }
 0x13001b24 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001b2d); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001b30; continue;
 }
 0x13001b30 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001b40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001b40); }
 let mut pc: u32 = 0x13001b40;
 loop { match pc {
 0x13001b40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001b5c; continue; }
  pc = 0x13001b50; continue;
 }
 0x13001b50 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001b59); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001b5c; continue;
 }
 0x13001b5c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001b77); f_13006b00(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001b90; continue; }
  pc = 0x13001b84; continue;
 }
 0x13001b84 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001b8d); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001b90; continue;
 }
 0x13001b90 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001ba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001ba0); }
 let mut pc: u32 = 0x13001ba0;
 loop { match pc {
 0x13001ba0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001bbc; continue; }
  pc = 0x13001bb0; continue;
 }
 0x13001bb0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001bb9); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001bbc; continue;
 }
 0x13001bbc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001bd7); f_13006b00(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001bf0; continue; }
  pc = 0x13001be4; continue;
 }
 0x13001be4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001bed); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001bf0; continue;
 }
 0x13001bf0 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001c00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001c00); }
 let mut pc: u32 = 0x13001c00;
 loop { match pc {
 0x13001c00 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001c1c; continue; }
  pc = 0x13001c10; continue;
 }
 0x13001c10 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001c19); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13001c1c; continue;
 }
 0x13001c1c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x13001c33); f_13006b00(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13001c4c; continue; }
  pc = 0x13001c40; continue;
 }
 0x13001c40 => {
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001c49); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001c4c; continue;
 }
 0x13001c4c => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001c60); }
 let mut pc: u32 = 0x13001c60;
 loop { match pc {
 0x13001c60 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13001c6f); f_13006a80(cpu);
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001c80); }
 let mut pc: u32 = 0x13001c80;
 loop { match pc {
 0x13001c80 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13001c8f); f_13006a80(cpu);
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001ca0); }
 let mut pc: u32 = 0x13001ca0;
 loop { match pc {
 0x13001ca0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13001cb3); f_13005c70(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001cc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001cc0); }
 let mut pc: u32 = 0x13001cc0;
 loop { match pc {
 0x13001cc0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001cca); f_13005c70(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001cd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001cd0); }
 let mut pc: u32 = 0x13001cd0;
 loop { match pc {
 0x13001cd0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13001d73u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13001d26; continue; }
  pc = 0x13001cfc; continue;
 }
 0x13001cfc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c030u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13001d10; continue; }
  pc = 0x13001d09; continue;
 }
 0x13001d09 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13001d10; continue;
 }
 0x13001d10 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13001d19); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13001d29; continue;
 }
 0x13001d26 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13001d29; continue;
 }
 0x13001d29 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13001d33); f_13005ba0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
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
pub fn f_13001d61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001d61); }
 let mut pc: u32 = 0x13001d61;
 loop { match pc {
 0x13001d61 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13001d72; continue; }
  pc = 0x13001d67; continue;
 }
 0x13001d67 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13001d72 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001d73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001d73); }
 let mut pc: u32 = 0x13001d73;
 loop { match pc {
 0x13001d73 => {
  { let v: u32 = 0x1300ca00u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001d7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001d7d); }
 let mut pc: u32 = 0x13001d7d;
 loop { match pc {
 0x13001d7d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001d90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001d90); }
 let mut pc: u32 = 0x13001d90;
 loop { match pc {
 0x13001d90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c030u32); }
  if !cpu.zf { pc = 0x13001dbd; continue; }
  pc = 0x13001da2; continue;
 }
 0x13001da2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13001db2; continue; }
  pc = 0x13001dab; continue;
 }
 0x13001dab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x13001db2; continue;
 }
 0x13001db2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  cpu.push(0x13001dbd); f_13004860(cpu);
  pc = 0x13001dbd; continue;
 }
 0x13001dbd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13001dce); f_13005c70(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001de0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001de0); }
 let mut pc: u32 = 0x13001de0;
 loop { match pc {
 0x13001de0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13001de8); f_13006340(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001def); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13001e02; continue; }
  pc = 0x13001df6; continue;
 }
 0x13001df6 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001dff); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001e02; continue;
 }
 0x13001e02 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001e10); }
 let mut pc: u32 = 0x13001e10;
 loop { match pc {
 0x13001e10 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13001e47; continue; }
  pc = 0x13001e20; continue;
 }
 0x13001e20 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13001e80u32; cpu.push(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13001e36); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13001e3c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13001e47 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001e4e); f_13006340(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001e55); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13001e69; continue; }
  pc = 0x13001e5d; continue;
 }
 0x13001e5d => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001e66); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001e69; continue;
 }
 0x13001e69 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001e80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001e80); }
 let mut pc: u32 = 0x13001e80;
 loop { match pc {
 0x13001e80 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001e8b); f_13006340(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001e92); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001ea0); }
 let mut pc: u32 = 0x13001ea0;
 loop { match pc {
 0x13001ea0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001eac); f_13006130(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001eb0); }
 let mut pc: u32 = 0x13001eb0;
 loop { match pc {
 0x13001eb0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13001ec3); f_13008290(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001ed0); }
 let mut pc: u32 = 0x13001ed0;
 loop { match pc {
 0x13001ed0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13001ed8); f_13008280(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001edf); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13001ef2; continue; }
  pc = 0x13001ee6; continue;
 }
 0x13001ee6 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001eef); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001ef2; continue;
 }
 0x13001ef2 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001f00); }
 let mut pc: u32 = 0x13001f00;
 loop { match pc {
 0x13001f00 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13001f37; continue; }
  pc = 0x13001f10; continue;
 }
 0x13001f10 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13001f70u32; cpu.push(v); }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x58u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13001f26); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13001f2c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13001f37 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001f3e); f_13008280(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001f45); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13001f59; continue; }
  pc = 0x13001f4d; continue;
 }
 0x13001f4d => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001f56); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13001f59; continue;
 }
 0x13001f59 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001f70); }
 let mut pc: u32 = 0x13001f70;
 loop { match pc {
 0x13001f70 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001f7b); f_13008280(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13001f82); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001f90); }
 let mut pc: u32 = 0x13001f90;
 loop { match pc {
 0x13001f90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(0x13001f98); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001fb0); }
 let mut pc: u32 = 0x13001fb0;
 loop { match pc {
 0x13001fb0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13001fc3; continue; }
  pc = 0x13001fbc; continue;
 }
 0x13001fbc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x13001fc3; continue;
 }
 0x13001fc3 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(0x13001fc8); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001fe0); }
 let mut pc: u32 = 0x13001fe0;
 loop { match pc {
 0x13001fe0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001fea); f_13007cb0(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13001ff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13001ff0); }
 let mut pc: u32 = 0x13001ff0;
 loop { match pc {
 0x13001ff0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13001ffa); f_13007cb0(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002000); }
 let mut pc: u32 = 0x13002000;
 loop { match pc {
 0x13002000 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300200a); f_13007fe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002010); }
 let mut pc: u32 = 0x13002010;
 loop { match pc {
 0x13002010 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300201a); f_13007fe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002020); }
 let mut pc: u32 = 0x13002020;
 loop { match pc {
 0x13002020 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x13002036); f_13007d60(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002040); }
 let mut pc: u32 = 0x13002040;
 loop { match pc {
 0x13002040 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300204a); f_13008c90(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002050); }
 let mut pc: u32 = 0x13002050;
 loop { match pc {
 0x13002050 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300205a); f_13008c90(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002060); }
 let mut pc: u32 = 0x13002060;
 loop { match pc {
 0x13002060 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1300206f); f_13008cf0(cpu);
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002080); }
 let mut pc: u32 = 0x13002080;
 loop { match pc {
 0x13002080 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1300208f); f_13008cf0(cpu);
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130020a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130020a0); }
 let mut pc: u32 = 0x130020a0;
 loop { match pc {
 0x130020a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x130020b3); f_13008290(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130020c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130020c0); }
 let mut pc: u32 = 0x130020c0;
 loop { match pc {
 0x130020c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130020ca); f_13008290(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130020d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130020d0); }
 let mut pc: u32 = 0x130020d0;
 loop { match pc {
 0x130020d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13002173u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13002126; continue; }
  pc = 0x130020fc; continue;
 }
 0x130020fc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c040u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13002110; continue; }
  pc = 0x13002109; continue;
 }
 0x13002109 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13002110; continue;
 }
 0x13002110 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13002119); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13002129; continue;
 }
 0x13002126 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13002129; continue;
 }
 0x13002129 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002133); f_130081d0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
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
pub fn f_13002161(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002161); }
 let mut pc: u32 = 0x13002161;
 loop { match pc {
 0x13002161 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13002172; continue; }
  pc = 0x13002167; continue;
 }
 0x13002167 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13002172 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002173(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002173); }
 let mut pc: u32 = 0x13002173;
 loop { match pc {
 0x13002173 => {
  { let v: u32 = 0x1300ca30u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300217d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300217d); }
 let mut pc: u32 = 0x1300217d;
 loop { match pc {
 0x1300217d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002190); }
 let mut pc: u32 = 0x13002190;
 loop { match pc {
 0x13002190 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c040u32); }
  if !cpu.zf { pc = 0x130021bd; continue; }
  pc = 0x130021a2; continue;
 }
 0x130021a2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x130021b2; continue; }
  pc = 0x130021ab; continue;
 }
 0x130021ab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x130021b2; continue;
 }
 0x130021b2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  cpu.push(0x130021bd); f_13004860(cpu);
  pc = 0x130021bd; continue;
 }
 0x130021bd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x130021ce); f_13008290(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130021e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130021e0); }
 let mut pc: u32 = 0x130021e0;
 loop { match pc {
 0x130021e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130021e8); f_130084b0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130021ef); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13002202; continue; }
  pc = 0x130021f6; continue;
 }
 0x130021f6 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130021ff); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002202; continue;
 }
 0x13002202 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002210); }
 let mut pc: u32 = 0x13002210;
 loop { match pc {
 0x13002210 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13002247; continue; }
  pc = 0x13002220; continue;
 }
 0x13002220 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13002280u32; cpu.push(v); }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x58u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13002236); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300223c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13002247 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300224e); f_130084b0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002255); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13002269; continue; }
  pc = 0x1300225d; continue;
 }
 0x1300225d => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002266); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002269; continue;
 }
 0x13002269 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002280); }
 let mut pc: u32 = 0x13002280;
 loop { match pc {
 0x13002280 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300228b); f_130084b0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002292); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130022a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130022a0); }
 let mut pc: u32 = 0x130022a0;
 loop { match pc {
 0x130022a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  f_13008010(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130022b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130022b0); }
 let mut pc: u32 = 0x130022b0;
 loop { match pc {
 0x130022b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  cpu.push(0x130022bc); f_13007cb0(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x130022c4); f_130022a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130022d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130022d0); }
 let mut pc: u32 = 0x130022d0;
 loop { match pc {
 0x130022d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x130022db); f_13007cb0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130022e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130022e0); }
 let mut pc: u32 = 0x130022e0;
 loop { match pc {
 0x130022e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130022e8); f_13005730(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130022ef); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13002302; continue; }
  pc = 0x130022f6; continue;
 }
 0x130022f6 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130022ff); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002302; continue;
 }
 0x13002302 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002310); }
 let mut pc: u32 = 0x13002310;
 loop { match pc {
 0x13002310 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13002347; continue; }
  pc = 0x13002320; continue;
 }
 0x13002320 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13002380u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13002336); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300233c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13002347 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300234e); f_13005730(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002355); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13002369; continue; }
  pc = 0x1300235d; continue;
 }
 0x1300235d => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002366); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002369; continue;
 }
 0x13002369 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002380); }
 let mut pc: u32 = 0x13002380;
 loop { match pc {
 0x13002380 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300238b); f_13005730(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002392); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130023a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130023a0); }
 let mut pc: u32 = 0x130023a0;
 loop { match pc {
 0x130023a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130023ae); f_13005c70(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  cpu.push(0x130023b7); f_13008290(cpu);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130023c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130023c0); }
 let mut pc: u32 = 0x130023c0;
 loop { match pc {
 0x130023c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x130023d7); f_13005c70(cpu);
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x130023e0); f_13008290(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130023f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130023f0); }
 let mut pc: u32 = 0x130023f0;
 loop { match pc {
 0x130023f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002400); }
 let mut pc: u32 = 0x13002400;
 loop { match pc {
 0x13002400 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13002408); f_13005950(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13002418; continue; }
  pc = 0x1300240f; continue;
 }
 0x1300240f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13002415); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002418; continue;
 }
 0x13002418 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002420); }
 let mut pc: u32 = 0x13002420;
 loop { match pc {
 0x13002420 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x13002453; continue; }
  pc = 0x13002430; continue;
 }
 0x13002430 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v = 0x13005950u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002443); f_1300ad20(cpu);
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13002449); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13002453 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x1300245a); f_13005950(cpu);
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x1300246b; continue; }
  pc = 0x13002462; continue;
 }
 0x13002462 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002468); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300246b; continue;
 }
 0x1300246b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002480); }
 let mut pc: u32 = 0x13002480;
 loop { match pc {
 0x13002480 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1300248f; continue; }
  pc = 0x1300248d; continue;
 }
 0x1300248d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x1300248f; continue;
 }
 0x1300248f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002490); }
 let mut pc: u32 = 0x13002490;
 loop { match pc {
 0x13002490 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130024a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130024a0); }
 let mut pc: u32 = 0x130024a0;
 loop { match pc {
 0x130024a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13002556u32; cpu.push(v); }
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
  { let v: u32 = 0x1300c004u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x1300c054u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002556(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002556); }
 let mut pc: u32 = 0x13002556;
 loop { match pc {
 0x13002556 => {
  { let v: u32 = 0x1300ca60u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002560); }
 let mut pc: u32 = 0x13002560;
 loop { match pc {
 0x13002560 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002570); }
 let mut pc: u32 = 0x13002570;
 loop { match pc {
 0x13002570 => {
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
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130025e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130025e0); }
 let mut pc: u32 = 0x130025e0;
 loop { match pc {
 0x130025e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130025e8); f_130031e0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x130025f8; continue; }
  pc = 0x130025ef; continue;
 }
 0x130025ef => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130025f5); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130025f8; continue;
 }
 0x130025f8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002600); }
 let mut pc: u32 = 0x13002600;
 loop { match pc {
 0x13002600 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x13002633; continue; }
  pc = 0x13002610; continue;
 }
 0x13002610 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v = 0x130031e0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x54u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002623); f_1300ad20(cpu);
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13002629); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13002633 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x1300263a); f_130031e0(cpu);
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x1300264b; continue; }
  pc = 0x13002642; continue;
 }
 0x13002642 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002648); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300264b; continue;
 }
 0x1300264b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002660); }
 let mut pc: u32 = 0x13002660;
 loop { match pc {
 0x13002660 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002670); }
 let mut pc: u32 = 0x13002670;
 loop { match pc {
 0x13002670 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x13002688; continue; }
  pc = 0x13002686; continue;
 }
 0x13002686 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x13002688; continue;
 }
 0x13002688 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002690); }
 let mut pc: u32 = 0x13002690;
 loop { match pc {
 0x13002690 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130026b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130026b0); }
 let mut pc: u32 = 0x130026b0;
 loop { match pc {
 0x130026b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x130026c3); f_130038a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130026d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130026d0); }
 let mut pc: u32 = 0x130026d0;
 loop { match pc {
 0x130026d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13002773u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13002726; continue; }
  pc = 0x130026fc; continue;
 }
 0x130026fc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c080u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13002710; continue; }
  pc = 0x13002709; continue;
 }
 0x13002709 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13002710; continue;
 }
 0x13002710 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13002719); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13002729; continue;
 }
 0x13002726 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13002729; continue;
 }
 0x13002729 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002733); f_13005ba0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c08cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_13002761(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002761); }
 let mut pc: u32 = 0x13002761;
 loop { match pc {
 0x13002761 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13002772; continue; }
  pc = 0x13002767; continue;
 }
 0x13002767 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13002772 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002773(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002773); }
 let mut pc: u32 = 0x13002773;
 loop { match pc {
 0x13002773 => {
  { let v: u32 = 0x1300ca88u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300277d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300277d); }
 let mut pc: u32 = 0x1300277d;
 loop { match pc {
 0x1300277d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002790); }
 let mut pc: u32 = 0x13002790;
 loop { match pc {
 0x13002790 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c080u32); }
  if !cpu.zf { pc = 0x130027bd; continue; }
  pc = 0x130027a2; continue;
 }
 0x130027a2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x130027b2; continue; }
  pc = 0x130027ab; continue;
 }
 0x130027ab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x130027b2; continue;
 }
 0x130027b2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  cpu.push(0x130027bd); f_13004860(cpu);
  pc = 0x130027bd; continue;
 }
 0x130027bd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x130027ce); f_13005c70(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130027e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130027e0); }
 let mut pc: u32 = 0x130027e0;
 loop { match pc {
 0x130027e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130027e8); f_13004460(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130027ef); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13002802; continue; }
  pc = 0x130027f6; continue;
 }
 0x130027f6 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130027ff); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002802; continue;
 }
 0x13002802 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002810); }
 let mut pc: u32 = 0x13002810;
 loop { match pc {
 0x13002810 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13002847; continue; }
  pc = 0x13002820; continue;
 }
 0x13002820 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13002880u32; cpu.push(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13002836); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300283c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13002847 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300284e); f_13004460(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002855); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13002869; continue; }
  pc = 0x1300285d; continue;
 }
 0x1300285d => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002866); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002869; continue;
 }
 0x13002869 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002880); }
 let mut pc: u32 = 0x13002880;
 loop { match pc {
 0x13002880 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300288b); f_13004460(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002892); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130028a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130028a0); }
 let mut pc: u32 = 0x130028a0;
 loop { match pc {
 0x130028a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130028b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130028b0); }
 let mut pc: u32 = 0x130028b0;
 loop { match pc {
 0x130028b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x130028c8; continue; }
  pc = 0x130028c6; continue;
 }
 0x130028c6 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x130028c8; continue;
 }
 0x130028c8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130028d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130028d0); }
 let mut pc: u32 = 0x130028d0;
 loop { match pc {
 0x130028d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130028f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130028f0); }
 let mut pc: u32 = 0x130028f0;
 loop { match pc {
 0x130028f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13002903); f_130038a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002910); }
 let mut pc: u32 = 0x13002910;
 loop { match pc {
 0x13002910 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130029b3u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13002966; continue; }
  pc = 0x1300293c; continue;
 }
 0x1300293c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c090u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13002950; continue; }
  pc = 0x13002949; continue;
 }
 0x13002949 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13002950; continue;
 }
 0x13002950 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13002959); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13002969; continue;
 }
 0x13002966 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13002969; continue;
 }
 0x13002969 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002973); f_130081d0(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c09cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_130029a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130029a1); }
 let mut pc: u32 = 0x130029a1;
 loop { match pc {
 0x130029a1 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x130029b2; continue; }
  pc = 0x130029a7; continue;
 }
 0x130029a7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x130029b2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130029b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130029b3); }
 let mut pc: u32 = 0x130029b3;
 loop { match pc {
 0x130029b3 => {
  { let v: u32 = 0x1300cab8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130029bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130029bd); }
 let mut pc: u32 = 0x130029bd;
 loop { match pc {
 0x130029bd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13001f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130029d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130029d0); }
 let mut pc: u32 = 0x130029d0;
 loop { match pc {
 0x130029d0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c090u32); }
  if !cpu.zf { pc = 0x130029fd; continue; }
  pc = 0x130029e2; continue;
 }
 0x130029e2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x130029f2; continue; }
  pc = 0x130029eb; continue;
 }
 0x130029eb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x130029f2; continue;
 }
 0x130029f2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  cpu.push(0x130029fd); f_13004860(cpu);
  pc = 0x130029fd; continue;
 }
 0x130029fd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x13002a0e); f_13008290(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002a20); }
 let mut pc: u32 = 0x13002a20;
 loop { match pc {
 0x13002a20 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13002a28); f_13007aa0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002a2f); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13002a42; continue; }
  pc = 0x13002a36; continue;
 }
 0x13002a36 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002a3f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002a42; continue;
 }
 0x13002a42 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002a50); }
 let mut pc: u32 = 0x13002a50;
 loop { match pc {
 0x13002a50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13002a87; continue; }
  pc = 0x13002a60; continue;
 }
 0x13002a60 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13002ac0u32; cpu.push(v); }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x58u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13002a76); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002a7c); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13002a87 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002a8e); f_13007aa0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002a95); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13002aa9; continue; }
  pc = 0x13002a9d; continue;
 }
 0x13002a9d => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002aa6); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002aa9; continue;
 }
 0x13002aa9 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002ac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002ac0); }
 let mut pc: u32 = 0x13002ac0;
 loop { match pc {
 0x13002ac0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002acb); f_13007aa0(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002ad2); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002ae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002ae0); }
 let mut pc: u32 = 0x13002ae0;
 loop { match pc {
 0x13002ae0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002af0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002af0); }
 let mut pc: u32 = 0x13002af0;
 loop { match pc {
 0x13002af0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x13002b08; continue; }
  pc = 0x13002b06; continue;
 }
 0x13002b06 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x13002b08; continue;
 }
 0x13002b08 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002b10); }
 let mut pc: u32 = 0x13002b10;
 loop { match pc {
 0x13002b10 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002b30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002b30); }
 let mut pc: u32 = 0x13002b30;
 loop { match pc {
 0x13002b30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13002b43); f_130038a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002b50); }
 let mut pc: u32 = 0x13002b50;
 loop { match pc {
 0x13002b50 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13002bfau32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13002bad; continue; }
  pc = 0x13002b7c; continue;
 }
 0x13002b7c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1300c0a8u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c0a0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.zf { pc = 0x13002b97; continue; }
  pc = 0x13002b90; continue;
 }
 0x13002b90 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  pc = 0x13002b97; continue;
 }
 0x13002b97 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  cpu.push(0x13002ba0); f_13004760(cpu);
  { let v: u32 = cpu.or32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13002bb0; continue;
 }
 0x13002bad => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x13002bb0; continue;
 }
 0x13002bb0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002bba); f_13005660(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1300c0b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_13002be8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002be8); }
 let mut pc: u32 = 0x13002be8;
 loop { match pc {
 0x13002be8 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13002bf9; continue; }
  pc = 0x13002bee; continue;
 }
 0x13002bee => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13002bf9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002bfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002bfa); }
 let mut pc: u32 = 0x13002bfa;
 loop { match pc {
 0x13002bfa => {
  { let v: u32 = 0x1300cae8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002c04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002c04); }
 let mut pc: u32 = 0x13002c04;
 loop { match pc {
 0x13002c04 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002c10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002c10); }
 let mut pc: u32 = 0x13002c10;
 loop { match pc {
 0x13002c10 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1300c0a8u32); }
  if !cpu.zf { pc = 0x13002c3d; continue; }
  pc = 0x13002c22; continue;
 }
 0x13002c22 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13002c32; continue; }
  pc = 0x13002c2b; continue;
 }
 0x13002c2b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x13002c32; continue;
 }
 0x13002c32 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.edi); cpu.ecx = v; }
  cpu.push(0x13002c3d); f_13004860(cpu);
  pc = 0x13002c3d; continue;
 }
 0x13002c3d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13002c4e); f_13005c70(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  cpu.push(0x13002c57); f_13008290(cpu);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002c60); }
 let mut pc: u32 = 0x13002c60;
 loop { match pc {
 0x13002c60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13002c68); f_13003d50(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002c6f); f_130047c0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13002c82; continue; }
  pc = 0x13002c76; continue;
 }
 0x13002c76 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002c7f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002c82; continue;
 }
 0x13002c82 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002c90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002c90); }
 let mut pc: u32 = 0x13002c90;
 loop { match pc {
 0x13002c90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13002cc7; continue; }
  pc = 0x13002ca0; continue;
 }
 0x13002ca0 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = 0x13002d00u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x13002cb6); f_1300ad20(cpu);
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13002cbc); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13002cc7 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002cce); f_13003d50(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002cd5); f_130047c0(cpu);
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x13002ce9; continue; }
  pc = 0x13002cdd; continue;
 }
 0x13002cdd => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002ce6); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13002ce9; continue;
 }
 0x13002ce9 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002d00); }
 let mut pc: u32 = 0x13002d00;
 loop { match pc {
 0x13002d00 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002d0b); f_13003d50(cpu);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002d12); f_130047c0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002d20); }
 let mut pc: u32 = 0x13002d20;
 loop { match pc {
 0x13002d20 => {
  cpu.push(0x13002d25); f_13002d30(cpu);
  pc = 0x13002db0; continue;
 }
 0x13002db0 => {
  { let v = 0x13002dc0u32; cpu.push(v); }
  cpu.push(0x13002dba); f_1300af30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002d30); }
 let mut pc: u32 = 0x13002d30;
 loop { match pc {
 0x13002d30 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13002d96u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x13002d53); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13002d74; continue; }
  pc = 0x13002d6b; continue;
 }
 0x13002d6b => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002d74); f_130030f0(cpu);
  pc = 0x13002d74; continue;
 }
 0x13002d74 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x130122c0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002d88); f_13008400(cpu);
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
pub fn f_13002d96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002d96); }
 let mut pc: u32 = 0x13002d96;
 loop { match pc {
 0x13002d96 => {
  { let v: u32 = 0x1300cb18u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002da0); }
 let mut pc: u32 = 0x13002da0;
 loop { match pc {
 0x13002da0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002da9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002dc0); }
 let mut pc: u32 = 0x13002dc0;
 loop { match pc {
 0x13002dc0 => {
  { let v: u32 = 0x130122c8u32; cpu.ecx = v; }
  cpu.push(0x13002dca); f_130084b0(cpu);
  { let v: u32 = 0x130122c8u32; cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002de0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002de0); }
 let mut pc: u32 = 0x13002de0;
 loop { match pc {
 0x13002de0 => {
  cpu.push(0x13002de5); f_13002df0(cpu);
  pc = 0x13002e20; continue;
 }
 0x13002e20 => {
  { let v = 0x13002e30u32; cpu.push(v); }
  cpu.push(0x13002e2a); f_1300af30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002df0); }
 let mut pc: u32 = 0x13002df0;
 loop { match pc {
 0x13002df0 => {
  { let v: u32 = 0x130122c0u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if cpu.zf { pc = 0x13002e0b; continue; }
  pc = 0x13002dfe; continue;
 }
 0x13002dfe => {
  { let v: u32 = cpu.r32(0x130122c0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x130122c0u32); cpu.eax = v; }
  pc = 0x13002e0b; continue;
 }
 0x13002e0b => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x130122b8u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002e18); f_130058f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002e30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002e30); }
 let mut pc: u32 = 0x13002e30;
 loop { match pc {
 0x13002e30 => {
  { let v: u32 = 0x130122b8u32; cpu.ecx = v; }
  f_13005950(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002e40); }
 let mut pc: u32 = 0x13002e40;
 loop { match pc {
 0x13002e40 => {
  cpu.push(0x13002e45); f_13002e50(cpu);
  pc = 0x13002ed0; continue;
 }
 0x13002ed0 => {
  { let v = 0x13002ee0u32; cpu.push(v); }
  cpu.push(0x13002eda); f_1300af30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002e50); }
 let mut pc: u32 = 0x13002e50;
 loop { match pc {
 0x13002e50 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13002eb6u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x13002e73); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13002e94; continue; }
  pc = 0x13002e8b; continue;
 }
 0x13002e8b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002e94); f_130030f0(cpu);
  pc = 0x13002e94; continue;
 }
 0x13002e94 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x13012318u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002ea8); f_13006290(cpu);
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
pub fn f_13002eb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002eb6); }
 let mut pc: u32 = 0x13002eb6;
 loop { match pc {
 0x13002eb6 => {
  { let v: u32 = 0x1300cb40u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002ec0); }
 let mut pc: u32 = 0x13002ec0;
 loop { match pc {
 0x13002ec0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002ec9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002ee0); }
 let mut pc: u32 = 0x13002ee0;
 loop { match pc {
 0x13002ee0 => {
  { let v: u32 = 0x13012324u32; cpu.ecx = v; }
  cpu.push(0x13002eea); f_13006340(cpu);
  { let v: u32 = 0x13012324u32; cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002f00); }
 let mut pc: u32 = 0x13002f00;
 loop { match pc {
 0x13002f00 => {
  cpu.push(0x13002f05); f_13002f10(cpu);
  pc = 0x13002f40; continue;
 }
 0x13002f40 => {
  { let v = 0x13002f50u32; cpu.push(v); }
  cpu.push(0x13002f4a); f_1300af30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002f10); }
 let mut pc: u32 = 0x13002f10;
 loop { match pc {
 0x13002f10 => {
  { let v: u32 = 0x13012318u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if cpu.zf { pc = 0x13002f2b; continue; }
  pc = 0x13002f1e; continue;
 }
 0x13002f1e => {
  { let v: u32 = cpu.r32(0x13012318u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x13012318u32); cpu.eax = v; }
  pc = 0x13002f2b; continue;
 }
 0x13002f2b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x13012374u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002f38); f_130058f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002f50); }
 let mut pc: u32 = 0x13002f50;
 loop { match pc {
 0x13002f50 => {
  { let v: u32 = 0x13012374u32; cpu.ecx = v; }
  f_13005950(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002f60); }
 let mut pc: u32 = 0x13002f60;
 loop { match pc {
 0x13002f60 => {
  cpu.push(0x13002f65); f_13002f70(cpu);
  pc = 0x13002ff0; continue;
 }
 0x13002ff0 => {
  { let v = 0x13003000u32; cpu.push(v); }
  cpu.push(0x13002ffa); f_1300af30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002f70); }
 let mut pc: u32 = 0x13002f70;
 loop { match pc {
 0x13002f70 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13002fd6u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x13002f93); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x13002fb4; continue; }
  pc = 0x13002fab; continue;
 }
 0x13002fab => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13002fb4); f_130030f0(cpu);
  pc = 0x13002fb4; continue;
 }
 0x13002fb4 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x13012380u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002fc8); f_13008400(cpu);
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
pub fn f_13002fd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002fd6); }
 let mut pc: u32 = 0x13002fd6;
 loop { match pc {
 0x13002fd6 => {
  { let v: u32 = 0x1300cb68u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13002fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13002fe0); }
 let mut pc: u32 = 0x13002fe0;
 loop { match pc {
 0x13002fe0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13002fe9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003000); }
 let mut pc: u32 = 0x13003000;
 loop { match pc {
 0x13003000 => {
  { let v: u32 = 0x13012388u32; cpu.ecx = v; }
  cpu.push(0x1300300a); f_130084b0(cpu);
  { let v: u32 = 0x13012388u32; cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003020); }
 let mut pc: u32 = 0x13003020;
 loop { match pc {
 0x13003020 => {
  cpu.push(0x13003025); f_13003030(cpu);
  pc = 0x13003060; continue;
 }
 0x13003060 => {
  { let v = 0x13003070u32; cpu.push(v); }
  cpu.push(0x1300306a); f_1300af30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003030); }
 let mut pc: u32 = 0x13003030;
 loop { match pc {
 0x13003030 => {
  { let v: u32 = 0x13012380u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if cpu.zf { pc = 0x1300304b; continue; }
  pc = 0x1300303e; continue;
 }
 0x1300303e => {
  { let v: u32 = cpu.r32(0x13012380u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x13012380u32); cpu.eax = v; }
  pc = 0x1300304b; continue;
 }
 0x1300304b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x13012378u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003058); f_130058f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003070); }
 let mut pc: u32 = 0x13003070;
 loop { match pc {
 0x13003070 => {
  { let v: u32 = 0x13012378u32; cpu.ecx = v; }
  f_13005950(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003080); }
 let mut pc: u32 = 0x13003080;
 loop { match pc {
 0x13003080 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130030d0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130030a6); f_130099f0(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c054u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_130030d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130030d0); }
 let mut pc: u32 = 0x130030d0;
 loop { match pc {
 0x130030d0 => {
  { let v: u32 = 0x1300cb90u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130030da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130030da); }
 let mut pc: u32 = 0x130030da;
 loop { match pc {
 0x130030da => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130030f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130030f0); }
 let mut pc: u32 = 0x130030f0;
 loop { match pc {
 0x130030f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13003144u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13003116); f_130099f0(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c054u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
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
pub fn f_13003144(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003144); }
 let mut pc: u32 = 0x13003144;
 loop { match pc {
 0x13003144 => {
  { let v: u32 = 0x1300cbb8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300314e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300314e); }
 let mut pc: u32 = 0x1300314e;
 loop { match pc {
 0x1300314e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003160); }
 let mut pc: u32 = 0x13003160;
 loop { match pc {
 0x13003160 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130031c9u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13003186); f_130099f0(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1300c054u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130031a2); f_130035d0(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x50u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130031c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130031c9); }
 let mut pc: u32 = 0x130031c9;
 loop { match pc {
 0x130031c9 => {
  { let v: u32 = 0x1300cbe0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130031d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130031d3); }
 let mut pc: u32 = 0x130031d3;
 loop { match pc {
 0x130031d3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130031e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130031e0); }
 let mut pc: u32 = 0x130031e0;
 loop { match pc {
 0x130031e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13003250u32; cpu.push(v); }
  { let v: u32 = 0x1300c054u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if (cpu.sf == cpu.of) { pc = 0x1300321d; continue; }
  pc = 0x13003211; continue;
 }
 0x13003211 => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300321a); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300321d; continue;
 }
 0x1300321d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x1300322f; continue; }
  pc = 0x13003226; continue;
 }
 0x13003226 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1300322d); f_13003270(cpu);
  pc = 0x13003237; continue;
 }
 0x1300322f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x13003237); f_130034c0(cpu);
  pc = 0x13003237; continue;
 }
 0x13003237 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13003243); f_1300325a(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003250); }
 let mut pc: u32 = 0x13003250;
 loop { match pc {
 0x13003250 => {
  { let v: u32 = 0x1300cc08u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300325a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300325a); }
 let mut pc: u32 = 0x1300325a;
 loop { match pc {
 0x1300325a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003270); }
 let mut pc: u32 = 0x13003270;
 loop { match pc {
 0x13003270 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if !cpu.zf { pc = 0x1300327f; continue; }
  pc = 0x1300327a; continue;
 }
 0x1300327a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1300327f => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003291; continue; }
  pc = 0x13003285; continue;
 }
 0x13003285 => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300328e); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003291; continue;
 }
 0x13003291 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x13003298); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x130131acu32); cpu.push(0x130032a4); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x130032cf; continue; }
  pc = 0x130032ac; continue;
 }
 0x130032ac => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x130032cf; continue; }
  pc = 0x130032b1; continue;
 }
 0x130032b1 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130032ca; continue; }
  pc = 0x130032be; continue;
 }
 0x130032be => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130032c7); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130032ca; continue;
 }
 0x130032ca => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130032cf => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130032e1; continue; }
  pc = 0x130032d5; continue;
 }
 0x130032d5 => {
  { let v: u32 = cpu.esi.wrapping_add(0x34u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x130032de); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130032e1; continue;
 }
 0x130032e1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130032f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130032f0); }
 let mut pc: u32 = 0x130032f0;
 loop { match pc {
 0x130032f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x130032f8); f_13009c80(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13003306; continue; }
  pc = 0x130032fd; continue;
 }
 0x130032fd => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13003306 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1300330d); f_130034c0(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300331b; continue; }
  pc = 0x13003312; continue;
 }
 0x13003312 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x1300331b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x13003331; continue; }
  pc = 0x13003322; continue;
 }
 0x13003322 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x13003331; continue;
 }
 0x13003331 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x13003352; continue; }
  pc = 0x13003338; continue;
 }
 0x13003338 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1300335b; continue; }
  pc = 0x1300333c; continue;
 }
 0x1300333c => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x1300335b; continue; }
  pc = 0x13003346; continue;
 }
 0x13003346 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x13003352; continue;
 }
 0x13003352 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x1300335b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x130131a8u32); cpu.push(0x1300336c); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x13003352; continue; }
  pc = 0x13003374; continue;
 }
 0x13003374 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003380); }
 let mut pc: u32 = 0x13003380;
 loop { match pc {
 0x13003380 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.cf { pc = 0x13003399; continue; }
  pc = 0x13003390; continue;
 }
 0x13003390 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13003399 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130033a0); f_13009c80(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130033af; continue; }
  pc = 0x130033a5; continue;
 }
 0x130033a5 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130033af => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130033b6); f_130034c0(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130033c5; continue; }
  pc = 0x130033bb; continue;
 }
 0x130033bb => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130033c5 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x130033f8; continue; }
  pc = 0x130033cb; continue;
 }
 0x130033cb => {
  { let v: u32 = 0x7u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x130131a4u32); cpu.push(0x130033dc); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x130033ed; continue; }
  pc = 0x130033e3; continue;
 }
 0x130033e3 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130033ed => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130033f8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x13003406; continue; }
  pc = 0x13003402; continue;
 }
 0x13003402 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  pc = 0x13003408; continue;
 }
 0x13003406 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x13003408; continue;
 }
 0x13003408 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x130131a4u32); cpu.push(0x13003414); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x13003425; continue; }
  pc = 0x1300341b; continue;
 }
 0x1300341b => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13003425 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003450); }
 let mut pc: u32 = 0x13003450;
 loop { match pc {
 0x13003450 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  if cpu.zf { pc = 0x13003470; continue; }
  pc = 0x1300345c; continue;
 }
 0x1300345c => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x13003474; continue; }
  pc = 0x13003461; continue;
 }
 0x13003461 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x1300347b; continue; }
  pc = 0x13003466; continue;
 }
 0x13003466 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x13003470 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x13003480; continue;
 }
 0x13003474 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x13003480; continue;
 }
 0x1300347b => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  pc = 0x13003480; continue;
 }
 0x13003480 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x13003487); f_130034c0(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x13003496; continue; }
  pc = 0x1300348c; continue;
 }
 0x1300348c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x13003496 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x130131a0u32); cpu.push(0x130034a6); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x130034b5; continue; }
  pc = 0x130034b3; continue;
 }
 0x130034b3 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x130034b5; continue;
 }
 0x130034b5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130034c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130034c0); }
 let mut pc: u32 = 0x130034c0;
 loop { match pc {
 0x130034c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130034d8; continue; }
  pc = 0x130034ce; continue;
 }
 0x130034ce => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130034d8 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x130035c5; continue; }
  pc = 0x130034e2; continue;
 }
 0x130034e2 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x130034f0; continue; }
  pc = 0x130034ec; continue;
 }
 0x130034ec => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  pc = 0x130034f2; continue;
 }
 0x130034f0 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x130034f2; continue;
 }
 0x130034f2 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13003530; continue; }
  pc = 0x130034f6; continue;
 }
 0x130034f6 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x130131a8u32); cpu.push(0x130034ff); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x13003530; continue; }
  pc = 0x13003506; continue;
 }
 0x13003506 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13003526; continue; }
  pc = 0x1300350a; continue;
 }
 0x1300350a => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x13003513; continue; }
  pc = 0x13003510; continue;
 }
 0x13003510 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x13003513; continue;
 }
 0x13003513 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x13013164u32); cpu.push(0x13003523); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x13003526; continue;
 }
 0x13003526 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x13003530 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.cf { pc = 0x13003549; continue; }
  pc = 0x13003545; continue;
 }
 0x13003545 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  pc = 0x13003549; continue;
 }
 0x13003549 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x130035a9; continue; }
  pc = 0x1300354d; continue;
 }
 0x1300354d => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1301312cu32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffffe7u32; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp & 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(8)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x80u8); }
  if cpu.zf { pc = 0x1300358b; continue; }
  pc = 0x13003576; continue;
 }
 0x13003576 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.cf { pc = 0x13003585; continue; }
  pc = 0x1300357a; continue;
 }
 0x1300357a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0xau8); }
  if !cpu.zf { pc = 0x13003580; continue; }
  pc = 0x1300357f; continue;
 }
 0x1300357f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x13003580; continue;
 }
 0x13003580 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.cf { pc = 0x1300357a; continue; }
  pc = 0x13003585; continue;
 }
 0x13003585 => {
  { cpu.and8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x1300358b; continue; }
  pc = 0x1300358a; continue;
 }
 0x1300358a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1300358b; continue;
 }
 0x1300358b => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x130131a0u32); cpu.push(0x13003597); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130035a9; continue; }
  pc = 0x1300359f; continue;
 }
 0x1300359f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x130035a9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x130035c5; continue;
 }
 0x130035c5 => {
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
pub fn f_130035d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130035d0); }
 let mut pc: u32 = 0x130035d0;
 loop { match pc {
 0x130035d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x130035e9; continue; }
  pc = 0x130035db; continue;
 }
 0x130035db => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x130035e9; continue; }
  pc = 0x130035e1; continue;
 }
 0x130035e1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x130035e9 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13003632; continue; }
  pc = 0x130035f1; continue;
 }
 0x130035f1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x13003632; continue; }
  pc = 0x130035f9; continue;
 }
 0x130035f9 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300360b; continue; }
  pc = 0x130035ff; continue;
 }
 0x130035ff => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003608); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300360b; continue;
 }
 0x1300360b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13003618); f_13009ce0(cpu);
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003639; continue; }
  pc = 0x1300361e; continue;
 }
 0x1300361e => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003627); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x13003632 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x13003639; continue;
 }
 0x13003639 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003650); }
 let mut pc: u32 = 0x13003650;
 loop { match pc {
 0x13003650 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x1300365f; continue; }
  pc = 0x13003659; continue;
 }
 0x13003659 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x1300365f => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003671; continue; }
  pc = 0x13003665; continue;
 }
 0x13003665 => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300366e); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003671; continue;
 }
 0x13003671 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x130036b4; continue; }
  pc = 0x1300367d; continue;
 }
 0x1300367d => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x130036b4; continue; }
  pc = 0x13003683; continue;
 }
 0x13003683 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x130036b4; continue; }
  pc = 0x13003689; continue;
 }
 0x13003689 => {
  { let v = 0x200u32; cpu.push(v); }
  cpu.push(0x13003693); f_1300ae80(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x130036a3; continue; }
  pc = 0x1300369a; continue;
 }
 0x1300369a => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x130036b4; continue;
 }
 0x130036a3 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x200u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130036b4); f_13009ce0(cpu);
  pc = 0x130036b4; continue;
 }
 0x130036b4 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130036c6; continue; }
  pc = 0x130036ba; continue;
 }
 0x130036ba => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130036c3); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130036c6; continue;
 }
 0x130036c6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130036d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130036d0); }
 let mut pc: u32 = 0x130036d0;
 loop { match pc {
 0x130036d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x130036e3; continue; }
  pc = 0x130036db; continue;
 }
 0x130036db => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x130036e3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x80u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffc000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8000u32); cpu.ecx = v; }
  { cpu.and32(cpu.edi, 0x20u32); }
  if !cpu.zf { pc = 0x1300370d; continue; }
  pc = 0x13003707; continue;
 }
 0x13003707 => {
  { let v: u32 = cpu.ecx | 0x100u32; cpu.ecx = v; }
  pc = 0x1300370d; continue;
 }
 0x1300370d => {
  { cpu.and32(cpu.edi, 0x40u32); }
  if cpu.zf { pc = 0x1300371b; continue; }
  pc = 0x13003715; continue;
 }
 0x13003715 => {
  { let v: u32 = cpu.ecx | 0x400u32; cpu.ecx = v; }
  pc = 0x1300371b; continue;
 }
 0x1300371b => {
  { cpu.and32(cpu.edi, 0x8u32); }
  if cpu.zf { pc = 0x13003729; continue; }
  pc = 0x13003723; continue;
 }
 0x13003723 => {
  { let v: u32 = cpu.edi | 0x2u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0x8u32; cpu.ecx = v; }
  pc = 0x13003729; continue;
 }
 0x13003729 => {
  { cpu.and32(cpu.edi, 0x10u32); }
  if cpu.zf { pc = 0x1300373a; continue; }
  pc = 0x13003731; continue;
 }
 0x13003731 => {
  { let v: u32 = cpu.edi | 0x2u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0x200u32; cpu.ecx = v; }
  pc = 0x1300373a; continue;
 }
 0x1300373a => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x13003794; continue; }
  pc = 0x13003742; continue;
 }
 0x13003742 => {
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x1300374f; continue; }
  pc = 0x1300374a; continue;
 }
 0x1300374a => {
  { let v: u32 = cpu.ecx | 0x2u32; cpu.ecx = v; }
  pc = 0x13003752; continue;
 }
 0x1300374f => {
  { let v: u32 = cpu.ecx | 0x1u32; cpu.ecx = v; }
  pc = 0x13003752; continue;
 }
 0x13003752 => {
  { cpu.and32(cpu.edi, 0x4du32); }
  if !cpu.zf { pc = 0x13003763; continue; }
  pc = 0x1300375a; continue;
 }
 0x1300375a => {
  { let v: u32 = cpu.edi | 0x10u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx | 0x200u32; cpu.ecx = v; }
  pc = 0x13003763; continue;
 }
 0x13003763 => {
  { let v: u32 = 0x40u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300c0c4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | cpu.r32(0x1300c0c0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx | cpu.r32(0x1300c0bcu32); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.eax, cpu.ebx); cpu.eax = v; }
  if cpu.zf { pc = 0x130037ce; continue; }
  pc = 0x13003782; continue;
 }
 0x13003782 => {
  { cpu.sub32(cpu.eax, 0xa00u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x130037a4; continue; }
  pc = 0x13003789; continue;
 }
 0x13003789 => {
  if cpu.zf { pc = 0x130037bb; continue; }
  pc = 0x1300378b; continue;
 }
 0x1300378b => {
  { cpu.sub32(cpu.eax, 0x800u32); }
  if cpu.zf { pc = 0x130037b4; continue; }
  pc = 0x13003792; continue;
 }
 0x13003792 => {
  pc = 0x130037ce; continue;
 }
 0x13003794 => {
  { cpu.and32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x13003763; continue; }
  pc = 0x1300379c; continue;
 }
 0x1300379c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x130037a4 => {
  { cpu.sub32(cpu.eax, 0xc00u32); }
  if cpu.zf { pc = 0x130037c2; continue; }
  pc = 0x130037ab; continue;
 }
 0x130037ab => {
  { cpu.sub32(cpu.eax, 0xe00u32); }
  if cpu.zf { pc = 0x130037c9; continue; }
  pc = 0x130037b2; continue;
 }
 0x130037b2 => {
  pc = 0x130037ce; continue;
 }
 0x130037b4 => {
  { let v: u32 = 0x10u32; cpu.edx = v; }
  pc = 0x130037ce; continue;
 }
 0x130037bb => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  pc = 0x130037ce; continue;
 }
 0x130037c2 => {
  { let v: u32 = 0x30u32; cpu.edx = v; }
  pc = 0x130037ce; continue;
 }
 0x130037c9 => {
  { let v: u32 = 0x40u32; cpu.edx = v; }
  pc = 0x130037ce; continue;
 }
 0x130037ce => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x180u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013168u32); cpu.push(0x130037e0); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x130037f3; continue; }
  pc = 0x130037eb; continue;
 }
 0x130037eb => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x130037f3 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003805; continue; }
  pc = 0x130037f9; continue;
 }
 0x130037f9 => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003802); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003805; continue;
 }
 0x13003805 => {
  { let v: u32 = 0x1u32; cpu.w32(0x50u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x13003843; continue; }
  pc = 0x13003812; continue;
 }
 0x13003812 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x13003843; continue; }
  pc = 0x13003818; continue;
 }
 0x13003818 => {
  { let v = 0x200u32; cpu.push(v); }
  cpu.push(0x13003822); f_1300ae80(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13003832; continue; }
  pc = 0x13003829; continue;
 }
 0x13003829 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x13003843; continue;
 }
 0x13003832 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x200u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13003843); f_13009ce0(cpu);
  pc = 0x13003843; continue;
 }
 0x13003843 => {
  { cpu.and32(cpu.edi, 0x4u32); }
  if cpu.zf { pc = 0x1300387d; continue; }
  pc = 0x1300384b; continue;
 }
 0x1300384b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x13003857); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1300387d; continue; }
  pc = 0x1300385c; continue;
 }
 0x1300385c => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x13003863); f_13003270(cpu);
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003875; continue; }
  pc = 0x13003869; continue;
 }
 0x13003869 => {
  { let v: u32 = cpu.esi.wrapping_add(0x34u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13003872); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003875; continue;
 }
 0x13003875 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1300387d => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300388f; continue; }
  pc = 0x13003883; continue;
 }
 0x13003883 => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300388c); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300388f; continue;
 }
 0x1300388f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130038a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130038a0); }
 let mut pc: u32 = 0x130038a0;
 loop { match pc {
 0x130038a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1300c0c8u32)); }
  if cpu.zf { pc = 0x130038c2; continue; }
  pc = 0x130038b0; continue;
 }
 0x130038b0 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300c0ccu32)); }
  if cpu.zf { pc = 0x130038c2; continue; }
  pc = 0x130038b8; continue;
 }
 0x130038b8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x130038c2 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130038d4; continue; }
  pc = 0x130038c8; continue;
 }
 0x130038c8 => {
  { let v: u32 = 0x34u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130038d1); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130038d4; continue;
 }
 0x130038d4 => {
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x130038f8; continue; }
  pc = 0x130038da; continue;
 }
 0x130038da => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x130038e1); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x130038f8; continue; }
  pc = 0x130038e6; continue;
 }
 0x130038e6 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x13013130u32); cpu.push(0x130038f1); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x130038fd; continue;
 }
 0x130038f8 => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x130038fd; continue;
 }
 0x130038fd => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300390f; continue; }
  pc = 0x13003903; continue;
 }
 0x13003903 => {
  { let v: u32 = cpu.esi.wrapping_add(0x34u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1300390c); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300390f; continue;
 }
 0x1300390f => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003920); }
 let mut pc: u32 = 0x13003920;
 loop { match pc {
 0x13003920 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13003a00u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13003968; continue; }
  pc = 0x1300394c; continue;
 }
 0x1300394c => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c0a8u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c0a0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13003961); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13003968; continue;
 }
 0x13003968 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x1300396f); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1300398e; continue; }
  pc = 0x13003987; continue;
 }
 0x13003987 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x1300398e); f_13003080(cpu);
  pc = 0x1300398e; continue;
 }
 0x1300398e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300399c); f_13005590(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c0b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_130039e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130039e1); }
 let mut pc: u32 = 0x130039e1;
 loop { match pc {
 0x130039e1 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x130039f2; continue; }
  pc = 0x130039e7; continue;
 }
 0x130039e7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x130039f2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130039f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130039f3); }
 let mut pc: u32 = 0x130039f3;
 loop { match pc {
 0x130039f3 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130039fc); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003a00); }
 let mut pc: u32 = 0x13003a00;
 loop { match pc {
 0x13003a00 => {
  { let v: u32 = 0x1300cc30u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003a0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003a0a); }
 let mut pc: u32 = 0x13003a0a;
 loop { match pc {
 0x13003a0a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003a20); }
 let mut pc: u32 = 0x13003a20;
 loop { match pc {
 0x13003a20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13003b38u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13003a68; continue; }
  pc = 0x13003a4c; continue;
 }
 0x13003a4c => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c0a8u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c0a0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13003a61); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13003a68; continue;
 }
 0x13003a68 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x13003a6f); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13003a8e; continue; }
  pc = 0x13003a87; continue;
 }
 0x13003a87 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x13003a8e); f_13003080(cpu);
  pc = 0x13003a8e; continue;
 }
 0x13003a8e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003a9c); f_13005590(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c0b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13003ae0); f_130036d0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13003afe; continue; }
  pc = 0x13003ae4; continue;
 }
 0x13003ae4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13003afe; continue;
 }
 0x13003afe => {
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
pub fn f_13003b19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003b19); }
 let mut pc: u32 = 0x13003b19;
 loop { match pc {
 0x13003b19 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13003b2a; continue; }
  pc = 0x13003b1f; continue;
 }
 0x13003b1f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13003b2a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003b2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003b2b); }
 let mut pc: u32 = 0x13003b2b;
 loop { match pc {
 0x13003b2b => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003b34); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003b38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003b38); }
 let mut pc: u32 = 0x13003b38;
 loop { match pc {
 0x13003b38 => {
  { let v: u32 = 0x1300cc68u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003b42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003b42); }
 let mut pc: u32 = 0x13003b42;
 loop { match pc {
 0x13003b42 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003b50); }
 let mut pc: u32 = 0x13003b50;
 loop { match pc {
 0x13003b50 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13003c34u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13003b98; continue; }
  pc = 0x13003b7c; continue;
 }
 0x13003b7c => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c0a8u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c0a0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13003b91); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13003b98; continue;
 }
 0x13003b98 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x13003b9f); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13003bc2; continue; }
  pc = 0x13003bb7; continue;
 }
 0x13003bb7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003bc2); f_130030f0(cpu);
  pc = 0x13003bc2; continue;
 }
 0x13003bc2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003bd0); f_13005590(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c0b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_13003c15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003c15); }
 let mut pc: u32 = 0x13003c15;
 loop { match pc {
 0x13003c15 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13003c26; continue; }
  pc = 0x13003c1b; continue;
 }
 0x13003c1b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13003c26 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003c27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003c27); }
 let mut pc: u32 = 0x13003c27;
 loop { match pc {
 0x13003c27 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003c30); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003c34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003c34); }
 let mut pc: u32 = 0x13003c34;
 loop { match pc {
 0x13003c34 => {
  { let v: u32 = 0x1300cca0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003c3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003c3e); }
 let mut pc: u32 = 0x13003c3e;
 loop { match pc {
 0x13003c3e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003c50); }
 let mut pc: u32 = 0x13003c50;
 loop { match pc {
 0x13003c50 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13003d3cu32; cpu.push(v); }
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
  if cpu.zf { pc = 0x13003c98; continue; }
  pc = 0x13003c7c; continue;
 }
 0x13003c7c => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c0a8u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1300c0a0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x13003c91); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x13003c98; continue;
 }
 0x13003c98 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x13003c9f); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x13003cca; continue; }
  pc = 0x13003cb7; continue;
 }
 0x13003cb7 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x13003cca); f_13003160(cpu);
  pc = 0x13003cca; continue;
 }
 0x13003cca => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003cd8); f_13005590(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c0b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003d1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003d1d); }
 let mut pc: u32 = 0x13003d1d;
 loop { match pc {
 0x13003d1d => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13003d2e; continue; }
  pc = 0x13003d23; continue;
 }
 0x13003d23 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13003d2e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003d2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003d2f); }
 let mut pc: u32 = 0x13003d2f;
 loop { match pc {
 0x13003d2f => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003d38); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003d3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003d3c); }
 let mut pc: u32 = 0x13003d3c;
 loop { match pc {
 0x13003d3c => {
  { let v: u32 = 0x1300ccd8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003d46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003d46); }
 let mut pc: u32 = 0x13003d46;
 loop { match pc {
 0x13003d46 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13002380(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003d50); }
 let mut pc: u32 = 0x13003d50;
 loop { match pc {
 0x13003d50 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13003d95u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c0b4u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13003d88); f_13003d9f(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003d95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003d95); }
 let mut pc: u32 = 0x13003d95;
 loop { match pc {
 0x13003d95 => {
  { let v: u32 = 0x1300cd10u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003d9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003d9f); }
 let mut pc: u32 = 0x13003d9f;
 loop { match pc {
 0x13003d9f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13005730(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003db0); }
 let mut pc: u32 = 0x13003db0;
 loop { match pc {
 0x13003db0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x13003de8; continue; }
  pc = 0x13003dc5; continue;
 }
 0x13003dc5 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.push(0x13003dd4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13003de8; continue; }
  pc = 0x13003dd8; continue;
 }
 0x13003dd8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x13003de8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003e0a; continue; }
  pc = 0x13003dfe; continue;
 }
 0x13003dfe => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003e07); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003e0a; continue;
 }
 0x13003e0a => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003e1f; continue; }
  pc = 0x13003e13; continue;
 }
 0x13003e13 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13003e1c); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003e1f; continue;
 }
 0x13003e1f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003e40; continue; }
  pc = 0x13003e34; continue;
 }
 0x13003e34 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003e3d); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003e40; continue;
 }
 0x13003e40 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003e54; continue; }
  pc = 0x13003e48; continue;
 }
 0x13003e48 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13003e51); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003e54; continue;
 }
 0x13003e54 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003e60); }
 let mut pc: u32 = 0x13003e60;
 loop { match pc {
 0x13003e60 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13003e79); f_13003650(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13003ee5; continue; }
  pc = 0x13003e7d; continue;
 }
 0x13003e7d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | 0x2u32; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003e9e; continue; }
  pc = 0x13003e92; continue;
 }
 0x13003e92 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003e9b); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003e9e; continue;
 }
 0x13003e9e => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003eb2; continue; }
  pc = 0x13003ea6; continue;
 }
 0x13003ea6 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x13003eaf); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003eb2; continue;
 }
 0x13003eb2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi | 0x2u32; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003ed1; continue; }
  pc = 0x13003ec5; continue;
 }
 0x13003ec5 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003ece); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003ed1; continue;
 }
 0x13003ed1 => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003ee5; continue; }
  pc = 0x13003ed9; continue;
 }
 0x13003ed9 => {
  { let v: u32 = cpu.esi.wrapping_add(0x38u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13003ee2); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13003ee5; continue;
 }
 0x13003ee5 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003ef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003ef0); }
 let mut pc: u32 = 0x13003ef0;
 loop { match pc {
 0x13003ef0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x13003f1d; continue; }
  pc = 0x13003f05; continue;
 }
 0x13003f05 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x13003f19); f_130036d0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13003f87; continue; }
  pc = 0x13003f1d; continue;
 }
 0x13003f1d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | 0x2u32; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003f3e; continue; }
  pc = 0x13003f32; continue;
 }
 0x13003f32 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003f3b); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003f3e; continue;
 }
 0x13003f3e => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003f52; continue; }
  pc = 0x13003f46; continue;
 }
 0x13003f46 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x13003f4f); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003f52; continue;
 }
 0x13003f52 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x2u32; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003f73; continue; }
  pc = 0x13003f67; continue;
 }
 0x13003f67 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003f70); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003f73; continue;
 }
 0x13003f73 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003f87; continue; }
  pc = 0x13003f7b; continue;
 }
 0x13003f7b => {
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x13003f84); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13003f87; continue;
 }
 0x13003f87 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13003f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13003f90); }
 let mut pc: u32 = 0x13003f90;
 loop { match pc {
 0x13003f90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13003fa4); f_13003270(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  if cpu.zf { pc = 0x1300400f; continue; }
  pc = 0x13003faa; continue;
 }
 0x13003faa => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003fc1; continue; }
  pc = 0x13003fb5; continue;
 }
 0x13003fb5 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003fbe); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003fc1; continue;
 }
 0x13003fc1 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003fda; continue; }
  pc = 0x13003fce; continue;
 }
 0x13003fce => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13003fd7); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003fda; continue;
 }
 0x13003fda => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13003ff1; continue; }
  pc = 0x13003fe5; continue;
 }
 0x13003fe5 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13003fee); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13003ff1; continue;
 }
 0x13003ff1 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13004077; continue; }
  pc = 0x13003ffe; continue;
 }
 0x13003ffe => {
  { let v: u32 = cpu.esi.wrapping_add(0x38u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13004007); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1300400f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi | 0x2u32; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300402e; continue; }
  pc = 0x13004022; continue;
 }
 0x13004022 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300402b); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300402e; continue;
 }
 0x1300402e => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13004042; continue; }
  pc = 0x13004036; continue;
 }
 0x13004036 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x38u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1300403f); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13004042; continue;
 }
 0x13004042 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13004063; continue; }
  pc = 0x13004057; continue;
 }
 0x13004057 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004060); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13004063; continue;
 }
 0x13004063 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13004077; continue; }
  pc = 0x1300406b; continue;
 }
 0x1300406b => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004074); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004077; continue;
 }
 0x13004077 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004080); }
 let mut pc: u32 = 0x13004080;
 loop { match pc {
 0x13004080 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300414fu32; cpu.push(v); }
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
  if cpu.zf { pc = 0x130040c1; continue; }
  pc = 0x130040ac; continue;
 }
 0x130040ac => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = 0x1300c080u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x130040ba); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130040c1; continue;
 }
 0x130040c1 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x130040c8); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x130040e7; continue; }
  pc = 0x130040e0; continue;
 }
 0x130040e0 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130040e7); f_13003080(cpu);
  pc = 0x130040e7; continue;
 }
 0x130040e7 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130040f5); f_13005af0(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c08cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
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
pub fn f_13004130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004130); }
 let mut pc: u32 = 0x13004130;
 loop { match pc {
 0x13004130 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13004141; continue; }
  pc = 0x13004136; continue;
 }
 0x13004136 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13004141 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004142(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004142); }
 let mut pc: u32 = 0x13004142;
 loop { match pc {
 0x13004142 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300414b); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300414f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300414f); }
 let mut pc: u32 = 0x1300414f;
 loop { match pc {
 0x1300414f => {
  { let v: u32 = 0x1300cd38u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004159(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004159); }
 let mut pc: u32 = 0x13004159;
 loop { match pc {
 0x13004159 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004170); }
 let mut pc: u32 = 0x13004170;
 loop { match pc {
 0x13004170 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300426au32; cpu.push(v); }
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
  if cpu.zf { pc = 0x130041b1; continue; }
  pc = 0x1300419c; continue;
 }
 0x1300419c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c080u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130041aa); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130041b1; continue;
 }
 0x130041b1 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x130041b8); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x130041d7; continue; }
  pc = 0x130041d0; continue;
 }
 0x130041d0 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x130041d7); f_13003080(cpu);
  pc = 0x130041d7; continue;
 }
 0x130041d7 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130041e5); f_13005af0(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c08cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13004222); f_130036d0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13004230; continue; }
  pc = 0x13004226; continue;
 }
 0x13004226 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x2u32); cpu.w32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  pc = 0x13004230; continue;
 }
 0x13004230 => {
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
pub fn f_1300424b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300424b); }
 let mut pc: u32 = 0x1300424b;
 loop { match pc {
 0x1300424b => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300425c; continue; }
  pc = 0x13004251; continue;
 }
 0x13004251 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300425c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300425d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300425d); }
 let mut pc: u32 = 0x1300425d;
 loop { match pc {
 0x1300425d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004266); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300426a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300426a); }
 let mut pc: u32 = 0x1300426a;
 loop { match pc {
 0x1300426a => {
  { let v: u32 = 0x1300cd70u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004274(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004274); }
 let mut pc: u32 = 0x13004274;
 loop { match pc {
 0x13004274 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004280); }
 let mut pc: u32 = 0x13004280;
 loop { match pc {
 0x13004280 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x13004353u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x130042c1; continue; }
  pc = 0x130042ac; continue;
 }
 0x130042ac => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x1300c080u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x130042ba); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130042c1; continue;
 }
 0x130042c1 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x130042c8); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x130042eb; continue; }
  pc = 0x130042e0; continue;
 }
 0x130042e0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130042eb); f_130030f0(cpu);
  pc = 0x130042eb; continue;
 }
 0x130042eb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130042f9); f_13005af0(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c08cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
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
pub fn f_13004334(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004334); }
 let mut pc: u32 = 0x13004334;
 loop { match pc {
 0x13004334 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x13004345; continue; }
  pc = 0x1300433a; continue;
 }
 0x1300433a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x13004345 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004346(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004346); }
 let mut pc: u32 = 0x13004346;
 loop { match pc {
 0x13004346 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300434f); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004353(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004353); }
 let mut pc: u32 = 0x13004353;
 loop { match pc {
 0x13004353 => {
  { let v: u32 = 0x1300cda8u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300435d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300435d); }
 let mut pc: u32 = 0x1300435d;
 loop { match pc {
 0x1300435d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004370); }
 let mut pc: u32 = 0x13004370;
 loop { match pc {
 0x13004370 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1300444bu32; cpu.push(v); }
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
  if cpu.zf { pc = 0x130043b1; continue; }
  pc = 0x1300439c; continue;
 }
 0x1300439c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = 0x1300c080u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x130043aa); f_13004670(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x130043b1; continue;
 }
 0x130043b1 => {
  { let v = 0x54u32; cpu.push(v); }
  cpu.push(0x130043b8); f_1300ae80(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x130043e3; continue; }
  pc = 0x130043d0; continue;
 }
 0x130043d0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x130043e3); f_13003160(cpu);
  pc = 0x130043e3; continue;
 }
 0x130043e3 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130043f1); f_13005af0(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x1300c08cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
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
pub fn f_1300442c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300442c); }
 let mut pc: u32 = 0x1300442c;
 loop { match pc {
 0x1300442c => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1300443d; continue; }
  pc = 0x13004432; continue;
 }
 0x13004432 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_130047c0(cpu); return;
 }
 0x1300443d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300443e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300443e); }
 let mut pc: u32 = 0x1300443e;
 loop { match pc {
 0x1300443e => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004447); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1300444b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1300444b); }
 let mut pc: u32 = 0x1300444b;
 loop { match pc {
 0x1300444b => {
  { let v: u32 = 0x1300cde0u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004455(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004455); }
 let mut pc: u32 = 0x13004455;
 loop { match pc {
 0x13004455 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_130019b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004460); }
 let mut pc: u32 = 0x13004460;
 loop { match pc {
 0x13004460 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130044a5u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1300c08cu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x13004498); f_130044af(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130044a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130044a5); }
 let mut pc: u32 = 0x130044a5;
 loop { match pc {
 0x130044a5 => {
  { let v: u32 = 0x1300ce18u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130044af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130044af); }
 let mut pc: u32 = 0x130044af;
 loop { match pc {
 0x130044af => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13005c60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130044c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130044c0); }
 let mut pc: u32 = 0x130044c0;
 loop { match pc {
 0x130044c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x130044f6; continue; }
  pc = 0x130044d4; continue;
 }
 0x130044d4 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.push(0x130044e3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x130044f6; continue; }
  pc = 0x130044e7; continue;
 }
 0x130044e7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x130044f6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13004517; continue; }
  pc = 0x1300450b; continue;
 }
 0x1300450b => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004514); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x13004517; continue;
 }
 0x13004517 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300452b; continue; }
  pc = 0x1300451f; continue;
 }
 0x1300451f => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004528); f_13007290(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300452b; continue;
 }
 0x1300452b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004540); }
 let mut pc: u32 = 0x13004540;
 loop { match pc {
 0x13004540 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13004558); f_13003650(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13004591; continue; }
  pc = 0x1300455c; continue;
 }
 0x1300455c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300457d; continue; }
  pc = 0x13004571; continue;
 }
 0x13004571 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300457a); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300457d; continue;
 }
 0x1300457d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13004591; continue; }
  pc = 0x13004585; continue;
 }
 0x13004585 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1300458e); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004591; continue;
 }
 0x13004591 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130045a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130045a0); }
 let mut pc: u32 = 0x130045a0;
 loop { match pc {
 0x130045a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4cu32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x130045cf; continue; }
  pc = 0x130045b4; continue;
 }
 0x130045b4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x130045cb); f_130036d0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13004604; continue; }
  pc = 0x130045cf; continue;
 }
 0x130045cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x2u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x130045f0; continue; }
  pc = 0x130045e4; continue;
 }
 0x130045e4 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130045ed); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130045f0; continue;
 }
 0x130045f0 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.edi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13004604; continue; }
  pc = 0x130045f8; continue;
 }
 0x130045f8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004601); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004604; continue;
 }
 0x13004604 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004610); }
 let mut pc: u32 = 0x13004610;
 loop { match pc {
 0x13004610 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  cpu.push(0x13004627); f_13003270(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x13004637; continue; }
  pc = 0x1300462b; continue;
 }
 0x1300462b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.edi = v; }
  { let v: u32 = cpu.edi | 0x2u32; cpu.edi = v; }
  pc = 0x13004637; continue;
 }
 0x13004637 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1300464e; continue; }
  pc = 0x13004642; continue;
 }
 0x13004642 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1300464b); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1300464e; continue;
 }
 0x1300464e => {
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x13004663; continue; }
  pc = 0x13004657; continue;
 }
 0x13004657 => {
  { let v: u32 = cpu.esi.wrapping_add(0x38u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13004660); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004663; continue;
 }
 0x13004663 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004670); }
 let mut pc: u32 = 0x13004670;
 loop { match pc {
 0x13004670 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x1300c0e4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x30u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x6u32; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x20u8; cpu.w8(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130046b3); f_13007260(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x13012188u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x13012188u32)); cpu.w32(0x13012188u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x130046d2; continue; }
  pc = 0x130046c5; continue;
 }
 0x130046c5 => {
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x130046cf); f_13007260(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130046d2; continue;
 }
 0x130046d2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130046e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130046e0); }
 let mut pc: u32 = 0x130046e0;
 loop { match pc {
 0x130046e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1300c0e4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x6u32; cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x20u8; cpu.w8(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1300472b); f_13007260(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x13012188u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x13012188u32)); cpu.w32(0x13012188u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1300474a; continue; }
  pc = 0x1300473d; continue;
 }
 0x1300473d => {
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x13004747); f_13007260(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1300474a; continue;
 }
 0x1300474a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004760); }
 let mut pc: u32 = 0x13004760;
 loop { match pc {
 0x13004760 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1300c0e4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13004781); f_13004860(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x13004791); f_13007260(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x13012188u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x13012188u32)); cpu.w32(0x13012188u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x130047b0; continue; }
  pc = 0x130047a3; continue;
 }
 0x130047a3 => {
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x130047ad); f_13007260(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x130047b0; continue;
 }
 0x130047b0 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130047c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130047c0); }
 let mut pc: u32 = 0x130047c0;
 loop { match pc {
 0x130047c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1300c0e4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x34u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x13012188u32)); cpu.w32(0x13012188u32, v); }
  if !cpu.zf { pc = 0x130047e5; continue; }
  pc = 0x130047d8; continue;
 }
 0x130047d8 => {
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x130047e2); f_13007270(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x130047e5; continue;
 }
 0x130047e5 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x130047ee); f_13007270(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x13004804; continue; }
  pc = 0x130047f7; continue;
 }
 0x130047f7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13004804; continue; }
  pc = 0x130047fe; continue;
 }
 0x130047fe => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x13004804); cpu.call(t);
  pc = 0x13004804; continue;
 }
 0x13004804 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004820); }
 let mut pc: u32 = 0x13004820;
 loop { match pc {
 0x13004820 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x13004836; continue; }
  pc = 0x13004829; continue;
 }
 0x13004829 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x13004836; continue; }
  pc = 0x13004830; continue;
 }
 0x13004830 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x13004836); cpu.call(t);
  pc = 0x13004836; continue;
 }
 0x13004836 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x13004849; continue; }
  pc = 0x13004841; continue;
 }
 0x13004841 => {
  { let v: u32 = cpu.and32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xfffffffbu32); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13004849 => {
  { let v: u32 = cpu.or32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x4u32); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004860); }
 let mut pc: u32 = 0x13004860;
 loop { match pc {
 0x13004860 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r8(0x28u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2cu32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = cpu.r8(0x30u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x13004896; continue; }
  pc = 0x13004890; continue;
 }
 0x13004890 => {
  { let v: u32 = cpu.or32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x13004896; continue;
 }
 0x13004896 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130048a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130048a0); }
 let mut pc: u32 = 0x130048a0;
 loop { match pc {
 0x130048a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x130048ab); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x13012164u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x7u32); }
  if (cpu.sf != cpu.of) { pc = 0x130048c0; continue; }
  pc = 0x130048b9; continue;
 }
 0x130048b9 => {
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  pc = 0x130048c7; continue;
 }
 0x130048c0 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x13012164u32, v); }
  pc = 0x130048c7; continue;
 }
 0x130048c7 => {
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x130048d1); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130048e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130048e0); }
 let mut pc: u32 = 0x130048e0;
 loop { match pc {
 0x130048e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x130123d8u32; cpu.push(v); }
  cpu.push(0x130048eb); f_13007280(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x13012160u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v = 0x130123d8u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x13012160u32, v); }
  cpu.push(0x13004906); f_13007290(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004910); }
 let mut pc: u32 = 0x13004910;
 loop { match pc {
 0x13004910 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x130049deu32; cpu.push(v); }
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
  { let v: u32 = 0x1300c004u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x60u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1300c0ecu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
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
pub fn f_130049de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130049de); }
 let mut pc: u32 = 0x130049de;
 loop { match pc {
 0x130049de => {
  { let v: u32 = 0x1300ce40u32; cpu.eax = v; }
  pc = 0x1300ae6e; continue;
 }
 0x1300ae6e => {
  pc = cpu.r32(0x130131b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130049e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130049e8); }
 let mut pc: u32 = 0x130049e8;
 loop { match pc {
 0x130049e8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_13009ac0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_130049f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x130049f0); }
 let mut pc: u32 = 0x130049f0;
 loop { match pc {
 0x130049f0 => {
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
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004a80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004a80); }
 let mut pc: u32 = 0x13004a80;
 loop { match pc {
 0x13004a80 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x13004a88); f_1300a1b0(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x13004a98; continue; }
  pc = 0x13004a8f; continue;
 }
 0x13004a8f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13004a95); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004a98; continue;
 }
 0x13004a98 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004aa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004aa0); }
 let mut pc: u32 = 0x13004aa0;
 loop { match pc {
 0x13004aa0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x13004ad3; continue; }
  pc = 0x13004ab0; continue;
 }
 0x13004ab0 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v = 0x1300a1b0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004ac3); f_1300ad20(cpu);
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x13004ac9); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x13004ad3 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x13004ada); f_1300a1b0(cpu);
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x13004aeb; continue; }
  pc = 0x13004ae2; continue;
 }
 0x13004ae2 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x13004ae8); f_1300ad18(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x13004aeb; continue;
 }
 0x13004aeb => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004b00); }
 let mut pc: u32 = 0x13004b00;
 loop { match pc {
 0x13004b00 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_13004b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x13004b10); }
 let mut pc: u32 = 0x13004b10;
 loop { match pc {
 0x13004b10 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = v; }
  f_1300a250(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
