#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_00497fce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x497fce); }
 let mut pc: u32 = 0x497fce;
 loop { match pc {
 0x497fce => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x497fd6); f_004a32a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00497fd7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x497fd7); }
 let mut pc: u32 = 0x497fd7;
 loop { match pc {
 0x497fd7 => {
  { let v: u32 = 0x4bba60u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00497ff1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x497ff1); }
 let mut pc: u32 = 0x497ff1;
 loop { match pc {
 0x497ff1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4980adu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x498022); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49802e; continue;
 }
 0x49802e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49804a; continue;
 }
 0x49804a => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498051; continue;
 }
 0x498051 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49805a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x498093; continue; }
  pc = 0x498062; continue;
 }
 0x498062 => {
  pc = 0x498067; continue;
 }
 0x498067 => {
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
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x498085); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49808e); cpu.call(t);
  pc = 0x498051; continue;
 }
 0x498093 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49809f); f_004980a4(cpu);
  pc = 0x4980b7; continue;
 }
 0x4980b7 => {
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
pub fn f_00498000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498000); }
 let mut pc: u32 = 0x498000;
 loop { match pc {
 0x498000 => {
  { let v: u8 = cpu.r8(0x64u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x498022); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49802e; continue;
 }
 0x49802e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49804a; continue;
 }
 0x49804a => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498051; continue;
 }
 0x498051 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49805a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x498093; continue; }
  pc = 0x498062; continue;
 }
 0x498062 => {
  pc = 0x498067; continue;
 }
 0x498067 => {
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
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x498085); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49808e); cpu.call(t);
  pc = 0x498051; continue;
 }
 0x498093 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49809f); f_004980a4(cpu);
  pc = 0x4980b7; continue;
 }
 0x4980b7 => {
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
pub fn f_0049802b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49802b); }
 let mut pc: u32 = 0x49802b;
 loop { match pc {
 0x49802b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  cpu.call(0x498031); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498040); }
 let mut pc: u32 = 0x498040;
 loop { match pc {
 0x498040 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x498049); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049807a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49807a); }
 let mut pc: u32 = 0x49807a;
 loop { match pc {
 0x49800b => {
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x53u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x498022); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49802e; continue;
 }
 0x49802e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49804a; continue;
 }
 0x49804a => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498051; continue;
 }
 0x498051 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49805a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x498093; continue; }
  pc = 0x498062; continue;
 }
 0x498062 => {
  pc = 0x498067; continue;
 }
 0x498067 => {
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
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x498085); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49808e); cpu.call(t);
  pc = 0x498051; continue;
 }
 0x49807a => {
  { let v: u8 = cpu.or8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x49800b; continue; }
  pc = 0x498080; continue;
 }
 0x498080 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  cpu.unimplemented(0x498081, "In");
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x15ffe84du32.wrapping_add(cpu.ebp)) | (cpu.ecx as u8); cpu.w8(0x15ffe84du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8) ^ 0x57u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8((cpu.ecx as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xc7ffffffu32; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  cpu.call(0x498096); return;
 }
 0x498093 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49809f); f_004980a4(cpu);
  pc = 0x4980b7; continue;
 }
 0x4980b7 => {
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
pub fn f_004980a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4980a4); }
 let mut pc: u32 = 0x4980a4;
 loop { match pc {
 0x4980a4 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4980ac); f_004a3240(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004980ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4980ad); }
 let mut pc: u32 = 0x4980ad;
 loop { match pc {
 0x4980ad => {
  { let v: u32 = 0x4bbaa0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004980c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4980c7); }
 let mut pc: u32 = 0x4980c7;
 loop { match pc {
 0x4980c7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49868du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xc0u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x24u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x498101); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49810d; continue;
 }
 0x49810d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498129; continue;
 }
 0x498129 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498130; continue;
 }
 0x498130 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x498139); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49818b; continue; }
  pc = 0x498141; continue;
 }
 0x498141 => {
  pc = 0x498146; continue;
 }
 0x498146 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49815f; continue;
 }
 0x49815f => {
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x49817d); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x498186); cpu.call(t);
  pc = 0x498130; continue;
 }
 0x49818b => {
  { let v: u32 = 0x4b3304u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498197; continue;
 }
 0x498197 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b3314u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4981a7; continue;
 }
 0x4981a7 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4981bf; continue;
 }
 0x4981bf => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff38u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4981e5); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4981ee; continue;
 }
 0x4981ee => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498207; continue;
 }
 0x498207 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x498224); cpu.call(t);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49822d; continue;
 }
 0x49822d => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498246; continue;
 }
 0x498246 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525770u32); cpu.push(0x498253); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525774u32); cpu.push(0x49826d); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49827d; continue; }
  pc = 0x498278; continue;
 }
 0x498278 => {
  pc = 0x498511; continue;
 }
 0x49827d => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x498286); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x498293; continue; }
  pc = 0x49828e; continue;
 }
 0x49828e => {
  pc = 0x498511; continue;
 }
 0x498293 => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff48u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff48u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525778u32); cpu.push(0x4982af); cpu.call(t);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4982b8; continue;
 }
 0x4982b8 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff4cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4982da; continue;
 }
 0x4982da => {
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff40u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525778u32); cpu.push(0x4982fa); cpu.call(t);
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498303; continue;
 }
 0x498303 => {
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff44u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498325; continue;
 }
 0x498325 => {
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x498335); cpu.call(t);
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff74u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x498344); f_004a52d0(cpu);
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x498354); f_004a52d0(cpu);
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498358; continue;
 }
 0x498358 => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525784u32); cpu.push(0x498369); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4984af; continue; }
  pc = 0x498374; continue;
 }
 0x498374 => {
  pc = 0x498379; continue;
 }
 0x498379 => {
  pc = 0x49837e; continue;
 }
 0x49837e => {
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4983a3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x498492; continue; }
  pc = 0x4983ab; continue;
 }
 0x4983ab => {
  pc = 0x4983b0; continue;
 }
 0x4983b0 => {
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff50u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4983c5); f_00483d99(cpu);
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff74u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52577cu32); cpu.push(0x4983dc); cpu.call(t);
  pc = 0x4983e1; continue;
 }
 0x4983e1 => {
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52577cu32); cpu.push(0x4983f1); cpu.call(t);
  pc = 0x4983f6; continue;
 }
 0x4983f6 => {
  pc = 0x4983fb; continue;
 }
 0x4983fb => {
  pc = 0x498400; continue;
 }
 0x498400 => {
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x498415); f_00483dfd(cpu);
  { let v: u32 = 0xffffff74u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525780u32); cpu.push(0x498421); cpu.call(t);
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525780u32); cpu.push(0x49842a); cpu.call(t);
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525784u32); cpu.push(0x498438); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x498472; continue; }
  pc = 0x498443; continue;
 }
 0x498443 => {
  pc = 0x498448; continue;
 }
 0x498448 => {
  { let v: u32 = 0xffffff50u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x49846a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x498400; continue; }
  pc = 0x498472; continue;
 }
 0x498472 => {
  pc = 0x498477; continue;
 }
 0x498477 => {
  { let v: u32 = 0xffffff50u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x498489); f_00483dfd(cpu);
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498492); f_004984d8(cpu);
  pc = 0x498492; continue;
 }
 0x498492 => {
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49849e); cpu.call(t);
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4984aa); cpu.call(t);
  pc = 0x498358; continue;
 }
 0x4984af => {
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4984b8); f_004984e4(cpu);
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4984c1); f_004984ed(cpu);
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4984ca); f_004984f9(cpu);
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4984d3); f_00498505(cpu);
  pc = 0x498511; continue;
 }
 0x498511 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49851a); f_00498528(cpu);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498523); f_00498531(cpu);
  pc = 0x49853a; continue;
 }
 0x49853a => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498543); f_0049867b(cpu);
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4985aa; continue; }
  pc = 0x49854d; continue;
 }
 0x49854d => {
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x49855c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4985aa; continue; }
  pc = 0x498564; continue;
 }
 0x498564 => {
  pc = 0x498569; continue;
 }
 0x498569 => {
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x49857a); f_00483ebf(cpu);
  { let a = cpu.st(0); let b = cpu.rf32(0x4b31c8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x4985a0; continue; }
  pc = 0x49858c; continue;
 }
 0x49858c => {
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x49859b); cpu.call(t);
  pc = 0x4985a5; continue;
 }
 0x4985a0 => {
  pc = 0x4985aa; continue;
 }
 0x4985a5 => {
  pc = 0x49854d; continue;
 }
 0x4985aa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4985c3); cpu.call(t);
  { let v: u8 = 0xfu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4985cc; continue;
 }
 0x4985cc => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4985e5; continue;
 }
 0x4985e5 => {
  { let v: u8 = 0x10u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4985e9; continue;
 }
 0x4985e9 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4985f2); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49864f; continue; }
  pc = 0x4985fa; continue;
 }
 0x4985fa => {
  pc = 0x4985ff; continue;
 }
 0x4985ff => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x49860a); f_00483ea4(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x498621); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x498641; continue; }
  pc = 0x498629; continue;
 }
 0x498629 => {
  pc = 0x49862e; continue;
 }
 0x49862e => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x498639); f_00483ebf(cpu);
  { let v = cpu.st(0); cpu.wf32(0xffffffe4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x49864f; continue;
 }
 0x498641 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49864a); cpu.call(t);
  pc = 0x4985e9; continue;
 }
 0x49864f => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49865e); f_00498672(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49866a); f_00498684(cpu);
  { let v = cpu.rf32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  pc = 0x498697; continue;
 }
 0x498697 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049811f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49811f); }
 let mut pc: u32 = 0x49811f;
 loop { match pc {
 0x49811f => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x498128); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004981b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4981b6); }
 let mut pc: u32 = 0x4981b6;
 loop { match pc {
 0x4981b6 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4981be); f_004a3300(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004981fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4981fd); }
 let mut pc: u32 = 0x4981fd;
 loop { match pc {
 0x4981fd => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x498206); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049823c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49823c); }
 let mut pc: u32 = 0x49823c;
 loop { match pc {
 0x49823c => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x498245); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004982cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4982cd); }
 let mut pc: u32 = 0x4982cd;
 loop { match pc {
 0x4982cd => {
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4982d9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498318(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498318); }
 let mut pc: u32 = 0x498318;
 loop { match pc {
 0x498318 => {
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x498324); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004984d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4984d8); }
 let mut pc: u32 = 0x4984d8;
 loop { match pc {
 0x4984d8 => {
  { let v: u32 = 0xffffff50u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4984e3); f_00483d4c(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004984e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4984e4); }
 let mut pc: u32 = 0x4984e4;
 loop { match pc {
 0x4984e4 => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4984ec); f_004a33c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004984ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4984ed); }
 let mut pc: u32 = 0x4984ed;
 loop { match pc {
 0x4984ed => {
  { let v: u32 = 0xffffff74u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4984f8); f_004a33c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004984f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4984f9); }
 let mut pc: u32 = 0x4984f9;
 loop { match pc {
 0x4984f9 => {
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x498504); f_004a33c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498505(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498505); }
 let mut pc: u32 = 0x498505;
 loop { match pc {
 0x498505 => {
  { let v: u32 = 0xffffff68u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x498510); f_004a33c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498528(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498528); }
 let mut pc: u32 = 0x498528;
 loop { match pc {
 0x498528 => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x498530); f_004a33c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498531(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498531); }
 let mut pc: u32 = 0x498531;
 loop { match pc {
 0x498531 => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x498539); f_004a33c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004985db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4985db); }
 let mut pc: u32 = 0x4985db;
 loop { match pc {
 0x4985db => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4985e4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498672(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498672); }
 let mut pc: u32 = 0x498672;
 loop { match pc {
 0x498672 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49867a); f_004a33c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049867b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49867b); }
 let mut pc: u32 = 0x49867b;
 loop { match pc {
 0x49867b => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x498683); f_004a3360(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498684(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498684); }
 let mut pc: u32 = 0x498684;
 loop { match pc {
 0x498684 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49868c); f_004a32a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049868d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49868d); }
 let mut pc: u32 = 0x49868d;
 loop { match pc {
 0x49868d => {
  { let v: u32 = 0x4bbad0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004986a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4986a7); }
 let mut pc: u32 = 0x4986a7;
 loop { match pc {
 0x4986a7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4987ddu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4986d8); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4986e4; continue;
 }
 0x4986e4 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498700; continue;
 }
 0x498700 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498707; continue;
 }
 0x498707 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x498710); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4987b7; continue; }
  pc = 0x498718; continue;
 }
 0x498718 => {
  pc = 0x49871d; continue;
 }
 0x49871d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x498728); f_00483ebf(cpu);
  { let a = cpu.st(0); let b = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if cpu.zf { pc = 0x4987a4; continue; }
  pc = 0x498737; continue;
 }
 0x498737 => {
  pc = 0x49873c; continue;
 }
 0x49873c => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x498747); f_00483ea4(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49874f; continue;
 }
 0x49874f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x49875a); f_00483edb(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.push(0x498773); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49879f; continue; }
  pc = 0x498780; continue;
 }
 0x498780 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.push(0x49879f); cpu.call(t);
  pc = 0x49879f; continue;
 }
 0x49879f => {
  pc = 0x4987a9; continue;
 }
 0x4987a4 => {
  pc = 0x4987b7; continue;
 }
 0x4987a9 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4987b2); cpu.call(t);
  pc = 0x498707; continue;
 }
 0x4987b7 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4987c3); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4987cf); f_004987d4(cpu);
  pc = 0x4987e7; continue;
 }
 0x4987e7 => {
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
pub fn f_004986f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4986f6); }
 let mut pc: u32 = 0x4986f6;
 loop { match pc {
 0x4986f6 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4986ff); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004987d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4987d4); }
 let mut pc: u32 = 0x4987d4;
 loop { match pc {
 0x4987d4 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4987dc); f_004a33c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004987dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4987dd); }
 let mut pc: u32 = 0x4987dd;
 loop { match pc {
 0x4987dd => {
  { let v: u32 = 0x4bbb78u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004987f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4987f7); }
 let mut pc: u32 = 0x4987f7;
 loop { match pc {
 0x4987f7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4988f7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x24u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x498828); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498834; continue;
 }
 0x498834 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498850; continue;
 }
 0x498850 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498857; continue;
 }
 0x498857 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x498860); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4988dd; continue; }
  pc = 0x498868; continue;
 }
 0x498868 => {
  pc = 0x49886d; continue;
 }
 0x49886d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if !cpu.zf { pc = 0x4988cf; continue; }
  pc = 0x49887f; continue;
 }
 0x49887f => {
  pc = 0x498884; continue;
 }
 0x498884 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4988a1); cpu.call(t);
  pc = 0x4988a6; continue;
 }
 0x4988a6 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4988be); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x4988ca); cpu.call(t);
  pc = 0x498857; continue;
 }
 0x4988cf => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4988d8); cpu.call(t);
  pc = 0x498857; continue;
 }
 0x4988dd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4988e9); f_004988ee(cpu);
  pc = 0x498901; continue;
 }
 0x498901 => {
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
pub fn f_00498846(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498846); }
 let mut pc: u32 = 0x498846;
 loop { match pc {
 0x498846 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49884f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004988ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4988ee); }
 let mut pc: u32 = 0x4988ee;
 loop { match pc {
 0x4988ee => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4988f6); f_004a32a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004988f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4988f7); }
 let mut pc: u32 = 0x4988f7;
 loop { match pc {
 0x4988f7 => {
  { let v: u32 = 0x4bbba8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498911(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498911); }
 let mut pc: u32 = 0x498911;
 loop { match pc {
 0x498911 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x498a39u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x498940; continue;
 }
 0x498940 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3324u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x498955; continue;
 }
 0x498955 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498970; continue;
 }
 0x498970 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x498983); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b33a4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x498996; continue;
 }
 0x498996 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4989b5; continue;
 }
 0x4989b5 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4989c5); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3394u32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x4989d8; continue;
 }
 0x4989d8 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4989f7; continue;
 }
 0x4989f7 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b335cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x498a09; continue;
 }
 0x498a09 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x498a43; continue;
 }
 0x498a43 => {
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
pub fn f_00498967(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498967); }
 let mut pc: u32 = 0x498967;
 loop { match pc {
 0x498967 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49896f); f_00489b10(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004989a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4989a8); }
 let mut pc: u32 = 0x4989a8;
 loop { match pc {
 0x4989a8 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4989b4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004989ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4989ea); }
 let mut pc: u32 = 0x4989ea;
 loop { match pc {
 0x4989ea => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4989f6); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498a18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498a18); }
 let mut pc: u32 = 0x498a18;
 loop { match pc {
 0x498a18 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x498a23); f_004a4ba0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498a24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498a24); }
 let mut pc: u32 = 0x498a24;
 loop { match pc {
 0x498a24 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  cpu.push(0x498a2f); f_004a4a70(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498a30); }
 let mut pc: u32 = 0x498a30;
 loop { match pc {
 0x498a30 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x498a38); f_004a34a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498a39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498a39); }
 let mut pc: u32 = 0x498a39;
 loop { match pc {
 0x498a39 => {
  { let v: u32 = 0x4bbbd8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498a52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498a52); }
 let mut pc: u32 = 0x498a52;
 loop { match pc {
 0x498a52 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x498b23u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b335cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x498a93); f_004993c5(cpu);
  pc = 0x498a93; continue;
 }
 0x498a93 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x498a9f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x498ada; continue; }
  pc = 0x498aa7; continue;
 }
 0x498aa7 => {
  pc = 0x498aac; continue;
 }
 0x498aac => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x498ac9); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x498ad5); cpu.call(t);
  pc = 0x498a93; continue;
 }
 0x498ada => {
  pc = 0x498adf; continue;
 }
 0x498adf => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498ae8); f_00498b02(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498af1); f_00498b0e(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498afd); f_00498b1a(cpu);
  pc = 0x498b2d; continue;
 }
 0x498b2d => {
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
pub fn f_00498b02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498b02); }
 let mut pc: u32 = 0x498b02;
 loop { match pc {
 0x498b02 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x498b0d); f_004a4ba0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498b0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498b0e); }
 let mut pc: u32 = 0x498b0e;
 loop { match pc {
 0x498b0e => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  cpu.push(0x498b19); f_004a4a70(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498b1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498b1a); }
 let mut pc: u32 = 0x498b1a;
 loop { match pc {
 0x498b1a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x498b22); f_004a34a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498b23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498b23); }
 let mut pc: u32 = 0x498b23;
 loop { match pc {
 0x498b23 => {
  { let v: u32 = 0x4bbc28u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498b3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498b3b); }
 let mut pc: u32 = 0x498b3b;
 loop { match pc {
 0x498b3b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x498d0cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x498b67); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x498b73); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x498c36; continue; }
  pc = 0x498b7b; continue;
 }
 0x498b7b => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7ae4u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x498b8b); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x498b9d); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498ba5; continue;
 }
 0x498ba5 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7b24u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x498bb5); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x498bcb); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498bd3; continue;
 }
 0x498bd3 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x498be2); cpu.call(t);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498beb); f_00498cf8(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x498bfa); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x498c05); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x498c0d); cpu.call(t);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498c16); f_00498cee(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498c29); f_00498d02(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x498d16; continue;
 }
 0x498c36 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x498c41); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x498ce2; continue; }
  pc = 0x498c49; continue;
 }
 0x498c49 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x498c62); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x498c9d; continue; }
  pc = 0x498c79; continue;
 }
 0x498c79 => {
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x498c82); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x498c92; continue;
 }
 0x498c92 => {
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498ca4; continue;
 }
 0x498c9d => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498ca4; continue;
 }
 0x498ca4 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x498cc1); cpu.call(t);
  pc = 0x498cd3; continue;
 }
 0x498cd3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x498d16; continue;
 }
 0x498ce2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x498d16; continue;
 }
 0x498d16 => {
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
pub fn f_00498cc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498cc6); }
 let mut pc: u32 = 0x498cc6;
 loop { match pc {
 0x498cc6 => {
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x498ccf); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498cee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498cee); }
 let mut pc: u32 = 0x498cee;
 loop { match pc {
 0x498cee => {
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x498cf7); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498cf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498cf8); }
 let mut pc: u32 = 0x498cf8;
 loop { match pc {
 0x498cf8 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x498d01); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498d02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498d02); }
 let mut pc: u32 = 0x498d02;
 loop { match pc {
 0x498d02 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x498d0b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498d0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498d0c); }
 let mut pc: u32 = 0x498d0c;
 loop { match pc {
 0x498d0c => {
  { let v: u32 = 0x4bbc60u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498d27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498d27); }
 let mut pc: u32 = 0x498d27;
 loop { match pc {
 0x498d27 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x498f4cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x498d58); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498d64; continue;
 }
 0x498d64 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498d80; continue;
 }
 0x498d80 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498d87; continue;
 }
 0x498d87 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x498d90); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x498e73; continue; }
  pc = 0x498d98; continue;
 }
 0x498d98 => {
  pc = 0x498d9d; continue;
 }
 0x498d9d => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x498db9); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x498dc4); cpu.call(t);
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x498e65; continue; }
  pc = 0x498dcc; continue;
 }
 0x498dcc => {
  pc = 0x498dd1; continue;
 }
 0x498dd1 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x498ded); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x498df9); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x498e0b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x498e18; continue; }
  pc = 0x498e13; continue;
 }
 0x498e13 => {
  pc = 0x498e1d; continue;
 }
 0x498e18 => {
  pc = 0x498e65; continue;
 }
 0x498e1d => {
  pc = 0x498e2c; continue;
 }
 0x498e2c => {
  pc = 0x498e31; continue;
 }
 0x498e31 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x498e4b); cpu.call(t);
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x498e54); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498e60); f_00498f43(cpu);
  pc = 0x498f56; continue;
 }
 0x498e65 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x498e6e); cpu.call(t);
  pc = 0x498d87; continue;
 }
 0x498e73 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7b44u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x498e83); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x498e92); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498e9a; continue;
 }
 0x498e9a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7b80u32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x498eaa); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x498ec0); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498ec8; continue;
 }
 0x498ec8 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x498ed7); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498ee0); f_00498f2f(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x498eef); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x498efa); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x498f02); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498f0b); f_00498f25(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498f14); f_00498f39(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x498f20); f_00498f43(cpu);
  pc = 0x498f56; continue;
 }
 0x498f56 => {
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
pub fn f_00498d76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498d76); }
 let mut pc: u32 = 0x498d76;
 loop { match pc {
 0x498d76 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x498d7f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498f25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498f25); }
 let mut pc: u32 = 0x498f25;
 loop { match pc {
 0x498f25 => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x498f2e); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498f2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498f2f); }
 let mut pc: u32 = 0x498f2f;
 loop { match pc {
 0x498f2f => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x498f38); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498f39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498f39); }
 let mut pc: u32 = 0x498f39;
 loop { match pc {
 0x498f39 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x498f42); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498f43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498f43); }
 let mut pc: u32 = 0x498f43;
 loop { match pc {
 0x498f43 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x498f4b); f_004a35e0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498f4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498f4c); }
 let mut pc: u32 = 0x498f4c;
 loop { match pc {
 0x498f4c => {
  { let v: u32 = 0x4bbca0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00498f66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498f66); }
 let mut pc: u32 = 0x498f66;
 loop { match pc {
 0x498f66 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4991e3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x498f97); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498fa3; continue;
 }
 0x498fa3 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x498fbf; continue;
 }
 0x498fbf => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x498fc6; continue;
 }
 0x498fc6 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x498fcf); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4990e5; continue; }
  pc = 0x498fd7; continue;
 }
 0x498fd7 => {
  pc = 0x498fdc; continue;
 }
 0x498fdc => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x498ff6); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x499008); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499015; continue; }
  pc = 0x499010; continue;
 }
 0x499010 => {
  pc = 0x49901a; continue;
 }
 0x499015 => {
  pc = 0x4990d7; continue;
 }
 0x49901a => {
  pc = 0x499029; continue;
 }
 0x499029 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7b90u32; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499039); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499042; continue;
 }
 0x499042 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7bbcu32; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499052); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499068); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499070; continue;
 }
 0x499070 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49907f); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499088); f_004991c6(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x499097); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x4990a2); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x4990aa); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4990b3); f_004991bc(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4990c3); f_004991d0(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4990cf); f_004991da(cpu);
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4991ed; continue;
 }
 0x4990d7 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4990e0); cpu.call(t);
  pc = 0x498fc6; continue;
 }
 0x4990e5 => {
  { let v = 0x44u32; cpu.push(v); }
  cpu.push(0x4990ec); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x499114; continue; }
  pc = 0x499100; continue;
 }
 0x499100 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49910c); f_004965df(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49911b; continue;
 }
 0x499114 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49911b; continue;
 }
 0x49911b => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49912c); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x499162; continue; }
  pc = 0x499140; continue;
 }
 0x499140 => {
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x499149); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x499157; continue;
 }
 0x499157 => {
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499169; continue;
 }
 0x499162 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499169; continue;
 }
 0x499169 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x499183); cpu.call(t);
  pc = 0x499195; continue;
 }
 0x499195 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4991a7); f_004991da(cpu);
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4991ed; continue;
 }
 0x4991ed => {
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
pub fn f_00498fb5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x498fb5); }
 let mut pc: u32 = 0x498fb5;
 loop { match pc {
 0x498fb5 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x498fbe); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499188(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499188); }
 let mut pc: u32 = 0x499188;
 loop { match pc {
 0x499188 => {
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x499191); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004991af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4991af); }
 let mut pc: u32 = 0x4991af;
 loop { match pc {
 0x4991af => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4991b8); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004991bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4991bc); }
 let mut pc: u32 = 0x4991bc;
 loop { match pc {
 0x4991bc => {
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x4991c5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004991c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4991c6); }
 let mut pc: u32 = 0x4991c6;
 loop { match pc {
 0x4991c6 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4991cf); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004991d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4991d0); }
 let mut pc: u32 = 0x4991d0;
 loop { match pc {
 0x4991d0 => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4991d9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004991da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4991da); }
 let mut pc: u32 = 0x4991da;
 loop { match pc {
 0x4991da => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4991e2); f_004a3640(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004991e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4991e3); }
 let mut pc: u32 = 0x4991e3;
 loop { match pc {
 0x4991e3 => {
  { let v: u32 = 0x4bbce8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004991fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4991fe); }
 let mut pc: u32 = 0x4991fe;
 loop { match pc {
 0x4991fe => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4993abu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x50u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49922f); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49923b; continue;
 }
 0x49923b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499257; continue;
 }
 0x499257 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49925e; continue;
 }
 0x49925e => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x499267); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4992d2; continue; }
  pc = 0x49926f; continue;
 }
 0x49926f => {
  pc = 0x499274; continue;
 }
 0x499274 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if !cpu.zf { pc = 0x4992c4; continue; }
  pc = 0x499283; continue;
 }
 0x499283 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4992aa; continue; }
  pc = 0x499299; continue;
 }
 0x499299 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4992a5); cpu.call(t);
  pc = 0x4992aa; continue;
 }
 0x4992aa => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x4992b3); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4992bf); f_004993a2(cpu);
  pc = 0x4993b5; continue;
 }
 0x4992c4 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4992cd); cpu.call(t);
  pc = 0x49925e; continue;
 }
 0x4992d2 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7bd0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4992e2); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4992f1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4992f9; continue;
 }
 0x4992f9 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7bfcu32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x499309); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49931f); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499327; continue;
 }
 0x499327 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x499336); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49933f); f_0049938e(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49934e); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x499359); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x499361); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49936a); f_00499384(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499373); f_00499398(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49937f); f_004993a2(cpu);
  pc = 0x4993b5; continue;
 }
 0x4993b5 => {
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
pub fn f_0049924d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49924d); }
 let mut pc: u32 = 0x49924d;
 loop { match pc {
 0x49924d => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x499256); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499384(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499384); }
 let mut pc: u32 = 0x499384;
 loop { match pc {
 0x499384 => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x49938d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049938e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49938e); }
 let mut pc: u32 = 0x49938e;
 loop { match pc {
 0x49938e => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x499397); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499398(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499398); }
 let mut pc: u32 = 0x499398;
 loop { match pc {
 0x499398 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4993a1); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004993a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4993a2); }
 let mut pc: u32 = 0x4993a2;
 loop { match pc {
 0x4993a2 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4993aa); f_004a3640(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004993ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4993ab); }
 let mut pc: u32 = 0x4993ab;
 loop { match pc {
 0x4993ab => {
  { let v: u32 = 0x4bbd40u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004993c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4993c5); }
 let mut pc: u32 = 0x4993c5;
 loop { match pc {
 0x4993c5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4993d1; continue;
 }
 0x4993d1 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4993dd); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499428; continue; }
  pc = 0x4993e5; continue;
 }
 0x4993e5 => {
  pc = 0x4993ea; continue;
 }
 0x4993ea => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x499417; continue; }
  pc = 0x499406; continue;
 }
 0x499406 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x499412); cpu.call(t);
  pc = 0x499417; continue;
 }
 0x499417 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x499423); cpu.call(t);
  pc = 0x4993d1; continue;
 }
 0x499428 => {
  pc = 0x49942d; continue;
 }
 0x49942d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499432(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499432); }
 let mut pc: u32 = 0x499432;
 loop { match pc {
 0x499432 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x499571u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x499463); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49946f; continue;
 }
 0x49946f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49948b; continue;
 }
 0x49948b => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499492; continue;
 }
 0x499492 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49949b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49954d; continue; }
  pc = 0x4994a3; continue;
 }
 0x4994a3 => {
  pc = 0x4994a8; continue;
 }
 0x4994a8 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x4994c2); cpu.call(t);
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x49953f; continue; }
  pc = 0x4994cb; continue;
 }
 0x4994cb => {
  pc = 0x4994d0; continue;
 }
 0x4994d0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4994ea); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4994fc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499509; continue; }
  pc = 0x499504; continue;
 }
 0x499504 => {
  pc = 0x49950e; continue;
 }
 0x499509 => {
  pc = 0x49953f; continue;
 }
 0x49950e => {
  pc = 0x49951d; continue;
 }
 0x49951d => {
  pc = 0x499522; continue;
 }
 0x499522 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499537); f_00499568(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49957b; continue;
 }
 0x49953f => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x499548); cpu.call(t);
  pc = 0x499492; continue;
 }
 0x49954d => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499560); f_00499568(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49957b; continue;
 }
 0x49957b => {
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
pub fn f_00499481(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499481); }
 let mut pc: u32 = 0x499481;
 loop { match pc {
 0x499481 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49948a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499568(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499568); }
 let mut pc: u32 = 0x499568;
 loop { match pc {
 0x499568 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x499570); f_004a35e0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499571(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499571); }
 let mut pc: u32 = 0x499571;
 loop { match pc {
 0x499571 => {
  { let v: u32 = 0x4bbd88u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049958c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49958c); }
 let mut pc: u32 = 0x49958c;
 loop { match pc {
 0x49958c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4996a3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4995bd); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4995c9; continue;
 }
 0x4995c9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4995e5; continue;
 }
 0x4995e5 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4995ec; continue;
 }
 0x4995ec => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4995f5); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49967f; continue; }
  pc = 0x4995fd; continue;
 }
 0x4995fd => {
  pc = 0x499602; continue;
 }
 0x499602 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x49961c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x49962e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49963b; continue; }
  pc = 0x499636; continue;
 }
 0x499636 => {
  pc = 0x499640; continue;
 }
 0x49963b => {
  pc = 0x499671; continue;
 }
 0x499640 => {
  pc = 0x49964f; continue;
 }
 0x49964f => {
  pc = 0x499654; continue;
 }
 0x499654 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499669); f_0049969a(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4996ad; continue;
 }
 0x499671 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49967a); cpu.call(t);
  pc = 0x4995ec; continue;
 }
 0x49967f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x499692); f_0049969a(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4996ad; continue;
 }
 0x4996ad => {
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
pub fn f_004995db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4995db); }
 let mut pc: u32 = 0x4995db;
 loop { match pc {
 0x4995db => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4995e4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049969a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49969a); }
 let mut pc: u32 = 0x49969a;
 loop { match pc {
 0x49969a => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4996a2); f_004a35e0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004996a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4996a3); }
 let mut pc: u32 = 0x4996a3;
 loop { match pc {
 0x4996a3 => {
  { let v: u32 = 0x4bbdb8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004996be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4996be); }
 let mut pc: u32 = 0x4996be;
 loop { match pc {
 0x4996be => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4998afu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49979f; continue; }
  pc = 0x4996f0; continue;
 }
 0x4996f0 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b33b8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b33b0u32; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  pc = 0x499712; continue;
 }
 0x499712 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x499746; continue;
 }
 0x499746 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x34u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x499769; continue;
 }
 0x499769 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499794; continue;
 }
 0x499794 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49979f; continue;
 }
 0x49979f => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x4997ab); cpu.call(t);
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4997be); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2cccu32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x4997d1; continue;
 }
 0x4997d1 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4997f0; continue;
 }
 0x4997f0 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b33fcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b33ccu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x49985b; continue;
 }
 0x49985b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4998b9; continue;
 }
 0x4998b9 => {
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
pub fn f_0049977e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49977e); }
 let mut pc: u32 = 0x49977e;
 loop { match pc {
 0x49977e => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x499793; continue; }
  pc = 0x499788; continue;
 }
 0x499788 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x38u32); cpu.ecx = v; }
  cpu.push(0x499793); f_00489b10(cpu);
  pc = 0x499793; continue;
 }
 0x499793 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004997e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4997e3); }
 let mut pc: u32 = 0x4997e3;
 loop { match pc {
 0x4997e3 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4997ef); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049986a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49986a); }
 let mut pc: u32 = 0x49986a;
 loop { match pc {
 0x49986a => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x499875); f_00490d80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499876(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499876); }
 let mut pc: u32 = 0x499876;
 loop { match pc {
 0x499876 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x499882); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499883(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499883); }
 let mut pc: u32 = 0x499883;
 loop { match pc {
 0x499883 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x499898; continue; }
  pc = 0x49988d; continue;
 }
 0x49988d => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x30u32); cpu.ecx = v; }
  cpu.push(0x499898); f_00490b40(cpu);
  pc = 0x499898; continue;
 }
 0x499898 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499899(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499899); }
 let mut pc: u32 = 0x499899;
 loop { match pc {
 0x499899 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4998ae; continue; }
  pc = 0x4998a3; continue;
 }
 0x4998a3 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x4998ae); f_00489b10(cpu);
  pc = 0x4998ae; continue;
 }
 0x4998ae => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004998af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4998af); }
 let mut pc: u32 = 0x4998af;
 loop { match pc {
 0x4998af => {
  { let v: u32 = 0x4bbde8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049998b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49998b); }
 let mut pc: u32 = 0x49998b;
 loop { match pc {
 0x49998b => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x499994); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d11); }
 let mut pc: u32 = 0x499d11;
 loop { match pc {
 0x499d11 => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x499d1a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d1b); }
 let mut pc: u32 = 0x499d1b;
 loop { match pc {
 0x499d1b => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x499d24); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d25); }
 let mut pc: u32 = 0x499d25;
 loop { match pc {
 0x499d25 => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x499d2e); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d2f); }
 let mut pc: u32 = 0x499d2f;
 loop { match pc {
 0x499d2f => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x499d38); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d39); }
 let mut pc: u32 = 0x499d39;
 loop { match pc {
 0x499d39 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x499d41); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d42); }
 let mut pc: u32 = 0x499d42;
 loop { match pc {
 0x499d42 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x499d4a); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d4b); }
 let mut pc: u32 = 0x499d4b;
 loop { match pc {
 0x499d4b => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x499d53); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d54); }
 let mut pc: u32 = 0x499d54;
 loop { match pc {
 0x499d54 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x499d5c); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d5d); }
 let mut pc: u32 = 0x499d5d;
 loop { match pc {
 0x499d5d => {
  { let v: u32 = 0x4bbe38u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499d78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499d78); }
 let mut pc: u32 = 0x499d78;
 loop { match pc {
 0x499d78 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499d84; continue;
 }
 0x499d84 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x499d90); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x499df9; continue; }
  pc = 0x499d98; continue;
 }
 0x499d98 => {
  pc = 0x499d9d; continue;
 }
 0x499d9d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x499dbf; continue; }
  pc = 0x499db3; continue;
 }
 0x499db3 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499dd1; continue;
 }
 0x499dbf => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x499dd1; continue;
 }
 0x499dd1 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x499de8); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x499df4); cpu.call(t);
  pc = 0x499d84; continue;
 }
 0x499df9 => {
  pc = 0x499dfe; continue;
 }
 0x499dfe => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00499fa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x499fa9); }
 let mut pc: u32 = 0x499fa9;
 loop { match pc {
 0x499fa9 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x499fb2); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a0e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a0e1); }
 let mut pc: u32 = 0x49a0e1;
 loop { match pc {
 0x49a0e1 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49a0ea); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a109(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a109); }
 let mut pc: u32 = 0x49a109;
 loop { match pc {
 0x49a109 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49a111); f_004a36c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a112(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a112); }
 let mut pc: u32 = 0x49a112;
 loop { match pc {
 0x49a112 => {
  { let v: u32 = 0x4bbea0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a182(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a182); }
 let mut pc: u32 = 0x49a182;
 loop { match pc {
 0x49a182 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49a18b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a3a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a3a7); }
 let mut pc: u32 = 0x49a3a7;
 loop { match pc {
 0x49a3a7 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x49a3b0); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a3b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a3b1); }
 let mut pc: u32 = 0x49a3b1;
 loop { match pc {
 0x49a3b1 => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49a3ba); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a3bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a3bb); }
 let mut pc: u32 = 0x49a3bb;
 loop { match pc {
 0x49a3bb => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49a3c4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a3c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a3c5); }
 let mut pc: u32 = 0x49a3c5;
 loop { match pc {
 0x49a3c5 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49a3ce); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a3cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a3cf); }
 let mut pc: u32 = 0x49a3cf;
 loop { match pc {
 0x49a3cf => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49a3d7); f_004a36c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a3d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a3d8); }
 let mut pc: u32 = 0x49a3d8;
 loop { match pc {
 0x49a3d8 => {
  { let v: u32 = 0x4bbed8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a3f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a3f2); }
 let mut pc: u32 = 0x49a3f2;
 loop { match pc {
 0x49a3f2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49a405; continue;
 }
 0x49a405 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a40c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a40c); }
 let mut pc: u32 = 0x49a40c;
 loop { match pc {
 0x49a40c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49a50bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49a43d); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a449; continue;
 }
 0x49a449 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a465; continue;
 }
 0x49a465 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a46c; continue;
 }
 0x49a46c => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49a475); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a4e7; continue; }
  pc = 0x49a47d; continue;
 }
 0x49a47d => {
  pc = 0x49a482; continue;
 }
 0x49a482 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x49a4b2); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49a4d9; continue; }
  pc = 0x49a4bf; continue;
 }
 0x49a4bf => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a4d1); f_0049a502(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49a515; continue;
 }
 0x49a4d9 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49a4e2); cpu.call(t);
  pc = 0x49a46c; continue;
 }
 0x49a4e7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49a4fa); f_0049a502(cpu);
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49a515; continue;
 }
 0x49a515 => {
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
pub fn f_0049a45b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a45b); }
 let mut pc: u32 = 0x49a45b;
 loop { match pc {
 0x49a45b => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49a464); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a502(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a502); }
 let mut pc: u32 = 0x49a502;
 loop { match pc {
 0x49a502 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49a50a); f_004a36c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a50b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a50b); }
 let mut pc: u32 = 0x49a50b;
 loop { match pc {
 0x49a50b => {
  { let v: u32 = 0x4bbf28u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a526(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a526); }
 let mut pc: u32 = 0x49a526;
 loop { match pc {
 0x49a526 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49a70bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49a57c; continue; }
  pc = 0x49a558; continue;
 }
 0x49a558 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3448u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x49a571; continue;
 }
 0x49a571 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a57c; continue;
 }
 0x49a57c => {
  pc = 0x49a5a5; continue;
 }
 0x49a5a5 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3434u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b341cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49a5d1; continue;
 }
 0x49a5d1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a5f9; continue;
 }
 0x49a5f9 => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7cbcu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49a613); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7cccu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49a62a); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x20u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x49a63a); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1174u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  pc = 0x49a64d; continue;
 }
 0x49a64d => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a66c; continue;
 }
 0x49a66c => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b346cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3454u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  pc = 0x49a6b7; continue;
 }
 0x49a6b7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49a715; continue;
 }
 0x49a715 => {
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
pub fn f_0049a5e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a5e3); }
 let mut pc: u32 = 0x49a5e3;
 loop { match pc {
 0x49a5e3 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x49a5f8; continue; }
  pc = 0x49a5ed; continue;
 }
 0x49a5ed => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  cpu.push(0x49a5f8); f_00485370(cpu);
  pc = 0x49a5f8; continue;
 }
 0x49a5f8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a65f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a65f); }
 let mut pc: u32 = 0x49a65f;
 loop { match pc {
 0x49a65f => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x49a66b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a6c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a6c6); }
 let mut pc: u32 = 0x49a6c6;
 loop { match pc {
 0x49a6c6 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  cpu.push(0x49a6d1); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a6d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a6d2); }
 let mut pc: u32 = 0x49a6d2;
 loop { match pc {
 0x49a6d2 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49a6de); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a6df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a6df); }
 let mut pc: u32 = 0x49a6df;
 loop { match pc {
 0x49a6df => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49a6eb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a6ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a6ec); }
 let mut pc: u32 = 0x49a6ec;
 loop { match pc {
 0x49a6ec => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49a6f4); f_004a3770(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a6f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a6f5); }
 let mut pc: u32 = 0x49a6f5;
 loop { match pc {
 0x49a6f5 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49a70a; continue; }
  pc = 0x49a6ff; continue;
 }
 0x49a6ff => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  cpu.push(0x49a70a); f_00485370(cpu);
  pc = 0x49a70a; continue;
 }
 0x49a70a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a70b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a70b); }
 let mut pc: u32 = 0x49a70b;
 loop { match pc {
 0x49a70b => {
  { let v: u32 = 0x4bbf58u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a7e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a7e3); }
 let mut pc: u32 = 0x49a7e3;
 loop { match pc {
 0x49a7e3 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x49a7f8; continue; }
  pc = 0x49a7ed; continue;
 }
 0x49a7ed => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  cpu.push(0x49a7f8); f_00485370(cpu);
  pc = 0x49a7f8; continue;
 }
 0x49a7f8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a859(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a859); }
 let mut pc: u32 = 0x49a859;
 loop { match pc {
 0x49a859 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x49a865); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a8bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a8bf); }
 let mut pc: u32 = 0x49a8bf;
 loop { match pc {
 0x49a8bf => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  cpu.push(0x49a8ca); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a8cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a8cb); }
 let mut pc: u32 = 0x49a8cb;
 loop { match pc {
 0x49a8cb => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49a8d7); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a8d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a8d8); }
 let mut pc: u32 = 0x49a8d8;
 loop { match pc {
 0x49a8d8 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49a8e4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a8e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a8e5); }
 let mut pc: u32 = 0x49a8e5;
 loop { match pc {
 0x49a8e5 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49a8ed); f_004a3770(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a8ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a8ee); }
 let mut pc: u32 = 0x49a8ee;
 loop { match pc {
 0x49a8ee => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49a903; continue; }
  pc = 0x49a8f8; continue;
 }
 0x49a8f8 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  cpu.push(0x49a903); f_00485370(cpu);
  pc = 0x49a903; continue;
 }
 0x49a903 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a91f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a91f); }
 let mut pc: u32 = 0x49a91f;
 loop { match pc {
 0x49a91f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xcu32); cpu.eax = v; }
  pc = 0x49a936; continue;
 }
 0x49a936 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a93b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a93b); }
 let mut pc: u32 = 0x49a93b;
 loop { match pc {
 0x49a93b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x14u32); cpu.eax = v; }
  pc = 0x49a952; continue;
 }
 0x49a952 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a957(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a957); }
 let mut pc: u32 = 0x49a957;
 loop { match pc {
 0x49a957 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x49a96e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x49a980); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49a992; continue; }
  pc = 0x49a988; continue;
 }
 0x49a988 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49a994; continue;
 }
 0x49a992 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49a994; continue;
 }
 0x49a994 => {
  pc = 0x49a99c; continue;
 }
 0x49a99c => {
  pc = 0x49a9a1; continue;
 }
 0x49a9a1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a9a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a9a8); }
 let mut pc: u32 = 0x49a9a8;
 loop { match pc {
 0x49a9a8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49a9bf; continue;
 }
 0x49a9bf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049a9c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a9c4); }
 let mut pc: u32 = 0x49a9c4;
 loop { match pc {
 0x49a9c4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49ac13u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49a9ee); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49abdd; continue;
 }
 0x49aa05 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49aa12); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x28u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x49aa29); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49aa35); f_0049ac09(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49aa42); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x49aa59); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49aa65); f_0049abff(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49aa6e); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49aa83); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49aa98; continue;
 }
 0x49aa95 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49aa98; continue;
 }
 0x49aa98 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x49abc7; continue; }
  pc = 0x49aaa4; continue;
 }
 0x49aaa4 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6a10u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49aabb); f_004af4ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49aac4); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49aadb; continue; }
  pc = 0x49aad4; continue;
 }
 0x49aad4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49ac1d; continue;
 }
 0x49aadb => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49aaf1; continue; }
  pc = 0x49aaeb; continue;
 }
 0x49aaeb => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49aaf1; continue;
 }
 0x49aaf1 => {
  pc = 0x49aaf6; continue;
 }
 0x49aaf6 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49ab0a); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ab83; continue; }
  pc = 0x49ab1e; continue;
 }
 0x49ab1e => {
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49ab33); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49ab55; continue; }
  pc = 0x49ab4b; continue;
 }
 0x49ab4b => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)).wrapping_add(0x1u32); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49ab55; continue;
 }
 0x49ab55 => {
  pc = 0x49ab5a; continue;
 }
 0x49ab5a => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ab63; continue;
 }
 0x49ab63 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ab7e; continue;
 }
 0x49ab7e => {
  pc = 0x49ab8a; continue;
 }
 0x49ab83 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ab8a; continue;
 }
 0x49ab8a => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49aba4); cpu.call(t);
  pc = 0x49abb6; continue;
 }
 0x49abb6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49abc2); f_0049abf6(cpu);
  pc = 0x49aa95; continue;
 }
 0x49abc7 => {
  pc = 0x49abec; continue;
 }
 0x49abcc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49ac1d; continue;
 }
 0x49abdd => {
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49aa05; continue; }
  pc = 0x49abe7; continue;
 }
 0x49abe7 => {
  pc = 0x49abcc; continue;
 }
 0x49abec => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49ac1d; continue;
 }
 0x49ac1d => {
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
pub fn f_0049ab72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ab72); }
 let mut pc: u32 = 0x49ab72;
 loop { match pc {
 0x49ab72 => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x49ab7d); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049aba9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49aba9); }
 let mut pc: u32 = 0x49aba9;
 loop { match pc {
 0x49aba9 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49abb2); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049abf6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49abf6); }
 let mut pc: u32 = 0x49abf6;
 loop { match pc {
 0x49abf6 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49abfe); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049abff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49abff); }
 let mut pc: u32 = 0x49abff;
 loop { match pc {
 0x49abff => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49ac08); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ac09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ac09); }
 let mut pc: u32 = 0x49ac09;
 loop { match pc {
 0x49ac09 => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49ac12); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ac13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ac13); }
 let mut pc: u32 = 0x49ac13;
 loop { match pc {
 0x49ac13 => {
  { let v: u32 = 0x4bc008u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ac2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ac2e); }
 let mut pc: u32 = 0x49ac2e;
 loop { match pc {
 0x49ac2e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49ad6bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ac5a); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x28u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ac6d); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ac80); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ac93); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x49aca2); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49acac); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49acbf); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49accb; continue;
 }
 0x49accb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ace7; continue;
 }
 0x49ace7 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49acee; continue;
 }
 0x49acee => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49acf7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49ad47; continue; }
  pc = 0x49acff; continue;
 }
 0x49acff => {
  pc = 0x49ad04; continue;
 }
 0x49ad04 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ad12; continue;
 }
 0x49ad12 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x49ad39); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49ad42); cpu.call(t);
  pc = 0x49acee; continue;
 }
 0x49ad47 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49ad5a); f_0049ad62(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ad75; continue;
 }
 0x49ad75 => {
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
pub fn f_0049acdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49acdd); }
 let mut pc: u32 = 0x49acdd;
 loop { match pc {
 0x49acdd => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49ace6); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ad62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ad62); }
 let mut pc: u32 = 0x49ad62;
 loop { match pc {
 0x49ad62 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49ad6a); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ad6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ad6b); }
 let mut pc: u32 = 0x49ad6b;
 loop { match pc {
 0x49ad6b => {
  { let v: u32 = 0x4bc050u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ae07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ae07); }
 let mut pc: u32 = 0x49ae07;
 loop { match pc {
 0x49ae07 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49af0bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ae3f; continue; }
  pc = 0x49ae39; continue;
 }
 0x49ae39 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49ae3f; continue;
 }
 0x49ae3f => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49aea7; continue; }
  pc = 0x49ae4c; continue;
 }
 0x49ae4c => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49aea7; continue; }
  pc = 0x49ae65; continue;
 }
 0x49ae65 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49aea7; continue; }
  pc = 0x49ae7e; continue;
 }
 0x49ae7e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x49aea2); cpu.call(t);
  pc = 0x49aea7; continue;
 }
 0x49aea7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x34u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49aeb5; continue;
 }
 0x49aeb5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4b17c8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49aefa); f_0049af02(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49af15; continue;
 }
 0x49af15 => {
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
pub fn f_0049af02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49af02); }
 let mut pc: u32 = 0x49af02;
 loop { match pc {
 0x49af02 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49af0a); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049af0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49af0b); }
 let mut pc: u32 = 0x49af0b;
 loop { match pc {
 0x49af0b => {
  { let v: u32 = 0x4bc080u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049af58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49af58); }
 let mut pc: u32 = 0x49af58;
 loop { match pc {
 0x49af58 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49af6f; continue;
 }
 0x49af6f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049af74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49af74); }
 let mut pc: u32 = 0x49af74;
 loop { match pc {
 0x49af74 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49af8b; continue;
 }
 0x49af8b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049af90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49af90); }
 let mut pc: u32 = 0x49af90;
 loop { match pc {
 0x49af90 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49afa7; continue;
 }
 0x49afa7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049afac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49afac); }
 let mut pc: u32 = 0x49afac;
 loop { match pc {
 0x49afac => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49afc3; continue;
 }
 0x49afc3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049afc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49afc8); }
 let mut pc: u32 = 0x49afc8;
 loop { match pc {
 0x49afc8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49afd9; continue;
 }
 0x49afd9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49afe7; continue;
 }
 0x49afe7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049afec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49afec); }
 let mut pc: u32 = 0x49afec;
 loop { match pc {
 0x49afec => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49b207u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49b024; continue; }
  pc = 0x49b01a; continue;
 }
 0x49b01a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49b211; continue;
 }
 0x49b024 => {
  pc = 0x49b029; continue;
 }
 0x49b029 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x49b043); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49b058); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49b1f3; continue; }
  pc = 0x49b065; continue;
 }
 0x49b065 => {
  pc = 0x49b076; continue;
 }
 0x49b076 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49b09e); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b0aa; continue;
 }
 0x49b0aa => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b0c6; continue;
 }
 0x49b0c6 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b0db; continue;
 }
 0x49b0d2 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49b0db); cpu.call(t);
  pc = 0x49b0db; continue;
 }
 0x49b0db => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49b0e4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b19f; continue; }
  pc = 0x49b0ec; continue;
 }
 0x49b0ec => {
  pc = 0x49b0f1; continue;
 }
 0x49b0f1 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x49b102); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4b17c8u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if cpu.zf { pc = 0x49b19a; continue; }
  pc = 0x49b113; continue;
 }
 0x49b113 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x49b122); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b13b; continue; }
  pc = 0x49b12f; continue;
 }
 0x49b12f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b14d; continue;
 }
 0x49b13b => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b14d; continue;
 }
 0x49b14d => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49b18e; continue; }
  pc = 0x49b157; continue;
 }
 0x49b157 => {
  pc = 0x49b15c; continue;
 }
 0x49b15c => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.push(0x49b186); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b19a; continue; }
  pc = 0x49b18e; continue;
 }
 0x49b18e => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b19f; continue;
 }
 0x49b19a => {
  pc = 0x49b0d2; continue;
 }
 0x49b19f => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49b1e7; continue; }
  pc = 0x49b1a9; continue;
 }
 0x49b1a9 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x49b1ba); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4b17c8u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if cpu.zf { pc = 0x49b1e7; continue; }
  pc = 0x49b1cb; continue;
 }
 0x49b1cb => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  pc = 0x49b1e7; continue;
 }
 0x49b1e7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49b1f3); f_0049b1fe(cpu);
  pc = 0x49b1f3; continue;
 }
 0x49b1f3 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49b211; continue;
 }
 0x49b211 => {
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
pub fn f_0049b0bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b0bc); }
 let mut pc: u32 = 0x49b0bc;
 loop { match pc {
 0x49b0bc => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49b0c5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b1fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b1fe); }
 let mut pc: u32 = 0x49b1fe;
 loop { match pc {
 0x49b1fe => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49b206); f_004a37a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b207(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b207); }
 let mut pc: u32 = 0x49b207;
 loop { match pc {
 0x49b207 => {
  { let v: u32 = 0x4bc0a8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b220); }
 let mut pc: u32 = 0x49b220;
 loop { match pc {
 0x49b220 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x49b23e; continue; }
  pc = 0x49b239; continue;
 }
 0x49b239 => {
  pc = 0x49b282; continue;
 }
 0x49b23e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x49b249); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49b25e; continue; }
  pc = 0x49b251; continue;
 }
 0x49b251 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.push(0x49b25e); cpu.call(t);
  pc = 0x49b25e; continue;
 }
 0x49b25e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4b17c8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  pc = 0x49b282; continue;
 }
 0x49b282 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b287(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b287); }
 let mut pc: u32 = 0x49b287;
 loop { match pc {
 0x49b287 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x49b29e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b2ad; continue; }
  pc = 0x49b2a6; continue;
 }
 0x49b2a6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b303; continue;
 }
 0x49b2ad => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49b2c4; continue; }
  pc = 0x49b2ba; continue;
 }
 0x49b2ba => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49b303; continue;
 }
 0x49b2c4 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x49b2e0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b2ef; continue; }
  pc = 0x49b2e8; continue;
 }
 0x49b2e8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b303; continue;
 }
 0x49b2ef => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49b303; continue;
 }
 0x49b303 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b308(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b308); }
 let mut pc: u32 = 0x49b308;
 loop { match pc {
 0x49b308 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x49b326; continue; }
  pc = 0x49b321; continue;
 }
 0x49b321 => {
  pc = 0x49b357; continue;
 }
 0x49b326 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b348; continue; }
  pc = 0x49b330; continue;
 }
 0x49b330 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49b348); cpu.call(t);
  pc = 0x49b348; continue;
 }
 0x49b348 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x49b357; continue;
 }
 0x49b357 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b35e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b35e); }
 let mut pc: u32 = 0x49b35e;
 loop { match pc {
 0x49b35e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49b4ccu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49b3b4; continue; }
  pc = 0x49b390; continue;
 }
 0x49b390 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3490u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  pc = 0x49b3a9; continue;
 }
 0x49b3a9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b3b4; continue;
 }
 0x49b3b4 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49b3be); f_0049a526(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x49b3d1); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x3cu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x49b3e1); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x44u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x49b3f1); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3484u32; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  pc = 0x49b404; continue;
 }
 0x49b404 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b423; continue;
 }
 0x49b423 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b34b4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b349cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x54u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x49b478; continue;
 }
 0x49b478 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49b4d6; continue;
 }
 0x49b4d6 => {
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
pub fn f_0049b416(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b416); }
 let mut pc: u32 = 0x49b416;
 loop { match pc {
 0x49b416 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x44u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x49b422); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b487(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b487); }
 let mut pc: u32 = 0x49b487;
 loop { match pc {
 0x49b487 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x44u32); cpu.ecx = v; }
  cpu.push(0x49b492); f_004a4030(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b493(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b493); }
 let mut pc: u32 = 0x49b493;
 loop { match pc {
 0x49b493 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x3cu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b49f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b4a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b4a0); }
 let mut pc: u32 = 0x49b4a0;
 loop { match pc {
 0x49b4a0 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b4ac); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b4ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b4ad); }
 let mut pc: u32 = 0x49b4ad;
 loop { match pc {
 0x49b4ad => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49b4b5); f_004a3800(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b4b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b4b6); }
 let mut pc: u32 = 0x49b4b6;
 loop { match pc {
 0x49b4b6 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49b4cb; continue; }
  pc = 0x49b4c0; continue;
 }
 0x49b4c0 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x58u32); cpu.ecx = v; }
  cpu.push(0x49b4cb); f_00485370(cpu);
  pc = 0x49b4cb; continue;
 }
 0x49b4cb => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b4cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b4cc); }
 let mut pc: u32 = 0x49b4cc;
 loop { match pc {
 0x49b4cc => {
  { let v: u32 = 0x4bc0d8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b4e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b4e7); }
 let mut pc: u32 = 0x49b4e7;
 loop { match pc {
 0x49b4e7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49b7deu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x64u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b511); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b794; continue;
 }
 0x49b528 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b531); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b547); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x49b55e); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49b56a); f_0049b7d4(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b577); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x49b58e); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49b59a); f_0049b7ca(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b5a3); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b5b8; continue;
 }
 0x49b5b5 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b5b8; continue;
 }
 0x49b5b8 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x49b760; continue; }
  pc = 0x49b5c4; continue;
 }
 0x49b5c4 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x49b5cd); cpu.call(t);
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x49b5dd); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b5e6; continue;
 }
 0x49b5e6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b60c; continue;
 }
 0x49b60c => {
  { let v: u32 = 0x4u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b620); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x49b631); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49b63a); f_0049b7b7(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b647); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x49b658); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49b661); f_0049b7ad(cpu);
  { let v = 0x18u32; cpu.push(v); }
  cpu.push(0x49b668); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49b71c; continue; }
  pc = 0x49b67c; continue;
 }
 0x49b67c => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49b691); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x49b6a7); cpu.call(t);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x49b6be); cpu.call(t);
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b6c7; continue;
 }
 0x49b6c7 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b6f3; continue;
 }
 0x49b6f3 => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b6fc; continue;
 }
 0x49b6fc => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b717; continue;
 }
 0x49b717 => {
  pc = 0x49b723; continue;
 }
 0x49b71c => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b723; continue;
 }
 0x49b723 => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49b73d); cpu.call(t);
  pc = 0x49b74f; continue;
 }
 0x49b74f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49b75b); f_0049b7c1(cpu);
  pc = 0x49b5b5; continue;
 }
 0x49b760 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x24u32); cpu.ecx = v; }
  cpu.push(0x49b76f); f_0049a9c4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b77e; continue; }
  pc = 0x49b777; continue;
 }
 0x49b777 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b7e8; continue;
 }
 0x49b77e => {
  pc = 0x49b7a3; continue;
 }
 0x49b783 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b7e8; continue;
 }
 0x49b794 => {
  { cpu.sub32(cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49b528; continue; }
  pc = 0x49b79e; continue;
 }
 0x49b79e => {
  pc = 0x49b783; continue;
 }
 0x49b7a3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49b7e8; continue;
 }
 0x49b7e8 => {
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
pub fn f_0049b5f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b5f8); }
 let mut pc: u32 = 0x49b5f8;
 loop { match pc {
 0x49b5f8 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b601); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b602(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b602); }
 let mut pc: u32 = 0x49b602;
 loop { match pc {
 0x49b602 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b60b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b6d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b6d9); }
 let mut pc: u32 = 0x49b6d9;
 loop { match pc {
 0x49b6d9 => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b6e5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b6e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b6e6); }
 let mut pc: u32 = 0x49b6e6;
 loop { match pc {
 0x49b6e6 => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b6f2); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b70b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b70b); }
 let mut pc: u32 = 0x49b70b;
 loop { match pc {
 0x49b70b => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x49b716); f_004a38d0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b742(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b742); }
 let mut pc: u32 = 0x49b742;
 loop { match pc {
 0x49b742 => {
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b74b); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b7ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b7ad); }
 let mut pc: u32 = 0x49b7ad;
 loop { match pc {
 0x49b7ad => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b7b6); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b7b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b7b7); }
 let mut pc: u32 = 0x49b7b7;
 loop { match pc {
 0x49b7b7 => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b7c0); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b7c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b7c1); }
 let mut pc: u32 = 0x49b7c1;
 loop { match pc {
 0x49b7c1 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49b7c9); f_004a38d0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b7ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b7ca); }
 let mut pc: u32 = 0x49b7ca;
 loop { match pc {
 0x49b7ca => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b7d3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b7d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b7d4); }
 let mut pc: u32 = 0x49b7d4;
 loop { match pc {
 0x49b7d4 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49b7dd); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b7de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b7de); }
 let mut pc: u32 = 0x49b7de;
 loop { match pc {
 0x49b7de => {
  { let v: u32 = 0x4bc128u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b7f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b7f9); }
 let mut pc: u32 = 0x49b7f9;
 loop { match pc {
 0x49b7f9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49b937u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b825); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b838); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x24u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b84b); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b85e); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x49b86d); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b877); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49b88a); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b896; continue;
 }
 0x49b896 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b8b2; continue;
 }
 0x49b8b2 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b8b9; continue;
 }
 0x49b8b9 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49b8c2); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b908; continue; }
  pc = 0x49b8ca; continue;
 }
 0x49b8ca => {
  pc = 0x49b8cf; continue;
 }
 0x49b8cf => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b8df); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x49b8e7; continue;
 }
 0x49b8e7 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b8f7); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49b903); cpu.call(t);
  pc = 0x49b8b9; continue;
 }
 0x49b908 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  cpu.push(0x49b917); f_0049ac2e(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49b926); f_0049b92e(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49b941; continue;
 }
 0x49b941 => {
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
pub fn f_0049b8a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b8a8); }
 let mut pc: u32 = 0x49b8a8;
 loop { match pc {
 0x49b8a8 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49b8b1); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b92e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b92e); }
 let mut pc: u32 = 0x49b92e;
 loop { match pc {
 0x49b92e => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49b936); f_004a37a0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b937(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b937); }
 let mut pc: u32 = 0x49b937;
 loop { match pc {
 0x49b937 => {
  { let v: u32 = 0x4bc1a0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b952(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b952); }
 let mut pc: u32 = 0x49b952;
 loop { match pc {
 0x49b952 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49bc74u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  pc = 0x49b992; continue;
 }
 0x49b992 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x44u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49b9ca); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b9d3; continue;
 }
 0x49b9d3 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b9ec; continue;
 }
 0x49b9ec => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b9f0; continue;
 }
 0x49b9f0 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49b9f9); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49bbc9; continue; }
  pc = 0x49ba01; continue;
 }
 0x49ba01 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49ba24); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49ba45; continue; }
  pc = 0x49ba39; continue;
 }
 0x49ba39 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ba57; continue;
 }
 0x49ba45 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ba57; continue;
 }
 0x49ba57 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f7648u32; cpu.push(v); }
  { let v = 0x4c27c8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49ba77); f_004a36a0(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49ba80); f_00405cb0(cpu);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x49ba91); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ba97); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49baac; continue; }
  pc = 0x49baa7; continue;
 }
 0x49baa7 => {
  pc = 0x49bbc9; continue;
 }
 0x49baac => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49babe); f_004a36a0(cpu);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49bac7); f_00405cb0(cpu);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x49bad9); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49bb4b; continue; }
  pc = 0x49bae1; continue;
 }
 0x49bae1 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49bb04); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49bb25; continue; }
  pc = 0x49bb19; continue;
 }
 0x49bb19 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bb37; continue;
 }
 0x49bb25 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bb37; continue;
 }
 0x49bb37 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49bb46); cpu.call(t);
  pc = 0x49bbc9; continue;
 }
 0x49bb4b => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49bb52); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49bb88; continue; }
  pc = 0x49bb66; continue;
 }
 0x49bb66 => {
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49bb6f); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49bb7d; continue;
 }
 0x49bb7d => {
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bb8f; continue;
 }
 0x49bb88 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bb8f; continue;
 }
 0x49bb8f => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49bba9); cpu.call(t);
  pc = 0x49bbbb; continue;
 }
 0x49bbbb => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49bbc4); cpu.call(t);
  pc = 0x49b9f0; continue;
 }
 0x49bbc9 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49bbd2); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49bc21; continue; }
  pc = 0x49bbda; continue;
 }
 0x49bbda => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x49bbfd); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49bc0d); f_0049bc62(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49bc19); f_0049bc6b(cpu);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49bc7e; continue;
 }
 0x49bc21 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.push(0x49bc2c); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49bc3c); f_0049bc62(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49bc48); f_0049bc6b(cpu);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49bc7e; continue;
 }
 0x49bc7e => {
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
pub fn f_0049b9e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b9e2); }
 let mut pc: u32 = 0x49b9e2;
 loop { match pc {
 0x49b9e2 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49b9eb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bbae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bbae); }
 let mut pc: u32 = 0x49bbae;
 loop { match pc {
 0x49bbae => {
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49bbb7); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bc50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bc50); }
 let mut pc: u32 = 0x49bc50;
 loop { match pc {
 0x49bc50 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49bc58); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bc59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bc59); }
 let mut pc: u32 = 0x49bc59;
 loop { match pc {
 0x49bc59 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49bc61); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bc62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bc62); }
 let mut pc: u32 = 0x49bc62;
 loop { match pc {
 0x49bc62 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49bc6a); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bc6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bc6b); }
 let mut pc: u32 = 0x49bc6b;
 loop { match pc {
 0x49bc6b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49bc73); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bc74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bc74); }
 let mut pc: u32 = 0x49bc74;
 loop { match pc {
 0x49bc74 => {
  { let v: u32 = 0x4bc1d0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bd4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bd4b); }
 let mut pc: u32 = 0x49bd4b;
 loop { match pc {
 0x49bd4b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49bd62; continue;
 }
 0x49bd62 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bd67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bd67); }
 let mut pc: u32 = 0x49bd67;
 loop { match pc {
 0x49bd67 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x49bd7f); cpu.call(t);
  pc = 0x49bd84; continue;
 }
 0x49bd84 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bd89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bd89); }
 let mut pc: u32 = 0x49bd89;
 loop { match pc {
 0x49bd89 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49be55u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49bdc6); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bdd2; continue;
 }
 0x49bdd2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bdee; continue;
 }
 0x49bdee => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49be04; continue;
 }
 0x49be01 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49be04; continue;
 }
 0x49be04 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x49be1e; continue; }
  pc = 0x49be10; continue;
 }
 0x49be10 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49be19); cpu.call(t);
  pc = 0x49be01; continue;
 }
 0x49be1e => {
  pc = 0x49be23; continue;
 }
 0x49be23 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49be38); f_0049be47(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49be50; continue;
 }
 0x49be50 => {
  pc = 0x49be5f; continue;
 }
 0x49be5f => {
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
pub fn f_0049bde4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bde4); }
 let mut pc: u32 = 0x49bde4;
 loop { match pc {
 0x49bde4 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49bded); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049be47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49be47); }
 let mut pc: u32 = 0x49be47;
 loop { match pc {
 0x49be47 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49be4f); f_004a3950(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049be55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49be55); }
 let mut pc: u32 = 0x49be55;
 loop { match pc {
 0x49be55 => {
  { let v: u32 = 0x4bc220u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049be70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49be70); }
 let mut pc: u32 = 0x49be70;
 loop { match pc {
 0x49be70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49bf2fu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49be98); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49bed1; continue; }
  pc = 0x49beaf; continue;
 }
 0x49beaf => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49beb8); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49bec6; continue;
 }
 0x49bec6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bed8; continue;
 }
 0x49bed1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bed8; continue;
 }
 0x49bed8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49bef5); cpu.call(t);
  pc = 0x49bf07; continue;
 }
 0x49bf07 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.push(0x49bf12); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49bf2a; continue; }
  pc = 0x49bf1f; continue;
 }
 0x49bf1f => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.push(0x49bf2a); cpu.call(t);
  pc = 0x49bf2a; continue;
 }
 0x49bf2a => {
  pc = 0x49bf39; continue;
 }
 0x49bf39 => {
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
pub fn f_0049befa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49befa); }
 let mut pc: u32 = 0x49befa;
 loop { match pc {
 0x49befa => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49bf03); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bf2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bf2f); }
 let mut pc: u32 = 0x49bf2f;
 loop { match pc {
 0x49bf2f => {
  { let v: u32 = 0x4bc250u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bf49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bf49); }
 let mut pc: u32 = 0x49bf49;
 loop { match pc {
 0x49bf49 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49c1d2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x68u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49bf7a); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bf86; continue;
 }
 0x49bf86 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bfa2; continue;
 }
 0x49bfa2 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bfa9; continue;
 }
 0x49bfa9 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49bfb2); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49c066; continue; }
  pc = 0x49bfba; continue;
 }
 0x49bfba => {
  pc = 0x49bfbf; continue;
 }
 0x49bfbf => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if !cpu.zf { pc = 0x49c058; continue; }
  pc = 0x49bfce; continue;
 }
 0x49bfce => {
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49bff1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49c012; continue; }
  pc = 0x49c006; continue;
 }
 0x49c006 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c024; continue;
 }
 0x49c012 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c024; continue;
 }
 0x49c024 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49c033); cpu.call(t);
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x49c03c); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.push(0x49c047); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c053); f_0049c1c9(cpu);
  pc = 0x49c1dc; continue;
 }
 0x49c058 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49c061); cpu.call(t);
  pc = 0x49bfa9; continue;
 }
 0x49c066 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7cdcu32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49c076); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49c09d); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c0a5; continue;
 }
 0x49c0a5 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7d08u32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49c0b5); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c0cb); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c0d3; continue;
 }
 0x49c0d3 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c0e2); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c0eb); f_0049c1b5(cpu);
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49c10e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c116; continue;
 }
 0x49c116 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7d1cu32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49c126); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c13c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c144; continue;
 }
 0x49c144 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c153); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c15c); f_0049c1ab(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49c16b); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49c176); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49c17e); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c187); f_0049c1a1(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c190); f_0049c1bf(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c19c); f_0049c1c9(cpu);
  pc = 0x49c1dc; continue;
 }
 0x49c1dc => {
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
pub fn f_0049bf98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bf98); }
 let mut pc: u32 = 0x49bf98;
 loop { match pc {
 0x49bf98 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49bfa1); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c00f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c00f); }
 let mut pc: u32 = 0x49c00f;
 loop { match pc {
 0x49c00f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x408b0845u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x408b0845u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x458904c0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x458904c0u32.wrapping_add(cpu.ebx), v); }
  { cpu.lods(1, false); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49c033); cpu.call(t);
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x49c03c); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.push(0x49c047); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c053); f_0049c1c9(cpu);
  pc = 0x49c1dc; continue;
 }
 0x49c1dc => {
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
pub fn f_0049c031(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c031); }
 let mut pc: u32 = 0x49c031;
 loop { match pc {
 0x49c031 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x15ffe04du32.wrapping_add(cpu.ebp)) & (cpu.ecx as u8); cpu.w8(0x15ffe04du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8) | 0x57u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0x8b8c45u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0x8b8c45u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.push(0x49c047); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c053); f_0049c1c9(cpu);
  pc = 0x49c1dc; continue;
 }
 0x49c1dc => {
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
pub fn f_0049c079(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c079); }
 let mut pc: u32 = 0x49c079;
 loop { match pc {
 0x49c079 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x408b0845u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x558b084du32.wrapping_add(cpu.ebx)) | (cpu.ecx as u8); cpu.w8(0x558b084du32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x528b0452u32.wrapping_add(cpu.ebx)) | (cpu.ecx as u8); cpu.w8(0x528b0452u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x548b085du32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0x548b085du32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), cpu.r8(0xff04084cu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0xe9a845u32.wrapping_add(cpu.ecx)) | (cpu.ecx as u8); cpu.w8(0xe9a845u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), ((cpu.ecx >> 8) as u8)); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v = 0x4f7d08u32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49c0b5); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c0cb); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c0d3; continue;
 }
 0x49c0d3 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c0e2); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c0eb); f_0049c1b5(cpu);
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49c10e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c116; continue;
 }
 0x49c116 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7d1cu32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49c126); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c13c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c144; continue;
 }
 0x49c144 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c153); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c15c); f_0049c1ab(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49c16b); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49c176); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49c17e); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c187); f_0049c1a1(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c190); f_0049c1bf(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c19c); f_0049c1c9(cpu);
  pc = 0x49c1dc; continue;
 }
 0x49c1dc => {
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
pub fn f_0049c0c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c0c7); }
 let mut pc: u32 = 0x49c0c7;
 loop { match pc {
 0x49c0c7 => {
  { let v: u8 = cpu.sbb8(cpu.r8(0x52u32.wrapping_add(cpu.edi)), (cpu.edx as u8)); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xe99c45u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xe99c45u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), ((cpu.ecx >> 8) as u8)); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c0e2); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c0eb); f_0049c1b5(cpu);
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49c10e); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c116; continue;
 }
 0x49c116 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7d1cu32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49c126); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c13c); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c144; continue;
 }
 0x49c144 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49c153); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c15c); f_0049c1ab(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49c16b); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49c176); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49c17e); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c187); f_0049c1a1(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c190); f_0049c1bf(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c19c); f_0049c1c9(cpu);
  pc = 0x49c1dc; continue;
 }
 0x49c1dc => {
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
pub fn f_0049c1a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c1a1); }
 let mut pc: u32 = 0x49c1a1;
 loop { match pc {
 0x49c1a1 => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x49c1aa); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c1ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c1ab); }
 let mut pc: u32 = 0x49c1ab;
 loop { match pc {
 0x49c1ab => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49c1b4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c1b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c1b5); }
 let mut pc: u32 = 0x49c1b5;
 loop { match pc {
 0x49c1b5 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49c1be); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c1bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c1bf); }
 let mut pc: u32 = 0x49c1bf;
 loop { match pc {
 0x49c1bf => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49c1c8); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c1c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c1c9); }
 let mut pc: u32 = 0x49c1c9;
 loop { match pc {
 0x49c1c9 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49c1d1); f_004a3950(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c1d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c1d2); }
 let mut pc: u32 = 0x49c1d2;
 loop { match pc {
 0x49c1d2 => {
  { let v: u32 = 0x4bc278u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c1ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c1ec); }
 let mut pc: u32 = 0x49c1ec;
 loop { match pc {
 0x49c1ec => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49c2f3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49c21d); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c229; continue;
 }
 0x49c229 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c245; continue;
 }
 0x49c245 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c25a; continue;
 }
 0x49c251 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49c25a); cpu.call(t);
  pc = 0x49c25a; continue;
 }
 0x49c25a => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49c263); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49c2cf; continue; }
  pc = 0x49c26b; continue;
 }
 0x49c26b => {
  pc = 0x49c270; continue;
 }
 0x49c270 => {
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
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x49c2a0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49c2ca; continue; }
  pc = 0x49c2a8; continue;
 }
 0x49c2a8 => {
  pc = 0x49c2ad; continue;
 }
 0x49c2ad => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c2c2); f_0049c2ea(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49c2fd; continue;
 }
 0x49c2ca => {
  pc = 0x49c251; continue;
 }
 0x49c2cf => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c2e2); f_0049c2ea(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49c2fd; continue;
 }
 0x49c2fd => {
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
pub fn f_0049c23b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c23b); }
 let mut pc: u32 = 0x49c23b;
 loop { match pc {
 0x49c23b => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49c244); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c2ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c2ea); }
 let mut pc: u32 = 0x49c2ea;
 loop { match pc {
 0x49c2ea => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49c2f2); f_004a3950(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c2f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c2f3); }
 let mut pc: u32 = 0x49c2f3;
 loop { match pc {
 0x49c2f3 => {
  { let v: u32 = 0x4bc2c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c30e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c30e); }
 let mut pc: u32 = 0x49c30e;
 loop { match pc {
 0x49c30e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49c444u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49c345); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c351; continue;
 }
 0x49c351 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c36d; continue;
 }
 0x49c36d => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c382; continue;
 }
 0x49c379 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49c382); cpu.call(t);
  pc = 0x49c382; continue;
 }
 0x49c382 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49c38b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49c3c3; continue; }
  pc = 0x49c393; continue;
 }
 0x49c393 => {
  pc = 0x49c398; continue;
 }
 0x49c398 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x49c3b2); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49c3be; continue; }
  pc = 0x49c3ba; continue;
 }
 0x49c3ba => {
  { let v: u16 = cpu.inc16(cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.w16(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c3be; continue;
 }
 0x49c3be => {
  pc = 0x49c379; continue;
 }
 0x49c3c3 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x49c3ce); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c40f; continue; }
  pc = 0x49c3db; continue;
 }
 0x49c3db => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.div32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c401); f_0049c43b(cpu);
  { let v: u16 = cpu.r16(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x49c44e; continue;
 }
 0x49c40f => {
  { let v: u16 = 0x64u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c421); f_0049c43b(cpu);
  { let v: u16 = cpu.r16(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x49c44e; continue;
 }
 0x49c44e => {
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
pub fn f_0049c363(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c363); }
 let mut pc: u32 = 0x49c363;
 loop { match pc {
 0x49c363 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49c36c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c43b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c43b); }
 let mut pc: u32 = 0x49c43b;
 loop { match pc {
 0x49c43b => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49c443); f_004a3950(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c444(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c444); }
 let mut pc: u32 = 0x49c444;
 loop { match pc {
 0x49c444 => {
  { let v: u32 = 0x4bc2f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c45d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c45d); }
 let mut pc: u32 = 0x49c45d;
 loop { match pc {
 0x49c45d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49c593u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49c494); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4a0; continue;
 }
 0x49c4a0 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4bc; continue;
 }
 0x49c4bc => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4d1; continue;
 }
 0x49c4c8 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49c4d1); cpu.call(t);
  pc = 0x49c4d1; continue;
 }
 0x49c4d1 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49c4da); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49c512; continue; }
  pc = 0x49c4e2; continue;
 }
 0x49c4e2 => {
  pc = 0x49c4e7; continue;
 }
 0x49c4e7 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x49c501); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49c50d; continue; }
  pc = 0x49c509; continue;
 }
 0x49c509 => {
  { let v: u16 = cpu.inc16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c50d; continue;
 }
 0x49c50d => {
  pc = 0x49c4c8; continue;
 }
 0x49c512 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x49c51d); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c55e; continue; }
  pc = 0x49c52a; continue;
 }
 0x49c52a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.div32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c550); f_0049c58a(cpu);
  { let v: u16 = cpu.r16(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x49c59d; continue;
 }
 0x49c55e => {
  { let v: u16 = 0x64u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c570); f_0049c58a(cpu);
  { let v: u16 = cpu.r16(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x49c59d; continue;
 }
 0x49c59d => {
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
pub fn f_0049c4b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c4b2); }
 let mut pc: u32 = 0x49c4b2;
 loop { match pc {
 0x49c4b2 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49c4bb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c58a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c58a); }
 let mut pc: u32 = 0x49c58a;
 loop { match pc {
 0x49c58a => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49c592); f_004a3950(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c593(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c593); }
 let mut pc: u32 = 0x49c593;
 loop { match pc {
 0x49c593 => {
  { let v: u32 = 0x4bc328u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c5ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c5ac); }
 let mut pc: u32 = 0x49c5ac;
 loop { match pc {
 0x49c5ac => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49c5d9; continue;
 }
 0x49c5d9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c5e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c5e0); }
 let mut pc: u32 = 0x49c5e0;
 loop { match pc {
 0x49c5e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49c5f7; continue;
 }
 0x49c5f7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c5fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c5fc); }
 let mut pc: u32 = 0x49c5fc;
 loop { match pc {
 0x49c5fc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49c6cdu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49c62d); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c639; continue;
 }
 0x49c639 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c655; continue;
 }
 0x49c655 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c66a; continue;
 }
 0x49c661 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49c66a); cpu.call(t);
  pc = 0x49c66a; continue;
 }
 0x49c66a => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49c673); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49c69f; continue; }
  pc = 0x49c67b; continue;
 }
 0x49c67b => {
  pc = 0x49c680; continue;
 }
 0x49c680 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49c69a); cpu.call(t);
  pc = 0x49c661; continue;
 }
 0x49c69f => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c6bc); f_0049c6c4(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49c6d7; continue;
 }
 0x49c6d7 => {
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
pub fn f_0049c64b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c64b); }
 let mut pc: u32 = 0x49c64b;
 loop { match pc {
 0x49c64b => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49c654); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c6c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c6c4); }
 let mut pc: u32 = 0x49c6c4;
 loop { match pc {
 0x49c6c4 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49c6cc); f_004a3950(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c6cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c6cd); }
 let mut pc: u32 = 0x49c6cd;
 loop { match pc {
 0x49c6cd => {
  { let v: u32 = 0x4bc358u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c6e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c6e6); }
 let mut pc: u32 = 0x49c6e6;
 loop { match pc {
 0x49c6e6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49c7afu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49c717); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c723; continue;
 }
 0x49c723 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c73f; continue;
 }
 0x49c73f => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c754; continue;
 }
 0x49c74b => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49c754); cpu.call(t);
  pc = 0x49c754; continue;
 }
 0x49c754 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49c75d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49c78b; continue; }
  pc = 0x49c765; continue;
 }
 0x49c765 => {
  pc = 0x49c76a; continue;
 }
 0x49c76a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.push(0x49c786); cpu.call(t);
  pc = 0x49c74b; continue;
 }
 0x49c78b => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c7a1); f_0049c7a6(cpu);
  pc = 0x49c7b9; continue;
 }
 0x49c7b9 => {
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
pub fn f_0049c735(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c735); }
 let mut pc: u32 = 0x49c735;
 loop { match pc {
 0x49c735 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49c73e); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c7a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c7a6); }
 let mut pc: u32 = 0x49c7a6;
 loop { match pc {
 0x49c7a6 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49c7ae); f_004a3950(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c7af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c7af); }
 let mut pc: u32 = 0x49c7af;
 loop { match pc {
 0x49c7af => {
  { let v: u32 = 0x4bc388u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c7c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c7c7); }
 let mut pc: u32 = 0x49c7c7;
 loop { match pc {
 0x49c7c7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49c8fbu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c7f6; continue;
 }
 0x49c7f6 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c8ed; continue; }
  pc = 0x49c800; continue;
 }
 0x49c800 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c8ed; continue; }
  pc = 0x49c80a; continue;
 }
 0x49c80a => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49c828); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c834; continue;
 }
 0x49c834 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c850; continue;
 }
 0x49c850 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c865; continue;
 }
 0x49c85c => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49c865); cpu.call(t);
  pc = 0x49c865; continue;
 }
 0x49c865 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49c86e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49c8dc; continue; }
  pc = 0x49c876; continue;
 }
 0x49c876 => {
  pc = 0x49c87b; continue;
 }
 0x49c87b => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x49c895); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49c8d7; continue; }
  pc = 0x49c89d; continue;
 }
 0x49c89d => {
  pc = 0x49c8a2; continue;
 }
 0x49c8a2 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x49c8bc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49c8d0; continue; }
  pc = 0x49c8c4; continue;
 }
 0x49c8c4 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c8d7; continue;
 }
 0x49c8d0 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c8d7; continue;
 }
 0x49c8d7 => {
  pc = 0x49c85c; continue;
 }
 0x49c8dc => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49c8e8); f_0049c8f2(cpu);
  pc = 0x49c7f6; continue;
 }
 0x49c8ed => {
  pc = 0x49c905; continue;
 }
 0x49c905 => {
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
pub fn f_0049c846(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c846); }
 let mut pc: u32 = 0x49c846;
 loop { match pc {
 0x49c846 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49c84f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c8f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c8f2); }
 let mut pc: u32 = 0x49c8f2;
 loop { match pc {
 0x49c8f2 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49c8fa); f_004a3950(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c8fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c8fb); }
 let mut pc: u32 = 0x49c8fb;
 loop { match pc {
 0x49c8fb => {
  { let v: u32 = 0x4bc3b8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c913(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c913); }
 let mut pc: u32 = 0x49c913;
 loop { match pc {
 0x49c913 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c924; continue;
 }
 0x49c924 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049c92b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c92b); }
 let mut pc: u32 = 0x49c92b;
 loop { match pc {
 0x49c92b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49ca93u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c981; continue; }
  pc = 0x49c95d; continue;
 }
 0x49c95d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b34c8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x49c976; continue;
 }
 0x49c976 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c981; continue;
 }
 0x49c981 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49c98b); f_0049a526(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x49c99e); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1174u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x49c9b1; continue;
 }
 0x49c9b1 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c9d0; continue;
 }
 0x49c9d0 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x49c9f7; continue;
 }
 0x49c9f7 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b34ecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b34d4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v = 0x4f7d20u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x49ca40); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x49ca59; continue;
 }
 0x49ca59 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ca9d; continue;
 }
 0x49ca9d => {
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
pub fn f_0049c9c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49c9c3); }
 let mut pc: u32 = 0x49c9c3;
 loop { match pc {
 0x49c9c3 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x49c9cf); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ca68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ca68); }
 let mut pc: u32 = 0x49ca68;
 loop { match pc {
 0x49ca68 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x30u32); cpu.ecx = v; }
  cpu.push(0x49ca73); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ca74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ca74); }
 let mut pc: u32 = 0x49ca74;
 loop { match pc {
 0x49ca74 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49ca7c); f_004a3800(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ca7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ca7d); }
 let mut pc: u32 = 0x49ca7d;
 loop { match pc {
 0x49ca7d => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49ca92; continue; }
  pc = 0x49ca87; continue;
 }
 0x49ca87 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x49ca92); f_00485370(cpu);
  pc = 0x49ca92; continue;
 }
 0x49ca92 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ca93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ca93); }
 let mut pc: u32 = 0x49ca93;
 loop { match pc {
 0x49ca93 => {
  { let v: u32 = 0x4bc3e8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049caae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49caae); }
 let mut pc: u32 = 0x49caae;
 loop { match pc {
 0x49caae => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49cb8eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b34ecu32; cpu.w32(0xffffffacu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b34d4u32; cpu.w32(0xffffffb0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x49cb20); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49cb57; continue; }
  pc = 0x49cb2d; continue;
 }
 0x49cb2d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49cb57; continue; }
  pc = 0x49cb46; continue;
 }
 0x49cb46 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x49cb52); cpu.call(t);
  pc = 0x49cb57; continue;
 }
 0x49cb57 => {
  pc = 0x49cb5c; continue;
 }
 0x49cb5c => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49cb65); f_0049cb76(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49cb71); f_0049cb82(cpu);
  pc = 0x49cb98; continue;
 }
 0x49cb98 => {
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
pub fn f_0049cb76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cb76); }
 let mut pc: u32 = 0x49cb76;
 loop { match pc {
 0x49cb76 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  cpu.push(0x49cb81); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049cb82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cb82); }
 let mut pc: u32 = 0x49cb82;
 loop { match pc {
 0x49cb82 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  cpu.push(0x49cb8d); f_004a39b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049cb8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cb8e); }
 let mut pc: u32 = 0x49cb8e;
 loop { match pc {
 0x49cb8e => {
  { let v: u32 = 0x4bc428u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049cba6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cba6); }
 let mut pc: u32 = 0x49cba6;
 loop { match pc {
 0x49cba6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49ce2cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x50u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cbd0); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ce0a; continue;
 }
 0x49cbe7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cbf0); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cc05; continue;
 }
 0x49cc02 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cc05; continue;
 }
 0x49cc05 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x49cd34; continue; }
  pc = 0x49cc11; continue;
 }
 0x49cc11 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6a10u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cc28); f_004af4ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cc31); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49cc48; continue; }
  pc = 0x49cc41; continue;
 }
 0x49cc41 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49ce36; continue;
 }
 0x49cc48 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49cc5e; continue; }
  pc = 0x49cc58; continue;
 }
 0x49cc58 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49cc5e; continue;
 }
 0x49cc5e => {
  pc = 0x49cc63; continue;
 }
 0x49cc63 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49cc77); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ccf0; continue; }
  pc = 0x49cc8b; continue;
 }
 0x49cc8b => {
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49cca0); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49ccc2; continue; }
  pc = 0x49ccb8; continue;
 }
 0x49ccb8 => {
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)).wrapping_add(0x1u32); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49ccc2; continue;
 }
 0x49ccc2 => {
  pc = 0x49ccc7; continue;
 }
 0x49ccc7 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ccd0; continue;
 }
 0x49ccd0 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cceb; continue;
 }
 0x49cceb => {
  pc = 0x49ccf7; continue;
 }
 0x49ccf0 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ccf7; continue;
 }
 0x49ccf7 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49cd11); cpu.call(t);
  pc = 0x49cd23; continue;
 }
 0x49cd23 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49cd2f); f_0049ce23(cpu);
  pc = 0x49cc02; continue;
 }
 0x49cd34 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cd3d); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49cdd6; continue; }
  pc = 0x49cd50; continue;
 }
 0x49cd50 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6a98u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cd67); f_004af4ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cd70); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49cd87; continue; }
  pc = 0x49cd80; continue;
 }
 0x49cd80 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49ce36; continue;
 }
 0x49cd87 => {
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  cpu.push(0x49cd99); f_004828cd(cpu);
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49cdd6; continue; }
  pc = 0x49cdc5; continue;
 }
 0x49cdc5 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x49cdd1); cpu.call(t);
  pc = 0x49cdd6; continue;
 }
 0x49cdd6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  cpu.push(0x49cde5); f_0049a9c4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49cdf4; continue; }
  pc = 0x49cded; continue;
 }
 0x49cded => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49ce36; continue;
 }
 0x49cdf4 => {
  pc = 0x49ce19; continue;
 }
 0x49cdf9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49ce36; continue;
 }
 0x49ce0a => {
  { cpu.sub32(cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49cbe7; continue; }
  pc = 0x49ce14; continue;
 }
 0x49ce14 => {
  pc = 0x49cdf9; continue;
 }
 0x49ce19 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49ce36; continue;
 }
 0x49ce36 => {
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
pub fn f_0049ccdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ccdf); }
 let mut pc: u32 = 0x49ccdf;
 loop { match pc {
 0x49ccdf => {
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x49ccea); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049cd16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cd16); }
 let mut pc: u32 = 0x49cd16;
 loop { match pc {
 0x49cd16 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cd1f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ce23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ce23); }
 let mut pc: u32 = 0x49ce23;
 loop { match pc {
 0x49ce23 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49ce2b); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ce2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ce2c); }
 let mut pc: u32 = 0x49ce2c;
 loop { match pc {
 0x49ce2c => {
  { let v: u32 = 0x4bc458u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ce47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ce47); }
 let mut pc: u32 = 0x49ce47;
 loop { match pc {
 0x49ce47 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49cf99u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ce73); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x49ce82); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ce8c); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x24u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49ce9f); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ceab; continue;
 }
 0x49ceab => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cec7; continue;
 }
 0x49cec7 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cece; continue;
 }
 0x49cece => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49ced7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49cf27; continue; }
  pc = 0x49cedf; continue;
 }
 0x49cedf => {
  pc = 0x49cee4; continue;
 }
 0x49cee4 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cef2; continue;
 }
 0x49cef2 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x49cf19); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49cf22); cpu.call(t);
  pc = 0x49cece; continue;
 }
 0x49cf27 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49cf5c; continue; }
  pc = 0x49cf34; continue;
 }
 0x49cf34 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cf3f); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.push(0x49cf57); cpu.call(t);
  pc = 0x49cf6a; continue;
 }
 0x49cf5c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cf67); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x49cf6a; continue;
 }
 0x49cf6a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  cpu.push(0x49cf79); f_0049ac2e(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49cf88); f_0049cf90(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49cfa3; continue;
 }
 0x49cfa3 => {
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
pub fn f_0049cebd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cebd); }
 let mut pc: u32 = 0x49cebd;
 loop { match pc {
 0x49cebd => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49cec6); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049cf90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cf90); }
 let mut pc: u32 = 0x49cf90;
 loop { match pc {
 0x49cf90 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49cf98); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049cf99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cf99); }
 let mut pc: u32 = 0x49cf99;
 loop { match pc {
 0x49cf99 => {
  { let v: u32 = 0x4bc490u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049cfb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cfb4); }
 let mut pc: u32 = 0x49cfb4;
 loop { match pc {
 0x49cfb4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49d1c7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff4cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff50u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x524080u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0xd4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x49d00a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49d017; continue; }
  pc = 0x49d012; continue;
 }
 0x49d012 => {
  pc = 0x49d01c; continue;
 }
 0x49d017 => {
  pc = 0x49d0a0; continue;
 }
 0x49d01c => {
  pc = 0x49d02b; continue;
 }
 0x49d02b => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffff4cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49d09b; continue; }
  pc = 0x49d03b; continue;
 }
 0x49d03b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7d30u32; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49d04d); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49d05b); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49d066); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49d06e); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d077); f_0049d1aa(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d080); f_0049d1b4(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d093); f_0049d1be(cpu);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49d1d1; continue;
 }
 0x49d09b => {
  pc = 0x49d110; continue;
 }
 0x49d0a0 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffff4cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x49d110; continue; }
  pc = 0x49d0b0; continue;
 }
 0x49d0b0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7d74u32; cpu.push(v); }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49d0c2); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49d0d0); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49d0db); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49d0e3); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d0ec); f_0049d196(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d0f5); f_0049d1a0(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d108); f_0049d1be(cpu);
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49d1d1; continue;
 }
 0x49d110 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d135; continue; }
  pc = 0x49d11a; continue;
 }
 0x49d11a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0xacu32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x49d143; continue;
 }
 0x49d135 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0xacu32); cpu.ecx = v; }
  cpu.push(0x49d143); f_00482784(cpu);
  pc = 0x49d143; continue;
 }
 0x49d143 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0xacu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.edx)); cpu.push(0x49d17b); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d18e); f_0049d1be(cpu);
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49d1d1; continue;
 }
 0x49d1d1 => {
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
pub fn f_0049d196(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d196); }
 let mut pc: u32 = 0x49d196;
 loop { match pc {
 0x49d196 => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x49d19f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d1a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d1a0); }
 let mut pc: u32 = 0x49d1a0;
 loop { match pc {
 0x49d1a0 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49d1a9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d1aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d1aa); }
 let mut pc: u32 = 0x49d1aa;
 loop { match pc {
 0x49d1aa => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x49d1b3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d1b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d1b4); }
 let mut pc: u32 = 0x49d1b4;
 loop { match pc {
 0x49d1b4 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49d1bd); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d1be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d1be); }
 let mut pc: u32 = 0x49d1be;
 loop { match pc {
 0x49d1be => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49d1c6); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d1c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d1c7); }
 let mut pc: u32 = 0x49d1c7;
 loop { match pc {
 0x49d1c7 => {
  { let v: u32 = 0x4bc4c0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d1e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d1e2); }
 let mut pc: u32 = 0x49d1e2;
 loop { match pc {
 0x49d1e2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d1ee; continue;
 }
 0x49d1ee => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x49d1fa); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49d295; continue; }
  pc = 0x49d202; continue;
 }
 0x49d202 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffff38u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49d231); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d241; continue;
 }
 0x49d241 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49d263; continue; }
  pc = 0x49d257; continue;
 }
 0x49d257 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d275; continue;
 }
 0x49d263 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d275; continue;
 }
 0x49d275 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49d284); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1cu32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x49d290); cpu.call(t);
  pc = 0x49d1ee; continue;
 }
 0x49d295 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x98u32); cpu.ecx = v; }
  cpu.push(0x49d2a3); f_00499d78(cpu);
  pc = 0x49d2a8; continue;
 }
 0x49d2a8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d39a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d39a); }
 let mut pc: u32 = 0x49d39a;
 loop { match pc {
 0x49d39a => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49d3a3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d3fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d3fd); }
 let mut pc: u32 = 0x49d3fd;
 loop { match pc {
 0x49d3fd => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49d405); f_004a3a60(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d40b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d40b); }
 let mut pc: u32 = 0x49d40b;
 loop { match pc {
 0x49d40b => {
  { let v: u32 = 0x4bc508u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d426(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d426); }
 let mut pc: u32 = 0x49d426;
 loop { match pc {
 0x49d426 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49d4d9u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49d44e); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d487; continue; }
  pc = 0x49d465; continue;
 }
 0x49d465 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49d46e); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d47c; continue;
 }
 0x49d47c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d48e; continue;
 }
 0x49d487 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d48e; continue;
 }
 0x49d48e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2cu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49d4ab); cpu.call(t);
  pc = 0x49d4bd; continue;
 }
 0x49d4bd => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff50u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff50u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x49d4d4); cpu.call(t);
  pc = 0x49d4e3; continue;
 }
 0x49d4e3 => {
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
pub fn f_0049d4b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d4b0); }
 let mut pc: u32 = 0x49d4b0;
 loop { match pc {
 0x49d4b0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49d4b9); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d4d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d4d9); }
 let mut pc: u32 = 0x49d4d9;
 loop { match pc {
 0x49d4d9 => {
  { let v: u32 = 0x4bc538u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d4f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d4f3); }
 let mut pc: u32 = 0x49d4f3;
 loop { match pc {
 0x49d4f3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49d7a0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x68u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49d524); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d530; continue;
 }
 0x49d530 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d54c; continue;
 }
 0x49d54c => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d553; continue;
 }
 0x49d553 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49d55c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49d628; continue; }
  pc = 0x49d564; continue;
 }
 0x49d564 => {
  pc = 0x49d569; continue;
 }
 0x49d569 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if !cpu.zf { pc = 0x49d61a; continue; }
  pc = 0x49d578; continue;
 }
 0x49d578 => {
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49d5a7); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49d5c8; continue; }
  pc = 0x49d5bc; continue;
 }
 0x49d5bc => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d5da; continue;
 }
 0x49d5c8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d5da; continue;
 }
 0x49d5da => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49d5e9); cpu.call(t);
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x49d5f2); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff50u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff50u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x49d609); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d615); f_0049d797(cpu);
  pc = 0x49d7aa; continue;
 }
 0x49d61a => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49d623); cpu.call(t);
  pc = 0x49d553; continue;
 }
 0x49d628 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7db8u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49d638); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49d65f); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d667; continue;
 }
 0x49d667 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7ddcu32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49d677); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49d68d); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d695; continue;
 }
 0x49d695 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49d6a4); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d6ad); f_0049d783(cpu);
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49d6dc); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d6e4; continue;
 }
 0x49d6e4 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7df0u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49d6f4); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49d70a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d712; continue;
 }
 0x49d712 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49d721); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d72a); f_0049d779(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49d739); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49d744); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49d74c); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d755); f_0049d76f(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d75e); f_0049d78d(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49d76a); f_0049d797(cpu);
  pc = 0x49d7aa; continue;
 }
 0x49d7aa => {
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
pub fn f_0049d542(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d542); }
 let mut pc: u32 = 0x49d542;
 loop { match pc {
 0x49d542 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49d54b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d76f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d76f); }
 let mut pc: u32 = 0x49d76f;
 loop { match pc {
 0x49d76f => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x49d778); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d779(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d779); }
 let mut pc: u32 = 0x49d779;
 loop { match pc {
 0x49d779 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49d782); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d783(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d783); }
 let mut pc: u32 = 0x49d783;
 loop { match pc {
 0x49d783 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49d78c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d78d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d78d); }
 let mut pc: u32 = 0x49d78d;
 loop { match pc {
 0x49d78d => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49d796); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d797(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d797); }
 let mut pc: u32 = 0x49d797;
 loop { match pc {
 0x49d797 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49d79f); f_004a3a60(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d7a0); }
 let mut pc: u32 = 0x49d7a0;
 loop { match pc {
 0x49d7a0 => {
  { let v: u32 = 0x4bc560u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d879(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d879); }
 let mut pc: u32 = 0x49d879;
 loop { match pc {
 0x49d879 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49d882); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d906(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d906); }
 let mut pc: u32 = 0x49d906;
 loop { match pc {
 0x49d906 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49d90e); f_004a3a60(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d90f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d90f); }
 let mut pc: u32 = 0x49d90f;
 loop { match pc {
 0x49d90f => {
  { let v: u32 = 0x4bc5b0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d9ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d9ac); }
 let mut pc: u32 = 0x49d9ac;
 loop { match pc {
 0x49d9ac => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49d9b5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049da10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49da10); }
 let mut pc: u32 = 0x49da10;
 loop { match pc {
 0x49da10 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49da18); f_004a3a60(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049da19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49da19); }
 let mut pc: u32 = 0x49da19;
 loop { match pc {
 0x49da19 => {
  { let v: u32 = 0x4bc5e0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049da33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49da33); }
 let mut pc: u32 = 0x49da33;
 loop { match pc {
 0x49da33 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49daebu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49da64); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49da70; continue;
 }
 0x49da70 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49da8c; continue;
 }
 0x49da8c => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49da93; continue;
 }
 0x49da93 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49da9c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49dad1; continue; }
  pc = 0x49daa4; continue;
 }
 0x49daa4 => {
  pc = 0x49daa9; continue;
 }
 0x49daa9 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x49dac3); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49dacc); cpu.call(t);
  pc = 0x49da93; continue;
 }
 0x49dad1 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49dadd); f_0049dae2(cpu);
  pc = 0x49daf5; continue;
 }
 0x49daf5 => {
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
pub fn f_0049da82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49da82); }
 let mut pc: u32 = 0x49da82;
 loop { match pc {
 0x49da82 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49da8b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049dae2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49dae2); }
 let mut pc: u32 = 0x49dae2;
 loop { match pc {
 0x49dae2 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49daea); f_004a3a60(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049daeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49daeb); }
 let mut pc: u32 = 0x49daeb;
 loop { match pc {
 0x49daeb => {
  { let v: u32 = 0x4bc610u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
