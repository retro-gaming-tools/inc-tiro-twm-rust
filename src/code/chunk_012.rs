#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0049db5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49db5c); }
 let mut pc: u32 = 0x49db5c;
 loop { match pc {
 0x49db5c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x6cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x49db7d; continue;
 }
 0x49db7d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049dd0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49dd0f); }
 let mut pc: u32 = 0x49dd0f;
 loop { match pc {
 0x49dd0f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49e49du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e223; continue; }
  pc = 0x49dd41; continue;
 }
 0x49dd41 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3528u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3520u32; cpu.w32(0x78u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3510u32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3500u32; cpu.w32(0x8cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x6cu32.wrapping_add(cpu.eax), v); }
  pc = 0x49dd7d; continue;
 }
 0x49dd7d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ddb1; continue;
 }
 0x49ddb1 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x74u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x78u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49ddd4; continue;
 }
 0x49ddd4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ddff; continue;
 }
 0x49ddff => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dedf; continue;
 }
 0x49dedf => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x7cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0x80u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0x80u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49df22; continue;
 }
 0x49df22 => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49df69; continue;
 }
 0x49df69 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e156; continue;
 }
 0x49e156 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e1b6; continue;
 }
 0x49e1b6 => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e218; continue;
 }
 0x49e218 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e223; continue;
 }
 0x49e223 => {
  pc = 0x49e298; continue;
 }
 0x49e298 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49e2a2); f_004996be(cpu);
  { let v: u32 = 0x11u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e34u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e04u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2decu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e30a; continue;
 }
 0x49e30a => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e364; continue;
 }
 0x49e364 => {
  { let v: u32 = 0x12u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  cpu.push(0x49e378); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b35acu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b357cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3564u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3544u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x6cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x49e425); f_00482784(cpu);
  pc = 0x49e42a; continue;
 }
 0x49e42a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e4a7; continue;
 }
 0x49e4a7 => {
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
pub fn f_0049dde9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49dde9); }
 let mut pc: u32 = 0x49dde9;
 loop { match pc {
 0x49dde9 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x49ddfe; continue; }
  pc = 0x49ddf3; continue;
 }
 0x49ddf3 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x7cu32); cpu.ecx = v; }
  cpu.push(0x49ddfe); f_00489b10(cpu);
  pc = 0x49ddfe; continue;
 }
 0x49ddfe => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049debb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49debb); }
 let mut pc: u32 = 0x49debb;
 loop { match pc {
 0x49debb => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x49ded3; continue; }
  pc = 0x49dec5; continue;
 }
 0x49dec5 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x90u32); cpu.ecx = v; }
  cpu.push(0x49ded3); f_00489b10(cpu);
  pc = 0x49ded3; continue;
 }
 0x49ded3 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049df37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49df37); }
 let mut pc: u32 = 0x49df37;
 loop { match pc {
 0x49df37 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x49df4f; continue; }
  pc = 0x49df41; continue;
 }
 0x49df41 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x88u32); cpu.ecx = v; }
  cpu.push(0x49df4f); f_00490b40(cpu);
  pc = 0x49df4f; continue;
 }
 0x49df4f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049df50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49df50); }
 let mut pc: u32 = 0x49df50;
 loop { match pc {
 0x49df50 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x49df68; continue; }
  pc = 0x49df5a; continue;
 }
 0x49df5a => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x84u32); cpu.ecx = v; }
  cpu.push(0x49df68); f_00489b10(cpu);
  pc = 0x49df68; continue;
 }
 0x49df68 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e035(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e035); }
 let mut pc: u32 = 0x49e035;
 loop { match pc {
 0x49e035 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x49e04d; continue; }
  pc = 0x49e03f; continue;
 }
 0x49e03f => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x9cu32); cpu.ecx = v; }
  cpu.push(0x49e04d); f_00489b10(cpu);
  pc = 0x49e04d; continue;
 }
 0x49e04d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e047(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e047); }
 let mut pc: u32 = 0x49e047;
 loop { match pc {
 0x49e047 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e04f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e04f); }
 let mut pc: u32 = 0x49e04f;
 loop { match pc {
 0x49e04f => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e0b9; continue;
 }
 0x49e0b9 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x9cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0xa0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0xa0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e0ff; continue;
 }
 0x49e0ff => {
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e148; continue;
 }
 0x49e148 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x400u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e1b6; continue;
 }
 0x49e1b6 => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e218; continue;
 }
 0x49e218 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e298; continue;
 }
 0x49e298 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49e2a2); f_004996be(cpu);
  { let v: u32 = 0x11u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e34u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e04u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2decu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e30a; continue;
 }
 0x49e30a => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e364; continue;
 }
 0x49e364 => {
  { let v: u32 = 0x12u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  cpu.push(0x49e378); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b35acu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b357cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3564u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3544u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x6cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x49e425); f_00482784(cpu);
  pc = 0x49e42a; continue;
 }
 0x49e42a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e4a7; continue;
 }
 0x49e4a7 => {
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
pub fn f_0049e051(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e051); }
 let mut pc: u32 = 0x49e051;
 loop { match pc {
 0x49e051 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e0b9; continue;
 }
 0x49e0b9 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x9cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0xa0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0xa0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e0ff; continue;
 }
 0x49e0ff => {
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e148; continue;
 }
 0x49e148 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x400u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e1b6; continue;
 }
 0x49e1b6 => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e218; continue;
 }
 0x49e218 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e298; continue;
 }
 0x49e298 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49e2a2); f_004996be(cpu);
  { let v: u32 = 0x11u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e34u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e04u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2decu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e30a; continue;
 }
 0x49e30a => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e364; continue;
 }
 0x49e364 => {
  { let v: u32 = 0x12u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  cpu.push(0x49e378); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b35acu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b357cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3564u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3544u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x6cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x49e425); f_00482784(cpu);
  pc = 0x49e42a; continue;
 }
 0x49e42a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e4a7; continue;
 }
 0x49e4a7 => {
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
pub fn f_0049e07f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e07f); }
 let mut pc: u32 = 0x49e07f;
 loop { match pc {
 0x49e07f => {
  { let v: u8 = cpu.r8(0x1d540000u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) & 0x4bu8; cpu.w8(0x1d540000u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1000u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xau32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xa8u32); cpu.ecx = v; }
  cpu.push(0x49e0ab); f_00490a80(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x2000u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x9cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0xa0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0xa0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e0ff; continue;
 }
 0x49e0ff => {
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e148; continue;
 }
 0x49e148 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x400u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e1b6; continue;
 }
 0x49e1b6 => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e218; continue;
 }
 0x49e218 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e298; continue;
 }
 0x49e298 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49e2a2); f_004996be(cpu);
  { let v: u32 = 0x11u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e34u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e04u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2decu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e30a; continue;
 }
 0x49e30a => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e364; continue;
 }
 0x49e364 => {
  { let v: u32 = 0x12u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  cpu.push(0x49e378); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b35acu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b357cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3564u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3544u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x6cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x49e425); f_00482784(cpu);
  pc = 0x49e42a; continue;
 }
 0x49e42a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e4a7; continue;
 }
 0x49e4a7 => {
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
pub fn f_0049e0c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e0c7); }
 let mut pc: u32 = 0x49e0c7;
 loop { match pc {
 0x49e0c7 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.ror8(cpu.r8(0xa080u32.wrapping_add(cpu.ebx)), 1u32); cpu.w8(0xa080u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x4d8b0440u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x4d8b0440u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.rol8(((cpu.ebx >> 8) as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.ecx as u8)); }
  { let v: u8 = cpu.r8(0x1c000000u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) & cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0xa0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e0ff; continue;
 }
 0x49e0ff => {
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e148; continue;
 }
 0x49e148 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x400u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e1b6; continue;
 }
 0x49e1b6 => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e218; continue;
 }
 0x49e218 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e298; continue;
 }
 0x49e298 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49e2a2); f_004996be(cpu);
  { let v: u32 = 0x11u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e34u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e04u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2decu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e30a; continue;
 }
 0x49e30a => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e364; continue;
 }
 0x49e364 => {
  { let v: u32 = 0x12u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  cpu.push(0x49e378); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b35acu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b357cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3564u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3544u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x6cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x49e425); f_00482784(cpu);
  pc = 0x49e42a; continue;
 }
 0x49e42a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e4a7; continue;
 }
 0x49e4a7 => {
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
pub fn f_0049e0f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e0f7); }
 let mut pc: u32 = 0x49e0f7;
 loop { match pc {
 0x49e0f7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) & (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x49e0ff; continue;
 }
 0x49e0ff => {
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e148; continue;
 }
 0x49e148 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x400u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0x8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e1b6; continue;
 }
 0x49e1b6 => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e218; continue;
 }
 0x49e218 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e298; continue;
 }
 0x49e298 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49e2a2); f_004996be(cpu);
  { let v: u32 = 0x11u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e34u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2e04u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2decu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e30a; continue;
 }
 0x49e30a => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e364; continue;
 }
 0x49e364 => {
  { let v: u32 = 0x12u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x28u32); cpu.ecx = v; }
  cpu.push(0x49e378); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b35acu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b357cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3564u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3544u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x6cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x49e425); f_00482784(cpu);
  pc = 0x49e42a; continue;
 }
 0x49e42a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e4a7; continue;
 }
 0x49e4a7 => {
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
pub fn f_0049e116(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e116); }
 let mut pc: u32 = 0x49e116;
 loop { match pc {
 0x49e116 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x49e12e; continue; }
  pc = 0x49e120; continue;
 }
 0x49e120 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xa8u32); cpu.ecx = v; }
  cpu.push(0x49e12e); f_00490b40(cpu);
  pc = 0x49e12e; continue;
 }
 0x49e12e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e12f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e12f); }
 let mut pc: u32 = 0x49e12f;
 loop { match pc {
 0x49e12f => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x49e147; continue; }
  pc = 0x49e139; continue;
 }
 0x49e139 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xa4u32); cpu.ecx = v; }
  cpu.push(0x49e147); f_00489b10(cpu);
  pc = 0x49e147; continue;
 }
 0x49e147 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e1cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e1cd); }
 let mut pc: u32 = 0x49e1cd;
 loop { match pc {
 0x49e1cd => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x49e1e5; continue; }
  pc = 0x49e1d7; continue;
 }
 0x49e1d7 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x9cu32); cpu.ecx = v; }
  cpu.push(0x49e1e5); f_004913d0(cpu);
  pc = 0x49e1e5; continue;
 }
 0x49e1e5 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e1e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e1e6); }
 let mut pc: u32 = 0x49e1e6;
 loop { match pc {
 0x49e1e6 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x49e1fe; continue; }
  pc = 0x49e1f0; continue;
 }
 0x49e1f0 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x94u32); cpu.ecx = v; }
  cpu.push(0x49e1fe); f_00490b40(cpu);
  pc = 0x49e1fe; continue;
 }
 0x49e1fe => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e1ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e1ff); }
 let mut pc: u32 = 0x49e1ff;
 loop { match pc {
 0x49e1ff => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49e217; continue; }
  pc = 0x49e209; continue;
 }
 0x49e209 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x90u32); cpu.ecx = v; }
  cpu.push(0x49e217); f_00489b10(cpu);
  pc = 0x49e217; continue;
 }
 0x49e217 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e319(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e319); }
 let mut pc: u32 = 0x49e319;
 loop { match pc {
 0x49e319 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49e321); f_00490ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e322(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e322); }
 let mut pc: u32 = 0x49e322;
 loop { match pc {
 0x49e322 => {
  { cpu.and8(cpu.r8(0xfffffff2u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49e337; continue; }
  pc = 0x49e32c; continue;
 }
 0x49e32c => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x38u32); cpu.ecx = v; }
  cpu.push(0x49e337); f_004913d0(cpu);
  pc = 0x49e337; continue;
 }
 0x49e337 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e338); }
 let mut pc: u32 = 0x49e338;
 loop { match pc {
 0x49e338 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x49e34d; continue; }
  pc = 0x49e342; continue;
 }
 0x49e342 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x30u32); cpu.ecx = v; }
  cpu.push(0x49e34d); f_00490b40(cpu);
  pc = 0x49e34d; continue;
 }
 0x49e34d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e34e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e34e); }
 let mut pc: u32 = 0x49e34e;
 loop { match pc {
 0x49e34e => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x49e363; continue; }
  pc = 0x49e358; continue;
 }
 0x49e358 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x28u32); cpu.ecx = v; }
  cpu.push(0x49e363); f_00489b10(cpu);
  pc = 0x49e363; continue;
 }
 0x49e363 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e439(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e439); }
 let mut pc: u32 = 0x49e439;
 loop { match pc {
 0x49e439 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49e441); f_00493170(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e442(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e442); }
 let mut pc: u32 = 0x49e442;
 loop { match pc {
 0x49e442 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x49e45a; continue; }
  pc = 0x49e44c; continue;
 }
 0x49e44c => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x88u32); cpu.ecx = v; }
  cpu.push(0x49e45a); f_00491400(cpu);
  pc = 0x49e45a; continue;
 }
 0x49e45a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e45b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e45b); }
 let mut pc: u32 = 0x49e45b;
 loop { match pc {
 0x49e45b => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x49e470; continue; }
  pc = 0x49e465; continue;
 }
 0x49e465 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x7cu32); cpu.ecx = v; }
  cpu.push(0x49e470); f_004913d0(cpu);
  pc = 0x49e470; continue;
 }
 0x49e470 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e471(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e471); }
 let mut pc: u32 = 0x49e471;
 loop { match pc {
 0x49e471 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x49e486; continue; }
  pc = 0x49e47b; continue;
 }
 0x49e47b => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x74u32); cpu.ecx = v; }
  cpu.push(0x49e486); f_00490b40(cpu);
  pc = 0x49e486; continue;
 }
 0x49e486 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e487(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e487); }
 let mut pc: u32 = 0x49e487;
 loop { match pc {
 0x49e487 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49e49c; continue; }
  pc = 0x49e491; continue;
 }
 0x49e491 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x6cu32); cpu.ecx = v; }
  cpu.push(0x49e49c); f_00489b10(cpu);
  pc = 0x49e49c; continue;
 }
 0x49e49c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e49d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e49d); }
 let mut pc: u32 = 0x49e49d;
 loop { match pc {
 0x49e49d => {
  { let v: u32 = 0x4bc640u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e4e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e4e3); }
 let mut pc: u32 = 0x49e4e3;
 loop { match pc {
 0x49e4e3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x60u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x49e504; continue;
 }
 0x49e504 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e50b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e50b); }
 let mut pc: u32 = 0x49e50b;
 loop { match pc {
 0x49e50b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49ed11u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e96d; continue; }
  pc = 0x49e53d; continue;
 }
 0x49e53d => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b36e8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b36d0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b36c8u32; cpu.w32(0xacu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b36b8u32; cpu.w32(0xb4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b36a8u32; cpu.w32(0xc0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0xa0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49e58a; continue;
 }
 0x49e58a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e5c4; continue;
 }
 0x49e5c4 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0xa8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0xacu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e5f0; continue;
 }
 0x49e5f0 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e620; continue;
 }
 0x49e620 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e700; continue;
 }
 0x49e700 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0xb0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xb4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0xb4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xb4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0xb4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e746; continue;
 }
 0x49e746 => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xb0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e78f; continue;
 }
 0x49e78f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e8a0; continue;
 }
 0x49e8a0 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0xbcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0xc0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0xc0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0xc0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e900; continue;
 }
 0x49e900 => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xbcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e962; continue;
 }
 0x49e962 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e96d; continue;
 }
 0x49e96d => {
  pc = 0x49ea08; continue;
 }
 0x49ea08 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3684u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b366cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b363cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3624u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3604u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49ea66; continue;
 }
 0x49ea66 => {
  { let v: u32 = 0xbu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ead0; continue;
 }
 0x49ead0 => {
  { let v: u32 = 0x10u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  cpu.push(0x49eae4); f_0049dd0f(cpu);
  { let v: u8 = 0x11u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1edcu32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49eaf7; continue;
 }
 0x49eaf7 => {
  { let v: u8 = 0x12u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b37ccu32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49eb0a; continue;
 }
 0x49eb0a => {
  { let v: u8 = 0x11u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49eb28; continue;
 }
 0x49eb28 => {
  { let v: u8 = 0x13u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x7cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  pc = 0x49eb55; continue;
 }
 0x49eb55 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x88u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x49eb64); cpu.call(t);
  { let v: u8 = 0x14u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2e5cu32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  pc = 0x49eb7a; continue;
 }
 0x49eb7a => {
  { let v: u8 = 0x13u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49eb9e; continue;
 }
 0x49eb9e => {
  { let v: u8 = 0x15u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b37a4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3784u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b376cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b373cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3724u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b3704u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0xb8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x98u32.wrapping_add(cpu.eax), v); }
  pc = 0x49ec6e; continue;
 }
 0x49ec6e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ed1b; continue;
 }
 0x49ed1b => {
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
pub fn f_0049e607(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e607); }
 let mut pc: u32 = 0x49e607;
 loop { match pc {
 0x49e607 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x49e61f; continue; }
  pc = 0x49e611; continue;
 }
 0x49e611 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb0u32); cpu.ecx = v; }
  cpu.push(0x49e61f); f_00489b10(cpu);
  pc = 0x49e61f; continue;
 }
 0x49e61f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e6dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e6dc); }
 let mut pc: u32 = 0x49e6dc;
 loop { match pc {
 0x49e6dc => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x49e6f4; continue; }
  pc = 0x49e6e6; continue;
 }
 0x49e6e6 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xc4u32); cpu.ecx = v; }
  cpu.push(0x49e6f4); f_00489b10(cpu);
  pc = 0x49e6f4; continue;
 }
 0x49e6f4 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e75d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e75d); }
 let mut pc: u32 = 0x49e75d;
 loop { match pc {
 0x49e75d => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x49e775; continue; }
  pc = 0x49e767; continue;
 }
 0x49e767 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xbcu32); cpu.ecx = v; }
  cpu.push(0x49e775); f_00490b40(cpu);
  pc = 0x49e775; continue;
 }
 0x49e775 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e776(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e776); }
 let mut pc: u32 = 0x49e776;
 loop { match pc {
 0x49e776 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x49e78e; continue; }
  pc = 0x49e780; continue;
 }
 0x49e780 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb8u32); cpu.ecx = v; }
  cpu.push(0x49e78e); f_00489b10(cpu);
  pc = 0x49e78e; continue;
 }
 0x49e78e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e85b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e85b); }
 let mut pc: u32 = 0x49e85b;
 loop { match pc {
 0x49e85b => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x49e873; continue; }
  pc = 0x49e865; continue;
 }
 0x49e865 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xd0u32); cpu.ecx = v; }
  cpu.push(0x49e873); f_00489b10(cpu);
  pc = 0x49e873; continue;
 }
 0x49e873 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e917(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e917); }
 let mut pc: u32 = 0x49e917;
 loop { match pc {
 0x49e917 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x49e92f; continue; }
  pc = 0x49e921; continue;
 }
 0x49e921 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xd0u32); cpu.ecx = v; }
  cpu.push(0x49e92f); f_004913d0(cpu);
  pc = 0x49e92f; continue;
 }
 0x49e92f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e930); }
 let mut pc: u32 = 0x49e930;
 loop { match pc {
 0x49e930 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x49e948; continue; }
  pc = 0x49e93a; continue;
 }
 0x49e93a => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xc8u32); cpu.ecx = v; }
  cpu.push(0x49e948); f_00490b40(cpu);
  pc = 0x49e948; continue;
 }
 0x49e948 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e949(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e949); }
 let mut pc: u32 = 0x49e949;
 loop { match pc {
 0x49e949 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49e961; continue; }
  pc = 0x49e953; continue;
 }
 0x49e953 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xc4u32); cpu.ecx = v; }
  cpu.push(0x49e961); f_00489b10(cpu);
  pc = 0x49e961; continue;
 }
 0x49e961 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ea78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ea78); }
 let mut pc: u32 = 0x49ea78;
 loop { match pc {
 0x49ea78 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x49ea8d; continue; }
  pc = 0x49ea82; continue;
 }
 0x49ea82 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1cu32); cpu.ecx = v; }
  cpu.push(0x49ea8d); f_00491400(cpu);
  pc = 0x49ea8d; continue;
 }
 0x49ea8d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ea8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ea8e); }
 let mut pc: u32 = 0x49ea8e;
 loop { match pc {
 0x49ea8e => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x49eaa3; continue; }
  pc = 0x49ea98; continue;
 }
 0x49ea98 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x49eaa3); f_004913d0(cpu);
  pc = 0x49eaa3; continue;
 }
 0x49eaa3 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049eaa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49eaa4); }
 let mut pc: u32 = 0x49eaa4;
 loop { match pc {
 0x49eaa4 => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x49eab9; continue; }
  pc = 0x49eaae; continue;
 }
 0x49eaae => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  cpu.push(0x49eab9); f_00490b40(cpu);
  pc = 0x49eab9; continue;
 }
 0x49eab9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049eaba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49eaba); }
 let mut pc: u32 = 0x49eaba;
 loop { match pc {
 0x49eaba => {
  { cpu.and8(cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x49eacf; continue; }
  pc = 0x49eac4; continue;
 }
 0x49eac4 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x49eacf); f_00489b10(cpu);
  pc = 0x49eacf; continue;
 }
 0x49eacf => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049eb1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49eb1c); }
 let mut pc: u32 = 0x49eb1c;
 loop { match pc {
 0x49eb1c => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x49eb27); f_0048a280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049eb8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49eb8e); }
 let mut pc: u32 = 0x49eb8e;
 loop { match pc {
 0x49eb8e => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x88u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x49eb9d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ec7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ec7d); }
 let mut pc: u32 = 0x49ec7d;
 loop { match pc {
 0x49ec7d => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x88u32); cpu.ecx = v; }
  cpu.push(0x49ec8b); f_00492080(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ec8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ec8c); }
 let mut pc: u32 = 0x49ec8c;
 loop { match pc {
 0x49ec8c => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x49ec97); f_00493da0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ec98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ec98); }
 let mut pc: u32 = 0x49ec98;
 loop { match pc {
 0x49ec98 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  cpu.push(0x49eca3); f_00493480(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049eca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49eca4); }
 let mut pc: u32 = 0x49eca4;
 loop { match pc {
 0x49eca4 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49ecac); f_00493440(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ecad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ecad); }
 let mut pc: u32 = 0x49ecad;
 loop { match pc {
 0x49ecad => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x49ecc5; continue; }
  pc = 0x49ecb7; continue;
 }
 0x49ecb7 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xbcu32); cpu.ecx = v; }
  cpu.push(0x49ecc5); f_00491400(cpu);
  pc = 0x49ecc5; continue;
 }
 0x49ecc5 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ecc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ecc6); }
 let mut pc: u32 = 0x49ecc6;
 loop { match pc {
 0x49ecc6 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x49ecde; continue; }
  pc = 0x49ecd0; continue;
 }
 0x49ecd0 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb0u32); cpu.ecx = v; }
  cpu.push(0x49ecde); f_004913d0(cpu);
  pc = 0x49ecde; continue;
 }
 0x49ecde => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ecdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ecdf); }
 let mut pc: u32 = 0x49ecdf;
 loop { match pc {
 0x49ecdf => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x49ecf7; continue; }
  pc = 0x49ece9; continue;
 }
 0x49ece9 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xa8u32); cpu.ecx = v; }
  cpu.push(0x49ecf7); f_00490b40(cpu);
  pc = 0x49ecf7; continue;
 }
 0x49ecf7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ecf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ecf8); }
 let mut pc: u32 = 0x49ecf8;
 loop { match pc {
 0x49ecf8 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x49ed10; continue; }
  pc = 0x49ed02; continue;
 }
 0x49ed02 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xa0u32); cpu.ecx = v; }
  cpu.push(0x49ed10); f_00489b10(cpu);
  pc = 0x49ed10; continue;
 }
 0x49ed10 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ed11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ed11); }
 let mut pc: u32 = 0x49ed11;
 loop { match pc {
 0x49ed11 => {
  { let v: u32 = 0x4bc6f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ed2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ed2c); }
 let mut pc: u32 = 0x49ed2c;
 loop { match pc {
 0x49ed2c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49ee49u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x74u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x78u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x3f800000u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3f800000u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3f800000u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ed82; continue;
 }
 0x49ed82 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x7cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49eda0; continue;
 }
 0x49eda0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x40u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49ee25; continue; }
  pc = 0x49edb3; continue;
 }
 0x49edb3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49ee25; continue; }
  pc = 0x49edc3; continue;
 }
 0x49edc3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49eddf); f_0049fccf(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49ee25; continue; }
  pc = 0x49ede7; continue;
 }
 0x49ede7 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x49ee0a); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49ee1d); f_0049ee40(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ee53; continue;
 }
 0x49ee25 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49ee38); f_0049ee40(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ee53; continue;
 }
 0x49ee53 => {
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
pub fn f_0049ee40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ee40); }
 let mut pc: u32 = 0x49ee40;
 loop { match pc {
 0x49ee40 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49ee48); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ee49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ee49); }
 let mut pc: u32 = 0x49ee49;
 loop { match pc {
 0x49ee49 => {
  { let v: u32 = 0x4bc7c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049efa6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49efa6); }
 let mut pc: u32 = 0x49efa6;
 loop { match pc {
 0x49efa6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49f20du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49efe4; continue; }
  pc = 0x49efd8; continue;
 }
 0x49efd8 => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49efe4; continue;
 }
 0x49efe4 => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49f007); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f028; continue; }
  pc = 0x49f01c; continue;
 }
 0x49f01c => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f03a; continue;
 }
 0x49f028 => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f03a; continue;
 }
 0x49f03a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4c27e8u32; cpu.push(v); }
  { let v = 0x4c27c8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49f05e); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x49f06f); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f075); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f0a0; continue; }
  pc = 0x49f085; continue;
 }
 0x49f085 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f098); f_0049f204(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f217; continue;
 }
 0x49f0a0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49f0b6); f_00405cb0(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x49f0c8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49f150; continue; }
  pc = 0x49f0d0; continue;
 }
 0x49f0d0 => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49f0f3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f114; continue; }
  pc = 0x49f108; continue;
 }
 0x49f108 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f126; continue;
 }
 0x49f114 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f126; continue;
 }
 0x49f126 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49f135); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f148); f_0049f204(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f217; continue;
 }
 0x49f150 => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x49f157); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49f18d; continue; }
  pc = 0x49f16b; continue;
 }
 0x49f16b => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x49f174); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49f182; continue;
 }
 0x49f182 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f194; continue;
 }
 0x49f18d => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f194; continue;
 }
 0x49f194 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x88u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x49f1b1); cpu.call(t);
  pc = 0x49f1c3; continue;
 }
 0x49f1c3 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49f1d8; continue; }
  pc = 0x49f1cd; continue;
 }
 0x49f1cd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.push(0x49f1d8); cpu.call(t);
  pc = 0x49f1d8; continue;
 }
 0x49f1d8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f1ea); f_0049f204(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f217; continue;
 }
 0x49f217 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f1b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f1b6); }
 let mut pc: u32 = 0x49f1b6;
 loop { match pc {
 0x49f1b6 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f1bf); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f1f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f1f2); }
 let mut pc: u32 = 0x49f1f2;
 loop { match pc {
 0x49f1f2 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49f1fa); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f1fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f1fb); }
 let mut pc: u32 = 0x49f1fb;
 loop { match pc {
 0x49f1fb => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49f203); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f204(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f204); }
 let mut pc: u32 = 0x49f204;
 loop { match pc {
 0x49f204 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49f20c); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f20d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f20d); }
 let mut pc: u32 = 0x49f20d;
 loop { match pc {
 0x49f20d => {
  { let v: u32 = 0x4bc7f0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f228(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f228); }
 let mut pc: u32 = 0x49f228;
 loop { match pc {
 0x49f228 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49f4c0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49f25b); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f267; continue;
 }
 0x49f267 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f283; continue;
 }
 0x49f283 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f28a; continue;
 }
 0x49f28a => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49f293); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49f354; continue; }
  pc = 0x49f29b; continue;
 }
 0x49f29b => {
  pc = 0x49f2a0; continue;
 }
 0x49f2a0 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if !cpu.zf { pc = 0x49f346; continue; }
  pc = 0x49f2af; continue;
 }
 0x49f2af => {
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49f2d2); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f2e2; continue;
 }
 0x49f2e2 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f301; continue; }
  pc = 0x49f2f5; continue;
 }
 0x49f2f5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f313; continue;
 }
 0x49f301 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f313; continue;
 }
 0x49f313 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49f322); cpu.call(t);
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x49f32b); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x74u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f341); f_0049f4b7(cpu);
  pc = 0x49f4ca; continue;
 }
 0x49f346 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49f34f); cpu.call(t);
  pc = 0x49f28a; continue;
 }
 0x49f354 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7df4u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49f364); cpu.call(t);
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
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49f38b); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f393; continue;
 }
 0x49f393 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7e1cu32; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49f3a3); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49f3b9); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f3c1; continue;
 }
 0x49f3c1 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49f3d0); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f3d9); f_0049f4a3(cpu);
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x49f3fc); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f404; continue;
 }
 0x49f404 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7e34u32; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x49f414); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49f42a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f432; continue;
 }
 0x49f432 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x49f441); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f44a); f_0049f499(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x49f459); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x49f464); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x49f46c); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f475); f_0049f48f(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f47e); f_0049f4ad(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f48a); f_0049f4b7(cpu);
  pc = 0x49f4ca; continue;
 }
 0x49f4ca => {
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
pub fn f_0049f279(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f279); }
 let mut pc: u32 = 0x49f279;
 loop { match pc {
 0x49f279 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49f282); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f48f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f48f); }
 let mut pc: u32 = 0x49f48f;
 loop { match pc {
 0x49f48f => {
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x49f498); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f499(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f499); }
 let mut pc: u32 = 0x49f499;
 loop { match pc {
 0x49f499 => {
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49f4a2); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f4a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f4a3); }
 let mut pc: u32 = 0x49f4a3;
 loop { match pc {
 0x49f4a3 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49f4ac); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f4ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f4ad); }
 let mut pc: u32 = 0x49f4ad;
 loop { match pc {
 0x49f4ad => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x49f4b6); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f4b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f4b7); }
 let mut pc: u32 = 0x49f4b7;
 loop { match pc {
 0x49f4b7 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49f4bf); f_004a3ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f4c0); }
 let mut pc: u32 = 0x49f4c0;
 loop { match pc {
 0x49f4c0 => {
  { let v: u32 = 0x4bc830u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f4da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f4da); }
 let mut pc: u32 = 0x49f4da;
 loop { match pc {
 0x49f4da => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f4e6; continue;
 }
 0x49f4e6 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x88u32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x49f4f5); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49f58a; continue; }
  pc = 0x49f4fd; continue;
 }
 0x49f4fd => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49f520); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f530; continue;
 }
 0x49f530 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f555; continue; }
  pc = 0x49f549; continue;
 }
 0x49f549 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f567; continue;
 }
 0x49f555 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f567; continue;
 }
 0x49f567 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49f576); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x88u32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x49f585); cpu.call(t);
  pc = 0x49f4e6; continue;
 }
 0x49f58a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x74u32.wrapping_add(cpu.eax), v); }
  pc = 0x49f599; continue;
 }
 0x49f599 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f59e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f59e); }
 let mut pc: u32 = 0x49f59e;
 loop { match pc {
 0x49f59e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x88u32); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x49f5b9); cpu.call(t);
  pc = 0x49f5be; continue;
 }
 0x49f5be => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f5c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f5c3); }
 let mut pc: u32 = 0x49f5c3;
 loop { match pc {
 0x49f5c3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49f691u32; cpu.push(v); }
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
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49f602); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f60e; continue;
 }
 0x49f60e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f62a; continue;
 }
 0x49f62a => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f640; continue;
 }
 0x49f63d => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f640; continue;
 }
 0x49f640 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x49f65a; continue; }
  pc = 0x49f64c; continue;
 }
 0x49f64c => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49f655); cpu.call(t);
  pc = 0x49f63d; continue;
 }
 0x49f65a => {
  pc = 0x49f65f; continue;
 }
 0x49f65f => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f674); f_0049f683(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49f68c; continue;
 }
 0x49f68c => {
  pc = 0x49f69b; continue;
 }
 0x49f69b => {
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
pub fn f_0049f620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f620); }
 let mut pc: u32 = 0x49f620;
 loop { match pc {
 0x49f620 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49f629); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f683(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f683); }
 let mut pc: u32 = 0x49f683;
 loop { match pc {
 0x49f683 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49f68b); f_004a3ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f691(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f691); }
 let mut pc: u32 = 0x49f691;
 loop { match pc {
 0x49f691 => {
  { let v: u32 = 0x4bc880u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f6ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f6ac); }
 let mut pc: u32 = 0x49f6ac;
 loop { match pc {
 0x49f6ac => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49f7b0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49f6df); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f6eb; continue;
 }
 0x49f6eb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f707; continue;
 }
 0x49f707 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f70e; continue;
 }
 0x49f70e => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49f717); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49f78c; continue; }
  pc = 0x49f71f; continue;
 }
 0x49f71f => {
  pc = 0x49f724; continue;
 }
 0x49f724 => {
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
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x49f754); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49f77e; continue; }
  pc = 0x49f75c; continue;
 }
 0x49f75c => {
  pc = 0x49f761; continue;
 }
 0x49f761 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f776); f_0049f7a7(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f7ba; continue;
 }
 0x49f77e => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49f787); cpu.call(t);
  pc = 0x49f70e; continue;
 }
 0x49f78c => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f79f); f_0049f7a7(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f7ba; continue;
 }
 0x49f7ba => {
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
pub fn f_0049f6fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f6fd); }
 let mut pc: u32 = 0x49f6fd;
 loop { match pc {
 0x49f6fd => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49f706); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f7a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f7a7); }
 let mut pc: u32 = 0x49f7a7;
 loop { match pc {
 0x49f7a7 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49f7af); f_004a3ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f7b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f7b0); }
 let mut pc: u32 = 0x49f7b0;
 loop { match pc {
 0x49f7b0 => {
  { let v: u32 = 0x4bc8b0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f7cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f7cb); }
 let mut pc: u32 = 0x49f7cb;
 loop { match pc {
 0x49f7cb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.edx)); cpu.push(0x49f7f2); cpu.call(t);
  pc = 0x49f7f7; continue;
 }
 0x49f7f7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f7fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f7fe); }
 let mut pc: u32 = 0x49f7fe;
 loop { match pc {
 0x49f7fe => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49f8feu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x88u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49f831); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f83d; continue;
 }
 0x49f83d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f859; continue;
 }
 0x49f859 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f860; continue;
 }
 0x49f860 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49f869); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49f8e4; continue; }
  pc = 0x49f871; continue;
 }
 0x49f871 => {
  pc = 0x49f876; continue;
 }
 0x49f876 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.push(0x49f890); cpu.call(t);
  pc = 0x49f895; continue;
 }
 0x49f895 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x49f8af); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49f8d6; continue; }
  pc = 0x49f8b7; continue;
 }
 0x49f8b7 => {
  pc = 0x49f8bc; continue;
 }
 0x49f8bc => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.push(0x49f8d6); cpu.call(t);
  pc = 0x49f8d6; continue;
 }
 0x49f8d6 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49f8df); cpu.call(t);
  pc = 0x49f860; continue;
 }
 0x49f8e4 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49f8f0); f_0049f8f5(cpu);
  pc = 0x49f908; continue;
 }
 0x49f908 => {
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
pub fn f_0049f84f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f84f); }
 let mut pc: u32 = 0x49f84f;
 loop { match pc {
 0x49f84f => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49f858); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f8f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f8f5); }
 let mut pc: u32 = 0x49f8f5;
 loop { match pc {
 0x49f8f5 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49f8fd); f_004a3ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f8fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f8fe); }
 let mut pc: u32 = 0x49f8fe;
 loop { match pc {
 0x49f8fe => {
  { let v: u32 = 0x4bc8e0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f9c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f9c2); }
 let mut pc: u32 = 0x49f9c2;
 loop { match pc {
 0x49f9c2 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49f9cb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fa45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fa45); }
 let mut pc: u32 = 0x49fa45;
 loop { match pc {
 0x49fa45 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49fa4d); f_004a3ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fa4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fa4e); }
 let mut pc: u32 = 0x49fa4e;
 loop { match pc {
 0x49fa4e => {
  { let v: u32 = 0x4bc910u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fa68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fa68); }
 let mut pc: u32 = 0x49fa68;
 loop { match pc {
 0x49fa68 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.edx)); cpu.push(0x49fa85); cpu.call(t);
  pc = 0x49fa8a; continue;
 }
 0x49fa8a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fae5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fae5); }
 let mut pc: u32 = 0x49fae5;
 loop { match pc {
 0x49fae5 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49faee); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fb74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fb74); }
 let mut pc: u32 = 0x49fb74;
 loop { match pc {
 0x49fb74 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49fb7c); f_004a3ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fb7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fb7d); }
 let mut pc: u32 = 0x49fb7d;
 loop { match pc {
 0x49fb7d => {
  { let v: u32 = 0x4bc940u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fc17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fc17); }
 let mut pc: u32 = 0x49fc17;
 loop { match pc {
 0x49fc17 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fc28; continue;
 }
 0x49fc28 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fc2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fc2d); }
 let mut pc: u32 = 0x49fc2d;
 loop { match pc {
 0x49fc2d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x49fc44; continue;
 }
 0x49fc44 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fc49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fc49); }
 let mut pc: u32 = 0x49fc49;
 loop { match pc {
 0x49fc49 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49fc63; continue;
 }
 0x49fc63 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fc6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fc6a); }
 let mut pc: u32 = 0x49fc6a;
 loop { match pc {
 0x49fc6a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49fc97; continue;
 }
 0x49fc97 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fc9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fc9e); }
 let mut pc: u32 = 0x49fc9e;
 loop { match pc {
 0x49fc9e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x74u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49fcc8; continue;
 }
 0x49fcc8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049fccf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49fccf); }
 let mut pc: u32 = 0x49fccf;
 loop { match pc {
 0x49fccf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x49ffbdu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x58u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49fd09; continue; }
  pc = 0x49fd03; continue;
 }
 0x49fd03 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49fd09; continue;
 }
 0x49fd09 => {
  pc = 0x49fd0e; continue;
 }
 0x49fd0e => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49fd29; continue; }
  pc = 0x49fd1f; continue;
 }
 0x49fd1f => {
  pc = 0x49fd53; continue;
 }
 0x49fd29 => {
  pc = 0x49fd38; continue;
 }
 0x49fd38 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49fd4b); f_0049ffb4(cpu);
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ffc7; continue;
 }
 0x49fd53 => {
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49fd76); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49fd97; continue; }
  pc = 0x49fd8b; continue;
 }
 0x49fd8b => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fda9; continue;
 }
 0x49fd97 => {
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fda9; continue;
 }
 0x49fda9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f74f8u32; cpu.push(v); }
  { let v = 0x4c27c8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49fdcd); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.push(0x49fdde); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49fde4); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49fe0f; continue; }
  pc = 0x49fdf4; continue;
 }
 0x49fdf4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49fe07); f_0049ffb4(cpu);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ffc7; continue;
 }
 0x49fe0f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49fe2d); f_00405cb0(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x49fe3f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49fec7; continue; }
  pc = 0x49fe47; continue;
 }
 0x49fe47 => {
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x49fe6a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49fe8b; continue; }
  pc = 0x49fe7f; continue;
 }
 0x49fe7f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fe9d; continue;
 }
 0x49fe8b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fe9d; continue;
 }
 0x49fe9d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.push(0x49feac); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49febf); f_0049ffb4(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ffc7; continue;
 }
 0x49fec7 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x49fed6); cpu.call(t);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x49feec); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49fef5; continue;
 }
 0x49fef5 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ff0e; continue;
 }
 0x49ff0e => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ff12; continue;
 }
 0x49ff12 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x49ff1b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49ff75; continue; }
  pc = 0x49ff23; continue;
 }
 0x49ff23 => {
  pc = 0x49ff28; continue;
 }
 0x49ff28 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49ff3b); f_0049fccf(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49ff67; continue; }
  pc = 0x49ff43; continue;
 }
 0x49ff43 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49ff53); f_0049ff99(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49ff5f); f_0049ffb4(cpu);
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ffc7; continue;
 }
 0x49ff67 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x49ff70); cpu.call(t);
  pc = 0x49ff12; continue;
 }
 0x49ff75 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49ff85); f_0049ff99(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x49ff91); f_0049ffb4(cpu);
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ffc7; continue;
 }
 0x49ffc7 => {
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
pub fn f_0049ff04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ff04); }
 let mut pc: u32 = 0x49ff04;
 loop { match pc {
 0x49ff04 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x49ff0d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ff99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ff99); }
 let mut pc: u32 = 0x49ff99;
 loop { match pc {
 0x49ff99 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49ffa1); f_004854c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ffa2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ffa2); }
 let mut pc: u32 = 0x49ffa2;
 loop { match pc {
 0x49ffa2 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49ffaa); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ffab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ffab); }
 let mut pc: u32 = 0x49ffab;
 loop { match pc {
 0x49ffab => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x49ffb3); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ffb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ffb4); }
 let mut pc: u32 = 0x49ffb4;
 loop { match pc {
 0x49ffb4 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x49ffbc); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ffbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ffbd); }
 let mut pc: u32 = 0x49ffbd;
 loop { match pc {
 0x49ffbd => {
  { let v: u32 = 0x4bc970u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ffd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ffd8); }
 let mut pc: u32 = 0x49ffd8;
 loop { match pc {
 0x49ffd8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a010cu32; cpu.push(v); }
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
  if cpu.zf { pc = 0x4a002e; continue; }
  pc = 0x4a000a; continue;
 }
 0x4a000a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b37e8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0023; continue;
 }
 0x4a0023 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a002e; continue;
 }
 0x4a002e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a0038); f_0049a526(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a004b); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1174u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a005e; continue;
 }
 0x4a005e => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a007d; continue;
 }
 0x4a007d => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b380cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b37f4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a00d2; continue;
 }
 0x4a00d2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a0116; continue;
 }
 0x4a0116 => {
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
pub fn f_004a0031(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0031); }
 let mut pc: u32 = 0x4a0031;
 loop { match pc {
 0x4a0031 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  cpu.push(0x4a0037); cpu.call(0xffeeef1f);
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a004b); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1174u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a005e; continue;
 }
 0x4a005e => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a007d; continue;
 }
 0x4a007d => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b380cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b37f4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a00d2; continue;
 }
 0x4a00d2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a0116; continue;
 }
 0x4a0116 => {
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
pub fn f_004a0049(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0049); }
 let mut pc: u32 = 0x4a0049;
 loop { match pc {
 0x4a0049 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x40c7e845u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x4bu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a007d; continue;
 }
 0x4a007d => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b380cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b37f4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a00d2; continue;
 }
 0x4a00d2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a0116; continue;
 }
 0x4a0116 => {
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
pub fn f_004a0070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0070); }
 let mut pc: u32 = 0x4a0070;
 loop { match pc {
 0x4a0070 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x30u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a007c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a00e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a00e1); }
 let mut pc: u32 = 0x4a00e1;
 loop { match pc {
 0x4a00e1 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x30u32); cpu.ecx = v; }
  cpu.push(0x4a00ec); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a00ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a00ed); }
 let mut pc: u32 = 0x4a00ed;
 loop { match pc {
 0x4a00ed => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a00f5); f_004a3800(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a00f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a00f6); }
 let mut pc: u32 = 0x4a00f6;
 loop { match pc {
 0x4a00f6 => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a010b; continue; }
  pc = 0x4a0100; continue;
 }
 0x4a0100 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x48u32); cpu.ecx = v; }
  cpu.push(0x4a010b); f_00485370(cpu);
  pc = 0x4a010b; continue;
 }
 0x4a010b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a010c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a010c); }
 let mut pc: u32 = 0x4a010c;
 loop { match pc {
 0x4a010c => {
  { let v: u32 = 0x4bc9b8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0127(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0127); }
 let mut pc: u32 = 0x4a0127;
 loop { match pc {
 0x4a0127 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a0207u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b380cu32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b37f4u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x18u32); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4a0199); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a01d0; continue; }
  pc = 0x4a01a6; continue;
 }
 0x4a01a6 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a01d0; continue; }
  pc = 0x4a01bf; continue;
 }
 0x4a01bf => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a01cb); cpu.call(t);
  pc = 0x4a01d0; continue;
 }
 0x4a01d0 => {
  pc = 0x4a01d5; continue;
 }
 0x4a01d5 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a01de); f_004a01ef(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a01ea); f_004a01fb(cpu);
  pc = 0x4a0211; continue;
 }
 0x4a0211 => {
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
pub fn f_004a01ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a01ef); }
 let mut pc: u32 = 0x4a01ef;
 loop { match pc {
 0x4a01ef => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x18u32); cpu.ecx = v; }
  cpu.push(0x4a01fa); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a01fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a01fb); }
 let mut pc: u32 = 0x4a01fb;
 loop { match pc {
 0x4a01fb => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a0206); f_004a39b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0207(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0207); }
 let mut pc: u32 = 0x4a0207;
 loop { match pc {
 0x4a0207 => {
  { let v: u32 = 0x4bc9f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a021f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a021f); }
 let mut pc: u32 = 0x4a021f;
 loop { match pc {
 0x4a021f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a0467u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a0249); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a043a; continue;
 }
 0x4a0260 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a0269); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a027e; continue;
 }
 0x4a027b => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a027e; continue;
 }
 0x4a027e => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x4a03ad; continue; }
  pc = 0x4a028a; continue;
 }
 0x4a028a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6a10u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a02a1); f_004af4ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a02aa); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a02c1; continue; }
  pc = 0x4a02ba; continue;
 }
 0x4a02ba => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a0471; continue;
 }
 0x4a02c1 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a02d7; continue; }
  pc = 0x4a02d1; continue;
 }
 0x4a02d1 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a02d7; continue;
 }
 0x4a02d7 => {
  pc = 0x4a02dc; continue;
 }
 0x4a02dc => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a02f0); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a0369; continue; }
  pc = 0x4a0304; continue;
 }
 0x4a0304 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a0319); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a033b; continue; }
  pc = 0x4a0331; continue;
 }
 0x4a0331 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)).wrapping_add(0x1u32); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a033b; continue;
 }
 0x4a033b => {
  pc = 0x4a0340; continue;
 }
 0x4a0340 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0349; continue;
 }
 0x4a0349 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0364; continue;
 }
 0x4a0364 => {
  pc = 0x4a0370; continue;
 }
 0x4a0369 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0370; continue;
 }
 0x4a0370 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x18u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4a038a); cpu.call(t);
  pc = 0x4a039c; continue;
 }
 0x4a039c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a03a8); f_004a045e(cpu);
  pc = 0x4a027b; continue;
 }
 0x4a03ad => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a03b6); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a0406; continue; }
  pc = 0x4a03c9; continue;
 }
 0x4a03c9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6a98u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a03e0); f_004af4ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a03e9); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a0406; continue; }
  pc = 0x4a03ff; continue;
 }
 0x4a03ff => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a0471; continue;
 }
 0x4a0406 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  cpu.push(0x4a0415); f_0049a9c4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a0424; continue; }
  pc = 0x4a041d; continue;
 }
 0x4a041d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a0471; continue;
 }
 0x4a0424 => {
  pc = 0x4a0449; continue;
 }
 0x4a0429 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a0471; continue;
 }
 0x4a043a => {
  { cpu.sub32(cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a0260; continue; }
  pc = 0x4a0444; continue;
 }
 0x4a0444 => {
  pc = 0x4a0429; continue;
 }
 0x4a0449 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x48u32); cpu.ecx = v; }
  cpu.push(0x4a0454); f_004a05ef(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a0471; continue;
 }
 0x4a0471 => {
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
pub fn f_004a0358(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0358); }
 let mut pc: u32 = 0x4a0358;
 loop { match pc {
 0x4a0358 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a0363); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a038f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a038f); }
 let mut pc: u32 = 0x4a038f;
 loop { match pc {
 0x4a038f => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a0398); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a045e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a045e); }
 let mut pc: u32 = 0x4a045e;
 loop { match pc {
 0x4a045e => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a0466); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0467(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0467); }
 let mut pc: u32 = 0x4a0467;
 loop { match pc {
 0x4a0467 => {
  { let v: u32 = 0x4bca28u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0482(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0482); }
 let mut pc: u32 = 0x4a0482;
 loop { match pc {
 0x4a0482 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a05d4u32; cpu.push(v); }
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
  cpu.push(0x4a04ae); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x18u32); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4a04bd); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a04c7); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x18u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a04da); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a04e6; continue;
 }
 0x4a04e6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0502; continue;
 }
 0x4a0502 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0509; continue;
 }
 0x4a0509 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a0512); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a0562; continue; }
  pc = 0x4a051a; continue;
 }
 0x4a051a => {
  pc = 0x4a051f; continue;
 }
 0x4a051f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a052d; continue;
 }
 0x4a052d => {
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
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x4a0554); cpu.call(t);
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a055d); cpu.call(t);
  pc = 0x4a0509; continue;
 }
 0x4a0562 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a0597; continue; }
  pc = 0x4a056f; continue;
 }
 0x4a056f => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a057a); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.push(0x4a0592); cpu.call(t);
  pc = 0x4a05a5; continue;
 }
 0x4a0597 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a05a2); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4a05a5; continue;
 }
 0x4a05a5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a05b4); f_0049ac2e(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a05c3); f_004a05cb(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a05de; continue;
 }
 0x4a05de => {
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
pub fn f_004a04f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a04f8); }
 let mut pc: u32 = 0x4a04f8;
 loop { match pc {
 0x4a04f8 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a0501); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a05cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a05cb); }
 let mut pc: u32 = 0x4a05cb;
 loop { match pc {
 0x4a05cb => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a05d3); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a05d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a05d4); }
 let mut pc: u32 = 0x4a05d4;
 loop { match pc {
 0x4a05d4 => {
  { let v: u32 = 0x4bca60u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a05ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a05ef); }
 let mut pc: u32 = 0x4a05ef;
 loop { match pc {
 0x4a05ef => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a091fu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x7cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x40u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a062d; continue; }
  pc = 0x4a0623; continue;
 }
 0x4a0623 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a0929; continue;
 }
 0x4a062d => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a0650); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a065c; continue;
 }
 0x4a065c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0678; continue;
 }
 0x4a0678 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a067f; continue;
 }
 0x4a067f => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a0688); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a08cb; continue; }
  pc = 0x4a0690; continue;
 }
 0x4a0690 => {
  pc = 0x4a0695; continue;
 }
 0x4a0695 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x4a06a8); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a06bf); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a06c8; continue;
 }
 0x4a06c8 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a06e1; continue;
 }
 0x4a06e1 => {
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a06f3; continue;
 }
 0x4a06ea => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a06f3); cpu.call(t);
  pc = 0x4a06f3; continue;
 }
 0x4a06f3 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a06fc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a07f6; continue; }
  pc = 0x4a0704; continue;
 }
 0x4a0704 => {
  pc = 0x4a0709; continue;
 }
 0x4a0709 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0717; continue;
 }
 0x4a0717 => {
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x4a072c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a07f1; continue; }
  pc = 0x4a0734; continue;
 }
 0x4a0734 => {
  pc = 0x4a0739; continue;
 }
 0x4a0739 => {
  pc = 0x4a073e; continue;
 }
 0x4a073e => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a076a; continue; }
  pc = 0x4a0762; continue;
 }
 0x4a0762 => {
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a076a; continue;
 }
 0x4a076a => {
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a07da; continue; }
  pc = 0x4a0779; continue;
 }
 0x4a0779 => {
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4a07da; continue; }
  pc = 0x4a0796; continue;
 }
 0x4a0796 => {
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a07da; continue; }
  pc = 0x4a07b1; continue;
 }
 0x4a07b1 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x4a07d5); cpu.call(t);
  pc = 0x4a07da; continue;
 }
 0x4a07da => {
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a07ec; continue;
 }
 0x4a07ec => {
  pc = 0x4a07f6; continue;
 }
 0x4a07f1 => {
  pc = 0x4a06ea; continue;
 }
 0x4a07f6 => {
  pc = 0x4a07fb; continue;
 }
 0x4a07fb => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x54u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a0815; continue; }
  pc = 0x4a080b; continue;
 }
 0x4a080b => {
  pc = 0x4a08ab; continue;
 }
 0x4a0815 => {
  pc = 0x4a0824; continue;
 }
 0x4a0824 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7e38u32; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a0834); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x4a0847); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f7e7cu32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4a0859); cpu.call(t);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525718u32); cpu.push(0x4a0867); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a0870); f_004a08ef(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x4a087f); cpu.call(t);
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x4a088a); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x4a0892); cpu.call(t);
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a089b); f_004a08e5(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a08ab); f_004a08f9(cpu);
  pc = 0x4a08ab; continue;
 }
 0x4a08ab => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a08b4); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a08bd); f_004a0903(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a08c6); f_004a090c(cpu);
  pc = 0x4a067f; continue;
 }
 0x4a08cb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a08dd); f_004a0916(cpu);
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a0929; continue;
 }
 0x4a0929 => {
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
pub fn f_004a066e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a066e); }
 let mut pc: u32 = 0x4a066e;
 loop { match pc {
 0x4a066e => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a0677); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a06d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a06d7); }
 let mut pc: u32 = 0x4a06d7;
 loop { match pc {
 0x4a06d7 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a06e0); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a08e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a08e5); }
 let mut pc: u32 = 0x4a08e5;
 loop { match pc {
 0x4a08e5 => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x4a08ee); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a08ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a08ef); }
 let mut pc: u32 = 0x4a08ef;
 loop { match pc {
 0x4a08ef => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a08f8); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a08f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a08f9); }
 let mut pc: u32 = 0x4a08f9;
 loop { match pc {
 0x4a08f9 => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a0902); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0903(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0903); }
 let mut pc: u32 = 0x4a0903;
 loop { match pc {
 0x4a0903 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a090b); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a090c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a090c); }
 let mut pc: u32 = 0x4a090c;
 loop { match pc {
 0x4a090c => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a0915); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0916(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0916); }
 let mut pc: u32 = 0x4a0916;
 loop { match pc {
 0x4a0916 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a091e); f_004854c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a091f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a091f); }
 let mut pc: u32 = 0x4a091f;
 loop { match pc {
 0x4a091f => {
  { let v: u32 = 0x4bca90u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0938); }
 let mut pc: u32 = 0x4a0938;
 loop { match pc {
 0x4a0938 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a0984u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a0973); f_004a097b(cpu);
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a098e; continue;
 }
 0x4a098e => {
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
pub fn f_004a097b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a097b); }
 let mut pc: u32 = 0x4a097b;
 loop { match pc {
 0x4a097b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a0983); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0984(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0984); }
 let mut pc: u32 = 0x4a0984;
 loop { match pc {
 0x4a0984 => {
  { let v: u32 = 0x4bcaf0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a099f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a099f); }
 let mut pc: u32 = 0x4a099f;
 loop { match pc {
 0x4a099f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.push(0x4a09ce); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.push(0x4a09e2); cpu.call(t);
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x4a09f5; continue; }
  pc = 0x4a09eb; continue;
 }
 0x4a09eb => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a09f7; continue;
 }
 0x4a09f5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a09f7; continue;
 }
 0x4a09f7 => {
  pc = 0x4a09fc; continue;
 }
 0x4a09fc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0a01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0a01); }
 let mut pc: u32 = 0x4a0a01;
 loop { match pc {
 0x4a0a01 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a0c35u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a0a3f); f_00405cb0(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a0a4f); f_0049b952(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a0a72; continue; }
  pc = 0x4a0a57; continue;
 }
 0x4a0a57 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a0a6a); f_004a0c2c(cpu);
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a0c3f; continue;
 }
 0x4a0a72 => {
  pc = 0x4a0a77; continue;
 }
 0x4a0a77 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a0a93; continue; }
  pc = 0x4a0a8d; continue;
 }
 0x4a0a8d => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0a93; continue;
 }
 0x4a0a93 => {
  pc = 0x4a0a98; continue;
 }
 0x4a0a98 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a0ab6; continue; }
  pc = 0x4a0aae; continue;
 }
 0x4a0aae => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0ab6; continue;
 }
 0x4a0ab6 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x54u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a0b1e; continue; }
  pc = 0x4a0ac3; continue;
 }
 0x4a0ac3 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4a0b1e; continue; }
  pc = 0x4a0adc; continue;
 }
 0x4a0adc => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a0b1e; continue; }
  pc = 0x4a0af5; continue;
 }
 0x4a0af5 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x4a0b19); cpu.call(t);
  pc = 0x4a0b1e; continue;
 }
 0x4a0b1e => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a0b2e; continue;
 }
 0x4a0b2e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a0b37); f_004a0c1a(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x4a0b4c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a0b96; continue; }
  pc = 0x4a0b54; continue;
 }
 0x4a0b54 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f7628u32; cpu.push(v); }
  { let v = 0x4f7648u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a0b6f); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a0b75); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a0b96; continue; }
  pc = 0x4a0b85; continue;
 }
 0x4a0b85 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.push(0x4a0b90); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x60u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4a0b96; continue;
 }
 0x4a0b96 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x3f800000u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x68u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x74u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x78u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x7cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a0c12); f_004a0c2c(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a0c3f; continue;
 }
 0x4a0c3f => {
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
pub fn f_004a0c1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0c1a); }
 let mut pc: u32 = 0x4a0c1a;
 loop { match pc {
 0x4a0c1a => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a0c22); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0c23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0c23); }
 let mut pc: u32 = 0x4a0c23;
 loop { match pc {
 0x4a0c23 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a0c2b); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0c2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0c2c); }
 let mut pc: u32 = 0x4a0c2c;
 loop { match pc {
 0x4a0c2c => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a0c34); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0c35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0c35); }
 let mut pc: u32 = 0x4a0c35;
 loop { match pc {
 0x4a0c35 => {
  { let v: u32 = 0x4bcb18u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0c50); }
 let mut pc: u32 = 0x4a0c50;
 loop { match pc {
 0x4a0c50 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x4a0c67); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a0c86; continue; }
  pc = 0x4a0c6f; continue;
 }
 0x4a0c6f => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x4a0c7e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a0c8b; continue; }
  pc = 0x4a0c86; continue;
 }
 0x4a0c86 => {
  pc = 0x4a0d02; continue;
 }
 0x4a0c8b => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x64u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x58u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x58u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a0ca8); f_004a1b01(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x58u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x4a0cdb; continue; }
  pc = 0x4a0cc0; continue;
 }
 0x4a0cc0 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0ce8; continue;
 }
 0x4a0cdb => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0ce8; continue;
 }
 0x4a0ce8 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a0cf7); f_004a1b57(cpu);
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x5cu32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4a0d02; continue;
 }
 0x4a0d02 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0d86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0d86); }
 let mut pc: u32 = 0x4a0d86;
 loop { match pc {
 0x4a0d86 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0d97; continue;
 }
 0x4a0d97 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0d9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0d9c); }
 let mut pc: u32 = 0x4a0d9c;
 loop { match pc {
 0x4a0d9c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a0db3; continue;
 }
 0x4a0db3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0db8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0db8); }
 let mut pc: u32 = 0x4a0db8;
 loop { match pc {
 0x4a0db8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a0ddf); cpu.call(t);
  pc = 0x4a0de4; continue;
 }
 0x4a0de4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0de9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0de9); }
 let mut pc: u32 = 0x4a0de9;
 loop { match pc {
 0x4a0de9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a0e10); cpu.call(t);
  pc = 0x4a0e15; continue;
 }
 0x4a0e15 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0e1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0e1a); }
 let mut pc: u32 = 0x4a0e1a;
 loop { match pc {
 0x4a0e1a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a0e41); cpu.call(t);
  pc = 0x4a0e46; continue;
 }
 0x4a0e46 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0e4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0e4b); }
 let mut pc: u32 = 0x4a0e4b;
 loop { match pc {
 0x4a0e4b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4a0e62; continue;
 }
 0x4a0e62 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0e67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0e67); }
 let mut pc: u32 = 0x4a0e67;
 loop { match pc {
 0x4a0e67 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a0e7e; continue;
 }
 0x4a0e7e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0e83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0e83); }
 let mut pc: u32 = 0x4a0e83;
 loop { match pc {
 0x4a0e83 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4a0e9a; continue;
 }
 0x4a0e9a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0e9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0e9f); }
 let mut pc: u32 = 0x4a0e9f;
 loop { match pc {
 0x4a0e9f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4a0eb6; continue;
 }
 0x4a0eb6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0ebb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0ebb); }
 let mut pc: u32 = 0x4a0ebb;
 loop { match pc {
 0x4a0ebb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a0ee1); cpu.call(t);
  pc = 0x4a0ee6; continue;
 }
 0x4a0ee6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0eed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0eed); }
 let mut pc: u32 = 0x4a0eed;
 loop { match pc {
 0x4a0eed => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4a0f04; continue;
 }
 0x4a0f04 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0f09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0f09); }
 let mut pc: u32 = 0x4a0f09;
 loop { match pc {
 0x4a0f09 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b31c8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a0f31; continue; }
  pc = 0x4a0f2a; continue;
 }
 0x4a0f2a => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0f31; continue;
 }
 0x4a0f31 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a0f4f; continue; }
  pc = 0x4a0f46; continue;
 }
 0x4a0f46 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0f4f; continue;
 }
 0x4a0f4f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a0f69); cpu.call(t);
  pc = 0x4a0f6e; continue;
 }
 0x4a0f6e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0f75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0f75); }
 let mut pc: u32 = 0x4a0f75;
 loop { match pc {
 0x4a0f75 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4a0f8c; continue;
 }
 0x4a0f8c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0f91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0f91); }
 let mut pc: u32 = 0x4a0f91;
 loop { match pc {
 0x4a0f91 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b31c8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a0fb9; continue; }
  pc = 0x4a0fb2; continue;
 }
 0x4a0fb2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0fb9; continue;
 }
 0x4a0fb9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x4a0fd7; continue; }
  pc = 0x4a0fce; continue;
 }
 0x4a0fce => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a0fd7; continue;
 }
 0x4a0fd7 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a0ff1); cpu.call(t);
  pc = 0x4a0ff6; continue;
 }
 0x4a0ff6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0ffd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0ffd); }
 let mut pc: u32 = 0x4a0ffd;
 loop { match pc {
 0x4a0ffd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a1014; continue;
 }
 0x4a1014 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1019(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1019); }
 let mut pc: u32 = 0x4a1019;
 loop { match pc {
 0x4a1019 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a103f); cpu.call(t);
  pc = 0x4a1044; continue;
 }
 0x4a1044 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a104b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a104b); }
 let mut pc: u32 = 0x4a104b;
 loop { match pc {
 0x4a104b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a1062; continue;
 }
 0x4a1062 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1067(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1067); }
 let mut pc: u32 = 0x4a1067;
 loop { match pc {
 0x4a1067 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a108d); cpu.call(t);
  pc = 0x4a1092; continue;
 }
 0x4a1092 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1099(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1099); }
 let mut pc: u32 = 0x4a1099;
 loop { match pc {
 0x4a1099 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x4a10b0; continue;
 }
 0x4a10b0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a10b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a10b5); }
 let mut pc: u32 = 0x4a10b5;
 loop { match pc {
 0x4a10b5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.push(0x4a10db); cpu.call(t);
  pc = 0x4a10e0; continue;
 }
 0x4a10e0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a10e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a10e7); }
 let mut pc: u32 = 0x4a10e7;
 loop { match pc {
 0x4a10e7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a1205u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a110d; continue;
 }
 0x4a110d => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a1126); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1132; continue;
 }
 0x4a1132 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a114e; continue;
 }
 0x4a114e => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1155; continue;
 }
 0x4a1155 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a115e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a11e1; continue; }
  pc = 0x4a1166; continue;
 }
 0x4a1166 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x50u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x4a117e; continue; }
  pc = 0x4a1172; continue;
 }
 0x4a1172 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1195; continue;
 }
 0x4a117e => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x50u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1195; continue;
 }
 0x4a1195 => {
  pc = 0x4a119a; continue;
 }
 0x4a119a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a11b0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a11d3; continue; }
  pc = 0x4a11b8; continue;
 }
 0x4a11b8 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a11cb); f_004a11fc(cpu);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a120f; continue;
 }
 0x4a11d3 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a11dc); cpu.call(t);
  pc = 0x4a1155; continue;
 }
 0x4a11e1 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a11f4); f_004a11fc(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a120f; continue;
 }
 0x4a120f => {
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
pub fn f_004a1144(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1144); }
 let mut pc: u32 = 0x4a1144;
 loop { match pc {
 0x4a1144 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a114d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a11fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a11fc); }
 let mut pc: u32 = 0x4a11fc;
 loop { match pc {
 0x4a11fc => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a1204); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1205(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1205); }
 let mut pc: u32 = 0x4a1205;
 loop { match pc {
 0x4a1205 => {
  { let v: u32 = 0x4bcb50u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a139c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a139c); }
 let mut pc: u32 = 0x4a139c;
 loop { match pc {
 0x4a139c => {
  { let v: u32 = cpu.r32(0xffffff68u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a13aa); f_00483d4c(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a13e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a13e2); }
 let mut pc: u32 = 0x4a13e2;
 loop { match pc {
 0x4a13e2 => {
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a13ee); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1676(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1676); }
 let mut pc: u32 = 0x4a1676;
 loop { match pc {
 0x4a1676 => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a167f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a175a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a175a); }
 let mut pc: u32 = 0x4a175a;
 loop { match pc {
 0x4a175a => {
  { let v: u32 = cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a1769); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1791(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1791); }
 let mut pc: u32 = 0x4a1791;
 loop { match pc {
 0x4a1791 => {
  { let v: u32 = cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  cpu.push(0x4a179f); f_00485560(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a17a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a17a0); }
 let mut pc: u32 = 0x4a17a0;
 loop { match pc {
 0x4a17a0 => {
  { let v: u32 = cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a17ab); f_004851f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a18a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a18a1); }
 let mut pc: u32 = 0x4a18a1;
 loop { match pc {
 0x4a18a1 => {
  { let v: u32 = cpu.r32(0xffffff48u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a18ad); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1997(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1997); }
 let mut pc: u32 = 0x4a1997;
 loop { match pc {
 0x4a1997 => {
  { let v: u32 = cpu.r32(0xffffff50u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a19a5); f_00483d4c(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a19dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a19dd); }
 let mut pc: u32 = 0x4a19dd;
 loop { match pc {
 0x4a19dd => {
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a19e9); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1a7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1a7f); }
 let mut pc: u32 = 0x4a1a7f;
 loop { match pc {
 0x4a1a7f => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a1a87); f_00483d4c(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1a88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1a88); }
 let mut pc: u32 = 0x4a1a88;
 loop { match pc {
 0x4a1a88 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a1a91); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1a95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1a95); }
 let mut pc: u32 = 0x4a1a95;
 loop { match pc {
 0x4a1a95 => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a1a9d); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1a9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1a9e); }
 let mut pc: u32 = 0x4a1a9e;
 loop { match pc {
 0x4a1a9e => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a1aa6); f_00483d4c(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1aa7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1aa7); }
 let mut pc: u32 = 0x4a1aa7;
 loop { match pc {
 0x4a1aa7 => {
  { let v: u32 = 0x4bcb80u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1ac2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1ac2); }
 let mut pc: u32 = 0x4a1ac2;
 loop { match pc {
 0x4a1ac2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1ad3; continue;
 }
 0x4a1ad3 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x6cu32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.rf32(0x68u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4b31d0u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x4a1af5; continue; }
  pc = 0x4a1af3; continue;
 }
 0x4a1af3 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  pc = 0x4a1af5; continue;
 }
 0x4a1af5 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x4a1afc; continue;
 }
 0x4a1afc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1b01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1b01); }
 let mut pc: u32 = 0x4a1b01;
 loop { match pc {
 0x4a1b01 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a1b15); f_004a1ac2(cpu);
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = (cpu.r64(0xfffffff0u32.wrapping_add(cpu.ebp)) as i64 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a1b4a; continue; }
  pc = 0x4a1b3e; continue;
 }
 0x4a1b3e => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4b31d4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4a1b4a; continue;
 }
 0x4a1b4a => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  pc = 0x4a1b52; continue;
 }
 0x4a1b52 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1b57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1b57); }
 let mut pc: u32 = 0x4a1b57;
 loop { match pc {
 0x4a1b57 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a1b6b); f_004a1ac2(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x4a1b73; continue;
 }
 0x4a1b73 => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x4a1b8a); f_004aec50(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1b9b; continue;
 }
 0x4a1b9b => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4a1c0d; continue; }
  pc = 0x4a1ba8; continue;
 }
 0x4a1ba8 => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  cpu.push(0x4a1bb3); f_004ae7b6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if cpu.zf { pc = 0x4a1bd7; continue; }
  pc = 0x4a1bcb; continue;
 }
 0x4a1bcb => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1bde; continue;
 }
 0x4a1bd7 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1bde; continue;
 }
 0x4a1bde => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x4a1c08; continue; }
  pc = 0x4a1beb; continue;
 }
 0x4a1beb => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a1c01; continue; }
  pc = 0x4a1bf5; continue;
 }
 0x4a1bf5 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1c08; continue;
 }
 0x4a1c01 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1c08; continue;
 }
 0x4a1c08 => {
  pc = 0x4a1c60; continue;
 }
 0x4a1c0d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x74u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x4a1c26; continue; }
  pc = 0x4a1c1a; continue;
 }
 0x4a1c1a => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1c2d; continue;
 }
 0x4a1c26 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1c2d; continue;
 }
 0x4a1c2d => {
  { let v = cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b31c8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x4a1c60; continue; }
  pc = 0x4a1c42; continue;
 }
 0x4a1c42 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x58u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4b31c8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x4a1c60; continue; }
  pc = 0x4a1c5a; continue;
 }
 0x4a1c5a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1c60; continue;
 }
 0x4a1c60 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a1c7d; continue; }
  pc = 0x4a1c6a; continue;
 }
 0x4a1c6a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x68u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  pc = 0x4a1c8b; continue;
 }
 0x4a1c7d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x6cu32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.set_st(0, v); }
  pc = 0x4a1c8b; continue;
 }
 0x4a1c8b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1c92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1c92); }
 let mut pc: u32 = 0x4a1c92;
 loop { match pc {
 0x4a1c92 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a1dc3u32; cpu.push(v); }
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
  if cpu.zf { pc = 0x4a1ce8; continue; }
  pc = 0x4a1cc4; continue;
 }
 0x4a1cc4 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3830u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a1cdd; continue;
 }
 0x4a1cdd => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1ce8; continue;
 }
 0x4a1ce8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a1cf2); f_0049c92b(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4a1d05); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3824u32; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a1d18; continue;
 }
 0x4a1d18 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a1d37; continue;
 }
 0x4a1d37 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3854u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b383cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v = 0x4f7e80u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525700u32); cpu.push(0x4a1d84); cpu.call(t);
  pc = 0x4a1d89; continue;
 }
 0x4a1d89 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a1dcd; continue;
 }
 0x4a1dcd => {
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
pub fn f_004a1d2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1d2a); }
 let mut pc: u32 = 0x4a1d2a;
 loop { match pc {
 0x4a1d2a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4a1d36); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1d98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1d98); }
 let mut pc: u32 = 0x4a1d98;
 loop { match pc {
 0x4a1d98 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  cpu.push(0x4a1da3); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1da4); }
 let mut pc: u32 = 0x4a1da4;
 loop { match pc {
 0x4a1da4 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a1dac); f_004a3da0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1dad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1dad); }
 let mut pc: u32 = 0x4a1dad;
 loop { match pc {
 0x4a1dad => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a1dc2; continue; }
  pc = 0x4a1db7; continue;
 }
 0x4a1db7 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x4a1dc2); f_00485370(cpu);
  pc = 0x4a1dc2; continue;
 }
 0x4a1dc2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1dc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1dc3); }
 let mut pc: u32 = 0x4a1dc3;
 loop { match pc {
 0x4a1dc3 => {
  { let v: u32 = 0x4bcc08u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1f21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1f21); }
 let mut pc: u32 = 0x4a1f21;
 loop { match pc {
 0x4a1f21 => {
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a1f2d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1f46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1f46); }
 let mut pc: u32 = 0x4a1f46;
 loop { match pc {
 0x4a1f46 => {
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a1f51); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a1f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a1f80); }
 let mut pc: u32 = 0x4a1f80;
 loop { match pc {
 0x4a1f80 => {
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a1f89); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a202b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a202b); }
 let mut pc: u32 = 0x4a202b;
 loop { match pc {
 0x4a202b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x15ffe44du32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x15ffe44du32.wrapping_add(cpu.ebp), v); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a203f; continue;
 }
 0x4a203f => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a2048); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a20a8; continue; }
  pc = 0x4a2050; continue;
 }
 0x4a2050 => {
  pc = 0x4a2055; continue;
 }
 0x4a2055 => {
  pc = 0x4a205a; continue;
 }
 0x4a205a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f8210u32; cpu.push(v); }
  { let v = 0x4f6a10u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2074); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a209a; continue; }
  pc = 0x4a2084; continue;
 }
 0x4a2084 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x54u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x525794u32); cpu.push(0x4a209a); cpu.call(t);
  pc = 0x4a209a; continue;
 }
 0x4a209a => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a20a3); cpu.call(t);
  pc = 0x4a203f; continue;
 }
 0x4a20a8 => {
  { let v: u32 = 0x4b386cu32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a20b4; continue;
 }
 0x4a20b4 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b387cu32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a20c4; continue;
 }
 0x4a20c4 => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a20dc; continue;
 }
 0x4a20dc => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff74u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff74u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a2108); cpu.call(t);
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2111; continue;
 }
 0x4a2111 => {
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a212d; continue;
 }
 0x4a212d => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4a2153); cpu.call(t);
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a215c; continue;
 }
 0x4a215c => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a217e; continue;
 }
 0x4a217e => {
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525770u32); cpu.push(0x4a218e); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525774u32); cpu.push(0x4a21ab); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a21bb; continue; }
  pc = 0x4a21b6; continue;
 }
 0x4a21b6 => {
  pc = 0x4a23f1; continue;
 }
 0x4a21bb => {
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4a21c4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a21d1; continue; }
  pc = 0x4a21cc; continue;
 }
 0x4a21cc => {
  pc = 0x4a23f1; continue;
 }
 0x4a21d1 => {
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff48u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a21e0); f_004a5340(cpu);
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a21f3); f_004a5340(cpu);
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a2203); cpu.call(t);
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2212); f_004a5340(cpu);
  { let v: u8 = 0xfu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2225); f_004a5340(cpu);
  { let v: u8 = 0x10u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2229; continue;
 }
 0x4a2229 => {
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525784u32); cpu.push(0x4a223d); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a238c; continue; }
  pc = 0x4a2248; continue;
 }
 0x4a2248 => {
  pc = 0x4a224d; continue;
 }
 0x4a224d => {
  pc = 0x4a2252; continue;
 }
 0x4a2252 => {
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a2277); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a236f; continue; }
  pc = 0x4a227f; continue;
 }
 0x4a227f => {
  pc = 0x4a2284; continue;
 }
 0x4a2284 => {
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff30u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2299); f_004a4e10(cpu);
  { let v: u8 = 0x11u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff48u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52577cu32); cpu.push(0x4a22b0); cpu.call(t);
  pc = 0x4a22b5; continue;
 }
 0x4a22b5 => {
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a22c7); f_004a5260(cpu);
  pc = 0x4a22cc; continue;
 }
 0x4a22cc => {
  pc = 0x4a22d1; continue;
 }
 0x4a22d1 => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a22e9); f_004a5290(cpu);
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525780u32); cpu.push(0x4a22f5); cpu.call(t);
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525780u32); cpu.push(0x4a2301); cpu.call(t);
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525784u32); cpu.push(0x4a2312); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4a234c; continue; }
  pc = 0x4a231d; continue;
 }
 0x4a231d => {
  pc = 0x4a2322; continue;
 }
 0x4a2322 => {
  { let v: u32 = 0xffffff30u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x4a2344); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a22d1; continue; }
  pc = 0x4a234c; continue;
 }
 0x4a234c => {
  pc = 0x4a2351; continue;
 }
 0x4a2351 => {
  { let v: u32 = 0xffffff30u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x4a2366); f_004a5290(cpu);
  { let v: u8 = 0x10u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a236f); f_004a23b5(cpu);
  pc = 0x4a236f; continue;
 }
 0x4a236f => {
  { let v: u32 = 0xffffff48u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a237b); cpu.call(t);
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x4a2387); cpu.call(t);
  pc = 0x4a2229; continue;
 }
 0x4a238c => {
  { let v: u8 = 0xfu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2395); f_004a23c1(cpu);
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a239e); f_004a23cd(cpu);
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a23a7); f_004a23d9(cpu);
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a23b0); f_004a23e5(cpu);
  pc = 0x4a23f1; continue;
 }
 0x4a23f1 => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a23fa); f_004a2408(cpu);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2403); f_004a2414(cpu);
  pc = 0x4a241d; continue;
 }
 0x4a241d => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2426); f_004a2441(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2439); f_004a244a(cpu);
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a2470; continue;
 }
 0x4a2470 => {
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
pub fn f_004a202e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a202e); }
 let mut pc: u32 = 0x4a202e;
 loop { match pc {
 0x4a202e => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a2037); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a20d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a20d3); }
 let mut pc: u32 = 0x4a20d3;
 loop { match pc {
 0x4a20d3 => {
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a20db); f_004a3dd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2123(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2123); }
 let mut pc: u32 = 0x4a2123;
 loop { match pc {
 0x4a2123 => {
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a212c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2171(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2171); }
 let mut pc: u32 = 0x4a2171;
 loop { match pc {
 0x4a2171 => {
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a217d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a23b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a23b5); }
 let mut pc: u32 = 0x4a23b5;
 loop { match pc {
 0x4a23b5 => {
  { let v: u32 = 0xffffff30u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a23c0); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a23c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a23c1); }
 let mut pc: u32 = 0x4a23c1;
 loop { match pc {
 0x4a23c1 => {
  { let v: u32 = 0xffffff60u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a23cc); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a23cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a23cd); }
 let mut pc: u32 = 0x4a23cd;
 loop { match pc {
 0x4a23cd => {
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a23d8); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a23d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a23d9); }
 let mut pc: u32 = 0x4a23d9;
 loop { match pc {
 0x4a23d9 => {
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a23e4); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a23e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a23e5); }
 let mut pc: u32 = 0x4a23e5;
 loop { match pc {
 0x4a23e5 => {
  { let v: u32 = 0xffffff48u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a23f0); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2408(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2408); }
 let mut pc: u32 = 0x4a2408;
 loop { match pc {
 0x4a2408 => {
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2413); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2414(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2414); }
 let mut pc: u32 = 0x4a2414;
 loop { match pc {
 0x4a2414 => {
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a241c); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2441(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2441); }
 let mut pc: u32 = 0x4a2441;
 loop { match pc {
 0x4a2441 => {
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2449); f_004a3ea0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a244a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a244a); }
 let mut pc: u32 = 0x4a244a;
 loop { match pc {
 0x4a244a => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2452); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2453(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2453); }
 let mut pc: u32 = 0x4a2453;
 loop { match pc {
 0x4a2453 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a245b); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a245c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a245c); }
 let mut pc: u32 = 0x4a245c;
 loop { match pc {
 0x4a245c => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a2465); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2466(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2466); }
 let mut pc: u32 = 0x4a2466;
 loop { match pc {
 0x4a2466 => {
  { let v: u32 = 0x4bcc48u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a24f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a24f7); }
 let mut pc: u32 = 0x4a24f7;
 loop { match pc {
 0x4a24f7 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a2500); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a258c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a258c); }
 let mut pc: u32 = 0x4a258c;
 loop { match pc {
 0x4a258c => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2594); f_004a3d40(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2595(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2595); }
 let mut pc: u32 = 0x4a2595;
 loop { match pc {
 0x4a2595 => {
  { let v: u32 = 0x4bccf8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a25b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a25b0); }
 let mut pc: u32 = 0x4a25b0;
 loop { match pc {
 0x4a25b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a26bdu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b38a4u32; cpu.w32(0xffffff88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b388cu32; cpu.w32(0xffffff8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a261a; continue;
 }
 0x4a261a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4a2626); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a2671; continue; }
  pc = 0x4a262e; continue;
 }
 0x4a262e => {
  pc = 0x4a2633; continue;
 }
 0x4a2633 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a2660; continue; }
  pc = 0x4a264f; continue;
 }
 0x4a264f => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4a265b); cpu.call(t);
  pc = 0x4a2660; continue;
 }
 0x4a2660 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x4a266c); cpu.call(t);
  pc = 0x4a261a; continue;
 }
 0x4a2671 => {
  pc = 0x4a2676; continue;
 }
 0x4a2676 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a267f); f_004a2699(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2688); f_004a26a5(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2694); f_004a26b1(cpu);
  pc = 0x4a26c7; continue;
 }
 0x4a26c7 => {
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
pub fn f_004a2699(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2699); }
 let mut pc: u32 = 0x4a2699;
 loop { match pc {
 0x4a2699 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4a26a4); f_004a44b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a26a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a26a5); }
 let mut pc: u32 = 0x4a26a5;
 loop { match pc {
 0x4a26a5 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  cpu.push(0x4a26b0); f_004a4280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a26b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a26b1); }
 let mut pc: u32 = 0x4a26b1;
 loop { match pc {
 0x4a26b1 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  cpu.push(0x4a26bc); f_004a3f80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a26bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a26bd); }
 let mut pc: u32 = 0x4a26bd;
 loop { match pc {
 0x4a26bd => {
  { let v: u32 = 0x4bcd28u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a26d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a26d5); }
 let mut pc: u32 = 0x4a26d5;
 loop { match pc {
 0x4a26d5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a2efcu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xe0u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2705); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2e8e; continue;
 }
 0x4a271f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2728); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a273d; continue;
 }
 0x4a273a => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a273d; continue;
 }
 0x4a273d => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x4a281f; continue; }
  pc = 0x4a2749; continue;
 }
 0x4a2749 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f8238u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2760); f_004af4ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2769); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a2780; continue; }
  pc = 0x4a2779; continue;
 }
 0x4a2779 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a2780 => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a2787); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a27d2; continue; }
  pc = 0x4a27a4; continue;
 }
 0x4a27a4 => {
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a27b0); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a27c1; continue;
 }
 0x4a27c1 => {
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff38u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a27dc; continue;
 }
 0x4a27d2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff38u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a27dc; continue;
 }
 0x4a27dc => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4a2805); cpu.call(t);
  pc = 0x4a281a; continue;
 }
 0x4a281a => {
  pc = 0x4a273a; continue;
 }
 0x4a281f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2828); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a283d; continue;
 }
 0x4a283a => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a283d; continue;
 }
 0x4a283d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x4a29bd; continue; }
  pc = 0x4a2849; continue;
 }
 0x4a2849 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2852); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0xffffffd0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2865); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a287f); f_00483464(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x4a288a); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff74u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a295e; continue; }
  pc = 0x4a28a4; continue;
 }
 0x4a28a4 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff74u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a28c5); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x4a28fb); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2904; continue;
 }
 0x4a2904 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff68u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a292c; continue;
 }
 0x4a292c => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2935; continue;
 }
 0x4a2935 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2959; continue;
 }
 0x4a2959 => {
  pc = 0x4a2968; continue;
 }
 0x4a295e => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2968; continue;
 }
 0x4a2968 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4a298e); cpu.call(t);
  pc = 0x4a29a3; continue;
 }
 0x4a29a3 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a29ac); f_004a2ee9(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a29b8); f_004a2ef2(cpu);
  pc = 0x4a283a; continue;
 }
 0x4a29bd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  cpu.push(0x4a29cf); f_0049b4e7(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a29de; continue; }
  pc = 0x4a29d7; continue;
 }
 0x4a29d7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a29de => {
  pc = 0x4a2eb8; continue;
 }
 0x4a29e3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a29ec); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a2b28; continue; }
  pc = 0x4a29fc; continue;
 }
 0x4a29fc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2a09); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2a22; continue;
 }
 0x4a2a1f => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2a22; continue;
 }
 0x4a2a22 => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x4a2b1c; continue; }
  pc = 0x4a2a2e; continue;
 }
 0x4a2a2e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f8238u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2a49); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2a52); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a2a83; continue; }
  pc = 0x4a2a62; continue;
 }
 0x4a2a62 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2a78); f_004a2edf(cpu);
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a2a83 => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a2a8a); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a2ad2; continue; }
  pc = 0x4a2aa4; continue;
 }
 0x4a2aa4 => {
  { let v: u32 = cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a2ab0); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a2ac1; continue;
 }
 0x4a2ac1 => {
  { let v: u32 = cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2adc; continue;
 }
 0x4a2ad2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2adc; continue;
 }
 0x4a2adc => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4a2b02); cpu.call(t);
  pc = 0x4a2b17; continue;
 }
 0x4a2b17 => {
  pc = 0x4a2a1f; continue;
 }
 0x4a2b1c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2b28); f_004a2edf(cpu);
  pc = 0x4a2b28; continue;
 }
 0x4a2b28 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2b31); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2b46; continue;
 }
 0x4a2b43 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2b46; continue;
 }
 0x4a2b46 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x4a2cc6; continue; }
  pc = 0x4a2b52; continue;
 }
 0x4a2b52 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2b5b); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0xffffff9cu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff94u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2b6e); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x8u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffff94u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2b88); f_00483464(cpu);
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x4a2b93); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff58u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a2c67; continue; }
  pc = 0x4a2bad; continue;
 }
 0x4a2bad => {
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff44u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff40u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a2bce); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff44u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff48u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff48u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffff48u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x4a2c04); cpu.call(t);
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2c0d; continue;
 }
 0x4a2c0d => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff4cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2c35; continue;
 }
 0x4a2c35 => {
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2c3e; continue;
 }
 0x4a2c3e => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2c62; continue;
 }
 0x4a2c62 => {
  pc = 0x4a2c71; continue;
 }
 0x4a2c67 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2c71; continue;
 }
 0x4a2c71 => {
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff54u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff50u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff50u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x24u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4a2c97); cpu.call(t);
  pc = 0x4a2cac; continue;
 }
 0x4a2cac => {
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2cb5); f_004a2ecc(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2cc1); f_004a2ed5(cpu);
  pc = 0x4a2b43; continue;
 }
 0x4a2cc6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  cpu.push(0x4a2cd8); f_0049b4e7(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a2ce7; continue; }
  pc = 0x4a2ce0; continue;
 }
 0x4a2ce0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a2ce7 => {
  pc = 0x4a2eb8; continue;
 }
 0x4a2cec => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2cf5); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a2e31; continue; }
  pc = 0x4a2d05; continue;
 }
 0x4a2d05 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2d12); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2d2b; continue;
 }
 0x4a2d28 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2d2b; continue;
 }
 0x4a2d2b => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x4a2e25; continue; }
  pc = 0x4a2d37; continue;
 }
 0x4a2d37 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f8238u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2d52); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2d5b); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4a2d8c; continue; }
  pc = 0x4a2d6b; continue;
 }
 0x4a2d6b => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2d81); f_004a2ec2(cpu);
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a2d8c => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4a2d93); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff24u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4a2ddb; continue; }
  pc = 0x4a2dad; continue;
 }
 0x4a2dad => {
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4a2db9); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4a2dca; continue;
 }
 0x4a2dca => {
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2de5; continue;
 }
 0x4a2ddb => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2de5; continue;
 }
 0x4a2de5 => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4a2e0b); cpu.call(t);
  pc = 0x4a2e20; continue;
 }
 0x4a2e20 => {
  pc = 0x4a2d28; continue;
 }
 0x4a2e25 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2e31); f_004a2ec2(cpu);
  pc = 0x4a2e31; continue;
 }
 0x4a2e31 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  cpu.push(0x4a2e43); f_0049b4e7(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a2e52; continue; }
  pc = 0x4a2e4b; continue;
 }
 0x4a2e4b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a2e52 => {
  pc = 0x4a2eb8; continue;
 }
 0x4a2e57 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  cpu.push(0x4a2e69); f_0049b4e7(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4a2e78; continue; }
  pc = 0x4a2e71; continue;
 }
 0x4a2e71 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a2e78 => {
  pc = 0x4a2eb8; continue;
 }
 0x4a2e7d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a2e8e => {
  { cpu.sub32(cpu.r32(0xffffff14u32.wrapping_add(cpu.ebp)), 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4a2e7d; continue; }
  pc = 0x4a2e9b; continue;
 }
 0x4a2e9b => {
  { let v: u32 = cpu.r32(0xffffff14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = cpu.r32(0x4a2ea8u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x4a2eb8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4a2f06; continue;
 }
 0x4a2f06 => {
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
pub fn f_004a280a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a280a); }
 let mut pc: u32 = 0x4a280a;
 loop { match pc {
 0x4a280a => {
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2816); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a291c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a291c); }
 let mut pc: u32 = 0x4a291c;
 loop { match pc {
 0x4a291c => {
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a292b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a294a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a294a); }
 let mut pc: u32 = 0x4a294a;
 loop { match pc {
 0x4a294a => {
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a2958); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2993(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2993); }
 let mut pc: u32 = 0x4a2993;
 loop { match pc {
 0x4a2993 => {
  { let v: u32 = cpu.r32(0xffffff74u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a299f); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2b07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2b07); }
 let mut pc: u32 = 0x4a2b07;
 loop { match pc {
 0x4a2b07 => {
  { let v: u32 = cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2b13); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2c25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2c25); }
 let mut pc: u32 = 0x4a2c25;
 loop { match pc {
 0x4a2c25 => {
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a2c34); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2c53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2c53); }
 let mut pc: u32 = 0x4a2c53;
 loop { match pc {
 0x4a2c53 => {
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4a2c61); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2c9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2c9c); }
 let mut pc: u32 = 0x4a2c9c;
 loop { match pc {
 0x4a2c9c => {
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2ca8); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2e10); }
 let mut pc: u32 = 0x4a2e10;
 loop { match pc {
 0x4a2e10 => {
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2e1c); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2ec2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2ec2); }
 let mut pc: u32 = 0x4a2ec2;
 loop { match pc {
 0x4a2ec2 => {
  { let v: u32 = 0xffffff88u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a2ecb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2ecc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2ecc); }
 let mut pc: u32 = 0x4a2ecc;
 loop { match pc {
 0x4a2ecc => {
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2ed4); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2ed5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2ed5); }
 let mut pc: u32 = 0x4a2ed5;
 loop { match pc {
 0x4a2ed5 => {
  { let v: u32 = 0xffffff94u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a2ede); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2edf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2edf); }
 let mut pc: u32 = 0x4a2edf;
 loop { match pc {
 0x4a2edf => {
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a2ee8); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2ee9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2ee9); }
 let mut pc: u32 = 0x4a2ee9;
 loop { match pc {
 0x4a2ee9 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4a2ef1); f_004a3e30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2ef2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2ef2); }
 let mut pc: u32 = 0x4a2ef2;
 loop { match pc {
 0x4a2ef2 => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a2efb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2efc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2efc); }
 let mut pc: u32 = 0x4a2efc;
 loop { match pc {
 0x4a2efc => {
  { let v: u32 = 0x4bcd60u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2f17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2f17); }
 let mut pc: u32 = 0x4a2f17;
 loop { match pc {
 0x4a2f17 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2f2e); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  cpu.push(0x4a2f40); f_0049b7f9(cpu);
  pc = 0x4a2f45; continue;
 }
 0x4a2f45 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2f50); }
 let mut pc: u32 = 0x4a2f50;
 loop { match pc {
 0x4a2f50 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b31ecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a2f6a; continue;
 }
 0x4a2f6a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2f70); }
 let mut pc: u32 = 0x4a2f70;
 loop { match pc {
 0x4a2f70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b31ecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a2f8a; continue;
 }
 0x4a2f8a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a2fa0; continue; }
  pc = 0x4a2f94; continue;
 }
 0x4a2f94 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a2f9d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a2fa0; continue;
 }
 0x4a2fa0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a2fa8; continue;
 }
 0x4a2fa8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a2fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2fb0); }
 let mut pc: u32 = 0x4a2fb0;
 loop { match pc {
 0x4a2fb0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3015u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a2fdd; continue;
 }
 0x4a2fdd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a2fe9); f_004a2fee(cpu);
  pc = 0x4a2ff7; continue;
 }
 0x4a2ff7 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a300d; continue; }
  pc = 0x4a3001; continue;
 }
 0x4a3001 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a300a); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a300d; continue;
 }
 0x4a300d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a301f; continue;
 }
 0x4a301f => {
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
pub fn f_004a2fee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a2fee); }
 let mut pc: u32 = 0x4a2fee;
 loop { match pc {
 0x4a2fee => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a2ff6); f_004a2f50(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3015(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3015); }
 let mut pc: u32 = 0x4a3015;
 loop { match pc {
 0x4a3015 => {
  { let v: u32 = 0x4bcdf8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3030); }
 let mut pc: u32 = 0x4a3030;
 loop { match pc {
 0x4a3030 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a30afu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3061; continue;
 }
 0x4a3061 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a306a); f_004a307b(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3076); f_004a3088(cpu);
  pc = 0x4a3091; continue;
 }
 0x4a3091 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a30a7; continue; }
  pc = 0x4a309b; continue;
 }
 0x4a309b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a30a4); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a30a7; continue;
 }
 0x4a30a7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a30b9; continue;
 }
 0x4a30b9 => {
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
pub fn f_004a307b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a307b); }
 let mut pc: u32 = 0x4a307b;
 loop { match pc {
 0x4a307b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a3087); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3088(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3088); }
 let mut pc: u32 = 0x4a3088;
 loop { match pc {
 0x4a3088 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a3090); f_00487ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a30af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a30af); }
 let mut pc: u32 = 0x4a30af;
 loop { match pc {
 0x4a30af => {
  { let v: u32 = 0x4bce20u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a30d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a30d0); }
 let mut pc: u32 = 0x4a30d0;
 loop { match pc {
 0x4a30d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3120u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3214u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3106; continue;
 }
 0x4a3106 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3112); f_004a3117(cpu);
  pc = 0x4a312a; continue;
 }
 0x4a312a => {
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
pub fn f_004a3117(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3117); }
 let mut pc: u32 = 0x4a3117;
 loop { match pc {
 0x4a3117 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a311f); f_00489b10(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3120); }
 let mut pc: u32 = 0x4a3120;
 loop { match pc {
 0x4a3120 => {
  { let v: u32 = 0x4bce50u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3140); }
 let mut pc: u32 = 0x4a3140;
 loop { match pc {
 0x4a3140 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a31aeu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3214u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3176; continue;
 }
 0x4a3176 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3182); f_004a3187(cpu);
  pc = 0x4a3190; continue;
 }
 0x4a3190 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a31a6; continue; }
  pc = 0x4a319a; continue;
 }
 0x4a319a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a31a3); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a31a6; continue;
 }
 0x4a31a6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a31b8; continue;
 }
 0x4a31b8 => {
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
pub fn f_004a3187(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3187); }
 let mut pc: u32 = 0x4a3187;
 loop { match pc {
 0x4a3187 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a318f); f_00489b10(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a31ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a31ae); }
 let mut pc: u32 = 0x4a31ae;
 loop { match pc {
 0x4a31ae => {
  { let v: u32 = 0x4bce78u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a31d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a31d0); }
 let mut pc: u32 = 0x4a31d0;
 loop { match pc {
 0x4a31d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a31e4); f_004967ad(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a31fa; continue; }
  pc = 0x4a31ee; continue;
 }
 0x4a31ee => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a31f7); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a31fa; continue;
 }
 0x4a31fa => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a3202; continue;
 }
 0x4a3202 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3210); }
 let mut pc: u32 = 0x4a3210;
 loop { match pc {
 0x4a3210 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a322a; continue;
 }
 0x4a322a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3240); }
 let mut pc: u32 = 0x4a3240;
 loop { match pc {
 0x4a3240 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3288u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a326d; continue;
 }
 0x4a326d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3279); f_004a327e(cpu);
  pc = 0x4a3292; continue;
 }
 0x4a3292 => {
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
pub fn f_004a327e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a327e); }
 let mut pc: u32 = 0x4a327e;
 loop { match pc {
 0x4a327e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3287); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3288(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3288); }
 let mut pc: u32 = 0x4a3288;
 loop { match pc {
 0x4a3288 => {
  { let v: u32 = 0x4bcea0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a32a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a32a0); }
 let mut pc: u32 = 0x4a32a0;
 loop { match pc {
 0x4a32a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a32e8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a32cd; continue;
 }
 0x4a32cd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a32d9); f_004a32de(cpu);
  pc = 0x4a32f2; continue;
 }
 0x4a32f2 => {
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
pub fn f_004a32de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a32de); }
 let mut pc: u32 = 0x4a32de;
 loop { match pc {
 0x4a32de => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a32e7); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a32e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a32e8); }
 let mut pc: u32 = 0x4a32e8;
 loop { match pc {
 0x4a32e8 => {
  { let v: u32 = 0x4bcec8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3300); }
 let mut pc: u32 = 0x4a3300;
 loop { match pc {
 0x4a3300 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3304u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a331a; continue;
 }
 0x4a331a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3320); }
 let mut pc: u32 = 0x4a3320;
 loop { match pc {
 0x4a3320 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3304u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a333a; continue;
 }
 0x4a333a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a3350; continue; }
  pc = 0x4a3344; continue;
 }
 0x4a3344 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a334d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a3350; continue;
 }
 0x4a3350 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a3358; continue;
 }
 0x4a3358 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3360); }
 let mut pc: u32 = 0x4a3360;
 loop { match pc {
 0x4a3360 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a33a7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a338d; continue;
 }
 0x4a338d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3399); f_004a339e(cpu);
  pc = 0x4a33b1; continue;
 }
 0x4a33b1 => {
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
pub fn f_004a339e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a339e); }
 let mut pc: u32 = 0x4a339e;
 loop { match pc {
 0x4a339e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a33a6); f_004a3300(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a33a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a33a7); }
 let mut pc: u32 = 0x4a33a7;
 loop { match pc {
 0x4a33a7 => {
  { let v: u32 = 0x4bcef0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a33c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a33c0); }
 let mut pc: u32 = 0x4a33c0;
 loop { match pc {
 0x4a33c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3408u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a33ed; continue;
 }
 0x4a33ed => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a33f9); f_004a33fe(cpu);
  pc = 0x4a3412; continue;
 }
 0x4a3412 => {
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
pub fn f_004a33fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a33fe); }
 let mut pc: u32 = 0x4a33fe;
 loop { match pc {
 0x4a33fe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3407); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3408(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3408); }
 let mut pc: u32 = 0x4a3408;
 loop { match pc {
 0x4a3408 => {
  { let v: u32 = 0x4bcf18u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3420); }
 let mut pc: u32 = 0x4a3420;
 loop { match pc {
 0x4a3420 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3485u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a344d; continue;
 }
 0x4a344d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3459); f_004a345e(cpu);
  pc = 0x4a3467; continue;
 }
 0x4a3467 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a347d; continue; }
  pc = 0x4a3471; continue;
 }
 0x4a3471 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a347a); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a347d; continue;
 }
 0x4a347d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a348f; continue;
 }
 0x4a348f => {
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
pub fn f_004a345e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a345e); }
 let mut pc: u32 = 0x4a345e;
 loop { match pc {
 0x4a345e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a3466); f_004a3300(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3485(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3485); }
 let mut pc: u32 = 0x4a3485;
 loop { match pc {
 0x4a3485 => {
  { let v: u32 = 0x4bcf40u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a34a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a34a0); }
 let mut pc: u32 = 0x4a34a0;
 loop { match pc {
 0x4a34a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a34f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3324u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a34d6; continue;
 }
 0x4a34d6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a34e2); f_004a34e7(cpu);
  pc = 0x4a34fa; continue;
 }
 0x4a34fa => {
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
pub fn f_004a34e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a34e7); }
 let mut pc: u32 = 0x4a34e7;
 loop { match pc {
 0x4a34e7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a34ef); f_00489b10(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a34f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a34f0); }
 let mut pc: u32 = 0x4a34f0;
 loop { match pc {
 0x4a34f0 => {
  { let v: u32 = 0x4bcf68u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3510); }
 let mut pc: u32 = 0x4a3510;
 loop { match pc {
 0x4a3510 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a357eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b3324u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3546; continue;
 }
 0x4a3546 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3552); f_004a3557(cpu);
  pc = 0x4a3560; continue;
 }
 0x4a3560 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a3576; continue; }
  pc = 0x4a356a; continue;
 }
 0x4a356a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a3573); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a3576; continue;
 }
 0x4a3576 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a3588; continue;
 }
 0x4a3588 => {
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
pub fn f_004a3557(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3557); }
 let mut pc: u32 = 0x4a3557;
 loop { match pc {
 0x4a3557 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a355f); f_00489b10(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a357e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a357e); }
 let mut pc: u32 = 0x4a357e;
 loop { match pc {
 0x4a357e => {
  { let v: u32 = 0x4bcf90u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a35a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a35a0); }
 let mut pc: u32 = 0x4a35a0;
 loop { match pc {
 0x4a35a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4a35b4); f_00498a52(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a35ca; continue; }
  pc = 0x4a35be; continue;
 }
 0x4a35be => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a35c7); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4a35ca; continue;
 }
 0x4a35ca => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4a35d2; continue;
 }
 0x4a35d2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a35e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a35e0); }
 let mut pc: u32 = 0x4a35e0;
 loop { match pc {
 0x4a35e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3628u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a360d; continue;
 }
 0x4a360d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3619); f_004a361e(cpu);
  pc = 0x4a3632; continue;
 }
 0x4a3632 => {
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
pub fn f_004a361e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a361e); }
 let mut pc: u32 = 0x4a361e;
 loop { match pc {
 0x4a361e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3627); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3628(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3628); }
 let mut pc: u32 = 0x4a3628;
 loop { match pc {
 0x4a3628 => {
  { let v: u32 = 0x4bcfb8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3640); }
 let mut pc: u32 = 0x4a3640;
 loop { match pc {
 0x4a3640 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3688u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a366d; continue;
 }
 0x4a366d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3679); f_004a367e(cpu);
  pc = 0x4a3692; continue;
 }
 0x4a3692 => {
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
pub fn f_004a367e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a367e); }
 let mut pc: u32 = 0x4a367e;
 loop { match pc {
 0x4a367e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3687); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3688(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3688); }
 let mut pc: u32 = 0x4a3688;
 loop { match pc {
 0x4a3688 => {
  { let v: u32 = 0x4bcfe0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a36a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a36a0); }
 let mut pc: u32 = 0x4a36a0;
 loop { match pc {
 0x4a36a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x4a36ba; continue;
 }
 0x4a36ba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a36c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a36c0); }
 let mut pc: u32 = 0x4a36c0;
 loop { match pc {
 0x4a36c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a3708u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a36ed; continue;
 }
 0x4a36ed => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a36f9); f_004a36fe(cpu);
  pc = 0x4a3712; continue;
 }
 0x4a3712 => {
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
pub fn f_004a36fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a36fe); }
 let mut pc: u32 = 0x4a36fe;
 loop { match pc {
 0x4a36fe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a3707); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3708(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3708); }
 let mut pc: u32 = 0x4a3708;
 loop { match pc {
 0x4a3708 => {
  { let v: u32 = 0x4bd008u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3720); }
 let mut pc: u32 = 0x4a3720;
 loop { match pc {
 0x4a3720 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3731; continue;
 }
 0x4a3731 => {
  pc = 0x4a3736; continue;
 }
 0x4a3736 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a3744; continue;
 }
 0x4a3744 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4a375d; continue; }
  pc = 0x4a374e; continue;
 }
 0x4a374e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4a375a); f_004ae7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4a375d; continue;
 }
 0x4a375d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0xcu32); cpu.eax = v; }
  pc = 0x4a3768; continue;
 }
 0x4a3768 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3770); }
 let mut pc: u32 = 0x4a3770;
 loop { match pc {
 0x4a3770 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3781; continue;
 }
 0x4a3781 => {
  pc = 0x4a3786; continue;
 }
 0x4a3786 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x4a3795; continue;
 }
 0x4a3795 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a37a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a37a0); }
 let mut pc: u32 = 0x4a37a0;
 loop { match pc {
 0x4a37a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a37e8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a37cd; continue;
 }
 0x4a37cd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a37d9); f_004a37de(cpu);
  pc = 0x4a37f2; continue;
 }
 0x4a37f2 => {
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
pub fn f_004a37de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a37de); }
 let mut pc: u32 = 0x4a37de;
 loop { match pc {
 0x4a37de => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4a37e7); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a37e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a37e8); }
 let mut pc: u32 = 0x4a37e8;
 loop { match pc {
 0x4a37e8 => {
  { let v: u32 = 0x4bd030u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3800); }
 let mut pc: u32 = 0x4a3800;
 loop { match pc {
 0x4a3800 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4a38abu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a3826; continue;
 }
 0x4a3826 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4a383e; continue;
 }
 0x4a383e => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3847); f_004a386a(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3850); f_004a3876(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3859); f_004a3883(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4a3865); f_004a3890(cpu);
  pc = 0x4a389c; continue;
 }
 0x4a389c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a38b5; continue;
 }
 0x4a38b5 => {
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
pub fn f_004a386a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a386a); }
 let mut pc: u32 = 0x4a386a;
 loop { match pc {
 0x4a386a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  cpu.push(0x4a3875); f_004098b0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3876(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3876); }
 let mut pc: u32 = 0x4a3876;
 loop { match pc {
 0x4a3876 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a3882); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3883(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3883); }
 let mut pc: u32 = 0x4a3883;
 loop { match pc {
 0x4a3883 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4a388f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a3890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a3890); }
 let mut pc: u32 = 0x4a3890;
 loop { match pc {
 0x4a3890 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  cpu.push(0x4a389b); f_00492f30(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a38ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a38ab); }
 let mut pc: u32 = 0x4a38ab;
 loop { match pc {
 0x4a38ab => {
  { let v: u32 = 0x4bd058u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
