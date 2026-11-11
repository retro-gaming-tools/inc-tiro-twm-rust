#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_12011081(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011081); }
 let mut pc: u32 = 0x12011081;
 loop { match pc {
 0x12010f31 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12011081 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  cpu.push(0x12011089); f_12011133(cpu);
  pc = 0x12010f31; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201108e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201108e); }
 let mut pc: u32 = 0x1201108e;
 loop { match pc {
 0x1201108e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12035790u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x120110dd; continue; }
  pc = 0x120110bb; continue;
 }
 0x120110bb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120110dd; continue; }
  pc = 0x120110c5; continue;
 }
 0x120110c5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120110d6); f_120112ae(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x120110dd; continue;
 }
 0x120110dd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120110ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120110ed); }
 let mut pc: u32 = 0x120110ed;
 loop { match pc {
 0x120110ed => {
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.ebp)), 0x1u8); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120110f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120110f5); }
 let mut pc: u32 = 0x120110f5;
 loop { match pc {
 0x120110d6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120110f5 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  cpu.push(0x120110fd); f_12011133(cpu);
  pc = 0x120110d6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120110ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120110ff); }
 let mut pc: u32 = 0x120110ff;
 loop { match pc {
 0x120110ff => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x1201111e; continue; }
  pc = 0x12011113; continue;
 }
 0x12011113 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  pc = 0x1201111e; continue;
 }
 0x1201111e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011120); }
 let mut pc: u32 = 0x12011120;
 loop { match pc {
 0x12011120 => {
  cpu.push(0x12011125); f_12003ac7(cpu);
  { cpu.sub32(cpu.r32(0x64u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { f_12011133(cpu); return; }
  pc = 0x1201112b; continue;
 }
 0x1201112b => {
  cpu.push(0x12011130); f_12003ac7(cpu);
  let t = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.push(0x12011133); cpu.call(t);
  f_12011133(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011133(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011133); }
 let mut pc: u32 = 0x12011133;
 loop { match pc {
 0x12011133 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x120357a0u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011165); f_12003ac7(cpu);
  { cpu.sub32(cpu.r32(0x60u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x12011181; continue; }
  pc = 0x1201116b; continue;
 }
 0x1201116b => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011177); f_12003ac7(cpu);
  let t = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.push(0x1201117a); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x12011181; continue;
 }
 0x12011181 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1201118d); f_120111a8(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201119d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201119d); }
 let mut pc: u32 = 0x1201119d;
 loop { match pc {
 0x1201119d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120111a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120111a3); }
 let mut pc: u32 = 0x120111a3;
 loop { match pc {
 0x1201117a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1201118d); f_120111a8(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120111a3 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  pc = 0x1201117a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120111a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120111a8); }
 let mut pc: u32 = 0x120111a8;
 loop { match pc {
 0x120111a8 => {
  { let v = 0xau32; cpu.push(v); }
  cpu.push(0x120111af); f_12012437(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x16u32; cpu.push(v); }
  cpu.push(0x120111b9); f_12014a18(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x120111c3); f_120123bf(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120111c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120111c7); }
 let mut pc: u32 = 0x120111c7;
 loop { match pc {
 0x120111c7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x120357b8u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1203b05cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201120d; continue; }
  pc = 0x120111fd; continue;
 }
 0x120111fd => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.eax; cpu.push(0x12011206); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201120d; continue;
 }
 0x1201120d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011219); f_12011133(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011229(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011229); }
 let mut pc: u32 = 0x12011229;
 loop { match pc {
 0x12011229 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201122f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201122f); }
 let mut pc: u32 = 0x1201122f;
 loop { match pc {
 0x12011206 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011219); f_12011133(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201122f => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  pc = 0x12011206; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011234(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011234); }
 let mut pc: u32 = 0x12011234;
 loop { match pc {
 0x12011234 => {
  f_12011133(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011239(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011239); }
 let mut pc: u32 = 0x12011239;
 loop { match pc {
 0x12011239 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x120357d0u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x12041000u32); cpu.push(0x12011278); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011280); }
 let mut pc: u32 = 0x12011280;
 loop { match pc {
 0x12011280 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  pc = cpu.eax; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120112ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120112ae); }
 let mut pc: u32 = 0x120112ae;
 loop { match pc {
 0x120112ae => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  pc = cpu.eax; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120112b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120112b5); }
 let mut pc: u32 = 0x120112b5;
 loop { match pc {
 0x120112b5 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  pc = cpu.eax; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120112bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120112bc); }
 let mut pc: u32 = 0x120112bc;
 loop { match pc {
 0x120112bc => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  pc = cpu.eax; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120112c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120112c3); }
 let mut pc: u32 = 0x120112c3;
 loop { match pc {
 0x120112c3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x120112efu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120112ef); f_12015c45(cpu);
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
pub fn f_1201130f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201130f); }
 let mut pc: u32 = 0x1201130f;
 loop { match pc {
 0x1201130f => {
  { let v: u32 = 0x12036d80u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011314(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011314); }
 let mut pc: u32 = 0x12011314;
 loop { match pc {
 0x12011314 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.df = false; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12011340); f_120106eb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011350); }
 let mut pc: u32 = 0x12011350;
 loop { match pc {
 0x12011350 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12011367); f_12010bdc(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201136d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201136d); }
 let mut pc: u32 = 0x1201136d;
 loop { match pc {
 0x1201136d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x120113cfu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120113bd); f_120135ec(cpu);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120113cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120113cf); }
 let mut pc: u32 = 0x120113cf;
 loop { match pc {
 0x120113cf => {
  { cpu.df = false; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120113f4); f_120106eb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120113f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120113f8); }
 let mut pc: u32 = 0x120113f8;
 loop { match pc {
 0x120113f8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x120114bfu32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x12011491u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011484); f_12003ac7(cpu);
  let t = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.push(0x12011487); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_120114bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120114bf); }
 let mut pc: u32 = 0x120114bf;
 loop { match pc {
 0x120114bf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.df = false; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0x66u8); }
  if cpu.zf { pc = 0x120114e1; continue; }
  pc = 0x120114ce; continue;
 }
 0x120114ce => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120114e1 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12011502); f_120106eb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x12011515; continue; }
  pc = 0x1201150e; continue;
 }
 0x1201150e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12011515); f_120112c3(cpu);
  pc = 0x12011515; continue;
 }
 0x12011515 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011521(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011521); }
 let mut pc: u32 = 0x12011521;
 loop { match pc {
 0x12011521 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1201130fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x1201154b); f_120133d7(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x120357fcu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
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
pub fn f_12011572(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011572); }
 let mut pc: u32 = 0x12011572;
 loop { match pc {
 0x12011572 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011575(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011575); }
 let mut pc: u32 = 0x12011575;
 loop { match pc {
 0x12011575 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  { let v: u32 = 0x12035bc4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if cpu.zf { pc = 0x1201158d; continue; }
  pc = 0x12011581; continue;
 }
 0x12011581 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201158a); f_12013634(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1201158d; continue;
 }
 0x1201158d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201158e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201158e); }
 let mut pc: u32 = 0x1201158e;
 loop { match pc {
 0x1201158e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x120115dfu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x120115b8); f_12013435(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x120357fcu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
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
pub fn f_120115df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120115df); }
 let mut pc: u32 = 0x120115df;
 loop { match pc {
 0x120115df => {
  { let v: u32 = 0x12036da8u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120115e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120115e9); }
 let mut pc: u32 = 0x120115e9;
 loop { match pc {
 0x120115e9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120115ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120115ee); }
 let mut pc: u32 = 0x120115ee;
 loop { match pc {
 0x120115ee => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120115fb); f_120134a6(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011601(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011601); }
 let mut pc: u32 = 0x12011601;
 loop { match pc {
 0x12011601 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12011638u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1201162b); f_12011642(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011638(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011638); }
 let mut pc: u32 = 0x12011638;
 loop { match pc {
 0x12011638 => {
  { let v: u32 = 0x12036de0u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011642(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011642); }
 let mut pc: u32 = 0x12011642;
 loop { match pc {
 0x12011642 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201164a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201164a); }
 let mut pc: u32 = 0x1201164a;
 loop { match pc {
 0x1201164a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12011699u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011674); f_120116a3(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x12011686; continue; }
  pc = 0x1201167a; continue;
 }
 0x1201167a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011683); f_12013634(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x12011686; continue;
 }
 0x12011686 => {
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
pub fn f_12011699(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011699); }
 let mut pc: u32 = 0x12011699;
 loop { match pc {
 0x12011699 => {
  { let v: u32 = 0x12036e08u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120116a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120116a3); }
 let mut pc: u32 = 0x120116a3;
 loop { match pc {
 0x120116a3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120116ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120116ab); }
 let mut pc: u32 = 0x120116ab;
 loop { match pc {
 0x120116ab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1201171bu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x12011703; continue; }
  pc = 0x120116d0; continue;
 }
 0x120116d0 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x12011601u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120116e6); f_120134d3(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x120116e7; continue;
 }
 0x120116e7 => {
  cpu.push(0x120116ec); f_12013634(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x120116ef; continue;
 }
 0x120116ef => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12011703 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1201170f); f_12011725(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x120116ef; continue; }
  pc = 0x12011715; continue;
 }
 0x12011715 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x120116e7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201171b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201171b); }
 let mut pc: u32 = 0x1201171b;
 loop { match pc {
 0x1201171b => {
  { let v: u32 = 0x12036e30u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011725(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011725); }
 let mut pc: u32 = 0x12011725;
 loop { match pc {
 0x12011725 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201172d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201172d); }
 let mut pc: u32 = 0x1201172d;
 loop { match pc {
 0x1201172d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x1201177eu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x12011757); f_120133d7(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1203580cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
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
pub fn f_1201177e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201177e); }
 let mut pc: u32 = 0x1201177e;
 loop { match pc {
 0x1201177e => {
  { let v: u32 = 0x12036e58u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011788(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011788); }
 let mut pc: u32 = 0x12011788;
 loop { match pc {
 0x12011788 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011790); }
 let mut pc: u32 = 0x12011790;
 loop { match pc {
 0x12011790 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x120117e1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x120117ba); f_12013435(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1203580cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
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
pub fn f_120117e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120117e1); }
 let mut pc: u32 = 0x120117e1;
 loop { match pc {
 0x120117e1 => {
  { let v: u32 = 0x12036e80u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120117eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120117eb); }
 let mut pc: u32 = 0x120117eb;
 loop { match pc {
 0x120117eb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120117f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120117f3); }
 let mut pc: u32 = 0x120117f3;
 loop { match pc {
 0x120117f3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011800); f_120134a6(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011806(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011806); }
 let mut pc: u32 = 0x12011806;
 loop { match pc {
 0x12011806 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011809(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011809(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011809); }
 let mut pc: u32 = 0x12011809;
 loop { match pc {
 0x12011809 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12011840u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011833); f_1201184a(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011840); }
 let mut pc: u32 = 0x12011840;
 loop { match pc {
 0x12011840 => {
  { let v: u32 = 0x12036ea8u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201184a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201184a); }
 let mut pc: u32 = 0x1201184a;
 loop { match pc {
 0x1201184a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011852(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011852); }
 let mut pc: u32 = 0x12011852;
 loop { match pc {
 0x12011852 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x120118a1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1201187c); f_120118ab(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1201188e; continue; }
  pc = 0x12011882; continue;
 }
 0x12011882 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201188b); f_12013634(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1201188e; continue;
 }
 0x1201188e => {
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
pub fn f_120118a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120118a1); }
 let mut pc: u32 = 0x120118a1;
 loop { match pc {
 0x120118a1 => {
  { let v: u32 = 0x12036ed0u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120118ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120118ab); }
 let mut pc: u32 = 0x120118ab;
 loop { match pc {
 0x120118ab => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120118b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120118b3); }
 let mut pc: u32 = 0x120118b3;
 loop { match pc {
 0x120118b3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12011923u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x1201190b; continue; }
  pc = 0x120118d8; continue;
 }
 0x120118d8 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x12011809u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120118ee); f_120134d3(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x120118ef; continue;
 }
 0x120118ef => {
  cpu.push(0x120118f4); f_12013634(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x120118f7; continue;
 }
 0x120118f7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x1201190b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011917); f_1201192d(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x120118f7; continue; }
  pc = 0x1201191d; continue;
 }
 0x1201191d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x120118ef; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011923(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011923); }
 let mut pc: u32 = 0x12011923;
 loop { match pc {
 0x12011923 => {
  { let v: u32 = 0x12036ef8u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201192d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201192d); }
 let mut pc: u32 = 0x1201192d;
 loop { match pc {
 0x1201192d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011935(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011935); }
 let mut pc: u32 = 0x12011935;
 loop { match pc {
 0x12011935 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x120119a2u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011965); f_120133d7(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x1203580cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1203581cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_1201199a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201199a); }
 let mut pc: u32 = 0x1201199a;
 loop { match pc {
 0x1201199a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120119a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120119a2); }
 let mut pc: u32 = 0x120119a2;
 loop { match pc {
 0x120119a2 => {
  { let v: u32 = 0x12036f20u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120119ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120119ac); }
 let mut pc: u32 = 0x120119ac;
 loop { match pc {
 0x120119ac => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12011a13u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x120119d6); f_12013435(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x1203580cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1203581cu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_12011a0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011a0b); }
 let mut pc: u32 = 0x12011a0b;
 loop { match pc {
 0x12011a0b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011575(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011a13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011a13); }
 let mut pc: u32 = 0x12011a13;
 loop { match pc {
 0x12011a13 => {
  { let v: u32 = 0x12036f50u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011a1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011a1d); }
 let mut pc: u32 = 0x12011a1d;
 loop { match pc {
 0x12011a1d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011809(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011a25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011a25); }
 let mut pc: u32 = 0x12011a25;
 loop { match pc {
 0x12011a25 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011a32); f_120134a6(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011a38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011a38); }
 let mut pc: u32 = 0x12011a38;
 loop { match pc {
 0x12011a38 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12011a6fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011a62); f_12011a79(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011a6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011a6f); }
 let mut pc: u32 = 0x12011a6f;
 loop { match pc {
 0x12011a6f => {
  { let v: u32 = 0x12036f80u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011a79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011a79); }
 let mut pc: u32 = 0x12011a79;
 loop { match pc {
 0x12011a79 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011809(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011a81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011a81); }
 let mut pc: u32 = 0x12011a81;
 loop { match pc {
 0x12011a81 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12011ad0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011aab); f_12011ada(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x12011abd; continue; }
  pc = 0x12011ab1; continue;
 }
 0x12011ab1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011aba); f_12013634(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x12011abd; continue;
 }
 0x12011abd => {
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
pub fn f_12011ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011ad0); }
 let mut pc: u32 = 0x12011ad0;
 loop { match pc {
 0x12011ad0 => {
  { let v: u32 = 0x12036fa8u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011ada(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011ada); }
 let mut pc: u32 = 0x12011ada;
 loop { match pc {
 0x12011ada => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011809(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011ae2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011ae2); }
 let mut pc: u32 = 0x12011ae2;
 loop { match pc {
 0x12011ae2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12011b52u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x12011b3a; continue; }
  pc = 0x12011b07; continue;
 }
 0x12011b07 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = 0x12011a38u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011b1d); f_120134d3(cpu);
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x12011b1e; continue;
 }
 0x12011b1e => {
  cpu.push(0x12011b23); f_12013634(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x12011b26; continue;
 }
 0x12011b26 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12011b3a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12011b46); f_12011b5c(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x12011b26; continue; }
  pc = 0x12011b4c; continue;
 }
 0x12011b4c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x12011b1e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011b52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011b52); }
 let mut pc: u32 = 0x12011b52;
 loop { match pc {
 0x12011b52 => {
  { let v: u32 = 0x12036fd0u32; cpu.eax = v; }
  f_12011314(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011b5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011b5c); }
 let mut pc: u32 = 0x12011b5c;
 loop { match pc {
 0x12011b5c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_12011809(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011b64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011b64); }
 let mut pc: u32 = 0x12011b64;
 loop { match pc {
 0x12011b64 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1bu32; cpu.push(v); }
  { let v: u32 = 0x120357f4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x12011b74); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011b87; continue; }
  pc = 0x12011b7e; continue;
 }
 0x12011b7e => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011b84); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12011b87; continue;
 }
 0x12011b87 => {
  { let v = 0x1bu32; cpu.push(v); }
  cpu.push(0x12011b8e); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011b93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011b93); }
 let mut pc: u32 = 0x12011b93;
 loop { match pc {
 0x12011b93 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x12011b9b); f_12011b64(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x12011bab; continue; }
  pc = 0x12011ba2; continue;
 }
 0x12011ba2 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12011ba8); f_12013634(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x12011bab; continue;
 }
 0x12011bab => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011bb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011bb1); }
 let mut pc: u32 = 0x12011bb1;
 loop { match pc {
 0x12011bb1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x9u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9u32); cpu.eax = v; }
  pc = 0x12011bbb; continue;
 }
 0x12011bbb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x12011bea; continue; }
  pc = 0x12011bc1; continue;
 }
 0x12011bc1 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x12011bd7; continue; }
  pc = 0x12011bc5; continue;
 }
 0x12011bc5 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x12011bea; continue; }
  pc = 0x12011bcd; continue;
 }
 0x12011bcd => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x12011bbb; continue; }
  pc = 0x12011bd7; continue;
 }
 0x12011bd7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x12011bd9; continue;
 }
 0x12011bd9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x12011be7; continue; }
  pc = 0x12011be2; continue;
 }
 0x12011be2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12011be7; continue;
 }
 0x12011be7 => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12011bea => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x12011bd9; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011bf1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011bf1); }
 let mut pc: u32 = 0x12011bf1;
 loop { match pc {
 0x12011bf1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x9u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9u32); cpu.eax = v; }
  pc = 0x12011bfb; continue;
 }
 0x12011bfb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x12011c27; continue; }
  pc = 0x12011c01; continue;
 }
 0x12011c01 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x12011c17; continue; }
  pc = 0x12011c05; continue;
 }
 0x12011c05 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x12011c27; continue; }
  pc = 0x12011c0d; continue;
 }
 0x12011c0d => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x12011bfb; continue; }
  pc = 0x12011c17; continue;
 }
 0x12011c17 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x12011c19; continue;
 }
 0x12011c19 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x12011c24; continue; }
  pc = 0x12011c22; continue;
 }
 0x12011c22 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x12011c24; continue;
 }
 0x12011c24 => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12011c27 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x12011c19; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011c2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011c2e); }
 let mut pc: u32 = 0x12011c2e;
 loop { match pc {
 0x12011c2e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x9u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9u32); cpu.eax = v; }
  pc = 0x12011c38; continue;
 }
 0x12011c38 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x12011c64; continue; }
  pc = 0x12011c3e; continue;
 }
 0x12011c3e => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x12011c54; continue; }
  pc = 0x12011c42; continue;
 }
 0x12011c42 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x12011c64; continue; }
  pc = 0x12011c4a; continue;
 }
 0x12011c4a => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x12011c38; continue; }
  pc = 0x12011c54; continue;
 }
 0x12011c54 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x12011c56; continue;
 }
 0x12011c56 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12011c61; continue; }
  pc = 0x12011c5f; continue;
 }
 0x12011c5f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x12011c61; continue;
 }
 0x12011c61 => {
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12011c64 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x12011c56; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011c6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011c6b); }
 let mut pc: u32 = 0x12011c6b;
 loop { match pc {
 0x12011c6b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011c6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011c6f); }
 let mut pc: u32 = 0x12011c6f;
 loop { match pc {
 0x12011c6f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x12011d1c; continue; }
  pc = 0x12011c7f; continue;
 }
 0x12011c7f => {
  { let v = 0x2800u32; cpu.push(v); }
  { let v: u32 = 0x9u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = 0x12005adbu32; cpu.push(v); }
  { let v = 0x12004858u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x12011c9b); f_1200a016(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x12011cc4; continue; }
  pc = 0x12011cb7; continue;
 }
 0x12011cb7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x20u8); }
  if cpu.zf { pc = 0x12011cb7; continue; }
  pc = 0x12011cc4; continue;
 }
 0x12011cc4 => {
  { let v = 0x1bu32; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x12011ccd); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x12011ce3); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x12011d0f); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1bu32; cpu.push(v); }
  cpu.push(0x12011d19); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x12011d1c; continue;
 }
 0x12011d1c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011d24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011d24); }
 let mut pc: u32 = 0x12011d24;
 loop { match pc {
 0x12011d24 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0xe06d7363u32); }
  if !cpu.zf { pc = 0x12011d50; continue; }
  pc = 0x12011d33; continue;
 }
 0x12011d33 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.eax)), 0x3u32); }
  if !cpu.zf { pc = 0x12011d50; continue; }
  pc = 0x12011d39; continue;
 }
 0x12011d39 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x19930520u32); }
  if !cpu.zf { pc = 0x12011d50; continue; }
  pc = 0x12011d42; continue;
 }
 0x12011d42 => {
  cpu.push(0x12011d47); f_12011133(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12011d50 => {
  { cpu.sub32(cpu.r32(0x1203e704u32), 0x0u32); }
  if cpu.zf { pc = 0x12011d76; continue; }
  pc = 0x12011d59; continue;
 }
 0x12011d59 => {
  { let v: u32 = cpu.r32(0x1203e704u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011d64); f_12011e07(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011d76; continue; }
  pc = 0x12011d6b; continue;
 }
 0x12011d6b => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x1203e704u32); cpu.push(0x12011d72); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12011d76 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011d7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011d7c); }
 let mut pc: u32 = 0x12011d7c;
 loop { match pc {
 0x12011d7c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12011d82); f_12003ac7(cpu);
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  cpu.push(0x12011d8a); f_12003ac7(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011d95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011d95); }
 let mut pc: u32 = 0x12011d95;
 loop { match pc {
 0x12011d95 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12011d9b); f_12003ac7(cpu);
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  cpu.push(0x12011da3); f_12003ac7(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011dae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011dae); }
 let mut pc: u32 = 0x12011dae;
 loop { match pc {
 0x12011dae => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12011db4); f_12003ac7(cpu);
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  cpu.push(0x12011dbc); f_12003ac7(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x68u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011dc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011dc7); }
 let mut pc: u32 = 0x12011dc7;
 loop { match pc {
 0x12011dc7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041218u32); cpu.push(0x12011ddd); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011de3; continue; }
  pc = 0x12011de1; continue;
 }
 0x12011de1 => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x12011de3; continue;
 }
 0x12011de3 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011de7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011de7); }
 let mut pc: u32 = 0x12011de7;
 loop { match pc {
 0x12011de7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041214u32); cpu.push(0x12011dfd); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011e03; continue; }
  pc = 0x12011e01; continue;
 }
 0x12011e01 => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x12011e03; continue;
 }
 0x12011e03 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011e07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011e07); }
 let mut pc: u32 = 0x12011e07;
 loop { match pc {
 0x12011e07 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041210u32); cpu.push(0x12011e18); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011e1e; continue; }
  pc = 0x12011e1c; continue;
 }
 0x12011e1c => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x12011e1e; continue;
 }
 0x12011e1e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011e22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011e22); }
 let mut pc: u32 = 0x12011e22;
 loop { match pc {
 0x12011e22 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x12011e34; continue; }
  pc = 0x12011e2a; continue;
 }
 0x12011e2a => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12011e3a; continue; }
  pc = 0x12011e2f; continue;
 }
 0x12011e2f => {
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x12011e46; continue; }
  pc = 0x12011e34; continue;
 }
 0x12011e34 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12011e3a => {
  { let v: u32 = cpu.r32(0x1203b1f4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1203b1f4u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12011e46 => {
  { let v: u32 = cpu.r32(0x1203b1f4u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011e4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011e4c); }
 let mut pc: u32 = 0x12011e4c;
 loop { match pc {
 0x12011e4c => {
  { let v: u32 = 0x0u32; cpu.w32(0x1203b0f0u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b0f4u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011e60); }
 let mut pc: u32 = 0x12011e60;
 loop { match pc {
 0x12011e60 => {
  { let v: u32 = cpu.r32(0x1203b0f4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011e74; continue; }
  pc = 0x12011e69; continue;
 }
 0x12011e69 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.eax; cpu.push(0x12011e70); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12011e74 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011e77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011e77); }
 let mut pc: u32 = 0x12011e77;
 loop { match pc {
 0x12011e77 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  { let v: u32 = 0x1203e708u32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x1204120cu32); cpu.push(0x12011e8f); cpu.call(t);
  { let v: u32 = cpu.r32(0x1203b1e4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1203b130u32, v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x12011ea5; continue; }
  pc = 0x12011e9f; continue;
 }
 0x12011e9f => {
  { let v: u32 = cpu.r32(0x1203b1e4u32); cpu.esi = v; }
  pc = 0x12011ea5; continue;
 }
 0x12011ea5 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12011eb9); f_12011f25(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12011ecd); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12011ee0; continue; }
  pc = 0x12011ed6; continue;
 }
 0x12011ed6 => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12011edd); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12011ee0; continue;
 }
 0x12011ee0 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12011ef9); f_12011f25(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1203b118u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b114u32, v); }
  cpu.push(0x12011f11); f_12012938(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011f1f; continue; }
  pc = 0x12011f15; continue;
 }
 0x12011f15 => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12011f1c); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12011f1f; continue;
 }
 0x12011f1f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12011f25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12011f25); }
 let mut pc: u32 = 0x12011f25;
 loop { match pc {
 0x12011f25 => {
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
  if cpu.zf { pc = 0x12011f57; continue; }
  pc = 0x12011f4c; continue;
 }
 0x12011f4c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x12011f57; continue;
 }
 0x12011f57 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011f60; continue; }
  pc = 0x12011f5b; continue;
 }
 0x12011f5b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12011f60; continue;
 }
 0x12011f60 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if cpu.zf { pc = 0x12011fc5; continue; }
  pc = 0x12011f67; continue;
 }
 0x12011f67 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011f72; continue; }
  pc = 0x12011f6d; continue;
 }
 0x12011f6d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12011f72; continue;
 }
 0x12011f72 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if cpu.zf { pc = 0x12011f8e; continue; }
  pc = 0x12011f82; continue;
 }
 0x12011f82 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011f8d; continue; }
  pc = 0x12011f88; continue;
 }
 0x12011f88 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12011f8d; continue;
 }
 0x12011f8d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x12011f8e; continue;
 }
 0x12011f8e => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x12011f9c; continue; }
  pc = 0x12011f93; continue;
 }
 0x12011f93 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12011fa0; continue; }
  pc = 0x12011f97; continue;
 }
 0x12011f97 => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if !cpu.zf { pc = 0x12011f67; continue; }
  pc = 0x12011f9c; continue;
 }
 0x12011f9c => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x12011fbb; continue; }
  pc = 0x12011fa0; continue;
 }
 0x12011fa0 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  pc = 0x12011fa1; continue;
 }
 0x12011fa1 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x12011fa3; continue;
 }
 0x12011fa3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x120120f1; continue; }
  pc = 0x12011fac; continue;
 }
 0x12011fac => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x12011fb8; continue; }
  pc = 0x12011fb3; continue;
 }
 0x12011fb3 => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if !cpu.zf { pc = 0x1201200d; continue; }
  pc = 0x12011fb8; continue;
 }
 0x12011fb8 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x12011fac; continue;
 }
 0x12011fbb => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011fa1; continue; }
  pc = 0x12011fbf; continue;
 }
 0x12011fbf => {
  { let v: u8 = 0x0u8; cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  pc = 0x12011fa1; continue;
 }
 0x12011fc5 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if cpu.zf { pc = 0x12011ffb; continue; }
  pc = 0x12011fcb; continue;
 }
 0x12011fcb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x12011ffb; continue; }
  pc = 0x12011fd1; continue;
 }
 0x12011fd1 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.edx)), 0x4u8); }
  if cpu.zf { pc = 0x12011fea; continue; }
  pc = 0x12011fde; continue;
 }
 0x12011fde => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011fea; continue; }
  pc = 0x12011fe4; continue;
 }
 0x12011fe4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12011fea; continue;
 }
 0x12011fea => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12011ff5; continue; }
  pc = 0x12011ff0; continue;
 }
 0x12011ff0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12011ff5; continue;
 }
 0x12011ff5 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if !cpu.zf { pc = 0x12011fcb; continue; }
  pc = 0x12011ffb; continue;
 }
 0x12011ffb => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012005; continue; }
  pc = 0x12012001; continue;
 }
 0x12012001 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12012005; continue;
 }
 0x12012005 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if !cpu.zf { pc = 0x12011fa1; continue; }
  pc = 0x1201200a; continue;
 }
 0x1201200a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x12011fa1; continue;
 }
 0x1201200d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x120120f1; continue; }
  pc = 0x12012016; continue;
 }
 0x12012016 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12012028; continue; }
  pc = 0x1201201d; continue;
 }
 0x1201201d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x12012028; continue;
 }
 0x12012028 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012037; continue; }
  pc = 0x12012032; continue;
 }
 0x12012032 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12012037; continue;
 }
 0x12012037 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12012039; continue;
 }
 0x12012039 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x5cu8); }
  if !cpu.zf { pc = 0x1201204c; continue; }
  pc = 0x12012045; continue;
 }
 0x12012045 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x5cu8); }
  if cpu.zf { pc = 0x12012045; continue; }
  pc = 0x1201204c; continue;
 }
 0x1201204c => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if !cpu.zf { pc = 0x12012071; continue; }
  pc = 0x12012051; continue;
 }
 0x12012051 => {
  { cpu.and32(cpu.ebp, 0x1u32); }
  if !cpu.zf { pc = 0x1201206e; continue; }
  pc = 0x12012059; continue;
 }
 0x12012059 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x120120c6; continue; }
  pc = 0x1201205d; continue;
 }
 0x1201205d => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x22u8); }
  if !cpu.zf { pc = 0x120120c6; continue; }
  pc = 0x12012065; continue;
 }
 0x12012065 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x12012067; continue;
 }
 0x12012067 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x1201206e; continue;
 }
 0x1201206e => {
  { let v: u32 = cpu.shr32(cpu.ebp, 1u32); cpu.ebp = v; }
  pc = 0x12012071; continue;
 }
 0x12012071 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x12012089; continue; }
  pc = 0x12012078; continue;
 }
 0x12012078 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012080; continue; }
  pc = 0x1201207c; continue;
 }
 0x1201207c => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12012080; continue;
 }
 0x12012080 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x12012078; continue; }
  pc = 0x12012089; continue;
 }
 0x12012089 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x120120e2; continue; }
  pc = 0x1201208f; continue;
 }
 0x1201208f => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201209d; continue; }
  pc = 0x12012093; continue;
 }
 0x12012093 => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x120120e2; continue; }
  pc = 0x12012098; continue;
 }
 0x12012098 => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if cpu.zf { pc = 0x120120e2; continue; }
  pc = 0x1201209d; continue;
 }
 0x1201209d => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x120120dc; continue; }
  pc = 0x120120a1; continue;
 }
 0x120120a1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120120ca; continue; }
  pc = 0x120120a5; continue;
 }
 0x120120a5 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if cpu.zf { pc = 0x120120b8; continue; }
  pc = 0x120120b2; continue;
 }
 0x120120b2 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x120120b8; continue;
 }
 0x120120b8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12012039; continue;
 }
 0x120120c6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x12012067; continue;
 }
 0x120120ca => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if cpu.zf { pc = 0x120120da; continue; }
  pc = 0x120120d7; continue;
 }
 0x120120d7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x120120da; continue;
 }
 0x120120da => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x120120dc; continue;
 }
 0x120120dc => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x12012039; continue;
 }
 0x120120e2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120120ea; continue; }
  pc = 0x120120e6; continue;
 }
 0x120120e6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x120120ea; continue;
 }
 0x120120ea => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12011fa3; continue;
 }
 0x120120f1 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12012102; continue; }
  pc = 0x120120f8; continue;
 }
 0x120120f8 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x12012102; continue;
 }
 0x12012102 => {
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
pub fn f_1201210d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201210d); }
 let mut pc: u32 = 0x1201210d;
 loop { match pc {
 0x1201210d => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  { let v: u32 = 0x1203e810u32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041208u32); cpu.push(0x12012125); cpu.call(t);
  { let v: u32 = cpu.r32(0x1203b1e0u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1203b134u32, v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x1201213c; continue; }
  pc = 0x12012136; continue;
 }
 0x12012136 => {
  { let v: u32 = cpu.r32(0x1203b1e0u32); cpu.esi = v; }
  pc = 0x1201213c; continue;
 }
 0x1201213c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12012150); f_120121bd(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012165); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12012178; continue; }
  pc = 0x1201216e; continue;
 }
 0x1201216e => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12012175); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12012178; continue;
 }
 0x12012178 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12012191); f_120121bd(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1203b11cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b114u32, v); }
  cpu.push(0x120121a9); f_12012f75(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120121b7; continue; }
  pc = 0x120121ad; continue;
 }
 0x120121ad => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x120121b4); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120121b7; continue;
 }
 0x120121b7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120121bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120121bd); }
 let mut pc: u32 = 0x120121bd;
 loop { match pc {
 0x120121bd => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x120121ef; continue; }
  pc = 0x120121e4; continue;
 }
 0x120121e4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x120121ef; continue;
 }
 0x120121ef => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x120121fd; continue; }
  pc = 0x120121f3; continue;
 }
 0x120121f3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x120121fd; continue;
 }
 0x120121fd => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if cpu.zf { pc = 0x12012260; continue; }
  pc = 0x12012205; continue;
 }
 0x12012205 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012215; continue; }
  pc = 0x1201220b; continue;
 }
 0x1201220b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12012215; continue;
 }
 0x12012215 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if cpu.zf { pc = 0x1201222c; continue; }
  pc = 0x12012221; continue;
 }
 0x12012221 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x12012231; continue; }
  pc = 0x12012226; continue;
 }
 0x12012226 => {
  { cpu.sub16((cpu.eax as u16), 0x9u16); }
  if !cpu.zf { pc = 0x12012205; continue; }
  pc = 0x1201222c; continue;
 }
 0x1201222c => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x12012254; continue; }
  pc = 0x12012231; continue;
 }
 0x12012231 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x2u32); cpu.esi = v; }
  pc = 0x12012234; continue;
 }
 0x12012234 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x12012236; continue;
 }
 0x12012236 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x12012374; continue; }
  pc = 0x12012240; continue;
 }
 0x12012240 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if cpu.zf { pc = 0x1201224f; continue; }
  pc = 0x12012249; continue;
 }
 0x12012249 => {
  { cpu.sub16((cpu.eax as u16), 0x9u16); }
  if !cpu.zf { pc = 0x120122a1; continue; }
  pc = 0x1201224f; continue;
 }
 0x1201224f => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x12012240; continue;
 }
 0x12012254 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012234; continue; }
  pc = 0x12012258; continue;
 }
 0x12012258 => {
  { let v: u16 = 0x0u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12012234; continue;
 }
 0x12012260 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if cpu.zf { pc = 0x12012288; continue; }
  pc = 0x12012269; continue;
 }
 0x12012269 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x12012288; continue; }
  pc = 0x1201226f; continue;
 }
 0x1201226f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1201227f; continue; }
  pc = 0x12012275; continue;
 }
 0x12012275 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x1201227f; continue;
 }
 0x1201227f => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if !cpu.zf { pc = 0x12012269; continue; }
  pc = 0x12012288; continue;
 }
 0x12012288 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012296; continue; }
  pc = 0x1201228e; continue;
 }
 0x1201228e => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x12012296; continue;
 }
 0x12012296 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if !cpu.zf { pc = 0x12012234; continue; }
  pc = 0x1201229c; continue;
 }
 0x1201229c => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x12012234; continue;
 }
 0x120122a1 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x12012374; continue; }
  pc = 0x120122ab; continue;
 }
 0x120122ab => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x120122bd; continue; }
  pc = 0x120122b2; continue;
 }
 0x120122b2 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x120122bd; continue;
 }
 0x120122bd => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x120122d1; continue; }
  pc = 0x120122c7; continue;
 }
 0x120122c7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x120122d1; continue;
 }
 0x120122d1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x120122d3; continue;
 }
 0x120122d3 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x5cu16); }
  if !cpu.zf { pc = 0x120122ea; continue; }
  pc = 0x120122e0; continue;
 }
 0x120122e0 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x5cu16); }
  if cpu.zf { pc = 0x120122e0; continue; }
  pc = 0x120122ea; continue;
 }
 0x120122ea => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if !cpu.zf { pc = 0x12012311; continue; }
  pc = 0x120122f0; continue;
 }
 0x120122f0 => {
  { cpu.and32(cpu.ebp, 0x1u32); }
  if !cpu.zf { pc = 0x1201230e; continue; }
  pc = 0x120122f8; continue;
 }
 0x120122f8 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201235d; continue; }
  pc = 0x120122fc; continue;
 }
 0x120122fc => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x22u16); }
  if !cpu.zf { pc = 0x1201235d; continue; }
  pc = 0x12012305; continue;
 }
 0x12012305 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x12012307; continue;
 }
 0x12012307 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x1201230e; continue;
 }
 0x1201230e => {
  { let v: u32 = cpu.shr32(cpu.ebp, 1u32); cpu.ebp = v; }
  pc = 0x12012311; continue;
 }
 0x12012311 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201232d; continue; }
  pc = 0x12012318; continue;
 }
 0x12012318 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012324; continue; }
  pc = 0x1201231c; continue;
 }
 0x1201231c => {
  { let v: u16 = 0x5cu16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x12012324; continue;
 }
 0x12012324 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12012318; continue; }
  pc = 0x1201232d; continue;
 }
 0x1201232d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x12012361; continue; }
  pc = 0x12012335; continue;
 }
 0x12012335 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12012345; continue; }
  pc = 0x12012339; continue;
 }
 0x12012339 => {
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if cpu.zf { pc = 0x12012361; continue; }
  pc = 0x1201233f; continue;
 }
 0x1201233f => {
  { cpu.sub16((cpu.eax as u16), 0x9u16); }
  if cpu.zf { pc = 0x12012361; continue; }
  pc = 0x12012345; continue;
 }
 0x12012345 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12012355; continue; }
  pc = 0x12012349; continue;
 }
 0x12012349 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012353; continue; }
  pc = 0x1201234d; continue;
 }
 0x1201234d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x12012353; continue;
 }
 0x12012353 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x12012355; continue;
 }
 0x12012355 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x120122d3; continue;
 }
 0x1201235d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x12012307; continue;
 }
 0x12012361 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1201236d; continue; }
  pc = 0x12012365; continue;
 }
 0x12012365 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x1201236d; continue;
 }
 0x1201236d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x12012236; continue;
 }
 0x12012374 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12012385; continue; }
  pc = 0x1201237b; continue;
 }
 0x1201237b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12012385; continue;
 }
 0x12012385 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012390); }
 let mut pc: u32 = 0x12012390;
 loop { match pc {
 0x12012390 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x1000u32); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  if cpu.cf { pc = 0x120123b0; continue; }
  pc = 0x1201239c; continue;
 }
 0x1201239c => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x1000u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1000u32); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); }
  { cpu.sub32(cpu.eax, 0x1000u32); }
  if !cpu.cf { pc = 0x1201239c; continue; }
  pc = 0x120123b0; continue;
 }
 0x120123b0 => {
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
pub fn f_120123bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120123bf); }
 let mut pc: u32 = 0x120123bf;
 loop { match pc {
 0x120123bf => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120123cd); f_12005b6f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120123d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120123d1); }
 let mut pc: u32 = 0x120123d1;
 loop { match pc {
 0x120123d1 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x120123dc); f_12005b6f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120123e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120123e0); }
 let mut pc: u32 = 0x120123e0;
 loop { match pc {
 0x120123e0 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x120123eb); f_12005b6f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120123ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120123ef); }
 let mut pc: u32 = 0x120123ef;
 loop { match pc {
 0x120123ef => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x120123f6); f_12012684(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120123fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120123fa); }
 let mut pc: u32 = 0x120123fa;
 loop { match pc {
 0x120123fa => {
  { let v: u32 = cpu.r32(0x1203b1f4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x12012411; continue; }
  pc = 0x12012404; continue;
 }
 0x12012404 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12012436; continue; }
  pc = 0x12012408; continue;
 }
 0x12012408 => {
  { cpu.sub32(cpu.r32(0x1203b1f8u32), 0x1u32); }
  if !cpu.zf { pc = 0x12012436; continue; }
  pc = 0x12012411; continue;
 }
 0x12012411 => {
  { let v = 0xfcu32; cpu.push(v); }
  cpu.push(0x1201241b); f_12012437(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203b1d8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012429; continue; }
  pc = 0x12012427; continue;
 }
 0x12012427 => {
  let t = cpu.eax; cpu.push(0x12012429); cpu.call(t);
  pc = 0x12012429; continue;
 }
 0x12012429 => {
  { let v = 0xffu32; cpu.push(v); }
  cpu.push(0x12012433); f_12012437(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x12012436; continue;
 }
 0x12012436 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012437(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012437); }
 let mut pc: u32 = 0x12012437;
 loop { match pc {
 0x12012437 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x1a8u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x1203b148u32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x1201244f; continue;
 }
 0x1201244f => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edx); }
  if cpu.zf { pc = 0x1201245e; continue; }
  pc = 0x12012453; continue;
 }
 0x12012453 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x1203b1d8u32); }
  if cpu.cf { pc = 0x1201244f; continue; }
  pc = 0x1201245e; continue;
 }
 0x1201245e => {
  { cpu.sub32(cpu.r32(0x1203b148u32.wrapping_add(cpu.ecx.wrapping_mul(8))), cpu.edx); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.ebx = v; }
  if !cpu.zf { pc = 0x12012622; continue; }
  pc = 0x12012472; continue;
 }
 0x12012472 => {
  { cpu.sub32(cpu.r32(0x1203b1f4u32), 0x1u32); }
  if cpu.zf { pc = 0x120125de; continue; }
  pc = 0x1201247f; continue;
 }
 0x1201247f => {
  { cpu.sub32(cpu.r32(0x1203b1f4u32), 0x0u32); }
  if !cpu.zf { pc = 0x12012495; continue; }
  pc = 0x12012488; continue;
 }
 0x12012488 => {
  { cpu.sub32(cpu.r32(0x1203b1f8u32), 0x1u32); }
  if cpu.zf { pc = 0x120125de; continue; }
  pc = 0x12012495; continue;
 }
 0x12012495 => {
  { cpu.sub32(cpu.edx, 0xfcu32); }
  if cpu.zf { pc = 0x12012622; continue; }
  pc = 0x120124a1; continue;
 }
 0x120124a1 => {
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1204120cu32); cpu.ebp = v; }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x120124b8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120124d2; continue; }
  pc = 0x120124bc; continue;
 }
 0x120124bc => {
  { let v: u32 = 0x12035af8u32; cpu.esi = v; }
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  pc = 0x120124d2; continue;
 }
 0x120124d2 => {
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.ebp = v; }
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3cu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12012516; continue; }
  pc = 0x120124f0; continue;
 }
 0x120124f0 => {
  { let v: u32 = 0xb4u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = 0x3u32; cpu.push(v); }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x7cu32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ebp = v; }
  { let v = 0x12035630u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12012513); f_12003cd6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x12012516; continue;
 }
 0x12012516 => {
  { let v: u32 = 0x12035adcu32; cpu.esi = v; }
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
  { let v: u32 = 0x12035ad8u32; cpu.edi = v; }
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
  { let v: u32 = cpu.r32(0x1203b14cu32.wrapping_add(cpu.ebx)); cpu.edi = v; }
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
  { let v = 0x12035ab0u32; cpu.push(v); }
  { cpu.movs(1, true); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120125d0); f_12014324(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1a8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120125de => {
  { cpu.sub32(cpu.r32(0x1203ef10u32), 0x0u32); }
  if cpu.zf { pc = 0x120125f4; continue; }
  pc = 0x120125e7; continue;
 }
 0x120125e7 => {
  { let v: u32 = cpu.r32(0x1203ef10u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x120125fe; continue; }
  pc = 0x120125f4; continue;
 }
 0x120125f4 => {
  { let v = 0xfffffff4u32; cpu.push(v); }
  let t = cpu.r32(0x120411f4u32); cpu.push(0x120125fc); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x120125fe; continue;
 }
 0x120125fe => {
  { let v: u32 = cpu.r32(0x1203b14cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
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
  let t = cpu.r32(0x120411f8u32); cpu.push(0x12012622); cpu.call(t);
  pc = 0x12012622; continue;
 }
 0x12012622 => {
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
pub fn f_1201262d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201262d); }
 let mut pc: u32 = 0x1201262d;
 loop { match pc {
 0x1201262d => {
  cpu.push(0x12012632); f_12003ac7(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1203b434u32, v); }
  if cpu.zf { pc = 0x1201267d; continue; }
  pc = 0x12012645; continue;
 }
 0x12012645 => {
  cpu.push(0x1201264a); f_1201210d(cpu);
  pc = 0x1201264a; continue;
 }
 0x1201264a => {
  { let v: u32 = cpu.r32(0x1203b114u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203b11cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1203b128u32), 0x0u32); }
  if !cpu.zf { pc = 0x12012670; continue; }
  pc = 0x1201266b; continue;
 }
 0x1201266b => {
  cpu.push(0x12012670); f_120143b6(cpu);
  pc = 0x12012670; continue;
 }
 0x12012670 => {
  { let v: u32 = cpu.r32(0x1203b128u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201267d => {
  cpu.push(0x12012682); f_12012d25(cpu);
  pc = 0x1201264a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012684(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012684); }
 let mut pc: u32 = 0x12012684;
 loop { match pc {
 0x12012684 => {
  { let v: u32 = cpu.r32(0x1203b1f4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1201269b; continue; }
  pc = 0x1201268e; continue;
 }
 0x1201268e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120126a0; continue; }
  pc = 0x12012692; continue;
 }
 0x12012692 => {
  { cpu.sub32(cpu.r32(0x1203b1f8u32), 0x1u32); }
  if !cpu.zf { pc = 0x120126a0; continue; }
  pc = 0x1201269b; continue;
 }
 0x1201269b => {
  cpu.push(0x120126a0); f_120123fa(cpu);
  pc = 0x120126a0; continue;
 }
 0x120126a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120126aa); f_12012437(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xffu32; cpu.push(v); }
  let t = cpu.r32(0x1203b1f0u32); cpu.push(0x120126b8); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126bc); }
 let mut pc: u32 = 0x120126bc;
 loop { match pc {
 0x120126bc => {
  { let v: u32 = 0x1203b1e0u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126c2); }
 let mut pc: u32 = 0x120126c2;
 loop { match pc {
 0x120126c2 => {
  { let v: u32 = 0x1203b438u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126c8); }
 let mut pc: u32 = 0x120126c8;
 loop { match pc {
 0x120126c8 => {
  { let v: u32 = 0x1203b118u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126ce); }
 let mut pc: u32 = 0x120126ce;
 loop { match pc {
 0x120126ce => {
  { let v: u32 = 0x1203b11cu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126d4); }
 let mut pc: u32 = 0x120126d4;
 loop { match pc {
 0x120126d4 => {
  { let v: u32 = 0x1203c474u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126da); }
 let mut pc: u32 = 0x120126da;
 loop { match pc {
 0x120126da => {
  { let v: u32 = 0x1203c478u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126e0); }
 let mut pc: u32 = 0x120126e0;
 loop { match pc {
 0x120126e0 => {
  { let v: u32 = 0x1203b120u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126e6); }
 let mut pc: u32 = 0x120126e6;
 loop { match pc {
 0x120126e6 => {
  { let v: u32 = 0x1203b128u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126ec); }
 let mut pc: u32 = 0x120126ec;
 loop { match pc {
 0x120126ec => {
  { let v: u32 = 0x1203b430u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126f2); }
 let mut pc: u32 = 0x120126f2;
 loop { match pc {
 0x120126f2 => {
  { let v: u32 = 0x1203b124u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126f8); }
 let mut pc: u32 = 0x120126f8;
 loop { match pc {
 0x120126f8 => {
  { let v: u32 = 0x1203b12cu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120126fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120126fe); }
 let mut pc: u32 = 0x120126fe;
 loop { match pc {
 0x120126fe => {
  { let v: u32 = 0x1203c1d8u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012704(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012704); }
 let mut pc: u32 = 0x12012704;
 loop { match pc {
 0x12012704 => {
  { let v: u32 = 0x1203befcu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201270a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201270a); }
 let mut pc: u32 = 0x1201270a;
 loop { match pc {
 0x1201270a => {
  { let v: u32 = 0x1203b104u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012710); }
 let mut pc: u32 = 0x12012710;
 loop { match pc {
 0x12012710 => {
  { let v: u32 = 0x1203bf08u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012716(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012716); }
 let mut pc: u32 = 0x12012716;
 loop { match pc {
 0x12012716 => {
  { let v: u32 = 0x1203bf0cu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201271c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201271c); }
 let mut pc: u32 = 0x1201271c;
 loop { match pc {
 0x1201271c => {
  { let v: u32 = 0x1203b130u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012722(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012722); }
 let mut pc: u32 = 0x12012722;
 loop { match pc {
 0x12012722 => {
  { let v: u32 = 0x1203b134u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012728(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012728); }
 let mut pc: u32 = 0x12012728;
 loop { match pc {
 0x12012728 => {
  { let v: u32 = 0x1203c470u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201272e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201272e); }
 let mut pc: u32 = 0x1201272e;
 loop { match pc {
 0x1201272e => {
  { let v: u32 = 0x1203c500u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012734(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012734); }
 let mut pc: u32 = 0x12012734;
 loop { match pc {
 0x12012734 => {
  { let v: u32 = 0x1203b10cu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201273a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201273a); }
 let mut pc: u32 = 0x1201273a;
 loop { match pc {
 0x1201273a => {
  { let v: u32 = 0x1203b110u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012740); }
 let mut pc: u32 = 0x12012740;
 loop { match pc {
 0x12012740 => {
  { let v: u32 = 0x1203b108u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012746(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012746); }
 let mut pc: u32 = 0x12012746;
 loop { match pc {
 0x12012746 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x1203c1d8u32); }
  if cpu.cf { pc = 0x12012770; continue; }
  pc = 0x12012754; continue;
 }
 0x12012754 => {
  { cpu.sub32(cpu.eax, 0x1203c438u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12012770; continue; }
  pc = 0x1201275b; continue;
 }
 0x1201275b => {
  { let v: u32 = cpu.eax.wrapping_sub(0x1203c1d8u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201276c); f_1200420b(cpu);
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012770 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411d8u32); cpu.push(0x1201277a); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201277c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201277c); }
 let mut pc: u32 = 0x1201277c;
 loop { match pc {
 0x1201277c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x1203c1d8u32); }
  if cpu.cf { pc = 0x120127a6; continue; }
  pc = 0x1201278a; continue;
 }
 0x1201278a => {
  { cpu.sub32(cpu.eax, 0x1203c438u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120127a6; continue; }
  pc = 0x12012791; continue;
 }
 0x12012791 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x1203c1d8u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120127a2); f_120040fd(cpu);
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120127a6 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411d4u32); cpu.push(0x120127b0); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120127b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120127b2); }
 let mut pc: u32 = 0x120127b2;
 loop { match pc {
 0x120127b2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v = 0x74u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x120127bf); f_12002393(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x12012805; continue; }
  pc = 0x120127c8; continue;
 }
 0x120127c8 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120127ce); f_12003b0b(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4cu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x12012834u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x120411c8u32); cpu.push(0x120127f4); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12012823; continue; }
  pc = 0x120127fd; continue;
 }
 0x120127fd => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12012803); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x12012805; continue;
 }
 0x12012805 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201280b); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201281b; continue; }
  pc = 0x12012812; continue;
 }
 0x12012812 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12012818); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1201281b; continue;
 }
 0x1201281b => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012823 => {
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x120411d0u32); cpu.push(0x1201282a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x120127fd; continue; }
  pc = 0x1201282f; continue;
 }
 0x1201282f => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012834(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012834); }
 let mut pc: u32 = 0x12012834;
 loop { match pc {
 0x12012834 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12035b80u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203b2c0u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411c4u32); cpu.push(0x1201286a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12012878; continue; }
  pc = 0x1201286e; continue;
 }
 0x1201286e => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x12012875); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12012878; continue;
 }
 0x12012878 => {
  { cpu.sub32(cpu.r32(0x1203d880u32), 0x0u32); }
  if cpu.zf { pc = 0x12012887; continue; }
  pc = 0x12012881; continue;
 }
 0x12012881 => {
  let t = cpu.r32(0x1203d880u32); cpu.push(0x12012887); cpu.call(t);
  pc = 0x12012887; continue;
 }
 0x12012887 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x48u32.wrapping_add(cpu.esi)); cpu.push(0x12012895); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x1201289d); f_120128e5(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
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
pub fn f_120128b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120128b9); }
 let mut pc: u32 = 0x120128b9;
 loop { match pc {
 0x120128b9 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120128d0); f_1201463a(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120128d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120128d4); }
 let mut pc: u32 = 0x120128d4;
 loop { match pc {
 0x1201289d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x120128d4 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120128e0); f_120123bf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201289d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120128e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120128e5); }
 let mut pc: u32 = 0x120128e5;
 loop { match pc {
 0x120128e5 => {
  { cpu.sub32(cpu.r32(0x1203d884u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x120128f5; continue; }
  pc = 0x120128ef; continue;
 }
 0x120128ef => {
  let t = cpu.r32(0x1203d884u32); cpu.push(0x120128f5); cpu.call(t);
  pc = 0x120128f5; continue;
 }
 0x120128f5 => {
  cpu.push(0x120128fa); f_12003ac7(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201290a; continue; }
  pc = 0x12012900; continue;
 }
 0x12012900 => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x12012907); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201290a; continue;
 }
 0x1201290a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x12012919; continue; }
  pc = 0x12012912; continue;
 }
 0x12012912 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x12012919); cpu.call(t);
  pc = 0x12012919; continue;
 }
 0x12012919 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201291f); f_12003915(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x120411c0u32); cpu.push(0x1201292a); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201292c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201292c); }
 let mut pc: u32 = 0x1201292c;
 loop { match pc {
 0x1201292c => {
  pc = cpu.r32(0x120411b8u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012932(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012932); }
 let mut pc: u32 = 0x12012932;
 loop { match pc {
 0x12012932 => {
  pc = cpu.r32(0x120411a4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012938); }
 let mut pc: u32 = 0x12012938;
 loop { match pc {
 0x12012938 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203b118u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203eb88u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203eb7cu32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.zf { pc = 0x120129af; continue; }
  pc = 0x1201294f; continue;
 }
 0x1201294f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x22u8); }
  if !cpu.zf { pc = 0x1201296f; continue; }
  pc = 0x1201295b; continue;
 }
 0x1201295b => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012961); f_12012c0a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120129a7; continue; }
  pc = 0x12012968; continue;
 }
 0x12012968 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201296f => {
  { let v = 0x12035b9cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201297a); f_12004309(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012998; continue; }
  pc = 0x12012981; continue;
 }
 0x12012981 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201298a); f_12012a3e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120129a7; continue; }
  pc = 0x12012991; continue;
 }
 0x12012991 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012998 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120129a0); f_12012c0a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120129dd; continue; }
  pc = 0x120129a7; continue;
 }
 0x120129a7 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1201294f; continue; }
  pc = 0x120129af; continue;
 }
 0x120129af => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1203eb7cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x120129c2; continue; }
  pc = 0x120129ba; continue;
 }
 0x120129ba => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120129ba; continue; }
  pc = 0x120129c2; continue;
 }
 0x120129c2 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120129cf); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120129e4; continue; }
  pc = 0x120129d6; continue;
 }
 0x120129d6 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120129dd => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120129e4 => {
  { let v: u32 = cpu.r32(0x1203eb7cu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b118u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1203b114u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012a07; continue; }
  pc = 0x120129f9; continue;
 }
 0x120129f9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x120129f9; continue; }
  pc = 0x12012a07; continue;
 }
 0x12012a07 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1203eb7cu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012a3a; continue; }
  pc = 0x12012a17; continue;
 }
 0x12012a17 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x12012a19; continue;
 }
 0x12012a19 => {
  { let v: u32 = cpu.r32(0x1203eb7cu32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1203eb7cu32, v); }
  cpu.push(0x12012a2d); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203eb7cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { pc = 0x12012a19; continue; }
  pc = 0x12012a3a; continue;
 }
 0x12012a3a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012a3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012a3e); }
 let mut pc: u32 = 0x12012a3e;
 loop { match pc {
 0x12012a3e => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x12012a7f; continue; }
  pc = 0x12012a5b; continue;
 }
 0x12012a5b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x12012a7f; continue; }
  pc = 0x12012a61; continue;
 }
 0x12012a61 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if cpu.zf { pc = 0x12012a7f; continue; }
  pc = 0x12012a65; continue;
 }
 0x12012a65 => {
  { cpu.sub8((cpu.eax as u8), 0x3au8); }
  if cpu.zf { pc = 0x12012a7f; continue; }
  pc = 0x12012a69; continue;
 }
 0x12012a69 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if !cpu.cf { pc = 0x12012a7d; continue; }
  pc = 0x12012a6e; continue;
 }
 0x12012a6e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12012a76); f_120139cc(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  pc = 0x12012a7d; continue;
 }
 0x12012a7d => {
  if !cpu.zf { pc = 0x12012a5b; continue; }
  pc = 0x12012a7f; continue;
 }
 0x12012a7f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x3au8); }
  if !cpu.zf { pc = 0x12012aa0; continue; }
  pc = 0x12012a86; continue;
 }
 0x12012a86 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x12012aa0; continue; }
  pc = 0x12012a8f; continue;
 }
 0x12012a8f => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12012a95); f_12012c0a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012aa0 => {
  { cpu.sub8((cpu.ecx as u8), 0x5cu8); }
  if cpu.zf { pc = 0x12012aaf; continue; }
  pc = 0x12012aa5; continue;
 }
 0x12012aa5 => {
  { cpu.sub8((cpu.ecx as u8), 0x2fu8); }
  if cpu.zf { pc = 0x12012aaf; continue; }
  pc = 0x12012aaa; continue;
 }
 0x12012aaa => {
  { cpu.sub8((cpu.ecx as u8), 0x3au8); }
  if !cpu.zf { pc = 0x12012ab8; continue; }
  pc = 0x12012aaf; continue;
 }
 0x12012aaf => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x12012ab8; continue;
 }
 0x12012ab8 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12012abe); f_12012c91(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12012bf9; continue; }
  pc = 0x12012acb; continue;
 }
 0x12012acb => {
  { let v: u32 = cpu.r32(0x1203eb88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x12012ad4; continue;
 }
 0x12012ad4 => {
  { let v = 0x12035ba4u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12012adf); f_12001e24(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012ba9; continue; }
  pc = 0x12012aea; continue;
 }
 0x12012aea => {
  { let v = 0x12035ba0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12012af5); f_12001e24(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012ba9; continue; }
  pc = 0x12012b00; continue;
 }
 0x12012b00 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x12012b37; continue; }
  pc = 0x12012b06; continue;
 }
 0x12012b06 => {
  { cpu.sub8((cpu.eax as u8), 0x3au8); }
  if cpu.zf { pc = 0x12012b37; continue; }
  pc = 0x12012b0a; continue;
 }
 0x12012b0a => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if cpu.zf { pc = 0x12012b37; continue; }
  pc = 0x12012b0e; continue;
 }
 0x12012b0e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12012b14); f_12003cb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x12012b2a; continue; }
  pc = 0x12012b1d; continue;
 }
 0x12012b1d => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12012b23); f_12012c0a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012ba5; continue; }
  pc = 0x12012b2a; continue;
 }
 0x12012b2a => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012b37 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012b53); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012be5; continue; }
  pc = 0x12012b5e; continue;
 }
 0x12012b5e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012b6c); f_12013929(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
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
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12012b9e); f_12012c0a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12012be5; continue; }
  pc = 0x12012ba5; continue;
 }
 0x12012ba5 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x12012ba9; continue;
 }
 0x12012ba9 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x12012bb0); f_12012c91(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x12012ad4; continue; }
  pc = 0x12012bbd; continue;
 }
 0x12012bbd => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12012bf9; continue; }
  pc = 0x12012bc4; continue;
 }
 0x12012bc4 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12012bf2; continue; }
  pc = 0x12012bcb; continue;
 }
 0x12012bcb => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x12012bd2; continue;
 }
 0x12012bd2 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012bd8); f_12012c52(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012be5 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012bf2 => {
  { let v: u32 = cpu.r32(0x1203eb7cu32); cpu.eax = v; }
  pc = 0x12012bd2; continue;
 }
 0x12012bf9 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12012bff); f_12012c0a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012c0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012c0a); }
 let mut pc: u32 = 0x12012c0a;
 loop { match pc {
 0x12012c0a => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12012c11); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12012c1e; continue; }
  pc = 0x12012c18; continue;
 }
 0x12012c18 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012c1e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.r32(0x1203eb7cu32), 0x0u32); }
  if cpu.zf { pc = 0x12012c45; continue; }
  pc = 0x12012c34; continue;
 }
 0x12012c34 => {
  { let v: u32 = cpu.r32(0x1203eb88u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203eb88u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012c45 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203eb7cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203eb88u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012c52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012c52); }
 let mut pc: u32 = 0x12012c52;
 loop { match pc {
 0x12012c52 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12012c8e; continue; }
  pc = 0x12012c5c; continue;
 }
 0x12012c5c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12012c8e; continue; }
  pc = 0x12012c63; continue;
 }
 0x12012c63 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12012c6e); f_12013a4c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x12012c7d; continue; }
  pc = 0x12012c75; continue;
 }
 0x12012c75 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x12012c7d; continue;
 }
 0x12012c7d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12012c63; continue; }
  pc = 0x12012c84; continue;
 }
 0x12012c84 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12012c63; continue; }
  pc = 0x12012c8e; continue;
 }
 0x12012c8e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012c91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012c91); }
 let mut pc: u32 = 0x12012c91;
 loop { match pc {
 0x12012c91 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12012cea; continue; }
  pc = 0x12012c9a; continue;
 }
 0x12012c9a => {
  { cpu.sub32(cpu.r32(0x1203eb80u32), 0x0u32); }
  if !cpu.zf { pc = 0x12012cb5; continue; }
  pc = 0x12012ca3; continue;
 }
 0x12012ca3 => {
  { let v = 0x244u32; cpu.push(v); }
  cpu.push(0x12012cad); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203eb80u32, v); }
  pc = 0x12012cb5; continue;
 }
 0x12012cb5 => {
  { let v: u32 = cpu.r32(0x1203eb84u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012ccf; continue; }
  pc = 0x12012cbe; continue;
 }
 0x12012cbe => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x12012cc5); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0x1203eb84u32, v); }
  pc = 0x12012ccf; continue;
 }
 0x12012ccf => {
  { let v: u32 = cpu.r32(0x1203eb80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041198u32); cpu.push(0x12012cdc); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203eb84u32, v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12012d1b; continue; }
  pc = 0x12012ce6; continue;
 }
 0x12012ce6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012cea => {
  { let v: u32 = cpu.r32(0x1203eb80u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203eb84u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1204119cu32); cpu.push(0x12012cfd); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12012d1b; continue; }
  pc = 0x12012d01; continue;
 }
 0x12012d01 => {
  { let v: u32 = cpu.r32(0x1203eb84u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x12012d0d); cpu.call(t);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1203eb84u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012d1b => {
  { let v: u32 = cpu.r32(0x1203eb80u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2cu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012d25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012d25); }
 let mut pc: u32 = 0x12012d25;
 loop { match pc {
 0x12012d25 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  { let v: u32 = 0x1203eb90u32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041208u32); cpu.push(0x12012d3d); cpu.call(t);
  { let v: u32 = cpu.r32(0x1203b1e0u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1203b134u32, v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x12012d54; continue; }
  pc = 0x12012d4e; continue;
 }
 0x12012d4e => {
  { let v: u32 = cpu.r32(0x1203b1e0u32); cpu.esi = v; }
  pc = 0x12012d54; continue;
 }
 0x12012d54 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12012d68); f_12012dc2(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012d7d); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12012d90; continue; }
  pc = 0x12012d86; continue;
 }
 0x12012d86 => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12012d8d); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12012d90; continue;
 }
 0x12012d90 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12012da9); f_12012dc2(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1203b11cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b114u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012dc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012dc2); }
 let mut pc: u32 = 0x12012dc2;
 loop { match pc {
 0x12012dc2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x12012df4; continue; }
  pc = 0x12012de9; continue;
 }
 0x12012de9 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12012df4; continue;
 }
 0x12012df4 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if cpu.zf { pc = 0x12012e55; continue; }
  pc = 0x12012dfa; continue;
 }
 0x12012dfa => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012e0a; continue; }
  pc = 0x12012e00; continue;
 }
 0x12012e00 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12012e0a; continue;
 }
 0x12012e0a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if cpu.zf { pc = 0x12012e21; continue; }
  pc = 0x12012e16; continue;
 }
 0x12012e16 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x12012e26; continue; }
  pc = 0x12012e1b; continue;
 }
 0x12012e1b => {
  { cpu.sub16((cpu.eax as u16), 0x9u16); }
  if !cpu.zf { pc = 0x12012dfa; continue; }
  pc = 0x12012e21; continue;
 }
 0x12012e21 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x12012e49; continue; }
  pc = 0x12012e26; continue;
 }
 0x12012e26 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x2u32); cpu.esi = v; }
  pc = 0x12012e29; continue;
 }
 0x12012e29 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x12012e2b; continue;
 }
 0x12012e2b => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x12012f59; continue; }
  pc = 0x12012e35; continue;
 }
 0x12012e35 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if cpu.zf { pc = 0x12012e44; continue; }
  pc = 0x12012e3e; continue;
 }
 0x12012e3e => {
  { cpu.sub16((cpu.eax as u16), 0x9u16); }
  if !cpu.zf { pc = 0x12012e96; continue; }
  pc = 0x12012e44; continue;
 }
 0x12012e44 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x12012e35; continue;
 }
 0x12012e49 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012e29; continue; }
  pc = 0x12012e4d; continue;
 }
 0x12012e4d => {
  { let v: u16 = 0x0u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12012e29; continue;
 }
 0x12012e55 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if cpu.zf { pc = 0x12012e7d; continue; }
  pc = 0x12012e5e; continue;
 }
 0x12012e5e => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x12012e7d; continue; }
  pc = 0x12012e64; continue;
 }
 0x12012e64 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012e74; continue; }
  pc = 0x12012e6a; continue;
 }
 0x12012e6a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x12012e74; continue;
 }
 0x12012e74 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if !cpu.zf { pc = 0x12012e5e; continue; }
  pc = 0x12012e7d; continue;
 }
 0x12012e7d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012e8b; continue; }
  pc = 0x12012e83; continue;
 }
 0x12012e83 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x12012e8b; continue;
 }
 0x12012e8b => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if !cpu.zf { pc = 0x12012e29; continue; }
  pc = 0x12012e91; continue;
 }
 0x12012e91 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x12012e29; continue;
 }
 0x12012e96 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x12012f59; continue; }
  pc = 0x12012ea0; continue;
 }
 0x12012ea0 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12012eb2; continue; }
  pc = 0x12012ea7; continue;
 }
 0x12012ea7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12012eb2; continue;
 }
 0x12012eb2 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12012eb8; continue;
 }
 0x12012eb8 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x5cu16); }
  if !cpu.zf { pc = 0x12012ecf; continue; }
  pc = 0x12012ec5; continue;
 }
 0x12012ec5 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x5cu16); }
  if cpu.zf { pc = 0x12012ec5; continue; }
  pc = 0x12012ecf; continue;
 }
 0x12012ecf => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x22u16); }
  if !cpu.zf { pc = 0x12012ef6; continue; }
  pc = 0x12012ed5; continue;
 }
 0x12012ed5 => {
  { cpu.and32(cpu.ebp, 0x1u32); }
  if !cpu.zf { pc = 0x12012ef3; continue; }
  pc = 0x12012edd; continue;
 }
 0x12012edd => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12012f42; continue; }
  pc = 0x12012ee1; continue;
 }
 0x12012ee1 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x22u16); }
  if !cpu.zf { pc = 0x12012f42; continue; }
  pc = 0x12012eea; continue;
 }
 0x12012eea => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x12012eec; continue;
 }
 0x12012eec => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x12012ef3; continue;
 }
 0x12012ef3 => {
  { let v: u32 = cpu.shr32(cpu.ebp, 1u32); cpu.ebp = v; }
  pc = 0x12012ef6; continue;
 }
 0x12012ef6 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012f12; continue; }
  pc = 0x12012efd; continue;
 }
 0x12012efd => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012f09; continue; }
  pc = 0x12012f01; continue;
 }
 0x12012f01 => {
  { let v: u16 = 0x5cu16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x12012f09; continue;
 }
 0x12012f09 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12012efd; continue; }
  pc = 0x12012f12; continue;
 }
 0x12012f12 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x12012f46; continue; }
  pc = 0x12012f1a; continue;
 }
 0x12012f1a => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12012f2a; continue; }
  pc = 0x12012f1e; continue;
 }
 0x12012f1e => {
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if cpu.zf { pc = 0x12012f46; continue; }
  pc = 0x12012f24; continue;
 }
 0x12012f24 => {
  { cpu.sub16((cpu.eax as u16), 0x9u16); }
  if cpu.zf { pc = 0x12012f46; continue; }
  pc = 0x12012f2a; continue;
 }
 0x12012f2a => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12012f3a; continue; }
  pc = 0x12012f2e; continue;
 }
 0x12012f2e => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012f38; continue; }
  pc = 0x12012f32; continue;
 }
 0x12012f32 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x12012f38; continue;
 }
 0x12012f38 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x12012f3a; continue;
 }
 0x12012f3a => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x12012eb8; continue;
 }
 0x12012f42 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x12012eec; continue;
 }
 0x12012f46 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12012f52; continue; }
  pc = 0x12012f4a; continue;
 }
 0x12012f4a => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x12012f52; continue;
 }
 0x12012f52 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x12012e2b; continue;
 }
 0x12012f59 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12012f6a; continue; }
  pc = 0x12012f60; continue;
 }
 0x12012f60 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12012f6a; continue;
 }
 0x12012f6a => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12012f75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12012f75); }
 let mut pc: u32 = 0x12012f75;
 loop { match pc {
 0x12012f75 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203b11cu32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203eda4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203ed98u32, v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.zf { pc = 0x12012fed; continue; }
  pc = 0x12012f8c; continue;
 }
 0x12012f8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ecx)), 0x22u16); }
  if !cpu.zf { pc = 0x12012fad; continue; }
  pc = 0x12012f99; continue;
 }
 0x12012f99 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012f9f); f_12013226(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012fe5; continue; }
  pc = 0x12012fa6; continue;
 }
 0x12012fa6 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012fad => {
  { let v = 0x12035ba8u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012fb8); f_120154e6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012fd6; continue; }
  pc = 0x12012fbf; continue;
 }
 0x12012fbf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012fc8); f_1201307c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12012fe5; continue; }
  pc = 0x12012fcf; continue;
 }
 0x12012fcf => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12012fd6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12012fde); f_12013226(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201301b; continue; }
  pc = 0x12012fe5; continue;
 }
 0x12012fe5 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x12012f8c; continue; }
  pc = 0x12012fed; continue;
 }
 0x12012fed => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1203ed98u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x12013000; continue; }
  pc = 0x12012ff8; continue;
 }
 0x12012ff8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12012ff8; continue; }
  pc = 0x12013000; continue;
 }
 0x12013000 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201300d); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12013022; continue; }
  pc = 0x12013014; continue;
 }
 0x12013014 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201301b => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013022 => {
  { let v: u32 = cpu.r32(0x1203ed98u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b11cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1203b114u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12013045; continue; }
  pc = 0x12013037; continue;
 }
 0x12013037 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x12013037; continue; }
  pc = 0x12013045; continue;
 }
 0x12013045 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1203ed98u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12013078; continue; }
  pc = 0x12013055; continue;
 }
 0x12013055 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x12013057; continue;
 }
 0x12013057 => {
  { let v: u32 = cpu.r32(0x1203ed98u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1203ed98u32, v); }
  cpu.push(0x1201306b); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203ed98u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { pc = 0x12013057; continue; }
  pc = 0x12013078; continue;
 }
 0x12013078 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201307c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201307c); }
 let mut pc: u32 = 0x1201307c;
 loop { match pc {
 0x1201307c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.zf { pc = 0x120130b1; continue; }
  pc = 0x12013095; continue;
 }
 0x12013095 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x120130b1; continue; }
  pc = 0x1201309e; continue;
 }
 0x1201309e => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if cpu.zf { pc = 0x120130b1; continue; }
  pc = 0x120130a4; continue;
 }
 0x120130a4 => {
  { cpu.sub16((cpu.eax as u16), 0x3au16); }
  if cpu.zf { pc = 0x120130b1; continue; }
  pc = 0x120130aa; continue;
 }
 0x120130aa => {
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if !cpu.zf { pc = 0x12013095; continue; }
  pc = 0x120130b1; continue;
 }
 0x120130b1 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3au16); }
  if !cpu.zf { pc = 0x120130d4; continue; }
  pc = 0x120130ba; continue;
 }
 0x120130ba => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xfffffffeu32); }
  if cpu.zf { pc = 0x120130d4; continue; }
  pc = 0x120130c3; continue;
 }
 0x120130c3 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120130c9); f_12013226(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120130d4 => {
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x120130e6; continue; }
  pc = 0x120130da; continue;
 }
 0x120130da => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if cpu.zf { pc = 0x120130e6; continue; }
  pc = 0x120130e0; continue;
 }
 0x120130e0 => {
  { cpu.sub16((cpu.eax as u16), 0x3au16); }
  if !cpu.zf { pc = 0x120130ee; continue; }
  pc = 0x120130e6; continue;
 }
 0x120130e6 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x120130ee; continue;
 }
 0x120130ee => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120130f4); f_120132ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x12013215; continue; }
  pc = 0x12013101; continue;
 }
 0x12013101 => {
  { let v: u32 = cpu.r32(0x1203eda4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201310a; continue;
 }
 0x1201310a => {
  { let v = 0x12035bb8u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12013115); f_1201559d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120131c5; continue; }
  pc = 0x12013120; continue;
 }
 0x12013120 => {
  { let v = 0x12035bb0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201312b); f_1201559d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120131c5; continue; }
  pc = 0x12013136; continue;
 }
 0x12013136 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x12013174; continue; }
  pc = 0x1201313f; continue;
 }
 0x1201313f => {
  { cpu.sub16((cpu.eax as u16), 0x3au16); }
  if cpu.zf { pc = 0x12013174; continue; }
  pc = 0x12013145; continue;
 }
 0x12013145 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if cpu.zf { pc = 0x12013174; continue; }
  pc = 0x1201314b; continue;
 }
 0x1201314b => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12013151); f_12015563(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12013167; continue; }
  pc = 0x1201315a; continue;
 }
 0x1201315a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12013160); f_12013226(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120131c1; continue; }
  pc = 0x12013167; continue;
 }
 0x12013167 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013174 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201317a); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201318c); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12013201; continue; }
  pc = 0x12013193; continue;
 }
 0x12013193 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201319c); f_1201551f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120131a8); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120131ba); f_12013226(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12013201; continue; }
  pc = 0x120131c1; continue;
 }
 0x120131c1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x120131c5; continue;
 }
 0x120131c5 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x120131cc); f_120132ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x1201310a; continue; }
  pc = 0x120131d9; continue;
 }
 0x120131d9 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12013215; continue; }
  pc = 0x120131e0; continue;
 }
 0x120131e0 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201320e; continue; }
  pc = 0x120131e7; continue;
 }
 0x120131e7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x120131ee; continue;
 }
 0x120131ee => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120131f4); f_1201326e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013201 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201320e => {
  { let v: u32 = cpu.r32(0x1203ed98u32); cpu.eax = v; }
  pc = 0x120131ee; continue;
 }
 0x12013215 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201321b); f_12013226(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013226(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013226); }
 let mut pc: u32 = 0x12013226;
 loop { match pc {
 0x12013226 => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1201322d); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201323a; continue; }
  pc = 0x12013234; continue;
 }
 0x12013234 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201323a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.r32(0x1203ed98u32), 0x0u32); }
  if cpu.zf { pc = 0x12013261; continue; }
  pc = 0x12013250; continue;
 }
 0x12013250 => {
  { let v: u32 = cpu.r32(0x1203eda4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203eda4u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013261 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203ed98u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203eda4u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201326e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201326e); }
 let mut pc: u32 = 0x1201326e;
 loop { match pc {
 0x1201326e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x120132aa; continue; }
  pc = 0x12013278; continue;
 }
 0x12013278 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x120132aa; continue; }
  pc = 0x1201327f; continue;
 }
 0x1201327f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201328a); f_120155e2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x12013299; continue; }
  pc = 0x12013291; continue;
 }
 0x12013291 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x12013299; continue;
 }
 0x12013299 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201327f; continue; }
  pc = 0x120132a0; continue;
 }
 0x120132a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201327f; continue; }
  pc = 0x120132aa; continue;
 }
 0x120132aa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120132ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120132ad); }
 let mut pc: u32 = 0x120132ad;
 loop { match pc {
 0x120132ad => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12013306; continue; }
  pc = 0x120132b6; continue;
 }
 0x120132b6 => {
  { cpu.sub32(cpu.r32(0x1203ed9cu32), 0x0u32); }
  if !cpu.zf { pc = 0x120132d1; continue; }
  pc = 0x120132bf; continue;
 }
 0x120132bf => {
  { let v = 0x354u32; cpu.push(v); }
  cpu.push(0x120132c9); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203ed9cu32, v); }
  pc = 0x120132d1; continue;
 }
 0x120132d1 => {
  { let v: u32 = cpu.r32(0x1203eda0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120132eb; continue; }
  pc = 0x120132da; continue;
 }
 0x120132da => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x120132e1); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0x1203eda0u32, v); }
  pc = 0x120132eb; continue;
 }
 0x120132eb => {
  { let v: u32 = cpu.r32(0x1203ed9cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041190u32); cpu.push(0x120132f8); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203eda0u32, v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12013337; continue; }
  pc = 0x12013302; continue;
 }
 0x12013302 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013306 => {
  { let v: u32 = cpu.r32(0x1203ed9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203eda0u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041194u32); cpu.push(0x12013319); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12013337; continue; }
  pc = 0x1201331d; continue;
 }
 0x1201331d => {
  { let v: u32 = cpu.r32(0x1203eda0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x12013329); cpu.call(t);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1203eda0u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013337 => {
  { let v: u32 = cpu.r32(0x1203ed9cu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2cu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013341(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013341); }
 let mut pc: u32 = 0x12013341;
 loop { match pc {
 0x12013341 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12013347); f_120133bd(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1203b2c8u32; cpu.eax = v; }
  pc = 0x12013354; continue;
 }
 0x12013354 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if cpu.zf { pc = 0x1201337a; continue; }
  pc = 0x12013358; continue;
 }
 0x12013358 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x1203b430u32); }
  if cpu.cf { pc = 0x12013354; continue; }
  pc = 0x12013363; continue;
 }
 0x12013363 => {
  { cpu.sub32(cpu.ecx, 0x13u32); }
  if cpu.cf { pc = 0x1201338a; continue; }
  pc = 0x12013368; continue;
 }
 0x12013368 => {
  { cpu.sub32(cpu.ecx, 0x24u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201338a; continue; }
  pc = 0x1201336d; continue;
 }
 0x1201336d => {
  cpu.push(0x12013372); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201337a => {
  cpu.push(0x1201337f); f_120133b4(cpu);
  { let v: u32 = cpu.r32(0x1203b2ccu32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201338a => {
  { cpu.sub32(cpu.ecx, 0xbcu32); }
  if cpu.cf { pc = 0x120133a7; continue; }
  pc = 0x12013392; continue;
 }
 0x12013392 => {
  { cpu.sub32(cpu.ecx, 0xcau32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120133a7; continue; }
  pc = 0x1201339a; continue;
 }
 0x1201339a => {
  cpu.push(0x1201339f); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120133a7 => {
  cpu.push(0x120133ac); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120133b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120133b4); }
 let mut pc: u32 = 0x120133b4;
 loop { match pc {
 0x120133b4 => {
  cpu.push(0x120133b9); f_12003ac7(cpu);
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120133bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120133bd); }
 let mut pc: u32 = 0x120133bd;
 loop { match pc {
 0x120133bd => {
  cpu.push(0x120133c2); f_12003ac7(cpu);
  { let v: u32 = cpu.add32(cpu.eax, 0xcu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120133c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120133c6); }
 let mut pc: u32 = 0x120133c6;
 loop { match pc {
 0x120133c6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x12035bc4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120133d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120133d7); }
 let mut pc: u32 = 0x120133d7;
 loop { match pc {
 0x120133d7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x12035bc4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120133f9); f_12013642(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x12013426; continue; }
  pc = 0x12013405; continue;
 }
 0x12013405 => {
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
  pc = 0x12013426; continue;
 }
 0x12013426 => {
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
pub fn f_12013435(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013435); }
 let mut pc: u32 = 0x12013435;
 loop { match pc {
 0x12013435 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x12035bc4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12013498; continue; }
  pc = 0x1201344e; continue;
 }
 0x1201344e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12013462); f_12013642(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1201349e; continue; }
  pc = 0x1201346e; continue;
 }
 0x1201346e => {
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
 0x12013498 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201349e; continue;
 }
 0x1201349e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120134a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120134a6); }
 let mut pc: u32 = 0x120134a6;
 loop { match pc {
 0x120134a6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.zf { pc = 0x120134bf; continue; }
  pc = 0x120134b2; continue;
 }
 0x120134b2 => {
  cpu.push(0x120134b7); f_12011575(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.push(0x120134bf); f_12013435(cpu);
  pc = 0x120134bf; continue;
 }
 0x120134bf => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120134c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120134c6); }
 let mut pc: u32 = 0x120134c6;
 loop { match pc {
 0x120134c6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120134d2; continue; }
  pc = 0x120134cd; continue;
 }
 0x120134cd => {
  { let v: u32 = 0x12035bccu32; cpu.eax = v; }
  pc = 0x120134d2; continue;
 }
 0x120134d2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120134d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120134d3); }
 let mut pc: u32 = 0x120134d3;
 loop { match pc {
 0x120134d3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12035be0u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
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
  if cpu.sf { pc = 0x1201351e; continue; }
  pc = 0x1201350d; continue;
 }
 0x1201350d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x12013519); cpu.call(t);
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1201350d; continue; }
  pc = 0x1201351e; continue;
 }
 0x1201351e => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x12013531); f_12013543(cpu);
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
pub fn f_12013543(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013543); }
 let mut pc: u32 = 0x12013543;
 loop { match pc {
 0x12013543 => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201355e; continue; }
  pc = 0x12013549; continue;
 }
 0x12013549 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201355e); f_1201355f(cpu);
  pc = 0x1201355e; continue;
 }
 0x1201355e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201355f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201355f); }
 let mut pc: u32 = 0x1201355f;
 loop { match pc {
 0x1201355f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12035bf0u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
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
  if cpu.sf { pc = 0x120135ab; continue; }
  pc = 0x12013595; continue;
 }
 0x12013595 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x1201359b; continue;
 }
 0x1201359b => {
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x120135a5); cpu.call(t);
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1201359b; continue; }
  pc = 0x120135ab; continue;
 }
 0x120135ab => {
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
pub fn f_120135c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120135c4); }
 let mut pc: u32 = 0x120135c4;
 loop { match pc {
 0x120135c4 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120135cd); f_120135d6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120135d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120135d1); }
 let mut pc: u32 = 0x120135d1;
 loop { match pc {
 0x120135ab => {
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
 0x120135d1 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  pc = 0x120135ab; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120135d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120135d6); }
 let mut pc: u32 = 0x120135d6;
 loop { match pc {
 0x120135d6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0xe06d7363u32); }
  if !cpu.zf { pc = 0x120135e9; continue; }
  pc = 0x120135e4; continue;
 }
 0x120135e4 => {
  cpu.push(0x120135e9); f_12011133(cpu);
  pc = 0x120135e9; continue;
 }
 0x120135e9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120135ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120135ec); }
 let mut pc: u32 = 0x120135ec;
 loop { match pc {
 0x120135ec => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  cpu.push(0x1201360f); f_1201422b(cpu);
  let t = cpu.eax; cpu.push(0x12013611); cpu.call(t);
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { cpu.sub32(cpu.ecx, 0x100u32); }
  if !cpu.zf { pc = 0x12013627; continue; }
  pc = 0x12013622; continue;
 }
 0x12013622 => {
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x12013627; continue;
 }
 0x12013627 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201362d); f_1201422b(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013634(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013634); }
 let mut pc: u32 = 0x12013634;
 loop { match pc {
 0x12013634 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201363e); f_12005adb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013642(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013642); }
 let mut pc: u32 = 0x12013642;
 loop { match pc {
 0x12013642 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201364e); f_1200481f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013652(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013652); }
 let mut pc: u32 = 0x12013652;
 loop { match pc {
 0x12013652 => {
  { let v: u32 = cpu.r32(0x1203bc5cu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013658(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013658); }
 let mut pc: u32 = 0x12013658;
 loop { match pc {
 0x12013658 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffff0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x780u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12013674; continue; }
  pc = 0x12013669; continue;
 }
 0x12013669 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203bc5cu32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013674 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013677(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013677); }
 let mut pc: u32 = 0x12013677;
 loop { match pc {
 0x12013677 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x8000u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204117cu32); cpu.push(0x12013690); cpu.call(t);
  { cpu.sub32(cpu.r32(0x1203bc54u32), cpu.esi); }
  if !cpu.zf { pc = 0x120136a0; continue; }
  pc = 0x12013698; continue;
 }
 0x12013698 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203bc54u32, v); }
  pc = 0x120136a0; continue;
 }
 0x120136a0 => {
  { cpu.sub32(cpu.esi, 0x1203b440u32); }
  if cpu.zf { pc = 0x120136c9; continue; }
  pc = 0x120136a8; continue;
 }
 0x120136a8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1203ef08u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1204118cu32); cpu.push(0x120136c7); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120136c9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1203bc50u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120136d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120136d5); }
 let mut pc: u32 = 0x120136d5;
 loop { match pc {
 0x120136d5 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1203b440u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x120136ec; continue;
 }
 0x120136ec => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x1203bc54u32)); }
  if !cpu.zf { pc = 0x120136fc; continue; }
  pc = 0x120136f8; continue;
 }
 0x120136f8 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x28u32.wrapping_add(cpu.esp))); cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x120136fc; continue;
 }
 0x120136fc => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x810u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12013763; continue; }
  pc = 0x1201370a; continue;
 }
 0x1201370a => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201371a; continue;
 }
 0x1201371a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1201378a; continue; }
  pc = 0x12013734; continue;
 }
 0x12013734 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201374c; continue; }
  pc = 0x1201373b; continue;
 }
 0x1201373b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ecx)), cpu.edx); }
  if !cpu.zf { pc = 0x12013837; continue; }
  pc = 0x1201374c; continue;
 }
 0x1201374c => {
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x12013750; continue;
 }
 0x12013750 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1000u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x1201371a; continue; }
  pc = 0x12013763; continue;
 }
 0x12013763 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x1203b440u32); }
  if !cpu.zf { pc = 0x120136ec; continue; }
  pc = 0x12013778; continue;
 }
 0x12013778 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esp)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.eax & 0xfffffff7u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201378a => {
  { let v: u32 = 0xf8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x12013844; continue; }
  pc = 0x1201379a; continue;
 }
 0x1201379a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if !cpu.zf { pc = 0x12013851; continue; }
  pc = 0x120137a3; continue;
 }
 0x120137a3 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  pc = 0x120137b5; continue;
 }
 0x120137b5 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xfffffff8u32); }
  if !cpu.zf { pc = 0x120137c1; continue; }
  pc = 0x120137c0; continue;
 }
 0x120137c0 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x120137c1; continue;
 }
 0x120137c1 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x120137fc; continue; }
  pc = 0x120137c9; continue;
 }
 0x120137c9 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x24u32.wrapping_add(cpu.esp))); cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x120137cf; continue;
 }
 0x120137cf => {
  { cpu.sub32(cpu.edi, 0xf0u32); }
  if (cpu.sf != cpu.of) { pc = 0x120137b5; continue; }
  pc = 0x120137d7; continue;
 }
 0x120137d7 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x24u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x12013885; continue; }
  pc = 0x120137e7; continue;
 }
 0x120137e7 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x12013750; continue; }
  pc = 0x120137ef; continue;
 }
 0x120137ef => {
  { let v: u32 = 0xfffffff8u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120137fc => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x410u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201385e; continue; }
  pc = 0x1201380c; continue;
 }
 0x1201380c => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x12013817; continue; }
  pc = 0x12013811; continue;
 }
 0x12013811 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201386b; continue; }
  pc = 0x12013816; continue;
 }
 0x12013816 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x12013817; continue;
 }
 0x12013817 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x12013833; continue; }
  pc = 0x12013826; continue;
 }
 0x12013826 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x12013828; continue;
 }
 0x12013828 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1))), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x12013878; continue; }
  pc = 0x1201382e; continue;
 }
 0x1201382e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x12013828; continue; }
  pc = 0x12013833; continue;
 }
 0x12013833 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x120137cf; continue;
 }
 0x12013837 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013844 => {
  { let v: u32 = 0xfffffffeu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013851 => {
  { let v: u32 = 0xfffffffdu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201385e => {
  { let v: u32 = 0xfffffffcu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201386b => {
  { let v: u32 = 0xfffffffbu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013878 => {
  { let v: u32 = 0xfffffffau32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013885 => {
  { let v: u32 = 0xfffffff9u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013892(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013892); }
 let mut pc: u32 = 0x12013892;
 loop { match pc {
 0x12013892 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x3a4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x12u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120138af; continue; }
  pc = 0x120138a0; continue;
 }
 0x120138a0 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x120138e4u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x120138d0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x120138af => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120138b2 => {
  { let v: u32 = 0x411u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120138b8 => {
  { let v: u32 = 0x804u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120138be => {
  { let v: u32 = 0x412u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120138c4 => {
  { let v: u32 = 0x404u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120138d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120138d0); }
 let mut pc: u32 = 0x120138d0;
 loop { match pc {
 0x120138d0 => {
  { let v: u8 = 0x38u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xbe120138u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0x120138u32.wrapping_add(cpu.edi))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_120138f7(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120138e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120138e4); }
 let mut pc: u32 = 0x120138e4;
 loop { match pc {
 0x120138e4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_120138f7(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120138f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120138f7); }
 let mut pc: u32 = 0x120138f7;
 loop { match pc {
 0x120138f7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1203bc88u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203bd98u32, v); }
  { let v: u32 = 0x1203bd98u32; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203bd8cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203bd90u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013923(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013923); }
 let mut pc: u32 = 0x12013923;
 loop { match pc {
 0x12013923 => {
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013929(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013929); }
 let mut pc: u32 = 0x12013929;
 loop { match pc {
 0x12013929 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  if !cpu.zf { pc = 0x12013954; continue; }
  pc = 0x1201393c; continue;
 }
 0x1201393c => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201394c); f_12003cd6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013954 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x1201395b); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1201396b; continue;
 }
 0x1201396b => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12013997; continue; }
  pc = 0x1201396f; continue;
 }
 0x1201396f => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x120139ba; continue; }
  pc = 0x1201397e; continue;
 }
 0x1201397e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x120139c6; continue; }
  pc = 0x12013986; continue;
 }
 0x12013986 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1201396b; continue; }
  pc = 0x12013993; continue;
 }
 0x12013993 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffeu32.wrapping_add(cpu.esi), v); }
  pc = 0x12013997; continue;
 }
 0x12013997 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120139a9; continue; }
  pc = 0x1201399e; continue;
 }
 0x1201399e => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201399e; continue; }
  pc = 0x120139a9; continue;
 }
 0x120139a9 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x120139b0); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120139ba => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x12013997; continue; }
  pc = 0x120139c4; continue;
 }
 0x120139c4 => {
  pc = 0x1201396b; continue;
 }
 0x120139c6 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffffffffu32.wrapping_add(cpu.esi), v); }
  pc = 0x12013997; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120139cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120139cc); }
 let mut pc: u32 = 0x120139cc;
 loop { match pc {
 0x120139cc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120139e1; continue; }
  pc = 0x120139db; continue;
 }
 0x120139db => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120139e1 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x0u32); }
  if !cpu.zf { pc = 0x120139f1; continue; }
  pc = 0x120139ea; continue;
 }
 0x120139ea => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120139f1 => {
  { let v = 0x19u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.ebx = v; }
  cpu.push(0x120139fb); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x12013a1c; continue; }
  pc = 0x12013a0b; continue;
 }
 0x12013a0b => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12013a12); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013a1c => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x12013a21; continue;
 }
 0x12013a21 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if cpu.cf { pc = 0x12013a32; continue; }
  pc = 0x12013a26; continue;
 }
 0x12013a26 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x12013a21; continue; }
  pc = 0x12013a32; continue;
 }
 0x12013a32 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12013a39); f_120040fd(cpu);
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
pub fn f_12013a4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013a4c); }
 let mut pc: u32 = 0x12013a4c;
 loop { match pc {
 0x12013a4c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x12013a72; continue; }
  pc = 0x12013a5a; continue;
 }
 0x12013a5a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12013a69); f_120156c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013a72 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12013a79); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12013a84; continue;
 }
 0x12013a84 => {
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x12013b5b; continue; }
  pc = 0x12013aa1; continue;
 }
 0x12013aa1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x12013af7; continue; }
  pc = 0x12013aa7; continue;
 }
 0x12013aa7 => {
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x12013aae; continue;
 }
 0x12013aae => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x12013bce; continue; }
  pc = 0x12013ac5; continue;
 }
 0x12013ac5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x12013b7e; continue; }
  pc = 0x12013acf; continue;
 }
 0x12013acf => {
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x12013ad2; continue;
 }
 0x12013ad2 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x12013beb; continue; }
  pc = 0x12013add; continue;
 }
 0x12013add => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x12013a84; continue; }
  pc = 0x12013ae5; continue;
 }
 0x12013ae5 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12013aec); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12013af7 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 8u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u8) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.esp), v); }
  { cpu.sub16(cpu.r16(0x1203bd98u32), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12013b2c; continue; }
  pc = 0x12013b16; continue;
 }
 0x12013b16 => {
  { cpu.sub16(cpu.r16(0x1203bd9au32), (cpu.eax as u16)); }
  if cpu.cf { pc = 0x12013b2c; continue; }
  pc = 0x12013b1f; continue;
 }
 0x12013b1f => {
  { let v: u16 = cpu.r16(0x1203bd9cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)).wrapping_add((cpu.eax as u16)); cpu.w16(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x12013aae; continue;
 }
 0x12013b2c => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1203bd9eu32), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12013aae; continue; }
  pc = 0x12013b3e; continue;
 }
 0x12013b3e => {
  { cpu.sub16(cpu.r16(0x1203bda0u32), (cpu.eax as u16)); }
  if cpu.cf { pc = 0x12013aae; continue; }
  pc = 0x12013b4b; continue;
 }
 0x12013b4b => {
  { let v: u16 = cpu.r16(0x1203bda2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)).wrapping_add((cpu.eax as u16)); cpu.w16(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x12013aae; continue;
 }
 0x12013b5b => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esp)), 0x41u16); }
  if cpu.cf { pc = 0x12013aae; continue; }
  pc = 0x12013b67; continue;
 }
 0x12013b67 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esp)), 0x5au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12013aae; continue; }
  pc = 0x12013b73; continue;
 }
 0x12013b73 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esp)).wrapping_add(0x20u16); cpu.w16(0xau32.wrapping_add(cpu.esp), v); }
  pc = 0x12013aae; continue;
 }
 0x12013b7e => {
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 8u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u8) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) | (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1203bd98u32), (cpu.ebx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12013ba8; continue; }
  pc = 0x12013b93; continue;
 }
 0x12013b93 => {
  { cpu.sub16(cpu.r16(0x1203bd9au32), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x12013ba8; continue; }
  pc = 0x12013b9c; continue;
 }
 0x12013b9c => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x1203bd9cu32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x12013ad2; continue;
 }
 0x12013ba8 => {
  { cpu.sub16(cpu.r16(0x1203bd9eu32), (cpu.ebx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12013ad2; continue; }
  pc = 0x12013bb5; continue;
 }
 0x12013bb5 => {
  { cpu.sub16(cpu.r16(0x1203bda0u32), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x12013ad2; continue; }
  pc = 0x12013bc2; continue;
 }
 0x12013bc2 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x1203bda2u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x12013ad2; continue;
 }
 0x12013bce => {
  { cpu.sub16((cpu.ebx as u16), 0x41u16); }
  if cpu.cf { pc = 0x12013ad2; continue; }
  pc = 0x12013bd8; continue;
 }
 0x12013bd8 => {
  { cpu.sub16((cpu.ebx as u16), 0x5au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12013ad2; continue; }
  pc = 0x12013be2; continue;
 }
 0x12013be2 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x12013ad2; continue;
 }
 0x12013beb => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12013bf2); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.esp))); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12013c06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12013c06); }
 let mut pc: u32 = 0x12013c06;
 loop { match pc {
 0x12013c06 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203b1f4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x320u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x120140fd; continue; }
  pc = 0x12013c20; continue;
 }
 0x12013c20 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12013c31; continue; }
  pc = 0x12013c24; continue;
 }
 0x12013c24 => {
  { cpu.sub32(cpu.r32(0x1203b1f8u32), 0x1u32); }
  if cpu.zf { pc = 0x120140fd; continue; }
  pc = 0x12013c31; continue;
 }
 0x12013c31 => {
  { let v: u32 = cpu.r32(0x12035cdcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12035ce0u32); cpu.ebx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x12035ce4u32); cpu.esi = v; }
  { let v: u32 = 0x12035cdcu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1203bed0u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x12035cd0u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0xfffffefcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x1204120cu32); cpu.push(0x12013ce0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12013cf9; continue; }
  pc = 0x12013ce4; continue;
 }
 0x12013ce4 => {
  { let v: u32 = 0x12035af8u32; cpu.esi = v; }
  { let v: u32 = 0xfffffefcu32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  pc = 0x12013cf9; continue;
 }
 0x12013cf9 => {
  { let v: u32 = 0xfffffefcu32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xau32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3cu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12013d3a; continue; }
  pc = 0x12013d14; continue;
 }
 0x12013d14 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = 0x3u32; cpu.push(v); }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203bec8u32); cpu.edx = v; }
  { let v: u32 = 0xfffffecau32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12013d37); f_12003cd6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x12013d3a; continue;
 }
 0x12013d3a => {
  { let v: u32 = cpu.r32(0x1203bec8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1203beccu32); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x12035cc8u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x7u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3cu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12013e0f; continue; }
  pc = 0x12013df2; continue;
 }
 0x12013df2 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = 0x3u32; cpu.push(v); }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffffffcbu32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12013e0c); f_12003cd6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x12013e0f; continue;
 }
 0x12013e0f => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1203beccu32); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x12035cc0u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = 0xau32; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xfffffcdfu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12013ecd); f_120039ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203bed0u32); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x12035cb0u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xb0u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x21cu32); }
  if (cpu.cf || cpu.zf) { pc = 0x120140e5; continue; }
  pc = 0x12013f73; continue;
 }
 0x12013f73 => {
  { let v: u32 = 0x16bu32; cpu.edx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12013f9a); f_1201578b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203bec8u32); cpu.edi = v; }
  pc = 0x12013fa3; continue;
 }
 0x12013fa3 => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1203bed0u32); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x12035c40u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1203bed0u32); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x12035c00u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v = 0x12012u32; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v = 0x12035ab0u32; cpu.push(v); }
  { cpu.movs(1, true); }
  { let v: u32 = 0xfffffce0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120140ba); f_12014324(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x120140d8; continue; }
  pc = 0x120140c4; continue;
 }
 0x120140c4 => {
  { let v = 0x16u32; cpu.push(v); }
  cpu.push(0x120140cb); f_12014a18(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x120140d5); f_120123bf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120140d8; continue;
 }
 0x120140d8 => {
  { cpu.sub32(cpu.esi, 0x4u32); }
  if !cpu.zf { pc = 0x120140ec; continue; }
  pc = 0x120140dd; continue;
 }
 0x120140dd => {
  cpu.unimplemented(0x120140dd, "Int3");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120140e5 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x12013fa3; continue;
 }
 0x120140ec => {
  { cpu.sub32(cpu.esi, 0x5u32); }
  if cpu.zf { pc = 0x1201414c; continue; }
  pc = 0x120140f1; continue;
 }
 0x120140f1 => {
  cpu.push(0x120140f6); f_120111a8(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120140fd => {
  { cpu.and32(cpu.r32(0x1203c224u32), 0x10cu32); }
  if !cpu.zf { pc = 0x1201411c; continue; }
  pc = 0x12014109; continue;
 }
 0x12014109 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1203c218u32; cpu.push(v); }
  cpu.push(0x12014119); f_12014fde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1201411c; continue;
 }
 0x1201411c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1203bea0u32; cpu.push(v); }
  { let v = 0x1203c218u32; cpu.push(v); }
  cpu.push(0x12014137); f_12014f95(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x1203c218u32; cpu.push(v); }
  cpu.push(0x12014144); f_12014f5c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x1201414c); f_120111a8(cpu);
  pc = 0x1201414c; continue;
 }
 0x1201414c => {
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
pub fn f_12014153(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014153); }
 let mut pc: u32 = 0x12014153;
 loop { match pc {
 0x12014153 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x1201415a); f_12012684(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201415e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201415e); }
 let mut pc: u32 = 0x1201415e;
 loop { match pc {
 0x1201415e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x12014176u32; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x12014176); f_12015c45(cpu);
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
pub fn f_1201417e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201417e); }
 let mut pc: u32 = 0x1201417e;
 loop { match pc {
 0x1201417e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x6u32); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if cpu.zf { pc = 0x1201419f; continue; }
  pc = 0x12014190; continue;
 }
 0x12014190 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x1201419f; continue;
 }
 0x1201419f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120141a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120141a0); }
 let mut pc: u32 = 0x120141a0;
 loop { match pc {
 0x120141a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xfffffffeu32; cpu.push(v); }
  { let v = 0x1201417eu32; cpu.push(v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  pc = 0x120141bd; continue;
 }
 0x120141bd => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x120141fa; continue; }
  pc = 0x120141cc; continue;
 }
 0x120141cc => {
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x120141fa; continue; }
  pc = 0x120141d2; continue;
 }
 0x120141d2 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))), 0x0u32); }
  if !cpu.zf { pc = 0x120141bd; continue; }
  pc = 0x120141e6; continue;
 }
 0x120141e6 => {
  { let v = 0x101u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x120141f4); f_12014240(cpu);
  let t = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.push(0x120141f8); cpu.call(t);
  pc = 0x120141bd; continue;
 }
 0x120141fa => {
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014208(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014208); }
 let mut pc: u32 = 0x12014208;
 loop { match pc {
 0x12014208 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x1201417eu32); }
  if !cpu.zf { pc = 0x1201422a; continue; }
  pc = 0x1201421a; continue;
 }
 0x1201421a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if !cpu.zf { pc = 0x1201422a; continue; }
  pc = 0x12014225; continue;
 }
 0x12014225 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1201422a; continue;
 }
 0x1201422a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201422b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201422b); }
 let mut pc: u32 = 0x1201422b;
 loop { match pc {
 0x1201422b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1203bed4u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014240); }
 let mut pc: u32 = 0x12014240;
 loop { match pc {
 0x12014232 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x12014240 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1203bed4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x12014232; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201424c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201424c); }
 let mut pc: u32 = 0x1201424c;
 loop { match pc {
 0x1201424c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.df = false; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x6u32); }
  if !cpu.zf { pc = 0x120142f2; continue; }
  pc = 0x1201426a; continue;
 }
 0x1201426a => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  pc = 0x1201427f; continue;
 }
 0x1201427f => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x120142eb; continue; }
  pc = 0x12014284; continue;
 }
 0x12014284 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x120142d3; continue; }
  pc = 0x1201428e; continue;
 }
 0x1201428e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.ebp = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x12014297); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x120142d3; continue; }
  pc = 0x120142a0; continue;
 }
 0x120142a0 => {
  if cpu.sf { pc = 0x120142de; continue; }
  pc = 0x120142a2; continue;
 }
 0x120142a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x120142ab); f_1201415e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x120142b8); f_120141a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x120142c9); f_12014240(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x120142d3); cpu.call(t);
  pc = 0x120142d3; continue;
 }
 0x120142d3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x1201427f; continue;
 }
 0x120142de => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x120142e3; continue;
 }
 0x120142e3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120142eb => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x120142e3; continue;
 }
 0x120142f2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x120142fe); f_120141a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x120142e3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014309(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014309); }
 let mut pc: u32 = 0x12014309;
 loop { match pc {
 0x12014309 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201431d); f_120141a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014324(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014324); }
 let mut pc: u32 = 0x12014324;
 loop { match pc {
 0x12014324 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1203bee4u32), cpu.esi); }
  if !cpu.zf { pc = 0x12014373; continue; }
  pc = 0x12014331; continue;
 }
 0x12014331 => {
  { let v = 0x12035d20u32; cpu.push(v); }
  let t = cpu.r32(0x1204115cu32); cpu.push(0x1201433c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x120143b0; continue; }
  pc = 0x12014342; continue;
 }
 0x12014342 => {
  { let v = 0x12035d14u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x120411ecu32); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x12014350); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203bee4u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120143b0; continue; }
  pc = 0x12014359; continue;
 }
 0x12014359 => {
  { let v = 0x12035d04u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x12014361); cpu.call(t);
  { let v = 0x12035cf0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203bee8u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x1201436e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203beecu32, v); }
  pc = 0x12014373; continue;
 }
 0x12014373 => {
  { let v: u32 = cpu.r32(0x1203bee8u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12014380; continue; }
  pc = 0x1201437c; continue;
 }
 0x1201437c => {
  let t = cpu.eax; cpu.push(0x1201437e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x12014380; continue;
 }
 0x12014380 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12014396; continue; }
  pc = 0x12014384; continue;
 }
 0x12014384 => {
  { cpu.sub32(cpu.r32(0x1203beecu32), 0x0u32); }
  if cpu.zf { pc = 0x12014396; continue; }
  pc = 0x1201438d; continue;
 }
 0x1201438d => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x1203beecu32); cpu.push(0x12014394); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x12014396; continue;
 }
 0x12014396 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x1203bee4u32); cpu.push(0x120143ac); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120143b0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120143b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120143b6); }
 let mut pc: u32 = 0x120143b6;
 loop { match pc {
 0x120143b6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203b120u32); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x1201441a; continue; }
  pc = 0x120143c5; continue;
 }
 0x120143c5 => {
  { let v: u32 = cpu.r32(0x12041158u32); cpu.edi = v; }
  pc = 0x120143cb; continue;
 }
 0x120143cb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x120143da); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12014421; continue; }
  pc = 0x120143e0; continue;
 }
 0x120143e0 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120143ed); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201442b; continue; }
  pc = 0x120143f6; continue;
 }
 0x120143f6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x12014403); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12014435; continue; }
  pc = 0x12014407; continue;
 }
 0x12014407 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12014412); f_12014c48(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x120143cb; continue; }
  pc = 0x1201441a; continue;
 }
 0x1201441a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014421 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201442b => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014435 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201443f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201443f); }
 let mut pc: u32 = 0x1201443f;
 loop { match pc {
 0x1201443f => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041144u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x1203bef4u32), cpu.ebx); }
  if !cpu.zf { pc = 0x1201446c; continue; }
  pc = 0x1201445a; continue;
 }
 0x1201445a => {
  let t = cpu.esi; cpu.push(0x1201445c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201448f; continue; }
  pc = 0x12014462; continue;
 }
 0x12014462 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203bef4u32, v); }
  pc = 0x1201446c; continue;
 }
 0x1201446c => {
  { cpu.sub32(cpu.r32(0x1203bef4u32), 0x1u32); }
  if !cpu.zf { pc = 0x1201451b; continue; }
  pc = 0x12014479; continue;
 }
 0x12014479 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x120144b1; continue; }
  pc = 0x1201447d; continue;
 }
 0x1201447d => {
  let t = cpu.esi; cpu.push(0x1201447f); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x120144b1; continue; }
  pc = 0x12014485; continue;
 }
 0x12014485 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201448f => {
  let t = cpu.r32(0x1204114cu32); cpu.push(0x12014495); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x120144a7; continue; }
  pc = 0x1201449b; continue;
 }
 0x1201449b => {
  { let v: u32 = 0x2u32; cpu.w32(0x1203bef4u32, v); }
  pc = 0x1201446c; continue;
 }
 0x120144a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120144b1 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0x0u16); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  if cpu.zf { pc = 0x120144cd; continue; }
  pc = 0x120144b9; continue;
 }
 0x120144b9 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x120144b9; continue; }
  pc = 0x120144c3; continue;
 }
 0x120144c3 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x120144b9; continue; }
  pc = 0x120144cd; continue;
 }
 0x120144cd => {
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x120144d8); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120144f4; continue; }
  pc = 0x120144e3; continue;
 }
 0x120144e3 => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x12041148u32); cpu.push(0x120144ea); cpu.call(t);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120144f4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  let t = cpu.r32(0x12041148u32); cpu.push(0x1201450f); cpu.call(t);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201451b => {
  { cpu.sub32(cpu.r32(0x1203bef4u32), 0x2u32); }
  if !cpu.zf { pc = 0x12014614; continue; }
  pc = 0x12014528; continue;
 }
 0x12014528 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x12014542; continue; }
  pc = 0x1201452c; continue;
 }
 0x1201452c => {
  let t = cpu.r32(0x1204114cu32); cpu.push(0x12014532); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x12014542; continue; }
  pc = 0x12014538; continue;
 }
 0x12014538 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014542 => {
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x1201457a; continue; }
  pc = 0x12014549; continue;
 }
 0x12014549 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x1201455e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120145a6; continue; }
  pc = 0x12014562; continue;
 }
 0x12014562 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)).wrapping_add(cpu.eax); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x12014549; continue; }
  pc = 0x1201457a; continue;
 }
 0x1201457a => {
  { let v: u32 = cpu.inc32(cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201458a); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120145b0; continue; }
  pc = 0x12014595; continue;
 }
 0x12014595 => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x12041150u32); cpu.push(0x1201459c); cpu.call(t);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120145a6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120145b0 => {
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x12014603; continue; }
  pc = 0x120145bb; continue;
 }
 0x120145bb => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x120145de); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201461c; continue; }
  pc = 0x120145e2; continue;
 }
 0x120145e2 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  cpu.push(0x120145f7); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x120145bb; continue; }
  pc = 0x12014603; continue;
 }
 0x12014603 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x12041150u32); cpu.push(0x12014610); cpu.call(t);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x12014614; continue;
 }
 0x12014614 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201461c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12014626); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x12041150u32); cpu.push(0x12014630); cpu.call(t);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201463a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201463a); }
 let mut pc: u32 = 0x1201463a;
 loop { match pc {
 0x1201463a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12014643); f_12003ac7(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12014653); f_12014789(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12014779; continue; }
  pc = 0x1201465e; continue;
 }
 0x1201465e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x12014779; continue; }
  pc = 0x12014669; continue;
 }
 0x12014669 => {
  { cpu.sub32(cpu.edx, 0x5u32); }
  if !cpu.zf { pc = 0x1201467f; continue; }
  pc = 0x1201466e; continue;
 }
 0x1201466e => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201467f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if !cpu.zf { pc = 0x1201468e; continue; }
  pc = 0x12014684; continue;
 }
 0x12014684 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201468e => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x8u32); }
  if !cpu.zf { pc = 0x12014767; continue; }
  pc = 0x120146a2; continue;
 }
 0x120146a2 => {
  { let v: u32 = cpu.r32(0x1203c190u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1203c194u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120146d9; continue; }
  pc = 0x120146b4; continue;
 }
 0x120146b4 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x120146ba; continue;
 }
 0x120146ba => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1203c194u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x1203c190u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x120146ba; continue; }
  pc = 0x120146d9; continue;
 }
 0x120146d9 => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xc000008eu32); }
  if !cpu.zf { pc = 0x12014707; continue; }
  pc = 0x120146e5; continue;
 }
 0x120146e5 => {
  { let v: u32 = 0x83u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x120146ec; continue;
 }
 0x120146ec => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  let t = cpu.edx; cpu.push(0x120146f4); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x120146fa; continue;
 }
 0x120146fa => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014707 => {
  { cpu.sub32(cpu.eax, 0xc0000090u32); }
  if !cpu.zf { pc = 0x12014717; continue; }
  pc = 0x1201470e; continue;
 }
 0x1201470e => {
  { let v: u32 = 0x81u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x120146ec; continue;
 }
 0x12014717 => {
  { cpu.sub32(cpu.eax, 0xc0000091u32); }
  if !cpu.zf { pc = 0x12014727; continue; }
  pc = 0x1201471e; continue;
 }
 0x1201471e => {
  { let v: u32 = 0x84u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x120146ec; continue;
 }
 0x12014727 => {
  { cpu.sub32(cpu.eax, 0xc0000093u32); }
  if !cpu.zf { pc = 0x12014737; continue; }
  pc = 0x1201472e; continue;
 }
 0x1201472e => {
  { let v: u32 = 0x85u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x120146ec; continue;
 }
 0x12014737 => {
  { cpu.sub32(cpu.eax, 0xc000008du32); }
  if !cpu.zf { pc = 0x12014747; continue; }
  pc = 0x1201473e; continue;
 }
 0x1201473e => {
  { let v: u32 = 0x82u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x120146ec; continue;
 }
 0x12014747 => {
  { cpu.sub32(cpu.eax, 0xc000008fu32); }
  if !cpu.zf { pc = 0x12014757; continue; }
  pc = 0x1201474e; continue;
 }
 0x1201474e => {
  { let v: u32 = 0x86u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x120146ec; continue;
 }
 0x12014757 => {
  { cpu.sub32(cpu.eax, 0xc0000092u32); }
  if !cpu.zf { pc = 0x120146ec; continue; }
  pc = 0x1201475e; continue;
 }
 0x1201475e => {
  { let v: u32 = 0x8au32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x120146ec; continue;
 }
 0x12014767 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edx; cpu.push(0x12014774); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x120146fa; continue;
 }
 0x12014779 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1204113cu32); cpu.push(0x12014784); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014789(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014789); }
 let mut pc: u32 = 0x12014789;
 loop { match pc {
 0x12014789 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x12014794; continue;
 }
 0x12014794 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.edx); }
  if cpu.zf { pc = 0x120147ab; continue; }
  pc = 0x12014798; continue;
 }
 0x12014798 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1203c19cu32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12014794; continue; }
  pc = 0x120147ab; continue;
 }
 0x120147ab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120147b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120147b8); }
 let mut pc: u32 = 0x120147b8;
 loop { match pc {
 0x120147b8 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4u32); }
  if cpu.zf { pc = 0x12014900; continue; }
  pc = 0x120147cc; continue;
 }
 0x120147cc => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if cpu.zf { pc = 0x12014900; continue; }
  pc = 0x120147d5; continue;
 }
 0x120147d5 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { pc = 0x12014893; continue; }
  pc = 0x120147e2; continue;
 }
 0x120147e2 => {
  { cpu.sub32(cpu.ebx, 0x15u32); }
  if cpu.zf { pc = 0x12014893; continue; }
  pc = 0x120147eb; continue;
 }
 0x120147eb => {
  { cpu.sub32(cpu.ebx, 0x16u32); }
  if cpu.zf { pc = 0x12014893; continue; }
  pc = 0x120147f4; continue;
 }
 0x120147f4 => {
  { cpu.sub32(cpu.ebx, 0xfu32); }
  if cpu.zf { pc = 0x12014893; continue; }
  pc = 0x120147fd; continue;
 }
 0x120147fd => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.zf { pc = 0x12014810; continue; }
  pc = 0x12014802; continue;
 }
 0x12014802 => {
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if cpu.zf { pc = 0x12014810; continue; }
  pc = 0x12014807; continue;
 }
 0x12014807 => {
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if !cpu.zf { pc = 0x12014900; continue; }
  pc = 0x12014810; continue;
 }
 0x12014810 => {
  cpu.push(0x12014815); f_12003ac7(cpu);
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.eax)), 0x1203c118u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x12014859; continue; }
  pc = 0x12014822; continue;
 }
 0x12014822 => {
  { let v: u32 = cpu.r32(0x1203c198u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201482d); f_12004858(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12014900; continue; }
  pc = 0x12014841; continue;
 }
 0x12014841 => {
  { let v: u32 = cpu.r32(0x1203c198u32); cpu.eax = v; }
  { let v: u32 = 0x1203c118u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x12014859; continue;
 }
 0x12014859 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12014867); f_12014c05(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12014900; continue; }
  pc = 0x12014872; continue;
 }
 0x12014872 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), cpu.ebx); }
  if !cpu.zf { pc = 0x1201492e; continue; }
  pc = 0x1201487e; continue;
 }
 0x1201487e => {
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), cpu.ebx); }
  if cpu.zf { pc = 0x1201487e; continue; }
  pc = 0x12014889; continue;
 }
 0x12014889 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014893 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1201489a); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { pc = 0x120148a7; continue; }
  pc = 0x120148a2; continue;
 }
 0x120148a2 => {
  { cpu.sub32(cpu.ebx, 0x15u32); }
  if !cpu.zf { pc = 0x120148cc; continue; }
  pc = 0x120148a7; continue;
 }
 0x120148a7 => {
  { cpu.sub32(cpu.r32(0x1203c1ccu32), 0x0u32); }
  if !cpu.zf { pc = 0x120148cc; continue; }
  pc = 0x120148b0; continue;
 }
 0x120148b0 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12014995u32; cpu.push(v); }
  let t = cpu.r32(0x12041138u32); cpu.push(0x120148bd); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x120148e7; continue; }
  pc = 0x120148c2; continue;
 }
 0x120148c2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203c1ccu32, v); }
  pc = 0x120148cc; continue;
 }
 0x120148cc => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x2u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x14u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12014962; continue; }
  pc = 0x120148d8; continue;
 }
 0x120148d8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12014980u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12014968u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x120148e7 => {
  cpu.push(0x120148ec); f_120133bd(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  let t = cpu.r32(0x120411ccu32); cpu.push(0x120148f4); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x120148fd); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014900; continue;
 }
 0x12014900 => {
  cpu.push(0x12014905); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014918 => {
  { let v: u32 = cpu.r32(0x1203c1bcu32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1203c1bcu32, v); }
  pc = 0x12014924; continue;
 }
 0x12014924 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1201492b); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201492e; continue;
 }
 0x1201492e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014938 => {
  { let v: u32 = cpu.r32(0x1203c1c8u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1203c1c8u32, v); }
  pc = 0x12014924; continue;
 }
 0x12014946 => {
  { let v: u32 = cpu.r32(0x1203c1c0u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1203c1c0u32, v); }
  pc = 0x12014924; continue;
 }
 0x12014954 => {
  { let v: u32 = cpu.r32(0x1203c1c4u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1203c1c4u32, v); }
  pc = 0x12014924; continue;
 }
 0x12014962 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x12014924; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014968(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014968); }
 let mut pc: u32 = 0x12014968;
 loop { match pc {
 0x12014968 => {
  { let v: u8 = cpu.sbb8(cpu.r8(0x1u32.wrapping_add(cpu.ecx)), (cpu.ecx as u8)); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12014978, "Bound");
  { let v: u8 = cpu.adc8((cpu.ecx as u8), ((cpu.eax >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1201497d, "Int3");
  cpu.unimplemented(0x1201497e, "Int3");
  cpu.unimplemented(0x1201497f, "Int3");
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
pub fn f_12014995(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014995); }
 let mut pc: u32 = 0x12014995;
 loop { match pc {
 0x12014995 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
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
pub fn f_12014a18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014a18); }
 let mut pc: u32 = 0x12014a18;
 loop { match pc {
 0x12014a18 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x14u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12014a3c; continue; }
  pc = 0x12014a2d; continue;
 }
 0x12014a2d => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x12014bf0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12014bd0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x12014a3c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014a49 => {
  { let v: u32 = 0x1203c1bcu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203c1bcu32); cpu.esi = v; }
  pc = 0x12014a57; continue;
 }
 0x12014a57 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x12014a80; continue;
 }
 0x12014a62 => {
  cpu.push(0x12014a67); f_12003ac7(cpu);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12014a73); f_12014c05(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x12014a80; continue;
 }
 0x12014a80 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12014a8e; continue; }
  pc = 0x12014a84; continue;
 }
 0x12014a84 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12014a8b); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014a8e; continue;
 }
 0x12014a8e => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x12014ade; continue; }
  pc = 0x12014a93; continue;
 }
 0x12014a93 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12014aa1; continue; }
  pc = 0x12014a97; continue;
 }
 0x12014a97 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12014a9e); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014aa1; continue;
 }
 0x12014aa1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014aab => {
  { let v: u32 = 0x1203c1c8u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203c1c8u32); cpu.esi = v; }
  pc = 0x12014a57; continue;
 }
 0x12014abb => {
  { let v: u32 = 0x1203c1c0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203c1c0u32); cpu.esi = v; }
  pc = 0x12014a57; continue;
 }
 0x12014acb => {
  { let v: u32 = 0x1203c1c4u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203c1c4u32); cpu.esi = v; }
  pc = 0x12014a57; continue;
 }
 0x12014ade => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x12014afa; continue; }
  pc = 0x12014ae2; continue;
 }
 0x12014ae2 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12014af0; continue; }
  pc = 0x12014ae6; continue;
 }
 0x12014ae6 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12014aed); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014af0; continue;
 }
 0x12014af0 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x12014af7); f_120123bf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014afa; continue;
 }
 0x12014afa => {
  { cpu.sub32(cpu.edi, 0x8u32); }
  if cpu.zf { pc = 0x12014b09; continue; }
  pc = 0x12014aff; continue;
 }
 0x12014aff => {
  { cpu.sub32(cpu.edi, 0xbu32); }
  if cpu.zf { pc = 0x12014b09; continue; }
  pc = 0x12014b04; continue;
 }
 0x12014b04 => {
  { cpu.sub32(cpu.edi, 0x4u32); }
  if !cpu.zf { pc = 0x12014b2e; continue; }
  pc = 0x12014b09; continue;
 }
 0x12014b09 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x8u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x12014bb7; continue; }
  pc = 0x12014b20; continue;
 }
 0x12014b20 => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x8cu32; cpu.w32(0x58u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x12014b2e; continue;
 }
 0x12014b2e => {
  { cpu.sub32(cpu.edi, 0x8u32); }
  if !cpu.zf { pc = 0x12014bb7; continue; }
  pc = 0x12014b37; continue;
 }
 0x12014b37 => {
  { let v: u32 = cpu.r32(0x1203c190u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203c194u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12014b6d; continue; }
  pc = 0x12014b48; continue;
 }
 0x12014b48 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  pc = 0x12014b4e; continue;
 }
 0x12014b4e => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1203c194u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x1203c190u32)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12014b4e; continue; }
  pc = 0x12014b6d; continue;
 }
 0x12014b6d => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12014b7b; continue; }
  pc = 0x12014b71; continue;
 }
 0x12014b71 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12014b78); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014b7b; continue;
 }
 0x12014b7b => {
  { cpu.sub32(cpu.edi, 0x8u32); }
  if !cpu.zf { pc = 0x12014bc3; continue; }
  pc = 0x12014b80; continue;
 }
 0x12014b80 => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x12014b88); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12014b8b; continue;
 }
 0x12014b8b => {
  { cpu.sub32(cpu.edi, 0x8u32); }
  if cpu.zf { pc = 0x12014b9a; continue; }
  pc = 0x12014b90; continue;
 }
 0x12014b90 => {
  { cpu.sub32(cpu.edi, 0xbu32); }
  if cpu.zf { pc = 0x12014b9a; continue; }
  pc = 0x12014b95; continue;
 }
 0x12014b95 => {
  { cpu.sub32(cpu.edi, 0x4u32); }
  if !cpu.zf { pc = 0x12014bad; continue; }
  pc = 0x12014b9a; continue;
 }
 0x12014b9a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x8u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x12014bad; continue; }
  pc = 0x12014ba6; continue;
 }
 0x12014ba6 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ebp), v); }
  pc = 0x12014bad; continue;
 }
 0x12014bad => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014bb7 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12014b6d; continue;
 }
 0x12014bc3 => {
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x12014bc6); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014b8b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014bd0); }
 let mut pc: u32 = 0x12014bd0;
 loop { match pc {
 0x12014bd0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12014bd4, "Bound");
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0x4au32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12014bdc, "Bound");
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), cpu.r8(0xbb12014au32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12014be8, "Retf");
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014bf0); }
 let mut pc: u32 = 0x12014bf0;
 loop { match pc {
 0x12014bf0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.eax.wrapping_add(0x24448b06u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)), (cpu.edx as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  cpu.call(0x12014c0b); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014c05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014c05); }
 let mut pc: u32 = 0x12014c05;
 loop { match pc {
 0x12014c05 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x12014c10; continue;
 }
 0x12014c10 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.edx); }
  if cpu.zf { pc = 0x12014c28; continue; }
  pc = 0x12014c15; continue;
 }
 0x12014c15 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1203c19cu32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12014c10; continue; }
  pc = 0x12014c28; continue;
 }
 0x12014c28 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014c36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014c36); }
 let mut pc: u32 = 0x12014c36;
 loop { match pc {
 0x12014c36 => {
  cpu.push(0x12014c3b); f_12003ac7(cpu);
  { let v: u32 = cpu.add32(cpu.eax, 0x58u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014c3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014c3f); }
 let mut pc: u32 = 0x12014c3f;
 loop { match pc {
 0x12014c3f => {
  cpu.push(0x12014c44); f_12003ac7(cpu);
  { let v: u32 = cpu.add32(cpu.eax, 0x54u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014c48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014c48); }
 let mut pc: u32 = 0x12014c48;
 loop { match pc {
 0x12014c48 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12014e81; continue; }
  pc = 0x12014c5b; continue;
 }
 0x12014c5b => {
  { let v = 0x3du32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12014c63); f_120158ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12014e81; continue; }
  pc = 0x12014c70; continue;
 }
 0x12014c70 => {
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if cpu.zf { pc = 0x12014e81; continue; }
  pc = 0x12014c78; continue;
 }
 0x12014c78 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.ebx)), 0x1u16); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203b128u32); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x1203b12cu32), cpu.ecx); }
  if !cpu.zf { pc = 0x12014ca1; continue; }
  pc = 0x12014c93; continue;
 }
 0x12014c93 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12014c99); f_12014eee(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b128u32, v); }
  pc = 0x12014ca1; continue;
 }
 0x12014ca1 => {
  { cpu.sub32(cpu.r32(0x1203b128u32), 0x0u32); }
  if !cpu.zf { pc = 0x12014d51; continue; }
  pc = 0x12014cae; continue;
 }
 0x12014cae => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12014cd8; continue; }
  pc = 0x12014cb5; continue;
 }
 0x12014cb5 => {
  { cpu.sub32(cpu.r32(0x1203b120u32), 0x0u32); }
  if cpu.zf { pc = 0x12014cd8; continue; }
  pc = 0x12014cbe; continue;
 }
 0x12014cbe => {
  cpu.push(0x12014cc3); f_120143b6(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12014d51; continue; }
  pc = 0x12014ccb; continue;
 }
 0x12014ccb => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014cd8 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12014ce9; continue; }
  pc = 0x12014cdf; continue;
 }
 0x12014cdf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014ce9 => {
  { cpu.sub32(cpu.r32(0x1203b120u32), 0x0u32); }
  if !cpu.zf { pc = 0x12014d1d; continue; }
  pc = 0x12014cf2; continue;
 }
 0x12014cf2 => {
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x12014cf9); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b120u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12014d12; continue; }
  pc = 0x12014d05; continue;
 }
 0x12014d05 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014d12 => {
  { let v: u32 = cpu.r32(0x1203b120u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12014d1d; continue;
 }
 0x12014d1d => {
  { cpu.sub32(cpu.r32(0x1203b128u32), 0x0u32); }
  if !cpu.zf { pc = 0x12014d51; continue; }
  pc = 0x12014d26; continue;
 }
 0x12014d26 => {
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x12014d2d); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b128u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12014d46; continue; }
  pc = 0x12014d39; continue;
 }
 0x12014d39 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014d46 => {
  { let v: u32 = cpu.r32(0x1203b128u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12014d51; continue;
 }
 0x12014d51 => {
  { let v: u32 = cpu.r32(0x1203b128u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12014d68); f_12014e8e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x12014e33; continue; }
  pc = 0x12014d75; continue;
 }
 0x12014d75 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x12014e33; continue; }
  pc = 0x12014d82; continue;
 }
 0x12014d82 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12014dca; continue; }
  pc = 0x12014d89; continue;
 }
 0x12014d89 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(4)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12014d95); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x12014dae; continue; }
  pc = 0x12014d9e; continue;
 }
 0x12014d9e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x12014d9e; continue; }
  pc = 0x12014dae; continue;
 }
 0x12014dae => {
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12014dbc); f_12001cbe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12014dd1; continue; }
  pc = 0x12014dc3; continue;
 }
 0x12014dc3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203b128u32, v); }
  pc = 0x12014dd1; continue;
 }
 0x12014dca => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x12014dd1; continue;
 }
 0x12014dd1 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12014e29; continue; }
  pc = 0x12014dd8; continue;
 }
 0x12014dd8 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12014dde); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12014dee); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12014e29; continue; }
  pc = 0x12014df7; continue;
 }
 0x12014df7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfffffffeu32; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014e01); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & cpu.eax; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041134u32); cpu.push(0x12014e20); cpu.call(t);
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014e26); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014e29; continue;
 }
 0x12014e29 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014e33 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x12014e77; continue; }
  pc = 0x12014e3a; continue;
 }
 0x12014e3a => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x12014e40; continue; }
  pc = 0x12014e3e; continue;
 }
 0x12014e3e => {
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x12014e40; continue;
 }
 0x12014e40 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12014e52); f_12001cbe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12014e66; continue; }
  pc = 0x12014e59; continue;
 }
 0x12014e59 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014e66 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12014dc3; continue;
 }
 0x12014e77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014e81 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014e8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014e8e); }
 let mut pc: u32 = 0x12014e8e;
 loop { match pc {
 0x12014e8e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203b128u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if cpu.zf { pc = 0x12014ece; continue; }
  pc = 0x12014e9c; continue;
 }
 0x12014e9c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x12014ea4; continue;
 }
 0x12014ea4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12014eae); f_120158d9(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12014ec6; continue; }
  pc = 0x12014eb5; continue;
 }
 0x12014eb5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3du16); }
  if cpu.zf { pc = 0x12014edf; continue; }
  pc = 0x12014ec1; continue;
 }
 0x12014ec1 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x12014edf; continue; }
  pc = 0x12014ec6; continue;
 }
 0x12014ec6 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x12014ea4; continue; }
  pc = 0x12014ece; continue;
 }
 0x12014ece => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1203b128u32)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014edf => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1203b128u32)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 2u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014eee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014eee); }
 let mut pc: u32 = 0x12014eee;
 loop { match pc {
 0x12014eee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x12014f58; continue; }
  pc = 0x12014efb; continue;
 }
 0x12014efb => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x12014f0e; continue; }
  pc = 0x12014f03; continue;
 }
 0x12014f03 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x12014f03; continue; }
  pc = 0x12014f0e; continue;
 }
 0x12014f0e => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12014f1b); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x12014f30; continue; }
  pc = 0x12014f26; continue;
 }
 0x12014f26 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x12014f2d); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12014f30; continue;
 }
 0x12014f30 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x12014f50; continue; }
  pc = 0x12014f35; continue;
 }
 0x12014f35 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.push(0x12014f45); f_12015563(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x12014f35; continue; }
  pc = 0x12014f50; continue;
 }
 0x12014f50 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12014f58; continue;
 }
 0x12014f58 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014f5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014f5c); }
 let mut pc: u32 = 0x12014f5c;
 loop { match pc {
 0x12014f5c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x12014f73; continue; }
  pc = 0x12014f66; continue;
 }
 0x12014f66 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x12014f6d); f_12003f9f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12014f73 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014f79); f_12012746(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014f82); f_1200409d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014f8d); f_1201277c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014f95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014f95); }
 let mut pc: u32 = 0x12014f95;
 loop { match pc {
 0x12014f95 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014fa2); f_12012746(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014fab); f_1201513d(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014fc0); f_12004e6b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12014fcc); f_120151c9(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12014fd5); f_1201277c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12014fde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12014fde); }
 let mut pc: u32 = 0x12014fde;
 loop { match pc {
 0x12014fde => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if cpu.zf { pc = 0x12015011; continue; }
  pc = 0x12014ff1; continue;
 }
 0x12014ff1 => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if cpu.cf { pc = 0x12015007; continue; }
  pc = 0x12014ff6; continue;
 }
 0x12014ff6 => {
  { cpu.sub32(cpu.edi, 0x7fffffffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12015007; continue; }
  pc = 0x12014ffe; continue;
 }
 0x12014ffe => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12015011; continue; }
  pc = 0x12015002; continue;
 }
 0x12015002 => {
  { cpu.sub32(cpu.esi, 0x40u32); }
  if cpu.zf { pc = 0x12015011; continue; }
  pc = 0x12015007; continue;
 }
 0x12015007 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015011 => {
  { let v: u32 = cpu.edi & 0xfffffffeu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201501e); f_12012746(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12015027); f_12004057(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12015030); f_12015226(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffc2f3u32; cpu.ecx = v; }
  { cpu.and32(cpu.esi, 0x4u32); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x12015067; continue; }
  pc = 0x12015047; continue;
 }
 0x12015047 => {
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x12015055; continue;
 }
 0x12015055 => {
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x12015081; continue;
 }
 0x12015067 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201509a; continue; }
  pc = 0x1201506f; continue;
 }
 0x1201506f => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12015075); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12015091; continue; }
  pc = 0x1201507c; continue;
 }
 0x1201507c => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  pc = 0x12015081; continue;
 }
 0x12015081 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12015087); f_1201277c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015091 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)) | 0x408u32; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x12015055; continue;
 }
 0x1201509a => {
  { let v: u32 = cpu.ecx | 0x500u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x12015055; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120150a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120150a5); }
 let mut pc: u32 = 0x120150a5;
 loop { match pc {
 0x120150a5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  cpu.push(0x120150b7); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203c458u32), cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201512c; continue; }
  pc = 0x120150c2; continue;
 }
 0x120150c2 => {
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x120411dcu32); cpu.edi = v; }
  pc = 0x120150cd; continue;
 }
 0x120150cd => {
  { let v: u32 = cpu.r32(0x1203f01cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12015120; continue; }
  pc = 0x120150d9; continue;
 }
 0x120150d9 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.eax)), 0x83u8); }
  if cpu.zf { pc = 0x120150ee; continue; }
  pc = 0x120150df; continue;
 }
 0x120150df => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120150e5); f_1201525f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x120150ee; continue; }
  pc = 0x120150ed; continue;
 }
 0x120150ed => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x120150ee; continue;
 }
 0x120150ee => {
  { cpu.sub32(cpu.ebx, 0x50u32); }
  if (cpu.sf != cpu.of) { pc = 0x12015120; continue; }
  pc = 0x120150f3; continue;
 }
 0x120150f3 => {
  { let v: u32 = cpu.r32(0x1203f01cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x12015101); cpu.call(t);
  { let v: u32 = cpu.r32(0x1203f01cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12015110); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203f01cu32); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x12015120; continue;
 }
 0x12015120 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203c458u32)); }
  if (cpu.sf != cpu.of) { pc = 0x120150cd; continue; }
  pc = 0x1201512c; continue;
 }
 0x1201512c => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x12015133); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_1201513d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201513d); }
 let mut pc: u32 = 0x1201513d;
 loop { match pc {
 0x1201513d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201514c); f_120165df(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12015158; continue; }
  pc = 0x12015153; continue;
 }
 0x12015153 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015158 => {
  { cpu.sub32(cpu.esi, 0x1203c1f8u32); }
  if !cpu.zf { pc = 0x12015170; continue; }
  pc = 0x12015160; continue;
 }
 0x12015160 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x12015162; continue;
 }
 0x12015162 => {
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x10cu32); }
  if cpu.zf { pc = 0x1201517f; continue; }
  pc = 0x1201516b; continue;
 }
 0x1201516b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015170 => {
  { cpu.sub32(cpu.esi, 0x1203c218u32); }
  if !cpu.zf { pc = 0x120151c4; continue; }
  pc = 0x12015178; continue;
 }
 0x12015178 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12015162; continue;
 }
 0x1201517f => {
  { let v: u32 = 0x1203c460u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x120151a3; continue; }
  pc = 0x1201518b; continue;
 }
 0x1201518b => {
  { let v = 0x1000u32; cpu.push(v); }
  cpu.push(0x12015195); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120151a3; continue; }
  pc = 0x1201519e; continue;
 }
 0x1201519e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120151a3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1000u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x1102u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120151c4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120151c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120151c9); }
 let mut pc: u32 = 0x120151c9;
 loop { match pc {
 0x120151c9 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x120151ff; continue; }
  pc = 0x120151d1; continue;
 }
 0x120151d1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x120151ff; continue; }
  pc = 0x120151db; continue;
 }
 0x120151db => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120151e1); f_12004057(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)) & 0xffffeeffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x120151ff; continue;
 }
 0x120151ff => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015201(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015201); }
 let mut pc: u32 = 0x12015201;
 loop { match pc {
 0x12015201 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015216(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015216); }
 let mut pc: u32 = 0x12015216;
 loop { match pc {
 0x12015216 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015226(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015226); }
 let mut pc: u32 = 0x12015226;
 loop { match pc {
 0x12015226 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x83u8); }
  if cpu.zf { pc = 0x1201525d; continue; }
  pc = 0x12015232; continue;
 }
 0x12015232 => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x1201525d; continue; }
  pc = 0x12015236; continue;
 }
 0x12015236 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201523f); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xfffffbf7u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201525d; continue;
 }
 0x1201525d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201525f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201525f); }
 let mut pc: u32 = 0x1201525f;
 loop { match pc {
 0x1201525f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x1201527c; continue; }
  pc = 0x12015270; continue;
 }
 0x12015270 => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201527c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12015282); f_12012746(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201528b); f_1201529e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12015296); f_1201277c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201529e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201529e); }
 let mut pc: u32 = 0x1201529e;
 loop { match pc {
 0x1201529e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x83u8); }
  if cpu.zf { pc = 0x120152d8; continue; }
  pc = 0x120152af; continue;
 }
 0x120152af => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120152b5); f_12004057(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120152c0); f_12015226(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120152cc); f_1201660f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x120152e4; continue; }
  pc = 0x120152d3; continue;
 }
 0x120152d3 => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x120152d8; continue;
 }
 0x120152d8 => {
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120152e4 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120152d8; continue; }
  pc = 0x120152eb; continue;
 }
 0x120152eb => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120152f1); f_12005adb(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x120152d8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120152fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120152fd); }
 let mut pc: u32 = 0x120152fd;
 loop { match pc {
 0x120152fd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x82u8); }
  if cpu.zf { pc = 0x1201543e; continue; }
  pc = 0x12015313; continue;
 }
 0x12015313 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x1201543e; continue; }
  pc = 0x1201531b; continue;
 }
 0x1201531b => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1201533a; continue; }
  pc = 0x1201531f; continue;
 }
 0x1201531f => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x120153ce; continue; }
  pc = 0x12015331; continue;
 }
 0x12015331 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)) & 0xfffffffeu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x1201533a; continue;
 }
 0x1201533a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax & 0xffffffefu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x10cu32); }
  if !cpu.zf { pc = 0x12015381; continue; }
  pc = 0x1201535b; continue;
 }
 0x1201535b => {
  { cpu.sub32(cpu.esi, 0x1203c1f8u32); }
  if cpu.zf { pc = 0x1201536b; continue; }
  pc = 0x12015363; continue;
 }
 0x12015363 => {
  { cpu.sub32(cpu.esi, 0x1203c218u32); }
  if !cpu.zf { pc = 0x12015378; continue; }
  pc = 0x1201536b; continue;
 }
 0x1201536b => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12015371); f_120165df(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12015381; continue; }
  pc = 0x12015378; continue;
 }
 0x12015378 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201537e); f_1201544e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12015381; continue;
 }
 0x12015381 => {
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x108u32); }
  if cpu.zf { pc = 0x12015418; continue; }
  pc = 0x1201538e; continue;
 }
 0x1201538e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120153de; continue; }
  pc = 0x120153a3; continue;
 }
 0x120153a3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120153ae); f_1201636c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x120153b3; continue;
 }
 0x120153b3 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x120153bc; continue;
 }
 0x120153bc => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x12015430; continue; }
  pc = 0x120153c0; continue;
 }
 0x120153c0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.or32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x20u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120153ce => {
  { let v: u32 = cpu.or32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120153de => {
  { let v: u32 = 0x1203bc60u32; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x12015403; continue; }
  pc = 0x120153e8; continue;
 }
 0x120153e8 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  pc = 0x12015403; continue;
 }
 0x12015403 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x120153b3; continue; }
  pc = 0x12015409; continue;
 }
 0x12015409 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12015413); f_12016706(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x120153b3; continue;
 }
 0x12015418 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12015429); f_1201636c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x120153bc; continue;
 }
 0x12015430 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201543e => {
  { let v: u32 = cpu.or32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201544e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201544e); }
 let mut pc: u32 = 0x1201544e;
 loop { match pc {
 0x1201544e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1000u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x1201545d); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12015480; continue; }
  pc = 0x12015467; continue;
 }
 0x12015467 => {
  { let v: u32 = cpu.or32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x8u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1000u32; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x12015472; continue;
 }
 0x12015472 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015480 => {
  { let v: u32 = cpu.or32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x4u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x2u32; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x12015472; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015493(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015493); }
 let mut pc: u32 = 0x12015493;
 loop { match pc {
 0x12015493 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  if cpu.zf { pc = 0x120154aa; continue; }
  pc = 0x120154a1; continue;
 }
 0x120154a1 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edx)), 0x0u16); }
  if !cpu.zf { pc = 0x120154a1; continue; }
  pc = 0x120154aa; continue;
 }
 0x120154aa => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x120154ae; continue;
 }
 0x120154ae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { cpu.and16((cpu.esi as u16), (cpu.esi as u16)); }
  if !cpu.zf { pc = 0x120154ae; continue; }
  pc = 0x120154c1; continue;
 }
 0x120154c1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120154c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120154c4); }
 let mut pc: u32 = 0x120154c4;
 loop { match pc {
 0x120154c4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x120154d0; continue;
 }
 0x120154d0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { cpu.and16((cpu.esi as u16), (cpu.esi as u16)); }
  if !cpu.zf { pc = 0x120154d0; continue; }
  pc = 0x120154e3; continue;
 }
 0x120154e3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120154e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120154e6); }
 let mut pc: u32 = 0x120154e6;
 loop { match pc {
 0x120154e6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x1201551a; continue; }
  pc = 0x120154f2; continue;
 }
 0x120154f2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x120154f9; continue;
 }
 0x120154f9 => {
  { cpu.and16((cpu.esi as u16), (cpu.esi as u16)); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  if cpu.zf { pc = 0x12015511; continue; }
  pc = 0x12015500; continue;
 }
 0x12015500 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x12015503; continue;
 }
 0x12015503 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edx)), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x1201551c; continue; }
  pc = 0x12015508; continue;
 }
 0x12015508 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edx)), 0x0u16); }
  if !cpu.zf { pc = 0x12015503; continue; }
  pc = 0x12015511; continue;
 }
 0x12015511 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x120154f9; continue; }
  pc = 0x1201551a; continue;
 }
 0x1201551a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1201551c; continue;
 }
 0x1201551c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201551f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201551f); }
 let mut pc: u32 = 0x1201551f;
 loop { match pc {
 0x1201551f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  if cpu.zf { pc = 0x1201555d; continue; }
  pc = 0x12015530; continue;
 }
 0x12015530 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x12015534; continue;
 }
 0x12015534 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x1201554a; continue; }
  pc = 0x12015547; continue;
 }
 0x12015547 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x12015534; continue; }
  pc = 0x1201554a; continue;
 }
 0x1201554a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1201555d; continue; }
  pc = 0x1201554e; continue;
 }
 0x1201554e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x1201555d; continue; }
  pc = 0x12015551; continue;
 }
 0x12015551 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.stos(2, true); }
  pc = 0x1201555d; continue;
 }
 0x1201555d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015563(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015563); }
 let mut pc: u32 = 0x12015563;
 loop { match pc {
 0x12015563 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x12015570; continue; }
  pc = 0x1201556c; continue;
 }
 0x1201556c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015570 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12015576); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015586); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12015599; continue; }
  pc = 0x1201558d; continue;
 }
 0x1201558d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015594); f_120154c4(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015599 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201559d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201559d); }
 let mut pc: u32 = 0x1201559d;
 loop { match pc {
 0x1201559d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  if !cpu.zf { pc = 0x120155ce; continue; }
  pc = 0x120155b4; continue;
 }
 0x120155b4 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x120155ce; continue; }
  pc = 0x120155ba; continue;
 }
 0x120155ba => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  if cpu.zf { pc = 0x120155b4; continue; }
  pc = 0x120155ce; continue;
 }
 0x120155ce => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x120155d9; continue; }
  pc = 0x120155d2; continue;
 }
 0x120155d2 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120155d9 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120155e0; continue; }
  pc = 0x120155db; continue;
 }
 0x120155db => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x120155e0; continue;
 }
 0x120155e0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120155e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120155e2); }
 let mut pc: u32 = 0x120155e2;
 loop { match pc {
 0x120155e2 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if cpu.zf { pc = 0x12015684; continue; }
  pc = 0x120155f6; continue;
 }
 0x120155f6 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  if cpu.zf { pc = 0x1201566c; continue; }
  pc = 0x120155ff; continue;
 }
 0x120155ff => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015606); f_1200420b(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12015611; continue;
 }
 0x12015611 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x12015619; continue;
 }
 0x12015619 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015627); f_12015f00(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201563b); f_12015f00(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1201564b; continue; }
  pc = 0x12015646; continue;
 }
 0x12015646 => {
  { cpu.sub16((cpu.ebp as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x12015619; continue; }
  pc = 0x1201564b; continue;
 }
 0x1201564b => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201567c; continue; }
  pc = 0x12015652; continue;
 }
 0x12015652 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015659); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201565c; continue;
 }
 0x1201565c => {
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebp as u16) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201566c => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12015611; continue;
 }
 0x1201567c => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x1201565c; continue;
 }
 0x12015684 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x1201568c; continue;
 }
 0x1201568c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x5au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201569f; continue; }
  pc = 0x12015695; continue;
 }
 0x12015695 => {
  { cpu.sub16((cpu.ebx as u16), 0x41u16); }
  if cpu.cf { pc = 0x1201569f; continue; }
  pc = 0x1201569b; continue;
 }
 0x1201569b => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1201569f; continue;
 }
 0x1201569f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x5au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120156b2; continue; }
  pc = 0x120156a8; continue;
 }
 0x120156a8 => {
  { cpu.sub16((cpu.ebp as u16), 0x41u16); }
  if cpu.cf { pc = 0x120156b2; continue; }
  pc = 0x120156ae; continue;
 }
 0x120156ae => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add(0x20u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x120156b2; continue;
 }
 0x120156b2 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1201565c; continue; }
  pc = 0x120156bd; continue;
 }
 0x120156bd => {
  { cpu.sub16((cpu.ebp as u16), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x1201565c; continue; }
  pc = 0x120156c2; continue;
 }
 0x120156c2 => {
  pc = 0x1201568c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120156c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120156c4); }
 let mut pc: u32 = 0x120156c4;
 loop { match pc {
 0x120156c4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x1203c1a0u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1201571a; continue; }
  pc = 0x120156dc; continue;
 }
 0x120156dc => {
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.eax = v; }
  pc = 0x120156e0; continue;
 }
 0x120156e0 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x12015712; continue; }
  pc = 0x120156e4; continue;
 }
 0x120156e4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x120156e0; continue; }
  pc = 0x120156ee; continue;
 }
 0x120156ee => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  { let v: u8 = cpu.sbb8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = (cpu.eax as u8); let b = ((cpu.eax >> 8) as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  { let v: u8 = cpu.sbb8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x120156e0; continue; }
  pc = 0x1201570e; continue;
 }
 0x1201570e => {
  { let v: u8 = cpu.sbb8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0xffu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x12015712; continue;
 }
 0x12015712 => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  pc = 0x12015715; continue;
 }
 0x12015715 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201571a => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201576f; continue; }
  pc = 0x12015723; continue;
 }
 0x12015723 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x1201572b; continue;
 }
 0x1201572b => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  {  }
  pc = 0x12015733; continue;
 }
 0x12015733 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1201575e; continue; }
  pc = 0x12015737; continue;
 }
 0x12015737 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x12015733; continue; }
  pc = 0x12015741; continue;
 }
 0x12015741 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12015748); f_12016012(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x12015752); f_12016012(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub8((cpu.ebx as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x12015733; continue; }
  pc = 0x12015759; continue;
 }
 0x12015759 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x1201575e; continue;
 }
 0x1201575e => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x1201577f; continue; }
  pc = 0x12015765; continue;
 }
 0x12015765 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x1201576b; continue;
 }
 0x1201576b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x12015715; continue;
 }
 0x1201576f => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015776); f_1200420b(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201572b; continue;
 }
 0x1201577f => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015786); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1201576b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201578b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201578b); }
 let mut pc: u32 = 0x1201578b;
 loop { match pc {
 0x1201578b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1201580e; continue; }
  pc = 0x12015794; continue;
 }
 0x12015794 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.edi, 0x3u32); }
  { let v = cpu.ebx; cpu.push(v); }
  if cpu.zf { pc = 0x120157b1; continue; }
  pc = 0x120157a2; continue;
 }
 0x120157a2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x120157e2; continue; }
  pc = 0x120157a9; continue;
 }
 0x120157a9 => {
  { cpu.and32(cpu.edi, 0x3u32); }
  if !cpu.zf { pc = 0x120157a2; continue; }
  pc = 0x120157b1; continue;
 }
 0x120157b1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0x7efefeffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.and32(cpu.eax, 0x81010100u32); }
  if cpu.zf { pc = 0x120157b1; continue; }
  pc = 0x120157c9; continue;
 }
 0x120157c9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1201581e; continue; }
  pc = 0x120157d0; continue;
 }
 0x120157d0 => {
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x12015819; continue; }
  pc = 0x120157d4; continue;
 }
 0x120157d4 => {
  { cpu.and32(cpu.eax, 0xff0000u32); }
  if cpu.zf { pc = 0x12015814; continue; }
  pc = 0x120157db; continue;
 }
 0x120157db => {
  { cpu.and32(cpu.eax, 0xff000000u32); }
  if !cpu.zf { pc = 0x120157b1; continue; }
  pc = 0x120157e2; continue;
 }
 0x120157e2 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x120157e3; continue;
 }
 0x120157e3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x12015823; continue; }
  pc = 0x120157ef; continue;
 }
 0x120157ef => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1201583f; continue; }
  pc = 0x120157f6; continue;
 }
 0x120157f6 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x1201580a; continue; }
  pc = 0x120157fd; continue;
 }
 0x120157fd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1201580c; continue; }
  pc = 0x12015807; continue;
 }
 0x12015807 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x120157fd; continue; }
  pc = 0x1201580a; continue;
 }
 0x1201580a => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201580c; continue;
 }
 0x1201580c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1201580e; continue;
 }
 0x1201580e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015814 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  pc = 0x120157e3; continue;
 }
 0x12015819 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x3u32); cpu.edi = v; }
  pc = 0x120157e3; continue;
 }
 0x1201581e => {
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  pc = 0x120157e3; continue;
 }
 0x12015823 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1201587b; continue; }
  pc = 0x1201582a; continue;
 }
 0x1201582a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x1201580a; continue; }
  pc = 0x12015830; continue;
 }
 0x12015830 => {
  { cpu.and32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x12015823; continue; }
  pc = 0x12015838; continue;
 }
 0x12015838 => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x120157f6; continue; }
  pc = 0x1201583f; continue;
 }
 0x1201583f => {
  { let v: u32 = 0x7efefeffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, 0x81010100u32); }
  if cpu.zf { pc = 0x12015885; continue; }
  pc = 0x12015859; continue;
 }
 0x12015859 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1201587b; continue; }
  pc = 0x1201585d; continue;
 }
 0x1201585d => {
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x120158a2; continue; }
  pc = 0x12015861; continue;
 }
 0x12015861 => {
  { cpu.and32(cpu.edx, 0xff0000u32); }
  if cpu.zf { pc = 0x12015892; continue; }
  pc = 0x12015869; continue;
 }
 0x12015869 => {
  { cpu.and32(cpu.edx, 0xff000000u32); }
  if !cpu.zf { pc = 0x12015885; continue; }
  pc = 0x12015871; continue;
 }
 0x12015871 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201587b => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015885 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1201583f; continue; }
  pc = 0x1201588d; continue;
 }
 0x1201588d => {
  pc = 0x120157f6; continue;
 }
 0x12015892 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120158a2 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120158ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120158ad); }
 let mut pc: u32 = 0x120158ad;
 loop { match pc {
 0x120158ad => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edx)), 0x0u16); }
  if cpu.zf { pc = 0x120158ca; continue; }
  pc = 0x120158bc; continue;
 }
 0x120158bc => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edx)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x120158ca; continue; }
  pc = 0x120158c1; continue;
 }
 0x120158c1 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edx)), 0x0u16); }
  if !cpu.zf { pc = 0x120158bc; continue; }
  pc = 0x120158ca; continue;
 }
 0x120158ca => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, cpu.edx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120158d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120158d9); }
 let mut pc: u32 = 0x120158d9;
 loop { match pc {
 0x120158d9 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x120158ee; continue; }
  pc = 0x120158e6; continue;
 }
 0x120158e6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120158ee => {
  { cpu.sub32(cpu.r32(0x1203c1a4u32), 0x0u32); }
  if !cpu.zf { pc = 0x12015910; continue; }
  pc = 0x120158f7; continue;
 }
 0x120158f7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12015907); f_12015ab3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015910 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  if cpu.zf { pc = 0x12015958; continue; }
  pc = 0x12015919; continue;
 }
 0x12015919 => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x12015925); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12015928; continue;
 }
 0x12015928 => {
  { cpu.sub32(cpu.r32(0x1203c1a4u32), 0x0u32); }
  if !cpu.zf { pc = 0x1201596a; continue; }
  pc = 0x12015931; continue;
 }
 0x12015931 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12015962; continue; }
  pc = 0x12015935; continue;
 }
 0x12015935 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x1201593c); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201593f; continue;
 }
 0x1201593f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201594f); f_12015ab3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015958 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12015928; continue;
 }
 0x12015962 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x1201593f; continue;
 }
 0x1201596a => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1a4u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12015981); f_120034b3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12015993; continue; }
  pc = 0x12015988; continue;
 }
 0x12015988 => {
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015993 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0xau32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120159a1); f_12005f00(cpu);
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203c1a4u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1001u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120159c2); f_12016bfe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x120159f7; continue; }
  pc = 0x120159cb; continue;
 }
 0x120159cb => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x120159ef; continue; }
  pc = 0x120159cf; continue;
 }
 0x120159cf => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x120159d6); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120159d9; continue;
 }
 0x120159d9 => {
  cpu.push(0x120159de); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120159ef => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x120159d9; continue;
 }
 0x120159f7 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12015a0e; continue; }
  pc = 0x120159fb; continue;
 }
 0x120159fb => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015a02); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015a0e => {
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015a1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015a1d); }
 let mut pc: u32 = 0x12015a1d;
 loop { match pc {
 0x12003b23 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x12005f1f; continue; }
  pc = 0x12003b33; continue;
 }
 0x12003b33 => {
  pc = 0x12015a8a; continue;
 }
 0x12005f1a => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x12005f1f; continue;
 }
 0x12005f1f => {
  { let v: u32 = 0x7efefeffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, 0x81010100u32); }
  if cpu.zf { pc = 0x12005f1a; continue; }
  pc = 0x12005f39; continue;
 }
 0x12005f39 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12005f66; continue; }
  pc = 0x12005f3d; continue;
 }
 0x12005f3d => {
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x12005f5d; continue; }
  pc = 0x12005f41; continue;
 }
 0x12005f41 => {
  { cpu.and32(cpu.edx, 0xff0000u32); }
  if cpu.zf { pc = 0x12015aa6; continue; }
  pc = 0x12005f4d; continue;
 }
 0x12005f4d => {
  { cpu.and32(cpu.edx, 0xff000000u32); }
  if !cpu.zf { pc = 0x12005f1a; continue; }
  pc = 0x12005f55; continue;
 }
 0x12005f55 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12005f5d => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12005f66 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015a1d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x12015a39; continue; }
  pc = 0x12015a2a; continue;
 }
 0x12015a2a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x12015a6a; continue; }
  pc = 0x12015a31; continue;
 }
 0x12015a31 => {
  { cpu.and32(cpu.ecx, 0x3u32); }
  if !cpu.zf { pc = 0x12015a2a; continue; }
  pc = 0x12015a39; continue;
 }
 0x12015a39 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x7efefeffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, 0x81010100u32); }
  if cpu.zf { pc = 0x12015a39; continue; }
  pc = 0x12015a51; continue;
 }
 0x12015a51 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x12015a82; continue; }
  pc = 0x12015a58; continue;
 }
 0x12015a58 => {
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x12015a7a; continue; }
  pc = 0x12015a5c; continue;
 }
 0x12015a5c => {
  { cpu.and32(cpu.eax, 0xff0000u32); }
  if cpu.zf { pc = 0x12015a72; continue; }
  pc = 0x12015a63; continue;
 }
 0x12015a63 => {
  { cpu.and32(cpu.eax, 0xff000000u32); }
  if !cpu.zf { pc = 0x12015a39; continue; }
  pc = 0x12015a6a; continue;
 }
 0x12015a6a => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  pc = 0x12003b23; continue;
 }
 0x12015a72 => {
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  pc = 0x12003b23; continue;
 }
 0x12015a7a => {
  { let v: u32 = 0xfffffffdu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  pc = 0x12003b23; continue;
 }
 0x12015a82 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  pc = 0x12003b23; continue;
 }
 0x12015a8a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12005f66; continue; }
  pc = 0x12015a95; continue;
 }
 0x12015a95 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x12005f1f; continue; }
  pc = 0x12015aa4; continue;
 }
 0x12015aa4 => {
  pc = 0x12015a8a; continue;
 }
 0x12015aa6 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015ab3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015ab3); }
 let mut pc: u32 = 0x12015ab3;
 loop { match pc {
 0x12015ab3 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), cpu.eax); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if cpu.zf { pc = 0x12015b45; continue; }
  pc = 0x12015ac6; continue;
 }
 0x12015ac6 => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), cpu.eax); }
  if cpu.zf { pc = 0x12015b65; continue; }
  pc = 0x12015ad2; continue;
 }
 0x12015ad2 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), cpu.eax); }
  if cpu.zf { pc = 0x12015b4d; continue; }
  pc = 0x12015ada; continue;
 }
 0x12015ada => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015ae1); f_1200420b(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12015aec; continue;
 }
 0x12015aec => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x12015af4; continue;
 }
 0x12015af4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015b02); f_12015f00(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015b16); f_12015f00(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.esp))); cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x12015b2c; continue; }
  pc = 0x12015b22; continue;
 }
 0x12015b22 => {
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x12015b2c; continue; }
  pc = 0x12015b27; continue;
 }
 0x12015b27 => {
  { cpu.sub16((cpu.ebp as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x12015af4; continue; }
  pc = 0x12015b2c; continue;
 }
 0x12015b2c => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12015b5d; continue; }
  pc = 0x12015b33; continue;
 }
 0x12015b33 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015b3a); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12015b3d; continue;
 }
 0x12015b3d => {
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebp as u16) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x12015b45; continue;
 }
 0x12015b45 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015b4d => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12015aec; continue;
 }
 0x12015b5d => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12015b3d; continue;
 }
 0x12015b65 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x12015b6d; continue;
 }
 0x12015b6d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x5au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12015b80; continue; }
  pc = 0x12015b76; continue;
 }
 0x12015b76 => {
  { cpu.sub16((cpu.ebx as u16), 0x41u16); }
  if cpu.cf { pc = 0x12015b80; continue; }
  pc = 0x12015b7c; continue;
 }
 0x12015b7c => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x12015b80; continue;
 }
 0x12015b80 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x5au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12015b93; continue; }
  pc = 0x12015b89; continue;
 }
 0x12015b89 => {
  { cpu.sub16((cpu.ebp as u16), 0x41u16); }
  if cpu.cf { pc = 0x12015b93; continue; }
  pc = 0x12015b8f; continue;
 }
 0x12015b8f => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add(0x20u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x12015b93; continue;
 }
 0x12015b93 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.esp))); cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x12015b3d; continue; }
  pc = 0x12015b9f; continue;
 }
 0x12015b9f => {
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x12015b3d; continue; }
  pc = 0x12015ba4; continue;
 }
 0x12015ba4 => {
  { cpu.sub16((cpu.ebp as u16), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x12015b3d; continue; }
  pc = 0x12015ba9; continue;
 }
 0x12015ba9 => {
  pc = 0x12015b6d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015bab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015bab); }
 let mut pc: u32 = 0x12015bab;
 loop { match pc {
 0x12015bab => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015bbd); f_12015c09(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015bc1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015bc1); }
 let mut pc: u32 = 0x12015bc1;
 loop { match pc {
 0x12015bc1 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.fstatus(); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  {  }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015bd4); f_12015c09(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015bd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015bd8); }
 let mut pc: u32 = 0x12015bd8;
 loop { match pc {
 0x12015bd8 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12015bde); f_12014c3f(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  cpu.push(0x12015be7); f_12003902(cpu);
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12015c07; continue; }
  pc = 0x12015beb; continue;
 }
 0x12015beb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x10008u32); }
  if cpu.zf { pc = 0x12015c07; continue; }
  pc = 0x12015bf6; continue;
 }
 0x12015bf6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffu32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x1cu32); cpu.eax = v; }
  pc = 0x12015c07; continue;
 }
 0x12015c07 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015c09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015c09); }
 let mut pc: u32 = 0x12015c09;
 loop { match pc {
 0x12015c09 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x12015c1a; continue; }
  pc = 0x12015c15; continue;
 }
 0x12015c15 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  pc = 0x12015c1a; continue;
 }
 0x12015c1a => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x12015c22; continue; }
  pc = 0x12015c1f; continue;
 }
 0x12015c1f => {
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  pc = 0x12015c22; continue;
 }
 0x12015c22 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x12015c2a; continue; }
  pc = 0x12015c27; continue;
 }
 0x12015c27 => {
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  pc = 0x12015c2a; continue;
 }
 0x12015c2a => {
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x12015c32; continue; }
  pc = 0x12015c2f; continue;
 }
 0x12015c2f => {
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  pc = 0x12015c32; continue;
 }
 0x12015c32 => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if cpu.zf { pc = 0x12015c3a; continue; }
  pc = 0x12015c37; continue;
 }
 0x12015c37 => {
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  pc = 0x12015c3a; continue;
 }
 0x12015c3a => {
  { cpu.and8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x12015c44; continue; }
  pc = 0x12015c3f; continue;
 }
 0x12015c3f => {
  { let v: u32 = cpu.or32(cpu.eax, 0x80000u32); cpu.eax = v; }
  pc = 0x12015c44; continue;
 }
 0x12015c44 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015c45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015c45); }
 let mut pc: u32 = 0x12015c45;
 loop { match pc {
 0x12015c45 => {
  pc = cpu.r32(0x12041220u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015c4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015c4b); }
 let mut pc: u32 = 0x12015c4b;
 loop { match pc {
 0x12015c4b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x12015c76; continue; }
  pc = 0x12015c59; continue;
 }
 0x12015c59 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x12015c76; continue; }
  pc = 0x12015c5d; continue;
 }
 0x12015c5d => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x12015c62; continue;
 }
 0x12015c62 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12015c6f); f_12003996(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015c76 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x12015c62; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015c7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015c7a); }
 let mut pc: u32 = 0x12015c7a;
 loop { match pc {
 0x12015c7a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12015c91); f_12003996(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015c98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015c98); }
 let mut pc: u32 = 0x12015c98;
 loop { match pc {
 0x12015c98 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x12015cd1; continue; }
  pc = 0x12015ca2; continue;
 }
 0x12015ca2 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esp)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12015cd1; continue; }
  pc = 0x12015ca9; continue;
 }
 0x12015ca9 => {
  if (cpu.sf != cpu.of) { pc = 0x12015cb2; continue; }
  pc = 0x12015cab; continue;
 }
 0x12015cab => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.cf { pc = 0x12015cd1; continue; }
  pc = 0x12015cb2; continue;
 }
 0x12015cb2 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x12015cb7; continue;
 }
 0x12015cb7 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015ccd); f_12015cd5(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015cd1 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  pc = 0x12015cb7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015cd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015cd5); }
 let mut pc: u32 = 0x12015cd5;
 loop { match pc {
 0x12015cd5 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if cpu.zf { pc = 0x12015ceb; continue; }
  pc = 0x12015ce7; continue;
 }
 0x12015ce7 => {
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x12015ceb; continue;
 }
 0x12015ceb => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x12015cfd; continue;
 }
 0x12015cfd => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015d16); f_12005d5f(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015d31); f_12004b55(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, 0x9u32); }
  { let v: u32 = cpu.edx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  if (cpu.cf || cpu.zf) { pc = 0x12015d70; continue; }
  pc = 0x12015d3e; continue;
 }
 0x12015d3e => {
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x57u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12015d41; continue;
 }
 0x12015d41 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12015cfd; continue; }
  pc = 0x12015d4b; continue;
 }
 0x12015d4b => {
  if cpu.cf { pc = 0x12015d54; continue; }
  pc = 0x12015d4d; continue;
 }
 0x12015d4d => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12015cfd; continue; }
  pc = 0x12015d54; continue;
 }
 0x12015d54 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  pc = 0x12015d58; continue;
 }
 0x12015d58 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.cf { pc = 0x12015d58; continue; }
  pc = 0x12015d66; continue;
 }
 0x12015d66 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x12015d70 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12015d41; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015d75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015d75); }
 let mut pc: u32 = 0x12015d75;
 loop { match pc {
 0x12015d75 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12015d91); f_12015cd5(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015d95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015d95); }
 let mut pc: u32 = 0x12015d95;
 loop { match pc {
 0x12015d95 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x12015dd6; continue; }
  pc = 0x12015da0; continue;
 }
 0x12015da0 => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x12015dac); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12015daf; continue;
 }
 0x12015daf => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12015dbe); f_12015deb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12015de0; continue; }
  pc = 0x12015dc7; continue;
 }
 0x12015dc7 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015dce); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015dd6 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12015daf; continue;
 }
 0x12015de0 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015deb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015deb); }
 let mut pc: u32 = 0x12015deb;
 loop { match pc {
 0x12015deb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x12015dfc; continue; }
  pc = 0x12015df6; continue;
 }
 0x12015df6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015dfc => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x12015e31; continue; }
  pc = 0x12015e05; continue;
 }
 0x12015e05 => {
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esp)), 0xffu16); }
  if (cpu.cf || cpu.zf) { pc = 0x12015e22; continue; }
  pc = 0x12015e0e; continue;
 }
 0x12015e0e => {
  cpu.push(0x12015e13); f_120133b4(cpu);
  { let v: u32 = 0x2au32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015e22 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015e31 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203befcu32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12015e60); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12015e6b; continue; }
  pc = 0x12015e64; continue;
 }
 0x12015e64 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12015e7b; continue; }
  pc = 0x12015e6b; continue;
 }
 0x12015e6b => {
  cpu.push(0x12015e70); f_120133b4(cpu);
  { let v: u32 = 0x2au32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x12015e7b; continue;
 }
 0x12015e7b => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015e7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015e7f); }
 let mut pc: u32 = 0x12015e7f;
 loop { match pc {
 0x12015e7f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edi as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x12015e93; continue; }
  pc = 0x12015e8d; continue;
 }
 0x12015e8d => {
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015e93 => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x12015eb2; continue; }
  pc = 0x12015e9c; continue;
 }
 0x12015e9c => {
  { cpu.sub16((cpu.edi as u16), 0x41u16); }
  if cpu.cf { pc = 0x12015eac; continue; }
  pc = 0x12015ea2; continue;
 }
 0x12015ea2 => {
  { cpu.sub16((cpu.edi as u16), 0x5au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12015eac; continue; }
  pc = 0x12015ea8; continue;
 }
 0x12015ea8 => {
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x20u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x12015eac; continue;
 }
 0x12015eac => {
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015eb2 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  if cpu.zf { pc = 0x12015eea; continue; }
  pc = 0x12015ebb; continue;
 }
 0x12015ebb => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x12015ec7); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12015eca; continue;
 }
 0x12015eca => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12015ed0); f_12015f00(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12015ef4; continue; }
  pc = 0x12015eda; continue;
 }
 0x12015eda => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015ee1); f_120040fd(cpu);
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015eea => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12015eca; continue;
 }
 0x12015ef4 => {
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015f00); }
 let mut pc: u32 = 0x12015f00;
 loop { match pc {
 0x12015f00 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0xffffu16); }
  if !cpu.zf { pc = 0x12015f15; continue; }
  pc = 0x12015f0c; continue;
 }
 0x12015f0c => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015f15 => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x12015f3d; continue; }
  pc = 0x12015f1e; continue;
 }
 0x12015f1e => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0x41u16); }
  if cpu.cf { pc = 0x12015f34; continue; }
  pc = 0x12015f26; continue;
 }
 0x12015f26 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0x5au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12015f34; continue; }
  pc = 0x12015f2e; continue;
 }
 0x12015f2e => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)).wrapping_add(0x20u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  pc = 0x12015f34; continue;
 }
 0x12015f34 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015f3d => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0x100u16); }
  if !cpu.cf { pc = 0x12015f62; continue; }
  pc = 0x12015f46; continue;
 }
 0x12015f46 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015f52); f_12004b0f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12015f62; continue; }
  pc = 0x12015f59; continue;
 }
 0x12015f59 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015f62 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1a8u32); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12015f83); f_12016e76(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12015f93; continue; }
  pc = 0x12015f8a; continue;
 }
 0x12015f8a => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015f93 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015f9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015f9c); }
 let mut pc: u32 = 0x12015f9c;
 loop { match pc {
 0x12015f9c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12015fa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12015fa4); }
 let mut pc: u32 = 0x12015fa4;
 loop { match pc {
 0x12015fa4 => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x12015fc3; continue; }
  pc = 0x12015faf; continue;
 }
 0x12015faf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x1201600f; continue; }
  pc = 0x12015fb8; continue;
 }
 0x12015fb8 => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201600f; continue; }
  pc = 0x12015fbd; continue;
 }
 0x12015fbd => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015fc3 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  if cpu.zf { pc = 0x12015ffd; continue; }
  pc = 0x12015fcc; continue;
 }
 0x12015fcc => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x12015fd8); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12015fdb; continue;
 }
 0x12015fdb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12015fe5); f_12016012(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12016007; continue; }
  pc = 0x12015fee; continue;
 }
 0x12015fee => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12015ff5); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12015ffd => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12015fdb; continue;
 }
 0x12016007 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x1201600f; continue;
 }
 0x1201600f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016012(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016012); }
 let mut pc: u32 = 0x12016012;
 loop { match pc {
 0x12016012 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  if !cpu.zf { pc = 0x12016037; continue; }
  pc = 0x12016023; continue;
 }
 0x12016023 => {
  { cpu.sub32(cpu.ebx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x12016030; continue; }
  pc = 0x12016028; continue;
 }
 0x12016028 => {
  { cpu.sub32(cpu.ebx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12016030; continue; }
  pc = 0x1201602d; continue;
 }
 0x1201602d => {
  { let v: u32 = cpu.ebx.wrapping_add(0x20u32); cpu.ebx = v; }
  pc = 0x12016030; continue;
 }
 0x12016030 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016037 => {
  { cpu.sub32(cpu.ebx, 0x100u32); }
  if (cpu.sf == cpu.of) { pc = 0x1201606f; continue; }
  pc = 0x1201603f; continue;
 }
 0x1201603f => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201605e; continue; }
  pc = 0x12016048; continue;
 }
 0x12016048 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12016050); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12016053; continue;
 }
 0x12016053 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201606f; continue; }
  pc = 0x12016057; continue;
 }
 0x12016057 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201605e => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  pc = 0x12016053; continue;
 }
 0x1201606f => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x120160c1; continue; }
  pc = 0x12016081; continue;
 }
 0x12016081 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x9u32.wrapping_add(cpu.esp), v); }
  pc = 0x12016093; continue;
 }
 0x12016093 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1a8u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120160b3); f_120170bd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120160d1; continue; }
  pc = 0x120160ba; continue;
 }
 0x120160ba => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120160c1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x9u32.wrapping_add(cpu.esp), v); }
  pc = 0x12016093; continue;
 }
 0x120160d1 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x120160e1; continue; }
  pc = 0x120160d6; continue;
 }
 0x120160d6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120160e1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax | cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120160f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120160f7); }
 let mut pc: u32 = 0x120160f7;
 loop { match pc {
 0x120160f7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201610d); f_12005d86(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016111(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016111); }
 let mut pc: u32 = 0x12016111;
 loop { match pc {
 0x12016111 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12016120); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016124(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016124); }
 let mut pc: u32 = 0x12016124;
 loop { match pc {
 0x12016124 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201614b; continue; }
  pc = 0x12016136; continue;
 }
 0x12016136 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201614b => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x12016193; continue; }
  pc = 0x1201615d; continue;
 }
 0x1201615d => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x9u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201616f; continue;
 }
 0x1201616f => {
  { let v: u32 = 0x6u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12016185); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120161a3; continue; }
  pc = 0x1201618c; continue;
 }
 0x1201618c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016193 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x9u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201616f; continue;
 }
 0x120161a3 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120161b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120161b5); }
 let mut pc: u32 = 0x120161b5;
 loop { match pc {
 0x120161b5 => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x120161d8; continue; }
  pc = 0x120161c0; continue;
 }
 0x120161c0 => {
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x61u16); }
  if cpu.cf { pc = 0x12016227; continue; }
  pc = 0x120161cb; continue;
 }
 0x120161cb => {
  { cpu.sub16((cpu.eax as u16), 0x7au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12016227; continue; }
  pc = 0x120161d1; continue;
 }
 0x120161d1 => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120161d8 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  if cpu.zf { pc = 0x12016214; continue; }
  pc = 0x120161e1; continue;
 }
 0x120161e1 => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x120161ed); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120161f0; continue;
 }
 0x120161f0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120161fa); f_1201622a(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1201621e; continue; }
  pc = 0x12016204; continue;
 }
 0x12016204 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x1201620b); f_120040fd(cpu);
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016214 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x120161f0; continue;
 }
 0x1201621e => {
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12016227; continue;
 }
 0x12016227 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201622a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201622a); }
 let mut pc: u32 = 0x1201622a;
 loop { match pc {
 0x1201622a => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0xffffu16); }
  if !cpu.zf { pc = 0x1201623f; continue; }
  pc = 0x12016236; continue;
 }
 0x12016236 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201623f => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x12016267; continue; }
  pc = 0x12016248; continue;
 }
 0x12016248 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0x61u16); }
  if cpu.cf { pc = 0x1201625e; continue; }
  pc = 0x12016250; continue;
 }
 0x12016250 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0x7au16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201625e; continue; }
  pc = 0x12016258; continue;
 }
 0x12016258 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)).wrapping_sub(0x20u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201625e; continue;
 }
 0x1201625e => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016267 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0x100u16); }
  if !cpu.cf { pc = 0x1201628c; continue; }
  pc = 0x12016270; continue;
 }
 0x12016270 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201627c); f_12004b0f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201628c; continue; }
  pc = 0x12016283; continue;
 }
 0x12016283 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201628c => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1a8u32); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x200u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120162ad); f_12016e76(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120162bd; continue; }
  pc = 0x120162b4; continue;
 }
 0x120162b4 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120162bd => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120162c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120162c6); }
 let mut pc: u32 = 0x120162c6;
 loop { match pc {
 0x120162c6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x12016358; continue; }
  pc = 0x120162d9; continue;
 }
 0x120162d9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x12016358; continue; }
  pc = 0x120162fb; continue;
 }
 0x120162fb => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12016301); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x12016339; continue; }
  pc = 0x1201630d; continue;
 }
 0x1201630d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  cpu.push(0x12016318); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041128u32); cpu.push(0x12016322); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201632e; continue; }
  pc = 0x12016326; continue;
 }
 0x12016326 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201632c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x1201632e; continue;
 }
 0x1201632e => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12016349; continue; }
  pc = 0x12016332; continue;
 }
 0x12016332 => {
  cpu.push(0x12016337); f_120133bd(cpu);
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12016339; continue;
 }
 0x12016339 => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  cpu.push(0x12016343); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12016349; continue;
 }
 0x12016349 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201634f); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016358 => {
  cpu.push(0x1201635d); f_120133b4(cpu);
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
pub fn f_1201636c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201636c); }
 let mut pc: u32 = 0x1201636c;
 loop { match pc {
 0x1201636c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x120163c6; continue; }
  pc = 0x1201637a; continue;
 }
 0x1201637a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x120163c6; continue; }
  pc = 0x1201639a; continue;
 }
 0x1201639a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120163a0); f_12016b72(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120163b3); f_120163e4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120163be); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120163c6 => {
  cpu.push(0x120163cb); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120163d6); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120163e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120163e4); }
 let mut pc: u32 = 0x120163e4;
 loop { match pc {
 0x120163e4 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x418u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x434u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  if !cpu.zf { pc = 0x1201640c; continue; }
  pc = 0x120163ff; continue;
 }
 0x120163ff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201640c => {
  { let v: u32 = cpu.r32(0x42cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1203ef10u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x20u8); }
  if cpu.zf { pc = 0x1201644d; continue; }
  pc = 0x12016440; continue;
 }
 0x12016440 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201644a); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1201644d; continue;
 }
 0x1201644d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ecx)), 0x80u8); }
  if cpu.zf { pc = 0x12016526; continue; }
  pc = 0x12016461; continue;
 }
 0x12016461 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x430u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x12016470; continue;
 }
 0x12016470 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x430u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.cf { pc = 0x120164e3; continue; }
  pc = 0x1201647d; continue;
 }
 0x1201647d => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.esi = v; }
  pc = 0x12016481; continue;
 }
 0x12016481 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x430u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if !cpu.cf { pc = 0x120164ac; continue; }
  pc = 0x1201648e; continue;
 }
 0x1201648e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if !cpu.zf { pc = 0x1201649a; continue; }
  pc = 0x12016495; continue;
 }
 0x12016495 => {
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1201649a; continue;
 }
 0x1201649a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x12016481; continue; }
  pc = 0x120164ac; continue;
 }
 0x120164ac => {
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
  let t = cpu.r32(0x120411f8u32); cpu.push(0x120164d3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12016555; continue; }
  pc = 0x120164d7; continue;
 }
 0x120164d7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)).wrapping_add(cpu.eax); cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x12016470; continue; }
  pc = 0x120164e3; continue;
 }
 0x120164e3 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x120165ce; continue; }
  pc = 0x120164ee; continue;
 }
 0x120164ee => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201657e; continue; }
  pc = 0x120164f9; continue;
 }
 0x120164f9 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x5u32); }
  if !cpu.zf { pc = 0x12016561; continue; }
  pc = 0x12016500; continue;
 }
 0x12016500 => {
  cpu.push(0x12016505); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12016510); f_120133bd(cpu);
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
 0x12016526 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x434u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120411f8u32); cpu.push(0x1201653f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12016555; continue; }
  pc = 0x12016543; continue;
 }
 0x12016543 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  pc = 0x120164e3; continue;
 }
 0x12016555 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201655b); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x120164e3; continue;
 }
 0x12016561 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201656b); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201657e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x40u8); }
  if cpu.zf { pc = 0x120165a8; continue; }
  pc = 0x1201658f; continue;
 }
 0x1201658f => {
  { let v: u32 = cpu.r32(0x430u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1au8); }
  if !cpu.zf { pc = 0x120165a8; continue; }
  pc = 0x1201659b; continue;
 }
 0x1201659b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120165a8 => {
  cpu.push(0x120165ad); f_120133b4(cpu);
  { let v: u32 = 0x1cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120165b8); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x418u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120165ce => {
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
pub fn f_120165df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120165df); }
 let mut pc: u32 = 0x120165df;
 loop { match pc {
 0x120165df => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x1203f010u32)); }
  if cpu.cf { pc = 0x120165ee; continue; }
  pc = 0x120165eb; continue;
 }
 0x120165eb => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120165ee => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x40u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201660f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201660f); }
 let mut pc: u32 = 0x1201660f;
 loop { match pc {
 0x1201660f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201665f; continue; }
  pc = 0x1201661d; continue;
 }
 0x1201661d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201665f; continue; }
  pc = 0x1201663d; continue;
 }
 0x1201663d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12016643); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201664c); f_1201667d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12016657); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201665f => {
  cpu.push(0x12016664); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201666f); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201667d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201667d); }
 let mut pc: u32 = 0x1201667d;
 loop { match pc {
 0x1201667d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.zf { pc = 0x1201668d; continue; }
  pc = 0x12016688; continue;
 }
 0x12016688 => {
  { cpu.sub32(cpu.esi, 0x2u32); }
  if !cpu.zf { pc = 0x120166a7; continue; }
  pc = 0x1201668d; continue;
 }
 0x1201668d => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x12016694); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x120166a0); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if cpu.zf { pc = 0x120166e1; continue; }
  pc = 0x120166a7; continue;
 }
 0x120166a7 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120166ad); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x120166b7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120166e1; continue; }
  pc = 0x120166bb; continue;
 }
 0x120166bb => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x120166c1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x120166c3; continue;
 }
 0x120166c3 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120166c9); f_120169e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x120166e5; continue; }
  pc = 0x120166d0; continue;
 }
 0x120166d0 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120166d6); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120166e1 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x120166c3; continue;
 }
 0x120166e5 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi & 0x1fu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016706(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016706); }
 let mut pc: u32 = 0x12016706;
 loop { match pc {
 0x12016706 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x12016760; continue; }
  pc = 0x12016714; continue;
 }
 0x12016714 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x12016760; continue; }
  pc = 0x12016734; continue;
 }
 0x12016734 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201673a); f_12016b72(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201674d); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12016758); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016760 => {
  cpu.push(0x12016765); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12016770); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201677e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201677e); }
 let mut pc: u32 = 0x1201677e;
 loop { match pc {
 0x1201677e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201678a); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x120167a5; continue; }
  pc = 0x12016792; continue;
 }
 0x12016792 => {
  cpu.push(0x12016797); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120167a5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041124u32); cpu.push(0x120167b8); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x120167ca; continue; }
  pc = 0x120167c4; continue;
 }
 0x120167c4 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x120167ca); cpu.call(t);
  pc = 0x120167ca; continue;
 }
 0x120167ca => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120167df; continue; }
  pc = 0x120167ce; continue;
 }
 0x120167ce => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120167d4); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120167df => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi & 0x1fu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
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
pub fn f_12016800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016800); }
 let mut pc: u32 = 0x12016800;
 loop { match pc {
 0x12016800 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v = 0x12u32; cpu.push(v); }
  { let v: u32 = 0x1203ef10u32; cpu.esi = v; }
  cpu.push(0x1201681a); f_1200420b(cpu);
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12016821; continue;
 }
 0x12016821 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x120168d7; continue; }
  pc = 0x1201682b; continue;
 }
 0x1201682b => {
  { let v: u32 = 0x480u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.cf || cpu.zf) { pc = 0x12016890; continue; }
  pc = 0x12016835; continue;
 }
 0x12016835 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x12016882; continue; }
  pc = 0x1201683b; continue;
 }
 0x1201683b => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x12016868; continue; }
  pc = 0x12016841; continue;
 }
 0x12016841 => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x12016848); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201685e; continue; }
  pc = 0x12016851; continue;
 }
 0x12016851 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411e0u32); cpu.push(0x1201685b); cpu.call(t);
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201685e; continue;
 }
 0x1201685e => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x12016865); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12016868; continue;
 }
 0x12016868 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411d8u32); cpu.push(0x12016872); cpu.call(t);
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x120168bf; continue; }
  pc = 0x12016878; continue;
 }
 0x12016878 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411d4u32); cpu.push(0x12016882); cpu.call(t);
  pc = 0x12016882; continue;
 }
 0x12016882 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x24u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x480u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12016835; continue; }
  pc = 0x12016890; continue;
 }
 0x12016890 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x120168ab; continue; }
  pc = 0x12016895; continue;
 }
 0x12016895 => {
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.esi, 0x1203f010u32); }
  if cpu.cf { pc = 0x12016821; continue; }
  pc = 0x120168ab; continue;
 }
 0x120168ab => {
  { let v = 0x12u32; cpu.push(v); }
  cpu.push(0x120168b2); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120168bf => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x12016890; continue;
 }
 0x120168d7 => {
  { let v = 0x480u32; cpu.push(v); }
  cpu.push(0x120168e1); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120168ab; continue; }
  pc = 0x120168e8; continue;
 }
 0x120168e8 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x480u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203f010u32).wrapping_add(0x20u32); cpu.w32(0x1203f010u32, v); }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if (cpu.cf || cpu.zf) { pc = 0x1201692c; continue; }
  pc = 0x12016906; continue;
 }
 0x12016906 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x1201690d; continue;
 }
 0x1201690d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x24u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffe1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x480u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201690d; continue; }
  pc = 0x1201692c; continue;
 }
 0x1201692c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12016939); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120168ab; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016941(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016941); }
 let mut pc: u32 = 0x12016941;
 loop { match pc {
 0x12016941 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.ecx, cpu.r32(0x1203f010u32)); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.cf { pc = 0x120169c4; continue; }
  pc = 0x12016950; continue;
 }
 0x12016950 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))), 0xffffffffu32); }
  if !cpu.zf { pc = 0x120169c4; continue; }
  pc = 0x12016971; continue;
 }
 0x12016971 => {
  { cpu.sub32(cpu.r32(0x1203b1f8u32), 0x1u32); }
  if !cpu.zf { pc = 0x12016988; continue; }
  pc = 0x1201697a; continue;
 }
 0x1201697a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1201698e; continue; }
  pc = 0x1201697e; continue;
 }
 0x1201697e => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x120169a6; continue; }
  pc = 0x12016983; continue;
 }
 0x12016983 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x120169b5; continue; }
  pc = 0x12016988; continue;
 }
 0x12016988 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x1201699b; continue;
 }
 0x1201698e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xfffffff6u32; cpu.push(v); }
  let t = cpu.r32(0x12041120u32); cpu.push(0x1201699b); cpu.call(t);
  pc = 0x1201699b; continue;
 }
 0x1201699b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120169a6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xfffffff5u32; cpu.push(v); }
  let t = cpu.r32(0x12041120u32); cpu.push(0x120169b3); cpu.call(t);
  pc = 0x1201699b; continue;
 }
 0x120169b5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xfffffff4u32; cpu.push(v); }
  let t = cpu.r32(0x12041120u32); cpu.push(0x120169c2); cpu.call(t);
  pc = 0x1201699b; continue;
 }
 0x120169c4 => {
  cpu.push(0x120169c9); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120169d4); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120169e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120169e3); }
 let mut pc: u32 = 0x120169e3;
 loop { match pc {
 0x120169e3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x12016a56; continue; }
  pc = 0x120169f1; continue;
 }
 0x120169f1 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x12016a56; continue; }
  pc = 0x12016a14; continue;
 }
 0x12016a14 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0xffffffffu32); }
  if cpu.zf { pc = 0x12016a56; continue; }
  pc = 0x12016a19; continue;
 }
 0x12016a19 => {
  { cpu.sub32(cpu.r32(0x1203b1f8u32), 0x1u32); }
  if !cpu.zf { pc = 0x12016a30; continue; }
  pc = 0x12016a22; continue;
 }
 0x12016a22 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12016a3e; continue; }
  pc = 0x12016a26; continue;
 }
 0x12016a26 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x12016a4a; continue; }
  pc = 0x12016a2b; continue;
 }
 0x12016a2b => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x12016a50; continue; }
  pc = 0x12016a30; continue;
 }
 0x12016a30 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016a3e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfffffff6u32; cpu.push(v); }
  pc = 0x12016a42; continue;
 }
 0x12016a42 => {
  let t = cpu.r32(0x12041120u32); cpu.push(0x12016a48); cpu.call(t);
  pc = 0x12016a30; continue;
 }
 0x12016a4a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfffffff5u32; cpu.push(v); }
  pc = 0x12016a42; continue;
 }
 0x12016a50 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfffffff4u32; cpu.push(v); }
  pc = 0x12016a42; continue;
 }
 0x12016a56 => {
  cpu.push(0x12016a5b); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12016a66); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016a74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016a74); }
 let mut pc: u32 = 0x12016a74;
 loop { match pc {
 0x12016a74 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x12016aa2; continue; }
  pc = 0x12016a80; continue;
 }
 0x12016a80 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x12016aa2; continue; }
  pc = 0x12016a9f; continue;
 }
 0x12016a9f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016aa2 => {
  cpu.push(0x12016aa7); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12016ab2); f_120133bd(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016abe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016abe); }
 let mut pc: u32 = 0x12016abe;
 loop { match pc {
 0x12016abe => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x12016acd; continue; }
  pc = 0x12016acb; continue;
 }
 0x12016acb => {
  { let v: u8 = 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12016acd; continue;
 }
 0x12016acd => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x12016ad5; continue; }
  pc = 0x12016ad2; continue;
 }
 0x12016ad2 => {
  { let v: u8 = (cpu.ebx as u8) | 0x80u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12016ad5; continue;
 }
 0x12016ad5 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x12041170u32); cpu.push(0x12016ae0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12016afc; continue; }
  pc = 0x12016ae4; continue;
 }
 0x12016ae4 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12016aea); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12016af0); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016afc => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x12016b2f; continue; }
  pc = 0x12016b01; continue;
 }
 0x12016b01 => {
  { let v: u8 = (cpu.ebx as u8) | 0x40u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12016b04; continue;
 }
 0x12016b04 => {
  cpu.push(0x12016b09); f_12016800(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  if !cpu.zf { pc = 0x12016b39; continue; }
  pc = 0x12016b10; continue;
 }
 0x12016b10 => {
  cpu.push(0x12016b15); f_120133b4(cpu);
  { let v: u32 = 0x18u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12016b20); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016b2f => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x12016b04; continue; }
  pc = 0x12016b34; continue;
 }
 0x12016b34 => {
  { let v: u8 = (cpu.ebx as u8) | 0x8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12016b04; continue;
 }
 0x12016b39 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12016b43); f_12016941(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  cpu.push(0x12016b69); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016b72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016b72); }
 let mut pc: u32 = 0x12016b72;
 loop { match pc {
 0x12016b72 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffffffe7u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x12016bc1; continue; }
  pc = 0x12016b9a; continue;
 }
 0x12016b9a => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x12016ba1); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x12016bb7; continue; }
  pc = 0x12016baa; continue;
 }
 0x12016baa => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411e0u32); cpu.push(0x12016bb4); cpu.call(t);
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.ebx))); cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12016bb7; continue;
 }
 0x12016bb7 => {
  { let v = 0x11u32; cpu.push(v); }
  cpu.push(0x12016bbe); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12016bc1; continue;
 }
 0x12016bc1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411d8u32); cpu.push(0x12016bcf); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016bd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016bd3); }
 let mut pc: u32 = 0x12016bd3;
 loop { match pc {
 0x12016bd3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411d4u32); cpu.push(0x12016bfa); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016bfb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016bfb); }
 let mut pc: u32 = 0x12016bfb;
 loop { match pc {
 0x12016bfb => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016bfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016bfe); }
 let mut pc: u32 = 0x12016bfe;
 loop { match pc {
 0x12016bfe => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203c694u32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if !cpu.zf { pc = 0x12016cd8; continue; }
  pc = 0x12016c12; continue;
 }
 0x12016c12 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041224u32); cpu.edi = v; }
  { let v = 0x12035e3cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12035e3cu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x12016c2c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12016ca3; continue; }
  pc = 0x12016c30; continue;
 }
 0x12016c30 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203c694u32, v); }
  pc = 0x12016c3a; continue;
 }
 0x12016c3a => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12016c52; continue; }
  pc = 0x12016c42; continue;
 }
 0x12016c42 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12016c4d); f_12016e43(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x12016c52; continue;
 }
 0x12016c52 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12016c6a; continue; }
  pc = 0x12016c5a; continue;
 }
 0x12016c5a => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12016c65); f_12016e43(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x12016c6a; continue;
 }
 0x12016c6a => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12016e1c; continue; }
  pc = 0x12016c72; continue;
 }
 0x12016c72 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12016e1c; continue; }
  pc = 0x12016c7a; continue;
 }
 0x12016c7a => {
  { cpu.sub32(cpu.r32(0x1203c694u32), 0x1u32); }
  if !cpu.zf { pc = 0x12016ce3; continue; }
  pc = 0x12016c83; continue;
 }
 0x12016c83 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x12016c9b); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016ca3 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12035e38u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12035e38u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041118u32); cpu.push(0x12016cbb); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12016cce; continue; }
  pc = 0x12016cbf; continue;
 }
 0x12016cbf => {
  { let v: u32 = 0x2u32; cpu.w32(0x1203c694u32, v); }
  pc = 0x12016c3a; continue;
 }
 0x12016cce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016cd8 => {
  { let v: u32 = cpu.r32(0x12041224u32); cpu.edi = v; }
  pc = 0x12016c3a; continue;
 }
 0x12016ce3 => {
  { cpu.sub32(cpu.r32(0x1203c694u32), 0x2u32); }
  if !cpu.zf { pc = 0x12016e3b; continue; }
  pc = 0x12016cf0; continue;
 }
 0x12016cf0 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  if !cpu.zf { pc = 0x12016d00; continue; }
  pc = 0x12016cfa; continue;
 }
 0x12016cfa => {
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.ebp = v; }
  pc = 0x12016d00; continue;
 }
 0x12016d00 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12016d1a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12016d2c; continue; }
  pc = 0x12016d22; continue;
 }
 0x12016d22 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016d2c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12016d36); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12016d4b; continue; }
  pc = 0x12016d41; continue;
 }
 0x12016d41 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016d4b => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12016d6b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12016dfc; continue; }
  pc = 0x12016d73; continue;
 }
 0x12016d73 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12016d8d); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12016dfc; continue; }
  pc = 0x12016d93; continue;
 }
 0x12016d93 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12016d99); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12016dfc; continue; }
  pc = 0x12016da2; continue;
 }
 0x12016da2 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12016dba); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12016dfc; continue; }
  pc = 0x12016dbe; continue;
 }
 0x12016dbe => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041118u32); cpu.push(0x12016dda); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12016de6); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12016def); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016dfc => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12016e06); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12016e0f); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016e1c => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if !cpu.zf { pc = 0x12016e2d; continue; }
  pc = 0x12016e20; continue;
 }
 0x12016e20 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016e2d => {
  { let v: u32 = cpu.sub32(cpu.esi, cpu.ebx); cpu.esi = v; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  if !cpu.sf { pc = 0x12016e3b; continue; }
  pc = 0x12016e36; continue;
 }
 0x12016e36 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12016e3b; continue;
 }
 0x12016e3b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016e43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016e43); }
 let mut pc: u32 = 0x12016e43;
 loop { match pc {
 0x12016e43 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  if cpu.zf { pc = 0x12016e66; continue; }
  pc = 0x12016e56; continue;
 }
 0x12016e56 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x12016e6c; continue; }
  pc = 0x12016e5c; continue;
 }
 0x12016e5c => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12016e56; continue; }
  pc = 0x12016e66; continue;
 }
 0x12016e66 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x12016e73; continue; }
  pc = 0x12016e6c; continue;
 }
 0x12016e6c => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x12016e73; continue;
 }
 0x12016e73 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12016e76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12016e76); }
 let mut pc: u32 = 0x12016e76;
 loop { match pc {
 0x12016e76 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c6a4u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if !cpu.zf { pc = 0x12016f21; continue; }
  pc = 0x12016e87; continue;
 }
 0x12016e87 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041104u32); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12035e3cu32; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x12016ea1); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12016ef7; continue; }
  pc = 0x12016ea5; continue;
 }
 0x12016ea5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12016eaa; continue;
 }
 0x12016eaa => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203c6a4u32, v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12016ecc; continue; }
  pc = 0x12016eb7; continue;
 }
 0x12016eb7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12016ec2); f_1201708a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203c6a4u32); cpu.eax = v; }
  pc = 0x12016ecc; continue;
 }
 0x12016ecc => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203c6a4u32, v); }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x12016f29; continue; }
  pc = 0x12016ed6; continue;
 }
 0x12016ed6 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x12016ef2); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016ef7 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12035e38u32; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041108u32); cpu.push(0x12016f0f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12016f1a; continue; }
  pc = 0x12016f13; continue;
 }
 0x12016f13 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x12016eaa; continue;
 }
 0x12016f1a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016f21 => {
  { let v: u32 = cpu.r32(0x12041104u32); cpu.esi = v; }
  pc = 0x12016eaa; continue;
 }
 0x12016f29 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203c6a4u32, v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x12017066; continue; }
  pc = 0x12016f37; continue;
 }
 0x12016f37 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), cpu.esi); }
  if !cpu.zf { pc = 0x12016f48; continue; }
  pc = 0x12016f3f; continue;
 }
 0x12016f3f => {
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  pc = 0x12016f48; continue;
 }
 0x12016f48 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12016f66); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x12016f73; continue; }
  pc = 0x12016f6c; continue;
 }
 0x12016f6c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016f73 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x12016f79); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x12016f89; continue; }
  pc = 0x12016f82; continue;
 }
 0x12016f82 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12016f89 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12016fa5); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12017052; continue; }
  pc = 0x12016fad; continue;
 }
 0x12016fad => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041108u32); cpu.push(0x12016fc3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12017052; continue; }
  pc = 0x12016fcd; continue;
 }
 0x12016fcd => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12016fd3); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12017052; continue; }
  pc = 0x12016fdc; continue;
 }
 0x12016fdc => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041108u32); cpu.push(0x12016ff0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12017052; continue; }
  pc = 0x12016ff4; continue;
 }
 0x12016ff4 => {
  { cpu.and8(cpu.r8(0x19u32.wrapping_add(cpu.esp)), 0x4u8); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  if cpu.zf { pc = 0x12017033; continue; }
  pc = 0x12016fff; continue;
 }
 0x12016fff => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1201701a; continue; }
  pc = 0x12017003; continue;
 }
 0x12017003 => {
  { cpu.sub32(cpu.ecx, cpu.edi); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  if (cpu.sf != cpu.of) { pc = 0x1201700f; continue; }
  pc = 0x1201700d; continue;
 }
 0x1201700d => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x1201700f; continue;
 }
 0x1201700f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017017); f_12003cd6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1201701a; continue;
 }
 0x1201701a => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12017020); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12017029); f_12005adb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017033 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1201706b; continue; }
  pc = 0x12017037; continue;
 }
 0x12017037 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041158u32); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201704c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201701a; continue; }
  pc = 0x12017052; continue;
 }
 0x12017052 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12017058); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12017061); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x12017066; continue;
 }
 0x12017066 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201706b => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041158u32); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x12017082); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201701a; continue; }
  pc = 0x12017088; continue;
 }
 0x12017088 => {
  pc = 0x12017052; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201708a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201708a); }
 let mut pc: u32 = 0x1201708a;
 loop { match pc {
 0x1201708a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  if cpu.zf { pc = 0x120170ad; continue; }
  pc = 0x1201709d; continue;
 }
 0x1201709d => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x120170b3; continue; }
  pc = 0x120170a3; continue;
 }
 0x120170a3 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201709d; continue; }
  pc = 0x120170ad; continue;
 }
 0x120170ad => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x120170ba; continue; }
  pc = 0x120170b3; continue;
 }
 0x120170b3 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x120170ba; continue;
 }
 0x120170ba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120170bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120170bd); }
 let mut pc: u32 = 0x120170bd;
 loop { match pc {
 0x120170bd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c6a8u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if !cpu.zf { pc = 0x12017168; continue; }
  pc = 0x120170ce; continue;
 }
 0x120170ce => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041108u32); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12035e38u32; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x120170e8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201713e; continue; }
  pc = 0x120170ec; continue;
 }
 0x120170ec => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x120170f1; continue;
 }
 0x120170f1 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203c6a8u32, v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12017113; continue; }
  pc = 0x120170fe; continue;
 }
 0x120170fe => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017109); f_120172e7(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203c6a8u32); cpu.eax = v; }
  pc = 0x12017113; continue;
 }
 0x12017113 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203c6a8u32, v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x12017170; continue; }
  pc = 0x1201711d; continue;
 }
 0x1201711d => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x12017139); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201713e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12035e3cu32; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041104u32); cpu.push(0x12017156); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12017161; continue; }
  pc = 0x1201715a; continue;
 }
 0x1201715a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x120170f1; continue;
 }
 0x12017161 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017168 => {
  { let v: u32 = cpu.r32(0x12041108u32); cpu.esi = v; }
  pc = 0x120170f1; continue;
 }
 0x12017170 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203c6a8u32, v); }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x1201724c; continue; }
  pc = 0x1201717e; continue;
 }
 0x1201717e => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), cpu.esi); }
  if !cpu.zf { pc = 0x1201718f; continue; }
  pc = 0x12017186; continue;
 }
 0x12017186 => {
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  pc = 0x1201718f; continue;
 }
 0x1201718f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x120171a6); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x120171b3; continue; }
  pc = 0x120171ac; continue;
 }
 0x120171ac => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120171b3 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120171c0); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x120171d0; continue; }
  pc = 0x120171c9; continue;
 }
 0x120171c9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120171d0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x120171e5); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12017238; continue; }
  pc = 0x120171e9; continue;
 }
 0x120171e9 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041104u32); cpu.push(0x120171ff); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12017238; continue; }
  pc = 0x12017205; continue;
 }
 0x12017205 => {
  { cpu.and8(cpu.r8(0x19u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x12017251; continue; }
  pc = 0x1201720c; continue;
 }
 0x1201720c => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120172a9; continue; }
  pc = 0x12017218; continue;
 }
 0x12017218 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x12017238; continue; }
  pc = 0x1201721c; continue;
 }
 0x1201721c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x12041104u32); cpu.push(0x12017234); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120172a9; continue; }
  pc = 0x12017238; continue;
 }
 0x12017238 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201723e); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12017247); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1201724c; continue;
 }
 0x1201724c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017251 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201725e); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12017238; continue; }
  pc = 0x12017267; continue;
 }
 0x12017267 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041104u32); cpu.push(0x1201727b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12017238; continue; }
  pc = 0x1201727f; continue;
 }
 0x1201727f => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120172c2; continue; }
  pc = 0x1201728b; continue;
 }
 0x1201728b => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041140u32); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x120172a3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12017238; continue; }
  pc = 0x120172a9; continue;
 }
 0x120172a9 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x120172af); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120172b8); f_12005adb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120172c2 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12041140u32); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x120172dc); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x120172a9; continue; }
  pc = 0x120172e2; continue;
 }
 0x120172e2 => {
  pc = 0x12017238; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120172e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120172e7); }
 let mut pc: u32 = 0x120172e7;
 loop { match pc {
 0x120172e7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  if cpu.zf { pc = 0x12017307; continue; }
  pc = 0x120172fa; continue;
 }
 0x120172fa => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x1201730c; continue; }
  pc = 0x120172ff; continue;
 }
 0x120172ff => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x120172fa; continue; }
  pc = 0x12017307; continue;
 }
 0x12017307 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x12017310; continue; }
  pc = 0x1201730c; continue;
 }
 0x1201730c => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x12017310; continue;
 }
 0x12017310 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12017313(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12017313); }
 let mut pc: u32 = 0x12017313;
 loop { match pc {
 0x12017313 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x63u32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x12017322; continue;
 }
 0x12017322 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1203c6f8u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201733b); f_120156c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201735b; continue; }
  pc = 0x12017342; continue;
 }
 0x12017342 => {
  if (cpu.sf == cpu.of) { pc = 0x12017357; continue; }
  pc = 0x12017344; continue;
 }
 0x12017344 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x12017346; continue;
 }
 0x12017346 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12017322; continue; }
  pc = 0x1201734f; continue;
 }
 0x1201734f => {
  { let v: u16 = cpu.xor16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017357 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x12017346; continue;
 }
 0x1201735b => {
  { let v: u16 = cpu.r16(0x1203c6fcu32.wrapping_add(cpu.ebp.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12017368(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12017368); }
 let mut pc: u32 = 0x12017368;
 loop { match pc {
 0x12017368 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x57u32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x12017377; continue;
 }
 0x12017377 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1203ca10u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12017390); f_120156c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120173b0; continue; }
  pc = 0x12017397; continue;
 }
 0x12017397 => {
  if (cpu.sf == cpu.of) { pc = 0x120173ac; continue; }
  pc = 0x12017399; continue;
 }
 0x12017399 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x1201739b; continue;
 }
 0x1201739b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12017377; continue; }
  pc = 0x120173a4; continue;
 }
 0x120173a4 => {
  { let v: u16 = cpu.xor16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120173ac => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x1201739b; continue;
 }
 0x120173b0 => {
  { let v: u16 = cpu.r16(0x1203ca14u32.wrapping_add(cpu.ebp.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120173bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120173bd); }
 let mut pc: u32 = 0x120173bd;
 loop { match pc {
 0x120173bd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), 0x64u16); }
  if (cpu.cf || cpu.zf) { pc = 0x120173d7; continue; }
  pc = 0x120173c9; continue;
 }
 0x120173c9 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120173cf); f_120173ff(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x120173fd; continue; }
  pc = 0x120173d7; continue;
 }
 0x120173d7 => {
  { let v: u16 = 0x64u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { cpu.sub16((cpu.edx as u16), 0x5au16); }
  if (cpu.cf || cpu.zf) { pc = 0x120173ef; continue; }
  pc = 0x120173ea; continue;
 }
 0x120173ea => {
  { let v: u16 = cpu.xor16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120173ef => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1203ccc8u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x120173fd; continue;
 }
 0x120173fd => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120173ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120173ff); }
 let mut pc: u32 = 0x120173ff;
 loop { match pc {
 0x120173ff => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x354u32); }
  if cpu.zf { pc = 0x12017413; continue; }
  pc = 0x1201740f; continue;
 }
 0x1201740f => {
  { let v: u16 = cpu.xor16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017413 => {
  { let v: u16 = 0xc04u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12017418(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12017418); }
 let mut pc: u32 = 0x12017418;
 loop { match pc {
 0x12017418 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = 0x64u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.esi as u16); cpu.div16(v); }
  { cpu.sub16((cpu.edx as u16), 0x5au16); }
  if (cpu.cf || cpu.zf) { pc = 0x12017441; continue; }
  pc = 0x1201743a; continue;
 }
 0x1201743a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017441 => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x1203ccc8u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  pc = 0x12017450; continue;
 }
 0x12017450 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x1201746e; continue; }
  pc = 0x12017458; continue;
 }
 0x12017458 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16) ^ (cpu.edi as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.ebp as u16), 0x3ffu16); }
  if cpu.zf { pc = 0x12017488; continue; }
  pc = 0x12017465; continue;
 }
 0x12017465 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x3u32); }
  if cpu.cf { pc = 0x12017450; continue; }
  pc = 0x1201746e; continue;
 }
 0x1201746e => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x1203ccc8u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017488 => {
  { let v: u32 = (cpu.edi as u16) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xfffffc00u32); }
  if !cpu.zf { pc = 0x12017499; continue; }
  pc = 0x12017492; continue;
 }
 0x12017492 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12017499; continue;
 }
 0x12017499 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120174a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120174a6); }
 let mut pc: u32 = 0x120174a6;
 loop { match pc {
 0x120174a6 => {
  { let v: u32 = cpu.r32(0x1203cf28u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120174ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120174ac); }
 let mut pc: u32 = 0x120174ac;
 loop { match pc {
 0x120174ac => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.eax = v; }
  pc = 0x120174c0; continue;
 }
 0x120174c0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x120174cd; continue; }
  pc = 0x120174c6; continue;
 }
 0x120174c6 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let index = cpu.eax; let at = (0x0u32.wrapping_add(cpu.esp)).wrapping_add((((index as i32) >> 5) as u32).wrapping_mul(4)); let bit = index & 31; let a = cpu.r32(at); cpu.cf = (a >> bit) & 1 != 0; cpu.w32(at, a | (1 << bit)); }
  pc = 0x120174c0; continue;
 }
 0x120174cd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx | 0xffffffffu32; cpu.ecx = v; }
  {  }
  pc = 0x120174d4; continue;
 }
 0x120174d4 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x120174e2; continue; }
  pc = 0x120174db; continue;
 }
 0x120174db => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let index = cpu.eax; let at = (0x0u32.wrapping_add(cpu.esp)).wrapping_add((((index as i32) >> 5) as u32).wrapping_mul(4)); let bit = index & 31; let a = cpu.r32(at); cpu.cf = (a >> bit) & 1 != 0;  }
  if !cpu.cf { pc = 0x120174d4; continue; }
  pc = 0x120174e2; continue;
 }
 0x120174e2 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120174ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120174ea); }
 let mut pc: u32 = 0x120174ea;
 loop { match pc {
 0x120174ea => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  if cpu.ecx == 0 { pc = 0x1201751b; continue; }
  pc = 0x120174f5; continue;
 }
 0x120174f5 => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.cmps(1, 1); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0xffffffffu32.wrapping_add(cpu.edi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12017519; continue; }
  pc = 0x12017515; continue;
 }
 0x12017515 => {
  if cpu.zf { pc = 0x1201751b; continue; }
  pc = 0x12017517; continue;
 }
 0x12017517 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x12017519; continue;
 }
 0x12017519 => {
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  pc = 0x1201751b; continue;
 }
 0x1201751b => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12017522(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12017522); }
 let mut pc: u32 = 0x12017522;
 loop { match pc {
 0x12017522 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203cfdcu32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x7u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  pc = 0x1201753c; continue;
 }
 0x1201753c => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x1201753c; continue; }
  pc = 0x1201756c; continue;
 }
 0x1201756c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12017573); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120175fb; continue; }
  pc = 0x1201757e; continue;
 }
 0x1201757e => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.ebp = v; }
  pc = 0x12017585; continue;
 }
 0x12017585 => {
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
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
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x12017585; continue; }
  pc = 0x120175f8; continue;
 }
 0x120175f8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x120175fb; continue;
 }
 0x120175fb => {
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
pub fn f_12017607(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12017607); }
 let mut pc: u32 = 0x12017607;
 loop { match pc {
 0x12017607 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203cfdcu32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x68u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  pc = 0x12017621; continue;
 }
 0x12017621 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x12017621; continue; }
  pc = 0x12017651; continue;
 }
 0x12017651 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12017658); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120176ea; continue; }
  pc = 0x12017667; continue;
 }
 0x12017667 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v: u32 = 0xcu32; cpu.edx = v; }
  pc = 0x12017671; continue;
 }
 0x12017671 => {
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x12017671; continue; }
  pc = 0x120176e6; continue;
 }
 0x120176e6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120176ea; continue;
 }
 0x120176ea => {
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
pub fn f_120176f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120176f6); }
 let mut pc: u32 = 0x120176f6;
 loop { match pc {
 0x120176f6 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203cfdcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  pc = 0x12017710; continue;
 }
 0x12017710 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x12017710; continue; }
  pc = 0x1201773d; continue;
 }
 0x1201773d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xcu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x38u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x1201774f; continue;
 }
 0x1201774f => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x1201774f; continue; }
  pc = 0x1201777c; continue;
 }
 0x1201777c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xa8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xacu32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017802); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12017a41; continue; }
  pc = 0x12017811; continue;
 }
 0x12017811 => {
  { let v: u32 = 0xacu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203cfdcu32); cpu.esi = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x12017836; continue;
 }
 0x12017836 => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x12017836; continue; }
  pc = 0x120178a9; continue;
 }
 0x120178a9 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x38u32); cpu.ebp = v; }
  pc = 0x120178bc; continue;
 }
 0x120178bc => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x120178bc; continue; }
  pc = 0x1201792f; continue;
 }
 0x1201792f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x98u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x9cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xa0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xa4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xa8u32.wrapping_add(cpu.eax), v); }
  pc = 0x12017a41; continue;
 }
 0x12017a41 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12017a4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12017a4d); }
 let mut pc: u32 = 0x12017a4d;
 loop { match pc {
 0x12017a4d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017a68); f_12017a6c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12017a6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12017a6c); }
 let mut pc: u32 = 0x12017a6c;
 loop { match pc {
 0x12017a6c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12017a81; continue; }
  pc = 0x12017a7b; continue;
 }
 0x12017a7b => {
  { let v: u32 = cpu.r32(0x1203cfdcu32); cpu.edi = v; }
  pc = 0x12017a81; continue;
 }
 0x12017a81 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x12017b3b; continue; }
  pc = 0x12017a96; continue;
 }
 0x12017a96 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12017a9d); f_1200420b(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12017aa8; continue;
 }
 0x12017aa8 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x12017b0c; continue; }
  pc = 0x12017aaf; continue;
 }
 0x12017aaf => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x12017abc; continue;
 }
 0x12017abc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12017b0c; continue; }
  pc = 0x12017ac5; continue;
 }
 0x12017ac5 => {
  { cpu.sub32(cpu.ecx, 0x25u32); }
  if cpu.zf { pc = 0x12017b4e; continue; }
  pc = 0x12017ace; continue;
 }
 0x12017ace => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x12017af4; continue; }
  pc = 0x12017adf; continue;
 }
 0x12017adf => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), cpu.ebp); }
  if (cpu.cf || cpu.zf) { pc = 0x12017af4; continue; }
  pc = 0x12017ae5; continue;
 }
 0x12017ae5 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.esp))); cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x12017af4; continue;
 }
 0x12017af4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.esp))); cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x12017b05; continue;
 }
 0x12017b05 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12017abc; continue; }
  pc = 0x12017b0c; continue;
 }
 0x12017b0c => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12017b7f; continue; }
  pc = 0x12017b13; continue;
 }
 0x12017b13 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12017b1a); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12017b1d; continue;
 }
 0x12017b1d => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x12017b87; continue; }
  pc = 0x12017b24; continue;
 }
 0x12017b24 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017b3b => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12017aa8; continue;
 }
 0x12017b4e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1203cfe0u32, v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x23u8); }
  if !cpu.zf { pc = 0x12017b65; continue; }
  pc = 0x12017b5e; continue;
 }
 0x12017b5e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1203cfe0u32, v); }
  pc = 0x12017b65; continue;
 }
 0x12017b65 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12017b7a); f_12017b91(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x12017b05; continue;
 }
 0x12017b7f => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12017b1d; continue;
 }
 0x12017b87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12017b91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12017b91); }
 let mut pc: u32 = 0x12017b91;
 loop { match pc {
 0x12017b91 => {
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.esp)) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x25u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x55u32); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12018067; continue; }
  pc = 0x12017ba6; continue;
 }
 0x12017ba6 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x120180ccu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1201806cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x12017bb5 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u8 = 0x25u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017bcb => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017bea); f_12018122(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017bf2 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017c11); f_12018122(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017c19 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017c3c); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017c44 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x12017c63; continue; }
  pc = 0x12017c61; continue;
 }
 0x12017c61 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  pc = 0x12017c63; continue;
 }
 0x12017c63 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12017c75); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017c7d => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017ca0); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017ca8 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017ccb); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017cd3 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x12017ce4; continue;
 }
 0x12017ce4 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x12017d26; continue; }
  pc = 0x12017ceb; continue;
 }
 0x12017ceb => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x12017ced; continue;
 }
 0x12017ced => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12017cff); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017d07 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x12017d23; continue; }
  pc = 0x12017d1c; continue;
 }
 0x12017d1c => {
  { let v: u32 = 0x6u32; cpu.edi = v; }
  pc = 0x12017ce4; continue;
 }
 0x12017d23 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x12017ce4; continue;
 }
 0x12017d26 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebp; cpu.idiv32(v); }
  { cpu.sub32(cpu.edx, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x12017ced; continue; }
  pc = 0x12017d3d; continue;
 }
 0x12017d3d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x12017ced; continue;
 }
 0x12017d40 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1203cfe0u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017d6a); f_12018232(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017d72 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.ecx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017db8); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017dc0 => {
  cpu.push(0x12017dc5); f_120029c0(cpu);
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c504u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017de6); f_12018122(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017dee => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017e0c); f_12018122(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017e14 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017e33); f_12018122(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017e3b => {
  { cpu.sub32(cpu.r32(0x1203cfe0u32), 0x0u32); }
  if cpu.zf { pc = 0x12017e9b; continue; }
  pc = 0x12017e44; continue;
 }
 0x12017e44 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1203cfe0u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017e6e); f_12018232(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x12018067; continue; }
  pc = 0x12017e7a; continue;
 }
 0x12017e7a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xa8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017e93); f_12018232(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017e9b => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017ebb); f_12018232(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x12018067; continue; }
  pc = 0x12017ec7; continue;
 }
 0x12017ec7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xa8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12017ee0); f_12018232(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017ee8 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017f0b); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017f13 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017f37); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017f3f => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017f63); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017f6b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0xbu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12017f97; continue; }
  pc = 0x12017f83; continue;
 }
 0x12017f83 => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017f8f); f_12018122(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017f97 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017fa3); f_12018122(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017fab => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12017fce); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12017fd6 => {
  { cpu.sub32(cpu.r32(0x1203cfe0u32), 0x0u32); }
  if cpu.zf { pc = 0x12018011; continue; }
  pc = 0x12017fdf; continue;
 }
 0x12017fdf => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1203cfe0u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018009); f_12018232(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018011 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018031); f_12018232(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018039 => {
  { let v: u32 = cpu.r32(0x1203cfe0u32); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1203cfe4u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12018064); f_1201814c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  pc = 0x12018067; continue;
 }
 0x12018067 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201806c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201806c); }
 let mut pc: u32 = 0x1201806c;
 loop { match pc {
 0x1201806c => {
  { let v: u8 = 0x7bu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x12018070, "Retf");
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120180cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120180cc); }
 let mut pc: u32 = 0x120180cc;
 loop { match pc {
 0x120180cc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.eax.wrapping_add(0x17171717u32); cpu.eax = v; }
  { cpu.pop(); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) | (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0xdu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(0x2b); }
  cpu.unimplemented(0x1201810b, "Movhps");
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.pop(); }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.pop(); }
  { let v: u8 = cpu.adc8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.r32(0x8b565316u32.wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.edx = v; }
  if cpu.zf { pc = 0x1201814b; continue; }
  pc = 0x12018127; continue;
 }
 0x12018127 => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x83u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffff8bu32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x12018135; continue;
 }
 0x12018135 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x12018149; continue; }
  pc = 0x1201813b; continue;
 }
 0x1201813b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x12018135; continue; }
  pc = 0x12018149; continue;
 }
 0x12018149 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x1201814b; continue;
 }
 0x1201814b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018122(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018122); }
 let mut pc: u32 = 0x12018122;
 loop { match pc {
 0x12018122 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x12018149; continue; }
  pc = 0x1201812d; continue;
 }
 0x1201812d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x12018135; continue;
 }
 0x12018135 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x12018149; continue; }
  pc = 0x1201813b; continue;
 }
 0x1201813b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x12018135; continue; }
  pc = 0x12018149; continue;
 }
 0x12018149 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201814c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201814c); }
 let mut pc: u32 = 0x1201814c;
 loop { match pc {
 0x1201814c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1203cfe4u32), cpu.edi); }
  if cpu.zf { pc = 0x12018176; continue; }
  pc = 0x1201815a; continue;
 }
 0x1201815a => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201816e); f_120181d4(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018176 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x120181c5; continue; }
  pc = 0x12018182; continue;
 }
 0x12018182 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x120181bf; continue; }
  pc = 0x12018188; continue;
 }
 0x12018188 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x12018193; continue;
 }
 0x12018193 => {
  { let v: u32 = 0xau32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebp; cpu.idiv32(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebp; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12018193; continue; }
  pc = 0x120181b2; continue;
 }
 0x120181b2 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120181bf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  pc = 0x120181b2; continue;
 }
 0x120181c5 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120181d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120181d4); }
 let mut pc: u32 = 0x120181d4;
 loop { match pc {
 0x120181d4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x12018216; continue; }
  pc = 0x120181e7; continue;
 }
 0x120181e7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x120181eb; continue;
 }
 0x120181eb => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0xau32; cpu.ebp = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.esi), v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebp; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018216; continue; }
  pc = 0x12018211; continue;
 }
 0x12018211 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120181eb; continue; }
  pc = 0x12018216; continue;
 }
 0x12018216 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  pc = 0x1201821f; continue;
 }
 0x1201821f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if cpu.cf { pc = 0x1201821f; continue; }
  pc = 0x1201822d; continue;
 }
 0x1201822d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018232(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018232); }
 let mut pc: u32 = 0x12018232;
 loop { match pc {
 0x12018232 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x120182fd; continue; }
  pc = 0x12018246; continue;
 }
 0x12018246 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x1201824e; continue;
 }
 0x1201824e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x120182fd; continue; }
  pc = 0x12018258; continue;
 }
 0x12018258 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1203cfe4u32, v); }
  pc = 0x12018272; continue;
 }
 0x12018272 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x12018272; continue; }
  pc = 0x12018282; continue;
 }
 0x12018282 => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x27u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x52u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12018326; continue; }
  pc = 0x12018292; continue;
 }
 0x12018292 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x120184e4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = cpu.r32(0x120184b8u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x120182af => {
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x12018308; continue; }
  pc = 0x120182b4; continue;
 }
 0x120182b4 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x120182fd; continue; }
  pc = 0x120182bb; continue;
 }
 0x120182bb => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x120182f4; continue; }
  pc = 0x120182c1; continue;
 }
 0x120182c1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x27u8); }
  if cpu.zf { pc = 0x12018305; continue; }
  pc = 0x120182c8; continue;
 }
 0x120182c8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x120182e3; continue; }
  pc = 0x120182d9; continue;
 }
 0x120182d9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120182e3; continue;
 }
 0x120182e3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x120182bb; continue; }
  pc = 0x120182f4; continue;
 }
 0x120182f4 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1201824e; continue; }
  pc = 0x120182fd; continue;
 }
 0x120182fd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018305 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x120182f4; continue;
 }
 0x12018308 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x120182f4; continue;
 }
 0x1201830c => {
  { let v = 0x12036500u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12018317); f_120156c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201834e; continue; }
  pc = 0x1201831e; continue;
 }
 0x1201831e => {
  { let v: u8 = 0x70u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x5u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x12018326; continue;
 }
 0x12018326 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esp)), 0x0u8); }
  if cpu.zf { pc = 0x12018480; continue; }
  pc = 0x12018331; continue;
 }
 0x12018331 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x13u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12018349); f_12017b91(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x120182f4; continue;
 }
 0x1201834e => {
  { let v = 0x120364fcu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12018359); f_120156c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12018363; continue; }
  pc = 0x12018360; continue;
 }
 0x12018360 => {
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x12018363; continue;
 }
 0x12018363 => {
  { let v: u8 = 0x70u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x1201836a => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x1201837b; continue; }
  pc = 0x1201836f; continue;
 }
 0x1201836f => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x12018326; continue; }
  pc = 0x12018374; continue;
 }
 0x12018374 => {
  { let v: u8 = 0x48u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x1201837b => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203cfe4u32, v); }
  pc = 0x12018374; continue;
 }
 0x12018387 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12018326; continue; }
  pc = 0x1201838d; continue;
 }
 0x1201838d => {
  pc = cpu.r32(0x12018538u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x12018394 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203cfe4u32, v); }
  pc = 0x1201839e; continue;
 }
 0x1201839e => {
  { let v: u8 = 0x6du8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x120183a5 => {
  { let v: u8 = 0x62u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x120183af => {
  { let v: u8 = 0x42u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x120183b9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12018326; continue; }
  pc = 0x120183c3; continue;
 }
 0x120183c3 => {
  pc = cpu.r32(0x12018548u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x120183ca => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203cfe4u32, v); }
  pc = 0x120183d4; continue;
 }
 0x120183d4 => {
  { let v: u8 = 0x64u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x120183de => {
  { let v: u8 = 0x61u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x120183e8 => {
  { let v: u8 = 0x41u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x120183f2 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x1201840a; continue; }
  pc = 0x120183f7; continue;
 }
 0x120183f7 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x12018326; continue; }
  pc = 0x12018400; continue;
 }
 0x12018400 => {
  { let v: u8 = 0x49u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x1201840a => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203cfe4u32, v); }
  pc = 0x12018400; continue;
 }
 0x12018416 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x1201842e; continue; }
  pc = 0x1201841b; continue;
 }
 0x1201841b => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x12018326; continue; }
  pc = 0x12018424; continue;
 }
 0x12018424 => {
  { let v: u8 = 0x4du8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x1201842e => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203cfe4u32, v); }
  pc = 0x12018424; continue;
 }
 0x1201843a => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x12018452; continue; }
  pc = 0x1201843f; continue;
 }
 0x1201843f => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x12018326; continue; }
  pc = 0x12018448; continue;
 }
 0x12018448 => {
  { let v: u8 = 0x53u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x12018452 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1203cfe4u32, v); }
  pc = 0x12018448; continue;
 }
 0x1201845e => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x12018476; continue; }
  pc = 0x12018463; continue;
 }
 0x12018463 => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { pc = 0x12018326; continue; }
  pc = 0x1201846c; continue;
 }
 0x1201846c => {
  { let v: u8 = 0x59u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x12018476 => {
  { let v: u8 = 0x79u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018326; continue;
 }
 0x12018480 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x120184a0; continue; }
  pc = 0x12018493; continue;
 }
 0x12018493 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120184a0; continue;
 }
 0x120184a0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x120182f4; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120184b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120184b8); }
 let mut pc: u32 = 0x120184b8;
 loop { match pc {
 0x120184b8 => {
  { cpu.scas(4, 0); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | 0x83u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = 0xffffff83u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let a = cpu.r32(0x830c1201u32.wrapping_add(cpu.ebx)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x830c1201u32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xf2120183u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.push(0x2b); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0xffffff84u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(0x12u32); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
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
pub fn f_12018558(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018558); }
 let mut pc: u32 = 0x12018558;
 loop { match pc {
 0x12018558 => {
  { let v = 0xbu32; cpu.push(v); }
  cpu.push(0x1201855f); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x12018567); f_120028a4(cpu);
  { let v = 0xbu32; cpu.push(v); }
  cpu.push(0x1201856e); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018572(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018572); }
 let mut pc: u32 = 0x12018572;
 loop { match pc {
 0x12018572 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x1201857f; continue;
 }
 0x1201857f => {
  { cpu.sub32(cpu.r32(0x1203befcu32), cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201859d; continue; }
  pc = 0x12018587; continue;
 }
 0x12018587 => {
  { let v = 0x8u32; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018593); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12018596; continue;
 }
 0x12018596 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120185b2; continue; }
  pc = 0x1201859a; continue;
 }
 0x1201859a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1201857f; continue;
 }
 0x1201859d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  pc = 0x12018596; continue;
 }
 0x120185b2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if cpu.zf { pc = 0x120185c3; continue; }
  pc = 0x120185be; continue;
 }
 0x120185be => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if !cpu.zf { pc = 0x120185c8; continue; }
  pc = 0x120185c3; continue;
 }
 0x120185c3 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x120185c8; continue;
 }
 0x120185c8 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x120185ca; continue;
 }
 0x120185ca => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120185f2; continue; }
  pc = 0x120185d3; continue;
 }
 0x120185d3 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x120185db); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x120185de; continue;
 }
 0x120185de => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12018603; continue; }
  pc = 0x120185e2; continue;
 }
 0x120185e2 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x120185ca; continue;
 }
 0x120185f2 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  pc = 0x120185de; continue;
 }
 0x12018603 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x2du32); }
  if !cpu.zf { pc = 0x12018611; continue; }
  pc = 0x1201860a; continue;
 }
 0x1201860a => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018611 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
