#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_004ae886(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae886); }
 let mut pc: u32 = 0x4ae886;
 loop { match pc {
 0x4ae886 => {
  pc = cpu.r32(0x525618u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae890); }
 let mut pc: u32 = 0x4ae890;
 loop { match pc {
 0x4ae890 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4b3c10u32; cpu.push(v); }
  { let v = 0x4ae7b0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xffffff98u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x2u32; cpu.push(v); }
  let t = cpu.r32(0x525674u32); cpu.push(0x4ae8c5); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x524330u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x524334u32, v); }
  let t = cpu.r32(0x525670u32); cpu.push(0x4ae8e2); cpu.call(t);
  { let v: u32 = cpu.r32(0x524304u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  let t = cpu.r32(0x52566cu32); cpu.push(0x4ae8f0); cpu.call(t);
  { let v: u32 = cpu.r32(0x524300u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x525668u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52432cu32, v); }
  cpu.push(0x4ae90a); f_00453e00(cpu);
  { let v: u32 = cpu.r32(0x4f8430u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ae921; continue; }
  pc = 0x4ae913; continue;
 }
 0x4ae913 => {
  { let v = 0x4aec90u32; cpu.push(v); }
  let t = cpu.r32(0x525664u32); cpu.push(0x4ae91e); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae921; continue;
 }
 0x4ae921 => {
  cpu.push(0x4ae926); f_004aec70(cpu);
  { let v = 0x4bf044u32; cpu.push(v); }
  { let v = 0x4bf040u32; cpu.push(v); }
  cpu.push(0x4ae935); f_004aec68(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5242fcu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff94u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5242f8u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x52565cu32); cpu.push(0x4ae95e); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x4bf03cu32; cpu.push(v); }
  { let v = 0x4bf000u32; cpu.push(v); }
  cpu.push(0x4ae970); f_004aec68(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  let t = cpu.r32(0x525658u32); cpu.push(0x4ae979); cpu.call(t);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if !cpu.zf { pc = 0x4aea2f; continue; }
  pc = 0x4ae987; continue;
 }
 0x4ae987 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4ae995; continue; }
  pc = 0x4ae991; continue;
 }
 0x4ae991 => {
  { cpu.sub8((cpu.eax as u8), 0x22u8); }
  if !cpu.zf { pc = 0x4ae987; continue; }
  pc = 0x4ae995; continue;
 }
 0x4ae995 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x22u8); }
  if !cpu.zf { pc = 0x4ae99e; continue; }
  pc = 0x4ae99a; continue;
 }
 0x4ae99a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae99e; continue;
 }
 0x4ae99e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4ae9ae; continue; }
  pc = 0x4ae9a4; continue;
 }
 0x4ae9a4 => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4ae9ae; continue; }
  pc = 0x4ae9a8; continue;
 }
 0x4ae9a8 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae99e; continue;
 }
 0x4ae9ae => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffa4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x52557cu32); cpu.push(0x4ae9bf); cpu.call(t);
  { cpu.and8(cpu.r8(0xffffffd0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae9cf; continue; }
  pc = 0x4ae9c5; continue;
 }
 0x4ae9c5 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x4ae9d4; continue;
 }
 0x4ae9cf => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  pc = 0x4ae9d4; continue;
 }
 0x4ae9d4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x525580u32); cpu.push(0x4ae9e0); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae9e6); f_004494c0(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525654u32); cpu.push(0x4ae9f0); cpu.call(t);
  pc = 0x4aea14; continue;
 }
 0x4aea14 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4aea2f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x20u8); }
  if (cpu.cf || cpu.zf) { pc = 0x4ae99e; continue; }
  pc = 0x4aea38; continue;
 }
 0x4aea38 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aea2f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae9f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae9f2); }
 let mut pc: u32 = 0x4ae9f2;
 loop { match pc {
 0x4ae9f2 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4aea03); f_004aec62(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aea07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea07); }
 let mut pc: u32 = 0x4aea07;
 loop { match pc {
 0x4aea07 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x5256a0u32); cpu.push(0x4aea14); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
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
pub fn f_004aea40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea40); }
 let mut pc: u32 = 0x4aea40;
 loop { match pc {
 0x4aea40 => {
  pc = cpu.r32(0x525614u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aea46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea46); }
 let mut pc: u32 = 0x4aea46;
 loop { match pc {
 0x4aea46 => {
  pc = cpu.r32(0x525694u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aea4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea4c); }
 let mut pc: u32 = 0x4aea4c;
 loop { match pc {
 0x4aea4c => {
  pc = cpu.r32(0x525688u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aea52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea52); }
 let mut pc: u32 = 0x4aea52;
 loop { match pc {
 0x4aea52 => {
  pc = cpu.r32(0x52568cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aea58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea58); }
 let mut pc: u32 = 0x4aea58;
 loop { match pc {
 0x4aea58 => {
  pc = cpu.r32(0x525690u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aea5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea5e); }
 let mut pc: u32 = 0x4aea5e;
 loop { match pc {
 0x4aea5e => {
  pc = cpu.r32(0x52564cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aea64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea64); }
 let mut pc: u32 = 0x4aea64;
 loop { match pc {
 0x4aea64 => {
  pc = cpu.r32(0x525634u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aea70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aea70); }
 let mut pc: u32 = 0x4aea70;
 loop { match pc {
 0x4aea70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4b3c20u32; cpu.push(v); }
  { let v = 0x4ae7b0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x4aeaa4; continue;
 }
 0x4aeaa4 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x4aeab9; continue; }
  pc = 0x4aeaa9; continue;
 }
 0x4aeaa9 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x4aeaae); cpu.call(t);
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aeaa4; continue;
 }
 0x4aeab9 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aeacc); f_004aeae8(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aeadf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aeadf); }
 let mut pc: u32 = 0x4aeadf;
 loop { match pc {
 0x4aeadf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  f_004aeae8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aeae8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aeae8); }
 let mut pc: u32 = 0x4aeae8;
 loop { match pc {
 0x4aeae8 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4aeafb; continue; }
  pc = 0x4aeaef; continue;
 }
 0x4aeaef => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4aeafb); f_004aebc0(cpu);
  pc = 0x4aeafb; continue;
 }
 0x4aeafb => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aeb10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aeb10); }
 let mut pc: u32 = 0x4aeb10;
 loop { match pc {
 0x4aeb10 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4b3c30u32; cpu.push(v); }
  { let v = 0x4ae7b0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aeb4c; continue;
 }
 0x4aeb4c => {
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if cpu.sf { pc = 0x4aeb5d; continue; }
  pc = 0x4aeb51; continue;
 }
 0x4aeb51 => {
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x4aeb5b); cpu.call(t);
  pc = 0x4aeb4c; continue;
 }
 0x4aeb5d => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aeb70); f_004aeb89(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aeb83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aeb83); }
 let mut pc: u32 = 0x4aeb83;
 loop { match pc {
 0x4aeb83 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  f_004aeb89(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aeb89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aeb89); }
 let mut pc: u32 = 0x4aeb89;
 loop { match pc {
 0x4aeb89 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4aeb9f; continue; }
  pc = 0x4aeb90; continue;
 }
 0x4aeb90 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4aeb9f); f_004aebc0(cpu);
  pc = 0x4aeb9f; continue;
 }
 0x4aeb9f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aebc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aebc0); }
 let mut pc: u32 = 0x4aebc0;
 loop { match pc {
 0x4aebc0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4b3c40u32; cpu.push(v); }
  { let v = 0x4ae7b0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aebed; continue;
 }
 0x4aebed => {
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if cpu.sf { pc = 0x4aec10; continue; }
  pc = 0x4aebf2; continue;
 }
 0x4aebf2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x4aebfe); cpu.call(t);
  pc = 0x4aebed; continue;
 }
 0x4aec10 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec00); }
 let mut pc: u32 = 0x4aec00;
 loop { match pc {
 0x4aec00 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aec09); f_004aec30(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec0d); }
 let mut pc: u32 = 0x4aec0d;
 loop { match pc {
 0x4aec0d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec30); }
 let mut pc: u32 = 0x4aec30;
 loop { match pc {
 0x4aec30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0xe06d7363u32); }
  if !cpu.zf { pc = 0x4aec43; continue; }
  pc = 0x4aec3e; continue;
 }
 0x4aec3e => {
  cpu.push(0x4aec43); f_004aeca0(cpu);
  pc = 0x4aec43; continue;
 }
 0x4aec43 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec50); }
 let mut pc: u32 = 0x4aec50;
 loop { match pc {
 0x4aec50 => {
  pc = cpu.r32(0x52569cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec56); }
 let mut pc: u32 = 0x4aec56;
 loop { match pc {
 0x4aec56 => {
  pc = cpu.r32(0x525644u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec5c); }
 let mut pc: u32 = 0x4aec5c;
 loop { match pc {
 0x4aec5c => {
  pc = cpu.r32(0x525648u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec62); }
 let mut pc: u32 = 0x4aec62;
 loop { match pc {
 0x4aec62 => {
  pc = cpu.r32(0x525650u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec68); }
 let mut pc: u32 = 0x4aec68;
 loop { match pc {
 0x4aec68 => {
  pc = cpu.r32(0x525660u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec70); }
 let mut pc: u32 = 0x4aec70;
 loop { match pc {
 0x4aec70 => {
  { let v = 0x30000u32; cpu.push(v); }
  { let v = 0x10000u32; cpu.push(v); }
  cpu.push(0x4aec7f); f_004aeca6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aec90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aec90); }
 let mut pc: u32 = 0x4aec90;
 loop { match pc {
 0x4aec90 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aeca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aeca0); }
 let mut pc: u32 = 0x4aeca0;
 loop { match pc {
 0x4aeca0 => {
  pc = cpu.r32(0x525678u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aeca6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aeca6); }
 let mut pc: u32 = 0x4aeca6;
 loop { match pc {
 0x4aeca6 => {
  pc = cpu.r32(0x5256a4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aecb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aecb0); }
 let mut pc: u32 = 0x4aecb0;
 loop { match pc {
 0x4aecb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aed3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aed3f); }
 let mut pc: u32 = 0x4aed3f;
 loop { match pc {
 0x4aed3f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4b3c60u32); cpu.fcom(a, b);  }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x4aed96; continue; }
  pc = 0x4aed78; continue;
 }
 0x4aed78 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x4aedc1; continue;
 }
 0x4aed96 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x4aedc1; continue;
 }
 0x4aedc1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aedc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aedc7); }
 let mut pc: u32 = 0x4aedc7;
 loop { match pc {
 0x4aedc7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x8u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0) + cpu.rf64(0x4b3c68u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4b3c58u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x4aeedb; continue; }
  pc = 0x4aee12; continue;
 }
 0x4aee12 => {
  { let v = cpu.rf64(0x4b3c68u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4b3c58u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x4aee84; continue; }
  pc = 0x4aee26; continue;
 }
 0x4aee26 => {
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4aee31); f_004af926(cpu);
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4aee42); f_004aea52(cpu);
  { let v = cpu.st(0); cpu.wf64(0xffffffd8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf64(0x4b3c68u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x4aee62); f_004aea52(cpu);
  { let v = cpu.st(0) / cpu.rf64(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x4aee7c); f_004aea52(cpu);
  { let v = cpu.st(0) / cpu.rf64(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x4aee92; continue;
 }
 0x4aee84 => {
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.rf64(0x4b3c68u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4aee92; continue;
 }
 0x4aee92 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4aef61; continue;
 }
 0x4aeedb => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0xffffffe0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x4b3c50u32); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x4b3c70u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0xffffffd8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf64(0x4b3c68u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf64(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x4aef19); f_004aea52(cpu);
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x4aef30); f_004aea52(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4aef61; continue;
 }
 0x4aef61 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_004aef75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aef75); }
 let mut pc: u32 = 0x4aef75;
 loop { match pc {
 0x4aef75 => {
  { let v = cpu.rf32(0x4b3c78u32); cpu.fpush(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0) - cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b3c80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aefa5); f_004aedc7(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4aefbb); f_004aedc7(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4aefc7); f_004aedc7(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aefd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aefd1); }
 let mut pc: u32 = 0x4aefd1;
 loop { match pc {
 0x4aefd1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af00a); f_004aea5e(cpu);
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x4af023; continue; }
  pc = 0x4af01a; continue;
 }
 0x4af01a => {
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4af023; continue;
 }
 0x4af023 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af04b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af04b); }
 let mut pc: u32 = 0x4af04b;
 loop { match pc {
 0x4af04b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4b3c58u32); cpu.fcom(a, b);  }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x4af091; continue; }
  pc = 0x4af07b; continue;
 }
 0x4af07b => {
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af086); f_004aea52(cpu);
  { let v = cpu.st(0) / cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4af09f; continue;
 }
 0x4af091 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3ff00000u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4af09f; continue;
 }
 0x4af09f => {
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af0aa); f_004aea4c(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af0d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af0d1); }
 let mut pc: u32 = 0x4af0d1;
 loop { match pc {
 0x4af0d1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4af0e3); f_004aed3f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4af0f6); f_004aecb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4af103); f_004aefd1(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4af116); f_004aecb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4af123); f_004aefd1(cpu);
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b3c84u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b3c84u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b3c84u32); cpu.set_st(0, v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b3c84u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  cpu.push(0x4af186); f_004af04b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af195); f_004aecb0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af19f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af19f); }
 let mut pc: u32 = 0x4af19f;
 loop { match pc {
 0x4af19f => {
  { let v: u32 = 0x4af260u32; cpu.eax = v; }
  cpu.push(0x4af1a9); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.and8(cpu.r8(0x524308u32), 0x1u8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  if !cpu.zf { pc = 0x4af1d7; continue; }
  pc = 0x4af1b8; continue;
 }
 0x4af1b8 => {
  { let v: u8 = cpu.r8(0x524308u32) | 0x1u8; cpu.w8(0x524308u32, v); }
  { let v: u32 = 0x524320u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x4af1ca); cpu.call(t);
  { let v = 0x4af273u32; cpu.push(v); }
  cpu.push(0x4af1d4); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4af1d7; continue;
 }
 0x4af1d7 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4af1df); f_004af92c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x4af1ea); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4af1f4); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x524320u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4af207); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af213); f_004af26a(cpu);
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x524320u32; cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x4af224); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x524320u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4af237); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af243); f_004af257(cpu);
  { let v: u32 = 0x524320u32; cpu.eax = v; }
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
pub fn f_004af257(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af257); }
 let mut pc: u32 = 0x4af257;
 loop { match pc {
 0x4af257 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af260); }
 let mut pc: u32 = 0x4af260;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af260 => {
  { let v: u32 = 0x4be478u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af26a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af26a); }
 let mut pc: u32 = 0x4af26a;
 loop { match pc {
 0x4af26a => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af273(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af273); }
 let mut pc: u32 = 0x4af273;
 loop { match pc {
 0x4af273 => {
  { let v: u32 = 0x524320u32; cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af27e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af27e); }
 let mut pc: u32 = 0x4af27e;
 loop { match pc {
 0x4af27e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(0x4af287); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x4af291); f_004af8f4(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.push(0x4af29d); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af2a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af2a2); }
 let mut pc: u32 = 0x4af2a2;
 loop { match pc {
 0x4af2a2 => {
  cpu.push(0x4af2a7); f_004af2ac(cpu);
  pc = 0x4af2fb; continue;
 }
 0x4af2fb => {
  { let v = 0x4af309u32; cpu.push(v); }
  cpu.push(0x4af305); f_004ae7f0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af2ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af2ac); }
 let mut pc: u32 = 0x4af2ac;
 loop { match pc {
 0x4af2ac => {
  { let v: u32 = 0x4af2e6u32; cpu.eax = v; }
  cpu.push(0x4af2b6); f_004aea58(cpu);
  { let v: u32 = 0x524310u32; cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4af2c1); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b3c8cu32; cpu.w32(0x524310u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af2e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af2e6); }
 let mut pc: u32 = 0x4af2e6;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af2e6 => {
  { let v: u32 = 0x4be4a8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af2f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af2f0); }
 let mut pc: u32 = 0x4af2f0;
 loop { match pc {
 0x4af2f0 => {
  { let v: u32 = 0x524310u32; cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af309(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af309); }
 let mut pc: u32 = 0x4af309;
 loop { match pc {
 0x4af309 => {
  { let v: u32 = 0x4af358u32; cpu.eax = v; }
  cpu.push(0x4af313); f_004aea58(cpu);
  { cpu.and8(cpu.r8(0x524328u32), 0x1u8); }
  if !cpu.zf { pc = 0x4af34b; continue; }
  pc = 0x4af31c; continue;
 }
 0x4af31c => {
  { let v: u8 = cpu.or8(cpu.r8(0x524328u32), 0x1u8); cpu.w8(0x524328u32, v); }
  { let v: u32 = 0x4b3c8cu32; cpu.w32(0x524310u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x524310u32; cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4af33f); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af34b); f_004af362(cpu);
  pc = 0x4af34b; continue;
 }
 0x4af34b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af358(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af358); }
 let mut pc: u32 = 0x4af358;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af358 => {
  { let v: u32 = 0x4be4d0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af362(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af362); }
 let mut pc: u32 = 0x4af362;
 loop { match pc {
 0x4af362 => {
  { let v: u32 = 0x524310u32; cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af36d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af36d); }
 let mut pc: u32 = 0x4af36d;
 loop { match pc {
 0x4af36d => {
  { let v: u32 = 0x4af3e5u32; cpu.eax = v; }
  cpu.push(0x4af377); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4af385); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4af38b); f_004af63a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4af3d7; continue; }
  pc = 0x4af392; continue;
 }
 0x4af392 => {
  { let v = 0xcu32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af39c); f_004ae83c(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x4af3c4; continue; }
  pc = 0x4af3b4; continue;
 }
 0x4af3b4 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4af3bc); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x4af3c4; continue;
 }
 0x4af3c4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x524310u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4af3d7); cpu.call(t);
  pc = 0x4af3d7; continue;
 }
 0x4af3d7 => {
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
pub fn f_004af3e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af3e5); }
 let mut pc: u32 = 0x4af3e5;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af3e5 => {
  { let v: u32 = 0x4be4f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af3ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af3ef); }
 let mut pc: u32 = 0x4af3ef;
 loop { match pc {
 0x4af3ef => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af3f7); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af3fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af3fb); }
 let mut pc: u32 = 0x4af3fb;
 loop { match pc {
 0x4af3fb => {
  { let v: u32 = 0x4af49fu32; cpu.eax = v; }
  cpu.push(0x4af405); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x524310u32; cpu.push(v); }
  let t = cpu.r32(0x525738u32); cpu.push(0x4af419); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4af429); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4af47a; continue; }
  pc = 0x4af42d; continue;
 }
 0x4af42d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4af438; continue;
 }
 0x4af438 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  let t = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.push(0x4af446); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.push(0x4af44d); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4af458); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4af474; continue; }
  pc = 0x4af45f; continue;
 }
 0x4af45f => {
  let t = cpu.r32(0x525734u32); cpu.push(0x4af465); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4af46e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4af438; continue; }
  pc = 0x4af472; continue;
 }
 0x4af472 => {
  pc = 0x4af47a; continue;
 }
 0x4af474 => {
  let t = cpu.r32(0x52570cu32); cpu.push(0x4af47a); cpu.call(t);
  pc = 0x4af47a; continue;
 }
 0x4af47a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af486); f_004af4a9(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_004af496(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af496); }
 let mut pc: u32 = 0x4af496;
 loop { match pc {
 0x4af496 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x525798u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af49f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af49f); }
 let mut pc: u32 = 0x4af49f;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af49f => {
  { let v: u32 = 0x4be520u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af4a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af4a9); }
 let mut pc: u32 = 0x4af4a9;
 loop { match pc {
 0x4af4a9 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = 0x4af4b1; continue;
 }
 0x4af4b1 => {
  { let v: u32 = 0x4af4dau32; cpu.eax = v; }
  cpu.push(0x4af4bb); f_004aea58(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af4cd); f_004af4e4(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af4da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af4da); }
 let mut pc: u32 = 0x4af4da;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af4da => {
  { let v: u32 = 0x4be550u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af4e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af4e4); }
 let mut pc: u32 = 0x4af4e4;
 loop { match pc {
 0x4af4e4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x525798u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af4ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af4ed); }
 let mut pc: u32 = 0x4af4ed;
 loop { match pc {
 0x4af4ed => {
  { let v: u32 = 0x4af548u32; cpu.eax = v; }
  cpu.push(0x4af4f7); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4af506); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af51c); f_004af552(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af52e); f_004af53f(cpu);
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
pub fn f_004af53f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af53f); }
 let mut pc: u32 = 0x4af53f;
 loop { match pc {
 0x4af53f => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af548(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af548); }
 let mut pc: u32 = 0x4af548;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af548 => {
  { let v: u32 = 0x4be578u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af552(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af552); }
 let mut pc: u32 = 0x4af552;
 loop { match pc {
 0x4af552 => {
  { let v: u32 = 0x4af627u32; cpu.eax = v; }
  cpu.push(0x4af55c); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af568); f_004af63a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4af588; continue; }
  pc = 0x4af56f; continue;
 }
 0x4af56f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x4af579); cpu.call(t);
  pc = 0x4af579; continue;
 }
 0x4af579 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4af588 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = 0x4f8494u32; cpu.push(v); }
  let t = cpu.r32(0x525714u32); cpu.push(0x4af598); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x4f8478u32; cpu.push(v); }
  let t = cpu.r32(0x525714u32); cpu.push(0x4af5af); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x4af5c3); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x525718u32); cpu.push(0x4af5ce); cpu.call(t);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af5d7); f_004af61e(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x4af5e6); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x4af5f1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x4af5f9); cpu.call(t);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af602); f_004af615(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af60e); f_004af631(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4af579; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af615(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af615); }
 let mut pc: u32 = 0x4af615;
 loop { match pc {
 0x4af615 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x5256f0u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af61e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af61e); }
 let mut pc: u32 = 0x4af61e;
 loop { match pc {
 0x4af61e => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af627(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af627); }
 let mut pc: u32 = 0x4af627;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af627 => {
  { let v: u32 = 0x4be5a0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af631(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af631); }
 let mut pc: u32 = 0x4af631;
 loop { match pc {
 0x4af631 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af63a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af63a); }
 let mut pc: u32 = 0x4af63a;
 loop { match pc {
 0x4af63a => {
  { let v: u32 = 0x4af6d3u32; cpu.eax = v; }
  cpu.push(0x4af644); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x524310u32; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4af658); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4af65f; continue;
 }
 0x4af65f => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4af668); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4af693; continue; }
  pc = 0x4af66c; continue;
 }
 0x4af66c => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4af677); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4af684); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4af6b0; continue; }
  pc = 0x4af688; continue;
 }
 0x4af688 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4af691); cpu.call(t);
  pc = 0x4af65f; continue;
 }
 0x4af693 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af69f); f_004af6dd(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4af6a1; continue;
 }
 0x4af6a1 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4af6b0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af6c5); f_004af6dd(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4af6a1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af6ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af6ca); }
 let mut pc: u32 = 0x4af6ca;
 loop { match pc {
 0x4af6ca => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x525798u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af6d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af6d3); }
 let mut pc: u32 = 0x4af6d3;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af6d3 => {
  { let v: u32 = 0x4be5d8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af6dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af6dd); }
 let mut pc: u32 = 0x4af6dd;
 loop { match pc {
 0x4af4b1 => {
  { let v: u32 = 0x4af4dau32; cpu.eax = v; }
  cpu.push(0x4af4bb); f_004aea58(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af4cd); f_004af4e4(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4af6dd => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = 0x4af4b1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af6e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af6e5); }
 let mut pc: u32 = 0x4af6e5;
 loop { match pc {
 0x4af6e5 => {
  { let v: u32 = 0x4af73du32; cpu.eax = v; }
  cpu.push(0x4af6ef); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4af6fa); f_004ae83c(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x4af725; continue; }
  pc = 0x4af712; continue;
 }
 0x4af712 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4af71a); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x4af725; continue;
 }
 0x4af725 => {
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
pub fn f_004af73d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af73d); }
 let mut pc: u32 = 0x4af73d;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af73d => {
  { let v: u32 = 0x4be608u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af747(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af747); }
 let mut pc: u32 = 0x4af747;
 loop { match pc {
 0x4af747 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af74f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af753(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af753); }
 let mut pc: u32 = 0x4af753;
 loop { match pc {
 0x4af753 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x4af75c); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af762(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af762); }
 let mut pc: u32 = 0x4af762;
 loop { match pc {
 0x4af762 => {
  { let v: u32 = 0x4af7b5u32; cpu.eax = v; }
  cpu.push(0x4af76c); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x4b3c8cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x525750u32); cpu.push(0x4af785); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af791); f_004af7bf(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4af7a2; continue; }
  pc = 0x4af797; continue;
 }
 0x4af797 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x4af79f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4af7a2; continue;
 }
 0x4af7a2 => {
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
pub fn f_004af7b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af7b5); }
 let mut pc: u32 = 0x4af7b5;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af7b5 => {
  { let v: u32 = 0x4be630u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af7bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af7bf); }
 let mut pc: u32 = 0x4af7bf;
 loop { match pc {
 0x4af7bf => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af7c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af7c8); }
 let mut pc: u32 = 0x4af7c8;
 loop { match pc {
 0x4af7c8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4af7db); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x4af7e8; continue; }
  pc = 0x4af7e3; continue;
 }
 0x4af7e3 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4af7e8; continue;
 }
 0x4af7e8 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af7ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af7ec); }
 let mut pc: u32 = 0x4af7ec;
 loop { match pc {
 0x4af7ec => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4af7ff); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if cpu.zf { pc = 0x4af80c; continue; }
  pc = 0x4af808; continue;
 }
 0x4af808 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x4af80c; continue;
 }
 0x4af80c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af810); }
 let mut pc: u32 = 0x4af810;
 loop { match pc {
 0x4af810 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4af823); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4af82d; continue; }
  pc = 0x4af827; continue;
 }
 0x4af827 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x4af833; continue;
 }
 0x4af82d => {
  { let v = cpu.rf32(0x4b3c98u32); cpu.fpush(v); }
  pc = 0x4af833; continue;
 }
 0x4af833 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af837(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af837); }
 let mut pc: u32 = 0x4af837;
 loop { match pc {
 0x4af837 => {
  { let v: u32 = 0x4af8c8u32; cpu.eax = v; }
  cpu.push(0x4af841); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4af854); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4af862); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x525764u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.push(0x4af87d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4af8a4; continue; }
  pc = 0x4af881; continue;
 }
 0x4af881 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x525714u32); cpu.push(0x4af88c); cpu.call(t);
  pc = 0x4af88c; continue;
 }
 0x4af88c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.or8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4af8a4 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4af8aa); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x4af8b6); cpu.call(t);
  pc = 0x4af88c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af8b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af8b8); }
 let mut pc: u32 = 0x4af8b8;
 loop { match pc {
 0x4af8b8 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4af8c7; continue; }
  pc = 0x4af8be; continue;
 }
 0x4af8be => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 0x4af8c7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af8c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af8c8); }
 let mut pc: u32 = 0x4af8c8;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af8c8 => {
  { let v: u32 = 0x4be658u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af8d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af8d2); }
 let mut pc: u32 = 0x4af8d2;
 loop { match pc {
 0x4af8d2 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x4af8e2); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af8e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af8e3); }
 let mut pc: u32 = 0x4af8e3;
 loop { match pc {
 0x4af8e3 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x4af8f3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af8f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af8f4); }
 let mut pc: u32 = 0x4af8f4;
 loop { match pc {
 0x4af8f4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x525760u32); cpu.push(0x4af900); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4af90d); f_004af8e3(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  let t = cpu.r32(0x525740u32); cpu.push(0x4af91a); cpu.call(t);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.push(0x4af923); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af926(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af926); }
 let mut pc: u32 = 0x4af926;
 loop { match pc {
 0x4af926 => {
  pc = cpu.r32(0x5256acu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af92c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af92c); }
 let mut pc: u32 = 0x4af92c;
 loop { match pc {
 0x4af92c => {
  pc = cpu.r32(0x5256b0u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af940); }
 let mut pc: u32 = 0x4af940;
 loop { match pc {
 0x4af940 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af949(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af949); }
 let mut pc: u32 = 0x4af949;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af949 => {
  { let v: u32 = 0x4b77d8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af960); }
 let mut pc: u32 = 0x4af960;
 loop { match pc {
 0x4af960 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af969(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af969); }
 let mut pc: u32 = 0x4af969;
 loop { match pc {
 0x4af969 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af972(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af972); }
 let mut pc: u32 = 0x4af972;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af972 => {
  { let v: u32 = 0x4b7800u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af980); }
 let mut pc: u32 = 0x4af980;
 loop { match pc {
 0x4af980 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af989); }
 let mut pc: u32 = 0x4af989;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af989 => {
  { let v: u32 = 0x4b7830u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9a0); }
 let mut pc: u32 = 0x4af9a0;
 loop { match pc {
 0x4af9a0 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042e680(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9a8); }
 let mut pc: u32 = 0x4af9a8;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af9a8 => {
  { let v: u32 = 0x4b7858u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9c0); }
 let mut pc: u32 = 0x4af9c0;
 loop { match pc {
 0x4af9c0 => {
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9c9); }
 let mut pc: u32 = 0x4af9c9;
 loop { match pc {
 0x4af9c9 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_00405cd0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9d1); }
 let mut pc: u32 = 0x4af9d1;
 loop { match pc {
 0x4af9d1 => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042e680(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9d9); }
 let mut pc: u32 = 0x4af9d9;
 loop { match pc {
 0x4af9d9 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042e680(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9e1); }
 let mut pc: u32 = 0x4af9e1;
 loop { match pc {
 0x4af9e1 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042e680(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9e9); }
 let mut pc: u32 = 0x4af9e9;
 loop { match pc {
 0x4af9e9 => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9f2); }
 let mut pc: u32 = 0x4af9f2;
 loop { match pc {
 0x4af9f2 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042e680(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004af9fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4af9fa); }
 let mut pc: u32 = 0x4af9fa;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4af9fa => {
  { let v: u32 = 0x4b7880u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa10); }
 let mut pc: u32 = 0x4afa10;
 loop { match pc {
 0x4afa10 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042e680(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa18); }
 let mut pc: u32 = 0x4afa18;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afa18 => {
  { let v: u32 = 0x4b78e0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa30); }
 let mut pc: u32 = 0x4afa30;
 loop { match pc {
 0x4afa30 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa39); }
 let mut pc: u32 = 0x4afa39;
 loop { match pc {
 0x4afa39 => {
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_00405cd0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa41); }
 let mut pc: u32 = 0x4afa41;
 loop { match pc {
 0x4afa41 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042e680(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa49); }
 let mut pc: u32 = 0x4afa49;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afa49 => {
  { let v: u32 = 0x4b7908u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa60); }
 let mut pc: u32 = 0x4afa60;
 loop { match pc {
 0x4afa60 => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa69); }
 let mut pc: u32 = 0x4afa69;
 loop { match pc {
 0x4afa69 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_00405cd0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa71); }
 let mut pc: u32 = 0x4afa71;
 loop { match pc {
 0x4afa71 => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa7a); }
 let mut pc: u32 = 0x4afa7a;
 loop { match pc {
 0x4afa7a => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa83); }
 let mut pc: u32 = 0x4afa83;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afa83 => {
  { let v: u32 = 0x4b7948u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa90); }
 let mut pc: u32 = 0x4afa90;
 loop { match pc {
 0x4afa90 => {
  { let v: u32 = 0xfffffee0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_004831da(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afa9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afa9b); }
 let mut pc: u32 = 0x4afa9b;
 loop { match pc {
 0x404490 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4af949u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4b1174u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x525750u32); cpu.push(0x4044c1); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x525754u32); cpu.push(0x4044d1); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4afa9b => {
  { let v: u32 = 0xfffffed0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = 0x404490; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afaa6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afaa6); }
 let mut pc: u32 = 0x4afaa6;
 loop { match pc {
 0x4afaa6 => {
  { let v: u32 = 0xfffffed0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afab2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afab2); }
 let mut pc: u32 = 0x4afab2;
 loop { match pc {
 0x4afab2 => {
  { let v: u32 = 0xfffffed0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afabe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afabe); }
 let mut pc: u32 = 0x4afabe;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afabe => {
  { let v: u32 = 0x4b7990u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afad0); }
 let mut pc: u32 = 0x4afad0;
 loop { match pc {
 0x4afad0 => {
  { let v: u32 = 0xfffffee0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_004831da(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afadb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afadb); }
 let mut pc: u32 = 0x4afadb;
 loop { match pc {
 0x404490 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4af949u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4b1174u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x525750u32); cpu.push(0x4044c1); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x525754u32); cpu.push(0x4044d1); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4afadb => {
  { let v: u32 = 0xfffffed0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = 0x404490; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afae6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afae6); }
 let mut pc: u32 = 0x4afae6;
 loop { match pc {
 0x4afae6 => {
  { let v: u32 = 0xfffffed0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afaf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afaf2); }
 let mut pc: u32 = 0x4afaf2;
 loop { match pc {
 0x4afaf2 => {
  { let v: u32 = 0xfffffed0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afafe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afafe); }
 let mut pc: u32 = 0x4afafe;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afafe => {
  { let v: u32 = 0x4b79d0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb10); }
 let mut pc: u32 = 0x4afb10;
 loop { match pc {
 0x4afb10 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb19); }
 let mut pc: u32 = 0x4afb19;
 loop { match pc {
 0x4afb19 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_00405cd0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb21); }
 let mut pc: u32 = 0x4afb21;
 loop { match pc {
 0x4afb21 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042c5c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb29); }
 let mut pc: u32 = 0x4afb29;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afb29 => {
  { let v: u32 = 0x4b7a10u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb40); }
 let mut pc: u32 = 0x4afb40;
 loop { match pc {
 0x4afb40 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4afb49); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb4b); }
 let mut pc: u32 = 0x4afb4b;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afb4b => {
  { let v: u32 = 0x4b7a50u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb60); }
 let mut pc: u32 = 0x4afb60;
 loop { match pc {
 0x4afb60 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x525754u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb69); }
 let mut pc: u32 = 0x4afb69;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afb69 => {
  { let v: u32 = 0x4b7a78u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb80); }
 let mut pc: u32 = 0x4afb80;
 loop { match pc {
 0x4afb80 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afb89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afb89); }
 let mut pc: u32 = 0x4afb89;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afb89 => {
  { let v: u32 = 0x4b7aa0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afba0); }
 let mut pc: u32 = 0x4afba0;
 loop { match pc {
 0x4afba0 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afba9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afba9); }
 let mut pc: u32 = 0x4afba9;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afba9 => {
  { let v: u32 = 0x4b7ac8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afbc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afbc0); }
 let mut pc: u32 = 0x4afbc0;
 loop { match pc {
 0x4afbc0 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afbc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afbc9); }
 let mut pc: u32 = 0x4afbc9;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afbc9 => {
  { let v: u32 = 0x4b7af0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afbe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afbe0); }
 let mut pc: u32 = 0x4afbe0;
 loop { match pc {
 0x4afbe0 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afbe9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afbe9); }
 let mut pc: u32 = 0x4afbe9;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afbe9 => {
  { let v: u32 = 0x4b7b18u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc00); }
 let mut pc: u32 = 0x4afc00;
 loop { match pc {
 0x4afc00 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc09); }
 let mut pc: u32 = 0x4afc09;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afc09 => {
  { let v: u32 = 0x4b7b40u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc20); }
 let mut pc: u32 = 0x4afc20;
 loop { match pc {
 0x4afc20 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc29); }
 let mut pc: u32 = 0x4afc29;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afc29 => {
  { let v: u32 = 0x4b7b68u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc40); }
 let mut pc: u32 = 0x4afc40;
 loop { match pc {
 0x4afc40 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc49); }
 let mut pc: u32 = 0x4afc49;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afc49 => {
  { let v: u32 = 0x4b7b90u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc60); }
 let mut pc: u32 = 0x4afc60;
 loop { match pc {
 0x4afc60 => {
  { let v: u32 = 0xfffffdd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc6c); }
 let mut pc: u32 = 0x4afc6c;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afc6c => {
  { let v: u32 = 0x4b7bb8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc80); }
 let mut pc: u32 = 0x4afc80;
 loop { match pc {
 0x4afc80 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afc89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afc89); }
 let mut pc: u32 = 0x4afc89;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afc89 => {
  { let v: u32 = 0x4b7be0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afca0); }
 let mut pc: u32 = 0x4afca0;
 loop { match pc {
 0x4afca0 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afca9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afca9); }
 let mut pc: u32 = 0x4afca9;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afca9 => {
  { let v: u32 = 0x4b7c08u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afcc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afcc0); }
 let mut pc: u32 = 0x4afcc0;
 loop { match pc {
 0x4afcc0 => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afcc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afcc9); }
 let mut pc: u32 = 0x4afcc9;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afcc9 => {
  { let v: u32 = 0x4b7c30u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afce0); }
 let mut pc: u32 = 0x4afce0;
 loop { match pc {
 0x4afce0 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afce9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afce9); }
 let mut pc: u32 = 0x4afce9;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afce9 => {
  { let v: u32 = 0x4b7c58u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd00); }
 let mut pc: u32 = 0x4afd00;
 loop { match pc {
 0x4afd00 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd09); }
 let mut pc: u32 = 0x4afd09;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afd09 => {
  { let v: u32 = 0x4b7c80u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd20); }
 let mut pc: u32 = 0x4afd20;
 loop { match pc {
 0x4afd20 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_0042c400(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd28); }
 let mut pc: u32 = 0x4afd28;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afd28 => {
  { let v: u32 = 0x4b7ca8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd40); }
 let mut pc: u32 = 0x4afd40;
 loop { match pc {
 0x4afd40 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_0042c400(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd48); }
 let mut pc: u32 = 0x4afd48;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afd48 => {
  { let v: u32 = 0x4b7cd0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd60); }
 let mut pc: u32 = 0x4afd60;
 loop { match pc {
 0x4afd60 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042c5c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd68); }
 let mut pc: u32 = 0x4afd68;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afd68 => {
  { let v: u32 = 0x4b7cf8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd80); }
 let mut pc: u32 = 0x4afd80;
 loop { match pc {
 0x4afd80 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4afd89); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afd8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afd8b); }
 let mut pc: u32 = 0x4afd8b;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afd8b => {
  { let v: u32 = 0x4b7d20u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afda0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afda0); }
 let mut pc: u32 = 0x4afda0;
 loop { match pc {
 0x4afda0 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_0042e680(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afda8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afda8); }
 let mut pc: u32 = 0x4afda8;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afda8 => {
  { let v: u32 = 0x4b7d48u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afdc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afdc0); }
 let mut pc: u32 = 0x4afdc0;
 loop { match pc {
 0x4afdc0 => {
  { let v: u32 = 0xfffffe6cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afdcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afdcc); }
 let mut pc: u32 = 0x4afdcc;
 loop { match pc {
 0x4afdcc => {
  { let v: u32 = 0xfffffe60u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afdd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afdd8); }
 let mut pc: u32 = 0x4afdd8;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afdd8 => {
  { let v: u32 = 0x4b7d70u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afdf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afdf0); }
 let mut pc: u32 = 0x4afdf0;
 loop { match pc {
 0x4afdf0 => {
  { let v: u32 = cpu.r32(0xfffffce4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4afdfc); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afdfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afdfe); }
 let mut pc: u32 = 0x4afdfe;
 loop { match pc {
 0x4afdfe => {
  { let v: u32 = cpu.r32(0xfffffce4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4afe0a); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe0c); }
 let mut pc: u32 = 0x4afe0c;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afe0c => {
  { let v: u32 = 0x4b7da0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe20); }
 let mut pc: u32 = 0x4afe20;
 loop { match pc {
 0x4afe20 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe29); }
 let mut pc: u32 = 0x4afe29;
 loop { match pc {
 0x4afe29 => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe32); }
 let mut pc: u32 = 0x4afe32;
 loop { match pc {
 0x4afe32 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe3b); }
 let mut pc: u32 = 0x4afe3b;
 loop { match pc {
 0x465b00 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4afea1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x52571cu32); cpu.edi = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x465b31); cpu.call(t);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x465b3b); cpu.call(t);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x465b45); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x465b51); cpu.call(t);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4afe3b => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = 0x465b00; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe43); }
 let mut pc: u32 = 0x4afe43;
 loop { match pc {
 0x4afe43 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe4c); }
 let mut pc: u32 = 0x4afe4c;
 loop { match pc {
 0x4afe4c => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe55); }
 let mut pc: u32 = 0x4afe55;
 loop { match pc {
 0x4afe55 => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe5e); }
 let mut pc: u32 = 0x4afe5e;
 loop { match pc {
 0x4afe5e => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe67); }
 let mut pc: u32 = 0x4afe67;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afe67 => {
  { let v: u32 = 0x4b7dd0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe80); }
 let mut pc: u32 = 0x4afe80;
 loop { match pc {
 0x4afe80 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe89); }
 let mut pc: u32 = 0x4afe89;
 loop { match pc {
 0x4afe89 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afe95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afe95); }
 let mut pc: u32 = 0x4afe95;
 loop { match pc {
 0x4afe95 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afea1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afea1); }
 let mut pc: u32 = 0x4afea1;
 loop { match pc {
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 0x4afea1 => {
  { let v: u32 = 0x4b7e30u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afeb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afeb0); }
 let mut pc: u32 = 0x4afeb0;
 loop { match pc {
 0x4afeb0 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afebc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afebc); }
 let mut pc: u32 = 0x4afebc;
 loop { match pc {
 0x4afebc => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afec8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afec8); }
 let mut pc: u32 = 0x4afec8;
 loop { match pc {
 0x4afec8 => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afed4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afed4); }
 let mut pc: u32 = 0x4afed4;
 loop { match pc {
 0x465b00 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4afea1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x52571cu32); cpu.edi = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x465b31); cpu.call(t);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x465b3b); cpu.call(t);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x465b45); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.edi; cpu.push(0x465b51); cpu.call(t);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4afed4 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = 0x465b00; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afedf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afedf); }
 let mut pc: u32 = 0x4afedf;
 loop { match pc {
 0x4afedf => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4afeeb); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afeed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afeed); }
 let mut pc: u32 = 0x4afeed;
 loop { match pc {
 0x4afeed => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afef9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afef9); }
 let mut pc: u32 = 0x4afef9;
 loop { match pc {
 0x4afef9 => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff05); }
 let mut pc: u32 = 0x4aff05;
 loop { match pc {
 0x4aff05 => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff11); }
 let mut pc: u32 = 0x4aff11;
 loop { match pc {
 0x4aff11 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aff1d); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff1f); }
 let mut pc: u32 = 0x4aff1f;
 loop { match pc {
 0x4aff1f => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff2b); }
 let mut pc: u32 = 0x4aff2b;
 loop { match pc {
 0x4aff2b => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff37); }
 let mut pc: u32 = 0x4aff37;
 loop { match pc {
 0x4aff37 => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff43); }
 let mut pc: u32 = 0x4aff43;
 loop { match pc {
 0x4aff43 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff4f); }
 let mut pc: u32 = 0x4aff4f;
 loop { match pc {
 0x4aff4f => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff5b); }
 let mut pc: u32 = 0x4aff5b;
 loop { match pc {
 0x4aff5b => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff67); }
 let mut pc: u32 = 0x4aff67;
 loop { match pc {
 0x4aff67 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aff73); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff75); }
 let mut pc: u32 = 0x4aff75;
 loop { match pc {
 0x4aff75 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aff81); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff83); }
 let mut pc: u32 = 0x4aff83;
 loop { match pc {
 0x4aff83 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff8f); }
 let mut pc: u32 = 0x4aff8f;
 loop { match pc {
 0x4aff8f => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aff9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aff9e); }
 let mut pc: u32 = 0x4aff9e;
 loop { match pc {
 0x4aff9e => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004affad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4affad); }
 let mut pc: u32 = 0x4affad;
 loop { match pc {
 0x4affad => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4affb9); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004affbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4affbb); }
 let mut pc: u32 = 0x4affbb;
 loop { match pc {
 0x4affbb => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4affc7); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004affc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4affc9); }
 let mut pc: u32 = 0x4affc9;
 loop { match pc {
 0x4affc9 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004affd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4affd5); }
 let mut pc: u32 = 0x4affd5;
 loop { match pc {
 0x4affd5 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004affe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4affe4); }
 let mut pc: u32 = 0x4affe4;
 loop { match pc {
 0x4affe4 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004afff3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4afff3); }
 let mut pc: u32 = 0x4afff3;
 loop { match pc {
 0x4afff3 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004affff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4affff); }
 let mut pc: u32 = 0x4affff;
 loop { match pc {
 0x4affff => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b000b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b000b); }
 let mut pc: u32 = 0x4b000b;
 loop { match pc {
 0x4b000b => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0012(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0012); }
 let mut pc: u32 = 0x4b0012;
 loop { match pc {
 0x4b0012 => {
  { let v: u32 = cpu.and32(cpu.eax, 0x52571cu32); cpu.eax = v; }
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0023(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0023); }
 let mut pc: u32 = 0x4b0023;
 loop { match pc {
 0x4b0023 => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b002f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b002f); }
 let mut pc: u32 = 0x4b002f;
 loop { match pc {
 0x4b002f => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b003b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b003b); }
 let mut pc: u32 = 0x4b003b;
 loop { match pc {
 0x4b003b => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b0047); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b003d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b003d); }
 let mut pc: u32 = 0x4b003d;
 loop { match pc {
 0x4b003d => {
  { let v: u8 = cpu.sar8((cpu.eax as u8), 255u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.push(0x4b0043); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.unimplemented(0x4b0044, "Out");
  let t = cpu.r32(0xffffffc3u32.wrapping_add(cpu.ecx)); cpu.push(0x4b0049); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0055(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0055); }
 let mut pc: u32 = 0x4b0055;
 loop { match pc {
 0x4b0055 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0064(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0064); }
 let mut pc: u32 = 0x4b0064;
 loop { match pc {
 0x4b0064 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0073(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0073); }
 let mut pc: u32 = 0x4b0073;
 loop { match pc {
 0x4b0073 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b007f); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0081(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0081); }
 let mut pc: u32 = 0x4b0081;
 loop { match pc {
 0x4b0081 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b008a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b008a); }
 let mut pc: u32 = 0x4b008a;
 loop { match pc {
 0x4b008a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0xfff8bc8du32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0xfff8bc8du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x25ff08c1u32.wrapping_add(cpu.ebx))); cpu.w32(0x25ff08c1u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x57u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0xfff8bc8du32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0xfff8bc8du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x25ff10c1u32.wrapping_add(cpu.ebx))); cpu.w32(0x25ff10c1u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x57u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0xfff8bc85u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0xfff8bc85u32.wrapping_add(cpu.ebx), v); }
  let t = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.push(0x4b00b3); cpu.call(t);
  cpu.call(0x5a4b009e); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b008d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b008d); }
 let mut pc: u32 = 0x4b008d;
 loop { match pc {
 0x4b008d => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b009c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b009c); }
 let mut pc: u32 = 0x4b009c;
 loop { match pc {
 0x4b009c => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b00ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b00ab); }
 let mut pc: u32 = 0x4b00ab;
 loop { match pc {
 0x4b00ab => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b00b7); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b00b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b00b9); }
 let mut pc: u32 = 0x4b00b9;
 loop { match pc {
 0x4b00b9 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b00c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b00c5); }
 let mut pc: u32 = 0x4b00c5;
 loop { match pc {
 0x4b00c5 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b00d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b00d4); }
 let mut pc: u32 = 0x4b00d4;
 loop { match pc {
 0x4b00d4 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b00e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b00e3); }
 let mut pc: u32 = 0x4b00e3;
 loop { match pc {
 0x4b00e3 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b00ef); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b00f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b00f1); }
 let mut pc: u32 = 0x4b00f1;
 loop { match pc {
 0x4b00f1 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b00fd); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b00ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b00ff); }
 let mut pc: u32 = 0x4b00ff;
 loop { match pc {
 0x4b00ff => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b010b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b010b); }
 let mut pc: u32 = 0x4b010b;
 loop { match pc {
 0x4b010b => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b011a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b011a); }
 let mut pc: u32 = 0x4b011a;
 loop { match pc {
 0x4b011a => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0129(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0129); }
 let mut pc: u32 = 0x4b0129;
 loop { match pc {
 0x4b0129 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b0135); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0137(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0137); }
 let mut pc: u32 = 0x4b0137;
 loop { match pc {
 0x4b0137 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b0143); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0145(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0145); }
 let mut pc: u32 = 0x4b0145;
 loop { match pc {
 0x4b0145 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0151(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0151); }
 let mut pc: u32 = 0x4b0151;
 loop { match pc {
 0x4b0151 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0160); }
 let mut pc: u32 = 0x4b0160;
 loop { match pc {
 0x4b0160 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b016f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b016f); }
 let mut pc: u32 = 0x4b016f;
 loop { match pc {
 0x4b016f => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b017b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b017b); }
 let mut pc: u32 = 0x4b017b;
 loop { match pc {
 0x4b017b => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0187(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0187); }
 let mut pc: u32 = 0x4b0187;
 loop { match pc {
 0x4b0187 => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0193(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0193); }
 let mut pc: u32 = 0x4b0193;
 loop { match pc {
 0x4b0193 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b019f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b019f); }
 let mut pc: u32 = 0x4b019f;
 loop { match pc {
 0x4b019f => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b01ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b01ab); }
 let mut pc: u32 = 0x4b01ab;
 loop { match pc {
 0x4b01ab => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b01b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b01b7); }
 let mut pc: u32 = 0x4b01b7;
 loop { match pc {
 0x4b01b7 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b01c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b01c3); }
 let mut pc: u32 = 0x4b01c3;
 loop { match pc {
 0x4b01c3 => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b01cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b01cf); }
 let mut pc: u32 = 0x4b01cf;
 loop { match pc {
 0x4b01cf => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b01db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b01db); }
 let mut pc: u32 = 0x4b01db;
 loop { match pc {
 0x4b01db => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b01e7); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b01e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b01e9); }
 let mut pc: u32 = 0x4b01e9;
 loop { match pc {
 0x4b01e9 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b01f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b01f5); }
 let mut pc: u32 = 0x4b01f5;
 loop { match pc {
 0x4b01f5 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0204(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0204); }
 let mut pc: u32 = 0x4b0204;
 loop { match pc {
 0x4b0204 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0213(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0213); }
 let mut pc: u32 = 0x4b0213;
 loop { match pc {
 0x4b0213 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b021f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b021f); }
 let mut pc: u32 = 0x4b021f;
 loop { match pc {
 0x4b021f => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b022b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b022b); }
 let mut pc: u32 = 0x4b022b;
 loop { match pc {
 0x4b022b => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0237(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0237); }
 let mut pc: u32 = 0x4b0237;
 loop { match pc {
 0x4b0237 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b0243); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0245(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0245); }
 let mut pc: u32 = 0x4b0245;
 loop { match pc {
 0x4b0245 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0251(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0251); }
 let mut pc: u32 = 0x4b0251;
 loop { match pc {
 0x4b0251 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0260); }
 let mut pc: u32 = 0x4b0260;
 loop { match pc {
 0x4b0260 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b026f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b026f); }
 let mut pc: u32 = 0x4b026f;
 loop { match pc {
 0x4b026f => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b027b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b027b); }
 let mut pc: u32 = 0x4b027b;
 loop { match pc {
 0x4b027b => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0287(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0287); }
 let mut pc: u32 = 0x4b0287;
 loop { match pc {
 0x4b0287 => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0293(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0293); }
 let mut pc: u32 = 0x4b0293;
 loop { match pc {
 0x4b0293 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b029f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b029f); }
 let mut pc: u32 = 0x4b029f;
 loop { match pc {
 0x4b029f => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b02ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b02ab); }
 let mut pc: u32 = 0x4b02ab;
 loop { match pc {
 0x4b02ab => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b02b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b02b7); }
 let mut pc: u32 = 0x4b02b7;
 loop { match pc {
 0x4b02b7 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b02c3); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b02c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b02c5); }
 let mut pc: u32 = 0x4b02c5;
 loop { match pc {
 0x4b02c5 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b02d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b02d1); }
 let mut pc: u32 = 0x4b02d1;
 loop { match pc {
 0x4b02d1 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b02e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b02e0); }
 let mut pc: u32 = 0x4b02e0;
 loop { match pc {
 0x4b02e0 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b02ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b02ef); }
 let mut pc: u32 = 0x4b02ef;
 loop { match pc {
 0x4b02ef => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b02fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b02fb); }
 let mut pc: u32 = 0x4b02fb;
 loop { match pc {
 0x4b02fb => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0307(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0307); }
 let mut pc: u32 = 0x4b0307;
 loop { match pc {
 0x4b0307 => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0313(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0313); }
 let mut pc: u32 = 0x4b0313;
 loop { match pc {
 0x4b0313 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b031f); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0321(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0321); }
 let mut pc: u32 = 0x4b0321;
 loop { match pc {
 0x4b0321 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b032d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b032d); }
 let mut pc: u32 = 0x4b032d;
 loop { match pc {
 0x4b032d => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b033c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b033c); }
 let mut pc: u32 = 0x4b033c;
 loop { match pc {
 0x4b033c => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b034b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b034b); }
 let mut pc: u32 = 0x4b034b;
 loop { match pc {
 0x4b034b => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b0357); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0359(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0359); }
 let mut pc: u32 = 0x4b0359;
 loop { match pc {
 0x4b0359 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0365(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0365); }
 let mut pc: u32 = 0x4b0365;
 loop { match pc {
 0x4b0365 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0374(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0374); }
 let mut pc: u32 = 0x4b0374;
 loop { match pc {
 0x4b0374 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0383(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0383); }
 let mut pc: u32 = 0x4b0383;
 loop { match pc {
 0x4b0383 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b038f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b038f); }
 let mut pc: u32 = 0x4b038f;
 loop { match pc {
 0x4b038f => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b039b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b039b); }
 let mut pc: u32 = 0x4b039b;
 loop { match pc {
 0x4b039b => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b03a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b03a7); }
 let mut pc: u32 = 0x4b03a7;
 loop { match pc {
 0x4b03a7 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b03b3); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b03b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b03b5); }
 let mut pc: u32 = 0x4b03b5;
 loop { match pc {
 0x4b03b5 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b03c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b03c1); }
 let mut pc: u32 = 0x4b03c1;
 loop { match pc {
 0x4b03c1 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b03d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b03d0); }
 let mut pc: u32 = 0x4b03d0;
 loop { match pc {
 0x4b03d0 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b03df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b03df); }
 let mut pc: u32 = 0x4b03df;
 loop { match pc {
 0x4b03df => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b03eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b03eb); }
 let mut pc: u32 = 0x4b03eb;
 loop { match pc {
 0x4b03eb => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b03f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b03f7); }
 let mut pc: u32 = 0x4b03f7;
 loop { match pc {
 0x4b03f7 => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0403(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0403); }
 let mut pc: u32 = 0x4b0403;
 loop { match pc {
 0x4b0403 => {
  { let v: u32 = 0xfffff8c4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b040f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b040f); }
 let mut pc: u32 = 0x4b040f;
 loop { match pc {
 0x4b040f => {
  { let v: u32 = 0xfffff8ccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b041b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b041b); }
 let mut pc: u32 = 0x4b041b;
 loop { match pc {
 0x4b041b => {
  { let v: u32 = 0xfffff8d4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0427(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0427); }
 let mut pc: u32 = 0x4b0427;
 loop { match pc {
 0x4b0427 => {
  { let v: u32 = cpu.r32(0xfffff8c0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4b0433); f_004ae7a0(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004b0435(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4b0435); }
 let mut pc: u32 = 0x4b0435;
 loop { match pc {
 0x4b0435 => {
  { let v: u32 = cpu.r32(0xfffff8bcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
