#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_1202fd74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fd74); }
 let mut pc: u32 = 0x1202fd74;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fd74 => {
  { let v: u32 = 0x1203e3d0u32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fd7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fd7e); }
 let mut pc: u32 = 0x1202fd7e;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fd7e => {
  { let v: u32 = 0x1203e3f0u32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fd88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fd88); }
 let mut pc: u32 = 0x1202fd88;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fd88 => {
  { let v: u32 = 0x1203e3b0u32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fd92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fd92); }
 let mut pc: u32 = 0x1202fd92;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fd92 => {
  { let v: u32 = 0x1203e3d0u32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fd9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fd9c); }
 let mut pc: u32 = 0x1202fd9c;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fd9c => {
  { let v: u32 = 0x1203e3f0u32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fda6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fda6); }
 let mut pc: u32 = 0x1202fda6;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fda6 => {
  { let v: u32 = 0x1203e42au32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fdb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fdb0); }
 let mut pc: u32 = 0x1202fdb0;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fdb0 => {
  { let v: u32 = 0x1203e44au32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fdba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fdba); }
 let mut pc: u32 = 0x1202fdba;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fdba => {
  { let v: u32 = 0x1203e46au32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fdc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fdc4); }
 let mut pc: u32 = 0x1202fdc4;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fdc4 => {
  { let v: u32 = 0x1203e42au32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fdce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fdce); }
 let mut pc: u32 = 0x1202fdce;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fdce => {
  { let v: u32 = 0x1203e44au32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fdd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fdd8); }
 let mut pc: u32 = 0x1202fdd8;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fdd8 => {
  { let v: u32 = 0x1203e46au32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fde2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fde2); }
 let mut pc: u32 = 0x1202fde2;
 loop { match pc {
 0x1202fc9e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fcb3); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fcc1); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xffffff71u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fcde); f_1203373f(cpu);
  cpu.push(0x1202fce3); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fde2 => {
  { let v: u32 = 0x1203e4b6u32; cpu.edx = v; }
  pc = 0x1202fc9e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fdec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fdec); }
 let mut pc: u32 = 0x1202fdec;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fdec => {
  { let v: u32 = 0x1203e526u32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fdf6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fdf6); }
 let mut pc: u32 = 0x1202fdf6;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fdf6 => {
  { let v: u32 = 0x1203e506u32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe00); }
 let mut pc: u32 = 0x1202fe00;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe00 => {
  { let v: u32 = 0x1203e546u32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe0a); }
 let mut pc: u32 = 0x1202fe0a;
 loop { match pc {
 0x1202fd37 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  {  }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203321d; continue; }
  pc = 0x1202fd56; continue;
 }
 0x1202fd56 => {
  cpu.push(0x1202fd5b); f_1203373f(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd67); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe0a => {
  { let v: u32 = 0x1203e4b6u32; cpu.edx = v; }
  pc = 0x1202fd37; continue;
 }
 0x1203321d => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf64(0xffffff82u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd56; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe14); }
 let mut pc: u32 = 0x1202fe14;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe14 => {
  { let v: u32 = 0x1203e526u32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe1e); }
 let mut pc: u32 = 0x1202fe1e;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe1e => {
  { let v: u32 = 0x1203e506u32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe28); }
 let mut pc: u32 = 0x1202fe28;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe28 => {
  { let v: u32 = 0x1203e546u32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe32); }
 let mut pc: u32 = 0x1202fe32;
 loop { match pc {
 0x1202fc9e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fcb3); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fcc1); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xffffff71u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fcde); f_1203373f(cpu);
  cpu.push(0x1202fce3); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe32 => {
  { let v: u32 = 0x1203db80u32; cpu.edx = v; }
  pc = 0x1202fc9e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe3c); }
 let mut pc: u32 = 0x1202fe3c;
 loop { match pc {
 0x1202fd37 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  {  }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203321d; continue; }
  pc = 0x1202fd56; continue;
 }
 0x1202fd56 => {
  cpu.push(0x1202fd5b); f_1203373f(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd67); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe3c => {
  { let v: u32 = 0x1203db80u32; cpu.edx = v; }
  pc = 0x1202fd37; continue;
 }
 0x1203321d => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf64(0xffffff82u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd56; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe46); }
 let mut pc: u32 = 0x1202fe46;
 loop { match pc {
 0x1202fe46 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  pc = 0x1202fe48; continue;
 }
 0x1202fe48 => {
  { cpu.sub32(cpu.r32(0x1203d878u32), 0x1u32); }
  if cpu.zf { pc = 0x1202fe5d; continue; }
  pc = 0x1202fe51; continue;
 }
 0x1202fe51 => {
  { cpu.fprem(); }
  pc = 0x1202fe53; continue;
 }
 0x1202fe53 => {
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  {  }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.pf() { pc = 0x1202fe48; continue; }
  pc = 0x1202fe5a; continue;
 }
 0x1202fe5a => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe5d => {
  cpu.push(0x1202fe62); f_12030882(cpu);
  pc = 0x1202fe53; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe64); }
 let mut pc: u32 = 0x1202fe64;
 loop { match pc {
 0x1202fc58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1202fc6d); f_12033381(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fc83); f_120336d8(cpu);
  cpu.push(0x1202fc88); f_1203323a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe64 => {
  { let v: u32 = 0x1203e5a0u32; cpu.edx = v; }
  pc = 0x1202fc58; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe6e); }
 let mut pc: u32 = 0x1202fe6e;
 loop { match pc {
 0x1202fceb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffd30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if cpu.zf { pc = 0x1203322f; continue; }
  pc = 0x1202fd09; continue;
 }
 0x1202fd09 => {
  cpu.push(0x1202fd0e); f_120336d8(cpu);
  { let v: u8 = cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfdu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1202fd21); f_12033241(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe6e => {
  { let v: u32 = 0x1203e5a0u32; cpu.edx = v; }
  pc = 0x1202fceb; continue;
 }
 0x1203322f => {
  { let v = cpu.st(0); cpu.wf64(0xffffff7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1202fd09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202fe78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202fe78); }
 let mut pc: u32 = 0x1202fe78;
 loop { match pc {
 0x1202fe78 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x1202fe80; continue;
 }
 0x1202fe80 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x1202ff12; continue; }
  pc = 0x1202fe8c; continue;
 }
 0x1202fe8c => {
  { let v: u32 = cpu.eax ^ 0xe000000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xe000000u32); }
  if cpu.zf { pc = 0x1202fe9b; continue; }
  pc = 0x1202fe98; continue;
 }
 0x1202fe98 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fe9b => {
  { let v: u32 = cpu.shr32(cpu.eax, 28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1203dbe0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1202feaa; continue; }
  pc = 0x1202fea7; continue;
 }
 0x1202fea7 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202feaa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1202ff1c; continue; }
  pc = 0x1202feb5; continue;
 }
 0x1202feb5 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x1202ff1c; continue; }
  pc = 0x1202febc; continue;
 }
 0x1202febc => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1202fefb; continue; }
  pc = 0x1202fee4; continue;
 }
 0x1202fee4 => {
  { let v = cpu.st(0) * cpu.rf32(0x1203dbf0u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x1203dbf0u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202fefb => {
  { let v = cpu.st(0) * cpu.rf32(0x1203dbf4u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x1203dbf4u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ff12 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1202ff1f; continue; }
  pc = 0x1202ff1c; continue;
 }
 0x1202ff1c => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ff1f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1202ff1c; continue; }
  pc = 0x1202ff2a; continue;
 }
 0x1202ff2a => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1202ff7e; continue; }
  pc = 0x1202ff4f; continue;
 }
 0x1202ff4f => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x1202ff86; continue; }
  pc = 0x1202ff56; continue;
 }
 0x1202ff56 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x1202ff86; continue; }
  pc = 0x1202ff5e; continue;
 }
 0x1202ff5e => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x1203dbf8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  {  }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  pc = 0x1202fe80; continue;
 }
 0x1202ff7e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x1202ff5e; continue; }
  pc = 0x1202ff86; continue;
 }
 0x1202ff86 => {
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202ff8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202ff8d); }
 let mut pc: u32 = 0x1202ff8d;
 loop { match pc {
 0x1202ff8d => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3fu32); cpu.eax = v; }
  pc = cpu.r32(0x1203dc2eu32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1202ff9a => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ffa0 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1202ffa3, "Int");
  pc = 0x1202ffa5; continue;
 }
 0x1202ffa5 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ffab => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1202ffae, "Int");
  pc = 0x1202ffb0; continue;
 }
 0x1202ffb0 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ffb6 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ffbc => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ffc2 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ffc8 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1202ffda); f_1202fe78(cpu);
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ffe4 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1202ffe7, "Int");
  pc = 0x1202ffe9; continue;
 }
 0x1202ffe9 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1202fff5); f_1202fe78(cpu);
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202ffff => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x12030002, "Int");
  pc = 0x12030004; continue;
 }
 0x12030004 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030018); f_1202fe78(cpu);
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030020 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203002c); f_1202fe78(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030030 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203003c); f_1202fe78(cpu);
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030044 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030050); f_1202fe78(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030054 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030068); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030074 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x12030077, "Int");
  pc = 0x12030079; continue;
 }
 0x12030079 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030087); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030093 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x12030096, "Int");
  pc = 0x12030098; continue;
 }
 0x12030098 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120300ae); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120300b8 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120300c6); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120300cc => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120300da); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120300e4 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120300f2); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120300f8 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203010c); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030118 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1203011b, "Int");
  pc = 0x1203011d; continue;
 }
 0x1203011d => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203012b); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030137 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1203013a, "Int");
  pc = 0x1203013c; continue;
 }
 0x1203013c => {
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030152); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203015c => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203016a); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030170 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203017e); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030188 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030196); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203019c => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120301b0); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120301bc => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x120301bf, "Int");
  pc = 0x120301c1; continue;
 }
 0x120301c1 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120301cf); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120301db => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x120301de, "Int");
  pc = 0x120301e0; continue;
 }
 0x120301e0 => {
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120301f6); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030200 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203020e); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030214 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030222); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203022c => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203023a); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030240 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030254); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030260 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x12030263, "Int");
  pc = 0x12030265; continue;
 }
 0x12030265 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030273); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203027f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x12030282, "Int");
  pc = 0x12030284; continue;
 }
 0x12030284 => {
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203029a); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120302a4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120302b2); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120302b8 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120302c6); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120302d0 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120302de); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120302e4 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120302f8); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030304 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x12030307, "Int");
  pc = 0x12030309; continue;
 }
 0x12030309 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030317); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030323 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x12030326, "Int");
  pc = 0x12030328; continue;
 }
 0x12030328 => {
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203033e); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030348 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030356); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203035c => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203036a); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030374 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030382); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030388 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203039c); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120303a8 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x120303ab, "Int");
  pc = 0x120303ad; continue;
 }
 0x120303ad => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120303bb); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120303c7 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x120303ca, "Int");
  pc = 0x120303cc; continue;
 }
 0x120303cc => {
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120303e2); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120303ec => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120303fa); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030400 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203040e); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030418 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030426); f_1202fe78(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203042c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203042c); }
 let mut pc: u32 = 0x1203042c;
 loop { match pc {
 0x1203042c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203043b); f_1202fe78(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203043f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203043f); }
 let mut pc: u32 = 0x1203043f;
 loop { match pc {
 0x1203043f => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203044e); f_1202fe78(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030452(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030452); }
 let mut pc: u32 = 0x12030452;
 loop { match pc {
 0x12030452 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7f800000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7f800000u32); }
  if cpu.zf { pc = 0x12030496; continue; }
  pc = 0x12030463; continue;
 }
 0x12030463 => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12030479; continue; }
  pc = 0x1203046c; continue;
 }
 0x1203046c => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x12030475); f_1203042c(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12030479 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x1203048a); f_1203042c(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12030496 => {
  { let v = cpu.st(0) / cpu.rf32(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203049e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203049e); }
 let mut pc: u32 = 0x1203049e;
 loop { match pc {
 0x1203049e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff00000u32); }
  if cpu.zf { pc = 0x120304e2; continue; }
  pc = 0x120304af; continue;
 }
 0x120304af => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x120304c5; continue; }
  pc = 0x120304b8; continue;
 }
 0x120304b8 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x120304c1); f_1203042c(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x120304c5 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x120304d6); f_1203042c(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x120304e2 => {
  { let v = cpu.st(0) / cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120304ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120304ea); }
 let mut pc: u32 = 0x120304ea;
 loop { match pc {
 0x120304ea => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12030501; continue; }
  pc = 0x120304f4; continue;
 }
 0x120304f4 => {
  { let v = (cpu.r16(0x8u32.wrapping_add(cpu.esp)) as i16 as f64); cpu.fpush(v); }
  cpu.push(0x120304fd); f_1203042c(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12030501 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = (cpu.r16(0x14u32.wrapping_add(cpu.esp)) as i16 as f64); cpu.fpush(v); }
  cpu.push(0x12030512); f_1203042c(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203051e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203051e); }
 let mut pc: u32 = 0x1203051e;
 loop { match pc {
 0x1203051e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12030535; continue; }
  pc = 0x12030528; continue;
 }
 0x12030528 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  cpu.push(0x12030531); f_1203042c(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12030535 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = (cpu.r32(0x14u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  cpu.push(0x12030546); f_1203042c(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030552(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030552); }
 let mut pc: u32 = 0x12030552;
 loop { match pc {
 0x12030552 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7f800000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7f800000u32); }
  if cpu.zf { pc = 0x12030596; continue; }
  pc = 0x12030563; continue;
 }
 0x12030563 => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12030579; continue; }
  pc = 0x1203056c; continue;
 }
 0x1203056c => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x12030575); f_1203043f(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12030579 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x1203058a); f_1203043f(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12030596 => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203059e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203059e); }
 let mut pc: u32 = 0x1203059e;
 loop { match pc {
 0x1203059e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff00000u32); }
  if cpu.zf { pc = 0x120305e2; continue; }
  pc = 0x120305af; continue;
 }
 0x120305af => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x120305c5; continue; }
  pc = 0x120305b8; continue;
 }
 0x120305b8 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x120305c1); f_1203043f(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x120305c5 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x120305d6); f_1203043f(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x120305e2 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120305ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120305ea); }
 let mut pc: u32 = 0x120305ea;
 loop { match pc {
 0x120305ea => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12030601; continue; }
  pc = 0x120305f4; continue;
 }
 0x120305f4 => {
  { let v = (cpu.r16(0x8u32.wrapping_add(cpu.esp)) as i16 as f64); cpu.fpush(v); }
  cpu.push(0x120305fd); f_1203043f(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12030601 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = (cpu.r16(0x14u32.wrapping_add(cpu.esp)) as i16 as f64); cpu.fpush(v); }
  cpu.push(0x12030612); f_1203043f(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203061e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203061e); }
 let mut pc: u32 = 0x1203061e;
 loop { match pc {
 0x1203061e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12030635; continue; }
  pc = 0x12030628; continue;
 }
 0x12030628 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  cpu.push(0x12030631); f_1203043f(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12030635 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = (cpu.r32(0x14u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  cpu.push(0x12030646); f_1203043f(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030652(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030652); }
 let mut pc: u32 = 0x12030652;
 loop { match pc {
 0x12030652 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030662); f_1202fe78(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030667(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030667); }
 let mut pc: u32 = 0x12030667;
 loop { match pc {
 0x12030667 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030677); f_1202fe78(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203067c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203067c); }
 let mut pc: u32 = 0x1203067c;
 loop { match pc {
 0x1203067c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ 0x700u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x700u32); }
  if !cpu.zf { pc = 0x12030876; continue; }
  pc = 0x12030693; continue;
 }
 0x12030693 => {
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1203dbfcu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x12030876; continue; }
  pc = 0x120306a6; continue;
 }
 0x120306a6 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fff0000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fff0000u32); }
  if cpu.zf { pc = 0x12030876; continue; }
  pc = 0x120306ba; continue;
 }
 0x120306ba => {
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fff0000u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12030876; continue; }
  pc = 0x120306c9; continue;
 }
 0x120306c9 => {
  { cpu.sub32(cpu.eax, 0x7fff0000u32); }
  if cpu.zf { pc = 0x12030876; continue; }
  pc = 0x120306d4; continue;
 }
 0x120306d4 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x12030876; continue; }
  pc = 0x120306e0; continue;
 }
 0x120306e0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x12030876; continue; }
  pc = 0x120306ec; continue;
 }
 0x120306ec => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3fu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x12030764; continue; }
  pc = 0x12030706; continue;
 }
 0x12030706 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if cpu.sf { pc = 0x12030876; continue; }
  pc = 0x12030724; continue;
 }
 0x12030724 => {
  { let v = cpu.rf80(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fprem(); }
  { let v = cpu.st(0); cpu.wf80(0x28u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x12030706; continue;
 }
 0x12030764 => {
  { cpu.and32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x12030774; continue; }
  pc = 0x1203076c; continue;
 }
 0x1203076c => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12030774; continue;
 }
 0x12030774 => {
  { let v = cpu.fpu_control; cpu.w16(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x38u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx | cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.rf80(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  pc = 0x120307ce; continue;
 }
 0x120307ce => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b);  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x100u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x120307db; continue; }
  pc = 0x120307d9; continue;
 }
 0x120307d9 => {
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  pc = 0x120307db; continue;
 }
 0x120307db => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc1cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x120307ce; continue; }
  pc = 0x120307ea; continue;
 }
 0x120307ea => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf80(0x28u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x1203dc24u32); cpu.fpush(v); }
  { cpu.fprem(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.fpu_control = cpu.r16(0x34u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.and32(cpu.ebx, 0x8000u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x12030814; continue; }
  pc = 0x12030812; continue;
 }
 0x12030812 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x12030814; continue;
 }
 0x12030814 => {
  { cpu.and32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x12030872; continue; }
  pc = 0x1203081c; continue;
 }
 0x1203081c => {
  { let v = cpu.fstatus(); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x12030847; continue; }
  pc = 0x12030828; continue;
 }
 0x12030828 => {
  { let v = cpu.fpu_control; cpu.w16(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x300u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x38u32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc14u32); cpu.set_st(0, v); }
  { cpu.fpu_control = cpu.r16(0x34u32.wrapping_add(cpu.esp)); }
  pc = 0x12030847; continue;
 }
 0x12030847 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.eax & 0x4300u32; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  cpu.unimplemented(0x1203085d, "Fnstenv");
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xbcffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) | cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.unimplemented(0x1203086c, "Fldenv");
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x12030872; continue;
 }
 0x12030872 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030876 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.fprem(); }
  pc = 0x12030814; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030882(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030882); }
 let mut pc: u32 = 0x12030882;
 loop { match pc {
 0x12030882 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x7fff0000u32); }
  if cpu.zf { pc = 0x120308a4; continue; }
  pc = 0x1203089a; continue;
 }
 0x1203089a => {
  cpu.push(0x1203089f); f_1203067c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120308a4 => {
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x18u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if cpu.zf { pc = 0x1203092d; continue; }
  pc = 0x120308b4; continue;
 }
 0x120308b4 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.edx | 0x2u32; cpu.edx = v; }
  { let v = cpu.fpu_control; cpu.w16(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x28u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fbeu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1203090b; continue; }
  pc = 0x120308e7; continue;
 }
 0x120308e7 => {
  { let v: u32 = cpu.edx | 0x1u32; cpu.edx = v; }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc0cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc0cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x120308fd; continue;
 }
 0x120308fd => {
  { cpu.fpu_control = cpu.r16(0x24u32.wrapping_add(cpu.esp)); }
  cpu.push(0x12030906); f_1203067c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203090b => {
  { let v = cpu.fpu_control; cpu.w16(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x300u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x28u32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc0cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x120308fd; continue;
 }
 0x1203092d => {
  { cpu.fprem(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030934(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030934); }
 let mut pc: u32 = 0x12030934;
 loop { match pc {
 0x12030934 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ 0x700u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x700u32); }
  if !cpu.zf { pc = 0x12030b2e; continue; }
  pc = 0x1203094b; continue;
 }
 0x1203094b => {
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1203dbfcu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x12030b2e; continue; }
  pc = 0x1203095e; continue;
 }
 0x1203095e => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fff0000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fff0000u32); }
  if cpu.zf { pc = 0x12030b2e; continue; }
  pc = 0x12030972; continue;
 }
 0x12030972 => {
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fff0000u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12030b2e; continue; }
  pc = 0x12030981; continue;
 }
 0x12030981 => {
  { cpu.sub32(cpu.eax, 0x7fff0000u32); }
  if cpu.zf { pc = 0x12030b2e; continue; }
  pc = 0x1203098c; continue;
 }
 0x1203098c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x12030b2e; continue; }
  pc = 0x12030998; continue;
 }
 0x12030998 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x12030b2e; continue; }
  pc = 0x120309a4; continue;
 }
 0x120309a4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3fu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x12030a1c; continue; }
  pc = 0x120309be; continue;
 }
 0x120309be => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if cpu.sf { pc = 0x12030b2e; continue; }
  pc = 0x120309dc; continue;
 }
 0x120309dc => {
  { let v = cpu.rf80(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fprem(); }
  { let v = cpu.st(0); cpu.wf80(0x28u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x120309be; continue;
 }
 0x12030a1c => {
  { cpu.and32(cpu.ebx, 0x2u32); }
  if !cpu.zf { pc = 0x12030a2c; continue; }
  pc = 0x12030a24; continue;
 }
 0x12030a24 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12030a2c; continue;
 }
 0x12030a2c => {
  { let v = cpu.fpu_control; cpu.w16(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x38u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx | 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7fffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx | cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.rf80(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  pc = 0x12030a86; continue;
 }
 0x12030a86 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b);  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x100u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x12030a93; continue; }
  pc = 0x12030a91; continue;
 }
 0x12030a91 => {
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  pc = 0x12030a93; continue;
 }
 0x12030a93 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc1cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x12030a86; continue; }
  pc = 0x12030aa2; continue;
 }
 0x12030aa2 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf80(0x28u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x1203dc24u32); cpu.fpush(v); }
  { cpu.fprem(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.fpu_control = cpu.r16(0x34u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.and32(cpu.ebx, 0x8000u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x12030acc; continue; }
  pc = 0x12030aca; continue;
 }
 0x12030aca => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x12030acc; continue;
 }
 0x12030acc => {
  { cpu.and32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x12030b2a; continue; }
  pc = 0x12030ad4; continue;
 }
 0x12030ad4 => {
  { let v = cpu.fstatus(); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x12030aff; continue; }
  pc = 0x12030ae0; continue;
 }
 0x12030ae0 => {
  { let v = cpu.fpu_control; cpu.w16(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x300u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x38u32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc14u32); cpu.set_st(0, v); }
  { cpu.fpu_control = cpu.r16(0x34u32.wrapping_add(cpu.esp)); }
  pc = 0x12030aff; continue;
 }
 0x12030aff => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.eax & 0x4300u32; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  cpu.unimplemented(0x12030b15, "Fnstenv");
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xbcffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) | cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.unimplemented(0x12030b24, "Fldenv");
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x12030b2a; continue;
 }
 0x12030b2a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030b2e => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.fprem(); }
  pc = 0x12030acc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030b3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030b3a); }
 let mut pc: u32 = 0x12030b3a;
 loop { match pc {
 0x12030b3a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x7fff0000u32); }
  if cpu.zf { pc = 0x12030b5f; continue; }
  pc = 0x12030b55; continue;
 }
 0x12030b55 => {
  cpu.push(0x12030b5a); f_12030934(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030b5f => {
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x18u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if cpu.zf { pc = 0x12030be8; continue; }
  pc = 0x12030b6f; continue;
 }
 0x12030b6f => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.edx | 0x2u32; cpu.edx = v; }
  { let v = cpu.fpu_control; cpu.w16(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x28u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fbeu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12030bc6; continue; }
  pc = 0x12030ba2; continue;
 }
 0x12030ba2 => {
  { let v: u32 = cpu.edx | 0x1u32; cpu.edx = v; }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc0cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc0cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12030bb8; continue;
 }
 0x12030bb8 => {
  { cpu.fpu_control = cpu.r16(0x24u32.wrapping_add(cpu.esp)); }
  cpu.push(0x12030bc1); f_12030934(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030bc6 => {
  { let v = cpu.fpu_control; cpu.w16(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x300u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x28u32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x1203dc0cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12030bb8; continue;
 }
 0x12030be8 => {
  { cpu.fprem(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x30u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030bef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030bef); }
 let mut pc: u32 = 0x12030bef;
 loop { match pc {
 0x12030bef => {
  cpu.push(0x12030bf4); f_12030882(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030bf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030bf5); }
 let mut pc: u32 = 0x12030bf5;
 loop { match pc {
 0x12030bf5 => {
  cpu.push(0x12030bfa); f_12030b3a(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030bfb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030bfb); }
 let mut pc: u32 = 0x12030bfb;
 loop { match pc {
 0x12030bfb => {
  { let v = cpu.st(1).atan2(cpu.st(0)); cpu.fpop(); cpu.set_st(0, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030bfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030bfe); }
 let mut pc: u32 = 0x12030bfe;
 loop { match pc {
 0x12030bfe => {
  { cpu.c2 = false; let a = cpu.st(0); cpu.set_st(0, a.tan()); cpu.fpush(1.0); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030c01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030c01); }
 let mut pc: u32 = 0x12030c01;
 loop { match pc {
 0x12030c01 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12030c1c); f_12030c3f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030c20); }
 let mut pc: u32 = 0x12030c20;
 loop { match pc {
 0x12030c20 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x11u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12030c3b); f_12030c3f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030c3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030c3f); }
 let mut pc: u32 = 0x12030c3f;
 loop { match pc {
 0x12030c3f => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x133fu32; cpu.push(v); }
  cpu.push(0x12030c52); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if cpu.zf { pc = 0x12030e75; continue; }
  pc = 0x12030c6a; continue;
 }
 0x12030c6a => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if cpu.zf { pc = 0x12030e75; continue; }
  pc = 0x12030c7d; continue;
 }
 0x12030c7d => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12030c94; continue; }
  pc = 0x12030c8a; continue;
 }
 0x12030c8a => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12030c94; continue;
 }
 0x12030c94 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x28u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12030cab; continue; }
  pc = 0x12030ca1; continue;
 }
 0x12030ca1 => {
  { let v = cpu.rf64(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x28u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12030cab; continue;
 }
 0x12030cab => {
  { let v = cpu.rf64(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b);  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x12030cbe; continue; }
  pc = 0x12030cbc; continue;
 }
 0x12030cbc => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  pc = 0x12030cbe; continue;
 }
 0x12030cbe => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x12036a88u32); cpu.fcom(a, b);  }
  { let v = cpu.st(0); cpu.wf64(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x12030cea; continue; }
  pc = 0x12030cd1; continue;
 }
 0x12030cd1 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12030cdc); f_12034452(cpu);
  { let v = cpu.rf64(0x12036a88u32); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030cea => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0x34u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) * cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x2cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x2cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030d28); f_1203433e(cpu);
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12030d45); f_1203433e(cpu);
  { let v = cpu.st(0) * cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12030d5f); f_120342c6(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x400u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12030dbc; continue; }
  pc = 0x12030d74; continue;
 }
 0x12030d74 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x600u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12030d8a); f_1203428d(cpu);
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x12030db4); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030dbc => {
  { cpu.sub32(cpu.ecx, 0xfffffc03u32); }
  if (cpu.sf == cpu.of) { pc = 0x12030e0c; continue; }
  pc = 0x12030dc4; continue;
 }
 0x12030dc4 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x600u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12030dda); f_1203428d(cpu);
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  cpu.push(0x12030e04); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030e0c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12030e1c); f_1203428d(cpu);
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.esi, 0x20u32); }
  if cpu.zf { pc = 0x12030e42; continue; }
  pc = 0x12030e2b; continue;
 }
 0x12030e2b => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12030e36); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030e42 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x12030e6d); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030e75 => {
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7ff8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x12030e97; continue; }
  pc = 0x12030e86; continue;
 }
 0x12030e86 => {
  { cpu.and32(cpu.r32(0x24u32.wrapping_add(cpu.esp)), 0x7ffffu32); }
  if !cpu.zf { pc = 0x12030eb7; continue; }
  pc = 0x12030e90; continue;
 }
 0x12030e90 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x12030eb7; continue; }
  pc = 0x12030e97; continue;
 }
 0x12030e97 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x12030eee; continue; }
  pc = 0x12030ea6; continue;
 }
 0x12030ea6 => {
  { cpu.and32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), 0x7ffffu32); }
  if !cpu.zf { pc = 0x12030eb7; continue; }
  pc = 0x12030eb0; continue;
 }
 0x12030eb0 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12030eee; continue; }
  pc = 0x12030eb7; continue;
 }
 0x12030eb7 => {
  { let v = cpu.rf64(0x28u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12030ee6); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030eee => {
  { cpu.sub16((cpu.ecx as u16), 0x7ff8u16); }
  if cpu.zf { pc = 0x12030f14; continue; }
  pc = 0x12030ef5; continue;
 }
 0x12030ef5 => {
  { cpu.sub16((cpu.eax as u16), 0x7ff8u16); }
  if cpu.zf { pc = 0x12030f14; continue; }
  pc = 0x12030efb; continue;
 }
 0x12030efb => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12030f06); f_12034452(cpu);
  { let v = cpu.rf64(0x1203e698u32); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12030f14 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12030f33); f_120338d8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030f3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030f3b); }
 let mut pc: u32 = 0x12030f3b;
 loop { match pc {
 0x12030f3b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030f63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030f63); }
 let mut pc: u32 = 0x12030f63;
 loop { match pc {
 0x12030f63 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ 0x80000000u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030f83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030f83); }
 let mut pc: u32 = 0x12030f83;
 loop { match pc {
 0x12030f83 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12030f97); f_12032e56(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12030f9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12030f9b); }
 let mut pc: u32 = 0x12030f9b;
 loop { match pc {
 0x12030f9b => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x133fu32; cpu.push(v); }
  cpu.push(0x12030fae); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x1203103c; continue; }
  pc = 0x12030fc6; continue;
 }
 0x12030fc6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12030fd1); f_120342db(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x12030fe3; continue; }
  pc = 0x12030fd9; continue;
 }
 0x12030fd9 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1203100b; continue; }
  pc = 0x12030fde; continue;
 }
 0x12030fde => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x12031022; continue; }
  pc = 0x12030fe3; continue;
 }
 0x12030fe3 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) + cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x25u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12031003); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203100b => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12031016); f_12034452(cpu);
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031022 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x25u32; cpu.push(v); }
  cpu.push(0x12031034); f_12033888(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203103c => {
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x12031071; continue; }
  pc = 0x12031047; continue;
 }
 0x12031047 => {
  { let v = cpu.rf64(0x1203e698u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x25u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x12031069); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031071 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12031085); f_1203433e(cpu);
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120310a6); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120310b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120310b2); }
 let mut pc: u32 = 0x120310b2;
 loop { match pc {
 0x120310b2 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x133fu32; cpu.push(v); }
  cpu.push(0x120310c5); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if cpu.zf { pc = 0x120310ec; continue; }
  pc = 0x120310d9; continue;
 }
 0x120310d9 => {
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x12031177; continue; }
  pc = 0x120310ec; continue;
 }
 0x120310ec => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7ff8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x1203110e; continue; }
  pc = 0x120310fd; continue;
 }
 0x120310fd => {
  { cpu.and32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x7ffffu32); }
  if !cpu.zf { pc = 0x1203112e; continue; }
  pc = 0x12031107; continue;
 }
 0x12031107 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1203112e; continue; }
  pc = 0x1203110e; continue;
 }
 0x1203110e => {
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x12031162; continue; }
  pc = 0x1203111d; continue;
 }
 0x1203111d => {
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x7ffffu32); }
  if !cpu.zf { pc = 0x1203112e; continue; }
  pc = 0x12031127; continue;
 }
 0x12031127 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12031162; continue; }
  pc = 0x1203112e; continue;
 }
 0x1203112e => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1203115a); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031162 => {
  { cpu.sub16((cpu.ecx as u16), 0x7ff8u16); }
  if cpu.zf { pc = 0x120313b2; continue; }
  pc = 0x1203116d; continue;
 }
 0x1203116d => {
  { cpu.sub16((cpu.eax as u16), 0x7ff8u16); }
  if cpu.zf { pc = 0x120313b2; continue; }
  pc = 0x12031177; continue;
 }
 0x12031177 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x1203119d; continue; }
  pc = 0x12031186; continue;
 }
 0x12031186 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12031191); f_12034452(cpu);
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203119d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x120311d3; continue; }
  pc = 0x120311ac; continue;
 }
 0x120311ac => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x120311d3; continue; }
  pc = 0x120311cb; continue;
 }
 0x120311cb => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  pc = 0x120311d3; continue;
 }
 0x120311d3 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x120311ef; continue; }
  pc = 0x120311e0; continue;
 }
 0x120311e0 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x12031213; continue; }
  pc = 0x120311ef; continue;
 }
 0x120311ef => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x120312b7; continue; }
  pc = 0x12031200; continue;
 }
 0x12031200 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x120312b7; continue; }
  pc = 0x12031213; continue;
 }
 0x12031213 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x12031232; continue; }
  pc = 0x1203122b; continue;
 }
 0x1203122b => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x1203122e; continue;
 }
 0x1203122e => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031232; continue;
 }
 0x12031232 => {
  { cpu.and16(cpu.r16(0xau32.wrapping_add(cpu.esp)), 0x7ff0u16); }
  if !cpu.zf { pc = 0x12031316; continue; }
  pc = 0x1203123f; continue;
 }
 0x1203123f => {
  { cpu.and32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xfffffu32); }
  if !cpu.zf { pc = 0x12031254; continue; }
  pc = 0x12031249; continue;
 }
 0x12031249 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12031316; continue; }
  pc = 0x12031254; continue;
 }
 0x12031254 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12031268); f_1203433e(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x600u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12031280); f_1203428d(cpu);
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  cpu.push(0x120312af); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120312b7 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x120312d3; continue; }
  pc = 0x120312c4; continue;
 }
 0x120312c4 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x120312f7; continue; }
  pc = 0x120312d3; continue;
 }
 0x120312d3 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12031232; continue; }
  pc = 0x120312e4; continue;
 }
 0x120312e4 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x12031232; continue; }
  pc = 0x120312f7; continue;
 }
 0x120312f7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x12031232; continue; }
  pc = 0x1203130e; continue;
 }
 0x1203130e => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x1203122e; continue;
 }
 0x12031316 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x7ff00000u32); }
  if !cpu.zf { pc = 0x12031327; continue; }
  pc = 0x12031320; continue;
 }
 0x12031320 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12031338; continue; }
  pc = 0x12031327; continue;
 }
 0x12031327 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xfff00000u32); }
  if !cpu.zf { pc = 0x1203139b; continue; }
  pc = 0x12031331; continue;
 }
 0x12031331 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1203139b; continue; }
  pc = 0x12031338; continue;
 }
 0x12031338 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1203134c); f_1203433e(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x600u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12031364); f_1203428d(cpu);
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x12031393); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203139b => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120313a6); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120313b2 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  cpu.push(0x120313ce); f_120338d8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120313d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120313d6); }
 let mut pc: u32 = 0x120313d6;
 loop { match pc {
 0x120313d6 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x7ff0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120313eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120313eb); }
 let mut pc: u32 = 0x120313eb;
 loop { match pc {
 0x120313eb => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x1203140b; continue; }
  pc = 0x120313fa; continue;
 }
 0x120313fa => {
  { cpu.and32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x7ffffu32); }
  if !cpu.zf { pc = 0x12031414; continue; }
  pc = 0x12031404; continue;
 }
 0x12031404 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x12031414; continue; }
  pc = 0x1203140b; continue;
 }
 0x1203140b => {
  { cpu.sub16((cpu.eax as u16), 0x7ff8u16); }
  if cpu.zf { pc = 0x12031414; continue; }
  pc = 0x12031411; continue;
 }
 0x12031411 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031414 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203141a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203141a); }
 let mut pc: u32 = 0x1203141a;
 loop { match pc {
 0x1203141a => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7ff0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x12031464; continue; }
  pc = 0x1203142b; continue;
 }
 0x1203142b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1203143a); f_120342db(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x12031452; continue; }
  pc = 0x12031442; continue;
 }
 0x12031442 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x12031458; continue; }
  pc = 0x12031447; continue;
 }
 0x12031447 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x1203145e; continue; }
  pc = 0x1203144c; continue;
 }
 0x1203144c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031452 => {
  { let v: u32 = 0x200u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031458 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203145e => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031464 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x7fffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  if !cpu.zf { pc = 0x1203148c; continue; }
  pc = 0x12031480; continue;
 }
 0x12031480 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203148c => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x120314ae; continue; }
  pc = 0x12031491; continue;
 }
 0x12031491 => {
  { cpu.and32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xfffffu32); }
  if !cpu.zf { pc = 0x120314a2; continue; }
  pc = 0x1203149b; continue;
 }
 0x1203149b => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x120314ae; continue; }
  pc = 0x120314a2; continue;
 }
 0x120314a2 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x70u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120314ae => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf8u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120314bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120314bc); }
 let mut pc: u32 = 0x120314bc;
 loop { match pc {
 0x120314bc => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x90u32.wrapping_add(cpu.esp)), 0xc000008eu32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if cpu.zf { pc = 0x12031514; continue; }
  pc = 0x120314d3; continue;
 }
 0x120314d3 => {
  { cpu.sub32(cpu.r32(0xa0u32.wrapping_add(cpu.esp)), 0xc000008fu32); }
  if cpu.zf { pc = 0x12031514; continue; }
  pc = 0x120314e0; continue;
 }
 0x120314e0 => {
  { cpu.sub32(cpu.r32(0xa0u32.wrapping_add(cpu.esp)), 0xc0000090u32); }
  if cpu.zf { pc = 0x12031514; continue; }
  pc = 0x120314ed; continue;
 }
 0x120314ed => {
  { cpu.sub32(cpu.r32(0xa0u32.wrapping_add(cpu.esp)), 0xc0000091u32); }
  if cpu.zf { pc = 0x12031514; continue; }
  pc = 0x120314fa; continue;
 }
 0x120314fa => {
  { cpu.sub32(cpu.r32(0xa0u32.wrapping_add(cpu.esp)), 0xc0000093u32); }
  if cpu.zf { pc = 0x12031514; continue; }
  pc = 0x12031507; continue;
 }
 0x12031507 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031514 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12031554; continue; }
  pc = 0x1203152e; continue;
 }
 0x1203152e => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.push(0x12031536); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12031bc2; continue; }
  pc = 0x12031542; continue;
 }
 0x12031542 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)) & 0xffffff00u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031554 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xfffffff8u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xd8u32); }
  if cpu.zf { pc = 0x12031573; continue; }
  pc = 0x12031566; continue;
 }
 0x12031566 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031573 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xc000u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xc000u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x3800u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x1203dd40u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1ffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xc00u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x400u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120317f6; continue; }
  pc = 0x120315e0; continue;
 }
 0x120315e0 => {
  if cpu.zf { pc = 0x1203181c; continue; }
  pc = 0x120315e6; continue;
 }
 0x120315e6 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12031812; continue; }
  pc = 0x120315ee; continue;
 }
 0x120315ee => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x300u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12031842; continue; }
  pc = 0x120315fb; continue;
 }
 0x120315fb => {
  { cpu.sub32(cpu.eax, 0x200u32); }
  if cpu.zf { pc = 0x12031855; continue; }
  pc = 0x12031606; continue;
 }
 0x12031606 => {
  { cpu.sub32(cpu.eax, 0x300u32); }
  if cpu.zf { pc = 0x12031868; continue; }
  pc = 0x12031611; continue;
 }
 0x12031611 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)) ^ cpu.edx; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)) ^ cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)) ^ cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x12031872; continue; }
  pc = 0x120316f8; continue;
 }
 0x120316f8 => {
  { cpu.and8(cpu.r8(0x50u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x12031872; continue; }
  pc = 0x12031703; continue;
 }
 0x12031703 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) | 0x1u32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031708; continue;
 }
 0x12031708 => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x2u8); }
  if cpu.zf { pc = 0x1203187c; continue; }
  pc = 0x12031713; continue;
 }
 0x12031713 => {
  { cpu.and8(cpu.r8(0x50u32.wrapping_add(cpu.esp)), 0x2u8); }
  if cpu.zf { pc = 0x1203187c; continue; }
  pc = 0x1203171e; continue;
 }
 0x1203171e => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) | 0x2u32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031723; continue;
 }
 0x12031723 => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x12031886; continue; }
  pc = 0x1203172e; continue;
 }
 0x1203172e => {
  { cpu.and8(cpu.r8(0x50u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x12031886; continue; }
  pc = 0x12031739; continue;
 }
 0x12031739 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) | 0x4u32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x1203173e; continue;
 }
 0x1203173e => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x8u8); }
  if cpu.zf { pc = 0x12031890; continue; }
  pc = 0x12031749; continue;
 }
 0x12031749 => {
  { cpu.and8(cpu.r8(0x50u32.wrapping_add(cpu.esp)), 0x8u8); }
  if cpu.zf { pc = 0x12031890; continue; }
  pc = 0x12031754; continue;
 }
 0x12031754 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) | 0x8u32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031759; continue;
 }
 0x12031759 => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x1203189a; continue; }
  pc = 0x12031764; continue;
 }
 0x12031764 => {
  { cpu.and8(cpu.r8(0x50u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x1203189a; continue; }
  pc = 0x1203176f; continue;
 }
 0x1203176f => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) | 0x10u32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031774; continue;
 }
 0x12031774 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1f000u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x8000u32); }
  if !cpu.zf { pc = 0x120318a4; continue; }
  pc = 0x1203178e; continue;
 }
 0x1203178e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x700u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  pc = 0x1203179b; continue;
 }
 0x1203179b => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebp & 0x3e0000u32; cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x100000u32); }
  if !cpu.zf { pc = 0x120318ac; continue; }
  pc = 0x120317af; continue;
 }
 0x120317af => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp & 0x700u32; cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 8u32); cpu.ebp = v; }
  pc = 0x120317bc; continue;
 }
 0x120317bc => {
  { let v: u32 = cpu.eax & 0x7c00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x2000000u32); }
  if !cpu.zf { pc = 0x120318b4; continue; }
  pc = 0x120317cc; continue;
 }
 0x120317cc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x700u32; cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 8u32); cpu.esi = v; }
  pc = 0x120317d9; continue;
 }
 0x120317d9 => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x3fffff72u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12031b45; continue; }
  pc = 0x120317ef; continue;
 }
 0x120317ef => {
  pc = cpu.r32(0x12031c50u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x120317f6 => {
  { cpu.sub32(cpu.eax, 0x800u32); }
  if cpu.zf { pc = 0x1203182f; continue; }
  pc = 0x120317fd; continue;
 }
 0x120317fd => {
  { cpu.sub32(cpu.eax, 0xc00u32); }
  if !cpu.zf { pc = 0x120315ee; continue; }
  pc = 0x12031808; continue;
 }
 0x12031808 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)) | 0x3u32; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  pc = 0x120315ee; continue;
 }
 0x12031812 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)) & 0xfffffffcu32; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  pc = 0x120315ee; continue;
 }
 0x1203181c => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffffdu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  pc = 0x120315ee; continue;
 }
 0x1203182f => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  pc = 0x120315ee; continue;
 }
 0x12031842 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffebu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031611; continue;
 }
 0x12031855 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031611; continue;
 }
 0x12031868 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)) & 0xffffffe3u32; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031611; continue;
 }
 0x12031872 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) & 0xfffffffeu32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031708; continue;
 }
 0x1203187c => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) & 0xfffffffdu32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031723; continue;
 }
 0x12031886 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) & 0xfffffffbu32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x1203173e; continue;
 }
 0x12031890 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) & 0xfffffff7u32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031759; continue;
 }
 0x1203189a => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) & 0xffffffefu32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031774; continue;
 }
 0x120318a4 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  pc = 0x1203179b; continue;
 }
 0x120318ac => {
  { let v: u32 = cpu.shr32(cpu.ebp, 17u32); cpu.ebp = v; }
  pc = 0x120317bc; continue;
 }
 0x120318b4 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 22u32); cpu.esi = v; }
  pc = 0x120317d9; continue;
 }
 0x120318be => {
  { let v: u32 = 0x54u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120318ca); f_12031c78(cpu);
  { let v: u32 = 0x78u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120318d9); f_12031c78(cpu);
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120318eb); f_12031c78(cpu);
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)) & 0xfffffffeu32; cpu.w32(0xa0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.push(0x12031902); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12031bc0; continue; }
  pc = 0x12031910; continue;
 }
 0x12031910 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 29u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1203192c); f_12031eb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1203192f; continue;
 }
 0x1203192f => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12031bc0; continue; }
  pc = 0x12031938; continue;
 }
 0x12031938 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)) & 0xffffff00u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12031b54; continue; }
  pc = 0x12031951; continue;
 }
 0x12031951 => {
  pc = cpu.r32(0x12031c68u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x12031958 => {
  { let v: u32 = cpu.shr32(cpu.eax, 22u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12031961); f_12032119(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12031a88; continue; }
  pc = 0x1203196c; continue;
 }
 0x1203196c => {
  { let v: u32 = 0x54u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12031978); f_12031c78(cpu);
  { let v: u32 = 0x44u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12031988); f_12032130(cpu);
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)) | 0x1u32; cpu.w32(0x94u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.esi, 0xdu32); }
  if !cpu.zf { pc = 0x12031a42; continue; }
  pc = 0x120319b2; continue;
 }
 0x120319b2 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)) & 0xffffffe1u32; cpu.w32(0x94u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x12036a90u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x120319c3; continue;
 }
 0x120319c3 => {
  { cpu.sub32(cpu.r32(0xa0u32.wrapping_add(cpu.esp)), 0xc0000091u32); }
  if !cpu.zf { pc = 0x120319da; continue; }
  pc = 0x120319d0; continue;
 }
 0x120319d0 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x120319da; continue;
 }
 0x120319da => {
  { let v = (cpu.r32(0x18u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.rf80(0x2cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.fscale(); }
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x2cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.sub32(cpu.esi, 0xdu32); }
  if !cpu.zf { pc = 0x12031a65; continue; }
  pc = 0x120319ef; continue;
 }
 0x120319ef => {
  { let v = cpu.rf80(0x2cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031a02; continue;
 }
 0x12031a02 => {
  {  }
  { let v: u32 = 0x44u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.push(0x12031a10); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12031bc0; continue; }
  pc = 0x12031a1e; continue;
 }
 0x12031a1e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 29u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12031a3a); f_12031eb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1203192f; continue;
 }
 0x12031a42 => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12036a94u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe3u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x94u32.wrapping_add(cpu.esp), v); }
  pc = 0x120319c3; continue;
 }
 0x12031a65 => {
  { let v = cpu.rf80(0x2cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0x24u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  pc = 0x12031a02; continue;
 }
 0x12031a88 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 29u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12031a9b); f_120320fa(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 29u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12031ab3); f_120320fa(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 29u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12031acb); f_120320fa(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if !cpu.zf { pc = 0x12031ad9; continue; }
  pc = 0x12031ad4; continue;
 }
 0x12031ad4 => {
  { let v: u32 = 0x1fu32; cpu.ebx = v; }
  pc = 0x12031ad9; continue;
 }
 0x12031ad9 => {
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if !cpu.zf { pc = 0x12031ae2; continue; }
  pc = 0x12031add; continue;
 }
 0x12031add => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  pc = 0x12031ae2; continue;
 }
 0x12031ae2 => {
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12031af1); f_12031c78(cpu);
  { let v: u32 = 0x60u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12031b00); f_12031c78(cpu);
  { let v: u32 = 0x78u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12031b0f); f_12031c78(cpu);
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.push(0x12031b1e); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12031bc0; continue; }
  pc = 0x12031b2c; continue;
 }
 0x12031b2c => {
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12031b3d); f_12031eb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1203192f; continue;
 }
 0x12031b45 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x1203192f; continue;
 }
 0x12031b4e => {
  { let v: u32 = cpu.ecx & 0xfffff3ffu32; cpu.ecx = v; }
  pc = 0x12031b54; continue;
 }
 0x12031b54 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1cu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12031bfa; continue; }
  pc = 0x12031b66; continue;
 }
 0x12031b66 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x12031c05; continue; }
  pc = 0x12031b6f; continue;
 }
 0x12031b6f => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x12031c16; continue; }
  pc = 0x12031b78; continue;
 }
 0x12031b78 => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x12031c21; continue; }
  pc = 0x12031b83; continue;
 }
 0x12031b83 => {
  { let v: u32 = cpu.ecx & 0xffffffdfu32; cpu.ecx = v; }
  pc = 0x12031b86; continue;
 }
 0x12031b86 => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x2u8); }
  if cpu.zf { pc = 0x12031c29; continue; }
  pc = 0x12031b91; continue;
 }
 0x12031b91 => {
  { let v: u32 = cpu.ecx & 0xffffffefu32; cpu.ecx = v; }
  pc = 0x12031b94; continue;
 }
 0x12031b94 => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x12031c31; continue; }
  pc = 0x12031b9f; continue;
 }
 0x12031b9f => {
  { let v: u32 = cpu.ecx & 0xfffffff7u32; cpu.ecx = v; }
  pc = 0x12031ba2; continue;
 }
 0x12031ba2 => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x8u8); }
  if cpu.zf { pc = 0x12031c39; continue; }
  pc = 0x12031bad; continue;
 }
 0x12031bad => {
  { let v: u32 = cpu.ecx & 0xfffffffbu32; cpu.ecx = v; }
  pc = 0x12031bb0; continue;
 }
 0x12031bb0 => {
  { cpu.and8(cpu.r8(0x4cu32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x12031c41; continue; }
  pc = 0x12031bbb; continue;
 }
 0x12031bbb => {
  { let v: u32 = cpu.ecx & 0xfffffffeu32; cpu.ecx = v; }
  pc = 0x12031bbe; continue;
 }
 0x12031bbe => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x12031bc0; continue;
 }
 0x12031bc0 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x12031bc2; continue;
 }
 0x12031bc2 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031bcd => {
  { let v: u32 = cpu.ecx & 0xfffff7ffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x400u32; cpu.ecx = v; }
  pc = 0x12031b54; continue;
 }
 0x12031bde => {
  { let v: u32 = cpu.ecx & 0xfffffbffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x800u32; cpu.ecx = v; }
  pc = 0x12031b54; continue;
 }
 0x12031bef => {
  { let v: u32 = cpu.ecx | 0xc00u32; cpu.ecx = v; }
  pc = 0x12031b54; continue;
 }
 0x12031bfa => {
  { let v: u32 = cpu.ecx | 0x300u32; cpu.ecx = v; }
  pc = 0x12031b78; continue;
 }
 0x12031c05 => {
  { let v: u32 = cpu.ecx & 0xfffffeffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x200u32; cpu.ecx = v; }
  pc = 0x12031b78; continue;
 }
 0x12031c16 => {
  { let v: u32 = cpu.ecx & 0xfffffcffu32; cpu.ecx = v; }
  pc = 0x12031b78; continue;
 }
 0x12031c21 => {
  { let v: u32 = cpu.ecx | 0x20u32; cpu.ecx = v; }
  pc = 0x12031b86; continue;
 }
 0x12031c29 => {
  { let v: u32 = cpu.ecx | 0x10u32; cpu.ecx = v; }
  pc = 0x12031b94; continue;
 }
 0x12031c31 => {
  { let v: u32 = cpu.ecx | 0x8u32; cpu.ecx = v; }
  pc = 0x12031ba2; continue;
 }
 0x12031c39 => {
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  pc = 0x12031bb0; continue;
 }
 0x12031c41 => {
  { let v: u32 = cpu.ecx | 0x1u32; cpu.ecx = v; }
  pc = 0x12031bbe; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12031c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12031c50); }
 let mut pc: u32 = 0x12031c50;
 loop { match pc {
 0x12031c50 => {
  { let v: u32 = 0x88120318u32; cpu.esi = v; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x58120318u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x1bu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x1bu32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  cpu.unimplemented(0x12031c6c, "Int");
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let a = cpu.st(0); let b = (cpu.r16(0x0u32.wrapping_add(cpu.ebx)) as i16 as f64); cpu.fcom(a, b); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  cpu.unimplemented(0x12031c74, "Out");
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x830c244cu32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12031c7d, "In");
  { let v: u8 = (cpu.eax as u8) | 0x56u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x1fu32); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12031e57; continue; }
  pc = 0x12031c96; continue;
 }
 0x12031c96 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12031e90u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12031e5cu32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x12031ca5 => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x12031cbe); f_12032130(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031cd8 => {
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031ce3 => {
  { let v: u32 = cpu.eax & 0xfffffff7u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x16u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x400u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x100u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx | cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.eax); cpu.edx = v; }
  if cpu.zf { pc = 0x12031d35; continue; }
  pc = 0x12031d1d; continue;
 }
 0x12031d1d => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x12031d40; continue; }
  pc = 0x12031d22; continue;
 }
 0x12031d22 => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if cpu.zf { pc = 0x12031d4b; continue; }
  pc = 0x12031d2a; continue;
 }
 0x12031d2a => {
  { let v: u32 = 0x3u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d35 => {
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d40 => {
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d4b => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d56 => {
  { let v: u32 = cpu.eax & 0xffffffe9u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d71 => {
  { let v: u32 = cpu.eax & 0xffffffebu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d8a => {
  { let v: u32 = cpu.eax & 0xffffffedu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0xcu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031da9 => {
  { let v: u32 = cpu.eax & 0xffffffe1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031dbf => {
  { let v: u32 = cpu.eax & 0xffffffe3u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031dde => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e05 => {
  { let v: u32 = cpu.eax & 0xfffffff5u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax | 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e2c => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v: u32 = 0x1203dd30u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e51 => {
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x12031e57; continue;
 }
 0x12031e57 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12031c68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12031c68); }
 let mut pc: u32 = 0x12031c68;
 loop { match pc {
 0x12031c68 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), ((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x830c244cu32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12031c7d, "In");
  { let v: u8 = (cpu.eax as u8) | 0x56u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x1fu32); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12031e57; continue; }
  pc = 0x12031c96; continue;
 }
 0x12031c96 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12031e90u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12031e5cu32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x12031ca5 => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x12031cbe); f_12032130(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031cd8 => {
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031ce3 => {
  { let v: u32 = cpu.eax & 0xfffffff7u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x16u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x400u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x100u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx | cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.eax); cpu.edx = v; }
  if cpu.zf { pc = 0x12031d35; continue; }
  pc = 0x12031d1d; continue;
 }
 0x12031d1d => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x12031d40; continue; }
  pc = 0x12031d22; continue;
 }
 0x12031d22 => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if cpu.zf { pc = 0x12031d4b; continue; }
  pc = 0x12031d2a; continue;
 }
 0x12031d2a => {
  { let v: u32 = 0x3u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d35 => {
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d40 => {
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d4b => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d56 => {
  { let v: u32 = cpu.eax & 0xffffffe9u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d71 => {
  { let v: u32 = cpu.eax & 0xffffffebu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d8a => {
  { let v: u32 = cpu.eax & 0xffffffedu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0xcu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031da9 => {
  { let v: u32 = cpu.eax & 0xffffffe1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031dbf => {
  { let v: u32 = cpu.eax & 0xffffffe3u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031dde => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e05 => {
  { let v: u32 = cpu.eax & 0xfffffff5u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax | 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e2c => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v: u32 = 0x1203dd30u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e51 => {
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x12031e57; continue;
 }
 0x12031e57 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12031c78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12031c78); }
 let mut pc: u32 = 0x12031c78;
 loop { match pc {
 0x12031c78 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x1fu32); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12031e57; continue; }
  pc = 0x12031c96; continue;
 }
 0x12031c96 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12031e90u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12031e5cu32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x12031ca5 => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x12031cbe); f_12032130(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031cd8 => {
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031ce3 => {
  { let v: u32 = cpu.eax & 0xfffffff7u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x16u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x400u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x100u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx | cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.eax); cpu.edx = v; }
  if cpu.zf { pc = 0x12031d35; continue; }
  pc = 0x12031d1d; continue;
 }
 0x12031d1d => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x12031d40; continue; }
  pc = 0x12031d22; continue;
 }
 0x12031d22 => {
  { cpu.sub32(cpu.edx, 0x100u32); }
  if cpu.zf { pc = 0x12031d4b; continue; }
  pc = 0x12031d2a; continue;
 }
 0x12031d2a => {
  { let v: u32 = 0x3u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d35 => {
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d40 => {
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d4b => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d56 => {
  { let v: u32 = cpu.eax & 0xffffffe9u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d71 => {
  { let v: u32 = cpu.eax & 0xffffffebu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031d8a => {
  { let v: u32 = cpu.eax & 0xffffffedu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0xcu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031da9 => {
  { let v: u32 = cpu.eax & 0xffffffe1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031dbf => {
  { let v: u32 = cpu.eax & 0xffffffe3u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031dde => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e05 => {
  { let v: u32 = cpu.eax & 0xfffffff5u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax | 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e2c => {
  { let v: u32 = cpu.eax & 0xffffffe5u32; cpu.eax = v; }
  { let v: u32 = 0x1203dd30u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031e51 => {
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x12031e57; continue;
 }
 0x12031e57 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12031e5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12031e5c); }
 let mut pc: u32 = 0x12031e5c;
 loop { match pc {
 0x12031e5c => {
  { cpu.movs(4, false); }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), (cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x1du32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  if !cpu.of { pc = 0x12031e8b; continue; }
  pc = 0x12031e6e; continue;
 }
 0x12031e6e => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1da91203u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u32 = 0xde12031du32; cpu.edi = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x1e051203u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x1eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x9080706u32.wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0xbu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_12031eb0(cpu); return;
 }
 0x12031e8b => {
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x1eu32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x9080706u32.wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0xbu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_12031eb0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12031eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12031eb0); }
 let mut pc: u32 = 0x12031eb0;
 loop { match pc {
 0x12031eb0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x12031eec; continue; }
  pc = 0x12031eba; continue;
 }
 0x12031eba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12031ec5); f_12032084(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12031ec8; continue;
 }
 0x12031ec8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12031ed8); f_12031ef2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12031ee9; continue; }
  pc = 0x12031edf; continue;
 }
 0x12031edf => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12031ee6); f_12032084(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x12031ee9; continue;
 }
 0x12031ee9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031eec => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x12031ec8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12031ef2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12031ef2); }
 let mut pc: u32 = 0x12031ef2;
 loop { match pc {
 0x12031ef2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x1fu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12032025; continue; }
  pc = 0x12031f00; continue;
 }
 0x12031f00 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x12032054u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12032028u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x12031f0f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12031f1f); f_1203218f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031f24 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12032025; continue; }
  pc = 0x12031f33; continue;
 }
 0x12031f33 => {
  pc = cpu.r32(0x12032074u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x12031f3a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x4000u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xfffffaffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031f53 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0xffffbaffu32); cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031f60 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x100u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffbbffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031f79 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x4500u32); cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031f86 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031f99 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031faa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031fc1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031fd2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12031fe9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032008 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x12032025; continue;
 }
 0x12032025 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032028(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032028); }
 let mut pc: u32 = 0x12032028;
 loop { match pc {
 0x12032028 => {
  {  }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.edi)); let b = (cpu.ebx as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); } { let v: u8 = a; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.pop(); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { cpu.stos(1, false); }
  { cpu.pop(); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 3u32); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  cpu.call(0x1a152368); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032054); }
 let mut pc: u32 = 0x12032054;
 loop { match pc {
 0x12032054 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x5u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi))); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.pop(); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { cpu.pushad(); }
  { cpu.pop(); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  if !cpu.sf { pc = 0x120320a1; continue; }
  pc = 0x12032082; continue;
 }
 0x12032082 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  f_12032084(cpu); return;
 }
 0x12032099 => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3800u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120320ac); f_1203220f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x800u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3800u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x12032099; continue; }
  pc = 0x120320c6; continue;
 }
 0x120320a1 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x120320a8); cpu.call(0xfa5a70b3);
  cpu.unimplemented(0x120320a8, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x800u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3800u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x12032099; continue; }
  pc = 0x120320c6; continue;
 }
 0x120320c6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032084(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032084); }
 let mut pc: u32 = 0x12032084;
 loop { match pc {
 0x12032084 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.esi = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120320ca; continue; }
  pc = 0x12032096; continue;
 }
 0x12032096 => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x12032099; continue;
 }
 0x12032099 => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3800u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120320ac); f_1203220f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x800u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3800u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x12032099; continue; }
  pc = 0x120320c6; continue;
 }
 0x120320c6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120320ca => {
  if (cpu.sf == cpu.of) { pc = 0x120320f6; continue; }
  pc = 0x120320cc; continue;
 }
 0x120320cc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xfffff800u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.eax & 0x3800u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax & 0x3800u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120320f3); f_1203220f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x120320f6; continue;
 }
 0x120320f6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120320fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120320fa); }
 let mut pc: u32 = 0x120320fa;
 loop { match pc {
 0x120320fa => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x12032118; continue; }
  pc = 0x12032102; continue;
 }
 0x12032102 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032108; continue; }
  pc = 0x12032107; continue;
 }
 0x12032107 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032108 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if cpu.sf { pc = 0x12032113; continue; }
  pc = 0x1203210e; continue;
 }
 0x1203210e => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032118; continue; }
  pc = 0x12032113; continue;
 }
 0x12032113 => {
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  pc = 0x12032118; continue;
 }
 0x12032118 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032119(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032119); }
 let mut pc: u32 = 0x12032119;
 loop { match pc {
 0x12032119 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if (cpu.sf != cpu.of) { pc = 0x12032127; continue; }
  pc = 0x12032122; continue;
 }
 0x12032122 => {
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1203212a; continue; }
  pc = 0x12032127; continue;
 }
 0x12032127 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203212a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032130); }
 let mut pc: u32 = 0x12032130;
 loop { match pc {
 0x12032130 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3800u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 11u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  if cpu.sf { pc = 0x1203216f; continue; }
  pc = 0x12032148; continue;
 }
 0x12032148 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.sf == cpu.of) { pc = 0x1203216f; continue; }
  pc = 0x1203214d; continue;
 }
 0x1203214d => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x62u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203216f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203dd30u32); cpu.ecx = v; }
  { let v: u32 = 0x1203dd30u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203218f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203218f); }
 let mut pc: u32 = 0x1203218f;
 loop { match pc {
 0x1203218f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x3800u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 11u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  if cpu.sf { pc = 0x120321f3; continue; }
  pc = 0x120321a7; continue;
 }
 0x120321a7 => {
  { cpu.sub32(cpu.edx, 0x8u32); }
  if (cpu.sf == cpu.of) { pc = 0x120321f3; continue; }
  pc = 0x120321ac; continue;
 }
 0x120321ac => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x62u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x120321f6; continue; }
  pc = 0x120321dc; continue;
 }
 0x120321dc => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x12032208; continue; }
  pc = 0x120321e3; continue;
 }
 0x120321e3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x120321e5; continue;
 }
 0x120321e5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120321f0); f_1203220f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x120321f3; continue;
 }
 0x120321f3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120321f6 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x12032208; continue; }
  pc = 0x120321fb; continue;
 }
 0x120321fb => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x12032208; continue; }
  pc = 0x12032201; continue;
 }
 0x12032201 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x120321e5; continue;
 }
 0x12032208 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x120321e5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203220f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203220f); }
 let mut pc: u32 = 0x1203220f;
 loop { match pc {
 0x1203220f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi & cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032237(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032237); }
 let mut pc: u32 = 0x12032237;
 loop { match pc {
 0x12032237 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12032251; continue; }
  pc = 0x12032247; continue;
 }
 0x12032247 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12032251; continue;
 }
 0x12032251 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x12036aa0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x12032287; continue; }
  pc = 0x12032262; continue;
 }
 0x12032262 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1203dfa0u32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = 0x1203df40u32; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12032283); f_12032321(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x30u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032287 => {
  { let v = cpu.rf64(0x12036aa8u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203dff8u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203dfc8u32; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) * cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120322ba); f_12032321(cpu);
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e058u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e028u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120322de); f_12032321(cpu);
  { let v = cpu.st(0) * cpu.rf64(0x28u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf64(0x12036ab0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { cpu.c2 = false; let v = cpu.st(0).cos(); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.c2 = false; let v = cpu.st(0).sin(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x12036ab8u32); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x12036ac0u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032321(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032321); }
 let mut pc: u32 = 0x12032321;
 loop { match pc {
 0x12032321 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(8))); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032350; continue; }
  pc = 0x12032339; continue;
 }
 0x12032339 => {
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  pc = 0x12032340; continue;
 }
 0x12032340 => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v = cpu.st(0) * cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  if !cpu.zf { pc = 0x1203239c; continue; }
  pc = 0x1203234e; continue;
 }
 0x1203234e => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x12032350; continue;
 }
 0x12032350 => {
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(8))); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1203238a; continue; }
  pc = 0x12032373; continue;
 }
 0x12032373 => {
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  pc = 0x1203237a; continue;
 }
 0x1203237a => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v = cpu.st(0) * cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  if !cpu.zf { pc = 0x120323a0; continue; }
  pc = 0x12032388; continue;
 }
 0x12032388 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x1203238a; continue;
 }
 0x1203238a => {
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203239c => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x12032340; continue;
 }
 0x120323a0 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x1203237a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120323a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120323a4); }
 let mut pc: u32 = 0x120323a4;
 loop { match pc {
 0x120323a4 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x120323c9; continue; }
  pc = 0x120323ba; continue;
 }
 0x120323ba => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x120323c9; continue;
 }
 0x120323c9 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x12036aa0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x1203240f; continue; }
  pc = 0x120323da; continue;
 }
 0x120323da => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1203e0e0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = 0x1203e088u32; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x120323fb); f_12032321(cpu);
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v = cpu.st(0) * (cpu.r32(0x4u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203240f => {
  { let v = cpu.rf64(0x12036aa8u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e138u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e108u32; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0x24u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) * cpu.rf64(0x24u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12032442); f_12032321(cpu);
  { let v = cpu.st(0); cpu.wf64(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e198u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e168u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12032466); f_12032321(cpu);
  { let v = cpu.st(0) * cpu.rf64(0x2cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x38u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf64(0x12036ac8u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { cpu.c2 = false; let v = cpu.st(0).cos(); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.c2 = false; let v = cpu.st(0).sin(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x12036ab8u32); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x12036ac0u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) * (cpu.r32(0x4u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120324b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120324b2); }
 let mut pc: u32 = 0x120324b2;
 loop { match pc {
 0x120324b2 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x120324e4; continue; }
  pc = 0x120324c2; continue;
 }
 0x120324c2 => {
  { let v: u32 = cpu.r32(0x1203e6a4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e6a0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x22u32; cpu.push(v); }
  cpu.push(0x120324e0); f_12032a21(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120324e4 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x12036aa0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x1203254d; continue; }
  pc = 0x120324f5; continue;
 }
 0x120324f5 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12032504); f_12032237(cpu);
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.fpush(std::f64::consts::LN_2); }
  { let v = cpu.rf64(0x24u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x1203e220u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x1203e1c8u32; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0) * cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x12036ad0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12032542); f_12032321(cpu);
  { let v = cpu.st(0) + cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203254d => {
  { let v = cpu.rf64(0x12036aa8u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203dff8u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203dfc8u32; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) * cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12032580); f_12032321(cpu);
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e058u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e028u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120325a4); f_12032321(cpu);
  { let v = cpu.st(0) * cpu.rf64(0x28u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf64(0x12036ab0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { cpu.c2 = false; let v = cpu.st(0).sin(); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.c2 = false; let v = cpu.st(0).cos(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x12036ab8u32); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x12036ac0u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120325e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120325e7); }
 let mut pc: u32 = 0x120325e7;
 loop { match pc {
 0x120325e7 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12032619; continue; }
  pc = 0x120325f7; continue;
 }
 0x120325f7 => {
  { let v: u32 = cpu.r32(0x1203e6a4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e6a0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x23u32; cpu.push(v); }
  cpu.push(0x12032615); f_12032a21(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032619 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x12036aa0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x1203268e; continue; }
  pc = 0x1203262a; continue;
 }
 0x1203262a => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12032639); f_120323a4(cpu);
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.fpush(std::f64::consts::LN_2); }
  { let v = cpu.rf64(0x24u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x1203e2a8u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = 0x1203e268u32; cpu.push(v); }
  { let v = cpu.st(0) * cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf64(0x2cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x12036ad0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203267f); f_12032321(cpu);
  { let v = cpu.st(0) * cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v = cpu.st(0) + cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203268e => {
  { let v = cpu.rf64(0x12036aa8u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e138u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e108u32; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) * cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120326c1); f_12032321(cpu);
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e198u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0x1203e168u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120326e5); f_12032321(cpu);
  { let v = cpu.st(0) * cpu.rf64(0x28u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x34u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf64(0x12036ac8u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { cpu.c2 = false; let v = cpu.st(0).sin(); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.c2 = false; let v = cpu.st(0).cos(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x12036ab8u32); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x12036ac0u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032728(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032728); }
 let mut pc: u32 = 0x12032728;
 loop { match pc {
 0x12032728 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x12032742; continue; }
  pc = 0x12032734; continue;
 }
 0x12032734 => {
  { let v = cpu.rf64(0x30u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v = cpu.st(0); cpu.wf64(0x30u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.and32(cpu.esi, cpu.esi); }
  pc = 0x12032742; continue;
 }
 0x12032742 => {
  if !cpu.zf { pc = 0x1203275b; continue; }
  pc = 0x12032744; continue;
 }
 0x12032744 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12032753); f_12032237(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203275b => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x12032777; continue; }
  pc = 0x12032764; continue;
 }
 0x12032764 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1203276f); f_120323a4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032777 => {
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x30u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1203278d; continue; }
  pc = 0x12032782; continue;
 }
 0x12032782 => {
  { let v = cpu.rf64(0x12036a98u32); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203278d => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.rf64(0x30u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12032821; continue; }
  pc = 0x120327a4; continue;
 }
 0x120327a4 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120327b5); f_12032237(cpu);
  { let v = cpu.st(0); cpu.wf64(0x24u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120327cb); f_120323a4(cpu);
  { let v = cpu.st(0); cpu.wf64(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x38u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x120327db; continue;
 }
 0x120327db => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  if cpu.zf { pc = 0x12032812; continue; }
  pc = 0x12032804; continue;
 }
 0x12032804 => {
  { let v = cpu.st(0); cpu.wf64(0x14u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x120327db; continue;
 }
 0x12032812 => {
  { let v = cpu.st(0); cpu.wf64(0x14u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032821 => {
  { let v = cpu.rf64(0x30u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x30u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032863; continue; }
  pc = 0x12032840; continue;
 }
 0x12032840 => {
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x12032848; continue;
 }
 0x12032848 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(0, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x120328d3; continue; }
  pc = 0x1203285b; continue;
 }
 0x1203285b => {
  { let v = cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12032863; continue;
 }
 0x12032863 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x3ff00000u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.rf64(0x30u32.wrapping_add(cpu.esp)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.esi.wrapping_sub(0x2u32); cpu.esi = v; }
  { let v = cpu.st(0); cpu.wf64(0x14u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x30u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x1203289e; continue;
 }
 0x1203289e => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v = (cpu.r32(0x24u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.sub32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  if cpu.zf { pc = 0x120328da; continue; }
  pc = 0x120328c5; continue;
 }
 0x120328c5 => {
  { let v = cpu.st(0); cpu.wf64(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1203289e; continue;
 }
 0x120328d3 => {
  { let v = cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  pc = 0x12032848; continue;
 }
 0x120328da => {
  { let v = cpu.st(0); cpu.wf64(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120328ef); f_12032237(cpu);
  { let v = cpu.st(0) / cpu.rf64(0x24u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0) * cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120328ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120328ff); }
 let mut pc: u32 = 0x120328ff;
 loop { match pc {
 0x120328ff => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12032938; continue; }
  pc = 0x12032911; continue;
 }
 0x12032911 => {
  { let v: u32 = cpu.r32(0x1203e6a4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e6a0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x24u32; cpu.push(v); }
  cpu.push(0x1203292f); f_12032a21(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032938 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x12032952; continue; }
  pc = 0x12032945; continue;
 }
 0x12032945 => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x12032950; continue; }
  pc = 0x1203294b; continue;
 }
 0x1203294b => {
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  pc = 0x12032950; continue;
 }
 0x12032950 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  pc = 0x12032952; continue;
 }
 0x12032952 => {
  if !cpu.zf { pc = 0x12032974; continue; }
  pc = 0x12032954; continue;
 }
 0x12032954 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12032963); f_120324b2(cpu);
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0) * (cpu.r32(0x8u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032974 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.edi, 0x1u32); }
  { let v = cpu.ecx; cpu.push(v); }
  if !cpu.zf { pc = 0x12032999; continue; }
  pc = 0x12032983; continue;
 }
 0x12032983 => {
  cpu.push(0x12032988); f_120325e7(cpu);
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0) * (cpu.r32(0x8u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032999 => {
  cpu.push(0x1203299e); f_120324b2(cpu);
  { let v = cpu.st(0); cpu.wf64(0x28u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120329b4); f_120325e7(cpu);
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x12036ac0u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0x38u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032a0f; continue; }
  pc = 0x120329d2; continue;
 }
 0x120329d2 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = (cpu.r32(0x10u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  { let v = cpu.st(0) * cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if (cpu.sf != cpu.of) { pc = 0x120329d2; continue; }
  pc = 0x12032a0f; continue;
 }
 0x12032a0f => {
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032a21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032a21); }
 let mut pc: u32 = 0x12032a21;
 loop { match pc {
 0x12032a21 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x133fu32; cpu.push(v); }
  cpu.push(0x12032a30); f_12034452(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12032a54); f_12033948(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032a58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032a58); }
 let mut pc: u32 = 0x12032a58;
 loop { match pc {
 0x12032a58 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203e2f0u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12032a6c); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  if !cpu.zf { pc = 0x12032afa; continue; }
  pc = 0x12032a8a; continue;
 }
 0x12032a8a => {
  cpu.push(0x12032a8f); f_120342db(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x12032aa1; continue; }
  pc = 0x12032a97; continue;
 }
 0x12032a97 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032ac9; continue; }
  pc = 0x12032a9c; continue;
 }
 0x12032a9c => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x12032ae0; continue; }
  pc = 0x12032aa1; continue;
 }
 0x12032aa1 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) + cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12032ac1); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032ac9 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032ad4); f_12034452(cpu);
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032ae0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x12032af2); f_12033888(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032afa => {
  cpu.push(0x12032aff); f_120344df(cpu);
  { let a = cpu.st(0); let b = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fcom(a, b);  }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x12032b28; continue; }
  pc = 0x12032b11; continue;
 }
 0x12032b11 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032b1c); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032b28 => {
  { cpu.and32(cpu.esi, 0x20u32); }
  if cpu.zf { pc = 0x12032b47; continue; }
  pc = 0x12032b30; continue;
 }
 0x12032b30 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032b3b); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032b47 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x12032b65); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032b6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032b6d); }
 let mut pc: u32 = 0x12032b6d;
 loop { match pc {
 0x12032b6d => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x133fu32; cpu.push(v); }
  cpu.push(0x12032b80); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x12032c2a; continue; }
  pc = 0x12032b9c; continue;
 }
 0x12032b9c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12032ba7); f_120342db(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x12032be1; continue; }
  pc = 0x12032baf; continue;
 }
 0x12032baf => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x12032bf8; continue; }
  pc = 0x12032bb4; continue;
 }
 0x12032bb4 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x12032c11; continue; }
  pc = 0x12032bba; continue;
 }
 0x12032bba => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) + cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x15u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12032bd9); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032be1 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032bec); f_12034452(cpu);
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032bf8 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032c03); f_12034452(cpu);
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032c11 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x15u32; cpu.push(v); }
  cpu.push(0x12032c22); f_12033888(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032c2a => {
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032c46); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032c52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032c52); }
 let mut pc: u32 = 0x12032c52;
 loop { match pc {
 0x12032c52 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203e2f4u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12032c66); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  if !cpu.zf { pc = 0x12032cf4; continue; }
  pc = 0x12032c84; continue;
 }
 0x12032c84 => {
  cpu.push(0x12032c89); f_120342db(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x12032c9b; continue; }
  pc = 0x12032c91; continue;
 }
 0x12032c91 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032cc3; continue; }
  pc = 0x12032c96; continue;
 }
 0x12032c96 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x12032cda; continue; }
  pc = 0x12032c9b; continue;
 }
 0x12032c9b => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) + cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12032cbb); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032cc3 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032cce); f_12034452(cpu);
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032cda => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  cpu.push(0x12032cec); f_12033888(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032cf4 => {
  cpu.push(0x12032cf9); f_120344df(cpu);
  { let a = cpu.st(0); let b = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fcom(a, b);  }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x12032d22; continue; }
  pc = 0x12032d0b; continue;
 }
 0x12032d0b => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032d16); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032d22 => {
  { cpu.and32(cpu.esi, 0x20u32); }
  if cpu.zf { pc = 0x12032d41; continue; }
  pc = 0x12032d2a; continue;
 }
 0x12032d2a => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032d35); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032d41 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x12032d5f); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032d67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032d67); }
 let mut pc: u32 = 0x12032d67;
 loop { match pc {
 0x12032d67 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x133fu32; cpu.push(v); }
  cpu.push(0x12032d7a); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x12032e28; continue; }
  pc = 0x12032d96; continue;
 }
 0x12032d96 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12032dab); f_120342db(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x12032dbd; continue; }
  pc = 0x12032db3; continue;
 }
 0x12032db3 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032de5; continue; }
  pc = 0x12032db8; continue;
 }
 0x12032db8 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x12032e0e; continue; }
  pc = 0x12032dbd; continue;
 }
 0x12032dbd => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) + cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x17u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12032ddd); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032de5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203e6a4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203e6a0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x17u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12032e06); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032e0e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x17u32; cpu.push(v); }
  cpu.push(0x12032e20); f_12033888(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032e28 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12032e38); f_1203433e(cpu);
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032e4a); f_12034452(cpu);
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032e56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032e56); }
 let mut pc: u32 = 0x12032e56;
 loop { match pc {
 0x12032e56 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x133fu32; cpu.push(v); }
  cpu.push(0x12032e6a); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x12032f15; continue; }
  pc = 0x12032e86; continue;
 }
 0x12032e86 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12032e91); f_120342db(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x12032ea3; continue; }
  pc = 0x12032e99; continue;
 }
 0x12032e99 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032ed8; continue; }
  pc = 0x12032e9e; continue;
 }
 0x12032e9e => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x12032ef0; continue; }
  pc = 0x12032ea3; continue;
 }
 0x12032ea3 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) + cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = (cpu.r32(0x34u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x19u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12032ecf); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032ed8 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032ee3); f_12034452(cpu);
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032ef0 => {
  { let v = (cpu.r32(0x20u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12032f0c); f_120338d8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032f15 => {
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x12032f38; continue; }
  pc = 0x12032f20; continue;
 }
 0x12032f20 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12032f2b); f_12034452(cpu);
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032f38 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12032f4c); f_1203433e(cpu);
  { let v = cpu.st(0); cpu.wf64(0x14u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7fffffffu32); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12032f6d; continue; }
  pc = 0x12032f67; continue;
 }
 0x12032f67 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x12032f6d; continue;
 }
 0x12032f6d => {
  { cpu.sub32(cpu.eax, 0xa00u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12032fc2; continue; }
  pc = 0x12032f74; continue;
 }
 0x12032f74 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203e69cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203e698u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12032f91); f_12030f3b(cpu);
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v = (cpu.r32(0x1cu32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x19u32; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x12032fb9); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12032fc2 => {
  { cpu.sub32(cpu.eax, 0x400u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12033014; continue; }
  pc = 0x12032fc9; continue;
 }
 0x12032fc9 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x600u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12032fde); f_1203428d(cpu);
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = (cpu.r32(0x1cu32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x19u32; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1203300b); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033014 => {
  { cpu.sub32(cpu.eax, 0xfffff603u32); }
  if (cpu.sf == cpu.of) { pc = 0x1203304a; continue; }
  pc = 0x1203301b; continue;
 }
 0x1203301b => {
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x19u32; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  cpu.push(0x12033041); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203304a => {
  { cpu.sub32(cpu.eax, 0xfffffc03u32); }
  if (cpu.sf == cpu.of) { pc = 0x1203309c; continue; }
  pc = 0x12033051; continue;
 }
 0x12033051 => {
  { let v: u32 = cpu.eax.wrapping_add(0x600u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12033066); f_1203428d(cpu);
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = (cpu.r32(0x1cu32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x19u32; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x12033093); f_12033a03(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203309c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120330ac); f_1203428d(cpu);
  { let v = cpu.st(0); cpu.wf64(0x14u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120330be); f_12034452(cpu);
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120330cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120330cb); }
 let mut pc: u32 = 0x120330cb;
 loop { match pc {
 0x120330cb => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x120330d9); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1203e2f8u32); cpu.eax = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120330ee); f_12034452(cpu);
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x120331bb; continue; }
  pc = 0x12033104; continue;
 }
 0x12033104 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203e6a4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x1203e6a0u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12033127); f_120342db(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x12033139; continue; }
  pc = 0x1203312f; continue;
 }
 0x1203312f => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12033164; continue; }
  pc = 0x12033134; continue;
 }
 0x12033134 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x1203319b; continue; }
  pc = 0x12033139; continue;
 }
 0x12033139 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) + cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1cu32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1203315b); f_12033948(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033164 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1203317c); f_12030f3b(cpu);
  { let v = cpu.st(0); cpu.wf64(0x18u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1203318e); f_12034452(cpu);
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203319b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1cu32; cpu.push(v); }
  cpu.push(0x120331b2); f_12033888(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120331bb => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x120331fb; continue; }
  pc = 0x120331ca; continue;
 }
 0x120331ca => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x120331e3; continue;
 }
 0x120331e3 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120331ee); f_12034452(cpu);
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120331fb => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1203320a); f_120344df(cpu);
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x120331e3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203323a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203323a); }
 let mut pc: u32 = 0x1203323a;
 loop { match pc {
 0x1203323a => {
  { let v: u8 = cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0xfffffd38u32.wrapping_add(cpu.ebp), v); }
  f_12033241(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033241(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033241); }
 let mut pc: u32 = 0x12033241;
 loop { match pc {
 0x12033241 => {
  { cpu.sub32(cpu.r32(0x1203d874u32), 0x0u32); }
  if !cpu.zf { pc = 0x120332d4; continue; }
  pc = 0x1203324e; continue;
 }
 0x1203324e => {
  { let v = cpu.st(0); cpu.wf64(0xfffffd30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x120332f2; continue; }
  pc = 0x12033262; continue;
 }
 0x12033262 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x120332dc; continue; }
  pc = 0x12033266; continue;
 }
 0x12033266 => {
  { cpu.sub8((cpu.eax as u8), 0xfeu8); }
  if cpu.zf { pc = 0x120332dc; continue; }
  pc = 0x1203326a; continue;
 }
 0x1203326a => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x120332d4; continue; }
  pc = 0x1203326e; continue;
 }
 0x1203326e => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff72u32.wrapping_add(cpu.ebp), v); }
  pc = 0x12033277; continue;
 }
 0x12033277 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffff76u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0xfffffd38u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x120332a9; continue; }
  pc = 0x1203328f; continue;
 }
 0x1203328f => {
  { cpu.df = false; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = 0xffffff7au32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x120332a9; continue; }
  pc = 0x120332a1; continue;
 }
 0x120332a1 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = 0xffffff82u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  pc = 0x120332a9; continue;
 }
 0x120332a9 => {
  { let v = cpu.st(0); cpu.wf64(0xffffff8au32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = 0xffffff72u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120332cc); f_120344f4(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.rf64(0xffffff8au32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  pc = 0x120332d4; continue;
 }
 0x120332d4 => {
  { cpu.fpu_control = cpu.r16(0xffffff5cu32.wrapping_add(cpu.ebp)); }
  {  }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120332dc => {
  { let v: u16 = cpu.r16(0xfffffd36u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x12033317; continue; }
  pc = 0x120332ec; continue;
 }
 0x120332ec => {
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if cpu.zf { pc = 0x1203334c; continue; }
  pc = 0x120332f2; continue;
 }
 0x120332f2 => {
  { let v: u16 = cpu.r16(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x120332d4; continue; }
  pc = 0x120332ff; continue;
 }
 0x120332ff => {
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x120332d4; continue; }
  pc = 0x12033308; continue;
 }
 0x12033308 => {
  { let v: u32 = 0x8u32; cpu.w32(0xffffff72u32.wrapping_add(cpu.ebp), v); }
  pc = 0x12033277; continue;
 }
 0x12033317 => {
  { let v: u32 = 0x4u32; cpu.w32(0xffffff72u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf64(0x12036b28u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fscale(); }
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x12036b18u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x12033277; continue; }
  pc = 0x12033341; continue;
 }
 0x12033341 => {
  { let v = cpu.st(0) * cpu.rf64(0x12036b38u32); cpu.set_st(0, v); }
  pc = 0x12033277; continue;
 }
 0x1203334c => {
  { let v: u32 = 0x3u32; cpu.w32(0xffffff72u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf64(0x12036b20u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fscale(); }
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x12036b10u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x12033277; continue; }
  pc = 0x12033376; continue;
 }
 0x12033376 => {
  { let v = cpu.st(0) * cpu.rf64(0x12036b30u32); cpu.set_st(0, v); }
  pc = 0x12033277; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033381(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033381); }
 let mut pc: u32 = 0x12033381;
 loop { match pc {
 0x12033381 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x120333b8; continue; }
  pc = 0x12033399; continue;
 }
 0x12033399 => {
  { let v: u16 = (cpu.ebx as u16) | 0x7fffu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ebx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff6u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf80(0xfffffff6u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  pc = 0x120333b5; continue;
 }
 0x120333b5 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120333b8 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  pc = 0x120333b5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120333bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120333bd); }
 let mut pc: u32 = 0x120333bd;
 loop { match pc {
 0x120333bd => {
  { cpu.c2 = false; let v = cpu.st(0).cos(); cpu.set_st(0, v); }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  {  }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.pf() { pc = 0x120333dc; continue; }
  pc = 0x120333c6; continue;
 }
 0x120333c6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120333dc => {
  { let v: u8 = 0x5u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120333c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120333c7); }
 let mut pc: u32 = 0x120333c7;
 loop { match pc {
 0x120333c7 => {
  { cpu.c2 = false; let v = cpu.st(0).sin(); cpu.set_st(0, v); }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  {  }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.pf() { pc = 0x120333dc; continue; }
  pc = 0x120333d0; continue;
 }
 0x120333d0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120333dc => {
  { let v: u8 = 0x5u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120333d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120333d1); }
 let mut pc: u32 = 0x120333d1;
 loop { match pc {
 0x120333d1 => {
  { cpu.c2 = false; let a = cpu.st(0); cpu.set_st(0, a.tan()); cpu.fpush(1.0); }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.pf() { pc = 0x120333dc; continue; }
  pc = 0x120333db; continue;
 }
 0x120333db => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120333dc => {
  { let v: u8 = 0x5u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120333fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120333fb); }
 let mut pc: u32 = 0x120333fb;
 loop { match pc {
 0x12006008 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1200600e; continue; }
  pc = 0x1200600c; continue;
 }
 0x1200600c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x1200600e; continue;
 }
 0x1200600e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120333fb => {
  { let v: u8 = 0xffu8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12033407); f_120334a4(cpu);
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x12033489; continue; }
  pc = 0x1203340b; continue;
 }
 0x1203340b => {
  cpu.push(0x12033410); f_120334eb(cpu);
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fscale(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033489 => {
  cpu.push(0x1203348e); f_1203360b(cpu);
  pc = 0x12006008; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203341f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203341f); }
 let mut pc: u32 = 0x1203341f;
 loop { match pc {
 0x12006008 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1200600e; continue; }
  pc = 0x1200600c; continue;
 }
 0x1200600c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x1200600e; continue;
 }
 0x1200600e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203341f => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.rf80(0x1203e420u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  {  }
  { cpu.and8(cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)), 0x41u8); }
  if !cpu.zf { pc = 0x12033493; continue; }
  pc = 0x1203343c; continue;
 }
 0x1203343c => {
  cpu.push(0x12033441); f_120334a4(cpu);
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x12033493; continue; }
  pc = 0x12033445; continue;
 }
 0x12033445 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  cpu.push(0x1203344c); f_120334d9(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  cpu.push(0x12033453); f_120334eb(cpu);
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033493 => {
  { let v: u8 = 0x8u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1203349f); f_12033574(cpu);
  pc = 0x12006008; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033458(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033458); }
 let mut pc: u32 = 0x12033458;
 loop { match pc {
 0x12033412 => {
  { cpu.fpush(1.0); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fscale(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033458 => {
  { let v: u8 = 0xffu8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12033464); f_120334a4(cpu);
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { f_1203360b(cpu); return; }
  pc = 0x1203346c; continue;
 }
 0x1203346c => {
  cpu.push(0x12033471); f_120334d9(cpu);
  pc = 0x12033412; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033473(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033473); }
 let mut pc: u32 = 0x12033473;
 loop { match pc {
 0x12006008 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1200600e; continue; }
  pc = 0x1200600c; continue;
 }
 0x1200600c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x1200600e; continue;
 }
 0x1200600e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033473 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e490u32); cpu.fpush(v); }
  pc = 0x12006008; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033480); }
 let mut pc: u32 = 0x12033480;
 loop { match pc {
 0x12033480 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e490u32); cpu.fpush(v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120334a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120334a4); }
 let mut pc: u32 = 0x120334a4;
 loop { match pc {
 0x1203341a => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120334a4 => {
  { cpu.fpush(std::f64::consts::LOG2_E); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x120334af); f_12033628(cpu);
  { let v: u8 = !(cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x120334c3; continue; }
  pc = 0x120334ba; continue;
 }
 0x120334ba => {
  cpu.push(0x120334bf); f_120334eb(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x120334c3; continue;
 }
 0x120334c3 => {
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x1203341a; continue; }
  pc = 0x120334cc; continue;
 }
 0x120334cc => {
  { let v: u8 = !((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fscale(); }
  pc = 0x1203341a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120334d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120334d9); }
 let mut pc: u32 = 0x120334d9;
 loop { match pc {
 0x120334d9 => {
  cpu.push(0x120334de); f_120334eb(cpu);
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u8 = cpu.or8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x120334ea; continue; }
  pc = 0x120334e4; continue;
 }
 0x120334e4 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x120334ea; continue;
 }
 0x120334ea => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120334eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120334eb); }
 let mut pc: u32 = 0x120334eb;
 loop { match pc {
 0x120334eb => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { cpu.fpush(1.0); }
  { let v: u8 = cpu.or8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x120334f9; continue; }
  pc = 0x120334f3; continue;
 }
 0x120334f3 => {
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  pc = 0x120334f9; continue;
 }
 0x120334f9 => {
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120334fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120334fc); }
 let mut pc: u32 = 0x120334fc;
 loop { match pc {
 0x12005fda => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x120333e3; continue;
 }
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203341a => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120334fc => {
  { let v: u8 = 0xfeu8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x12033556; continue; }
  pc = 0x12033507; continue;
 }
 0x12033507 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  pc = 0x12033509; continue;
 }
 0x12033509 => {
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  cpu.push(0x12033510); f_12033628(cpu);
  { cpu.fpush(1.0); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.and8(cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1203352a; continue; }
  pc = 0x1203351d; continue;
 }
 0x1203351d => {
  { cpu.fpush(1.0); }
  { cpu.sub32(cpu.r32(0x1203d878u32), 0x1u32); }
  if cpu.zf { pc = 0x1203354f; continue; }
  pc = 0x12033528; continue;
 }
 0x12033528 => {
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x1203352a; continue;
 }
 0x1203352a => {
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x12033531; continue; }
  pc = 0x1203352f; continue;
 }
 0x1203352f => {
  { cpu.fscale(); }
  pc = 0x12033531; continue;
 }
 0x12033531 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x1203341a; continue; }
  pc = 0x12033539; continue;
 }
 0x12033539 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x1203341a; continue;
 }
 0x1203354f => {
  cpu.push(0x12033554); f_12030667(cpu);
  pc = 0x1203352a; continue;
 }
 0x12033556 => {
  cpu.push(0x1203355b); f_1203366b(cpu);
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x12005fda; continue; }
  pc = 0x12033563; continue;
 }
 0x12033563 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x1203356c; continue; }
  pc = 0x1203356a; continue;
 }
 0x1203356a => {
  { let v: u8 = !((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1203356c; continue;
 }
 0x1203356c => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  pc = 0x12033509; continue;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033540); }
 let mut pc: u32 = 0x12033540;
 loop { match pc {
 0x1203341a => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203350b => {
  cpu.push(0x12033510); f_12033628(cpu);
  { cpu.fpush(1.0); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.and8(cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1203352a; continue; }
  pc = 0x1203351d; continue;
 }
 0x1203351d => {
  { cpu.fpush(1.0); }
  { cpu.sub32(cpu.r32(0x1203d878u32), 0x1u32); }
  if cpu.zf { pc = 0x1203354f; continue; }
  pc = 0x12033528; continue;
 }
 0x12033528 => {
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x1203352a; continue;
 }
 0x1203352a => {
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x12033531; continue; }
  pc = 0x1203352f; continue;
 }
 0x1203352f => {
  { cpu.fscale(); }
  pc = 0x12033531; continue;
 }
 0x12033531 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x1203341a; continue; }
  pc = 0x12033539; continue;
 }
 0x12033539 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x1203341a; continue;
 }
 0x12033540 => {
  { let v: u8 = 0xfeu8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.fpush(std::f64::consts::LOG2_E); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x1203350b; continue;
 }
 0x1203354f => {
  cpu.push(0x12033554); f_12030667(cpu);
  pc = 0x1203352a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033574(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033574); }
 let mut pc: u32 = 0x12033574;
 loop { match pc {
 0x12033574 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033579(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033579); }
 let mut pc: u32 = 0x12033579;
 loop { match pc {
 0x12033579 => {
  { let v: u8 = 0x2u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e49au32); cpu.fpush(v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033589(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033589); }
 let mut pc: u32 = 0x12033589;
 loop { match pc {
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033589 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x120333e3; continue; }
  pc = 0x12033591; continue;
 }
 0x12033591 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033592(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033592); }
 let mut pc: u32 = 0x12033592;
 loop { match pc {
 0x12005fda => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x120333e3; continue;
 }
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033592 => {
  { cpu.fpush(std::f64::consts::LOG10_2); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x12005fda; continue; }
  pc = 0x1203359e; continue;
 }
 0x1203359e => {
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120335a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120335a1); }
 let mut pc: u32 = 0x120335a1;
 loop { match pc {
 0x12005fda => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x120333e3; continue;
 }
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033594 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x12005fda; continue; }
  pc = 0x1203359e; continue;
 }
 0x1203359e => {
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120335a1 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  pc = 0x12033594; continue;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120335a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120335a5); }
 let mut pc: u32 = 0x120335a5;
 loop { match pc {
 0x12033572 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  f_12033574(cpu); return;
 }
 0x120335a5 => {
  pc = 0x12033572; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120335a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120335a7); }
 let mut pc: u32 = 0x120335a7;
 loop { match pc {
 0x120335a7 => {
  cpu.push(0x120335ac); f_1203366b(cpu);
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x120335c2; continue; }
  pc = 0x120335b4; continue;
 }
 0x120335b4 => {
  { cpu.fpush(0.0); }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x120335c1; continue; }
  pc = 0x120335bb; continue;
 }
 0x120335bb => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x120335c1; continue; }
  pc = 0x120335bf; continue;
 }
 0x120335bf => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x120335c1; continue;
 }
 0x120335c1 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120335c2 => {
  { let v: u8 = 0x2u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf80(0x1203e490u32); cpu.fpush(v); }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x120335c1; continue; }
  pc = 0x120335d4; continue;
 }
 0x120335d4 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x120335c1; continue; }
  pc = 0x120335d8; continue;
 }
 0x120335d8 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x120335c1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203360b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203360b); }
 let mut pc: u32 = 0x1203360b;
 loop { match pc {
 0x1203360b => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e490u32); cpu.fpush(v); }
  { let v: u8 = 0x3u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203361b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203361b); }
 let mut pc: u32 = 0x1203361b;
 loop { match pc {
 0x12033606 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203361b => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x12033606; continue; }
  pc = 0x1203361f; continue;
 }
 0x1203361f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e490u32); cpu.fpush(v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033628(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033628); }
 let mut pc: u32 = 0x12033628;
 loop { match pc {
 0x120335dc => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  {  }
  { cpu.and8(cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x120335ff; continue; }
  pc = 0x120335f0; continue;
 }
 0x120335f0 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e490u32); cpu.fpush(v); }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x120335fe; continue; }
  pc = 0x120335fc; continue;
 }
 0x120335fc => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x120335fe; continue;
 }
 0x120335fe => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120335ff => {
  { let v: u8 = 0x4u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033628 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.rf80(0x1203e4a4u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  {  }
  { cpu.and8(cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)), 0x41u8); }
  if !cpu.zf { pc = 0x120335dc; continue; }
  pc = 0x12033645; continue;
 }
 0x12033645 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = 2f64.powf(cpu.st(0)) - 1.0; cpu.set_st(0, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203366b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203366b); }
 let mut pc: u32 = 0x1203366b;
 loop { match pc {
 0x1203366b => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x12033691; continue; }
  pc = 0x12033677; continue;
 }
 0x12033677 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x1203e4aeu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.zf { pc = 0x12033698; continue; }
  pc = 0x1203368b; continue;
 }
 0x1203368b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12033690; continue;
 }
 0x12033690 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033691 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x12033690; continue;
 }
 0x12033698 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x12033690; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203369f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203369f); }
 let mut pc: u32 = 0x1203369f;
 loop { match pc {
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203369f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  {  }
  cpu.unimplemented(0x120336b3, "Fnsave");
  cpu.push(0x120336bb); f_12034640(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  cpu.unimplemented(0x120336be, "Frstor");
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x74u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120336d7; continue; }
  pc = 0x120336cb; continue;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 0x120336d7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120336d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120336d8); }
 let mut pc: u32 = 0x120336d8;
 loop { match pc {
 0x120336d8 => {
  { cpu.sub8(cpu.r8(0xeu32.wrapping_add(cpu.edx)), 0x5u8); }
  if !cpu.zf { pc = 0x12033739; continue; }
  pc = 0x120336de; continue;
 }
 0x120336de => {
  { let v: u16 = cpu.r16(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | 0x2u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xfeu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x3fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x120336ed; continue;
 }
 0x120336ed => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xffffff5eu32.wrapping_add(cpu.ebp), v); }
  { cpu.fpu_control = cpu.r16(0xffffff5eu32.wrapping_add(cpu.ebp)); }
  { let v: u32 = 0x1203e58cu32; cpu.ebx = v; }
  { cpu.fxam(); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rol8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x404u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  pc = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); continue;
 }
 0x12033739 => {
  { let v: u16 = 0x133fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x120336ed; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203373f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203373f); }
 let mut pc: u32 = 0x1203373f;
 loop { match pc {
 0x1203373f => {
  { cpu.sub8(cpu.r8(0xeu32.wrapping_add(cpu.edx)), 0x5u8); }
  if !cpu.zf { pc = 0x120337c9; continue; }
  pc = 0x12033749; continue;
 }
 0x12033749 => {
  { let v: u16 = cpu.r16(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | 0x2u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xfeu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x3fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12033758; continue;
 }
 0x12033758 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xffffff5eu32.wrapping_add(cpu.ebp), v); }
  { cpu.fpu_control = cpu.r16(0xffffff5eu32.wrapping_add(cpu.ebp)); }
  { let v: u32 = 0x1203e58cu32; cpu.ebx = v; }
  { cpu.fxam(); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u8 = cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.fxam(); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u8 = cpu.r8(0xffffff61u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.sar8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.rol8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rol8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) | ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x404u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  pc = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); continue;
 }
 0x120337c9 => {
  { let v: u16 = 0x133fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x12033758; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120337cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120337cf); }
 let mut pc: u32 = 0x120337cf;
 loop { match pc {
 0x12033606 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120337cf => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x12033606; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120337d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120337d6); }
 let mut pc: u32 = 0x120337d6;
 loop { match pc {
 0x120337d6 => {
  { let v = cpu.st(0); cpu.wf80(0xffffff62u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf80(0xffffff62u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0xffffff69u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x120337f3; continue; }
  pc = 0x120337eb; continue;
 }
 0x120337eb => {
  { let v: u8 = 0x7u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120337f3 => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0) + cpu.rf64(0x1203e584u32); cpu.set_st(0, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033801(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033801); }
 let mut pc: u32 = 0x12033801;
 loop { match pc {
 0x12033801 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xffffff62u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf80(0xffffff62u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0xffffff69u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x12033822; continue; }
  pc = 0x12033818; continue;
 }
 0x12033818 => {
  { let v: u8 = 0x7u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1203381f; continue;
 }
 0x1203381f => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033822 => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1203381f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203382b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203382b); }
 let mut pc: u32 = 0x1203382b;
 loop { match pc {
 0x1203382b => {
  { let v = cpu.st(0); cpu.wf80(0xffffff62u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf80(0xffffff62u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0xffffff69u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x12033861; continue; }
  pc = 0x12033840; continue;
 }
 0x12033840 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xffffff62u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf80(0xffffff62u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0xffffff69u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x12033861; continue; }
  pc = 0x12033857; continue;
 }
 0x12033857 => {
  { let v: u8 = 0x7u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1203385e; continue;
 }
 0x1203385e => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033861 => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1203385e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033875(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033875); }
 let mut pc: u32 = 0x12033875;
 loop { match pc {
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 0x12033875 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x120333e3; continue; }
  pc = 0x1203387d; continue;
 }
 0x1203387d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203387e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203387e); }
 let mut pc: u32 = 0x1203387e;
 loop { match pc {
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 0x1203386a => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x120333e3; continue; }
  pc = 0x12033872; continue;
 }
 0x12033872 => {
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203387e => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  {  }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x1203386a; continue; }
  pc = 0x12033887; continue;
 }
 0x12033887 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033888(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033888); }
 let mut pc: u32 = 0x12033888;
 loop { match pc {
 0x12033888 => {
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if !cpu.zf { pc = 0x120338b6; continue; }
  pc = 0x12033891; continue;
 }
 0x12033891 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x120338b2); f_1203415b(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120338b6 => {
  cpu.push(0x120338bb); f_120133b4(cpu);
  { let v = 0xffffu32; cpu.push(v); }
  { let v: u32 = 0x21u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120338d0); f_12034452(cpu);
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120338d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120338d8); }
 let mut pc: u32 = 0x120338d8;
 loop { match pc {
 0x120338d8 => {
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  if !cpu.zf { pc = 0x12033923; continue; }
  pc = 0x120338f0; continue;
 }
 0x120338f0 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1203391f); f_1203415b(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033923 => {
  cpu.push(0x12033928); f_120133b4(cpu);
  { let v = 0xffffu32; cpu.push(v); }
  { let v: u32 = 0x21u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1203393d); f_12034452(cpu);
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033948(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033948); }
 let mut pc: u32 = 0x12033948;
 loop { match pc {
 0x12033948 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x58u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12033961); f_12033e94(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120339d5; continue; }
  pc = 0x12033968; continue;
 }
 0x12033968 => {
  { let v: u32 = 0x74u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)) & 0xfffffffeu32; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1203398f); f_12033aef(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  pc = 0x12033992; continue;
 }
 0x12033992 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12033998); f_1203425b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if !cpu.zf { pc = 0x120339db; continue; }
  pc = 0x120339a4; continue;
 }
 0x120339a4 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120339db; continue; }
  pc = 0x120339a8; continue;
 }
 0x120339a8 => {
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120339cc); f_1203415b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x58u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120339d5 => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x12033992; continue;
 }
 0x120339db => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120339e1); f_12034209(cpu);
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120339f6); f_12034452(cpu);
  { let v = cpu.rf64(0x7cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x58u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033a03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033a03); }
 let mut pc: u32 = 0x12033a03;
 loop { match pc {
 0x12033a03 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x58u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12033a1c); f_12033e94(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12033abe; continue; }
  pc = 0x12033a27; continue;
 }
 0x12033a27 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)) | 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe3u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x7cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12033a6c); f_12033aef(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  pc = 0x12033a6f; continue;
 }
 0x12033a6f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12033a75); f_1203425b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203b0f0u32), 0x0u32); }
  if !cpu.zf { pc = 0x12033ac4; continue; }
  pc = 0x12033a81; continue;
 }
 0x12033a81 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12033ac4; continue; }
  pc = 0x12033a85; continue;
 }
 0x12033a85 => {
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x7cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12033ab5); f_1203415b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x58u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033abe => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x12033a6f; continue;
 }
 0x12033ac4 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12033aca); f_12034209(cpu);
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12033adf); f_12034452(cpu);
  { let v = cpu.rf64(0x84u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x58u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033aef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033aef); }
 let mut pc: u32 = 0x12033aef;
 loop { match pc {
 0x12033aef => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x12033d66; continue; }
  pc = 0x12033b19; continue;
 }
 0x12033b19 => {
  { let v: u32 = 0xc000008fu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)) | 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033b26; continue;
 }
 0x12033b26 => {
  { cpu.and8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x12033b38; continue; }
  pc = 0x12033b2b; continue;
 }
 0x12033b2b => {
  { let v: u32 = 0xc0000093u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)) | 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033b38; continue;
 }
 0x12033b38 => {
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x12033b4a; continue; }
  pc = 0x12033b3d; continue;
 }
 0x12033b3d => {
  { let v: u32 = 0xc0000091u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)) | 0x4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033b4a; continue;
 }
 0x12033b4a => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x12033b5c; continue; }
  pc = 0x12033b4f; continue;
 }
 0x12033b4f => {
  { let v: u32 = 0xc000008eu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)) | 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033b5c; continue;
 }
 0x12033b5c => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x12033b6e; continue; }
  pc = 0x12033b61; continue;
 }
 0x12033b61 => {
  { let v: u32 = 0xc0000090u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)) | 0x10u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033b6e; continue;
 }
 0x12033b6e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x12033c13); f_1203442f(cpu);
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x12033c1f; continue; }
  pc = 0x12033c17; continue;
 }
 0x12033c17 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)) | 0x10u32; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12033c1f; continue;
 }
 0x12033c1f => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x12033c2b; continue; }
  pc = 0x12033c23; continue;
 }
 0x12033c23 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)) | 0x8u32; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12033c2b; continue;
 }
 0x12033c2b => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x12033c37; continue; }
  pc = 0x12033c2f; continue;
 }
 0x12033c2f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)) | 0x4u32; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12033c37; continue;
 }
 0x12033c37 => {
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x12033c43; continue; }
  pc = 0x12033c3b; continue;
 }
 0x12033c3b => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)) | 0x2u32; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12033c43; continue;
 }
 0x12033c43 => {
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x12033c4f; continue; }
  pc = 0x12033c47; continue;
 }
 0x12033c47 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)) | 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x12033c4f; continue;
 }
 0x12033c4f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xc00u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x400u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12033d6f; continue; }
  pc = 0x12033c61; continue;
 }
 0x12033c61 => {
  if cpu.zf { pc = 0x12033d99; continue; }
  pc = 0x12033c67; continue;
 }
 0x12033c67 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12033d8d; continue; }
  pc = 0x12033c6f; continue;
 }
 0x12033c6f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x300u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12033dc7; continue; }
  pc = 0x12033c7c; continue;
 }
 0x12033c7c => {
  { cpu.sub32(cpu.eax, 0x200u32); }
  if cpu.zf { pc = 0x12033dda; continue; }
  pc = 0x12033c87; continue;
 }
 0x12033c87 => {
  { cpu.sub32(cpu.eax, 0x300u32); }
  if cpu.zf { pc = 0x12033df1; continue; }
  pc = 0x12033c92; continue;
 }
 0x12033c92 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1ffe0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)) | 0x1u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffffffe3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edx)) | 0x1u32; cpu.w32(0x50u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffffffe3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x12033d0e); f_12034440(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041000u32); cpu.push(0x12033d1e); cpu.call(t);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x12033d2d; continue; }
  pc = 0x12033d2a; continue;
 }
 0x12033d2a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) & 0xfffffffeu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033d2d; continue;
 }
 0x12033d2d => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x8u8); }
  if cpu.zf { pc = 0x12033d35; continue; }
  pc = 0x12033d32; continue;
 }
 0x12033d32 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) & 0xfffffffbu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033d35; continue;
 }
 0x12033d35 => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x12033d3d; continue; }
  pc = 0x12033d3a; continue;
 }
 0x12033d3a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) & 0xfffffff7u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033d3d; continue;
 }
 0x12033d3d => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x12033d45; continue; }
  pc = 0x12033d42; continue;
 }
 0x12033d42 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) & 0xffffffefu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033d45; continue;
 }
 0x12033d45 => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x12033d4d; continue; }
  pc = 0x12033d4a; continue;
 }
 0x12033d4a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) & 0xffffffdfu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033d4d; continue;
 }
 0x12033d4d => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12033e03; continue; }
  pc = 0x12033d5f; continue;
 }
 0x12033d5f => {
  pc = cpu.r32(0x12033e84u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x12033d66 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x12033b26; continue;
 }
 0x12033d6f => {
  { cpu.sub32(cpu.eax, 0x800u32); }
  if cpu.zf { pc = 0x12033db0; continue; }
  pc = 0x12033d76; continue;
 }
 0x12033d76 => {
  { cpu.sub32(cpu.eax, 0xc00u32); }
  if !cpu.zf { pc = 0x12033c6f; continue; }
  pc = 0x12033d81; continue;
 }
 0x12033d81 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)) | 0x3u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033c6f; continue;
 }
 0x12033d8d => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)) & 0xfffffffcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033c6f; continue;
 }
 0x12033d99 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffffdu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12033c6f; continue;
 }
 0x12033db0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12033c6f; continue;
 }
 0x12033dc7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffffebu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033c92; continue;
 }
 0x12033dda => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12033c92; continue;
 }
 0x12033df1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)) & 0xffffffe3u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12033c92; continue;
 }
 0x12033dfd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) & 0xfffff3ffu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033e03; continue;
 }
 0x12033e03 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1cu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12033e5b; continue; }
  pc = 0x12033e13; continue;
 }
 0x12033e13 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x12033e6b; continue; }
  pc = 0x12033e18; continue;
 }
 0x12033e18 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x12033e7b; continue; }
  pc = 0x12033e1d; continue;
 }
 0x12033e1d => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033e33 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffff7ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x400u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033e03; continue;
 }
 0x12033e43 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffbffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x800u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033e03; continue;
 }
 0x12033e53 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | 0xc00u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033e03; continue;
 }
 0x12033e5b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffff3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x300u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033e1d; continue;
 }
 0x12033e6b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffff3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x200u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033e1d; continue;
 }
 0x12033e7b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) & 0xfffff3ffu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033e1d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033e84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033e84); }
 let mut pc: u32 = 0x12033e84;
 loop { match pc {
 0x12033e84 => {
  { cpu.df = true; }
  { cpu.sub32(cpu.eax, 0x3e331203u32); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  f_12033e94(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033e94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033e94); }
 let mut pc: u32 = 0x12033e94;
 loop { match pc {
 0x12033e94 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x1fu32; cpu.esi = v; }
  { cpu.and32(cpu.edi, 0x8u32); }
  if cpu.zf { pc = 0x12033eeb; continue; }
  pc = 0x12033eac; continue;
 }
 0x12033eac => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x12033eeb; continue; }
  pc = 0x12033eb3; continue;
 }
 0x12033eb3 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi & 0xfffffff7u32; cpu.esi = v; }
  cpu.push(0x12033ebd); f_12034484(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12033ec0; continue;
 }
 0x12033ec0 => {
  { cpu.and32(cpu.edi, 0x10u32); }
  if cpu.zf { pc = 0x12033edc; continue; }
  pc = 0x12033ec8; continue;
 }
 0x12033ec8 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x20u8); }
  if cpu.zf { pc = 0x12033edc; continue; }
  pc = 0x12033ecf; continue;
 }
 0x12033ecf => {
  { let v = 0x20u32; cpu.push(v); }
  { let v: u32 = cpu.esi & 0xffffffefu32; cpu.esi = v; }
  cpu.push(0x12033ed9); f_12034484(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12033edc; continue;
 }
 0x12033edc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033eeb => {
  { cpu.and32(cpu.edi, 0x4u32); }
  if cpu.zf { pc = 0x12033f09; continue; }
  pc = 0x12033ef3; continue;
 }
 0x12033ef3 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x12033f09; continue; }
  pc = 0x12033efa; continue;
 }
 0x12033efa => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.esi & 0xfffffffbu32; cpu.esi = v; }
  cpu.push(0x12033f04); f_12034484(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12033ec0; continue;
 }
 0x12033f09 => {
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x1203404a; continue; }
  pc = 0x12033f15; continue;
 }
 0x12033f15 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x8u8); }
  if cpu.zf { pc = 0x1203404a; continue; }
  pc = 0x12033f20; continue;
 }
 0x12033f20 => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12033f27); f_12034484(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax & 0xc00u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x400u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12033f48; continue; }
  pc = 0x12033f3a; continue;
 }
 0x12033f3a => {
  if cpu.zf { pc = 0x12033f9f; continue; }
  pc = 0x12033f3c; continue;
 }
 0x12033f3c => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12033f66; continue; }
  pc = 0x12033f40; continue;
 }
 0x12033f40 => {
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  pc = 0x12033ec0; continue;
 }
 0x12033f48 => {
  { cpu.sub32(cpu.eax, 0x800u32); }
  if cpu.zf { pc = 0x12033fd8; continue; }
  pc = 0x12033f53; continue;
 }
 0x12033f53 => {
  { cpu.sub32(cpu.eax, 0xc00u32); }
  if cpu.zf { pc = 0x12034011; continue; }
  pc = 0x12033f5e; continue;
 }
 0x12033f5e => {
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  pc = 0x12033ec0; continue;
 }
 0x12033f66 => {
  { cpu.fpush(0.0); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x0u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x12033f8d; continue; }
  pc = 0x12033f75; continue;
 }
 0x12033f75 => {
  { let v: u32 = cpu.r32(0x1203e69cu32); cpu.eax = v; }
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1203e698u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033ec0; continue;
 }
 0x12033f8d => {
  { let v = cpu.rf64(0x1203e698u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  pc = 0x12033ec0; continue;
 }
 0x12033f9f => {
  { cpu.fpush(0.0); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x0u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x12033fc6; continue; }
  pc = 0x12033fae; continue;
 }
 0x12033fae => {
  { let v: u32 = cpu.r32(0x1203e6acu32); cpu.eax = v; }
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1203e6a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033ec0; continue;
 }
 0x12033fc6 => {
  { let v = cpu.rf64(0x1203e698u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  pc = 0x12033ec0; continue;
 }
 0x12033fd8 => {
  { cpu.fpush(0.0); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x0u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x12033fff; continue; }
  pc = 0x12033fe7; continue;
 }
 0x12033fe7 => {
  { let v: u32 = cpu.r32(0x1203e69cu32); cpu.eax = v; }
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1203e698u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033ec0; continue;
 }
 0x12033fff => {
  { let v = cpu.rf64(0x1203e6a8u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  pc = 0x12033ec0; continue;
 }
 0x12034011 => {
  { cpu.fpush(0.0); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x0u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x12034038; continue; }
  pc = 0x12034020; continue;
 }
 0x12034020 => {
  { let v: u32 = cpu.r32(0x1203e6acu32); cpu.eax = v; }
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1203e6a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12033ec0; continue;
 }
 0x12034038 => {
  { let v = cpu.rf64(0x1203e6a8u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.esi & 0xfffffffeu32; cpu.esi = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  pc = 0x12033ec0; continue;
 }
 0x1203404a => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x12033ec0; continue; }
  pc = 0x12034056; continue;
 }
 0x12034056 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x12033ec0; continue; }
  pc = 0x12034061; continue;
 }
 0x12034061 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.edi, 0x10u32); }
  if cpu.zf { pc = 0x12034070; continue; }
  pc = 0x1203406b; continue;
 }
 0x1203406b => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x12034070; continue;
 }
 0x12034070 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  if cpu.zf { pc = 0x12034151; continue; }
  pc = 0x12034084; continue;
 }
 0x12034084 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12034095); f_1203433e(cpu);
  { let v = cpu.st(0); cpu.wf64(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x600u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xfffffbceu32); }
  if (cpu.sf == cpu.of) { pc = 0x120340e0; continue; }
  pc = 0x120340ae; continue;
 }
 0x120340ae => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x120340bd; continue;
 }
 0x120340bd => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x120340ca; continue;
 }
 0x120340ca => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x120340d8; continue; }
  pc = 0x120340ce; continue;
 }
 0x120340ce => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x120340d5); f_12034484(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120340d8; continue;
 }
 0x120340d8 => {
  { let v: u32 = cpu.esi & 0xfffffffdu32; cpu.esi = v; }
  pc = 0x12033ec0; continue;
 }
 0x120340e0 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x120340f4; continue; }
  pc = 0x120340f2; continue;
 }
 0x120340f2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x120340f4; continue;
 }
 0x120340f4 => {
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esp)) & 0xfu16; cpu.w16(0x16u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esp)) | 0x10u8; cpu.w8(0x16u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ecx, 0xfffffc03u32); }
  if (cpu.sf == cpu.of) { pc = 0x1203413a; continue; }
  pc = 0x12034107; continue;
 }
 0x12034107 => {
  { let v: u32 = 0xfffffc03u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x12034113; continue;
 }
 0x12034113 => {
  { cpu.and32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), cpu.ecx); }
  if cpu.zf { pc = 0x1203411f; continue; }
  pc = 0x12034119; continue;
 }
 0x12034119 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x1203411f; continue; }
  pc = 0x1203411d; continue;
 }
 0x1203411d => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1203411f; continue;
 }
 0x1203411f => {
  { let v: u32 = cpu.shr32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 1u32); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), cpu.ecx); }
  if cpu.zf { pc = 0x12034132; continue; }
  pc = 0x1203412a; continue;
 }
 0x1203412a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)) | 0x80000000u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x12034132; continue;
 }
 0x12034132 => {
  { let v: u32 = cpu.shr32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 1u32); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x12034113; continue; }
  pc = 0x1203413a; continue;
 }
 0x1203413a => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x120340bd; continue; }
  pc = 0x12034142; continue;
 }
 0x12034142 => {
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x120340bd; continue;
 }
 0x12034151 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x120340ca; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203415b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203415b); }
 let mut pc: u32 = 0x1203415b;
 loop { match pc {
 0x1203415b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12034169); f_12034235(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120341e1; continue; }
  pc = 0x12034174; continue;
 }
 0x12034174 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x120341bb); f_12034452(cpu);
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120341c8); f_12011e60(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120341d8; continue; }
  pc = 0x120341cf; continue;
 }
 0x120341cf => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120341d5); f_12034209(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120341d8; continue;
 }
 0x120341d8 => {
  { let v = cpu.rf64(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120341e1 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120341f0); f_12034452(cpu);
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120341fd); f_12034209(cpu);
  { let v = cpu.rf64(0x44u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12034209(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12034209); }
 let mut pc: u32 = 0x12034209;
 loop { match pc {
 0x12034209 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1203421d; continue; }
  pc = 0x12034212; continue;
 }
 0x12034212 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x12034234; continue; }
  pc = 0x12034217; continue;
 }
 0x12034217 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12034229; continue; }
  pc = 0x1203421c; continue;
 }
 0x1203421c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203421d => {
  cpu.push(0x12034222); f_120133b4(cpu);
  { let v: u32 = 0x21u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034229 => {
  cpu.push(0x1203422e); f_120133b4(cpu);
  { let v: u32 = 0x22u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12034234; continue;
 }
 0x12034234 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12034235(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12034235); }
 let mut pc: u32 = 0x12034235;
 loop { match pc {
 0x12034235 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1203e5c0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x12034240; continue;
 }
 0x12034240 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.ecx); }
  if cpu.zf { pc = 0x12034253; continue; }
  pc = 0x12034244; continue;
 }
 0x12034244 => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x1203e698u32); }
  if cpu.cf { pc = 0x12034240; continue; }
  pc = 0x12034250; continue;
 }
 0x12034250 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034253 => {
  { let v: u32 = cpu.r32(0x1203e5c4u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203425b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203425b); }
 let mut pc: u32 = 0x1203425b;
 loop { match pc {
 0x1203425b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x12034269; continue; }
  pc = 0x12034263; continue;
 }
 0x12034263 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034269 => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x12034273; continue; }
  pc = 0x1203426d; continue;
 }
 0x1203426d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034273 => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1203427d; continue; }
  pc = 0x12034277; continue;
 }
 0x12034277 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203427d => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x12034287; continue; }
  pc = 0x12034281; continue;
 }
 0x12034281 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034287 => {
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203428d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203428d); }
 let mut pc: u32 = 0x1203428d;
 loop { match pc {
 0x1203428d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3feu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x800fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120342c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120342c6); }
 let mut pc: u32 = 0x120342c6;
 loop { match pc {
 0x120342c6 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x3feu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120342db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120342db); }
 let mut pc: u32 = 0x120342db;
 loop { match pc {
 0x120342db => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x7ff00000u32); }
  if !cpu.zf { pc = 0x120342f2; continue; }
  pc = 0x120342e5; continue;
 }
 0x120342e5 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x120342f2; continue; }
  pc = 0x120342ec; continue;
 }
 0x120342ec => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120342f2 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xfff00000u32); }
  if !cpu.zf { pc = 0x12034309; continue; }
  pc = 0x120342fc; continue;
 }
 0x120342fc => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x12034309; continue; }
  pc = 0x12034303; continue;
 }
 0x12034303 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034309 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7ff8u16); }
  if !cpu.zf { pc = 0x1203431e; continue; }
  pc = 0x12034318; continue;
 }
 0x12034318 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203431e => {
  { cpu.sub16((cpu.eax as u16), 0x7ff0u16); }
  if !cpu.zf { pc = 0x1203433b; continue; }
  pc = 0x12034324; continue;
 }
 0x12034324 => {
  { cpu.and32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x7ffffu32); }
  if !cpu.zf { pc = 0x12034335; continue; }
  pc = 0x1203432e; continue;
 }
 0x1203432e => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1203433b; continue; }
  pc = 0x12034335; continue;
 }
 0x12034335 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203433b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203433e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203433e); }
 let mut pc: u32 = 0x1203433e;
 loop { match pc {
 0x1203433e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x1203436c; continue; }
  pc = 0x12034352; continue;
 }
 0x12034352 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x1203435c; continue;
 }
 0x1203435c => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203436c => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.esi as u16), 0x7ff0u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x12034405; continue; }
  pc = 0x1203437c; continue;
 }
 0x1203437c => {
  { cpu.and32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0xfffffu32); }
  if !cpu.zf { pc = 0x1203438d; continue; }
  pc = 0x12034386; continue;
 }
 0x12034386 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12034405; continue; }
  pc = 0x1203438d; continue;
 }
 0x1203438d => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = 0xfffffc03u32; cpu.esi = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if cpu.zf { pc = 0x120343a6; continue; }
  pc = 0x120343a4; continue;
 }
 0x120343a4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x120343a6; continue;
 }
 0x120343a6 => {
  { cpu.and8(cpu.r8(0x1au32.wrapping_add(cpu.esp)), 0x10u8); }
  if !cpu.zf { pc = 0x120343d8; continue; }
  pc = 0x120343ad; continue;
 }
 0x120343ad => {
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.edi = v; }
  pc = 0x120343bc; continue;
 }
 0x120343bc => {
  { let v: u32 = cpu.shl32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 1u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), cpu.ecx); }
  if cpu.zf { pc = 0x120343cb; continue; }
  pc = 0x120343c7; continue;
 }
 0x120343c7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)) | cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x120343cb; continue;
 }
 0x120343cb => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 1u32); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.and16(cpu.r16(0x1au32.wrapping_add(cpu.esp)), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x120343bc; continue; }
  pc = 0x120343d8; continue;
 }
 0x120343d8 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)) & 0xffefu16; cpu.w16(0x1au32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120343e8; continue; }
  pc = 0x120343e3; continue;
 }
 0x120343e3 => {
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.esp)) | 0x80u8; cpu.w8(0x1bu32.wrapping_add(cpu.esp), v); }
  pc = 0x120343e8; continue;
 }
 0x120343e8 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120343f9); f_1203428d(cpu);
  { let v = cpu.st(0); cpu.wf64(0x14u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1203435c; continue;
 }
 0x12034405 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = cpu.shr16((cpu.esi as u16), 4u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = (cpu.esi as u16) as i16 as i32 as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x3feu32); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12034423); f_1203428d(cpu);
  { let v = cpu.st(0); cpu.wf64(0x14u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1203435c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203442f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203442f); }
 let mut pc: u32 = 0x1203442f;
 loop { match pc {
 0x1203442f => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12034440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12034440); }
 let mut pc: u32 = 0x12034440;
 loop { match pc {
 0x12034440 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.fstatus(); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  {  }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12034452(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12034452); }
 let mut pc: u32 = 0x12034452;
 loop { match pc {
 0x12034452 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = !(cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) | (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x2u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12034484(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12034484); }
 let mut pc: u32 = 0x12034484;
 loop { match pc {
 0x12034484 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x1203449b; continue; }
  pc = 0x12034490; continue;
 }
 0x12034490 => {
  { let v = cpu.rf80(0x1203e6c0u32); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  {  }
  pc = 0x1203449b; continue;
 }
 0x1203449b => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x120344b1; continue; }
  pc = 0x120344a0; continue;
 }
 0x120344a0 => {
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = cpu.rf80(0x1203e6c0u32); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  {  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  pc = 0x120344b1; continue;
 }
 0x120344b1 => {
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x120344c1; continue; }
  pc = 0x120344b6; continue;
 }
 0x120344b6 => {
  { let v = cpu.rf80(0x1203e6d0u32); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  {  }
  pc = 0x120344c1; continue;
 }
 0x120344c1 => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x120344cf; continue; }
  pc = 0x120344c6; continue;
 }
 0x120344c6 => {
  { cpu.fpush(0.0); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  {  }
  pc = 0x120344cf; continue;
 }
 0x120344cf => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if cpu.zf { pc = 0x120344db; continue; }
  pc = 0x120344d4; continue;
 }
 0x120344d4 => {
  { cpu.fpush(std::f64::consts::PI); }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  {  }
  pc = 0x120344db; continue;
 }
 0x120344db => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120344df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120344df); }
 let mut pc: u32 = 0x120344df;
 loop { match pc {
 0x120344df => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120344f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120344f4); }
 let mut pc: u32 = 0x120344f4;
 loop { match pc {
 0x120344f4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x5cu32); cpu.esp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120345de; continue; }
  pc = 0x12034517; continue;
 }
 0x12034517 => {
  pc = cpu.r32(0x12034620u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1203451e => {
  { let v: u32 = 0x8u32; cpu.edi = v; }
  pc = 0x12034523; continue;
 }
 0x12034523 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1203457a; continue; }
  pc = 0x12034527; continue;
 }
 0x12034527 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12034536); f_12033e94(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1203457a; continue; }
  pc = 0x1203453d; continue;
 }
 0x1203453d => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if cpu.zf { pc = 0x120345ef; continue; }
  pc = 0x1203454a; continue;
 }
 0x1203454a => {
  { cpu.sub32(cpu.ecx, 0x16u32); }
  if cpu.zf { pc = 0x120345ef; continue; }
  pc = 0x12034553; continue;
 }
 0x12034553 => {
  { cpu.sub32(cpu.ecx, 0x1du32); }
  if cpu.zf { pc = 0x120345ef; continue; }
  pc = 0x1203455c; continue;
 }
 0x1203455c => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) & 0xfffffffeu32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  pc = 0x12034561; continue;
 }
 0x12034561 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12034577); f_12033aef(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  pc = 0x1203457a; continue;
 }
 0x1203457a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12034589); f_12034452(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x8u32); }
  if cpu.zf { pc = 0x120345a4; continue; }
  pc = 0x12034593; continue;
 }
 0x12034593 => {
  { cpu.sub32(cpu.r32(0x1203b0f0u32), cpu.eax); }
  if !cpu.zf { pc = 0x120345a4; continue; }
  pc = 0x1203459b; continue;
 }
 0x1203459b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120345a1); f_12011e60(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120345a4; continue;
 }
 0x120345a4 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120345b3; continue; }
  pc = 0x120345a8; continue;
 }
 0x120345a8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120345b0); f_12034209(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120345b3; continue;
 }
 0x120345b3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120345ba => {
  { let v: u32 = 0x4u32; cpu.edi = v; }
  pc = 0x12034523; continue;
 }
 0x120345c4 => {
  { let v: u32 = 0x11u32; cpu.edi = v; }
  pc = 0x12034523; continue;
 }
 0x120345ce => {
  { let v: u32 = 0x12u32; cpu.edi = v; }
  pc = 0x12034523; continue;
 }
 0x120345d8 => {
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x120345de; continue;
 }
 0x120345de => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x12034523; continue;
 }
 0x120345e5 => {
  { let v: u32 = 0x10u32; cpu.edi = v; }
  pc = 0x12034523; continue;
 }
 0x120345ef => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)) | 0x1u32; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffffe3u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  pc = 0x12034561; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12034620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12034620); }
 let mut pc: u32 = 0x12034620;
 loop { match pc {
 0x12034620 => {
  { cpu.push(0x2b); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v: u32 = 0xc4120345u32; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  cpu.unimplemented(0x1203462c, "Into");
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v = cpu.st(0) + (cpu.r16(0x3u32.wrapping_add(cpu.ebp)) as i16 as f64); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  cpu.unimplemented(0x1203463c, "In");
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  f_12034640(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12034640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12034640); }
 let mut pc: u32 = 0x12034640;
 loop { match pc {
 0x12034640 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), 0x7ff00000u32); }
  if !cpu.zf { pc = 0x120346c9; continue; }
  pc = 0x1203465a; continue;
 }
 0x1203465a => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), cpu.esi); }
  if !cpu.zf { pc = 0x120346c9; continue; }
  pc = 0x12034660; continue;
 }
 0x12034660 => {
  { cpu.fpush(1.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x12034688; continue; }
  pc = 0x1203466d; continue;
 }
 0x1203466d => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203e69cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e698u32); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034688 => {
  { cpu.fpush(1.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x120346ad; continue; }
  pc = 0x12034699; continue;
 }
 0x12034699 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120346ad => {
  { let v: u32 = cpu.r32(0x1203e6a4u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e6a0u32); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120346c9 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), 0xfff00000u32); }
  if !cpu.zf { pc = 0x12034743; continue; }
  pc = 0x120346d3; continue;
 }
 0x120346d3 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x12034743; continue; }
  pc = 0x120346da; continue;
 }
 0x120346da => {
  { cpu.fpush(1.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x120346ff; continue; }
  pc = 0x120346e7; continue;
 }
 0x120346e7 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120346ff => {
  { cpu.fpush(1.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12034727; continue; }
  pc = 0x12034710; continue;
 }
 0x12034710 => {
  { let v: u32 = cpu.r32(0x1203e69cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e698u32); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034727 => {
  { let v: u32 = cpu.r32(0x1203e6a4u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e6a0u32); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034743 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x7ff00000u32); }
  if !cpu.zf { pc = 0x120347af; continue; }
  pc = 0x1203474d; continue;
 }
 0x1203474d => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x120347af; continue; }
  pc = 0x12034754; continue;
 }
 0x12034754 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x1203477c; continue; }
  pc = 0x12034761; continue;
 }
 0x12034761 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203e69cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e698u32); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203477c => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x120347a1; continue; }
  pc = 0x12034793; continue;
 }
 0x12034793 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120347a1 => {
  { let v: u32 = 0x3ff00000u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120347af => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0xfff00000u32); }
  if !cpu.zf { pc = 0x12034881; continue; }
  pc = 0x120347bd; continue;
 }
 0x120347bd => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x12034881; continue; }
  pc = 0x120347c8; continue;
 }
 0x120347c8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120347d7); f_12034888(cpu);
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x12034827; continue; }
  pc = 0x120347e9; continue;
 }
 0x120347e9 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x12034812; continue; }
  pc = 0x120347ee; continue;
 }
 0x120347ee => {
  { let v = cpu.rf64(0x1203e698u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x120347fa; continue;
 }
 0x120347fa => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034812 => {
  { let v: u32 = cpu.r32(0x1203e69cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e698u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x120347fa; continue;
 }
 0x12034827 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x18u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x12034870; continue; }
  pc = 0x12034834; continue;
 }
 0x12034834 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x12034864; continue; }
  pc = 0x12034839; continue;
 }
 0x12034839 => {
  { let v: u32 = cpu.r32(0x1203e6b4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203e6b0u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x1203484c; continue;
 }
 0x1203484c => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034864 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  pc = 0x1203484c; continue;
 }
 0x12034870 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x3ff00000u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12034881; continue;
 }
 0x12034881 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12034888(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12034888); }
 let mut pc: u32 = 0x12034888;
 loop { match pc {
 0x12034888 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1203489a); f_1203141a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and8((cpu.eax as u8), 0x90u8); }
  if cpu.zf { pc = 0x120348a7; continue; }
  pc = 0x120348a1; continue;
 }
 0x120348a1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120348a7 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120348b6); f_120344df(cpu);
  { let a = cpu.st(0); let b = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x12034901; continue; }
  pc = 0x120348c4; continue;
 }
 0x120348c4 => {
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x12036bf8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120348e1); f_120344df(cpu);
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x120348f8; continue; }
  pc = 0x120348ef; continue;
 }
 0x120348ef => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120348f8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12034901 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120051c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120051c0); }
 let mut pc: u32 = 0x120051c0;
 loop { match pc {
 0x120051c0 => {
  { let _ = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.fs_base)); }
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let _ = cpu.r16(0x0u32.wrapping_add(cpu.eax)); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.cf = true; }
  { let _ = cpu.r16(0x0u32.wrapping_add(cpu.eax)); }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x120051d4, "Int3");
  cpu.unimplemented(0x120051d5, "Int3");
  cpu.unimplemented(0x120051d6, "Int3");
  cpu.unimplemented(0x120051d7, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x12008f04u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0xffffff8fu32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.push(0x2b); }
  {  }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = 0x2bu16; cpu.w16(0x8f121200u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12005224, "Out");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12005228, "Bound");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.cf = false; }
  {  }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x91471200u32.wrapping_add(cpu.ecx)).wrapping_sub((cpu.edx as u8)); cpu.w8(0x91471200u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8) & 0x90u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xae120091u32); cpu.eax = v; }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12005248, "Aaa");
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  cpu.unimplemented(0x12005281, "Punpckhbw");
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x1200528c); f_120042f6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = 0x12011d24u32; cpu.push(v); }
  let t = cpu.r32(0x1204121cu32); cpu.push(0x1200529a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203e704u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120051d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120051d8); }
 let mut pc: u32 = 0x120051d8;
 loop { match pc {
 0x120051d8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x12008f04u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0xffffff8fu32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.push(0x2b); }
  {  }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = 0x2bu16; cpu.w16(0x8f121200u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12005224, "Out");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12005228, "Bound");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.cf = false; }
  {  }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x91471200u32.wrapping_add(cpu.ecx)).wrapping_sub((cpu.edx as u8)); cpu.w8(0x91471200u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8) & 0x90u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xae120091u32); cpu.eax = v; }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12005248, "Aaa");
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  cpu.unimplemented(0x12005281, "Punpckhbw");
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x1200528c); f_120042f6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = 0x12011d24u32; cpu.push(v); }
  let t = cpu.r32(0x1204121cu32); cpu.push(0x1200529a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203e704u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200524c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200524c); }
 let mut pc: u32 = 0x1200524c;
 loop { match pc {
 0x1200524c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  cpu.unimplemented(0x12005281, "Punpckhbw");
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x1200528c); f_120042f6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = 0x12011d24u32; cpu.push(v); }
  let t = cpu.r32(0x1204121cu32); cpu.push(0x1200529a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203e704u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12005fda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12005fda); }
 let mut pc: u32 = 0x12005fda;
 loop { match pc {
 0x12005fda => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x120333e3; continue;
 }
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12006003(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12006003); }
 let mut pc: u32 = 0x12006003;
 loop { match pc {
 0x12006003 => {
  cpu.push(0x12006008); f_12005fe1(cpu);
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1200600e; continue; }
  pc = 0x1200600c; continue;
 }
 0x1200600c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x1200600e; continue;
 }
 0x1200600e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120093b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120093b0); }
 let mut pc: u32 = 0x120093b0;
 loop { match pc {
 0x120052d6 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120093b0 => {
  { let v: u8 = cpu.r8(0x5050105u32).wrapping_add((cpu.eax as u8)); cpu.w8(0x5050105u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4050503u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x120052d6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12009970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12009970); }
 let mut pc: u32 = 0x12009970;
 loop { match pc {
 0x12009970 => {
  { let v: u32 = 0xbf120095u32; cpu.edi = v; }
  { let a = cpu.ebp; let b = cpu.eax; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12009978, "Retf");
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120099c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120099c0); }
 let mut pc: u32 = 0x120099c0;
 loop { match pc {
 0x120057d2 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12009960 => {
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  cpu.unimplemented(0x12009968, "Int3");
  cpu.unimplemented(0x12009969, "Int3");
  cpu.unimplemented(0x1200996a, "Int3");
  cpu.unimplemented(0x1200996b, "Int3");
  cpu.unimplemented(0x1200996c, "Int3");
  cpu.unimplemented(0x1200996d, "Int3");
  cpu.unimplemented(0x1200996e, "Int3");
  cpu.unimplemented(0x1200996f, "Int3");
  { let v: u32 = 0xbf120095u32; cpu.edi = v; }
  { let a = cpu.ebp; let b = cpu.eax; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12009978, "Retf");
 }
 0x12009964 => {
  { let v: u32 = cpu.eax.wrapping_add(0xcc900404u32); cpu.eax = v; }
  cpu.unimplemented(0x12009969, "Int3");
  cpu.unimplemented(0x1200996a, "Int3");
  cpu.unimplemented(0x1200996b, "Int3");
  cpu.unimplemented(0x1200996c, "Int3");
  cpu.unimplemented(0x1200996d, "Int3");
  cpu.unimplemented(0x1200996e, "Int3");
  cpu.unimplemented(0x1200996f, "Int3");
  { let v: u32 = 0xbf120095u32; cpu.edi = v; }
  { let a = cpu.ebp; let b = cpu.eax; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12009978, "Retf");
 }
 0x120099c0 => {
  { let a = (cpu.esi as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let a = (cpu.esi as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if !cpu.of { pc = 0x12009960; continue; }
  pc = 0x120099ca; continue;
 }
 0x120099ca => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if !cpu.of { pc = 0x12009964; continue; }
  pc = 0x120099ce; continue;
 }
 0x120099ce => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x200u32; cpu.esi = v; }
  pc = 0x120057d2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200ab68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200ab68); }
 let mut pc: u32 = 0x1200ab68;
 loop { match pc {
 0x1200ab68 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  pc = 0x1200ab8f; continue;
 }
 0x1200ab8f => {
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  pc = 0x1200ab92; continue;
 }
 0x1200ab92 => {
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5050505u32.wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7050505u32); cpu.eax = v; }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.push(0x2b); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xa8a21200u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xa8u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = (cpu.r32(0xa8da1200u32.wrapping_add(cpu.eax)) as i32 as f64) - cpu.st(0); cpu.set_st(0, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.df = true; }
  { cpu.and8((cpu.eax as u8), 0x0u8); }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), (cpu.edx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.cmps(4, 0); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if (cpu.sf == cpu.of) { pc = 0x1200ab8f; continue; }
  pc = 0x1200abe6; continue;
 }
 0x1200abe6 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if cpu.zf { pc = 0x1200ab92; continue; }
  pc = 0x1200abea; continue;
 }
 0x1200abea => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200abec, "Int3");
  cpu.unimplemented(0x1200abed, "Int3");
  cpu.unimplemented(0x1200abee, "Int3");
  cpu.unimplemented(0x1200abef, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0xe9900706u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xaa311200u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac4c, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac50, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac54, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  f_1200ac58(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200abc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200abc8); }
 let mut pc: u32 = 0x1200abc8;
 loop { match pc {
 0x1200ab8f => {
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  pc = 0x1200ab92; continue;
 }
 0x1200ab92 => {
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5050505u32.wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7050505u32); cpu.eax = v; }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.push(0x2b); }
  pc = 0x1200abc8; continue;
 }
 0x1200abc8 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xa8a21200u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xa8u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = (cpu.r32(0xa8da1200u32.wrapping_add(cpu.eax)) as i32 as f64) - cpu.st(0); cpu.set_st(0, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.df = true; }
  { cpu.and8((cpu.eax as u8), 0x0u8); }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), (cpu.edx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.cmps(4, 0); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if (cpu.sf == cpu.of) { pc = 0x1200ab8f; continue; }
  pc = 0x1200abe6; continue;
 }
 0x1200abe6 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if cpu.zf { pc = 0x1200ab92; continue; }
  pc = 0x1200abea; continue;
 }
 0x1200abea => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200abec, "Int3");
  cpu.unimplemented(0x1200abed, "Int3");
  cpu.unimplemented(0x1200abee, "Int3");
  cpu.unimplemented(0x1200abef, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0xe9900706u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xaa311200u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac4c, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac50, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac54, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  f_1200ac58(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200abf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200abf0); }
 let mut pc: u32 = 0x1200abf0;
 loop { match pc {
 0x1200abf0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0xe9900706u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xaa311200u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac4c, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac50, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200ac54, "Out");
  { cpu.stos(1, false); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  f_1200ac58(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200b680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200b680); }
 let mut pc: u32 = 0x1200b680;
 loop { match pc {
 0x1200b680 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_1200b6b3(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200cf78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200cf78); }
 let mut pc: u32 = 0x1200cf78;
 loop { match pc {
 0x1200cf51 => {
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x5u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.edi = v; }
  pc = 0x1200cf78; continue;
 }
 0x1200cf78 => {
  { cpu.sub32(cpu.eax, 0x931200bau32); }
  { let v: u32 = 0xbae91200u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xbb471200u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0xbb691200u32; cpu.ebx = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if cpu.pf() { pc = 0x1200cf51; continue; }
  pc = 0x1200cf96; continue;
 }
 0x1200cf96 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xec831200u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x6au32.wrapping_add(cpu.esi)), (cpu.edx as u8)); cpu.w8(0x6au32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200cfa3, "Hlt");
  cpu.unimplemented(0x1200cfa4, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200cfb4); f_1200f943(cpu);
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1200d080; continue; }
  pc = 0x1200cfc2; continue;
 }
 0x1200cfc2 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if cpu.zf { pc = 0x1200d040; continue; }
  pc = 0x1200cfd0; continue;
 }
 0x1200cfd0 => {
  { cpu.sub32(cpu.eax, 0x32u32); }
  if cpu.zf { pc = 0x1200d050; continue; }
  pc = 0x1200cfd5; continue;
 }
 0x1200cfd5 => {
  { cpu.sub32(cpu.eax, 0x35u32); }
  if cpu.zf { pc = 0x1200d06c; continue; }
  pc = 0x1200cfde; continue;
 }
 0x1200cfde => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1203560cu32; cpu.push(v); }
  cpu.push(0x1200cfec); f_1200fff7(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200d040 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120354f8u32; cpu.push(v); }
  cpu.push(0x1200d04e); f_1200fff7(cpu);
  pc = 0x1200cfde; continue;
 }
 0x1200d050 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200d05a); f_1200d090(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200d067); f_1201006f(cpu);
  pc = 0x1200cfde; continue;
 }
 0x1200d06c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1200d079); f_1200fb01(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200d080 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1200d08b); f_120100e5(cpu);
  pc = 0x1200cfde; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200d928(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200d928); }
 let mut pc: u32 = 0x1200d928;
 loop { match pc {
 0x1200d928 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.unimplemented(0x1200d929, "Aad");
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0xffffffd5u32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let a = cpu.ebp; let b = cpu.edx; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.edx = v; } }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0); cpu.set_st(7, v); cpu.fpop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebp ^ cpu.edx; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x48a110ecu32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf8835600u32; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1200db47; continue; }
  pc = 0x1200d995; continue;
 }
 0x1200d995 => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if cpu.zf { pc = 0x1200db64; continue; }
  pc = 0x1200d99e; continue;
 }
 0x1200d99e => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200d9a8); f_1200db81(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0xfu8); }
  if !cpu.zf { pc = 0x1200daf5; continue; }
  pc = 0x1200d9b6; continue;
 }
 0x1200d9b6 => {
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1200d9e4; continue; }
  pc = 0x1200d9c2; continue;
 }
 0x1200d9c2 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.zf { pc = 0x1200da36; continue; }
  pc = 0x1200d9c7; continue;
 }
 0x1200d9c7 => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if cpu.zf { pc = 0x1200da8e; continue; }
  pc = 0x1200d9d0; continue;
 }
 0x1200d9d0 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1200d9dd); f_1200fb01(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200d9e4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200da36 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200da8e => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x12035628u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  cpu.push(0x1200daa7); f_1200fd31(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 28u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 28u32); cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x40u32; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200daf5 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200db47 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x120354f8u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  cpu.push(0x1200db5d); f_1200f943(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200db64 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x12035630u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  cpu.push(0x1200db7a); f_1200f943(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200d93c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200d93c); }
 let mut pc: u32 = 0x1200d93c;
 loop { match pc {
 0x1200d93c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x48a110ecu32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf8835600u32; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1200db47; continue; }
  pc = 0x1200d995; continue;
 }
 0x1200d995 => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if cpu.zf { pc = 0x1200db64; continue; }
  pc = 0x1200d99e; continue;
 }
 0x1200d99e => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200d9a8); f_1200db81(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0xfu8); }
  if !cpu.zf { pc = 0x1200daf5; continue; }
  pc = 0x1200d9b6; continue;
 }
 0x1200d9b6 => {
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1200d9e4; continue; }
  pc = 0x1200d9c2; continue;
 }
 0x1200d9c2 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.zf { pc = 0x1200da36; continue; }
  pc = 0x1200d9c7; continue;
 }
 0x1200d9c7 => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if cpu.zf { pc = 0x1200da8e; continue; }
  pc = 0x1200d9d0; continue;
 }
 0x1200d9d0 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1200d9dd); f_1200fb01(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200d9e4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200da36 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200da8e => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x12035628u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  cpu.push(0x1200daa7); f_1200fd31(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 28u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 28u32); cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x40u32; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200daf5 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200db47 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x120354f8u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  cpu.push(0x1200db5d); f_1200f943(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200db64 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x12035630u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  cpu.push(0x1200db7a); f_1200f943(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200e258(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200e258); }
 let mut pc: u32 = 0x1200e258;
 loop { match pc {
 0x1200e258 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), cpu.r8(0xc31200dfu32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), (cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.st(0) + (cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as f64); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), ((cpu.eax >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e275, "Int3");
  cpu.unimplemented(0x1200e276, "Int3");
  cpu.unimplemented(0x1200e277, "Int3");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.eax.wrapping_add(0x1200dffcu32); cpu.eax = v; }
  { let v: u8 = 0xe0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xe0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xe0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xe0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.add32(cpu.eax, 0xf200498du32); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2cb; continue; }
  pc = 0x1200e2cb; continue;
 }
 0x1200e2cb => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2cf; continue; }
  pc = 0x1200e2cf; continue;
 }
 0x1200e2cf => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2d3; continue; }
  pc = 0x1200e2d3; continue;
 }
 0x1200e2d3 => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2d7; continue; }
  pc = 0x1200e2d7; continue;
 }
 0x1200e2d7 => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2db; continue; }
  pc = 0x1200e2db; continue;
 }
 0x1200e2db => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2df; continue; }
  pc = 0x1200e2df; continue;
 }
 0x1200e2df => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2e3; continue; }
  pc = 0x1200e2e3; continue;
 }
 0x1200e2e3 => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2e7; continue; }
  pc = 0x1200e2e7; continue;
 }
 0x1200e2e7 => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2eb; continue; }
  pc = 0x1200e2eb; continue;
 }
 0x1200e2eb => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x44c738ecu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xfffffff0u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffefu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffdfu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffbfu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1200e343; continue; }
  pc = 0x1200e314; continue;
 }
 0x1200e314 => {
  { cpu.sub32(cpu.esi, 0xffffu32); }
  if !cpu.zf { pc = 0x1200e330; continue; }
  pc = 0x1200e31c; continue;
 }
 0x1200e31c => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1200e329); f_1200fb01(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e330 => {
  { cpu.sub32(cpu.esi, 0xfffeu32); }
  if !cpu.zf { pc = 0x1200e369; continue; }
  pc = 0x1200e338; continue;
 }
 0x1200e338 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1200e343); f_1201025b(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e343 => {
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x57u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e420; continue; }
  pc = 0x1200e35a; continue;
 }
 0x1200e35a => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1200e698u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1200e680u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1200e369 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if !cpu.zf { pc = 0x1200e343; continue; }
  pc = 0x1200e371; continue;
 }
 0x1200e371 => {
  { cpu.and16(cpu.r16(0x1203b058u32), 0x402u16); }
  if !cpu.zf { pc = 0x1200e395; continue; }
  pc = 0x1200e37c; continue;
 }
 0x1200e37c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e386); f_1200cf9c(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e393); f_12010203(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e395 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e39f); f_1200cf9c(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e3ac); f_12010168(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e3ae => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x12035700u32; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e3cc); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e3d3); f_1200fdeb(cpu);
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1200e3e3); f_1200fecc(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e3ea => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e3f3; continue;
 }
 0x1200e3f3 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356f8u32; cpu.push(v); }
  cpu.push(0x1200e40b); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e412); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e416 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e420); f_12010203(cpu);
  pc = 0x1200e420; continue;
 }
 0x1200e420 => {
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x1200e455; continue; }
  pc = 0x1200e428; continue;
 }
 0x1200e428 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356d4u32; cpu.push(v); }
  cpu.push(0x1200e440); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e447); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e455); f_12010203(cpu);
  pc = 0x1200e455; continue;
 }
 0x1200e455 => {
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x1200e48a; continue; }
  pc = 0x1200e45d; continue;
 }
 0x1200e45d => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356ccu32; cpu.push(v); }
  cpu.push(0x1200e475); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e47c); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e48a); f_12010203(cpu);
  pc = 0x1200e48a; continue;
 }
 0x1200e48a => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e494); f_1200d090(cpu);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e4a5); f_1201006f(cpu);
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e4f9 => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e506; continue;
 }
 0x1200e506 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356f0u32; cpu.push(v); }
  cpu.push(0x1200e51e); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e525); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e52e => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e53b; continue;
 }
 0x1200e53b => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356e8u32; cpu.push(v); }
  cpu.push(0x1200e553); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e55a); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e563 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356e0u32; cpu.push(v); }
  cpu.push(0x1200e57b); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e582); f_1200fdeb(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xfffffff0u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffefu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffdfu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffbfu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1200e677; continue; }
  pc = 0x1200e5ae; continue;
 }
 0x1200e5ae => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e5c0; continue; }
  pc = 0x1200e5b9; continue;
 }
 0x1200e5b9 => {
  pc = cpu.r32(0x1200e700u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1200e5c0 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x1200e5c2; continue;
 }
 0x1200e5c2 => {
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e5cb); f_1200fb01(cpu);
  pc = 0x1200e65d; continue;
 }
 0x1200e5d0 => {
  { let v = 0x12035720u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e5d5 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e5de); f_120101cd(cpu);
  pc = 0x1200e5de; continue;
 }
 0x1200e5de => {
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x31u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e63d; continue; }
  pc = 0x1200e5f4; continue;
 }
 0x1200e5f4 => {
  pc = cpu.r32(0x1200e720u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1200e5fb => {
  { let v = 0x12035718u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e602 => {
  { let v = 0x12035710u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e609 => {
  { let v = 0x120356c4u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e610 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356b8u32; cpu.push(v); }
  cpu.push(0x1200e628); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e62f); f_1200fdeb(cpu);
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e63d); f_12010203(cpu);
  pc = 0x1200e63d; continue;
 }
 0x1200e63d => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e647); f_1200d090(cpu);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e65d); f_1200fdeb(cpu);
  pc = 0x1200e65d; continue;
 }
 0x1200e65d => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1200e670); f_1200fdeb(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e677 => {
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x1200e5c2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200e278(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200e278); }
 let mut pc: u32 = 0x1200e278;
 loop { match pc {
 0x1200e278 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.eax.wrapping_add(0x1200dffcu32); cpu.eax = v; }
  { let v: u8 = 0xe0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xe0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xe0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xe0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.add32(cpu.eax, 0xf200498du32); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2cb; continue; }
  pc = 0x1200e2cb; continue;
 }
 0x1200e2cb => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2cf; continue; }
  pc = 0x1200e2cf; continue;
 }
 0x1200e2cf => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2d3; continue; }
  pc = 0x1200e2d3; continue;
 }
 0x1200e2d3 => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2d7; continue; }
  pc = 0x1200e2d7; continue;
 }
 0x1200e2d7 => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2db; continue; }
  pc = 0x1200e2db; continue;
 }
 0x1200e2db => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2df; continue; }
  pc = 0x1200e2df; continue;
 }
 0x1200e2df => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2e3; continue; }
  pc = 0x1200e2e3; continue;
 }
 0x1200e2e3 => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2e7; continue; }
  pc = 0x1200e2e7; continue;
 }
 0x1200e2e7 => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2eb; continue; }
  pc = 0x1200e2eb; continue;
 }
 0x1200e2eb => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x44c738ecu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xfffffff0u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffefu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffdfu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffbfu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1200e343; continue; }
  pc = 0x1200e314; continue;
 }
 0x1200e314 => {
  { cpu.sub32(cpu.esi, 0xffffu32); }
  if !cpu.zf { pc = 0x1200e330; continue; }
  pc = 0x1200e31c; continue;
 }
 0x1200e31c => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1200e329); f_1200fb01(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e330 => {
  { cpu.sub32(cpu.esi, 0xfffeu32); }
  if !cpu.zf { pc = 0x1200e369; continue; }
  pc = 0x1200e338; continue;
 }
 0x1200e338 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1200e343); f_1201025b(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e343 => {
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x57u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e420; continue; }
  pc = 0x1200e35a; continue;
 }
 0x1200e35a => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1200e698u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1200e680u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1200e369 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if !cpu.zf { pc = 0x1200e343; continue; }
  pc = 0x1200e371; continue;
 }
 0x1200e371 => {
  { cpu.and16(cpu.r16(0x1203b058u32), 0x402u16); }
  if !cpu.zf { pc = 0x1200e395; continue; }
  pc = 0x1200e37c; continue;
 }
 0x1200e37c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e386); f_1200cf9c(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e393); f_12010203(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e395 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e39f); f_1200cf9c(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e3ac); f_12010168(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e3ae => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x12035700u32; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e3cc); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e3d3); f_1200fdeb(cpu);
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1200e3e3); f_1200fecc(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e3ea => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e3f3; continue;
 }
 0x1200e3f3 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356f8u32; cpu.push(v); }
  cpu.push(0x1200e40b); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e412); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e416 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e420); f_12010203(cpu);
  pc = 0x1200e420; continue;
 }
 0x1200e420 => {
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x1200e455; continue; }
  pc = 0x1200e428; continue;
 }
 0x1200e428 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356d4u32; cpu.push(v); }
  cpu.push(0x1200e440); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e447); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e455); f_12010203(cpu);
  pc = 0x1200e455; continue;
 }
 0x1200e455 => {
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x1200e48a; continue; }
  pc = 0x1200e45d; continue;
 }
 0x1200e45d => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356ccu32; cpu.push(v); }
  cpu.push(0x1200e475); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e47c); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e48a); f_12010203(cpu);
  pc = 0x1200e48a; continue;
 }
 0x1200e48a => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e494); f_1200d090(cpu);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e4a5); f_1201006f(cpu);
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e4f9 => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e506; continue;
 }
 0x1200e506 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356f0u32; cpu.push(v); }
  cpu.push(0x1200e51e); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e525); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e52e => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e53b; continue;
 }
 0x1200e53b => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356e8u32; cpu.push(v); }
  cpu.push(0x1200e553); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e55a); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e563 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356e0u32; cpu.push(v); }
  cpu.push(0x1200e57b); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e582); f_1200fdeb(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xfffffff0u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffefu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffdfu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffbfu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1200e677; continue; }
  pc = 0x1200e5ae; continue;
 }
 0x1200e5ae => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e5c0; continue; }
  pc = 0x1200e5b9; continue;
 }
 0x1200e5b9 => {
  pc = cpu.r32(0x1200e700u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1200e5c0 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x1200e5c2; continue;
 }
 0x1200e5c2 => {
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e5cb); f_1200fb01(cpu);
  pc = 0x1200e65d; continue;
 }
 0x1200e5d0 => {
  { let v = 0x12035720u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e5d5 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e5de); f_120101cd(cpu);
  pc = 0x1200e5de; continue;
 }
 0x1200e5de => {
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x31u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e63d; continue; }
  pc = 0x1200e5f4; continue;
 }
 0x1200e5f4 => {
  pc = cpu.r32(0x1200e720u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1200e5fb => {
  { let v = 0x12035718u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e602 => {
  { let v = 0x12035710u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e609 => {
  { let v = 0x120356c4u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e610 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356b8u32; cpu.push(v); }
  cpu.push(0x1200e628); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e62f); f_1200fdeb(cpu);
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e63d); f_12010203(cpu);
  pc = 0x1200e63d; continue;
 }
 0x1200e63d => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e647); f_1200d090(cpu);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e65d); f_1200fdeb(cpu);
  pc = 0x1200e65d; continue;
 }
 0x1200e65d => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1200e670); f_1200fdeb(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e677 => {
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x1200e5c2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200e2a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200e2a8); }
 let mut pc: u32 = 0x1200e2a8;
 loop { match pc {
 0x1200e2a8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.add32(cpu.eax, 0xf200498du32); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2cb; continue; }
  pc = 0x1200e2cb; continue;
 }
 0x1200e2cb => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2cf; continue; }
  pc = 0x1200e2cf; continue;
 }
 0x1200e2cf => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2d3; continue; }
  pc = 0x1200e2d3; continue;
 }
 0x1200e2d3 => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2d7; continue; }
  pc = 0x1200e2d7; continue;
 }
 0x1200e2d7 => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2db; continue; }
  pc = 0x1200e2db; continue;
 }
 0x1200e2db => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2df; continue; }
  pc = 0x1200e2df; continue;
 }
 0x1200e2df => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2e3; continue; }
  pc = 0x1200e2e3; continue;
 }
 0x1200e2e3 => {
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2e7; continue; }
  pc = 0x1200e2e7; continue;
 }
 0x1200e2e7 => {
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1200e2eb; continue; }
  pc = 0x1200e2eb; continue;
 }
 0x1200e2eb => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x44c738ecu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xfffffff0u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffefu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffdfu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)) & 0xffffffbfu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1200e343; continue; }
  pc = 0x1200e314; continue;
 }
 0x1200e314 => {
  { cpu.sub32(cpu.esi, 0xffffu32); }
  if !cpu.zf { pc = 0x1200e330; continue; }
  pc = 0x1200e31c; continue;
 }
 0x1200e31c => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  cpu.push(0x1200e329); f_1200fb01(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e330 => {
  { cpu.sub32(cpu.esi, 0xfffeu32); }
  if !cpu.zf { pc = 0x1200e369; continue; }
  pc = 0x1200e338; continue;
 }
 0x1200e338 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1200e343); f_1201025b(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e343 => {
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x57u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e420; continue; }
  pc = 0x1200e35a; continue;
 }
 0x1200e35a => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1200e698u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1200e680u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1200e369 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if !cpu.zf { pc = 0x1200e343; continue; }
  pc = 0x1200e371; continue;
 }
 0x1200e371 => {
  { cpu.and16(cpu.r16(0x1203b058u32), 0x402u16); }
  if !cpu.zf { pc = 0x1200e395; continue; }
  pc = 0x1200e37c; continue;
 }
 0x1200e37c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e386); f_1200cf9c(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e393); f_12010203(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e395 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e39f); f_1200cf9c(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e3ac); f_12010168(cpu);
  pc = 0x1200e343; continue;
 }
 0x1200e3ae => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x12035700u32; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e3cc); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e3d3); f_1200fdeb(cpu);
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1200e3e3); f_1200fecc(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e3ea => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e3f3; continue;
 }
 0x1200e3f3 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356f8u32; cpu.push(v); }
  cpu.push(0x1200e40b); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e412); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e416 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e420); f_12010203(cpu);
  pc = 0x1200e420; continue;
 }
 0x1200e420 => {
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x1200e455; continue; }
  pc = 0x1200e428; continue;
 }
 0x1200e428 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356d4u32; cpu.push(v); }
  cpu.push(0x1200e440); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e447); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e455); f_12010203(cpu);
  pc = 0x1200e455; continue;
 }
 0x1200e455 => {
  { cpu.and32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x1200e48a; continue; }
  pc = 0x1200e45d; continue;
 }
 0x1200e45d => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356ccu32; cpu.push(v); }
  cpu.push(0x1200e475); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e47c); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e48a); f_12010203(cpu);
  pc = 0x1200e48a; continue;
 }
 0x1200e48a => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e494); f_1200d090(cpu);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e4a5); f_1201006f(cpu);
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e4f9 => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e506; continue;
 }
 0x1200e506 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356f0u32; cpu.push(v); }
  cpu.push(0x1200e51e); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e525); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e52e => {
  { cpu.and8(cpu.r8(0x1203b059u32), 0x10u8); }
  if !cpu.zf { pc = 0x1200e420; continue; }
  pc = 0x1200e53b; continue;
 }
 0x1200e53b => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356e8u32; cpu.push(v); }
  cpu.push(0x1200e553); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e55a); f_1200fdeb(cpu);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1200e416; continue;
 }
 0x1200e563 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356e0u32; cpu.push(v); }
  cpu.push(0x1200e57b); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e582); f_1200fdeb(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xfffffff0u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffefu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffdfu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)) & 0xffffffbfu32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1200e677; continue; }
  pc = 0x1200e5ae; continue;
 }
 0x1200e5ae => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e5c0; continue; }
  pc = 0x1200e5b9; continue;
 }
 0x1200e5b9 => {
  pc = cpu.r32(0x1200e700u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1200e5c0 => {
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x1200e5c2; continue;
 }
 0x1200e5c2 => {
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e5cb); f_1200fb01(cpu);
  pc = 0x1200e65d; continue;
 }
 0x1200e5d0 => {
  { let v = 0x12035720u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e5d5 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e5de); f_120101cd(cpu);
  pc = 0x1200e5de; continue;
 }
 0x1200e5de => {
  { let v: u32 = cpu.r32(0x1203b048u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203b048u32)); cpu.w32(0x1203b048u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x31u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1200e63d; continue; }
  pc = 0x1200e5f4; continue;
 }
 0x1200e5f4 => {
  pc = cpu.r32(0x1200e720u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1200e5fb => {
  { let v = 0x12035718u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e602 => {
  { let v = 0x12035710u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e609 => {
  { let v = 0x120356c4u32; cpu.push(v); }
  pc = 0x1200e5d5; continue;
 }
 0x1200e610 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x120356b8u32; cpu.push(v); }
  cpu.push(0x1200e628); f_1200f943(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e62f); f_1200fdeb(cpu);
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e63d); f_12010203(cpu);
  pc = 0x1200e63d; continue;
 }
 0x1200e63d => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1200e647); f_1200d090(cpu);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x1200e65d); f_1200fdeb(cpu);
  pc = 0x1200e65d; continue;
 }
 0x1200e65d => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1200e670); f_1200fdeb(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1200e677 => {
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x1200e5c2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200e698(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200e698); }
 let mut pc: u32 = 0x1200e698;
 loop { match pc {
 0x1200e698 => {
  { let v: u8 = cpu.r8(0x5050505u32).wrapping_add((cpu.eax as u8)); cpu.w8(0x5050505u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2010505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(8)))); cpu.eax = v; }
  cpu.unimplemented(0x1200e6f1, "Int3");
  cpu.unimplemented(0x1200e6f2, "Int3");
  cpu.unimplemented(0x1200e6f3, "Int3");
  cpu.unimplemented(0x1200e6f4, "Int3");
  cpu.unimplemented(0x1200e6f5, "Int3");
  cpu.unimplemented(0x1200e6f6, "Int3");
  cpu.unimplemented(0x1200e6f7, "Int3");
  cpu.unimplemented(0x1200e6f8, "Int3");
  cpu.unimplemented(0x1200e6f9, "Int3");
  cpu.unimplemented(0x1200e6fa, "Int3");
  cpu.unimplemented(0x1200e6fb, "Int3");
  cpu.unimplemented(0x1200e6fc, "Int3");
  cpu.unimplemented(0x1200e6fd, "Int3");
  cpu.unimplemented(0x1200e6fe, "Int3");
  cpu.unimplemented(0x1200e6ff, "Int3");
  { let v: u8 = cpu.shl8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.shl8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200e708, "Sti");
  cpu.unimplemented(0x1200e709, "In");
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x1200e70d, "In");
  { let v: u8 = cpu.adc8((cpu.ebx as u8), ((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e711, "In");
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e715, "Out");
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e719, "Out");
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e71d, "Out");
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e721, "Out");
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x101200e6u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x1200e729, "Out");
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x101200e6u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x1200e731, "Out");
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x101200e6u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x1200e739, "Out");
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x48a138ecu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0xffffff8au32.wrapping_add(cpu.ebx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff84u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); cpu.w8(0xffffff84u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x75u32.wrapping_add(cpu.edi)), 37u32); cpu.w8(0x75u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1200e762); f_1200fb01(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e769); f_1200fdeb(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1200e700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1200e700); }
 let mut pc: u32 = 0x1200e700;
 loop { match pc {
 0x1200e700 => {
  { let v: u8 = cpu.shl8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.shl8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1200e708, "Sti");
  cpu.unimplemented(0x1200e709, "In");
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x1200e70d, "In");
  { let v: u8 = cpu.adc8((cpu.ebx as u8), ((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e711, "In");
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e715, "Out");
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e719, "Out");
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e71d, "Out");
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1200e721, "Out");
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x101200e6u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x1200e729, "Out");
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x101200e6u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x1200e731, "Out");
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x101200e6u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x1200e739, "Out");
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x48a138ecu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0xffffff8au32.wrapping_add(cpu.ebx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff84u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); cpu.w8(0xffffff84u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x75u32.wrapping_add(cpu.edi)), 37u32); cpu.w8(0x75u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1200e762); f_1200fb01(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1200e769); f_1200fdeb(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120112ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120112ef); }
 let mut pc: u32 = 0x120112ef;
 loop { match pc {
 0x120112ef => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0xfffffffdu32); cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011491(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011491); }
 let mut pc: u32 = 0x12011491;
 loop { match pc {
 0x12011491 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x120114b4; continue; }
  pc = 0x12011497; continue;
 }
 0x12011497 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  pc = 0x120114ac; continue;
 }
 0x120114ac => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120114b4 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  pc = 0x120114ac; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014176(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014176); }
 let mut pc: u32 = 0x12014176;
 loop { match pc {
 0x12014176 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
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
pub fn f_12014980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014980); }
 let mut pc: u32 = 0x12014980;
 loop { match pc {
 0x12014980 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x57u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1201499e); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x120149cd; continue; }
  pc = 0x120149a8; continue;
 }
 0x120149a8 => {
  { let v: u32 = 0x1203c1bcu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203c1bcu32); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  pc = 0x120149b8; continue;
 }
 0x120149b8 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x120149df; continue; }
  pc = 0x120149bc; continue;
 }
 0x120149bc => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x120149c3); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x120149cd => {
  { let v: u32 = 0x1203c1c0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203c1c0u32); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.edi = v; }
  pc = 0x120149b8; continue;
 }
 0x120149df => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x12014a04; continue; }
  pc = 0x120149e4; continue;
 }
 0x120149e4 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120149f1); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x120149f7); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12014a04 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12014a0b); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120184e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120184e4); }
 let mut pc: u32 = 0x120184e4;
 loop { match pc {
 0x120184e4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x60a0a0au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  {  }
  { let a = cpu.esp; let b = cpu.eax; { let v: u32 = b; cpu.esp = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.scas(4, 0); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x12018548, "Retf");
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018538(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018538); }
 let mut pc: u32 = 0x12018538;
 loop { match pc {
 0x12018538 => {
  { let a = cpu.esp; let b = cpu.eax; { let v: u32 = b; cpu.esp = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.scas(4, 0); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x12018548, "Retf");
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018548(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018548); }
 let mut pc: u32 = 0x12018548;
 loop { match pc {
 0x12018548 => {
  cpu.unimplemented(0x12018548, "Retf");
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019e10); }
 let mut pc: u32 = 0x12019e10;
 loop { match pc {
 0x12019e10 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x8104244cu32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12019e26, "In");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x53002444u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x53002444u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1204119cu32); cpu.push(0x12019e3a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019ea9; continue; }
  pc = 0x12019e3e; continue;
 }
 0x12019e3e => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12019e44); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12019e5b; continue; }
  pc = 0x12019e4c; continue;
 }
 0x12019e4c => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x12019f48u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12019f38u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x12019e5b => {
  cpu.push(0x12019e60); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019e75 => {
  cpu.push(0x12019e7a); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019e8f => {
  cpu.push(0x12019e94); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019ea9 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x12019f27; continue; }
  pc = 0x12019eb3; continue;
 }
 0x12019eb3 => {
  { let v: u32 = cpu.r32(0x154u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12019ec0; continue;
 }
 0x12019ec0 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019eca); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019eda); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019eea); f_12019f7c(cpu);
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019f27 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x154u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12019ec0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a158(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a158); }
 let mut pc: u32 = 0x1201a158;
 loop { match pc {
 0x1201a158 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x8104244cu32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1201a16e, "In");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x53002444u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x53002444u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1204119cu32); cpu.push(0x1201a182); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201a1f1; continue; }
  pc = 0x1201a186; continue;
 }
 0x1201a186 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a18c); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201a1a3; continue; }
  pc = 0x1201a194; continue;
 }
 0x1201a194 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1201a2b0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1201a2a0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1201a1a3 => {
  cpu.push(0x1201a1a8); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a1bd => {
  cpu.push(0x1201a1c2); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a1d7 => {
  cpu.push(0x1201a1dc); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a1f1 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x1201a28b; continue; }
  pc = 0x1201a1ff; continue;
 }
 0x1201a1ff => {
  { let v: u32 = cpu.r32(0x154u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201a20c; continue;
 }
 0x1201a20c => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a216); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a226); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a236); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a24c); f_12018734(cpu);
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x2cu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a28b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x154u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201a20c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a450); }
 let mut pc: u32 = 0x1201a450;
 loop { match pc {
 0x1201a450 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x8104244cu32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1201a466, "In");
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x56002444u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x56002444u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041194u32); cpu.push(0x1201a478); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201a4e1; continue; }
  pc = 0x1201a47c; continue;
 }
 0x1201a47c => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a482); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201a499; continue; }
  pc = 0x1201a48a; continue;
 }
 0x1201a48a => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1201a570u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1201a560u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1201a499 => {
  cpu.push(0x1201a49e); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a4b1 => {
  cpu.push(0x1201a4b6); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a4c9 => {
  cpu.push(0x1201a4ce); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a4e1 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x1201a54a; continue; }
  pc = 0x1201a4eb; continue;
 }
 0x1201a4eb => {
  { let v: u32 = cpu.r32(0x25cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a4f8; continue;
 }
 0x1201a4f8 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a502); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a512); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a522); f_12019f7c(cpu);
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201a53d); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a54a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x25cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a4f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a570); }
 let mut pc: u32 = 0x1201a570;
 loop { match pc {
 0x1201a570 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x8104244cu32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1201a586, "In");
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x56002444u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x56002444u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041190u32); cpu.push(0x1201a599); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x1201a608; continue; }
  pc = 0x1201a5a0; continue;
 }
 0x1201a5a0 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a5a6); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201a5bd; continue; }
  pc = 0x1201a5ae; continue;
 }
 0x1201a5ae => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1201a6b0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1201a6a0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1201a5bd => {
  cpu.push(0x1201a5c2); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a5d6 => {
  cpu.push(0x1201a5db); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a5ef => {
  cpu.push(0x1201a5f4); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a608 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x1201a68a; continue; }
  pc = 0x1201a612; continue;
 }
 0x1201a612 => {
  { let v: u32 = cpu.r32(0x260u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a61f; continue;
 }
 0x1201a61f => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a629); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a639); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a649); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a65f); f_12018734(cpu);
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x28u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x18u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201a67c); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a68a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x260u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a61f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a7f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a7f0); }
 let mut pc: u32 = 0x1201a7f0;
 loop { match pc {
 0x1201a7f0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x50042444u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x120410ccu32); cpu.push(0x1201a80c); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201a826; continue; }
  pc = 0x1201a811; continue;
 }
 0x1201a811 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a817); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a81d); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a826 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1201a84d; continue; }
  pc = 0x1201a82a; continue;
 }
 0x1201a82a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x2u8); }
  if cpu.zf { pc = 0x1201a84d; continue; }
  pc = 0x1201a831; continue;
 }
 0x1201a831 => {
  cpu.push(0x1201a836); f_120133b4(cpu);
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201a841); f_120133bd(cpu);
  { let v: u32 = 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a84d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201e9b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201e9b0); }
 let mut pc: u32 = 0x1201e9b0;
 loop { match pc {
 0x1201e93d => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x1201e948); cpu.call(t);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e999 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x18244489u32); cpu.eax = v; }
  pc = 0x1201e93d; continue;
 }
 0x1201e99d => {
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x18u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1201e93d; continue;
 }
 0x1201e9a1 => {
  {  }
  cpu.unimplemented(0x1201e9a2, "Int3");
  cpu.unimplemented(0x1201e9a3, "Int3");
  cpu.unimplemented(0x1201e9a4, "Int3");
  pc = 0x1201e9a5; continue;
 }
 0x1201e9a5 => {
  cpu.unimplemented(0x1201e9a5, "Int3");
  cpu.unimplemented(0x1201e9a6, "Int3");
  cpu.unimplemented(0x1201e9a7, "Int3");
  cpu.unimplemented(0x1201e9a8, "Int3");
  pc = 0x1201e9a9; continue;
 }
 0x1201e9a9 => {
  cpu.unimplemented(0x1201e9a9, "Int3");
  cpu.unimplemented(0x1201e9aa, "Int3");
  cpu.unimplemented(0x1201e9ab, "Int3");
  cpu.unimplemented(0x1201e9ac, "Int3");
  cpu.unimplemented(0x1201e9ad, "Int3");
  cpu.unimplemented(0x1201e9ae, "Int3");
  cpu.unimplemented(0x1201e9af, "Int3");
  pc = 0x1201e9b0; continue;
 }
 0x1201e9b0 => {
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e999; continue; }
  pc = 0x1201e9b2; continue;
 }
 0x1201e9b2 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e99d; continue; }
  pc = 0x1201e9b6; continue;
 }
 0x1201e9b6 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e9a1; continue; }
  pc = 0x1201e9ba; continue;
 }
 0x1201e9ba => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e9a5; continue; }
  pc = 0x1201e9be; continue;
 }
 0x1201e9be => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  if cpu.cf { pc = 0x1201e9a9; continue; }
  pc = 0x1201e9c2; continue;
 }
 0x1201e9c2 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  f_1201e9c4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020b80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020b80); }
 let mut pc: u32 = 0x12020b80;
 loop { match pc {
 0x12020b80 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_12020bb1(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120257b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120257b8); }
 let mut pc: u32 = 0x120257b8;
 loop { match pc {
 0x120257b8 => {
  cpu.unimplemented(0x120257b8, "Out");
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257c0, "Sti");
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x2u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0xad120250u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5050105u32).wrapping_add((cpu.eax as u8)); cpu.w8(0x5050105u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4050503u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(1, false); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257e8, "Daa");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257ec, "Das");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257f0, "Aaa");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x12025160u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = 0x51u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x51u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025834, "Outsd");
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rcl32(cpu.r32(0x2u32.wrapping_add(cpu.ebp)), 1u32); cpu.w32(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025840, "Outsb");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x59120251u32; cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x120258a7; continue; }
  pc = 0x12025852; continue;
 }
 0x12025852 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025858, "Int3");
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x120258b0; continue; }
  pc = 0x1202585e; continue;
 }
 0x1202585e => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.r32(0x2u32.wrapping_add(cpu.esi)), cpu.edx); cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.call(0x120258bc); return; }
  pc = 0x1202586a; continue;
 }
 0x1202586a => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  if !cpu.pf() { cpu.call(0x62067ceb); return; }
  pc = 0x120258a7; continue;
 }
 0x120258a7 => {
  cpu.push(0x120258ac); f_12020e15(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x120258bf; continue; }
  pc = 0x120258b8; continue;
 }
 0x120258b0 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.dec32(cpu.r32(0x75082444u32.wrapping_add(cpu.ebx))); cpu.w32(0x75082444u32.wrapping_add(cpu.ebx), v); }
  { cpu.pop(); }
  pc = 0x120258b8; continue;
 }
 0x120258b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120258bf => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120257d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120257d0); }
 let mut pc: u32 = 0x120257d0;
 loop { match pc {
 0x120257d0 => {
  { let v: u8 = cpu.r8(0x5050105u32).wrapping_add((cpu.eax as u8)); cpu.w8(0x5050105u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4050503u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(1, false); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257e8, "Daa");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257ec, "Das");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257f0, "Aaa");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x12025160u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = 0x51u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x51u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025834, "Outsd");
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rcl32(cpu.r32(0x2u32.wrapping_add(cpu.ebp)), 1u32); cpu.w32(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025840, "Outsb");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x59120251u32; cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x120258a7; continue; }
  pc = 0x12025852; continue;
 }
 0x12025852 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025858, "Int3");
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x120258b0; continue; }
  pc = 0x1202585e; continue;
 }
 0x1202585e => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.r32(0x2u32.wrapping_add(cpu.esi)), cpu.edx); cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.call(0x120258bc); return; }
  pc = 0x1202586a; continue;
 }
 0x1202586a => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  if !cpu.pf() { cpu.call(0x62067ceb); return; }
  pc = 0x120258a7; continue;
 }
 0x120258a7 => {
  cpu.push(0x120258ac); f_12020e15(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x120258bf; continue; }
  pc = 0x120258b8; continue;
 }
 0x120258b0 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.dec32(cpu.r32(0x75082444u32.wrapping_add(cpu.ebx))); cpu.w32(0x75082444u32.wrapping_add(cpu.ebx), v); }
  { cpu.pop(); }
  pc = 0x120258b8; continue;
 }
 0x120258b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120258bf => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120257e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120257e4); }
 let mut pc: u32 = 0x120257e4;
 loop { match pc {
 0x120257e4 => {
  { cpu.stos(1, false); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257e8, "Daa");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257ec, "Das");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120257f0, "Aaa");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x12025160u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = 0x51u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x51u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025834, "Outsd");
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rcl32(cpu.r32(0x2u32.wrapping_add(cpu.ebp)), 1u32); cpu.w32(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025840, "Outsb");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x59120251u32; cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x120258a7; continue; }
  pc = 0x12025852; continue;
 }
 0x12025852 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025858, "Int3");
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x120258b0; continue; }
  pc = 0x1202585e; continue;
 }
 0x1202585e => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.r32(0x2u32.wrapping_add(cpu.esi)), cpu.edx); cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.call(0x120258bc); return; }
  pc = 0x1202586a; continue;
 }
 0x1202586a => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  if !cpu.pf() { cpu.call(0x62067ceb); return; }
  pc = 0x120258a7; continue;
 }
 0x120258a7 => {
  cpu.push(0x120258ac); f_12020e15(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x120258bf; continue; }
  pc = 0x120258b8; continue;
 }
 0x120258b0 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.dec32(cpu.r32(0x75082444u32.wrapping_add(cpu.ebx))); cpu.w32(0x75082444u32.wrapping_add(cpu.ebx), v); }
  { cpu.pop(); }
  pc = 0x120258b8; continue;
 }
 0x120258b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120258bf => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120257f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120257f8); }
 let mut pc: u32 = 0x120257f8;
 loop { match pc {
 0x120257f8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x12025160u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = 0x51u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x51u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025834, "Outsd");
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rcl32(cpu.r32(0x2u32.wrapping_add(cpu.ebp)), 1u32); cpu.w32(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025840, "Outsb");
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x59120251u32; cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x120258a7; continue; }
  pc = 0x12025852; continue;
 }
 0x12025852 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12025858, "Int3");
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x120258b0; continue; }
  pc = 0x1202585e; continue;
 }
 0x1202585e => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.r32(0x2u32.wrapping_add(cpu.esi)), cpu.edx); cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.call(0x120258bc); return; }
  pc = 0x1202586a; continue;
 }
 0x1202586a => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  if !cpu.pf() { cpu.call(0x62067ceb); return; }
  pc = 0x120258a7; continue;
 }
 0x120258a7 => {
  cpu.push(0x120258ac); f_12020e15(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x120258bf; continue; }
  pc = 0x120258b8; continue;
 }
 0x120258b0 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.dec32(cpu.r32(0x75082444u32.wrapping_add(cpu.ebx))); cpu.w32(0x75082444u32.wrapping_add(cpu.ebx), v); }
  { cpu.pop(); }
  pc = 0x120258b8; continue;
 }
 0x120258b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120258bf => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202586c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202586c); }
 let mut pc: u32 = 0x1202586c;
 loop { match pc {
 0x1202586c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  if !cpu.pf() { cpu.call(0x62067ceb); return; }
  pc = 0x120258a7; continue;
 }
 0x120258a7 => {
  cpu.push(0x120258ac); f_12020e15(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x120258bf; continue; }
  pc = 0x120258b8; continue;
 }
 0x120258b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120258bf => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12026554(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12026554); }
 let mut pc: u32 = 0x12026554;
 loop { match pc {
 0x12026554 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x6080808u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.pop(); }
  { let v: u32 = cpu.edi; cpu.edi = v; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.esp = (cpu.esp & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, cpu.r32(0x2u32.wrapping_add(cpu.eax))); cpu.esp = v; }
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120265b0, "Les");
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, cpu.r32(0x2u32.wrapping_add(cpu.eax))); cpu.esp = v; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.pushad(); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, cpu.r32(0x2u32.wrapping_add(cpu.eax))); cpu.esp = v; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), ((cpu.eax >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x2u32.wrapping_add(cpu.eax)), 0x12u32); cpu.esp = v; }
  { let v: u32 = cpu.sbb32(cpu.ebx, cpu.r32(0x12u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x70b0b06u32.wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  f_120265ed(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120265a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120265a4); }
 let mut pc: u32 = 0x120265a4;
 loop { match pc {
 0x120265a4 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.esp = (cpu.esp & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, cpu.r32(0x2u32.wrapping_add(cpu.eax))); cpu.esp = v; }
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), (cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x120265b0, "Les");
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, cpu.r32(0x2u32.wrapping_add(cpu.eax))); cpu.esp = v; }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.pushad(); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, cpu.r32(0x2u32.wrapping_add(cpu.eax))); cpu.esp = v; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), ((cpu.eax >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x2u32.wrapping_add(cpu.eax)), 0x12u32); cpu.esp = v; }
  { let v: u32 = cpu.sbb32(cpu.ebx, cpu.r32(0x12u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x70b0b06u32.wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  f_120265ed(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120265d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120265d4); }
 let mut pc: u32 = 0x120265d4;
 loop { match pc {
 0x120265d4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x70b0b06u32.wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  f_120265ed(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12026798(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12026798); }
 let mut pc: u32 = 0x12026798;
 loop { match pc {
 0x12026798 => {
  { let v: u8 = cpu.r8(0x5050505u32).wrapping_add((cpu.eax as u8)); cpu.w8(0x5050505u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1050505u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x5050505u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x5050505u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5050505u32)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x68u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  cpu.push(0x120267ec); f_120281cf(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202a4e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202a4e8); }
 let mut pc: u32 = 0x1202a4e8;
 loop { match pc {
 0x1202a490 => {
  { let v: u8 = cpu.add8(cpu.r8(0x10e910c4u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x10e910c4u32.wrapping_add(cpu.ebx), v); }
  { cpu.df = false; }
  cpu.call(0x1202a497); return;
 }
 0x1202a4e8 => {
  let t = cpu.r32(0x9e701202u32.wrapping_add(cpu.ebp)); cpu.push(0x1202a4ee); cpu.call(t);
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.of { pc = 0x1202a490; continue; }
  pc = 0x1202a4f2; continue;
 }
 0x1202a4f2 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xd61202a2u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x9e0a1202u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x9e7c1202u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xa2661202u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xa2b51202u32, v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0xa2f71202u32.wrapping_add(cpu.edi)), cpu.ebx); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xa3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x1202a52f); cpu.call(0x0);
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x10u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, 0x10101010u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x10100e10u32), (cpu.ecx as u8)); cpu.w8(0x10100e10u32, v); }
  if !cpu.pf() { cpu.call(0x9d0ac9ab); return; }
  pc = 0x1202a567; continue;
 }
 0x1202a567 => {
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1202a571); f_12028532(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if !cpu.zf { pc = 0x1202a585; continue; }
  pc = 0x1202a57e; continue;
 }
 0x1202a57e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202a585 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202a7f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202a7f8); }
 let mut pc: u32 = 0x1202a7f8;
 loop { match pc {
 0x1202a7f8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x9090906u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  f_1202a842(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202b9c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202b9c0); }
 let mut pc: u32 = 0x1202b9c0;
 loop { match pc {
 0x1202b9c0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x6080808u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.pop(); }
  { let v: u32 = cpu.edi; cpu.edi = v; }
  cpu.unimplemented(0x1202ba10, "Int3");
  cpu.unimplemented(0x1202ba11, "Int3");
  cpu.unimplemented(0x1202ba12, "Int3");
  cpu.unimplemented(0x1202ba13, "Int3");
  cpu.unimplemented(0x1202ba14, "Int3");
  cpu.unimplemented(0x1202ba15, "Int3");
  cpu.unimplemented(0x1202ba16, "Int3");
  cpu.unimplemented(0x1202ba17, "Int3");
  cpu.unimplemented(0x1202ba18, "Int3");
  cpu.unimplemented(0x1202ba19, "Int3");
  cpu.unimplemented(0x1202ba1a, "Int3");
  cpu.unimplemented(0x1202ba1b, "Int3");
  cpu.unimplemented(0x1202ba1c, "Int3");
  cpu.unimplemented(0x1202ba1d, "Int3");
  cpu.unimplemented(0x1202ba1e, "Int3");
  cpu.unimplemented(0x1202ba1f, "Int3");
  { let v: u8 = cpu.sbb8(cpu.r8(0xb40c1202u32.wrapping_add(cpu.eax)), ((cpu.edx >> 8) as u8)); cpu.w8(0xb40c1202u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.st(0) / cpu.rf32(0xb3a51202u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v: u8 = 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0xffffffb4u32.wrapping_add(cpu.edi))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.scas(4, 0); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x70b0b06u32.wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  f_1202ba69(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202ba20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202ba20); }
 let mut pc: u32 = 0x1202ba20;
 loop { match pc {
 0x1202ba20 => {
  { let v: u8 = cpu.sbb8(cpu.r8(0xb40c1202u32.wrapping_add(cpu.eax)), ((cpu.edx >> 8) as u8)); cpu.w8(0xb40c1202u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.st(0) / cpu.rf32(0xb3a51202u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v: u8 = 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0xffffffb4u32.wrapping_add(cpu.edi))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.scas(4, 0); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x70b0b06u32.wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  f_1202ba69(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202ba50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202ba50); }
 let mut pc: u32 = 0x1202ba50;
 loop { match pc {
 0x1202ba50 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x70b0b06u32.wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  f_1202ba69(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202cac4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202cac4); }
 let mut pc: u32 = 0x1202cac4;
 loop { match pc {
 0x1202cac4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1202caee; continue;
 }
 0x1202caee => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x6080808u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.pop(); }
  { let v: u32 = cpu.edi; cpu.edi = v; }
  { cpu.sub8(((cpu.ebx >> 8) as u8), cpu.r8(0xc4701202u32.wrapping_add(cpu.edi))); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp | cpu.eax; cpu.esp = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.esp); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.of { pc = 0x1202caee; continue; }
  pc = 0x1202cb2a; continue;
 }
 0x1202cb2a => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.esp, 0x2u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x701202c4u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1202cb35, "Les");
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1202cb39, "Les");
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), cpu.r8(0xa11202c4u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1001202u32; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x70b0b06u32.wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  f_1202cb5d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202cb14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202cb14); }
 let mut pc: u32 = 0x1202cb14;
 loop { match pc {
 0x1202caee => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x6080808u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.pop(); }
  { let v: u32 = cpu.edi; cpu.edi = v; }
  pc = 0x1202cb14; continue;
 }
 0x1202cb14 => {
  { cpu.sub8(((cpu.ebx >> 8) as u8), cpu.r8(0xc4701202u32.wrapping_add(cpu.edi))); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp | cpu.eax; cpu.esp = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.esp); }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.of { pc = 0x1202caee; continue; }
  pc = 0x1202cb2a; continue;
 }
 0x1202cb2a => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.esp, 0x2u32); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x701202c4u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1202cb35, "Les");
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1202cb39, "Les");
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), cpu.r8(0xa11202c4u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1001202u32; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x70b0b06u32.wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  f_1202cb5d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12031e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12031e90); }
 let mut pc: u32 = 0x12031e90;
 loop { match pc {
 0x12031e90 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x9080706u32.wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0xbu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_12031eb0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12032074(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12032074); }
 let mut pc: u32 = 0x12032074;
 loop { match pc {
 0x12032074 => {
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi))); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.pop(); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  { cpu.pushad(); }
  { cpu.pop(); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  if !cpu.sf { pc = 0x120320a1; continue; }
  pc = 0x12032082; continue;
 }
 0x12032082 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.edx = v; }
  f_12032084(cpu); return;
 }
 0x12032099 => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3800u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120320ac); f_1203220f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x800u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3800u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x12032099; continue; }
  pc = 0x120320c6; continue;
 }
 0x120320a1 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x120320a8); cpu.call(0xfa5a70b3);
  cpu.unimplemented(0x120320a8, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x800u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx ^ cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3800u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x12032099; continue; }
  pc = 0x120320c6; continue;
 }
 0x120320c6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120333e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120333e3); }
 let mut pc: u32 = 0x120333e3;
 loop { match pc {
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203341e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203341e); }
 let mut pc: u32 = 0x1203341e;
 loop { match pc {
 0x1203341e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203349a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203349a); }
 let mut pc: u32 = 0x1203349a;
 loop { match pc {
 0x12006008 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1200600e; continue; }
  pc = 0x1200600c; continue;
 }
 0x1200600c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x1200600e; continue;
 }
 0x1200600e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1203349a => {
  cpu.push(0x1203349f); f_12033574(cpu);
  pc = 0x12006008; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033572(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033572); }
 let mut pc: u32 = 0x12033572;
 loop { match pc {
 0x12033572 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  f_12033574(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12033803(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12033803); }
 let mut pc: u32 = 0x12033803;
 loop { match pc {
 0x12033803 => {
  { let v = cpu.st(0); cpu.wf80(0xffffff62u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf80(0xffffff62u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0xffffff69u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x12033822; continue; }
  pc = 0x12033818; continue;
 }
 0x12033818 => {
  { let v: u8 = 0x7u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1203381f; continue;
 }
 0x1203381f => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12033822 => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1203381f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1203386a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1203386a); }
 let mut pc: u32 = 0x1203386a;
 loop { match pc {
 0x120333e3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x1203e570u32); cpu.fpush(v); }
  { cpu.sub8(cpu.r8(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120336cb; continue; }
  pc = 0x120333f8; continue;
 }
 0x120333f8 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120336cb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120333f8; continue;
 }
 0x1203386a => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x120333e3; continue; }
  pc = 0x12033872; continue;
 }
 0x12033872 => {
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
