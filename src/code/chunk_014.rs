#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_004a82d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a82d6); }
 let mut pc: u32 = 0x4a82d6;
 loop { match pc {
 0x4a82d6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a82ed; continue;
 }
 0x4a82ed => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a85e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a85e6); }
 let mut pc: u32 = 0x4a85e6;
 loop { match pc {
 0x4a85e6 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a85ef); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a85f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a85f3); }
 let mut pc: u32 = 0x4a85f3;
 loop { match pc {
 0x4a85f3 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a85fc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a85fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a85fd); }
 let mut pc: u32 = 0x4a85fd;
 loop { match pc {
 0x4a85fd => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a8606); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8607(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8607); }
 let mut pc: u32 = 0x4a8607;
 loop { match pc {
 0x4a8607 => {
  { let v: u32 = 0x4bdae0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a86c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a86c7); }
 let mut pc: u32 = 0x4a86c7;
 loop { match pc {
 0x4a86c7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a8767u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a86f6; continue;
 }
 0x4a86f6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b39dcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a870b; continue;
 }
 0x4a870b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8726; continue;
 }
 0x4a8726 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b39bcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a874f; continue;
 }
 0x4a874f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a8771; continue;
 }
 0x4a8771 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a871d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a871d); }
 let mut pc: u32 = 0x4a871d;
 loop { match pc {
 0x4a871d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a8725); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a875e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a875e); }
 let mut pc: u32 = 0x4a875e;
 loop { match pc {
 0x4a875e => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a8766); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8767(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8767); }
 let mut pc: u32 = 0x4a8767;
 loop { match pc {
 0x4a8767 => {
  { let v: u32 = 0x4bdb18u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8780); }
 let mut pc: u32 = 0x4a8780;
 loop { match pc {
 0x4a8780 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4a8797; continue;
 }
 0x4a8797 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a879c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a879c); }
 let mut pc: u32 = 0x4a879c;
 loop { match pc {
 0x4a879c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a87b1); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8802; continue;
 }
 0x4a87c8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a87d1); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a87e3); f_004af810(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4a8811; continue;
 }
 0x4a87f1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a881b; continue;
 }
 0x4a8802 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a87c8; continue; }
  pc = 0x4a880c; continue;
 }
 0x4a880c => {
  pc = 0x4a87f1; continue;
 }
 0x4a8811 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a881b; continue;
 }
 0x4a881b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8822(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8822); }
 let mut pc: u32 = 0x4a8822;
 loop { match pc {
 0x4a8822 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a8839); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a884c); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a885f); f_004af8e3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a886c; continue;
 }
 0x4a886c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8873(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8873); }
 let mut pc: u32 = 0x4a8873;
 loop { match pc {
 0x4a8873 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8884; continue;
 }
 0x4a8884 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x78u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a88c1); cpu.call(t);
  pc = 0x4a88c6; continue;
 }
 0x4a88c6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a88cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a88cb); }
 let mut pc: u32 = 0x4a88cb;
 loop { match pc {
 0x4a88cb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a8994u32; cpu.push(v); }
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
  cpu.push(0x4a8909); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a8919); f_0049ae07(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a893c; continue; }
  pc = 0x4a8921; continue;
 }
 0x4a8921 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a8934); f_004a898b(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a899e; continue;
 }
 0x4a893c => {
  pc = 0x4a8941; continue;
 }
 0x4a8941 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4a8955); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a897a); f_004a898b(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a899e; continue;
 }
 0x4a899e => {
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
pub fn f_004a8982(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8982); }
 let mut pc: u32 = 0x4a8982;
 loop { match pc {
 0x4a8982 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a898a); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a898b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a898b); }
 let mut pc: u32 = 0x4a898b;
 loop { match pc {
 0x4a898b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a8993); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8994(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8994); }
 let mut pc: u32 = 0x4a8994;
 loop { match pc {
 0x4a8994 => {
  { let v: u32 = 0x4bdb48u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a89af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a89af); }
 let mut pc: u32 = 0x4a89af;
 loop { match pc {
 0x4a89af => {
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
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a89d8); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a89de); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4a89f3); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8a07); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a8a56; continue; }
  pc = 0x4a8a16; continue;
 }
 0x4a8a16 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8a27); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a8a42; continue; }
  pc = 0x4a8a36; continue;
 }
 0x4a8a36 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)).wrapping_sub(0x14u32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a8a16; continue;
 }
 0x4a8a42 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a8ab1; continue;
 }
 0x4a8a56 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8a67); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a8ab1; continue; }
  pc = 0x4a8a76; continue;
 }
 0x4a8a76 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8a87); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a8aa2; continue; }
  pc = 0x4a8a96; continue;
 }
 0x4a8a96 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)).wrapping_add(0x14u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a8a76; continue;
 }
 0x4a8aa2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a8ab1; continue;
 }
 0x4a8ab1 => {
  pc = 0x4a8ab6; continue;
 }
 0x4a8ab6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8abd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8abd); }
 let mut pc: u32 = 0x4a8abd;
 loop { match pc {
 0x4a8abd => {
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
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8ada); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8aee); cpu.call(t);
  { let v = cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74a0u32; cpu.push(v); }
  { let v = 0x4c27e8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a8b0d); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a8b13); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4a8b28); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8b3c); cpu.call(t);
  { let v = cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x4a8b62; continue; }
  pc = 0x4a8b57; continue;
 }
 0x4a8b57 => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  pc = 0x4a8b68; continue;
 }
 0x4a8b62 => {
  { let v = cpu.rf32(0x4b38f0u32); cpu.fpush(v); }
  pc = 0x4a8b68; continue;
 }
 0x4a8b68 => {
  pc = 0x4a8b6d; continue;
 }
 0x4a8b6d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8b72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8b72); }
 let mut pc: u32 = 0x4a8b72;
 loop { match pc {
 0x4a8b72 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a8b86); f_004a8abd(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8b9a); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a8bae); cpu.call(t);
  { let v = cpu.rf32(0xffffffdcu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38e0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a8c58; continue; }
  pc = 0x4a8bc3; continue;
 }
 0x4a8bc3 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x10u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8c3d; continue;
 }
 0x4a8c3d => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a8caa; continue;
 }
 0x4a8c58 => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f4u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x4a8c8e; continue; }
  pc = 0x4a8c6d; continue;
 }
 0x4a8c6d => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a8caa; continue;
 }
 0x4a8c8e => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a8caa; continue;
 }
 0x4a8caa => {
  pc = 0x4a8caf; continue;
 }
 0x4a8caf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8cb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8cb6); }
 let mut pc: u32 = 0x4a8cb6;
 loop { match pc {
 0x4a8cb6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a8ee2u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x4a8d0f; continue; }
  pc = 0x4a8ce8; continue;
 }
 0x4a8ce8 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b39f8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a8d04; continue;
 }
 0x4a8d04 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8d0f; continue;
 }
 0x4a8d0f => {
  pc = 0x4a8d38; continue;
 }
 0x4a8d38 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a8d42); f_0049b35e(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a8d55); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a8d68; continue;
 }
 0x4a8d68 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8d87; continue;
 }
 0x4a8d87 => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a8d97); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38c4u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a8daa; continue;
 }
 0x4a8daa => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8dc9; continue;
 }
 0x4a8dc9 => {
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
  pc = 0x4a8e02; continue;
 }
 0x4a8e02 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8e48; continue;
 }
 0x4a8e48 => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3a1cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3a04u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  { let v = 0x4f8340u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4a8eac); cpu.call(t);
  pc = 0x4a8eb1; continue;
 }
 0x4a8eb1 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a8eec; continue;
 }
 0x4a8eec => {
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
pub fn f_004a8d7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8d7a); }
 let mut pc: u32 = 0x4a8d7a;
 loop { match pc {
 0x4a8d7a => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a8d86); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8dbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8dbc); }
 let mut pc: u32 = 0x4a8dbc;
 loop { match pc {
 0x4a8dbc => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a8dc8); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8e11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8e11); }
 let mut pc: u32 = 0x4a8e11;
 loop { match pc {
 0x4a8e11 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x4a8e1c); f_004a44b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8e1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8e1d); }
 let mut pc: u32 = 0x4a8e1d;
 loop { match pc {
 0x4a8e1d => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x4a8e28); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8e29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8e29); }
 let mut pc: u32 = 0x4a8e29;
 loop { match pc {
 0x4a8e29 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a8e31); f_004ad9f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8e32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8e32); }
 let mut pc: u32 = 0x4a8e32;
 loop { match pc {
 0x4a8e32 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4a8e47; continue; }
  pc = 0x4a8e3c; continue;
 }
 0x4a8e3c => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x78u32); cpu.ecx = v; }
  cpu.push(0x4a8e47); f_00485370(cpu);
  pc = 0x4a8e47; continue;
 }
 0x4a8e47 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8ec0); }
 let mut pc: u32 = 0x4a8ec0;
 loop { match pc {
 0x4a8ec0 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a8ec8); f_004ad9c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8ec9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8ec9); }
 let mut pc: u32 = 0x4a8ec9;
 loop { match pc {
 0x4a8ec9 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a8ee1; continue; }
  pc = 0x4a8ed3; continue;
 }
 0x4a8ed3 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x80u32); cpu.ecx = v; }
  cpu.push(0x4a8ee1); f_00485370(cpu);
  pc = 0x4a8ee1; continue;
 }
 0x4a8ee1 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8ee2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8ee2); }
 let mut pc: u32 = 0x4a8ee2;
 loop { match pc {
 0x4a8ee2 => {
  { let v: u32 = 0x4bdb78u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8efd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8efd); }
 let mut pc: u32 = 0x4a8efd;
 loop { match pc {
 0x4a8efd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a8fb8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3a1cu32; cpu.w32(0xffffff80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3a04u32; cpu.w32(0xffffff84u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  if cpu.zf { pc = 0x4a8f96; continue; }
  pc = 0x4a8f6c; continue;
 }
 0x4a8f6c => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a8f96; continue; }
  pc = 0x4a8f85; continue;
 }
 0x4a8f85 => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a8f91); cpu.call(t);
  pc = 0x4a8f96; continue;
 }
 0x4a8f96 => {
  pc = 0x4a8f9b; continue;
 }
 0x4a8f9b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a8fa7); f_004a8fac(cpu);
  pc = 0x4a8fc2; continue;
 }
 0x4a8fc2 => {
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
pub fn f_004a8fac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8fac); }
 let mut pc: u32 = 0x4a8fac;
 loop { match pc {
 0x4a8fac => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a8fb7); f_004a25b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8fb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8fb8); }
 let mut pc: u32 = 0x4a8fb8;
 loop { match pc {
 0x4a8fb8 => {
  { let v: u32 = 0x4bdbd8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a8fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a8fd0); }
 let mut pc: u32 = 0x4a8fd0;
 loop { match pc {
 0x4a8fd0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a915bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4a8ffa); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a9144; continue; }
  pc = 0x4a900d; continue;
 }
 0x4a900d => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9028); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a9073; continue; }
  pc = 0x4a903f; continue;
 }
 0x4a903f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4add80u32; cpu.push(v); }
  { let v = 0x4a9595u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9065); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a907a; continue;
 }
 0x4a9073 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a907a; continue;
 }
 0x4a907a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x78u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a909e; continue; }
  pc = 0x4a9097; continue;
 }
 0x4a9097 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a9165; continue;
 }
 0x4a909e => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a90a7; continue;
 }
 0x4a90a7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4a90b0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a9144; continue; }
  pc = 0x4a90b8; continue;
 }
 0x4a90b8 => {
  pc = 0x4a90bd; continue;
 }
 0x4a90bd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a90ce; continue;
 }
 0x4a90ce => {
  pc = 0x4a90d3; continue;
 }
 0x4a90d3 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a9100; continue;
 }
 0x4a9100 => {
  pc = 0x4a9105; continue;
 }
 0x4a9105 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a9132; continue; }
  pc = 0x4a9121; continue;
 }
 0x4a9121 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a912d); cpu.call(t);
  pc = 0x4a9132; continue;
 }
 0x4a9132 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x4a913b); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0x14u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a90a7; continue;
 }
 0x4a9144 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a9165; continue;
 }
 0x4a9165 => {
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
pub fn f_004a914e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a914e); }
 let mut pc: u32 = 0x4a914e;
 loop { match pc {
 0x4a914e => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9157); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a915b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a915b); }
 let mut pc: u32 = 0x4a915b;
 loop { match pc {
 0x4a915b => {
  { let v: u32 = 0x4bdc00u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9176(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9176); }
 let mut pc: u32 = 0x4a9176;
 loop { match pc {
 0x4a9176 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a918d; continue;
 }
 0x4a918d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9192(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9192); }
 let mut pc: u32 = 0x4a9192;
 loop { match pc {
 0x4a9192 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a91a9; continue;
 }
 0x4a91a9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a94b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a94b4); }
 let mut pc: u32 = 0x4a94b4;
 loop { match pc {
 0x4a94b4 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a94bd); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a94c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a94c1); }
 let mut pc: u32 = 0x4a94c1;
 loop { match pc {
 0x4a94c1 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a94ca); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a94cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a94cb); }
 let mut pc: u32 = 0x4a94cb;
 loop { match pc {
 0x4a94cb => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a94d4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a94d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a94d5); }
 let mut pc: u32 = 0x4a94d5;
 loop { match pc {
 0x4a94d5 => {
  { let v: u32 = 0x4bdc28u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9595(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9595); }
 let mut pc: u32 = 0x4a9595;
 loop { match pc {
 0x4a9595 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a964eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a95c4; continue;
 }
 0x4a95c4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b39dcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a95d9; continue;
 }
 0x4a95d9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a95f4; continue;
 }
 0x4a95f4 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a961e; continue;
 }
 0x4a961e => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3a3cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a9636; continue;
 }
 0x4a9636 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a9658; continue;
 }
 0x4a9658 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a95eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a95eb); }
 let mut pc: u32 = 0x4a95eb;
 loop { match pc {
 0x4a95eb => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a95f3); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9645(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9645); }
 let mut pc: u32 = 0x4a9645;
 loop { match pc {
 0x4a9645 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a964d); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a964e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a964e); }
 let mut pc: u32 = 0x4a964e;
 loop { match pc {
 0x4a964e => {
  { let v: u32 = 0x4bdc60u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9667(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9667); }
 let mut pc: u32 = 0x4a9667;
 loop { match pc {
 0x4a9667 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4a967e; continue;
 }
 0x4a967e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9683(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9683); }
 let mut pc: u32 = 0x4a9683;
 loop { match pc {
 0x4a9683 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9698); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a970d; continue;
 }
 0x4a96af => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a96b8); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a96ca); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a96dc); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a96ee); f_004af810(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x10u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4a971c; continue;
 }
 0x4a96fc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a9726; continue;
 }
 0x4a970d => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a96af; continue; }
  pc = 0x4a9717; continue;
 }
 0x4a9717 => {
  pc = 0x4a96fc; continue;
 }
 0x4a971c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a9726; continue;
 }
 0x4a9726 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a972d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a972d); }
 let mut pc: u32 = 0x4a972d;
 loop { match pc {
 0x4a972d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9744); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9757); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a976a); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a977d); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9790); f_004af8e3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a979d; continue;
 }
 0x4a979d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a97a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a97a4); }
 let mut pc: u32 = 0x4a97a4;
 loop { match pc {
 0x4a97a4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a97b5; continue;
 }
 0x4a97b5 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xfffffffdu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x78u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = 0xfffffffdu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a97e9); cpu.call(t);
  pc = 0x4a97ee; continue;
 }
 0x4a97ee => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a97f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a97f3); }
 let mut pc: u32 = 0x4a97f3;
 loop { match pc {
 0x4a97f3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a98bcu32; cpu.push(v); }
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
  cpu.push(0x4a9831); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a9841); f_0049ae07(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a9864; continue; }
  pc = 0x4a9849; continue;
 }
 0x4a9849 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a985c); f_004a98b3(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a98c6; continue;
 }
 0x4a9864 => {
  pc = 0x4a9869; continue;
 }
 0x4a9869 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4a987d); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a98a2); f_004a98b3(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a98c6; continue;
 }
 0x4a98c6 => {
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
pub fn f_004a98aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a98aa); }
 let mut pc: u32 = 0x4a98aa;
 loop { match pc {
 0x4a98aa => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a98b2); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a98b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a98b3); }
 let mut pc: u32 = 0x4a98b3;
 loop { match pc {
 0x4a98b3 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a98bb); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a98bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a98bc); }
 let mut pc: u32 = 0x4a98bc;
 loop { match pc {
 0x4a98bc => {
  { let v: u32 = 0x4bdc90u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a98d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a98d7); }
 let mut pc: u32 = 0x4a98d7;
 loop { match pc {
 0x4a98d7 => {
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
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a9900); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9906); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4a991b); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a992f); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a997e; continue; }
  pc = 0x4a993e; continue;
 }
 0x4a993e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a994f); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a996a; continue; }
  pc = 0x4a995e; continue;
 }
 0x4a995e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)).wrapping_sub(0x18u32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a993e; continue;
 }
 0x4a996a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a99d9; continue;
 }
 0x4a997e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a998f); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a99d9; continue; }
  pc = 0x4a999e; continue;
 }
 0x4a999e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a99af); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a99ca; continue; }
  pc = 0x4a99be; continue;
 }
 0x4a99be => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)).wrapping_add(0x18u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a999e; continue;
 }
 0x4a99ca => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a99d9; continue;
 }
 0x4a99d9 => {
  pc = 0x4a99de; continue;
 }
 0x4a99de => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a99e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a99e5); }
 let mut pc: u32 = 0x4a99e5;
 loop { match pc {
 0x4a99e5 => {
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
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a9a02); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a9a16); cpu.call(t);
  { let v = cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74a0u32; cpu.push(v); }
  { let v = 0x4c27e8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a9a35); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9a3b); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4a9a50); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a9a64); cpu.call(t);
  { let v = cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x4a9a8a; continue; }
  pc = 0x4a9a7f; continue;
 }
 0x4a9a7f => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  pc = 0x4a9a90; continue;
 }
 0x4a9a8a => {
  { let v = cpu.rf32(0x4b38f0u32); cpu.fpush(v); }
  pc = 0x4a9a90; continue;
 }
 0x4a9a90 => {
  pc = 0x4a9a95; continue;
 }
 0x4a9a95 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9a9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9a9a); }
 let mut pc: u32 = 0x4a9a9a;
 loop { match pc {
 0x4a9a9a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xdcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a9ab7); f_004a99e5(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a9ad1); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffff24u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4a9aee); cpu.call(t);
  { let v = cpu.rf32(0xffffff24u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38e0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4a9b4b; continue; }
  pc = 0x4a9b06; continue;
 }
 0x4a9b06 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9b2d); f_004aedc7(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a9b39); f_00482756(cpu);
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x4a9bad; continue;
 }
 0x4a9b4b => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f4u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x4a9b89; continue; }
  pc = 0x4a9b60; continue;
 }
 0x4a9b60 => {
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff6cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a9b78); f_00482756(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x4a9bad; continue;
 }
 0x4a9b89 => {
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff2cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a9ba1); f_00482756(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x4a9bad; continue;
 }
 0x4a9bad => {
  pc = 0x4a9bb2; continue;
 }
 0x4a9bb2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9bb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9bb9); }
 let mut pc: u32 = 0x4a9bb9;
 loop { match pc {
 0x4a9bb9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a9de5u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x4a9c12; continue; }
  pc = 0x4a9beb; continue;
 }
 0x4a9beb => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3a58u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a9c07; continue;
 }
 0x4a9c07 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9c12; continue;
 }
 0x4a9c12 => {
  pc = 0x4a9c3b; continue;
 }
 0x4a9c3b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a9c45); f_0049b35e(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a9c58); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a9c6b; continue;
 }
 0x4a9c6b => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9c8a; continue;
 }
 0x4a9c8a => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a9c9a); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38c4u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a9cad; continue;
 }
 0x4a9cad => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9ccc; continue;
 }
 0x4a9ccc => {
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
  pc = 0x4a9d05; continue;
 }
 0x4a9d05 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9d4b; continue;
 }
 0x4a9d4b => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3a7cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3a64u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  { let v = 0x4f8368u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4a9daf); cpu.call(t);
  pc = 0x4a9db4; continue;
 }
 0x4a9db4 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a9def; continue;
 }
 0x4a9def => {
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
pub fn f_004a9c7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9c7d); }
 let mut pc: u32 = 0x4a9c7d;
 loop { match pc {
 0x4a9c7d => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a9c89); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9cbf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9cbf); }
 let mut pc: u32 = 0x4a9cbf;
 loop { match pc {
 0x4a9cbf => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a9ccb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9d14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9d14); }
 let mut pc: u32 = 0x4a9d14;
 loop { match pc {
 0x4a9d14 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x4a9d1f); f_004a44b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9d20); }
 let mut pc: u32 = 0x4a9d20;
 loop { match pc {
 0x4a9d20 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x4a9d2b); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9d2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9d2c); }
 let mut pc: u32 = 0x4a9d2c;
 loop { match pc {
 0x4a9d2c => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a9d34); f_004ad9f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9d35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9d35); }
 let mut pc: u32 = 0x4a9d35;
 loop { match pc {
 0x4a9d35 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4a9d4a; continue; }
  pc = 0x4a9d3f; continue;
 }
 0x4a9d3f => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x78u32); cpu.ecx = v; }
  cpu.push(0x4a9d4a); f_00485370(cpu);
  pc = 0x4a9d4a; continue;
 }
 0x4a9d4a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9dc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9dc3); }
 let mut pc: u32 = 0x4a9dc3;
 loop { match pc {
 0x4a9dc3 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a9dcb); f_004ad9c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9dcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9dcc); }
 let mut pc: u32 = 0x4a9dcc;
 loop { match pc {
 0x4a9dcc => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a9de4; continue; }
  pc = 0x4a9dd6; continue;
 }
 0x4a9dd6 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x80u32); cpu.ecx = v; }
  cpu.push(0x4a9de4); f_00485370(cpu);
  pc = 0x4a9de4; continue;
 }
 0x4a9de4 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9de5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9de5); }
 let mut pc: u32 = 0x4a9de5;
 loop { match pc {
 0x4a9de5 => {
  { let v: u32 = 0x4bdcc0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9e00); }
 let mut pc: u32 = 0x4a9e00;
 loop { match pc {
 0x4a9e00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a9ebbu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3a7cu32; cpu.w32(0xffffff80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3a64u32; cpu.w32(0xffffff84u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  if cpu.zf { pc = 0x4a9e99; continue; }
  pc = 0x4a9e6f; continue;
 }
 0x4a9e6f => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a9e99; continue; }
  pc = 0x4a9e88; continue;
 }
 0x4a9e88 => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a9e94); cpu.call(t);
  pc = 0x4a9e99; continue;
 }
 0x4a9e99 => {
  pc = 0x4a9e9e; continue;
 }
 0x4a9e9e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a9eaa); f_004a9eaf(cpu);
  pc = 0x4a9ec5; continue;
 }
 0x4a9ec5 => {
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
pub fn f_004a9eaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9eaf); }
 let mut pc: u32 = 0x4a9eaf;
 loop { match pc {
 0x4a9eaf => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a9eba); f_004a25b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9ebb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9ebb); }
 let mut pc: u32 = 0x4a9ebb;
 loop { match pc {
 0x4a9ebb => {
  { let v: u32 = 0x4bdd20u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a9ed3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a9ed3); }
 let mut pc: u32 = 0x4a9ed3;
 loop { match pc {
 0x4a9ed3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4aa064u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4a9efd); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4aa04d; continue; }
  pc = 0x4a9f10; continue;
 }
 0x4a9f10 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9f2b); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a9f76; continue; }
  pc = 0x4a9f42; continue;
 }
 0x4a9f42 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4adea0u32; cpu.push(v); }
  { let v = 0x4aa4a2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x18u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9f68); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9f7d; continue;
 }
 0x4a9f76 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9f7d; continue;
 }
 0x4a9f7d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x78u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a9fa1; continue; }
  pc = 0x4a9f9a; continue;
 }
 0x4a9f9a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4aa06e; continue;
 }
 0x4a9fa1 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9faa; continue;
 }
 0x4a9faa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4a9fb3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4aa04d; continue; }
  pc = 0x4a9fbb; continue;
 }
 0x4a9fbb => {
  pc = 0x4a9fc0; continue;
 }
 0x4a9fc0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9fd1; continue;
 }
 0x4a9fd1 => {
  pc = 0x4a9fd6; continue;
 }
 0x4a9fd6 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4aa009; continue;
 }
 0x4aa009 => {
  pc = 0x4aa00e; continue;
 }
 0x4aa00e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aa03b; continue; }
  pc = 0x4aa02a; continue;
 }
 0x4aa02a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4aa036); cpu.call(t);
  pc = 0x4aa03b; continue;
 }
 0x4aa03b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x4aa044); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0x18u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9faa; continue;
 }
 0x4aa04d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4aa06e; continue;
 }
 0x4aa06e => {
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
pub fn f_004aa057(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa057); }
 let mut pc: u32 = 0x4aa057;
 loop { match pc {
 0x4aa057 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa060); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa064(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa064); }
 let mut pc: u32 = 0x4aa064;
 loop { match pc {
 0x4aa064 => {
  { let v: u32 = 0x4bdd48u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa07f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa07f); }
 let mut pc: u32 = 0x4aa07f;
 loop { match pc {
 0x4aa07f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4aa096; continue;
 }
 0x4aa096 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa09b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa09b); }
 let mut pc: u32 = 0x4aa09b;
 loop { match pc {
 0x4aa09b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4aa0b2; continue;
 }
 0x4aa0b2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa3c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa3c1); }
 let mut pc: u32 = 0x4aa3c1;
 loop { match pc {
 0x4aa3c1 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa3ca); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa3ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa3ce); }
 let mut pc: u32 = 0x4aa3ce;
 loop { match pc {
 0x4aa3ce => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4aa3d7); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa3d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa3d8); }
 let mut pc: u32 = 0x4aa3d8;
 loop { match pc {
 0x4aa3d8 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4aa3e1); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa3e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa3e2); }
 let mut pc: u32 = 0x4aa3e2;
 loop { match pc {
 0x4aa3e2 => {
  { let v: u32 = 0x4bdd70u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa4a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa4a2); }
 let mut pc: u32 = 0x4aa4a2;
 loop { match pc {
 0x4aa4a2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4aa565u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aa4d1; continue;
 }
 0x4aa4d1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b39dcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aa4e6; continue;
 }
 0x4aa4e6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa501; continue;
 }
 0x4aa501 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aa535; continue;
 }
 0x4aa535 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3a9cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aa54d; continue;
 }
 0x4aa54d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4aa56f; continue;
 }
 0x4aa56f => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa4f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa4f8); }
 let mut pc: u32 = 0x4aa4f8;
 loop { match pc {
 0x4aa4f8 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4aa500); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa55c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa55c); }
 let mut pc: u32 = 0x4aa55c;
 loop { match pc {
 0x4aa55c => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4aa564); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa565(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa565); }
 let mut pc: u32 = 0x4aa565;
 loop { match pc {
 0x4aa565 => {
  { let v: u32 = 0x4bdda8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa57e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa57e); }
 let mut pc: u32 = 0x4aa57e;
 loop { match pc {
 0x4aa57e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4aa595; continue;
 }
 0x4aa595 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa59a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa59a); }
 let mut pc: u32 = 0x4aa59a;
 loop { match pc {
 0x4aa59a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa5af); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa636; continue;
 }
 0x4aa5c6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa5cf); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa5e1); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa5f3); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa605); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x10u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa617); f_004af810(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4aa645; continue;
 }
 0x4aa625 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4aa64f; continue;
 }
 0x4aa636 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aa5c6; continue; }
  pc = 0x4aa640; continue;
 }
 0x4aa640 => {
  pc = 0x4aa625; continue;
 }
 0x4aa645 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4aa64f; continue;
 }
 0x4aa64f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa656(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa656); }
 let mut pc: u32 = 0x4aa656;
 loop { match pc {
 0x4aa656 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa66d); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa680); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa693); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa6a6); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa6b9); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa6cc); f_004af8e3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4aa6d9; continue;
 }
 0x4aa6d9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa6e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa6e0); }
 let mut pc: u32 = 0x4aa6e0;
 loop { match pc {
 0x4aa6e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa6f1; continue;
 }
 0x4aa6f1 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x78u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aa72e); cpu.call(t);
  pc = 0x4aa733; continue;
 }
 0x4aa733 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa738(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa738); }
 let mut pc: u32 = 0x4aa738;
 loop { match pc {
 0x4aa738 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4aa801u32; cpu.push(v); }
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
  cpu.push(0x4aa776); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aa786); f_0049ae07(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4aa7a9; continue; }
  pc = 0x4aa78e; continue;
 }
 0x4aa78e => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aa7a1); f_004aa7f8(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4aa80b; continue;
 }
 0x4aa7a9 => {
  pc = 0x4aa7ae; continue;
 }
 0x4aa7ae => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4aa7c2); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aa7e7); f_004aa7f8(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4aa80b; continue;
 }
 0x4aa80b => {
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
pub fn f_004aa7ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa7ef); }
 let mut pc: u32 = 0x4aa7ef;
 loop { match pc {
 0x4aa7ef => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4aa7f7); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa7f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa7f8); }
 let mut pc: u32 = 0x4aa7f8;
 loop { match pc {
 0x4aa7f8 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4aa800); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa801(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa801); }
 let mut pc: u32 = 0x4aa801;
 loop { match pc {
 0x4aa801 => {
  { let v: u32 = 0x4bddd8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa81c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa81c); }
 let mut pc: u32 = 0x4aa81c;
 loop { match pc {
 0x4aa81c => {
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
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4aa845); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa84b); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4aa860); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aa874); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4aa8c3; continue; }
  pc = 0x4aa883; continue;
 }
 0x4aa883 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aa894); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4aa8af; continue; }
  pc = 0x4aa8a3; continue;
 }
 0x4aa8a3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)).wrapping_sub(0x14u32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aa883; continue;
 }
 0x4aa8af => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4aa91e; continue;
 }
 0x4aa8c3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aa8d4); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4aa91e; continue; }
  pc = 0x4aa8e3; continue;
 }
 0x4aa8e3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aa8f4); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4aa90f; continue; }
  pc = 0x4aa903; continue;
 }
 0x4aa903 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)).wrapping_add(0x14u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aa8e3; continue;
 }
 0x4aa90f => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4aa91e; continue;
 }
 0x4aa91e => {
  pc = 0x4aa923; continue;
 }
 0x4aa923 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa92a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa92a); }
 let mut pc: u32 = 0x4aa92a;
 loop { match pc {
 0x4aa92a => {
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
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aa947); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aa95b); cpu.call(t);
  { let v = cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74a0u32; cpu.push(v); }
  { let v = 0x4c27e8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4aa97a); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa980); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4aa995); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aa9a9); cpu.call(t);
  { let v = cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x4aa9cf; continue; }
  pc = 0x4aa9c4; continue;
 }
 0x4aa9c4 => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  pc = 0x4aa9d5; continue;
 }
 0x4aa9cf => {
  { let v = cpu.rf32(0x4b38f0u32); cpu.fpush(v); }
  pc = 0x4aa9d5; continue;
 }
 0x4aa9d5 => {
  pc = 0x4aa9da; continue;
 }
 0x4aa9da => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aa9df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aa9df); }
 let mut pc: u32 = 0x4aa9df;
 loop { match pc {
 0x4aa9df => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4aa9f3); f_004aa92a(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aaa07); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffccu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aaa1b); cpu.call(t);
  { let v = cpu.rf32(0xffffffccu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38e0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4aaadc; continue; }
  pc = 0x4aaa36; continue;
 }
 0x4aaa36 => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x4b38f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf32(0x4b38fcu32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffd8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38f8u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aab12; continue;
 }
 0x4aaadc => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f4u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x4aab04; continue; }
  pc = 0x4aaaf1; continue;
 }
 0x4aaaf1 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4aab12; continue;
 }
 0x4aab04 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4aab12; continue;
 }
 0x4aab12 => {
  pc = 0x4aab17; continue;
 }
 0x4aab17 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aab1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aab1e); }
 let mut pc: u32 = 0x4aab1e;
 loop { match pc {
 0x4aab1e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4aad4au32; cpu.push(v); }
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
  if cpu.zf { pc = 0x4aab77; continue; }
  pc = 0x4aab50; continue;
 }
 0x4aab50 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3ab8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aab6c; continue;
 }
 0x4aab6c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aab77; continue;
 }
 0x4aab77 => {
  pc = 0x4aaba0; continue;
 }
 0x4aaba0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4aabaa); f_0049b35e(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4aabbd); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aabd0; continue;
 }
 0x4aabd0 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aabef; continue;
 }
 0x4aabef => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4aabff); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38c4u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x4aac12; continue;
 }
 0x4aac12 => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aac31; continue;
 }
 0x4aac31 => {
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
  pc = 0x4aac6a; continue;
 }
 0x4aac6a => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aacb0; continue;
 }
 0x4aacb0 => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3adcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3ac4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  { let v = 0x4f8394u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4aad14); cpu.call(t);
  pc = 0x4aad19; continue;
 }
 0x4aad19 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4aad54; continue;
 }
 0x4aad54 => {
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
pub fn f_004aabe2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aabe2); }
 let mut pc: u32 = 0x4aabe2;
 loop { match pc {
 0x4aabe2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4aabee); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aac24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aac24); }
 let mut pc: u32 = 0x4aac24;
 loop { match pc {
 0x4aac24 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4aac30); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aac79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aac79); }
 let mut pc: u32 = 0x4aac79;
 loop { match pc {
 0x4aac79 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x4aac84); f_004a44b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aac85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aac85); }
 let mut pc: u32 = 0x4aac85;
 loop { match pc {
 0x4aac85 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x4aac90); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aac91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aac91); }
 let mut pc: u32 = 0x4aac91;
 loop { match pc {
 0x4aac91 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4aac99); f_004ad9f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aac9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aac9a); }
 let mut pc: u32 = 0x4aac9a;
 loop { match pc {
 0x4aac9a => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4aacaf; continue; }
  pc = 0x4aaca4; continue;
 }
 0x4aaca4 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x78u32); cpu.ecx = v; }
  cpu.push(0x4aacaf); f_00485370(cpu);
  pc = 0x4aacaf; continue;
 }
 0x4aacaf => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aad28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aad28); }
 let mut pc: u32 = 0x4aad28;
 loop { match pc {
 0x4aad28 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4aad30); f_004ad9c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aad31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aad31); }
 let mut pc: u32 = 0x4aad31;
 loop { match pc {
 0x4aad31 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4aad49; continue; }
  pc = 0x4aad3b; continue;
 }
 0x4aad3b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x80u32); cpu.ecx = v; }
  cpu.push(0x4aad49); f_00485370(cpu);
  pc = 0x4aad49; continue;
 }
 0x4aad49 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aad4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aad4a); }
 let mut pc: u32 = 0x4aad4a;
 loop { match pc {
 0x4aad4a => {
  { let v: u32 = 0x4bde08u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aad65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aad65); }
 let mut pc: u32 = 0x4aad65;
 loop { match pc {
 0x4aad65 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4aae20u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3adcu32; cpu.w32(0xffffff80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3ac4u32; cpu.w32(0xffffff84u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  if cpu.zf { pc = 0x4aadfe; continue; }
  pc = 0x4aadd4; continue;
 }
 0x4aadd4 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aadfe; continue; }
  pc = 0x4aaded; continue;
 }
 0x4aaded => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4aadf9); cpu.call(t);
  pc = 0x4aadfe; continue;
 }
 0x4aadfe => {
  pc = 0x4aae03; continue;
 }
 0x4aae03 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aae0f); f_004aae14(cpu);
  pc = 0x4aae2a; continue;
 }
 0x4aae2a => {
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
pub fn f_004aae14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aae14); }
 let mut pc: u32 = 0x4aae14;
 loop { match pc {
 0x4aae14 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4aae1f); f_004a25b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aae20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aae20); }
 let mut pc: u32 = 0x4aae20;
 loop { match pc {
 0x4aae20 => {
  { let v: u32 = 0x4bde68u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aae38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aae38); }
 let mut pc: u32 = 0x4aae38;
 loop { match pc {
 0x4aae38 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4aafcbu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4aae62); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4aafb4; continue; }
  pc = 0x4aae75; continue;
 }
 0x4aae75 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aae90); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aaedb; continue; }
  pc = 0x4aaea7; continue;
 }
 0x4aaea7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4adfc0u32; cpu.push(v); }
  { let v = 0x4ab40du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aaecd); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aaee2; continue;
 }
 0x4aaedb => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aaee2; continue;
 }
 0x4aaee2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x78u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4aaf06; continue; }
  pc = 0x4aaeff; continue;
 }
 0x4aaeff => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4aafd5; continue;
 }
 0x4aaf06 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aaf0f; continue;
 }
 0x4aaf0f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4aaf18); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4aafb4; continue; }
  pc = 0x4aaf20; continue;
 }
 0x4aaf20 => {
  pc = 0x4aaf25; continue;
 }
 0x4aaf25 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aaf36; continue;
 }
 0x4aaf36 => {
  pc = 0x4aaf3b; continue;
 }
 0x4aaf3b => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4aaf70; continue;
 }
 0x4aaf70 => {
  pc = 0x4aaf75; continue;
 }
 0x4aaf75 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aafa2; continue; }
  pc = 0x4aaf91; continue;
 }
 0x4aaf91 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4aaf9d); cpu.call(t);
  pc = 0x4aafa2; continue;
 }
 0x4aafa2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x4aafab); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0x14u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aaf0f; continue;
 }
 0x4aafb4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4aafd5; continue;
 }
 0x4aafd5 => {
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
pub fn f_004aafbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aafbe); }
 let mut pc: u32 = 0x4aafbe;
 loop { match pc {
 0x4aafbe => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aafc7); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aafcb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aafcb); }
 let mut pc: u32 = 0x4aafcb;
 loop { match pc {
 0x4aafcb => {
  { let v: u32 = 0x4bde90u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aafe6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aafe6); }
 let mut pc: u32 = 0x4aafe6;
 loop { match pc {
 0x4aafe6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4aaffd; continue;
 }
 0x4aaffd => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab002(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab002); }
 let mut pc: u32 = 0x4ab002;
 loop { match pc {
 0x4ab002 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4ab019; continue;
 }
 0x4ab019 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab32c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab32c); }
 let mut pc: u32 = 0x4ab32c;
 loop { match pc {
 0x4ab32c => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab335); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab339(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab339); }
 let mut pc: u32 = 0x4ab339;
 loop { match pc {
 0x4ab339 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ab342); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab343(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab343); }
 let mut pc: u32 = 0x4ab343;
 loop { match pc {
 0x4ab343 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ab34c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab34d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab34d); }
 let mut pc: u32 = 0x4ab34d;
 loop { match pc {
 0x4ab34d => {
  { let v: u32 = 0x4bdeb8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab40d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab40d); }
 let mut pc: u32 = 0x4ab40d;
 loop { match pc {
 0x4ab40d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ab4c1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ab43c; continue;
 }
 0x4ab43c => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b39dcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ab451; continue;
 }
 0x4ab451 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab46c; continue;
 }
 0x4ab46c => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3afcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ab4a9; continue;
 }
 0x4ab4a9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ab4cb; continue;
 }
 0x4ab4cb => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab463(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab463); }
 let mut pc: u32 = 0x4ab463;
 loop { match pc {
 0x4ab463 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ab46b); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab4b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab4b8); }
 let mut pc: u32 = 0x4ab4b8;
 loop { match pc {
 0x4ab4b8 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ab4c0); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab4c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab4c1); }
 let mut pc: u32 = 0x4ab4c1;
 loop { match pc {
 0x4ab4c1 => {
  { let v: u32 = 0x4bdef0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab4da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab4da); }
 let mut pc: u32 = 0x4ab4da;
 loop { match pc {
 0x4ab4da => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4ab4f1; continue;
 }
 0x4ab4f1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab4f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab4f6); }
 let mut pc: u32 = 0x4ab4f6;
 loop { match pc {
 0x4ab4f6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab50b); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab580; continue;
 }
 0x4ab522 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab52b); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab53d); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab54f); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab561); f_004af810(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x10u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4ab58f; continue;
 }
 0x4ab56f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ab599; continue;
 }
 0x4ab580 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ab522; continue; }
  pc = 0x4ab58a; continue;
 }
 0x4ab58a => {
  pc = 0x4ab56f; continue;
 }
 0x4ab58f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ab599; continue;
 }
 0x4ab599 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab5a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab5a0); }
 let mut pc: u32 = 0x4ab5a0;
 loop { match pc {
 0x4ab5a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab5b7); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab5ca); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab5dd); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab5f0); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab603); f_004af8e3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ab610; continue;
 }
 0x4ab610 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab617(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab617); }
 let mut pc: u32 = 0x4ab617;
 loop { match pc {
 0x4ab617 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab628; continue;
 }
 0x4ab628 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x78u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab667); cpu.call(t);
  pc = 0x4ab66c; continue;
 }
 0x4ab66c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab671(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab671); }
 let mut pc: u32 = 0x4ab671;
 loop { match pc {
 0x4ab671 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ab73au32; cpu.push(v); }
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
  cpu.push(0x4ab6af); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ab6bf); f_0049ae07(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ab6e2; continue; }
  pc = 0x4ab6c7; continue;
 }
 0x4ab6c7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ab6da); f_004ab731(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ab744; continue;
 }
 0x4ab6e2 => {
  pc = 0x4ab6e7; continue;
 }
 0x4ab6e7 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4ab6fb); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ab720); f_004ab731(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ab744; continue;
 }
 0x4ab744 => {
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
pub fn f_004ab728(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab728); }
 let mut pc: u32 = 0x4ab728;
 loop { match pc {
 0x4ab728 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ab730); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab731(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab731); }
 let mut pc: u32 = 0x4ab731;
 loop { match pc {
 0x4ab731 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4ab739); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab73a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab73a); }
 let mut pc: u32 = 0x4ab73a;
 loop { match pc {
 0x4ab73a => {
  { let v: u32 = 0x4bdf20u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab755(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab755); }
 let mut pc: u32 = 0x4ab755;
 loop { match pc {
 0x4ab755 => {
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
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4ab77e); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab784); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4ab799); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab7ad); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4ab7fc; continue; }
  pc = 0x4ab7bc; continue;
 }
 0x4ab7bc => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab7cd); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4ab7e8; continue; }
  pc = 0x4ab7dc; continue;
 }
 0x4ab7dc => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)).wrapping_sub(0x2cu32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ab7bc; continue;
 }
 0x4ab7e8 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ab857; continue;
 }
 0x4ab7fc => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab80d); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4ab857; continue; }
  pc = 0x4ab81c; continue;
 }
 0x4ab81c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab82d); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4ab848; continue; }
  pc = 0x4ab83c; continue;
 }
 0x4ab83c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)).wrapping_add(0x2cu32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ab81c; continue;
 }
 0x4ab848 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ab857; continue;
 }
 0x4ab857 => {
  pc = 0x4ab85c; continue;
 }
 0x4ab85c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab863(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab863); }
 let mut pc: u32 = 0x4ab863;
 loop { match pc {
 0x4ab863 => {
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
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab880); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab894); cpu.call(t);
  { let v = cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74a0u32; cpu.push(v); }
  { let v = 0x4c27e8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4ab8b3); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab8b9); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4ab8ce); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab8e2); cpu.call(t);
  { let v = cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x4ab908; continue; }
  pc = 0x4ab8fd; continue;
 }
 0x4ab8fd => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  pc = 0x4ab90e; continue;
 }
 0x4ab908 => {
  { let v = cpu.rf32(0x4b38f0u32); cpu.fpush(v); }
  pc = 0x4ab90e; continue;
 }
 0x4ab90e => {
  pc = 0x4ab913; continue;
 }
 0x4ab913 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ab918(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ab918); }
 let mut pc: u32 = 0x4ab918;
 loop { match pc {
 0x4ab918 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ab92c); f_004ab863(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab940); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffbcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ab954); cpu.call(t);
  { let v = cpu.rf32(0xffffffbcu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38e0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4aba90; continue; }
  pc = 0x4ab96f; continue;
 }
 0x4ab96f => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x4b38f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf32(0x4b38fcu32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffe4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab9b0; continue;
 }
 0x4ab9b0 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab9bf; continue;
 }
 0x4ab9bc => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab9bf; continue;
 }
 0x4ab9bf => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x3u32); }
  if !cpu.cf { pc = 0x4aba75; continue; }
  pc = 0x4ab9c9; continue;
 }
 0x4ab9c9 => {
  pc = 0x4ab9ce; continue;
 }
 0x4ab9ce => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab9e3; continue;
 }
 0x4ab9e3 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab9f8; continue;
 }
 0x4ab9f8 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.rf32(0x20u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffd0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4aba13; continue;
 }
 0x4aba13 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffccu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aba38; continue;
 }
 0x4aba38 => {
  { let v = cpu.rf32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38f8u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b38f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4ab9bc; continue;
 }
 0x4aba75 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4abae2; continue;
 }
 0x4aba90 => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f4u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x4abac6; continue; }
  pc = 0x4abaa5; continue;
 }
 0x4abaa5 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4abae2; continue;
 }
 0x4abac6 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4abae2; continue;
 }
 0x4abae2 => {
  pc = 0x4abae7; continue;
 }
 0x4abae7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abaee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abaee); }
 let mut pc: u32 = 0x4abaee;
 loop { match pc {
 0x4abaee => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4abd1au32; cpu.push(v); }
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
  if cpu.zf { pc = 0x4abb47; continue; }
  pc = 0x4abb20; continue;
 }
 0x4abb20 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3b18u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x4abb3c; continue;
 }
 0x4abb3c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abb47; continue;
 }
 0x4abb47 => {
  pc = 0x4abb70; continue;
 }
 0x4abb70 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4abb7a); f_0049b35e(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4abb8d); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4abba0; continue;
 }
 0x4abba0 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abbbf; continue;
 }
 0x4abbbf => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4abbcf); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38c4u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x4abbe2; continue;
 }
 0x4abbe2 => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abc01; continue;
 }
 0x4abc01 => {
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
  pc = 0x4abc3a; continue;
 }
 0x4abc3a => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abc80; continue;
 }
 0x4abc80 => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3b3cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3b24u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  { let v = 0x4f83b8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4abce4); cpu.call(t);
  pc = 0x4abce9; continue;
 }
 0x4abce9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4abd24; continue;
 }
 0x4abd24 => {
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
pub fn f_004abbb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abbb2); }
 let mut pc: u32 = 0x4abbb2;
 loop { match pc {
 0x4abbb2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4abbbe); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abbf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abbf4); }
 let mut pc: u32 = 0x4abbf4;
 loop { match pc {
 0x4abbf4 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4abc00); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abc49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abc49); }
 let mut pc: u32 = 0x4abc49;
 loop { match pc {
 0x4abc49 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x4abc54); f_004a44b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abc55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abc55); }
 let mut pc: u32 = 0x4abc55;
 loop { match pc {
 0x4abc55 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x4abc60); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abc61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abc61); }
 let mut pc: u32 = 0x4abc61;
 loop { match pc {
 0x4abc61 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4abc69); f_004ad9f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abc6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abc6a); }
 let mut pc: u32 = 0x4abc6a;
 loop { match pc {
 0x4abc6a => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4abc7f; continue; }
  pc = 0x4abc74; continue;
 }
 0x4abc74 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x78u32); cpu.ecx = v; }
  cpu.push(0x4abc7f); f_00485370(cpu);
  pc = 0x4abc7f; continue;
 }
 0x4abc7f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abcf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abcf8); }
 let mut pc: u32 = 0x4abcf8;
 loop { match pc {
 0x4abcf8 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4abd00); f_004ad9c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abd01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abd01); }
 let mut pc: u32 = 0x4abd01;
 loop { match pc {
 0x4abd01 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4abd19; continue; }
  pc = 0x4abd0b; continue;
 }
 0x4abd0b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x80u32); cpu.ecx = v; }
  cpu.push(0x4abd19); f_00485370(cpu);
  pc = 0x4abd19; continue;
 }
 0x4abd19 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abd1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abd1a); }
 let mut pc: u32 = 0x4abd1a;
 loop { match pc {
 0x4abd1a => {
  { let v: u32 = 0x4bdf50u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abd35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abd35); }
 let mut pc: u32 = 0x4abd35;
 loop { match pc {
 0x4abd35 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4abdf0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3b3cu32; cpu.w32(0xffffff80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3b24u32; cpu.w32(0xffffff84u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  if cpu.zf { pc = 0x4abdce; continue; }
  pc = 0x4abda4; continue;
 }
 0x4abda4 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4abdce; continue; }
  pc = 0x4abdbd; continue;
 }
 0x4abdbd => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4abdc9); cpu.call(t);
  pc = 0x4abdce; continue;
 }
 0x4abdce => {
  pc = 0x4abdd3; continue;
 }
 0x4abdd3 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4abddf); f_004abde4(cpu);
  pc = 0x4abdfa; continue;
 }
 0x4abdfa => {
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
pub fn f_004abde4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abde4); }
 let mut pc: u32 = 0x4abde4;
 loop { match pc {
 0x4abde4 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4abdef); f_004a25b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abdf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abdf0); }
 let mut pc: u32 = 0x4abdf0;
 loop { match pc {
 0x4abdf0 => {
  { let v: u32 = 0x4bdfb0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abe08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abe08); }
 let mut pc: u32 = 0x4abe08;
 loop { match pc {
 0x4abe08 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4abfcfu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4abe32); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4abfb8; continue; }
  pc = 0x4abe45; continue;
 }
 0x4abe45 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4abe64); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4abeaf; continue; }
  pc = 0x4abe7b; continue;
 }
 0x4abe7b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4ae0e0u32; cpu.push(v); }
  { let v = 0x4ac449u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4abea1); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abeb6; continue;
 }
 0x4abeaf => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abeb6; continue;
 }
 0x4abeb6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x78u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4abeda; continue; }
  pc = 0x4abed3; continue;
 }
 0x4abed3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4abfd9; continue;
 }
 0x4abeda => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abee3; continue;
 }
 0x4abee3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4abeec); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4abfb8; continue; }
  pc = 0x4abef4; continue;
 }
 0x4abef4 => {
  pc = 0x4abef9; continue;
 }
 0x4abef9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abf0a; continue;
 }
 0x4abf0a => {
  pc = 0x4abf0f; continue;
 }
 0x4abf0f => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4abf74; continue;
 }
 0x4abf74 => {
  pc = 0x4abf79; continue;
 }
 0x4abf79 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4abfa6; continue; }
  pc = 0x4abf95; continue;
 }
 0x4abf95 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4abfa1); cpu.call(t);
  pc = 0x4abfa6; continue;
 }
 0x4abfa6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x4abfaf); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0x2cu32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4abee3; continue;
 }
 0x4abfb8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4abfd9; continue;
 }
 0x4abfd9 => {
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
pub fn f_004abfc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abfc2); }
 let mut pc: u32 = 0x4abfc2;
 loop { match pc {
 0x4abfc2 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4abfcb); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abfcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abfcf); }
 let mut pc: u32 = 0x4abfcf;
 loop { match pc {
 0x4abfcf => {
  { let v: u32 = 0x4bdfd8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004abfea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4abfea); }
 let mut pc: u32 = 0x4abfea;
 loop { match pc {
 0x4abfea => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4ac001; continue;
 }
 0x4ac001 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac006(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac006); }
 let mut pc: u32 = 0x4ac006;
 loop { match pc {
 0x4ac006 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4ac01d; continue;
 }
 0x4ac01d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac0fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac0fb); }
 let mut pc: u32 = 0x4ac0fb;
 loop { match pc {
 0x4ac0fb => {
  { cpu.lods(1, false); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4ac106); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac10e; continue;
 }
 0x4ac10e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac117); f_004ac375(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ac142; continue; }
  pc = 0x4ac127; continue;
 }
 0x4ac127 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac13a); f_004ac37f(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac142 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac161); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ac1a9; continue; }
  pc = 0x4ac175; continue;
 }
 0x4ac175 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4ae0e0u32; cpu.push(v); }
  { let v = 0x4ac449u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac19b); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac1b0; continue;
 }
 0x4ac1a9 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac1b0; continue;
 }
 0x4ac1b0 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4ac1e5; continue; }
  pc = 0x4ac1ca; continue;
 }
 0x4ac1ca => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac1dd); f_004ac37f(cpu);
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac1e5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac1f4; continue;
 }
 0x4ac1f1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac1f4; continue;
 }
 0x4ac1f4 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4ac305; continue; }
  pc = 0x4ac203; continue;
 }
 0x4ac203 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f73a0u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac21e); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac227); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4ac252; continue; }
  pc = 0x4ac237; continue;
 }
 0x4ac237 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac24a); f_004ac37f(cpu);
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac252 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac26f; continue;
 }
 0x4ac26f => {
  pc = 0x4ac274; continue;
 }
 0x4ac274 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ac2d9; continue;
 }
 0x4ac2d9 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ac300; continue; }
  pc = 0x4ac2ef; continue;
 }
 0x4ac2ef => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4ac2fb); cpu.call(t);
  pc = 0x4ac300; continue;
 }
 0x4ac300 => {
  pc = 0x4ac1f1; continue;
 }
 0x4ac305 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac311); f_004ac37f(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4ac320); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ac32f; continue; }
  pc = 0x4ac328; continue;
 }
 0x4ac328 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac32f => {
  pc = 0x4ac35e; continue;
 }
 0x4ac35e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac393 => {
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
pub fn f_004ac368(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac368); }
 let mut pc: u32 = 0x4ac368;
 loop { match pc {
 0x4ac368 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac371); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac375(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac375); }
 let mut pc: u32 = 0x4ac375;
 loop { match pc {
 0x4ac375 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ac37e); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac37f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac37f); }
 let mut pc: u32 = 0x4ac37f;
 loop { match pc {
 0x4ac37f => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ac388); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac389(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac389); }
 let mut pc: u32 = 0x4ac389;
 loop { match pc {
 0x4ac389 => {
  { let v: u32 = 0x4be000u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac449(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac449); }
 let mut pc: u32 = 0x4ac449;
 loop { match pc {
 0x4ac449 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ac548u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ac478; continue;
 }
 0x4ac478 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b39dcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ac48d; continue;
 }
 0x4ac48d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac4a8; continue;
 }
 0x4ac4a8 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ac4d2; continue;
 }
 0x4ac4d2 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  pc = 0x4ac4f5; continue;
 }
 0x4ac4f5 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ac518; continue;
 }
 0x4ac518 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3b5cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ac530; continue;
 }
 0x4ac530 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac552; continue;
 }
 0x4ac552 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac49f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac49f); }
 let mut pc: u32 = 0x4ac49f;
 loop { match pc {
 0x4ac49f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ac4a7); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac53f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac53f); }
 let mut pc: u32 = 0x4ac53f;
 loop { match pc {
 0x4ac53f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ac547); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac548(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac548); }
 let mut pc: u32 = 0x4ac548;
 loop { match pc {
 0x4ac548 => {
  { let v: u32 = 0x4be038u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac561(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac561); }
 let mut pc: u32 = 0x4ac561;
 loop { match pc {
 0x4ac561 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4ac578; continue;
 }
 0x4ac578 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac57d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac57d); }
 let mut pc: u32 = 0x4ac57d;
 loop { match pc {
 0x4ac57d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac592); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac673; continue;
 }
 0x4ac5a9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac5b2); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac5c4); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac5d6); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac5e8); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x10u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac5fa); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac60c); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac61e); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac630); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac642); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac654); f_004af810(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4ac682; continue;
 }
 0x4ac662 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ac68c; continue;
 }
 0x4ac673 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ac5a9; continue; }
  pc = 0x4ac67d; continue;
 }
 0x4ac67d => {
  pc = 0x4ac662; continue;
 }
 0x4ac682 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ac68c; continue;
 }
 0x4ac68c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac693(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac693); }
 let mut pc: u32 = 0x4ac693;
 loop { match pc {
 0x4ac693 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac6aa); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac6bd); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac6d0); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac6e3); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac6f6); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac709); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac71c); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac72f); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac742); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac755); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac768); f_004af8e3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ac775; continue;
 }
 0x4ac775 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac77c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac77c); }
 let mut pc: u32 = 0x4ac77c;
 loop { match pc {
 0x4ac77c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac78d; continue;
 }
 0x4ac78d => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x78u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ac7c9); cpu.call(t);
  pc = 0x4ac7ce; continue;
 }
 0x4ac7ce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac7d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac7d3); }
 let mut pc: u32 = 0x4ac7d3;
 loop { match pc {
 0x4ac7d3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ac89cu32; cpu.push(v); }
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
  cpu.push(0x4ac811); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac821); f_0049ae07(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ac844; continue; }
  pc = 0x4ac829; continue;
 }
 0x4ac829 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac83c); f_004ac893(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac8a6; continue;
 }
 0x4ac844 => {
  pc = 0x4ac849; continue;
 }
 0x4ac849 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4ac85d); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac882); f_004ac893(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac8a6; continue;
 }
 0x4ac8a6 => {
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
pub fn f_004ac88a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac88a); }
 let mut pc: u32 = 0x4ac88a;
 loop { match pc {
 0x4ac88a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ac892); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac893(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac893); }
 let mut pc: u32 = 0x4ac893;
 loop { match pc {
 0x4ac893 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4ac89b); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac89c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac89c); }
 let mut pc: u32 = 0x4ac89c;
 loop { match pc {
 0x4ac89c => {
  { let v: u32 = 0x4be068u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac8b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac8b7); }
 let mut pc: u32 = 0x4ac8b7;
 loop { match pc {
 0x4ac8b7 => {
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
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4ac8e0); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac8e6); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4ac8fb); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ac90f); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4ac95e; continue; }
  pc = 0x4ac91e; continue;
 }
 0x4ac91e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ac92f); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4ac94a; continue; }
  pc = 0x4ac93e; continue;
 }
 0x4ac93e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)).wrapping_sub(0x38u32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ac91e; continue;
 }
 0x4ac94a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x38u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ac9b9; continue;
 }
 0x4ac95e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ac96f); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4ac9b9; continue; }
  pc = 0x4ac97e; continue;
 }
 0x4ac97e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ac98f); cpu.call(t);
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4ac9aa; continue; }
  pc = 0x4ac99e; continue;
 }
 0x4ac99e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)).wrapping_add(0x38u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ac97e; continue;
 }
 0x4ac9aa => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x38u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ac9b9; continue;
 }
 0x4ac9b9 => {
  pc = 0x4ac9be; continue;
 }
 0x4ac9be => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ac9c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ac9c5); }
 let mut pc: u32 = 0x4ac9c5;
 loop { match pc {
 0x4ac9c5 => {
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
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ac9e2); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4ac9f6); cpu.call(t);
  { let v = cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74a0u32; cpu.push(v); }
  { let v = 0x4c27e8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4aca15); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aca1b); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x4aca30); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4aca44); cpu.call(t);
  { let v = cpu.rf32(0xffffffe8u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x4aca6a; continue; }
  pc = 0x4aca5f; continue;
 }
 0x4aca5f => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  pc = 0x4aca70; continue;
 }
 0x4aca6a => {
  { let v = cpu.rf32(0x4b38f0u32); cpu.fpush(v); }
  pc = 0x4aca70; continue;
 }
 0x4aca70 => {
  pc = 0x4aca75; continue;
 }
 0x4aca75 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aca7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aca7a); }
 let mut pc: u32 = 0x4aca7a;
 loop { match pc {
 0x4aca7a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xdcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4aca97); f_004ac9c5(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4acab1); cpu.call(t);
  { let v = cpu.st(0); cpu.wf32(0xffffff24u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.push(0x4acace); cpu.call(t);
  { let v = cpu.rf32(0xffffff24u32.wrapping_add(cpu.ebp)) - cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38e0u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x4acb45; continue; }
  pc = 0x4acae6; continue;
 }
 0x4acae6 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x28u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4acb27); f_004aef75(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4acb33); f_00482756(cpu);
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x4acba7; continue;
 }
 0x4acb45 => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b38f4u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x4acb83; continue; }
  pc = 0x4acb5a; continue;
 }
 0x4acb5a => {
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff6cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4acb72); f_00482756(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x4acba7; continue;
 }
 0x4acb83 => {
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff2cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4acb9b); f_00482756(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x4acba7; continue;
 }
 0x4acba7 => {
  pc = 0x4acbac; continue;
 }
 0x4acbac => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acbb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acbb3); }
 let mut pc: u32 = 0x4acbb3;
 loop { match pc {
 0x4acbb3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4acddfu32; cpu.push(v); }
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
  if cpu.zf { pc = 0x4acc0c; continue; }
  pc = 0x4acbe5; continue;
 }
 0x4acbe5 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3b78u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x4acc01; continue;
 }
 0x4acc01 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4acc0c; continue;
 }
 0x4acc0c => {
  pc = 0x4acc35; continue;
 }
 0x4acc35 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4acc3f); f_0049b35e(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4acc52); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4acc65; continue;
 }
 0x4acc65 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4acc84; continue;
 }
 0x4acc84 => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4acc94); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38c4u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x4acca7; continue;
 }
 0x4acca7 => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4accc6; continue;
 }
 0x4accc6 => {
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
  pc = 0x4accff; continue;
 }
 0x4accff => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4acd45; continue;
 }
 0x4acd45 => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3b9cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3b84u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  { let v = 0x4f83e0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4acda9); cpu.call(t);
  pc = 0x4acdae; continue;
 }
 0x4acdae => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4acde9; continue;
 }
 0x4acde9 => {
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
pub fn f_004acc77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acc77); }
 let mut pc: u32 = 0x4acc77;
 loop { match pc {
 0x4acc77 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4acc83); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004accb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4accb9); }
 let mut pc: u32 = 0x4accb9;
 loop { match pc {
 0x4accb9 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4accc5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acd0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acd0e); }
 let mut pc: u32 = 0x4acd0e;
 loop { match pc {
 0x4acd0e => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x4acd19); f_004a44b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acd1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acd1a); }
 let mut pc: u32 = 0x4acd1a;
 loop { match pc {
 0x4acd1a => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x4acd25); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acd26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acd26); }
 let mut pc: u32 = 0x4acd26;
 loop { match pc {
 0x4acd26 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4acd2e); f_004ad9f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acd2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acd2f); }
 let mut pc: u32 = 0x4acd2f;
 loop { match pc {
 0x4acd2f => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4acd44; continue; }
  pc = 0x4acd39; continue;
 }
 0x4acd39 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x78u32); cpu.ecx = v; }
  cpu.push(0x4acd44); f_00485370(cpu);
  pc = 0x4acd44; continue;
 }
 0x4acd44 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acdbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acdbd); }
 let mut pc: u32 = 0x4acdbd;
 loop { match pc {
 0x4acdbd => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4acdc5); f_004ad9c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acdc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acdc6); }
 let mut pc: u32 = 0x4acdc6;
 loop { match pc {
 0x4acdc6 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4acdde; continue; }
  pc = 0x4acdd0; continue;
 }
 0x4acdd0 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x80u32); cpu.ecx = v; }
  cpu.push(0x4acdde); f_00485370(cpu);
  pc = 0x4acdde; continue;
 }
 0x4acdde => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acddf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acddf); }
 let mut pc: u32 = 0x4acddf;
 loop { match pc {
 0x4acddf => {
  { let v: u32 = 0x4be098u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acdfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acdfa); }
 let mut pc: u32 = 0x4acdfa;
 loop { match pc {
 0x4acdfa => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4aceb5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3b9cu32; cpu.w32(0xffffff80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3b84u32; cpu.w32(0xffffff84u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
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
  if cpu.zf { pc = 0x4ace93; continue; }
  pc = 0x4ace69; continue;
 }
 0x4ace69 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ace93; continue; }
  pc = 0x4ace82; continue;
 }
 0x4ace82 => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4ace8e); cpu.call(t);
  pc = 0x4ace93; continue;
 }
 0x4ace93 => {
  pc = 0x4ace98; continue;
 }
 0x4ace98 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4acea4); f_004acea9(cpu);
  pc = 0x4acebf; continue;
 }
 0x4acebf => {
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
pub fn f_004acea9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acea9); }
 let mut pc: u32 = 0x4acea9;
 loop { match pc {
 0x4acea9 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4aceb4); f_004a25b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004aceb5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4aceb5); }
 let mut pc: u32 = 0x4aceb5;
 loop { match pc {
 0x4aceb5 => {
  { let v: u32 = 0x4be0f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004acecd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4acecd); }
 let mut pc: u32 = 0x4acecd;
 loop { match pc {
 0x4acecd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ad0a6u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4acef7); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4ad08f; continue; }
  pc = 0x4acf0a; continue;
 }
 0x4acf0a => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4acf29); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4acf74; continue; }
  pc = 0x4acf40; continue;
 }
 0x4acf40 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4ae200u32; cpu.push(v); }
  { let v = 0x4ad675u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x38u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4acf66); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4acf7b; continue;
 }
 0x4acf74 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4acf7b; continue;
 }
 0x4acf7b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x78u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4acf9f; continue; }
  pc = 0x4acf98; continue;
 }
 0x4acf98 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ad0b0; continue;
 }
 0x4acf9f => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4acfa8; continue;
 }
 0x4acfa8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4acfb1); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ad08f; continue; }
  pc = 0x4acfb9; continue;
 }
 0x4acfb9 => {
  pc = 0x4acfbe; continue;
 }
 0x4acfbe => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4acfcf; continue;
 }
 0x4acfcf => {
  pc = 0x4acfd4; continue;
 }
 0x4acfd4 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x18u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ad04b; continue;
 }
 0x4ad04b => {
  pc = 0x4ad050; continue;
 }
 0x4ad050 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ad07d; continue; }
  pc = 0x4ad06c; continue;
 }
 0x4ad06c => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4ad078); cpu.call(t);
  pc = 0x4ad07d; continue;
 }
 0x4ad07d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x4ad086); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0x38u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4acfa8; continue;
 }
 0x4ad08f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ad0b0; continue;
 }
 0x4ad0b0 => {
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
pub fn f_004ad099(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad099); }
 let mut pc: u32 = 0x4ad099;
 loop { match pc {
 0x4ad099 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad0a2); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad0a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad0a6); }
 let mut pc: u32 = 0x4ad0a6;
 loop { match pc {
 0x4ad0a6 => {
  { let v: u32 = 0x4be120u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad0c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad0c1); }
 let mut pc: u32 = 0x4ad0c1;
 loop { match pc {
 0x4ad0c1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4ad0d8; continue;
 }
 0x4ad0d8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad0dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad0dd); }
 let mut pc: u32 = 0x4ad0dd;
 loop { match pc {
 0x4ad0dd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4ad0f4; continue;
 }
 0x4ad0f4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad45d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad45d); }
 let mut pc: u32 = 0x4ad45d;
 loop { match pc {
 0x4ad45d => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad466); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad46a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad46a); }
 let mut pc: u32 = 0x4ad46a;
 loop { match pc {
 0x4ad46a => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ad473); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad474); }
 let mut pc: u32 = 0x4ad474;
 loop { match pc {
 0x4ad474 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ad47d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad47e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad47e); }
 let mut pc: u32 = 0x4ad47e;
 loop { match pc {
 0x4ad47e => {
  { let v: u32 = 0x4be148u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad53e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad53e); }
 let mut pc: u32 = 0x4ad53e;
 loop { match pc {
 0x4ad53e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad56b; continue;
 }
 0x4ad56b => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad58c; continue;
 }
 0x4ad58c => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad5a4; continue;
 }
 0x4ad5a1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad5a4; continue;
 }
 0x4ad5a4 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4ad66b; continue; }
  pc = 0x4ad5b3; continue;
 }
 0x4ad5b3 => {
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ad5dc; continue; }
  pc = 0x4ad5bd; continue;
 }
 0x4ad5bd => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x30u32); cpu.eax = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ad5dc; continue;
 }
 0x4ad5dc => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x4ad60d; continue; }
  pc = 0x4ad5ec; continue;
 }
 0x4ad5ec => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ad62c; continue;
 }
 0x4ad60d => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ad62c; continue;
 }
 0x4ad62c => {
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4ad652; continue; }
  pc = 0x4ad636; continue;
 }
 0x4ad636 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ad652; continue;
 }
 0x4ad652 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ad662); f_004ad7ab(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)).wrapping_add(0x38u32); cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad5a1; continue;
 }
 0x4ad66b => {
  pc = 0x4ad670; continue;
 }
 0x4ad670 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad675(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad675); }
 let mut pc: u32 = 0x4ad675;
 loop { match pc {
 0x4ad675 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ad792u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ad6a4; continue;
 }
 0x4ad6a4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b39dcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ad6b9; continue;
 }
 0x4ad6b9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad6d4; continue;
 }
 0x4ad6d4 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ad708; continue;
 }
 0x4ad708 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ad735; continue;
 }
 0x4ad735 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ad762; continue;
 }
 0x4ad762 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3bbcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ad77a; continue;
 }
 0x4ad77a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ad79c; continue;
 }
 0x4ad79c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad6cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad6cb); }
 let mut pc: u32 = 0x4ad6cb;
 loop { match pc {
 0x4ad6cb => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ad6d3); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad789(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad789); }
 let mut pc: u32 = 0x4ad789;
 loop { match pc {
 0x4ad789 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ad791); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad792(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad792); }
 let mut pc: u32 = 0x4ad792;
 loop { match pc {
 0x4ad792 => {
  { let v: u32 = 0x4be180u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad7ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad7ab); }
 let mut pc: u32 = 0x4ad7ab;
 loop { match pc {
 0x4ad7ab => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad7cf); f_004af0d1(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x28u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x18u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ad81b; continue;
 }
 0x4ad81b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad822(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad822); }
 let mut pc: u32 = 0x4ad822;
 loop { match pc {
 0x4ad822 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4ad839; continue;
 }
 0x4ad839 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad83e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad83e); }
 let mut pc: u32 = 0x4ad83e;
 loop { match pc {
 0x4ad83e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad853); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad8da; continue;
 }
 0x4ad86a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad873); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad885); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad897); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad8a9); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x10u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad8bb); f_004af810(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4ad8e9; continue;
 }
 0x4ad8c9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ad8f3; continue;
 }
 0x4ad8da => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ad86a; continue; }
  pc = 0x4ad8e4; continue;
 }
 0x4ad8e4 => {
  pc = 0x4ad8c9; continue;
 }
 0x4ad8e9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ad8f3; continue;
 }
 0x4ad8f3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad8fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad8fa); }
 let mut pc: u32 = 0x4ad8fa;
 loop { match pc {
 0x4ad8fa => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad911); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad924); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad937); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad94a); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad95d); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad970); f_004af8e3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ad97d; continue;
 }
 0x4ad97d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad990); }
 let mut pc: u32 = 0x4ad990;
 loop { match pc {
 0x4ad990 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad9a1; continue;
 }
 0x4ad9a1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x48u32); cpu.ecx = v; }
  cpu.push(0x4ad9ac); f_004a0127(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ad9bb; continue;
 }
 0x4ad9bb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad9c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad9c0); }
 let mut pc: u32 = 0x4ad9c0;
 loop { match pc {
 0x4ad9c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad9d1; continue;
 }
 0x4ad9d1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x78u32); cpu.ecx = v; }
  cpu.push(0x4ad9dc); f_004a25b0(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x78u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ad9eb; continue;
 }
 0x4ad9eb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ad9f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ad9f0); }
 let mut pc: u32 = 0x4ad9f0;
 loop { match pc {
 0x4ad9f0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ada9bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ada16; continue;
 }
 0x4ada16 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ada2e; continue;
 }
 0x4ada2e => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ada37); f_004ada5a(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ada40); f_004ada66(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ada49); f_004ada73(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ada55); f_004ada80(cpu);
  pc = 0x4ada8c; continue;
 }
 0x4ada8c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  pc = 0x4adaa5; continue;
 }
 0x4adaa5 => {
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
pub fn f_004ada5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ada5a); }
 let mut pc: u32 = 0x4ada5a;
 loop { match pc {
 0x4ada5a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x44u32); cpu.ecx = v; }
  cpu.push(0x4ada65); f_004a4030(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ada66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ada66); }
 let mut pc: u32 = 0x4ada66;
 loop { match pc {
 0x4ada66 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x3cu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ada72); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ada73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ada73); }
 let mut pc: u32 = 0x4ada73;
 loop { match pc {
 0x4ada73 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ada7f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ada80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ada80); }
 let mut pc: u32 = 0x4ada80;
 loop { match pc {
 0x4ada80 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  cpu.push(0x4ada8b); f_004a39b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ada9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ada9b); }
 let mut pc: u32 = 0x4ada9b;
 loop { match pc {
 0x4ada9b => {
  { let v: u32 = 0x4be1b0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adac0); }
 let mut pc: u32 = 0x4adac0;
 loop { match pc {
 0x4adac0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4adb5au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4adb16; continue; }
  pc = 0x4adaeb; continue;
 }
 0x4adaeb => {
  { let v = 0x4adb80u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4adb02); f_004aeb10(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4adb0e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4adb52; continue;
 }
 0x4adb16 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4adb22; continue;
 }
 0x4adb22 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4adb2e); f_004adb33(cpu);
  pc = 0x4adb3c; continue;
 }
 0x4adb3c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4adb52; continue; }
  pc = 0x4adb46; continue;
 }
 0x4adb46 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4adb4f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4adb52; continue;
 }
 0x4adb52 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4adb64; continue;
 }
 0x4adb64 => {
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
pub fn f_004adb33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adb33); }
 let mut pc: u32 = 0x4adb33;
 loop { match pc {
 0x4adb33 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4adb3b); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adb5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adb5a); }
 let mut pc: u32 = 0x4adb5a;
 loop { match pc {
 0x4adb5a => {
  { let v: u32 = 0x4be1f0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adb80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adb80); }
 let mut pc: u32 = 0x4adb80;
 loop { match pc {
 0x4adb80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4adbc7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4adbad; continue;
 }
 0x4adbad => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4adbb9); f_004adbbe(cpu);
  pc = 0x4adbd1; continue;
 }
 0x4adbd1 => {
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
pub fn f_004adbbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adbbe); }
 let mut pc: u32 = 0x4adbbe;
 loop { match pc {
 0x4adbbe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4adbc6); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adbc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adbc7); }
 let mut pc: u32 = 0x4adbc7;
 loop { match pc {
 0x4adbc7 => {
  { let v: u32 = 0x4be218u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adbe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adbe0); }
 let mut pc: u32 = 0x4adbe0;
 loop { match pc {
 0x4adbe0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4adc27u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4adc0d; continue;
 }
 0x4adc0d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4adc19); f_004adc1e(cpu);
  pc = 0x4adc31; continue;
 }
 0x4adc31 => {
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
pub fn f_004adc1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adc1e); }
 let mut pc: u32 = 0x4adc1e;
 loop { match pc {
 0x4adc1e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4adc26); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adc27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adc27); }
 let mut pc: u32 = 0x4adc27;
 loop { match pc {
 0x4adc27 => {
  { let v: u32 = 0x4be240u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adc40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adc40); }
 let mut pc: u32 = 0x4adc40;
 loop { match pc {
 0x4adc40 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4adca5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4adc6d; continue;
 }
 0x4adc6d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4adc79); f_004adc7e(cpu);
  pc = 0x4adc87; continue;
 }
 0x4adc87 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4adc9d; continue; }
  pc = 0x4adc91; continue;
 }
 0x4adc91 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4adc9a); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4adc9d; continue;
 }
 0x4adc9d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4adcaf; continue;
 }
 0x4adcaf => {
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
pub fn f_004adc7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adc7e); }
 let mut pc: u32 = 0x4adc7e;
 loop { match pc {
 0x4adc7e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4adc86); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adca5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adca5); }
 let mut pc: u32 = 0x4adca5;
 loop { match pc {
 0x4adca5 => {
  { let v: u32 = 0x4be268u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adcc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adcc0); }
 let mut pc: u32 = 0x4adcc0;
 loop { match pc {
 0x4adcc0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4add5au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4add16; continue; }
  pc = 0x4adceb; continue;
 }
 0x4adceb => {
  { let v = 0x4add80u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4add02); f_004aeb10(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4add0e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4add52; continue;
 }
 0x4add16 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4add22; continue;
 }
 0x4add22 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4add2e); f_004add33(cpu);
  pc = 0x4add3c; continue;
 }
 0x4add3c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4add52; continue; }
  pc = 0x4add46; continue;
 }
 0x4add46 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4add4f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4add52; continue;
 }
 0x4add52 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4add64; continue;
 }
 0x4add64 => {
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
pub fn f_004add33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4add33); }
 let mut pc: u32 = 0x4add33;
 loop { match pc {
 0x4add33 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4add3b); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004add5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4add5a); }
 let mut pc: u32 = 0x4add5a;
 loop { match pc {
 0x4add5a => {
  { let v: u32 = 0x4be290u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004add80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4add80); }
 let mut pc: u32 = 0x4add80;
 loop { match pc {
 0x4add80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4addc7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4addad; continue;
 }
 0x4addad => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4addb9); f_004addbe(cpu);
  pc = 0x4addd1; continue;
 }
 0x4addd1 => {
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
pub fn f_004addbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4addbe); }
 let mut pc: u32 = 0x4addbe;
 loop { match pc {
 0x4addbe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4addc6); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004addc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4addc7); }
 let mut pc: u32 = 0x4addc7;
 loop { match pc {
 0x4addc7 => {
  { let v: u32 = 0x4be2b8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adde0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adde0); }
 let mut pc: u32 = 0x4adde0;
 loop { match pc {
 0x4adde0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ade7au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4ade36; continue; }
  pc = 0x4ade0b; continue;
 }
 0x4ade0b => {
  { let v = 0x4adea0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x18u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ade22); f_004aeb10(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ade2e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ade72; continue;
 }
 0x4ade36 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ade42; continue;
 }
 0x4ade42 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ade4e); f_004ade53(cpu);
  pc = 0x4ade5c; continue;
 }
 0x4ade5c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ade72; continue; }
  pc = 0x4ade66; continue;
 }
 0x4ade66 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ade6f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ade72; continue;
 }
 0x4ade72 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ade84; continue;
 }
 0x4ade84 => {
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
pub fn f_004ade53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ade53); }
 let mut pc: u32 = 0x4ade53;
 loop { match pc {
 0x4ade53 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ade5b); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ade7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ade7a); }
 let mut pc: u32 = 0x4ade7a;
 loop { match pc {
 0x4ade7a => {
  { let v: u32 = 0x4be2e0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adea0); }
 let mut pc: u32 = 0x4adea0;
 loop { match pc {
 0x4adea0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4adee7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4adecd; continue;
 }
 0x4adecd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aded9); f_004adede(cpu);
  pc = 0x4adef1; continue;
 }
 0x4adef1 => {
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
pub fn f_004adede(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adede); }
 let mut pc: u32 = 0x4adede;
 loop { match pc {
 0x4adede => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4adee6); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adee7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adee7); }
 let mut pc: u32 = 0x4adee7;
 loop { match pc {
 0x4adee7 => {
  { let v: u32 = 0x4be308u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adf00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adf00); }
 let mut pc: u32 = 0x4adf00;
 loop { match pc {
 0x4adf00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4adf9au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4adf56; continue; }
  pc = 0x4adf2b; continue;
 }
 0x4adf2b => {
  { let v = 0x4adfc0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4adf42); f_004aeb10(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4adf4e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4adf92; continue;
 }
 0x4adf56 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4adf62; continue;
 }
 0x4adf62 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4adf6e); f_004adf73(cpu);
  pc = 0x4adf7c; continue;
 }
 0x4adf7c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4adf92; continue; }
  pc = 0x4adf86; continue;
 }
 0x4adf86 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4adf8f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4adf92; continue;
 }
 0x4adf92 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4adfa4; continue;
 }
 0x4adfa4 => {
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
pub fn f_004adf73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adf73); }
 let mut pc: u32 = 0x4adf73;
 loop { match pc {
 0x4adf73 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4adf7b); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adf9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adf9a); }
 let mut pc: u32 = 0x4adf9a;
 loop { match pc {
 0x4adf9a => {
  { let v: u32 = 0x4be330u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004adfc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adfc0); }
 let mut pc: u32 = 0x4adfc0;
 loop { match pc {
 0x4adfc0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ae007u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4adfed; continue;
 }
 0x4adfed => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4adff9); f_004adffe(cpu);
  pc = 0x4ae011; continue;
 }
 0x4ae011 => {
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
pub fn f_004adffe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4adffe); }
 let mut pc: u32 = 0x4adffe;
 loop { match pc {
 0x4adffe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae006); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae007(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae007); }
 let mut pc: u32 = 0x4ae007;
 loop { match pc {
 0x4ae007 => {
  { let v: u32 = 0x4be358u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae020); }
 let mut pc: u32 = 0x4ae020;
 loop { match pc {
 0x4ae020 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ae0bau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4ae076; continue; }
  pc = 0x4ae04b; continue;
 }
 0x4ae04b => {
  { let v = 0x4ae0e0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae062); f_004aeb10(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae06e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae0b2; continue;
 }
 0x4ae076 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae082; continue;
 }
 0x4ae082 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae08e); f_004ae093(cpu);
  pc = 0x4ae09c; continue;
 }
 0x4ae09c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae0b2; continue; }
  pc = 0x4ae0a6; continue;
 }
 0x4ae0a6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae0af); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae0b2; continue;
 }
 0x4ae0b2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ae0c4; continue;
 }
 0x4ae0c4 => {
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
pub fn f_004ae024(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae024); }
 let mut pc: u32 = 0x4ae024;
 loop { match pc {
 0x4ae024 => {
  pc = cpu.r32(0xffffffbau32.wrapping_add(cpu.eax)); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae02d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae02d); }
 let mut pc: u32 = 0x4ae02d;
 loop { match pc {
 0x4ae02d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x64u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4ae076; continue; }
  pc = 0x4ae04b; continue;
 }
 0x4ae04b => {
  { let v = 0x4ae0e0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae062); f_004aeb10(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae06e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae0b2; continue;
 }
 0x4ae076 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae082; continue;
 }
 0x4ae082 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae08e); f_004ae093(cpu);
  pc = 0x4ae09c; continue;
 }
 0x4ae09c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae0b2; continue; }
  pc = 0x4ae0a6; continue;
 }
 0x4ae0a6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae0af); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae0b2; continue;
 }
 0x4ae0b2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ae0c4; continue;
 }
 0x4ae0c4 => {
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
pub fn f_004ae05f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae05f); }
 let mut pc: u32 = 0x4ae05f;
 loop { match pc {
 0x4ae05f => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xe883f045u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xe883f045u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x50u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x4ae06e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae0b2; continue;
 }
 0x4ae0b2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ae0c4; continue;
 }
 0x4ae0c4 => {
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
pub fn f_004ae07b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae07b); }
 let mut pc: u32 = 0x4ae07b;
 loop { match pc {
 0x4ae07b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae082; continue;
 }
 0x4ae082 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae08e); f_004ae093(cpu);
  pc = 0x4ae09c; continue;
 }
 0x4ae09c => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae0b2; continue; }
  pc = 0x4ae0a6; continue;
 }
 0x4ae0a6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae0af); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae0b2; continue;
 }
 0x4ae0b2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ae0c4; continue;
 }
 0x4ae0c4 => {
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
pub fn f_004ae093(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae093); }
 let mut pc: u32 = 0x4ae093;
 loop { match pc {
 0x4ae093 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae09b); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae0ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae0ba); }
 let mut pc: u32 = 0x4ae0ba;
 loop { match pc {
 0x4ae0ba => {
  { let v: u32 = 0x4be380u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae0e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae0e0); }
 let mut pc: u32 = 0x4ae0e0;
 loop { match pc {
 0x4ae0e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ae127u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae10d; continue;
 }
 0x4ae10d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae119); f_004ae11e(cpu);
  pc = 0x4ae131; continue;
 }
 0x4ae131 => {
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
pub fn f_004ae11e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae11e); }
 let mut pc: u32 = 0x4ae11e;
 loop { match pc {
 0x4ae11e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae126); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae127(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae127); }
 let mut pc: u32 = 0x4ae127;
 loop { match pc {
 0x4ae127 => {
  { let v: u32 = 0x4be3a8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae140); }
 let mut pc: u32 = 0x4ae140;
 loop { match pc {
 0x4ae140 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ae1dau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4ae196; continue; }
  pc = 0x4ae16b; continue;
 }
 0x4ae16b => {
  { let v = 0x4ae200u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x38u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae182); f_004aeb10(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae18e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae1d2; continue;
 }
 0x4ae196 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae1a2; continue;
 }
 0x4ae1a2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae1ae); f_004ae1b3(cpu);
  pc = 0x4ae1bc; continue;
 }
 0x4ae1bc => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae1d2; continue; }
  pc = 0x4ae1c6; continue;
 }
 0x4ae1c6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae1cf); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae1d2; continue;
 }
 0x4ae1d2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ae1e4; continue;
 }
 0x4ae1e4 => {
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
pub fn f_004ae1b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae1b3); }
 let mut pc: u32 = 0x4ae1b3;
 loop { match pc {
 0x4ae1b3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae1bb); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae1da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae1da); }
 let mut pc: u32 = 0x4ae1da;
 loop { match pc {
 0x4ae1da => {
  { let v: u32 = 0x4be3d0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae200); }
 let mut pc: u32 = 0x4ae200;
 loop { match pc {
 0x4ae200 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ae247u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae22d; continue;
 }
 0x4ae22d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae239); f_004ae23e(cpu);
  pc = 0x4ae251; continue;
 }
 0x4ae251 => {
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
pub fn f_004ae23e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae23e); }
 let mut pc: u32 = 0x4ae23e;
 loop { match pc {
 0x4ae23e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae246); f_004adbe0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae247(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae247); }
 let mut pc: u32 = 0x4ae247;
 loop { match pc {
 0x4ae247 => {
  { let v: u32 = 0x4be3f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae260); }
 let mut pc: u32 = 0x4ae260;
 loop { match pc {
 0x4ae260 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ae270; continue;
 }
 0x4ae270 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ae30bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae296; continue;
 }
 0x4ae296 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae2a6; continue;
 }
 0x4ae2a6 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae2af); f_004ae2c0(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae2bb); f_004ae2cd(cpu);
  pc = 0x4ae2d9; continue;
 }
 0x4ae2d9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae2e7; continue;
 }
 0x4ae2e7 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae300; continue; }
  pc = 0x4ae2f1; continue;
 }
 0x4ae2f1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x3cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae2fd); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae300; continue;
 }
 0x4ae300 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x3cu32); cpu.eax = v; }
  pc = 0x4ae315; continue;
 }
 0x4ae315 => {
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
pub fn f_004ae2c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae2c0); }
 let mut pc: u32 = 0x4ae2c0;
 loop { match pc {
 0x4ae2c0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4ae2cc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae2cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae2cd); }
 let mut pc: u32 = 0x4ae2cd;
 loop { match pc {
 0x4ae2cd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4ae2d8); f_004a39b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae30b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae30b); }
 let mut pc: u32 = 0x4ae30b;
 loop { match pc {
 0x4ae30b => {
  { let v: u32 = 0x4be420u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae330); }
 let mut pc: u32 = 0x4ae330;
 loop { match pc {
 0x4a70fd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a71c5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a7127); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a71a2; continue;
 }
 0x4a713e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a714b); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4a7162); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a716e); f_004a71bb(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a717d); f_0049a9c4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a718c; continue; }
  pc = 0x4a7185; continue;
 }
 0x4a7185 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a71cf; continue;
 }
 0x4a718c => {
  pc = 0x4a71b1; continue;
 }
 0x4a7191 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a71cf; continue;
 }
 0x4a71a2 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a713e; continue; }
  pc = 0x4a71ac; continue;
 }
 0x4a71ac => {
  pc = 0x4a7191; continue;
 }
 0x4a71b1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a71cf; continue;
 }
 0x4a71cf => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae330 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a70fd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae340); }
 let mut pc: u32 = 0x4ae340;
 loop { match pc {
 0x4a71e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a71f7); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a720a); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a721c); f_0049ac2e(cpu);
  pc = 0x4a7221; continue;
 }
 0x4a7221 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae340 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a71e0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae350); }
 let mut pc: u32 = 0x4ae350;
 loop { match pc {
 0x4ae350 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ae360; continue;
 }
 0x4ae360 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ae3deu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae386; continue;
 }
 0x4ae386 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae392; continue;
 }
 0x4ae392 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ae39e); f_004ae3a3(cpu);
  pc = 0x4ae3ac; continue;
 }
 0x4ae3ac => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae3ba; continue;
 }
 0x4ae3ba => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae3d3; continue; }
  pc = 0x4ae3c4; continue;
 }
 0x4ae3c4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x48u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae3d0); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae3d3; continue;
 }
 0x4ae3d3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x48u32); cpu.eax = v; }
  pc = 0x4ae3e8; continue;
 }
 0x4ae3e8 => {
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
pub fn f_004ae3a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae3a3); }
 let mut pc: u32 = 0x4ae3a3;
 loop { match pc {
 0x4ae3a3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae3ab); f_004a0127(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae3de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae3de); }
 let mut pc: u32 = 0x4ae3de;
 loop { match pc {
 0x4ae3de => {
  { let v: u32 = 0x4be450u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae400); }
 let mut pc: u32 = 0x4ae400;
 loop { match pc {
 0x4a77bd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a77d2); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a781a; continue;
 }
 0x4a77e9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a77f5); f_004a021f(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a7804; continue; }
  pc = 0x4a77fd; continue;
 }
 0x4a77fd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a7833; continue;
 }
 0x4a7804 => {
  pc = 0x4a7829; continue;
 }
 0x4a7809 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a7833; continue;
 }
 0x4a781a => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a77e9; continue; }
  pc = 0x4a7824; continue;
 }
 0x4a7824 => {
  pc = 0x4a7809; continue;
 }
 0x4a7829 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a7833; continue;
 }
 0x4a7833 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae400 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a77bd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae410); }
 let mut pc: u32 = 0x4ae410;
 loop { match pc {
 0x4a783a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a7851); f_004af8d2(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a7860); f_004a0482(cpu);
  pc = 0x4a7865; continue;
 }
 0x4a7865 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae410 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a783a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae420); }
 let mut pc: u32 = 0x4ae420;
 loop { match pc {
 0x4ae420 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ae430; continue;
 }
 0x4ae430 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae441; continue;
 }
 0x4ae441 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae449); f_004a8051(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae457; continue;
 }
 0x4ae457 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae472; continue; }
  pc = 0x4ae461; continue;
 }
 0x4ae461 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae46f); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae472; continue;
 }
 0x4ae472 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x80u32); cpu.eax = v; }
  pc = 0x4ae47f; continue;
 }
 0x4ae47f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae490); }
 let mut pc: u32 = 0x4ae490;
 loop { match pc {
 0x4a82f2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a8607u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a831c); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a85c3; continue;
 }
 0x4a8333 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a8342); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a8351; continue; }
  pc = 0x4a834a; continue;
 }
 0x4a834a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a8611; continue;
 }
 0x4a8351 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x80u32); cpu.ecx = v; }
  cpu.push(0x4a8366); f_004a8124(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a8375; continue; }
  pc = 0x4a836e; continue;
 }
 0x4a836e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a8611; continue;
 }
 0x4a8375 => {
  pc = 0x4a85dc; continue;
 }
 0x4a837a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a8383); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a858f; continue; }
  pc = 0x4a8399; continue;
 }
 0x4a8399 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a83a6); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f8330u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a83c0); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4a83d6); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a83de; continue;
 }
 0x4a83de => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a83e7); f_004a85f3(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a8412; continue; }
  pc = 0x4a83f7; continue;
 }
 0x4a83f7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a840a); f_004a85fd(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a8611; continue;
 }
 0x4a8412 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a842d); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a8475; continue; }
  pc = 0x4a8441; continue;
 }
 0x4a8441 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4adb80u32; cpu.push(v); }
  { let v = 0x4a86c7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a8467); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a847c; continue;
 }
 0x4a8475 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a847c; continue;
 }
 0x4a847c => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a84b1; continue; }
  pc = 0x4a8496; continue;
 }
 0x4a8496 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a84a9); f_004a85fd(cpu);
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a8611; continue;
 }
 0x4a84b1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a84c0; continue;
 }
 0x4a84bd => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a84c0; continue;
 }
 0x4a84c0 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4a8583; continue; }
  pc = 0x4a84cf; continue;
 }
 0x4a84cf => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6dc8u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a84ea); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a84f3); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a851e; continue; }
  pc = 0x4a8503; continue;
 }
 0x4a8503 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a8516); f_004a85fd(cpu);
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a8611; continue;
 }
 0x4a851e => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8535; continue;
 }
 0x4a8535 => {
  pc = 0x4a853a; continue;
 }
 0x4a853a => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a8557; continue;
 }
 0x4a8557 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a857e; continue; }
  pc = 0x4a856d; continue;
 }
 0x4a856d => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a8579); cpu.call(t);
  pc = 0x4a857e; continue;
 }
 0x4a857e => {
  pc = 0x4a84bd; continue;
 }
 0x4a8583 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a858f); f_004a85fd(cpu);
  pc = 0x4a858f; continue;
 }
 0x4a858f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a859e); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a85ad; continue; }
  pc = 0x4a85a6; continue;
 }
 0x4a85a6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a8611; continue;
 }
 0x4a85ad => {
  pc = 0x4a85dc; continue;
 }
 0x4a85b2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a8611; continue;
 }
 0x4a85c3 => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a8333; continue; }
  pc = 0x4a85cd; continue;
 }
 0x4a85cd => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x4a837a; continue; }
  pc = 0x4a85d7; continue;
 }
 0x4a85d7 => {
  pc = 0x4a85b2; continue;
 }
 0x4a85dc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a8611; continue;
 }
 0x4a8611 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae490 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a82f2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae4a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae4a0); }
 let mut pc: u32 = 0x4ae4a0;
 loop { match pc {
 0x4a8622 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a8639); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a864c); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8667; continue;
 }
 0x4a8664 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8667; continue;
 }
 0x4a8667 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4a86ac; continue; }
  pc = 0x4a8676; continue;
 }
 0x4a8676 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x4a8694; continue; }
  pc = 0x4a8680; continue;
 }
 0x4a8680 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a868f); cpu.call(t);
  pc = 0x4a86a3; continue;
 }
 0x4a8694 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x4a86a3); cpu.call(t);
  pc = 0x4a86a3; continue;
 }
 0x4a86a3 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0xcu32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a8664; continue;
 }
 0x4a86ac => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a86bb); f_004a2f17(cpu);
  pc = 0x4a86c0; continue;
 }
 0x4a86c0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae4a0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a8622; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae4b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae4b0); }
 let mut pc: u32 = 0x4ae4b0;
 loop { match pc {
 0x4ae4b0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ae4c0; continue;
 }
 0x4ae4c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae4d1; continue;
 }
 0x4ae4d1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae4d9); f_004a8efd(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae4e7; continue;
 }
 0x4ae4e7 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae502; continue; }
  pc = 0x4ae4f1; continue;
 }
 0x4ae4f1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae4ff); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae502; continue;
 }
 0x4ae502 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x80u32); cpu.eax = v; }
  pc = 0x4ae50f; continue;
 }
 0x4ae50f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae520); }
 let mut pc: u32 = 0x4ae520;
 loop { match pc {
 0x4a91ae => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a94d5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a91d8); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9491; continue;
 }
 0x4a91ef => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a91fe); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a920d; continue; }
  pc = 0x4a9206; continue;
 }
 0x4a9206 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a94df; continue;
 }
 0x4a920d => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x80u32); cpu.ecx = v; }
  cpu.push(0x4a9222); f_004a8fd0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a9231; continue; }
  pc = 0x4a922a; continue;
 }
 0x4a922a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a94df; continue;
 }
 0x4a9231 => {
  pc = 0x4a94aa; continue;
 }
 0x4a9236 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a923f); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a945d; continue; }
  pc = 0x4a9255; continue;
 }
 0x4a9255 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9262); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f8358u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a927c); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4a9292); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a929a; continue;
 }
 0x4a929a => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a92a3); f_004a94c1(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a92ce; continue; }
  pc = 0x4a92b3; continue;
 }
 0x4a92b3 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a92c6); f_004a94cb(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a94df; continue;
 }
 0x4a92ce => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a92e9); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a9331; continue; }
  pc = 0x4a92fd; continue;
 }
 0x4a92fd => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4add80u32; cpu.push(v); }
  { let v = 0x4a9595u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9323); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9338; continue;
 }
 0x4a9331 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9338; continue;
 }
 0x4a9338 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a936d; continue; }
  pc = 0x4a9352; continue;
 }
 0x4a9352 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a9365); f_004a94cb(cpu);
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a94df; continue;
 }
 0x4a936d => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a937c; continue;
 }
 0x4a9379 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a937c; continue;
 }
 0x4a937c => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4a9451; continue; }
  pc = 0x4a938b; continue;
 }
 0x4a938b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6d80u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a93a6); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a93af); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a93da; continue; }
  pc = 0x4a93bf; continue;
 }
 0x4a93bf => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a93d2); f_004a94cb(cpu);
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a94df; continue;
 }
 0x4a93da => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a93f3; continue;
 }
 0x4a93f3 => {
  pc = 0x4a93f8; continue;
 }
 0x4a93f8 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a9425; continue;
 }
 0x4a9425 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a944c; continue; }
  pc = 0x4a943b; continue;
 }
 0x4a943b => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a9447); cpu.call(t);
  pc = 0x4a944c; continue;
 }
 0x4a944c => {
  pc = 0x4a9379; continue;
 }
 0x4a9451 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a945d); f_004a94cb(cpu);
  pc = 0x4a945d; continue;
 }
 0x4a945d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a946c); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a947b; continue; }
  pc = 0x4a9474; continue;
 }
 0x4a9474 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a94df; continue;
 }
 0x4a947b => {
  pc = 0x4a94aa; continue;
 }
 0x4a9480 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a94df; continue;
 }
 0x4a9491 => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a91ef; continue; }
  pc = 0x4a949b; continue;
 }
 0x4a949b => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x4a9236; continue; }
  pc = 0x4a94a5; continue;
 }
 0x4a94a5 => {
  pc = 0x4a9480; continue;
 }
 0x4a94aa => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a94df; continue;
 }
 0x4a94df => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae520 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a91ae; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae530); }
 let mut pc: u32 = 0x4ae530;
 loop { match pc {
 0x4a94f0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a9507); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a951a); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9535; continue;
 }
 0x4a9532 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9535; continue;
 }
 0x4a9535 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4a957a; continue; }
  pc = 0x4a9544; continue;
 }
 0x4a9544 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x4a9562; continue; }
  pc = 0x4a954e; continue;
 }
 0x4a954e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a955d); cpu.call(t);
  pc = 0x4a9571; continue;
 }
 0x4a9562 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x4a9571); cpu.call(t);
  pc = 0x4a9571; continue;
 }
 0x4a9571 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x14u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a9532; continue;
 }
 0x4a957a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a9589); f_004a2f17(cpu);
  pc = 0x4a958e; continue;
 }
 0x4a958e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae530 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4a94f0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae540); }
 let mut pc: u32 = 0x4ae540;
 loop { match pc {
 0x4ae540 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ae550; continue;
 }
 0x4ae550 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae561; continue;
 }
 0x4ae561 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae569); f_004a9e00(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae577; continue;
 }
 0x4ae577 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae592; continue; }
  pc = 0x4ae581; continue;
 }
 0x4ae581 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae58f); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae592; continue;
 }
 0x4ae592 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x80u32); cpu.eax = v; }
  pc = 0x4ae59f; continue;
 }
 0x4ae59f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae5b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae5b0); }
 let mut pc: u32 = 0x4ae5b0;
 loop { match pc {
 0x4aa0b7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4aa3e2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa0e1); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa39e; continue;
 }
 0x4aa0f8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4aa107); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4aa116; continue; }
  pc = 0x4aa10f; continue;
 }
 0x4aa10f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4aa3ec; continue;
 }
 0x4aa116 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x80u32); cpu.ecx = v; }
  cpu.push(0x4aa12b); f_004a9ed3(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4aa13a; continue; }
  pc = 0x4aa133; continue;
 }
 0x4aa133 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4aa3ec; continue;
 }
 0x4aa13a => {
  pc = 0x4aa3b7; continue;
 }
 0x4aa13f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa148); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4aa36a; continue; }
  pc = 0x4aa15e; continue;
 }
 0x4aa15e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa16b); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f8380u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4aa185); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4aa19b); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa1a3; continue;
 }
 0x4aa1a3 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aa1ac); f_004aa3ce(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aa1d7; continue; }
  pc = 0x4aa1bc; continue;
 }
 0x4aa1bc => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aa1cf); f_004aa3d8(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4aa3ec; continue;
 }
 0x4aa1d7 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa1f2); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aa23a; continue; }
  pc = 0x4aa206; continue;
 }
 0x4aa206 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4adea0u32; cpu.push(v); }
  { let v = 0x4aa4a2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x18u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa22c); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa241; continue;
 }
 0x4aa23a => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa241; continue;
 }
 0x4aa241 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4aa276; continue; }
  pc = 0x4aa25b; continue;
 }
 0x4aa25b => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aa26e); f_004aa3d8(cpu);
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4aa3ec; continue;
 }
 0x4aa276 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa285; continue;
 }
 0x4aa282 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa285; continue;
 }
 0x4aa285 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4aa35e; continue; }
  pc = 0x4aa294; continue;
 }
 0x4aa294 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6d30u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa2af); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa2b8); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4aa2e3; continue; }
  pc = 0x4aa2c8; continue;
 }
 0x4aa2c8 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aa2db); f_004aa3d8(cpu);
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4aa3ec; continue;
 }
 0x4aa2e3 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa2fa; continue;
 }
 0x4aa2fa => {
  pc = 0x4aa2ff; continue;
 }
 0x4aa2ff => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4aa332; continue;
 }
 0x4aa332 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aa359; continue; }
  pc = 0x4aa348; continue;
 }
 0x4aa348 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4aa354); cpu.call(t);
  pc = 0x4aa359; continue;
 }
 0x4aa359 => {
  pc = 0x4aa282; continue;
 }
 0x4aa35e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4aa36a); f_004aa3d8(cpu);
  pc = 0x4aa36a; continue;
 }
 0x4aa36a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4aa379); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4aa388; continue; }
  pc = 0x4aa381; continue;
 }
 0x4aa381 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4aa3ec; continue;
 }
 0x4aa388 => {
  pc = 0x4aa3b7; continue;
 }
 0x4aa38d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4aa3ec; continue;
 }
 0x4aa39e => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4aa0f8; continue; }
  pc = 0x4aa3a8; continue;
 }
 0x4aa3a8 => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x4aa13f; continue; }
  pc = 0x4aa3b2; continue;
 }
 0x4aa3b2 => {
  pc = 0x4aa38d; continue;
 }
 0x4aa3b7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4aa3ec; continue;
 }
 0x4aa3ec => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae5b0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4aa0b7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae5c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae5c0); }
 let mut pc: u32 = 0x4ae5c0;
 loop { match pc {
 0x4aa3fd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa414); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4aa427); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa442; continue;
 }
 0x4aa43f => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa442; continue;
 }
 0x4aa442 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4aa487; continue; }
  pc = 0x4aa451; continue;
 }
 0x4aa451 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x4aa46f; continue; }
  pc = 0x4aa45b; continue;
 }
 0x4aa45b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4aa46a); cpu.call(t);
  pc = 0x4aa47e; continue;
 }
 0x4aa46f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x4aa47e); cpu.call(t);
  pc = 0x4aa47e; continue;
 }
 0x4aa47e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x18u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4aa43f; continue;
 }
 0x4aa487 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4aa496); f_004a2f17(cpu);
  pc = 0x4aa49b; continue;
 }
 0x4aa49b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae5c0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4aa3fd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae5d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae5d0); }
 let mut pc: u32 = 0x4ae5d0;
 loop { match pc {
 0x4ae5d0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ae5e0; continue;
 }
 0x4ae5e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae5f1; continue;
 }
 0x4ae5f1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae5f9); f_004aad65(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae607; continue;
 }
 0x4ae607 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae622; continue; }
  pc = 0x4ae611; continue;
 }
 0x4ae611 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae61f); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae622; continue;
 }
 0x4ae622 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x80u32); cpu.eax = v; }
  pc = 0x4ae62f; continue;
 }
 0x4ae62f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae640); }
 let mut pc: u32 = 0x4ae640;
 loop { match pc {
 0x4ab01e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ab34du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab048); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab309; continue;
 }
 0x4ab05f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4ab06e); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ab07d; continue; }
  pc = 0x4ab076; continue;
 }
 0x4ab076 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ab357; continue;
 }
 0x4ab07d => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x80u32); cpu.ecx = v; }
  cpu.push(0x4ab092); f_004aae38(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ab0a1; continue; }
  pc = 0x4ab09a; continue;
 }
 0x4ab09a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ab357; continue;
 }
 0x4ab0a1 => {
  pc = 0x4ab322; continue;
 }
 0x4ab0a6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab0af); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4ab2d5; continue; }
  pc = 0x4ab0c5; continue;
 }
 0x4ab0c5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab0d2); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f83a8u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4ab0ec); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4ab102); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab10a; continue;
 }
 0x4ab10a => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ab113); f_004ab339(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ab13e; continue; }
  pc = 0x4ab123; continue;
 }
 0x4ab123 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ab136); f_004ab343(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ab357; continue;
 }
 0x4ab13e => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab159); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ab1a1; continue; }
  pc = 0x4ab16d; continue;
 }
 0x4ab16d => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4adfc0u32; cpu.push(v); }
  { let v = 0x4ab40du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab193); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab1a8; continue;
 }
 0x4ab1a1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab1a8; continue;
 }
 0x4ab1a8 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4ab1dd; continue; }
  pc = 0x4ab1c2; continue;
 }
 0x4ab1c2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ab1d5); f_004ab343(cpu);
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ab357; continue;
 }
 0x4ab1dd => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab1ec; continue;
 }
 0x4ab1e9 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab1ec; continue;
 }
 0x4ab1ec => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4ab2c9; continue; }
  pc = 0x4ab1fb; continue;
 }
 0x4ab1fb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f7358u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab216); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab21f); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4ab24a; continue; }
  pc = 0x4ab22f; continue;
 }
 0x4ab22f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ab242); f_004ab343(cpu);
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ab357; continue;
 }
 0x4ab24a => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab263; continue;
 }
 0x4ab263 => {
  pc = 0x4ab268; continue;
 }
 0x4ab268 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ab29d; continue;
 }
 0x4ab29d => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ab2c4; continue; }
  pc = 0x4ab2b3; continue;
 }
 0x4ab2b3 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4ab2bf); cpu.call(t);
  pc = 0x4ab2c4; continue;
 }
 0x4ab2c4 => {
  pc = 0x4ab1e9; continue;
 }
 0x4ab2c9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ab2d5); f_004ab343(cpu);
  pc = 0x4ab2d5; continue;
 }
 0x4ab2d5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4ab2e4); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ab2f3; continue; }
  pc = 0x4ab2ec; continue;
 }
 0x4ab2ec => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ab357; continue;
 }
 0x4ab2f3 => {
  pc = 0x4ab322; continue;
 }
 0x4ab2f8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ab357; continue;
 }
 0x4ab309 => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ab05f; continue; }
  pc = 0x4ab313; continue;
 }
 0x4ab313 => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x4ab0a6; continue; }
  pc = 0x4ab31d; continue;
 }
 0x4ab31d => {
  pc = 0x4ab2f8; continue;
 }
 0x4ab322 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ab357; continue;
 }
 0x4ab357 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae640 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ab01e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae650); }
 let mut pc: u32 = 0x4ae650;
 loop { match pc {
 0x4ab368 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab37f); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ab392); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab3ad; continue;
 }
 0x4ab3aa => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab3ad; continue;
 }
 0x4ab3ad => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4ab3f2; continue; }
  pc = 0x4ab3bc; continue;
 }
 0x4ab3bc => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x4ab3da; continue; }
  pc = 0x4ab3c6; continue;
 }
 0x4ab3c6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4ab3d5); cpu.call(t);
  pc = 0x4ab3e9; continue;
 }
 0x4ab3da => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x4ab3e9); cpu.call(t);
  pc = 0x4ab3e9; continue;
 }
 0x4ab3e9 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x14u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ab3aa; continue;
 }
 0x4ab3f2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4ab401); f_004a2f17(cpu);
  pc = 0x4ab406; continue;
 }
 0x4ab406 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae650 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ab368; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae660); }
 let mut pc: u32 = 0x4ae660;
 loop { match pc {
 0x4ae660 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ae670; continue;
 }
 0x4ae670 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae681; continue;
 }
 0x4ae681 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae689); f_004abd35(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae697; continue;
 }
 0x4ae697 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae6b2; continue; }
  pc = 0x4ae6a1; continue;
 }
 0x4ae6a1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae6af); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae6b2; continue;
 }
 0x4ae6b2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x80u32); cpu.eax = v; }
  pc = 0x4ae6bf; continue;
 }
 0x4ae6bf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae6d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae6d0); }
 let mut pc: u32 = 0x4ae6d0;
 loop { match pc {
 0x4ac022 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ac389u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac04c); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac345; continue;
 }
 0x4ac063 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4ac072); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ac081; continue; }
  pc = 0x4ac07a; continue;
 }
 0x4ac07a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac081 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x80u32); cpu.ecx = v; }
  cpu.push(0x4ac096); f_004abe08(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ac0a5; continue; }
  pc = 0x4ac09e; continue;
 }
 0x4ac09e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac0a5 => {
  pc = 0x4ac35e; continue;
 }
 0x4ac0aa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac0b3); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4ac311; continue; }
  pc = 0x4ac0c9; continue;
 }
 0x4ac0c9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac0d6); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f83d0u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4ac0f0); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4ac106); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac10e; continue;
 }
 0x4ac10e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac117); f_004ac375(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ac142; continue; }
  pc = 0x4ac127; continue;
 }
 0x4ac127 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac13a); f_004ac37f(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac142 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac161); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ac1a9; continue; }
  pc = 0x4ac175; continue;
 }
 0x4ac175 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4ae0e0u32; cpu.push(v); }
  { let v = 0x4ac449u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac19b); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac1b0; continue;
 }
 0x4ac1a9 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac1b0; continue;
 }
 0x4ac1b0 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4ac1e5; continue; }
  pc = 0x4ac1ca; continue;
 }
 0x4ac1ca => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac1dd); f_004ac37f(cpu);
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac1e5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac1f4; continue;
 }
 0x4ac1f1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac1f4; continue;
 }
 0x4ac1f4 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4ac305; continue; }
  pc = 0x4ac203; continue;
 }
 0x4ac203 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f73a0u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac21e); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac227); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4ac252; continue; }
  pc = 0x4ac237; continue;
 }
 0x4ac237 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac24a); f_004ac37f(cpu);
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac252 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac26f; continue;
 }
 0x4ac26f => {
  pc = 0x4ac274; continue;
 }
 0x4ac274 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ac2d9; continue;
 }
 0x4ac2d9 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ac300; continue; }
  pc = 0x4ac2ef; continue;
 }
 0x4ac2ef => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4ac2fb); cpu.call(t);
  pc = 0x4ac300; continue;
 }
 0x4ac300 => {
  pc = 0x4ac1f1; continue;
 }
 0x4ac305 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ac311); f_004ac37f(cpu);
  pc = 0x4ac311; continue;
 }
 0x4ac311 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4ac320); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ac32f; continue; }
  pc = 0x4ac328; continue;
 }
 0x4ac328 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac32f => {
  pc = 0x4ac35e; continue;
 }
 0x4ac334 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac345 => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ac063; continue; }
  pc = 0x4ac34f; continue;
 }
 0x4ac34f => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x4ac0aa; continue; }
  pc = 0x4ac359; continue;
 }
 0x4ac359 => {
  pc = 0x4ac334; continue;
 }
 0x4ac35e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ac393; continue;
 }
 0x4ac393 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae6d0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ac022; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae6e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae6e0); }
 let mut pc: u32 = 0x4ae6e0;
 loop { match pc {
 0x4ac3a4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac3bb); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ac3ce); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac3e9; continue;
 }
 0x4ac3e6 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac3e9; continue;
 }
 0x4ac3e9 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4ac42e; continue; }
  pc = 0x4ac3f8; continue;
 }
 0x4ac3f8 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x4ac416; continue; }
  pc = 0x4ac402; continue;
 }
 0x4ac402 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4ac411); cpu.call(t);
  pc = 0x4ac425; continue;
 }
 0x4ac416 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x4ac425); cpu.call(t);
  pc = 0x4ac425; continue;
 }
 0x4ac425 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x2cu32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ac3e6; continue;
 }
 0x4ac42e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4ac43d); f_004a2f17(cpu);
  pc = 0x4ac442; continue;
 }
 0x4ac442 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae6e0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ac3a4; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae6f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae6f0); }
 let mut pc: u32 = 0x4ae6f0;
 loop { match pc {
 0x4ae6f0 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ae700; continue;
 }
 0x4ae700 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ae711; continue;
 }
 0x4ae711 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4ae719); f_004acdfa(cpu);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4ae727; continue;
 }
 0x4ae727 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae742; continue; }
  pc = 0x4ae731; continue;
 }
 0x4ae731 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae73f); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4ae742; continue;
 }
 0x4ae742 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x80u32); cpu.eax = v; }
  pc = 0x4ae74f; continue;
 }
 0x4ae74f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae760); }
 let mut pc: u32 = 0x4ae760;
 loop { match pc {
 0x4ad0f9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4ad47eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad123); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad42c; continue;
 }
 0x4ad13a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4ad149); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ad158; continue; }
  pc = 0x4ad151; continue;
 }
 0x4ad151 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ad488; continue;
 }
 0x4ad158 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x80u32); cpu.ecx = v; }
  cpu.push(0x4ad16d); f_004acecd(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ad17c; continue; }
  pc = 0x4ad175; continue;
 }
 0x4ad175 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ad488; continue;
 }
 0x4ad17c => {
  pc = 0x4ad445; continue;
 }
 0x4ad181 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad18a); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4ad3f8; continue; }
  pc = 0x4ad1a0; continue;
 }
 0x4ad1a0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad1ad); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f83f8u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4ad1c7); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4ad1dd); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad1e5; continue;
 }
 0x4ad1e5 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ad1ee); f_004ad46a(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ad219; continue; }
  pc = 0x4ad1fe; continue;
 }
 0x4ad1fe => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ad211); f_004ad474(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ad488; continue;
 }
 0x4ad219 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad238); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ad280; continue; }
  pc = 0x4ad24c; continue;
 }
 0x4ad24c => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x4ae200u32; cpu.push(v); }
  { let v = 0x4ad675u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x38u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad272); f_004aea70(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad287; continue;
 }
 0x4ad280 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad287; continue;
 }
 0x4ad287 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4ad2bc; continue; }
  pc = 0x4ad2a1; continue;
 }
 0x4ad2a1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ad2b4); f_004ad474(cpu);
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ad488; continue;
 }
 0x4ad2bc => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad2cb; continue;
 }
 0x4ad2c8 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad2cb; continue;
 }
 0x4ad2cb => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4ad3ec; continue; }
  pc = 0x4ad2da; continue;
 }
 0x4ad2da => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f73f0u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad2f5); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad2fe); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4ad329; continue; }
  pc = 0x4ad30e; continue;
 }
 0x4ad30e => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ad321); f_004ad474(cpu);
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4ad488; continue;
 }
 0x4ad329 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad344; continue;
 }
 0x4ad344 => {
  pc = 0x4ad349; continue;
 }
 0x4ad349 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x18u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x4ad3c0; continue;
 }
 0x4ad3c0 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ad3e7; continue; }
  pc = 0x4ad3d6; continue;
 }
 0x4ad3d6 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4ad3e2); cpu.call(t);
  pc = 0x4ad3e7; continue;
 }
 0x4ad3e7 => {
  pc = 0x4ad2c8; continue;
 }
 0x4ad3ec => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4ad3f8); f_004ad474(cpu);
  pc = 0x4ad3f8; continue;
 }
 0x4ad3f8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x4ad407); f_004a26d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4ad416; continue; }
  pc = 0x4ad40f; continue;
 }
 0x4ad40f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ad488; continue;
 }
 0x4ad416 => {
  pc = 0x4ad445; continue;
 }
 0x4ad41b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4ad488; continue;
 }
 0x4ad42c => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4ad13a; continue; }
  pc = 0x4ad436; continue;
 }
 0x4ad436 => {
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x4ad181; continue; }
  pc = 0x4ad440; continue;
 }
 0x4ad440 => {
  pc = 0x4ad41b; continue;
 }
 0x4ad445 => {
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x80u32); cpu.ecx = v; }
  cpu.push(0x4ad453); f_004ad53e(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4ad488; continue;
 }
 0x4ad488 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae760 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ad0f9; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae770); }
 let mut pc: u32 = 0x4ae770;
 loop { match pc {
 0x4ad499 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad4b0); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ad4c3); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad4de; continue;
 }
 0x4ad4db => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad4de; continue;
 }
 0x4ad4de => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x4ad523; continue; }
  pc = 0x4ad4ed; continue;
 }
 0x4ad4ed => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x4ad50b; continue; }
  pc = 0x4ad4f7; continue;
 }
 0x4ad4f7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4ad506); cpu.call(t);
  pc = 0x4ad51a; continue;
 }
 0x4ad50b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x4ad51a); cpu.call(t);
  pc = 0x4ad51a; continue;
 }
 0x4ad51a => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x38u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4ad4db; continue;
 }
 0x4ad523 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4ad532); f_004a2f17(cpu);
  pc = 0x4ad537; continue;
 }
 0x4ad537 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x4ae770 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  pc = 0x4ad499; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae778(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae778); }
 let mut pc: u32 = 0x4ae778;
 loop { match pc {
 0x4ae778 => {
  pc = cpu.r32(0x5254e0u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae77e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae77e); }
 let mut pc: u32 = 0x4ae77e;
 loop { match pc {
 0x4ae77e => {
  pc = cpu.r32(0x5254dcu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae784(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae784); }
 let mut pc: u32 = 0x4ae784;
 loop { match pc {
 0x4ae784 => {
  pc = cpu.r32(0x5254f0u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae78a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae78a); }
 let mut pc: u32 = 0x4ae78a;
 loop { match pc {
 0x4ae78a => {
  pc = cpu.r32(0x5254e8u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae790); }
 let mut pc: u32 = 0x4ae790;
 loop { match pc {
 0x4ae790 => {
  pc = cpu.r32(0x5254ecu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae796(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae796); }
 let mut pc: u32 = 0x4ae796;
 loop { match pc {
 0x4ae796 => {
  pc = cpu.r32(0x5254f8u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae7a0); }
 let mut pc: u32 = 0x4ae7a0;
 loop { match pc {
 0x4ae7a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae7aa); f_004ae880(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae7b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae7b0); }
 let mut pc: u32 = 0x4ae7b0;
 loop { match pc {
 0x4ae7b0 => {
  pc = cpu.r32(0x5256bcu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae7b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae7b6); }
 let mut pc: u32 = 0x4ae7b6;
 loop { match pc {
 0x4ae7b6 => {
  pc = cpu.r32(0x5256d0u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae7c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae7c0); }
 let mut pc: u32 = 0x4ae7c0;
 loop { match pc {
 0x4ae7c0 => {
  { cpu.sub32(cpu.r32(0x524334u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x4ae7d8; continue; }
  pc = 0x4ae7c9; continue;
 }
 0x4ae7c9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525698u32); cpu.push(0x4ae7d4); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4ae7d8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x524330u32; cpu.push(v); }
  { let v = 0x524334u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4ae7ec); f_004aec56(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae7f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae7f0); }
 let mut pc: u32 = 0x4ae7f0;
 loop { match pc {
 0x4ae7f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4ae7fa); f_004ae7c0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae810); }
 let mut pc: u32 = 0x4ae810;
 loop { match pc {
 0x4ae810 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x4ae818); f_004aec5c(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x4ae828; continue; }
  pc = 0x4ae81f; continue;
 }
 0x4ae81f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4ae825); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4ae828; continue;
 }
 0x4ae828 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae836(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae836); }
 let mut pc: u32 = 0x4ae836;
 loop { match pc {
 0x4ae836 => {
  pc = cpu.r32(0x5256d4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae83c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae83c); }
 let mut pc: u32 = 0x4ae83c;
 loop { match pc {
 0x4ae83c => {
  pc = cpu.r32(0x525680u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae842(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae842); }
 let mut pc: u32 = 0x4ae842;
 loop { match pc {
 0x4ae842 => {
  pc = cpu.r32(0x52567cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004ae850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae850); }
 let mut pc: u32 = 0x4ae850;
 loop { match pc {
 0x4ae850 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x1000u32); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  if cpu.cf { pc = 0x4ae870; continue; }
  pc = 0x4ae85c; continue;
 }
 0x4ae85c => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x1000u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1000u32); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); }
  { cpu.sub32(cpu.eax, 0x1000u32); }
  if !cpu.cf { pc = 0x4ae85c; continue; }
  pc = 0x4ae870; continue;
 }
 0x4ae870 => {
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
pub fn f_004ae880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4ae880); }
 let mut pc: u32 = 0x4ae880;
 loop { match pc {
 0x4ae880 => {
  pc = cpu.r32(0x52563cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
