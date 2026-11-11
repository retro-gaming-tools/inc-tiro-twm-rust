#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_15001003(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15001003); }
 let mut pc: u32 = 0x15001003;
 loop { match pc {
 0x15001003 => {
  {  }
  {  }
  {  }
  {  }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15013f08u32.wrapping_add(cpu.edx.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x15001049; continue; }
  pc = 0x15001021; continue;
 }
 0x15001021 => {
  { let v = 0x15016f9cu32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001030); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001046); f_1500ffde(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x15001049; continue;
 }
 0x15001049 => {
  { let v: u32 = cpu.esi; cpu.w32(0x15013f08u32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15001010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15001010); }
 let mut pc: u32 = 0x15001010;
 loop { match pc {
 0x15001010 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15013f08u32.wrapping_add(cpu.edx.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x15001049; continue; }
  pc = 0x15001021; continue;
 }
 0x15001021 => {
  { let v = 0x15016f9cu32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001030); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001046); f_1500ffde(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x15001049; continue;
 }
 0x15001049 => {
  { let v: u32 = cpu.esi; cpu.w32(0x15013f08u32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15001054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15001054); }
 let mut pc: u32 = 0x15001054;
 loop { match pc {
 0x15001054 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x14u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500106e; continue; }
  pc = 0x15001067; continue;
 }
 0x15001067 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15001738; continue;
 }
 0x1500106e => {
  { let v = 0xffffu32; cpu.push(v); }
  cpu.push(0x15001078); f_1500ffd8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150010b8; continue; }
  pc = 0x15001081; continue;
 }
 0x15001081 => {
  { let v = 0x15016fe0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001090); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150010a6); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150010b8 => {
  { cpu.sub32(cpu.r32(0x15010020u32), 0x0u32); }
  if !cpu.zf { pc = 0x1500116f; continue; }
  pc = 0x150010c5; continue;
 }
 0x150010c5 => {
  { let v = 0x7ef4u32; cpu.push(v); }
  cpu.push(0x150010cf); f_1500ffd8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010020u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15001112; continue; }
  pc = 0x150010db; continue;
 }
 0x150010db => {
  { let v = 0x15017020u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150010ea); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001100); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001112 => {
  { let v = 0x3a98u32; cpu.push(v); }
  cpu.push(0x1500111c); f_1500ffd8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010024u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500115f; continue; }
  pc = 0x15001128; continue;
 }
 0x15001128 => {
  { let v = 0x15017060u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001137); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500114d); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500115f => {
  { let v = 0x3a98u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500116c); f_1500ffd2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1500116f; continue;
 }
 0x1500116f => {
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v = 0x150170a8u32; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x14u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500118c); f_1500ffcc(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x15011974u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x150011a0); f_1500ffc6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15011974u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15001205; continue; }
  pc = 0x150011ac; continue;
 }
 0x150011ac => {
  { let v = 0x150170b8u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150011bb); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150011cb); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150011dd); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150011f3); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001205 => {
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v = 0x150170e4u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500121f); f_1500ffcc(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500122a); f_1500ffba(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500128c; continue; }
  pc = 0x15001233; continue;
 }
 0x15001233 => {
  { let v = 0x150170f4u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001242); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001252); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001264); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500127a); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500128c => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x1500128e; continue;
 }
 0x1500128e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15001294); f_1500ffb4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500144d; continue; }
  pc = 0x1500129f; continue;
 }
 0x1500129f => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x108u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x150012af); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500130f; continue; }
  pc = 0x150012b6; continue;
 }
 0x150012b6 => {
  { let v = 0x15017114u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150012c5); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150012d5); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150012e7); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150012fd); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500130f => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x15001376; continue; }
  pc = 0x1500131d; continue;
 }
 0x1500131d => {
  { let v = 0x15017138u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500132c); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500133c); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1501715cu32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500134e); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001364); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001376 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1500137e); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x150013de; continue; }
  pc = 0x15001385; continue;
 }
 0x15001385 => {
  { let v = 0x1501717cu32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001394); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150013a4); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150013b6); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150013cc); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150013de => {
  { let v = 0x5u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x150013e7); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001447; continue; }
  pc = 0x150013ee; continue;
 }
 0x150013ee => {
  { let v = 0x150171a0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150013fd); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500140d); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500141f); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001435); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001447 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1500128e; continue;
 }
 0x1500144d => {
  { let v: u32 = cpu.edi; cpu.w32(0x1501001cu32, v); }
  { cpu.sub32(cpu.edi, 0x9c4u32); }
  if (cpu.sf != cpu.of) { pc = 0x15001492; continue; }
  pc = 0x1500145b; continue;
 }
 0x1500145b => {
  { let v = 0x150171c4u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500146a); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001480); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001492 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1500149a); f_1500ffa8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2710u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x15013f08u32; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x150014b0); f_1500ffd2(cpu);
  { let v: u32 = cpu.r32(0x1501001cu32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15001721; continue; }
  pc = 0x150014c1; continue;
 }
 0x150014c1 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x150014c3; continue;
 }
 0x150014c3 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x108u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x150014d3); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001533; continue; }
  pc = 0x150014da; continue;
 }
 0x150014da => {
  { let v = 0x150171f4u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150014e9); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150014f9); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500150b); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001521); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001533 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15001542); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x150015a2; continue; }
  pc = 0x15001549; continue;
 }
 0x15001549 => {
  { let v = 0x15017218u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001558); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001568); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500157a); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001590); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150015a2 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x110u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x150015b2); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001612; continue; }
  pc = 0x150015b9; continue;
 }
 0x150015b9 => {
  { let v = 0x1501723cu32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150015c8); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150015d8); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150015ea); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001600); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001612 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15001622); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001682; continue; }
  pc = 0x15001629; continue;
 }
 0x15001629 => {
  { let v = 0x15017260u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001638); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001648); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500165a); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001670); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001682 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15011974u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x15001697); f_15001010(cpu);
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x150016a5); f_1500ffd8(cpu);
  { let v: u32 = cpu.r32(0x15010024u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150016ee; continue; }
  pc = 0x150016b7; continue;
 }
 0x150016b7 => {
  { let v = 0x15017284u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150016c6); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150016dc); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150016ee => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x15001711); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x1501001cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x150014c3; continue; }
  pc = 0x15001721; continue;
 }
 0x15001721 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15001727); f_1500ffa2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001730); f_1500ff9c(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x15001738; continue;
 }
 0x15001738 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15001748(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15001748); }
 let mut pc: u32 = 0x15001748;
 loop { match pc {
 0x15001748 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x104u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x15010028u32), 0x0u16); }
  if !cpu.zf { pc = 0x1500187e; continue; }
  pc = 0x1500175f; continue;
 }
 0x1500175f => {
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v = 0x150172c8u32; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x15010028u32, v); }
  { let v: u32 = 0x118u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500178b); f_1500ffcc(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x1508be20u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500179d); f_1500ffc6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150017f5; continue; }
  pc = 0x150017a4; continue;
 }
 0x150017a4 => {
  { let v = 0x150172d4u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150017b3); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150017c3); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150017d5); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x150017eb); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15001883; continue;
 }
 0x150017f5 => {
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v = 0x150172f4u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500180f); f_1500ffcc(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x1508e620u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15001821); f_1500ffc6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500187e; continue; }
  pc = 0x15001828; continue;
 }
 0x15001828 => {
  { let v = 0x150172fcu32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001837); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001847); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001859); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x1500186f); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x104u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500187e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x15001883; continue;
 }
 0x15001883 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x104u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15001948(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15001948); }
 let mut pc: u32 = 0x15001948;
 loop { match pc {
 0x15001948 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x150821bcu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15001989; continue; }
  pc = 0x15001961; continue;
 }
 0x15001961 => {
  { let v: u32 = 0x15011988u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  pc = 0x15001969; continue;
 }
 0x15001969 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001970); f_1500ff96(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500197b; continue; }
  pc = 0x15001977; continue;
 }
 0x15001977 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x15001989; continue;
 }
 0x1500197b => {
  { let v: u32 = cpu.r32(0x150821bcu32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15001969; continue; }
  pc = 0x15001989; continue;
 }
 0x15001989 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150019e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150019e8); }
 let mut pc: u32 = 0x150019e8;
 loop { match pc {
 0x150019e8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1501196cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15001ac1; continue; }
  pc = 0x150019fe; continue;
 }
 0x150019fe => {
  { let v: u32 = cpu.esi; cpu.w32(0x150821c0u32, v); }
  { cpu.sub32(cpu.esi, 0x12cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15001a3b; continue; }
  pc = 0x15001a0c; continue;
 }
 0x15001a0c => {
  { let v = 0x1501731cu32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001a1b); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001a31); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x15001ad2; continue;
 }
 0x15001a3b => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x15001a96; continue; }
  pc = 0x15001a3f; continue;
 }
 0x15001a3f => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15001a4c); f_1500ffd8(cpu);
  { let v: u32 = cpu.r32(0x150821c0u32); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1501196cu32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15001a96; continue; }
  pc = 0x15001a60; continue;
 }
 0x15001a60 => {
  { let v = 0x15017354u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001a6f); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001a85); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150821c0u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001a96 => {
  { let v: u32 = cpu.r32(0x1501196cu32); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x150821c0u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x150821b8u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15001ac1; continue; }
  pc = 0x15001aae; continue;
 }
 0x15001aae => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15001ab2; continue;
 }
 0x15001ab2 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x15001ab2; continue; }
  pc = 0x15001ac1; continue;
 }
 0x15001ac1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x150821c0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1501196cu32, v); }
  pc = 0x15001ad2; continue;
 }
 0x15001ad2 => {
  { let v: u32 = cpu.r32(0x150821c0u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15001ae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15001ae0); }
 let mut pc: u32 = 0x15001ae0;
 loop { match pc {
 0x15001ae0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10cu32); cpu.esp = v; }
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v = 0x1501739cu32; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x21cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x150170a4u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15001b0b); f_1500ffcc(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x15001b18); f_1500ffba(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001b72; continue; }
  pc = 0x15001b21; continue;
 }
 0x15001b21 => {
  { let v = 0x150173a8u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001b30); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001b40); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001b52); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001b68); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15001d82; continue;
 }
 0x15001b72 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15001b82); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001b8e; continue; }
  pc = 0x15001b89; continue;
 }
 0x15001b89 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x15001b8e; continue;
 }
 0x15001b8e => {
  { cpu.sub32(cpu.r32(0x108u32.wrapping_add(cpu.esp)), 0x4e494244u32); }
  if cpu.zf { pc = 0x15001bf4; continue; }
  pc = 0x15001b9b; continue;
 }
 0x15001b9b => {
  { let v = 0x150173d8u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001baa); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001bba); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001bcc); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001be2); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001bf4 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x150821bcu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001c01); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001c0d; continue; }
  pc = 0x15001c08; continue;
 }
 0x15001c08 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x15001c0d; continue;
 }
 0x15001c0d => {
  { let v: u32 = cpu.r32(0x150821bcu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x12cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15001c52; continue; }
  pc = 0x15001c1b; continue;
 }
 0x15001c1b => {
  { let v = 0x15017404u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001c2a); f_1500ffe4(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001c40); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001c52 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15001d15; continue; }
  pc = 0x15001c5c; continue;
 }
 0x15001c5c => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x108u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001c6c); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001c7d; continue; }
  pc = 0x15001c73; continue;
 }
 0x15001c73 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x15001d15; continue;
 }
 0x15001c7d => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15011988u32); cpu.ebx = v; }
  { let v = 0xdu32; cpu.push(v); }
  { let v: u32 = 0x13u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001c99); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001ca5; continue; }
  pc = 0x15001ca0; continue;
 }
 0x15001ca0 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x15001ca5; continue;
 }
 0x15001ca5 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001cb5); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001cc1; continue; }
  pc = 0x15001cbc; continue;
 }
 0x15001cbc => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x15001cc1; continue;
 }
 0x15001cc1 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001cdb); f_1500ffae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15001ce7; continue; }
  pc = 0x15001ce2; continue;
 }
 0x15001ce2 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x15001ce7; continue;
 }
 0x15001ce7 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x150821bcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15001c5c; continue; }
  pc = 0x15001d15; continue;
 }
 0x15001d15 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001d1b); f_1500ffa2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub16((cpu.edi as u16), 0x1u16); }
  if !cpu.zf { pc = 0x15001d7d; continue; }
  pc = 0x15001d24; continue;
 }
 0x15001d24 => {
  { let v = 0x15017440u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001d33); f_1500ffe4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001d43); f_1500ffc0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x150170e0u32; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001d55); f_1500ffc0(cpu);
  { let v: u32 = cpu.r32(0x15010016u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017c20u32; cpu.push(v); }
  cpu.push(0x15001d6b); f_1500ffde(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001d7d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x15001d82; continue;
 }
 0x15001d82 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15001f44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15001f44); }
 let mut pc: u32 = 0x15001f44;
 loop { match pc {
 0x15001f44 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15010020u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x15001f58; continue; }
  pc = 0x15001f4f; continue;
 }
 0x15001f4f => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15001f55); f_1500ff9c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x15001f58; continue;
 }
 0x15001f58 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010020u32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x15001f64; continue;
 }
 0x15001f64 => {
  { let v: u32 = cpu.r32(0x15010024u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x15001f84; continue; }
  pc = 0x15001f72; continue;
 }
 0x15001f72 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15001f78); f_1500ff9c(cpu);
  { let v: u32 = cpu.r32(0x15010024u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x15001f84; continue;
 }
 0x15001f84 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x6u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x3a98u32); }
  if !cpu.zf { pc = 0x15001f64; continue; }
  pc = 0x15001f8f; continue;
 }
 0x15001f8f => {
  { let v: u32 = cpu.r32(0x15010024u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x15001fa2; continue; }
  pc = 0x15001f99; continue;
 }
 0x15001f99 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15001f9f); f_1500ff9c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x15001fa2; continue;
 }
 0x15001fa2 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1501196cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010024u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15001fbc; continue; }
  pc = 0x15001fb3; continue;
 }
 0x15001fb3 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15001fb9); f_1500ff9c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x15001fbc; continue;
 }
 0x15001fbc => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15011974u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1501196cu32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x15001fd7; continue; }
  pc = 0x15001fce; continue;
 }
 0x15001fce => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x15001fd4); f_1500ff9c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x15001fd7; continue;
 }
 0x15001fd7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15011974u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002000); }
 let mut pc: u32 = 0x15002000;
 loop { match pc {
 0x15002000 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x1500200b; continue;
 }
 0x1500200b => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x15002012); f_1500ff90(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15002019); f_1500ff8a(cpu);
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if cpu.zf { pc = 0x1500202c; continue; }
  pc = 0x1500201e; continue;
 }
 0x1500201e => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xff0000u32; cpu.push(v); }
  pc = 0x15002035; continue;
 }
 0x1500202c => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x15002035; continue;
 }
 0x15002035 => {
  cpu.push(0x1500203a); f_1500ff84(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x15002042); f_1500ff8a(cpu);
  { cpu.sub32(cpu.ebx, 0x1eu32); }
  if (cpu.sf != cpu.of) { pc = 0x1500200b; continue; }
  pc = 0x15002047; continue;
 }
 0x15002047 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500204c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500204c); }
 let mut pc: u32 = 0x1500204c;
 loop { match pc {
 0x1500204c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x15002057); f_1500ff7e(cpu);
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15010b20u32, v); }
  cpu.push(0x15002067); f_150025c0(cpu);
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x150021d0; continue; }
  pc = 0x15002070; continue;
 }
 0x15002070 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15002076); f_1500ff78(cpu);
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1500207d); f_1500ff72(cpu);
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x15002084); f_1500ff6c(cpu);
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1500208b); f_1500ff66(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x15002094); f_1500ff8a(cpu);
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500209b); f_1500ff60(cpu);
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1501194eu32, v); }
  cpu.push(0x150020ab); f_1500ff5a(cpu);
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x150020b2); f_1500ff54(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x15002608u32; cpu.ebx = v; }
  { let v: u32 = 0x15002614u32; cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x15002718u32; cpu.edi = v; }
  { let v: u32 = 0x15002730u32; cpu.ebp = v; }
  cpu.push(0x150020cf); f_1500ff4e(cpu);
  { let v: u32 = 0x150026e0u32; cpu.ecx = v; }
  { let v: u32 = 0x15002748u32; cpu.eax = v; }
  { let v: u32 = 0x15002754u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010f3cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15010f44u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15010f48u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010f4cu32, v); }
  { let v: u32 = 0x15002898u32; cpu.ebx = v; }
  { let v: u32 = 0x150028bcu32; cpu.esi = v; }
  { let v: u32 = 0x150028c8u32; cpu.edi = v; }
  { let v: u32 = 0x150028dcu32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010f40u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15010f50u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15010f54u32, v); }
  { let v: u32 = 0x150028a8u32; cpu.ecx = v; }
  { let v: u32 = 0x150028e8u32; cpu.eax = v; }
  { let v: u32 = 0x15002910u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010f58u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15010f60u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15010f64u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010f68u32, v); }
  { let v: u32 = 0x15002938u32; cpu.ebx = v; }
  { let v: u32 = 0x15002970u32; cpu.esi = v; }
  { let v: u32 = 0x15002998u32; cpu.edi = v; }
  { let v: u32 = 0x150029c0u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010f5cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15010f6cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15010f70u32, v); }
  { let v: u32 = 0x15002954u32; cpu.ecx = v; }
  { let v: u32 = 0x150029d4u32; cpu.eax = v; }
  { let v: u32 = 0x1500ff48u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010f74u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15010f7cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15010f80u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010f84u32, v); }
  { let v: u32 = 0x1500ff42u32; cpu.ebx = v; }
  { let v: u32 = 0x150029ecu32; cpu.esi = v; }
  { let v: u32 = 0x15002878u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010f78u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15010f88u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15010f8cu32, v); }
  { let v: u32 = 0x150029e0u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010f90u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15010f98u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15010f9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010f94u32, v); }
  pc = 0x150021d0; continue;
 }
 0x150021d0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150021d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150021d8); }
 let mut pc: u32 = 0x150021d8;
 loop { match pc {
 0x150021d8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1501194cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x150021ee; continue; }
  pc = 0x150021e8; continue;
 }
 0x150021e8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150021ee => {
  { let v = 0x15082064u32; cpu.push(v); }
  cpu.push(0x150021f8); f_1500ff3c(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15002202; continue; }
  pc = 0x150021fc; continue;
 }
 0x150021fc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002202 => {
  cpu.push(0x15002207); f_1500ff36(cpu);
  { let v = 0x15082064u32; cpu.push(v); }
  cpu.push(0x15002211); f_1500ff30(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500221b; continue; }
  pc = 0x15002215; continue;
 }
 0x15002215 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500221b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x15002227); f_1500ff2a(cpu);
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1501194cu32, v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150025c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150025c0); }
 let mut pc: u32 = 0x150025c0;
 loop { match pc {
 0x15002438 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.esi = v; }
  { let v: u32 = 0x80u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x150820fcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15082100u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15082104u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x15002460; continue;
 }
 0x15002460 => {
  { let v: u32 = 0xau32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15082108u32, v); }
  cpu.push(0x15002472); f_1500ff06(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x150820f8u32, v); }
  cpu.push(0x1500247e); f_1500ff00(cpu);
  { let v: u32 = cpu.r32(0x150820f8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15010b24u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082060u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15010b28u32, v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x150024ac; continue; }
  pc = 0x150024a2; continue;
 }
 0x150024a2 => {
  { let v: u32 = 0x40u32; cpu.w32(0x15010b28u32, v); }
  pc = 0x150024ac; continue;
 }
 0x150024ac => {
  { let v: u32 = cpu.r32(0x15010b28u32); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15002531; continue; }
  pc = 0x150024bc; continue;
 }
 0x150024bc => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x150024be; continue;
 }
 0x150024be => {
  { let v = 0x150820fcu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15081ed0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x150024d0); f_1500fefa(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15082104u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15082100u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150820fcu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15082108u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500250d); f_1500fef4(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x15081830u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x15002521; continue; }
  pc = 0x15002518; continue;
 }
 0x15002518 => {
  cpu.push(0x1500251d); f_1500ff1e(cpu);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15002586; continue;
 }
 0x15002521 => {
  { let v: u32 = cpu.r32(0x15010b28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x150024be; continue; }
  pc = 0x15002531; continue;
 }
 0x15002531 => {
  { let v: u32 = cpu.r32(0x15010b28u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15002562; continue; }
  pc = 0x1500253b; continue;
 }
 0x1500253b => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  pc = 0x15002547; continue;
 }
 0x15002547 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x15010b1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x15010b20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010b24u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x15002547; continue; }
  pc = 0x15002562; continue;
 }
 0x15002562 => {
  { let v: u32 = 0x15011988u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x15002569; continue;
 }
 0x15002569 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x12cu32); }
  if (cpu.sf != cpu.of) { pc = 0x15002569; continue; }
  pc = 0x15002581; continue;
 }
 0x15002581 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x15002586; continue;
 }
 0x15002586 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150025c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.push(0x150025ce); f_1500feee(cpu);
  { cpu.sub32(cpu.r32(0x15010b24u32), 0x20000u32); }
  if !cpu.zf { pc = 0x15002438; continue; }
  pc = 0x150025de; continue;
 }
 0x150025de => {
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150820fcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15082100u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15082104u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x15002460; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002608(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002608); }
 let mut pc: u32 = 0x15002608;
 loop { match pc {
 0x15002608 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1500260f); f_1500fee8(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002614(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002614); }
 let mut pc: u32 = 0x15002614;
 loop { match pc {
 0x15002614 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500261f); f_1500ff8a(cpu);
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  cpu.push(0x1500262d); f_1500ff84(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x15002636); f_1500ff8a(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500263c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500263c); }
 let mut pc: u32 = 0x1500263c;
 loop { match pc {
 0x1500263c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15002646); f_1500ff90(cpu);
  { let v: u32 = cpu.r32(0x15010fa4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010fdcu32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010fd8u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15010fa0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15010fa4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15010fa0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15002673); f_1500fee2(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500267c); f_1500ff8a(cpu);
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x15002683); f_1500ff66(cpu);
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15002690); f_1500ff84(cpu);
  { let v = 0x1e0u32; cpu.push(v); }
  { let v = 0x280u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150026a3); f_1500fee2(cpu);
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x150026aa); f_1500ff66(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x150026b3); f_1500ff8a(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x150026d4; continue; }
  pc = 0x150026bc; continue;
 }
 0x150026bc => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1501194eu32, v); }
  cpu.push(0x150026cf); f_1500ff60(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x150026d4 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1501194eu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002760); }
 let mut pc: u32 = 0x15002760;
 loop { match pc {
 0x15002760 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15002788); f_1500fed6(cpu);
  { let v = (cpu.r32(0x15010fe8u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x24u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x15017498u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x2cu32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(3) * cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = 0x15081fd0u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  cpu.push(0x150027c3); f_1500fed0(cpu);
  { let v: u8 = cpu.r8(0x15081fd0u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x150027d5; continue; }
  pc = 0x150027d1; continue;
 }
 0x150027d1 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x150027e6; continue;
 }
 0x150027d5 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x150027e6; continue; }
  pc = 0x150027db; continue;
 }
 0x150027db => {
  { cpu.sub8(cpu.r8(0x15081fd0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x150027d1; continue; }
  pc = 0x150027e4; continue;
 }
 0x150027e4 => {
  pc = 0x150027d5; continue;
 }
 0x150027e6 => {
  { let v: u8 = cpu.r8(0x15081fd0u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x150027f7; continue; }
  pc = 0x150027f3; continue;
 }
 0x150027f3 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x15002808; continue;
 }
 0x150027f7 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x15002808; continue; }
  pc = 0x150027fd; continue;
 }
 0x150027fd => {
  { cpu.sub8(cpu.r8(0x15081fd0u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x150027f3; continue; }
  pc = 0x15002806; continue;
 }
 0x15002806 => {
  pc = 0x150027f7; continue;
 }
 0x15002808 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x15002870; continue; }
  pc = 0x1500280d; continue;
 }
 0x1500280d => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x15002870; continue; }
  pc = 0x15002812; continue;
 }
 0x15002812 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = (cpu.r32(0x20u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.rf64(0x150174a0u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) * cpu.rf64(0x150174a8u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x15002866; continue; }
  pc = 0x1500283a; continue;
 }
 0x1500283a => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x15002841; continue;
 }
 0x15002841 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  cpu.push(0x15002848); f_150030a4(cpu);
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15081fcfu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x15002841; continue; }
  pc = 0x15002861; continue;
 }
 0x15002861 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x15002866; continue;
 }
 0x15002866 => {
  { let v = 0x15081fd0u32; cpu.push(v); }
  cpu.push(0x15002870); f_1500feca(cpu);
  pc = 0x15002870; continue;
 }
 0x15002870 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002878(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002878); }
 let mut pc: u32 = 0x15002878;
 loop { match pc {
 0x15002878 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x150820fcu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1508210cu32, v); }
  { let v: u32 = cpu.r32(0x15081ed0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15002895); f_1500fec4(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002898(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002898); }
 let mut pc: u32 = 0x15002898;
 loop { match pc {
 0x15002898 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150028a3); f_1500ff4e(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150028a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150028a8); }
 let mut pc: u32 = 0x150028a8;
 loop { match pc {
 0x150028a8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x150028b9); f_1500febe(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150028bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150028bc); }
 let mut pc: u32 = 0x150028bc;
 loop { match pc {
 0x150028bc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x150028c5); f_1500ff54(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150028c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150028c8); }
 let mut pc: u32 = 0x150028c8;
 loop { match pc {
 0x150028c8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x150028d9); f_1500febe(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150028dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150028dc); }
 let mut pc: u32 = 0x150028dc;
 loop { match pc {
 0x150028dc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x150028e5); f_1500ff54(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150028e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150028e8); }
 let mut pc: u32 = 0x150028e8;
 loop { match pc {
 0x150028e8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x150028f9); f_1500febe(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x15002906); f_1500feb8(cpu);
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500290d); f_1500ff66(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002910); }
 let mut pc: u32 = 0x15002910;
 loop { match pc {
 0x15002910 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x1500291f); f_1500feb8(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x1500292e); f_1500febe(cpu);
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x15002935); f_1500ff66(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002938); }
 let mut pc: u32 = 0x15002938;
 loop { match pc {
 0x15002938 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x15002947); f_1500feb8(cpu);
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500294e); f_1500ff66(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002954(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002954); }
 let mut pc: u32 = 0x15002954;
 loop { match pc {
 0x15002954 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x15002963); f_1500feb8(cpu);
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1500296a); f_1500ff66(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002970); }
 let mut pc: u32 = 0x15002970;
 loop { match pc {
 0x15002970 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500297f); f_1500feb8(cpu);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x1500298e); f_1500febe(cpu);
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15002995); f_1500ff66(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002998(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002998); }
 let mut pc: u32 = 0x15002998;
 loop { match pc {
 0x15002998 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x150029a7); f_1500feb8(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x150029b6); f_1500febe(cpu);
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x150029bd); f_1500ff66(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150029c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150029c0); }
 let mut pc: u32 = 0x150029c0;
 loop { match pc {
 0x150029c0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x150029c9); f_1500feb2(cpu);
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150029d0); f_1500feac(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150029d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150029d4); }
 let mut pc: u32 = 0x150029d4;
 loop { match pc {
 0x150029d4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150029dd); f_1500feb2(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150029e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150029e0); }
 let mut pc: u32 = 0x150029e0;
 loop { match pc {
 0x150029e0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x150029e8); f_1500fea6(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150029ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150029ec); }
 let mut pc: u32 = 0x150029ec;
 loop { match pc {
 0x150029ec => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x150820fcu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15081ed0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15002a04); f_1500fea0(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002a20); }
 let mut pc: u32 = 0x15002a20;
 loop { match pc {
 0x15001ea0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x15001eac); f_150021d8(cpu);
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x15001ebb; continue; }
  pc = 0x15001eb1; continue;
 }
 0x15001eb1 => {
  { let v: u32 = 0x186a1u32; cpu.eax = v; }
  pc = 0x15001f3b; continue;
 }
 0x15001ebb => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15001ec2); f_15001748(cpu);
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x15001ed2; continue; }
  pc = 0x15001ec7; continue;
 }
 0x15001ec7 => {
  { let v: u32 = 0x186a2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001ed2 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15001ed9); f_15001054(cpu);
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x15001ee9; continue; }
  pc = 0x15001ede; continue;
 }
 0x15001ede => {
  { let v: u32 = 0x186a3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001ee9 => {
  { let v: u32 = cpu.r32(0x320u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15011978u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150821c4u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15001f03); f_150019e8(cpu);
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x15001f13; continue; }
  pc = 0x15001f08; continue;
 }
 0x15001f08 => {
  { let v: u32 = 0x186a4u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001f13 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15001f1a); f_15001ae0(cpu);
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x15001f2a; continue; }
  pc = 0x15001f1f; continue;
 }
 0x15001f1f => {
  { let v: u32 = 0x186a4u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15001f2a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010fb0u32, v); }
  cpu.push(0x15001f36); f_15007596(cpu);
  { let v: u32 = 0x186a0u32; cpu.eax = v; }
  pc = 0x15001f3b; continue;
 }
 0x15001f3b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002a20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x15001ea0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002a2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002a2c); }
 let mut pc: u32 = 0x15002a2c;
 loop { match pc {
 0x15001e38 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1501001cu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15001e7c; continue; }
  pc = 0x15001e51; continue;
 }
 0x15001e51 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x15001e53; continue;
 }
 0x15001e53 => {
  { let v: u32 = cpu.r32(0x15010024u32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15001e63); f_1500ff96(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15001e6e; continue; }
  pc = 0x15001e6a; continue;
 }
 0x15001e6a => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x15001e7c; continue;
 }
 0x15001e6e => {
  { let v: u32 = cpu.r32(0x1501001cu32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15001e53; continue; }
  pc = 0x15001e7c; continue;
 }
 0x15001e7c => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002a2c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x15001e38; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002a38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002a38); }
 let mut pc: u32 = 0x15002a38;
 loop { match pc {
 0x15002a38 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002a62); f_150073c0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002a6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002a6c); }
 let mut pc: u32 = 0x15002a6c;
 loop { match pc {
 0x15002a6c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15002a74); f_15001f44(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002a78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002a78); }
 let mut pc: u32 = 0x15002a78;
 loop { match pc {
 0x15002a78 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15002a80); f_15007700(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002a84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002a84); }
 let mut pc: u32 = 0x15002a84;
 loop { match pc {
 0x15002a84 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15002a8c); f_150077ca(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002a90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002a90); }
 let mut pc: u32 = 0x15002a90;
 loop { match pc {
 0x15002a90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002aa4); f_15008efa(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002aa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002aa8); }
 let mut pc: u32 = 0x15002aa8;
 loop { match pc {
 0x15002aa8 => {
  pc = 0x150077d6; continue;
 }
 0x150077d6 => {
  { let v: u32 = 0x15017d7cu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010030u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15010034u32, v); }
  { let v: u32 = 0x15019cbcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1502777cu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010038u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1501003cu32, v); }
  { let v: u32 = 0x150296bcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ab0); }
 let mut pc: u32 = 0x15002ab0;
 loop { match pc {
 0x15002ab0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002abc); f_15007860(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ac0); }
 let mut pc: u32 = 0x15002ac0;
 loop { match pc {
 0x15002ac0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15002ac8); f_15007814(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002acc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002acc); }
 let mut pc: u32 = 0x15002acc;
 loop { match pc {
 0x15002acc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15002ad4); f_15007801(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ad8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ad8); }
 let mut pc: u32 = 0x15002ad8;
 loop { match pc {
 0x15002ad8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002ae8); f_1500ccf0(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002aec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002aec); }
 let mut pc: u32 = 0x15002aec;
 loop { match pc {
 0x15002aec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002b04); f_1500da56(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002b08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002b08); }
 let mut pc: u32 = 0x15002b08;
 loop { match pc {
 0x15002b08 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002b20); f_1500d8f3(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002b24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002b24); }
 let mut pc: u32 = 0x15002b24;
 loop { match pc {
 0x15002b24 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002b38); f_1500d09c(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002b3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002b3c); }
 let mut pc: u32 = 0x15002b3c;
 loop { match pc {
 0x15002b3c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002b58); f_1500d109(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002b5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002b5c); }
 let mut pc: u32 = 0x15002b5c;
 loop { match pc {
 0x15002b5c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002b6c); f_15009000(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002b70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002b70); }
 let mut pc: u32 = 0x15002b70;
 loop { match pc {
 0x15002b70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x1500904f; continue;
 }
 0x1500904f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002b7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002b7c); }
 let mut pc: u32 = 0x15002b7c;
 loop { match pc {
 0x15002b7c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002b97); f_150090a0(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002b9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002b9c); }
 let mut pc: u32 = 0x15002b9c;
 loop { match pc {
 0x15002b9c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002bbc); f_1500913a(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002bc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002bc0); }
 let mut pc: u32 = 0x15002bc0;
 loop { match pc {
 0x15002bc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002bdc); f_1500924e(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002be0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002be0); }
 let mut pc: u32 = 0x15002be0;
 loop { match pc {
 0x15002be0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002bf0); f_150092c5(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002bf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002bf4); }
 let mut pc: u32 = 0x15002bf4;
 loop { match pc {
 0x15002bf4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  cpu.push(0x15002c00); f_150099f8(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c04); }
 let mut pc: u32 = 0x15002c04;
 loop { match pc {
 0x15002c04 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002c18); f_15009a49(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c1c); }
 let mut pc: u32 = 0x15002c1c;
 loop { match pc {
 0x15002c1c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002c34); f_1500df84(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c38); }
 let mut pc: u32 = 0x15002c38;
 loop { match pc {
 0x15002c38 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002c44); f_1500e0f2(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c48); }
 let mut pc: u32 = 0x15002c48;
 loop { match pc {
 0x15002c48 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002c58); f_1500e09f(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c5c); }
 let mut pc: u32 = 0x15002c5c;
 loop { match pc {
 0x15002c5c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x15002c68); f_1500e2fa(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c6c); }
 let mut pc: u32 = 0x15002c6c;
 loop { match pc {
 0x15002c6c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x15002c78); f_1500e345(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c7c); }
 let mut pc: u32 = 0x15002c7c;
 loop { match pc {
 0x15002c7c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x15002c88); f_1500e308(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c8c); }
 let mut pc: u32 = 0x15002c8c;
 loop { match pc {
 0x15002c8c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x15002c98); f_1500e353(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002c9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002c9c); }
 let mut pc: u32 = 0x15002c9c;
 loop { match pc {
 0x15002c9c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  f_1500e07c(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ca8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ca8); }
 let mut pc: u32 = 0x15002ca8;
 loop { match pc {
 0x15002ca8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002cb8); f_1500ccd7(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002cbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002cbc); }
 let mut pc: u32 = 0x15002cbc;
 loop { match pc {
 0x15002cbc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002ccc); f_1500ccd9(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002cd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002cd0); }
 let mut pc: u32 = 0x15002cd0;
 loop { match pc {
 0x15002cd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002ce0); f_1500ccd8(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ce4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ce4); }
 let mut pc: u32 = 0x15002ce4;
 loop { match pc {
 0x15002ce4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002cf4); f_1500ccdb(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002cf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002cf8); }
 let mut pc: u32 = 0x15002cf8;
 loop { match pc {
 0x15002cf8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002d08); f_1500ccdd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002d0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002d0c); }
 let mut pc: u32 = 0x15002d0c;
 loop { match pc {
 0x15002d0c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002d1c); f_1500ccde(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002d20); }
 let mut pc: u32 = 0x15002d20;
 loop { match pc {
 0x15002d20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002d30); f_1500ccdf(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002d34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002d34); }
 let mut pc: u32 = 0x15002d34;
 loop { match pc {
 0x15002d34 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002d46); f_1500764a(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002d48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002d48); }
 let mut pc: u32 = 0x15002d48;
 loop { match pc {
 0x15001fe0 => {
  { let v: u16 = 0x1u16; cpu.w16(0x15010018u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002d48 => {
  pc = 0x15001fe0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002d50); }
 let mut pc: u32 = 0x15002d50;
 loop { match pc {
 0x15001fec => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x15010018u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002d50 => {
  pc = 0x15001fec; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002d58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002d58); }
 let mut pc: u32 = 0x15002d58;
 loop { match pc {
 0x15002d58 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002d78); f_150075f6(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002d7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002d7c); }
 let mut pc: u32 = 0x15002d7c;
 loop { match pc {
 0x15002d7c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x15002d88); f_150075c4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002d8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002d8c); }
 let mut pc: u32 = 0x15002d8c;
 loop { match pc {
 0x15002d8c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002da0); f_15007837(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002da4); }
 let mut pc: u32 = 0x15002da4;
 loop { match pc {
 0x15002da4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15002dac); f_1500c648(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002db0); }
 let mut pc: u32 = 0x15002db0;
 loop { match pc {
 0x15002db0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x15002db8); f_1500c746(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002dbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002dbc); }
 let mut pc: u32 = 0x15002dbc;
 loop { match pc {
 0x15002dbc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002dd0); f_1500e045(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  {  }
  f_15002dd4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002dd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002dd4); }
 let mut pc: u32 = 0x15002dd4;
 loop { match pc {
 0x15002dd4 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002dd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002dd8); }
 let mut pc: u32 = 0x15002dd8;
 loop { match pc {
 0x15002dd8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002dec); f_150090c8(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002df0); }
 let mut pc: u32 = 0x15002df0;
 loop { match pc {
 0x15002df0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002dfe); f_15007665(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e00); }
 let mut pc: u32 = 0x15002e00;
 loop { match pc {
 0x15002e00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002e13); f_1500cc79(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e18); }
 let mut pc: u32 = 0x15002e18;
 loop { match pc {
 0x15002e18 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x15002e24); f_1500fa11(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e28); }
 let mut pc: u32 = 0x15002e28;
 loop { match pc {
 0x15002e28 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002e3c); f_1500e49c(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e40); }
 let mut pc: u32 = 0x15002e40;
 loop { match pc {
 0x15002e40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002e4a); f_1500fab0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e4c); }
 let mut pc: u32 = 0x15002e4c;
 loop { match pc {
 0x15002e4c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002e5b); f_1500fabf(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e60); }
 let mut pc: u32 = 0x15002e60;
 loop { match pc {
 0x15002e60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002e73); f_1500fac7(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e78); }
 let mut pc: u32 = 0x15002e78;
 loop { match pc {
 0x15002e78 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002e86); f_1500fad9(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e88); }
 let mut pc: u32 = 0x15002e88;
 loop { match pc {
 0x15002e88 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x1500faed; continue;
 }
 0x1500faed => {
  { let v: u32 = 0x40u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002e94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002e94); }
 let mut pc: u32 = 0x15002e94;
 loop { match pc {
 0x15002e94 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x1500faf1; continue;
 }
 0x1500faf1 => {
  { let v: u32 = cpu.or32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x80000000u32); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ea0); }
 let mut pc: u32 = 0x15002ea0;
 loop { match pc {
 0x15002ea0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x1500faf9; continue;
 }
 0x1500faf9 => {
  { let v: u32 = cpu.and32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x7fffffffu32); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002eac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002eac); }
 let mut pc: u32 = 0x15002eac;
 loop { match pc {
 0x15002eac => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002ebe); f_1500a9a9(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ec0); }
 let mut pc: u32 = 0x15002ec0;
 loop { match pc {
 0x15002ec0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002ecc); f_1500c940(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ed0); }
 let mut pc: u32 = 0x15002ed0;
 loop { match pc {
 0x15002ed0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002edc); f_1500c9eb(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ee0); }
 let mut pc: u32 = 0x15002ee0;
 loop { match pc {
 0x15002ee0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002eec); f_1500ca96(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ef0); }
 let mut pc: u32 = 0x15002ef0;
 loop { match pc {
 0x15002ef0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  f_15001948(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002efc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002efc); }
 let mut pc: u32 = 0x15002efc;
 loop { match pc {
 0x15002234 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x1500223d); f_150021d8(cpu);
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1500227c; continue; }
  pc = 0x15002242; continue;
 }
 0x15002242 => {
  { cpu.sub32(cpu.r32(0x15082068u32), 0x1u32); }
  if cpu.zf { pc = 0x1500224d; continue; }
  pc = 0x1500224b; continue;
 }
 0x1500224b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1500224d; continue;
 }
 0x1500224d => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1500225f); f_1500ff24(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500226d; continue; }
  pc = 0x15002263; continue;
 }
 0x15002263 => {
  cpu.push(0x15002268); f_1500ff1e(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500226d => {
  cpu.push(0x15002272); f_1500204c(cpu);
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1500227c; continue; }
  pc = 0x15002277; continue;
 }
 0x15002277 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1500227c; continue;
 }
 0x1500227c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002efc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x15002234; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f08); }
 let mut pc: u32 = 0x15002f08;
 loop { match pc {
 0x15002f08 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  cpu.push(0x15002f2c); f_1500263c(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f30); }
 let mut pc: u32 = 0x15002f30;
 loop { match pc {
 0x150026e0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x150026e8); f_1500ff90(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002f30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x150026e0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f3c); }
 let mut pc: u32 = 0x15002f3c;
 loop { match pc {
 0x15002718 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x1501194eu32), 0x1u16); }
  if !cpu.zf { pc = 0x1500272b; continue; }
  pc = 0x15002724; continue;
 }
 0x15002724 => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1500272b); f_1500ff60(cpu);
  pc = 0x1500272b; continue;
 }
 0x1500272b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002f3c => {
  pc = 0x15002718; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f44); }
 let mut pc: u32 = 0x15002f44;
 loop { match pc {
 0x15002730 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x1501194eu32), 0x1u16); }
  if !cpu.zf { pc = 0x15002743; continue; }
  pc = 0x1500273c; continue;
 }
 0x1500273c => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15002743); f_1500ff60(cpu);
  pc = 0x15002743; continue;
 }
 0x15002743 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002f44 => {
  pc = 0x15002730; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f4c); }
 let mut pc: u32 = 0x15002f4c;
 loop { match pc {
 0x15002f4c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x15002f67); f_15002760(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f6c); }
 let mut pc: u32 = 0x15002f6c;
 loop { match pc {
 0x15002748 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x15002751); f_1500ff72(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002f6c => {
  pc = 0x15002748; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f74); }
 let mut pc: u32 = 0x15002f74;
 loop { match pc {
 0x15002754 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500275d); f_1500ff72(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002f74 => {
  pc = 0x15002754; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f7c); }
 let mut pc: u32 = 0x15002f7c;
 loop { match pc {
 0x15002280 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x15011950u32), 0x0u16); }
  if cpu.zf { pc = 0x150022a1; continue; }
  pc = 0x15002295; continue;
 }
 0x15002295 => {
  cpu.push(0x1500229a); f_15002000(cpu);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15002318; continue;
 }
 0x150022a1 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x15011950u32, v); }
  cpu.push(0x150022b4); f_1500ff72(cpu);
  { cpu.sub16(cpu.r16(0x1501194eu32), 0x1u16); }
  if !cpu.zf { pc = 0x150022c5; continue; }
  pc = 0x150022be; continue;
 }
 0x150022be => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150022c5); f_1500ff60(cpu);
  pc = 0x150022c5; continue;
 }
 0x150022c5 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150022cc); f_1500ff66(cpu);
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x150022e0); f_1500ff18(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150022e6; continue; }
  pc = 0x150022e4; continue;
 }
 0x150022e4 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x150022e6; continue;
 }
 0x150022e6 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x15002307; continue; }
  pc = 0x150022ea; continue;
 }
 0x150022ea => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x150022f9); f_1500ff18(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15002307; continue; }
  pc = 0x150022fd; continue;
 }
 0x150022fd => {
  { let v: u32 = cpu.eax ^ cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002307 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x15002318; continue;
 }
 0x15002318 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002f7c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x15002280; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f8c); }
 let mut pc: u32 = 0x15002f8c;
 loop { match pc {
 0x15002320 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x15011950u32), 0x0u16); }
  if !cpu.zf { pc = 0x15002336; continue; }
  pc = 0x1500232d; continue;
 }
 0x1500232d => {
  cpu.push(0x15002332); f_15002000(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002336 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x15011950u32, v); }
  cpu.push(0x15002348); f_1500ff12(cpu);
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x1500234f); f_1500ff66(cpu);
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x15002356); f_1500ff72(cpu);
  { cpu.sub16(cpu.r16(0x1501194eu32), 0x1u16); }
  if !cpu.zf { pc = 0x15002367; continue; }
  pc = 0x15002360; continue;
 }
 0x15002360 => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x15002367); f_1500ff60(cpu);
  pc = 0x15002367; continue;
 }
 0x15002367 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002f8c => {
  pc = 0x15002320; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002f94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002f94); }
 let mut pc: u32 = 0x15002f94;
 loop { match pc {
 0x1500236c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15002388); f_1500ff18(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15002390; continue; }
  pc = 0x1500238c; continue;
 }
 0x1500238c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x150023a1; continue;
 }
 0x15002390 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x150023a1; continue;
 }
 0x150023a1 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002f94 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x1500236c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002fa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002fa4); }
 let mut pc: u32 = 0x15002fa4;
 loop { match pc {
 0x150023a8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150023b3); f_1500ff12(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002fa4 => {
  pc = 0x150023a8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002fac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002fac); }
 let mut pc: u32 = 0x15002fac;
 loop { match pc {
 0x150023b8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150023d4); f_1500ff18(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150023dc; continue; }
  pc = 0x150023d8; continue;
 }
 0x150023d8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15002407; continue;
 }
 0x150023dc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010fb4u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010fb8u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x150023f8; continue; }
  pc = 0x150023f2; continue;
 }
 0x150023f2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x150023f8; continue;
 }
 0x150023f8 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x15002402; continue; }
  pc = 0x150023fc; continue;
 }
 0x150023fc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x15002402; continue;
 }
 0x15002402 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x15002407; continue;
 }
 0x15002407 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002fac => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  pc = 0x150023b8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002fbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002fbc); }
 let mut pc: u32 = 0x15002fbc;
 loop { match pc {
 0x15002410 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500241b); f_1500ff12(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002fbc => {
  pc = 0x15002410; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002fc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002fc4); }
 let mut pc: u32 = 0x15002fc4;
 loop { match pc {
 0x15002420 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x15002429); f_1500ff0c(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002fc4 => {
  pc = 0x15002420; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002fcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002fcc); }
 let mut pc: u32 = 0x15002fcc;
 loop { match pc {
 0x1500242c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x15002435); f_1500ff0c(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002fcc => {
  pc = 0x1500242c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002fd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002fd4); }
 let mut pc: u32 = 0x15002fd4;
 loop { match pc {
 0x15001994 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1500199e); f_15001948(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x150019a7; continue; }
  pc = 0x150019a2; continue;
 }
 0x150019a2 => {
  cpu.push(0x150019a7); f_1500cb41(cpu);
  pc = 0x150019a7; continue;
 }
 0x150019a7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15002fd4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x15001994; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002fe0); }
 let mut pc: u32 = 0x15002fe0;
 loop { match pc {
 0x15002fe0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15002ff4); f_15007670(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15002ff8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15002ff8); }
 let mut pc: u32 = 0x15002ff8;
 loop { match pc {
 0x15002ff8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x1500905d; continue;
 }
 0x1500905d => {
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x26u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003004(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003004); }
 let mut pc: u32 = 0x15003004;
 loop { match pc {
 0x15002a08 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15002a0f); f_1500ff1e(cpu);
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1501194cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15003004 => {
  pc = 0x15002a08; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500300c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500300c); }
 let mut pc: u32 = 0x1500300c;
 loop { match pc {
 0x150025ad => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150025b0 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x15010b24u32)); }
  if cpu.zf { pc = 0x150025ad; continue; }
  pc = 0x150025b8; continue;
 }
 0x150025b8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010b24u32, v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  f_150025c0(cpu); return;
 }
 0x1500300c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x150025b0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003018(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003018); }
 let mut pc: u32 = 0x15003018;
 loop { match pc {
 0x15003018 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x1500302c); f_1500e490(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003030); }
 let mut pc: u32 = 0x15003030;
 loop { match pc {
 0x15003030 => {
  pc = 0x150030d4; continue;
 }
 0x150030d4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500321a; continue; }
  pc = 0x150030ed; continue;
 }
 0x150030ed => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = cpu.r32(0x150030c4u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x150030f7 => {
  { let v: u32 = cpu.eax ^ cpu.edi; cpu.eax = v; }
  cpu.push(0x150030fe); f_150034b4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500321c; continue; }
  pc = 0x15003106; continue;
 }
 0x15003106 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1500310e); f_150036fc(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x1500321a; continue;
 }
 0x15003115 => {
  { let v: u32 = cpu.r32(0x15094730u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15094730u32, v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15003139; continue; }
  pc = 0x15003127; continue;
 }
 0x15003127 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1500312e); f_15003715(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003139; continue; }
  pc = 0x15003132; continue;
 }
 0x15003132 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x1500321a; continue;
 }
 0x15003139 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x15003143); f_150037e6(cpu);
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x15003151); f_1500389c(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003132; continue; }
  pc = 0x15003155; continue;
 }
 0x15003155 => {
  cpu.push(0x1500315a); f_15003468(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003132; continue; }
  pc = 0x1500315e; continue;
 }
 0x1500315e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x15003165); f_150034b4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003132; continue; }
  pc = 0x15003169; continue;
 }
 0x15003169 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x15003173); f_150037e6(cpu);
  cpu.push(0x15003178); f_15003578(cpu);
  { cpu.sub32(cpu.r32(0x15016bb0u32), 0x0u32); }
  if cpu.zf { pc = 0x1500319a; continue; }
  pc = 0x15003181; continue;
 }
 0x15003181 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x15016bb0u32); cpu.push(0x1500318a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500319a; continue; }
  pc = 0x1500318e; continue;
 }
 0x1500318e => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  cpu.push(0x15003198); f_15003831(cpu);
  pc = 0x15003132; continue;
 }
 0x1500319a => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  cpu.push(0x150031a4); f_150037e6(cpu);
  cpu.push(0x150031a9); f_15003a28(cpu);
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150031b1); f_150036fc(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500321a; continue; }
  pc = 0x150031b7; continue;
 }
 0x150031b7 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x150031c1); f_15003831(cpu);
  pc = 0x1500321a; continue;
 }
 0x150031c3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150031cb); f_150036fc(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x150031d4); f_15003501(cpu);
  pc = 0x1500321a; continue;
 }
 0x150031d6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150031de); f_150036fc(cpu);
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.push(0x150031ef); f_15003831(cpu);
  { cpu.sub32(cpu.r32(0x15016bb0u32), 0x0u32); }
  if cpu.zf { pc = 0x15003201; continue; }
  pc = 0x150031f8; continue;
 }
 0x150031f8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x15016bb0u32); cpu.push(0x15003201); cpu.call(t);
  pc = 0x15003201; continue;
 }
 0x15003201 => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x1500320d); f_15003831(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x15003214); f_15003501(cpu);
  { let v: u32 = cpu.dec32(cpu.r32(0x15094730u32)); cpu.w32(0x15094730u32, v); }
  pc = 0x1500321a; continue;
 }
 0x1500321a => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x1500321c; continue;
 }
 0x1500321c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150030a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150030a4); }
 let mut pc: u32 = 0x150030a4;
 loop { match pc {
 0x150030a4 => {
  { let v = cpu.eax; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  {  }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v: u8 = 0x1fu8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x0u32.wrapping_add(cpu.esp)); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { cpu.fpu_control = cpu.r16(0x4u32.wrapping_add(cpu.esp)); }
  {  }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150030c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150030c2); }
 let mut pc: u32 = 0x150030c2;
 loop { match pc {
 0x150030c2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150030c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150030c3); }
 let mut pc: u32 = 0x150030c3;
 loop { match pc {
 0x150030c3 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150030c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150030c4); }
 let mut pc: u32 = 0x150030c4;
 loop { match pc {
 0x150030c4 => {
  cpu.unimplemented(0x150030c4, "Salc");
  { let v: u32 = cpu.xor32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x15003115u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.div32(v); }
  { let v: u8 = cpu.r8(0x150031c3u32).wrapping_add((cpu.edx as u8)); cpu.w8(0x150031c3u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500321a; continue; }
  pc = 0x150030ed; continue;
 }
 0x150030ed => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = cpu.r32(0x150030c4u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x150030f7 => {
  { let v: u32 = cpu.eax ^ cpu.edi; cpu.eax = v; }
  cpu.push(0x150030fe); f_150034b4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500321c; continue; }
  pc = 0x15003106; continue;
 }
 0x15003106 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1500310e); f_150036fc(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x1500321a; continue;
 }
 0x15003115 => {
  { let v: u32 = cpu.r32(0x15094730u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15094730u32, v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15003139; continue; }
  pc = 0x15003127; continue;
 }
 0x15003127 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1500312e); f_15003715(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003139; continue; }
  pc = 0x15003132; continue;
 }
 0x15003132 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x1500321a; continue;
 }
 0x15003139 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x15003143); f_150037e6(cpu);
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x15003151); f_1500389c(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003132; continue; }
  pc = 0x15003155; continue;
 }
 0x15003155 => {
  cpu.push(0x1500315a); f_15003468(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003132; continue; }
  pc = 0x1500315e; continue;
 }
 0x1500315e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x15003165); f_150034b4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003132; continue; }
  pc = 0x15003169; continue;
 }
 0x15003169 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x15003173); f_150037e6(cpu);
  cpu.push(0x15003178); f_15003578(cpu);
  { cpu.sub32(cpu.r32(0x15016bb0u32), 0x0u32); }
  if cpu.zf { pc = 0x1500319a; continue; }
  pc = 0x15003181; continue;
 }
 0x15003181 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x15016bb0u32); cpu.push(0x1500318a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500319a; continue; }
  pc = 0x1500318e; continue;
 }
 0x1500318e => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  cpu.push(0x15003198); f_15003831(cpu);
  pc = 0x15003132; continue;
 }
 0x1500319a => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  cpu.push(0x150031a4); f_150037e6(cpu);
  cpu.push(0x150031a9); f_15003a28(cpu);
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150031b1); f_150036fc(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500321a; continue; }
  pc = 0x150031b7; continue;
 }
 0x150031b7 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x150031c1); f_15003831(cpu);
  pc = 0x1500321a; continue;
 }
 0x150031c3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150031cb); f_150036fc(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x150031d4); f_15003501(cpu);
  pc = 0x1500321a; continue;
 }
 0x150031d6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150031de); f_150036fc(cpu);
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.push(0x150031ef); f_15003831(cpu);
  { cpu.sub32(cpu.r32(0x15016bb0u32), 0x0u32); }
  if cpu.zf { pc = 0x15003201; continue; }
  pc = 0x150031f8; continue;
 }
 0x150031f8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x15016bb0u32); cpu.push(0x15003201); cpu.call(t);
  pc = 0x15003201; continue;
 }
 0x15003201 => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x1500320d); f_15003831(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x15003214); f_15003501(cpu);
  { let v: u32 = cpu.dec32(cpu.r32(0x15094730u32)); cpu.w32(0x15094730u32, v); }
  pc = 0x1500321a; continue;
 }
 0x1500321a => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x1500321c; continue;
 }
 0x1500321c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003224); }
 let mut pc: u32 = 0x15003224;
 loop { match pc {
 0x15003224 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x15003a34u32; cpu.edx = v; }
  { let v: u32 = 0x15003b7du32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15016bb4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15016bb8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500323f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500323f); }
 let mut pc: u32 = 0x1500323f;
 loop { match pc {
 0x1500323f => {
  {  }
  cpu.unimplemented(0x15003240, "Fnsave");
  {  }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003244); }
 let mut pc: u32 = 0x15003244;
 loop { match pc {
 0x15003244 => {
  cpu.unimplemented(0x15003244, "Frstor");
  {  }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003248(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003248); }
 let mut pc: u32 = 0x15003248;
 loop { match pc {
 0x15003248 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15016b0du32), 0x0u8); }
  if cpu.zf { pc = 0x15003269; continue; }
  pc = 0x15003253; continue;
 }
 0x15003253 => {
  { let v: u32 = 0x1500323fu32; cpu.edx = v; }
  { let v: u32 = 0x15003244u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15016bbcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15016bc0u32, v); }
  pc = 0x15003269; continue;
 }
 0x15003269 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x15016bc4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x15003276); f_15003b9e(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003279(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003279); }
 let mut pc: u32 = 0x15003279;
 loop { match pc {
 0x15003279 => {
  { cpu.sub8(cpu.r8(0x15016b0du32), 0x0u8); }
  if !cpu.zf { f_15003248(cpu); return; }
  pc = 0x15003282; continue;
 }
 0x15003282 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003283(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003283); }
 let mut pc: u32 = 0x15003283;
 loop { match pc {
 0x15003283 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x15016b0cu32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x150032cb; continue; }
  pc = 0x1500328e; continue;
 }
 0x1500328e => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x15016b0du32, v); }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x150032b6; continue; }
  pc = 0x150032ad; continue;
 }
 0x150032ad => {
  cpu.push(0x150032b2); f_15003248(cpu);
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x150032b6; continue;
 }
 0x150032b6 => {
  { cpu.sub8(cpu.r8(0x15016b44u32), 0x0u8); }
  if !cpu.zf { pc = 0x150032cb; continue; }
  pc = 0x150032bf; continue;
 }
 0x150032bf => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x15016b0cu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x15016b0du32, v); }
  pc = 0x150032cb; continue;
 }
 0x150032cb => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150032cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150032cd); }
 let mut pc: u32 = 0x150032cd;
 loop { match pc {
 0x150032cd => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150032ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150032ce); }
 let mut pc: u32 = 0x150032ce;
 loop { match pc {
 0x150032ce => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150032cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150032cf); }
 let mut pc: u32 = 0x150032cf;
 loop { match pc {
 0x150032cf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x150032e2; continue; }
  pc = 0x150032da; continue;
 }
 0x150032da => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x150032e2); f_1500fc30(cpu);
  pc = 0x150032e2; continue;
 }
 0x150032e2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150032fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150032fb); }
 let mut pc: u32 = 0x150032fb;
 loop { match pc {
 0x150032fb => {
  { let v: u32 = 0x15094734u32; cpu.eax = v; }
  f_15003382(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003305(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003305); }
 let mut pc: u32 = 0x15003305;
 loop { match pc {
 0x15003305 => {
  { let v: u32 = 0x15094734u32; cpu.eax = v; }
  f_150033e4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500330f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500330f); }
 let mut pc: u32 = 0x1500330f;
 loop { match pc {
 0x1500330f => {
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x15094754u32); cpu.eax = v; }
  f_15003382(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500331c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500331c); }
 let mut pc: u32 = 0x1500331c;
 loop { match pc {
 0x1500331c => {
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x15094754u32); cpu.eax = v; }
  f_150033e4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500332c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500332c); }
 let mut pc: u32 = 0x1500332c;
 loop { match pc {
 0x1500332c => {
  f_15003c0a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003331(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003331); }
 let mut pc: u32 = 0x15003331;
 loop { match pc {
 0x15003331 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x15094754u32); cpu.eax = v; }
  cpu.push(0x15003344); f_150032cf(cpu);
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1500334b); f_15003d1f(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500334d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500334d); }
 let mut pc: u32 = 0x1500334d;
 loop { match pc {
 0x1500334d => {
  { let v: u32 = 0x15094854u32; cpu.eax = v; }
  f_15003382(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003354(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003354); }
 let mut pc: u32 = 0x15003354;
 loop { match pc {
 0x15003354 => {
  { let v: u32 = 0x15094854u32; cpu.eax = v; }
  f_150033e4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500335e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500335e); }
 let mut pc: u32 = 0x1500335e;
 loop { match pc {
 0x1500335e => {
  { let v: u32 = 0x15094744u32; cpu.eax = v; }
  f_15003382(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003365(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003365); }
 let mut pc: u32 = 0x15003365;
 loop { match pc {
 0x15003365 => {
  { let v: u32 = 0x15094744u32; cpu.eax = v; }
  f_150033e4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500336f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500336f); }
 let mut pc: u32 = 0x1500336f;
 loop { match pc {
 0x1500336f => {
  { let v: u32 = 0x15094874u32; cpu.eax = v; }
  f_15003382(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003376(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003376); }
 let mut pc: u32 = 0x15003376;
 loop { match pc {
 0x15003376 => {
  { let v: u32 = 0x15094874u32; cpu.eax = v; }
  f_150033e4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500337d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500337d); }
 let mut pc: u32 = 0x1500337d;
 loop { match pc {
 0x1500337d => {
  { let v: u32 = 0x15094884u32; cpu.eax = v; }
  f_15003382(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003382(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003382); }
 let mut pc: u32 = 0x15003382;
 loop { match pc {
 0x15003382 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x1500338e); f_1500fc2a(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x150033d6; continue; }
  pc = 0x15003397; continue;
 }
 0x15003397 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x150033c9; continue; }
  pc = 0x1500339e; continue;
 }
 0x1500339e => {
  { let v: u32 = 0x15094864u32; cpu.eax = v; }
  cpu.push(0x150033a8); f_15003382(cpu);
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x150033bf; continue; }
  pc = 0x150033ae; continue;
 }
 0x150033ae => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x150033b6); f_1500fc24(cpu);
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x150033bf; continue;
 }
 0x150033bf => {
  { let v: u32 = 0x15094864u32; cpu.eax = v; }
  cpu.push(0x150033c9); f_150033e4(cpu);
  pc = 0x150033c9; continue;
 }
 0x150033c9 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x150033d3); f_1500fc1e(cpu);
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x150033d6; continue;
 }
 0x150033d6 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xcu32.wrapping_add(cpu.ebx))); cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150033df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150033df); }
 let mut pc: u32 = 0x150033df;
 loop { match pc {
 0x150033df => {
  { let v: u32 = 0x15094884u32; cpu.eax = v; }
  f_150033e4(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150033e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150033e4); }
 let mut pc: u32 = 0x150033e4;
 loop { match pc {
 0x150033e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x15003404; continue; }
  pc = 0x150033ef; continue;
 }
 0x150033ef => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x15003404; continue; }
  pc = 0x150033f9; continue;
 }
 0x150033f9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x15003404); f_1500fc18(cpu);
  pc = 0x15003404; continue;
 }
 0x15003404 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003409(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003409); }
 let mut pc: u32 = 0x15003409;
 loop { match pc {
 0x15003409 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15003417); f_1500fc12(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15003424; continue; }
  pc = 0x1500341d; continue;
 }
 0x1500341d => {
  cpu.push(0x15003422); f_15003d94(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x15003424; continue;
 }
 0x15003424 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003429(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003429); }
 let mut pc: u32 = 0x15003429;
 loop { match pc {
 0x15003429 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500344c; continue; }
  pc = 0x15003432; continue;
 }
 0x15003432 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15016bccu32); cpu.edx = v; }
  cpu.push(0x15003442); f_15003eb4(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500344c; continue; }
  pc = 0x15003448; continue;
 }
 0x15003448 => {
  { let v: u8 = 0x1u8; cpu.w8(0x52u32.wrapping_add(cpu.eax), v); }
  pc = 0x1500344c; continue;
 }
 0x1500344c => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x15003462; continue; }
  pc = 0x15003450; continue;
 }
 0x15003450 => {
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x1500345c); f_1500fc2a(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xdau32.wrapping_add(cpu.ebx), v); }
  pc = 0x15003462; continue;
 }
 0x15003462 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003468(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003468); }
 let mut pc: u32 = 0x15003468;
 loop { match pc {
 0x15003468 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1500346f); f_1500fc0c(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x15016b64u32, v); }
  { let v: u16 = cpu.r16(0x15016b51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x150034a2; continue; }
  pc = 0x15003486; continue;
 }
 0x15003486 => {
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x150034a2; continue; }
  pc = 0x15003491; continue;
 }
 0x15003491 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x150034a2; continue; }
  pc = 0x15003496; continue;
 }
 0x15003496 => {
  cpu.push(0x1500349b); f_1500fc0c(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x15016b64u32, v); }
  pc = 0x15003486; continue;
 }
 0x150034a2 => {
  { cpu.sub32(cpu.r32(0x15016b64u32), 0xffffffffu32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150034b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150034b4); }
 let mut pc: u32 = 0x150034b4;
 loop { match pc {
 0x150034b4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x15016b64u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x150034c6; continue; }
  pc = 0x150034c0; continue;
 }
 0x150034c0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150034c6 => {
  cpu.push(0x150034cb); f_15003429(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150034fd; continue; }
  pc = 0x150034d1; continue;
 }
 0x150034d1 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xdau32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x150034de); f_15003dcb(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150034eb; continue; }
  pc = 0x150034e2; continue;
 }
 0x150034e2 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x150034e9); f_15003ecc(cpu);
  pc = 0x150034c0; continue;
 }
 0x150034eb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x150034f8); f_1500fc06(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x150034fd; continue;
 }
 0x150034fd => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003501(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003501); }
 let mut pc: u32 = 0x15003501;
 loop { match pc {
 0x15003501 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x15003549; continue; }
  pc = 0x15003512; continue;
 }
 0x15003512 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15003518); f_1500fc12(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003549; continue; }
  pc = 0x1500351c; continue;
 }
 0x1500351c => {
  { let v: u32 = cpu.r32(0xdeu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xdau32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x1500352d); f_15003e2c(cpu);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1500353b); f_1500fc06(cpu);
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x15003549; continue; }
  pc = 0x1500353f; continue;
 }
 0x1500353f => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x15003549; continue; }
  pc = 0x15003543; continue;
 }
 0x15003543 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15003549); f_1500fc30(cpu);
  pc = 0x15003549; continue;
 }
 0x15003549 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500354e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500354e); }
 let mut pc: u32 = 0x1500354e;
 loop { match pc {
 0x1500354e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x15003558); f_15003501(cpu);
  f_15003558(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003558(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003558); }
 let mut pc: u32 = 0x15003558;
 loop { match pc {
 0x15003558 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x15003575; continue; }
  pc = 0x15003565; continue;
 }
 0x15003565 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1500356b); f_1500fc00(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0x15016b64u32, v); }
  pc = 0x15003575; continue;
 }
 0x15003575 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003578(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003578); }
 let mut pc: u32 = 0x15003578;
 loop { match pc {
 0x15003578 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1500330fu32; cpu.edx = v; }
  { let v: u32 = 0x1500331cu32; cpu.ebx = v; }
  { let v: u32 = 0x1500332cu32; cpu.ecx = v; }
  { let v: u32 = 0x15003331u32; cpu.esi = v; }
  { let v: u32 = 0x150032fbu32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x15003305u32; cpu.ebp = v; }
  { let v: u32 = 0x15003382u32; cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15016b6cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15016b70u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15016b74u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15016b78u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15016b7cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x15016b80u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15016b58u32, v); }
  { let v: u32 = 0x150033e4u32; cpu.edx = v; }
  { let v: u32 = 0x150032cfu32; cpu.ebx = v; }
  { let v: u32 = 0x1500334du32; cpu.ecx = v; }
  { let v: u32 = 0x1500335eu32; cpu.esi = v; }
  { let v: u32 = 0x15003354u32; cpu.edi = v; }
  { let v: u32 = 0x15003365u32; cpu.ebp = v; }
  { let v: u32 = 0x1500336fu32; cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15016b5cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15016b60u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15016b84u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15016b88u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15016b8cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x15016b90u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15016b94u32, v); }
  { let v: u32 = 0x15003376u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x150033dfu32; cpu.esi = v; }
  { let v: u32 = 0x1500354eu32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15016b98u32, v); }
  cpu.push(0x1500363b); f_1500fc24(cpu);
  { let v: u32 = 0x1500337du32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094c88u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15094864u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15094868u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15016ba0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15016ba4u32, v); }
  { let v: u32 = cpu.r32(0xdau32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15016b9cu32, v); }
  cpu.push(0x1500366e); f_15003dcb(cpu);
  { let v: u32 = cpu.r32(0x15094c88u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15003680); f_1500fc06(cpu);
  { let v: u32 = 0x15003409u32; cpu.w32(0x15016b68u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003691(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003691); }
 let mut pc: u32 = 0x15003691;
 loop { match pc {
 0x15003691 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x15094854u32; cpu.eax = v; }
  let t = cpu.r32(0x15016b60u32); cpu.push(0x1500369e); cpu.call(t);
  { let v: u32 = 0x15094744u32; cpu.eax = v; }
  { let v: u32 = 0x15094754u32; cpu.edx = v; }
  let t = cpu.r32(0x15016b60u32); cpu.push(0x150036ae); cpu.call(t);
  { let v: u32 = 0x15094734u32; cpu.eax = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  let t = cpu.r32(0x15016b60u32); cpu.push(0x150036bf); cpu.call(t);
  pc = 0x150036bf; continue;
 }
 0x150036bf => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  let t = cpu.r32(0x15016b60u32); cpu.push(0x150036ca); cpu.call(t);
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if !cpu.zf { pc = 0x150036bf; continue; }
  pc = 0x150036ce; continue;
 }
 0x150036ce => {
  { let v: u32 = 0x15094884u32; cpu.eax = v; }
  let t = cpu.r32(0x15016b60u32); cpu.push(0x150036d9); cpu.call(t);
  cpu.push(0x150036de); f_15003f5a(cpu);
  { let v: u32 = 0x15094874u32; cpu.eax = v; }
  let t = cpu.r32(0x15016b60u32); cpu.push(0x150036e9); cpu.call(t);
  { let v: u32 = 0x15094864u32; cpu.eax = v; }
  let t = cpu.r32(0x15016b60u32); cpu.push(0x150036f4); cpu.call(t);
  cpu.push(0x150036f9); f_15003558(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150036fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150036fc); }
 let mut pc: u32 = 0x150036fc;
 loop { match pc {
 0x150036fc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x15003711); f_15003f5f(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003715(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003715); }
 let mut pc: u32 = 0x15003715;
 loop { match pc {
 0x15003715 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let v = 0x15094a7cu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1500372d); f_1500fbfa(cpu);
  { let v = 0x104u32; cpu.push(v); }
  { let v = 0x15094b80u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x150174b0u32; cpu.esi = v; }
  cpu.push(0x15003742); f_1500fbfa(cpu);
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x15094894u32; cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x15094894u32, v); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x1500375b; continue;
 }
 0x1500375b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x15003773; continue; }
  pc = 0x15003763; continue;
 }
 0x15003763 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x1500375b; continue; }
  pc = 0x15003773; continue;
 }
 0x15003773 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.pop(); }
  { let v: u32 = 0x15094a7cu32; cpu.esi = v; }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x15003786; continue;
 }
 0x15003786 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1500379e; continue; }
  pc = 0x1500378e; continue;
 }
 0x1500378e => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x15003786; continue; }
  pc = 0x1500379e; continue;
 }
 0x1500379e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.pop(); }
  { let v: u32 = 0x150174e1u32; cpu.esi = v; }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x150037b1; continue;
 }
 0x150037b1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x150037c9; continue; }
  pc = 0x150037b9; continue;
 }
 0x150037b9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x150037b1; continue; }
  pc = 0x150037c9; continue;
 }
 0x150037c9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.pop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x15094b80u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150037da); f_1500fbf4(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150037e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150037e6); }
 let mut pc: u32 = 0x150037e6;
 loop { match pc {
 0x150037e6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(0x2b); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x150037ed; continue;
 }
 0x150037ed => {
  { let v: u32 = 0x15017ac8u32; cpu.esi = v; }
  { let v: u32 = 0x15017ae6u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x150037fd; continue;
 }
 0x150037fd => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x15003815; continue; }
  pc = 0x15003801; continue;
 }
 0x15003801 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x15003810; continue; }
  pc = 0x15003806; continue;
 }
 0x15003806 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15003810; continue; }
  pc = 0x1500380b; continue;
 }
 0x1500380b => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x15003810; continue;
 }
 0x15003810 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  pc = 0x150037fd; continue;
 }
 0x15003815 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x1500382b; continue; }
  pc = 0x15003819; continue;
 }
 0x15003819 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x15003826; continue; }
  pc = 0x15003820; continue;
 }
 0x15003820 => {
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.push(0x15003825); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x15003826; continue;
 }
 0x15003826 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x150037ed; continue;
 }
 0x1500382b => {
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003831(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003831); }
 let mut pc: u32 = 0x15003831;
 loop { match pc {
 0x15003831 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(0x2b); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x15003839; continue;
 }
 0x15003839 => {
  { let v: u32 = 0x15017ae6u32; cpu.esi = v; }
  { let v: u32 = 0x15017af8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x15003849; continue;
 }
 0x15003849 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x15003861; continue; }
  pc = 0x1500384d; continue;
 }
 0x1500384d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1500385c; continue; }
  pc = 0x15003852; continue;
 }
 0x15003852 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500385c; continue; }
  pc = 0x15003857; continue;
 }
 0x15003857 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1500385c; continue;
 }
 0x1500385c => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  pc = 0x15003849; continue;
 }
 0x15003861 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x1500387b; continue; }
  pc = 0x15003865; continue;
 }
 0x15003865 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15003876; continue; }
  pc = 0x15003869; continue;
 }
 0x15003869 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x15003876; continue; }
  pc = 0x15003870; continue;
 }
 0x15003870 => {
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.push(0x15003875); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x15003876; continue;
 }
 0x15003876 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x15003839; continue;
 }
 0x1500387b => {
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500388c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500388c); }
 let mut pc: u32 = 0x1500388c;
 loop { match pc {
 0x1500388c => {
  { let v: u32 = cpu.r32(0x15094c88u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003892(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003892); }
 let mut pc: u32 = 0x15003892;
 loop { match pc {
 0x15003892 => {
  f_15003c0a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003897(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003897); }
 let mut pc: u32 = 0x15003897;
 loop { match pc {
 0x15003897 => {
  f_15003d1f(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500389c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500389c); }
 let mut pc: u32 = 0x1500389c;
 loop { match pc {
 0x1500389c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x104u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x15094c84u32, v); }
  cpu.push(0x150038b3); f_15003429(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x15094c88u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150038ce; continue; }
  pc = 0x150038bc; continue;
 }
 0x150038bc => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x150038c7; continue; }
  pc = 0x150038c0; continue;
 }
 0x150038c0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x150039b8; continue;
 }
 0x150038c7 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x150038ce); f_1500fbee(cpu);
  pc = 0x150038ce; continue;
 }
 0x150038ce => {
  cpu.push(0x150038d3); f_15003d4b(cpu);
  cpu.push(0x150038d8); f_1500fbe8(cpu);
  { let v = 0x104u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15016b49u32, v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x150038ee); f_1500fbfa(cpu);
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x150038f5); f_15004668(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x15016b1cu32, v); }
  cpu.push(0x150038ff); f_1500fbe2(cpu);
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.push(0x15003906); f_15004668(cpu);
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x15003925; continue; }
  pc = 0x1500390f; continue;
 }
 0x1500390f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x22u32); }
  if cpu.zf { pc = 0x1500391d; continue; }
  pc = 0x15003919; continue;
 }
 0x15003919 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1500390f; continue; }
  pc = 0x1500391d; continue;
 }
 0x1500391d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x15003948; continue; }
  pc = 0x15003922; continue;
 }
 0x15003922 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x15003948; continue;
 }
 0x15003925 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15017930u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x15003948; continue; }
  pc = 0x15003940; continue;
 }
 0x15003940 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x15003948; continue; }
  pc = 0x15003945; continue;
 }
 0x15003945 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x15003925; continue;
 }
 0x15003948 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15017930u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x15003922; continue; }
  pc = 0x15003963; continue;
 }
 0x15003963 => {
  { let v: u32 = cpu.eax; cpu.w32(0x15016b18u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x15003988; continue; }
  pc = 0x1500396c; continue;
 }
 0x1500396c => {
  { let v = 0x104u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1500397c); f_1500fbfa(cpu);
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x15003983); f_15004668(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x15016b20u32, v); }
  pc = 0x15003988; continue;
 }
 0x15003988 => {
  cpu.push(0x1500398d); f_1500fbdc(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x15016b51u32, v); }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x15016b4fu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15016b50u32, v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x150039b8; continue;
 }
 0x150039b8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x104u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003a28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003a28); }
 let mut pc: u32 = 0x15003a28;
 loop { match pc {
 0x15003a28 => {
  { let v: u32 = 0x8000u32; cpu.w32(0x15016c3cu32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003a34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003a34); }
 let mut pc: u32 = 0x15003a34;
 loop { match pc {
 0x15003a34 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x2bu32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  if cpu.cf { pc = 0x15003a73; continue; }
  pc = 0x15003a56; continue;
 }
 0x15003a56 => {
  if (cpu.cf || cpu.zf) { pc = 0x15003a80; continue; }
  pc = 0x15003a58; continue;
 }
 0x15003a58 => {
  { cpu.sub8((cpu.edx as u8), 0x66u8); }
  if cpu.cf { pc = 0x15003a69; continue; }
  pc = 0x15003a5d; continue;
 }
 0x15003a5d => {
  if (cpu.cf || cpu.zf) { pc = 0x15003a95; continue; }
  pc = 0x15003a5f; continue;
 }
 0x15003a5f => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.zf { pc = 0x15003a80; continue; }
  pc = 0x15003a64; continue;
 }
 0x15003a64 => {
  pc = 0x15003b12; continue;
 }
 0x15003a69 => {
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.zf { pc = 0x15003a90; continue; }
  pc = 0x15003a6e; continue;
 }
 0x15003a6e => {
  pc = 0x15003b12; continue;
 }
 0x15003a73 => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  if cpu.cf { pc = 0x15003b12; continue; }
  pc = 0x15003a7c; continue;
 }
 0x15003a7c => {
  if (cpu.cf || cpu.zf) { pc = 0x15003a90; continue; }
  pc = 0x15003a7e; continue;
 }
 0x15003a7e => {
  pc = 0x15003a95; continue;
 }
 0x15003a80 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x15003a8d; continue; }
  pc = 0x15003a86; continue;
 }
 0x15003a86 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x15003a8d; continue;
 }
 0x15003a8d => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x15003a90; continue;
 }
 0x15003a90 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x15003a95; continue;
 }
 0x15003a95 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x15003ab6; continue; }
  pc = 0x15003aaf; continue;
 }
 0x15003aaf => {
  { let v: u32 = 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x15003ab6; continue;
 }
 0x15003ab6 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x5fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = 0xffu32; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15003ade); f_1500482a(cpu);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2au32); }
  if !cpu.zf { pc = 0x15003aeb; continue; }
  pc = 0x15003ae7; continue;
 }
 0x15003ae7 => {
  { let v: u8 = 0x2au8; cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x15003aeb; continue;
 }
 0x15003aeb => {
  { let v: u8 = 0x0u8; cpu.w8(0xffu32.wrapping_add(cpu.esi), v); }
  pc = 0x15003af2; continue;
 }
 0x15003af2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x15003afe; continue; }
  pc = 0x15003afb; continue;
 }
 0x15003afb => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x15003af2; continue;
 }
 0x15003afe => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15003b07); f_15003b1e(cpu);
  { let v: u32 = 0x2bu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x15003b12; continue;
 }
 0x15003b12 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003b1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003b1e); }
 let mut pc: u32 = 0x15003b1e;
 loop { match pc {
 0x15003b1e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x15003b7b; continue; }
  pc = 0x15003b2f; continue;
 }
 0x15003b2f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x15003b45; continue; }
  pc = 0x15003b34; continue;
 }
 0x15003b34 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x65u32); }
  if cpu.zf { pc = 0x15003b45; continue; }
  pc = 0x15003b3d; continue;
 }
 0x15003b3d => {
  { cpu.sub32(cpu.edx, 0x45u32); }
  if cpu.zf { pc = 0x15003b45; continue; }
  pc = 0x15003b42; continue;
 }
 0x15003b42 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x15003b2f; continue;
 }
 0x15003b45 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x15003b5b; continue; }
  pc = 0x15003b56; continue;
 }
 0x15003b56 => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if !cpu.zf { pc = 0x15003b67; continue; }
  pc = 0x15003b5b; continue;
 }
 0x15003b5b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x15003b67; continue; }
  pc = 0x15003b64; continue;
 }
 0x15003b64 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x15003b5b; continue;
 }
 0x15003b67 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if cpu.zf { pc = 0x15003b71; continue; }
  pc = 0x15003b70; continue;
 }
 0x15003b70 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x15003b71; continue;
 }
 0x15003b71 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x15003b71; continue; }
  pc = 0x15003b7b; continue;
 }
 0x15003b7b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003b7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003b7d); }
 let mut pc: u32 = 0x15003b7d;
 loop { match pc {
 0x15003b7d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x15003b87); f_15004a64(cpu);
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003b8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003b8b); }
 let mut pc: u32 = 0x15003b8b;
 loop { match pc {
 0x15003b8b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x15017594u32; cpu.eax = v; }
  cpu.push(0x15003b9b); f_15004c8e(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003b9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003b9d); }
 let mut pc: u32 = 0x15003b9d;
 loop { match pc {
 0x15003b9d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003b9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003b9e); }
 let mut pc: u32 = 0x15003b9e;
 loop { match pc {
 0x15003b9e => {
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpush(1.0); }
  { cpu.fpush(0.0); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.zf { pc = 0x15003bb8; continue; }
  pc = 0x15003bb6; continue;
 }
 0x15003bb6 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x15003bb8; continue;
 }
 0x15003bb8 => {
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x0u32.wrapping_add(cpu.esp)); }
  { let a = cpu.r16(0x0u32.wrapping_add(cpu.esp)); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003c0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003c0a); }
 let mut pc: u32 = 0x15003c0a;
 loop { match pc {
 0x15003c0a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  let t = cpu.r32(0x15016b9cu32); cpu.push(0x15003c16); cpu.call(t);
  { let v: u32 = cpu.r32(0x15094c8cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x15003c45; continue;
 }
 0x15003c25 => {
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x15003c41; continue; }
  pc = 0x15003c32; continue;
 }
 0x15003c32 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  let t = cpu.r32(0x15016ba0u32); cpu.push(0x15003c3a); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15003c41 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x15003c45; continue;
 }
 0x15003c45 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15003c25; continue; }
  pc = 0x15003c49; continue;
 }
 0x15003c49 => {
  { let v: u32 = cpu.r32(0x15094c8cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  cpu.push(0x15003c5d); f_15004d15(cpu);
  { let v: u32 = cpu.r32(0x15094c8cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15094c90u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15094c8cu32, v); }
  let t = cpu.r32(0x15016ba0u32); cpu.push(0x15003c78); cpu.call(t);
  { let v: u32 = cpu.r32(0x15094c8cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003d1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003d1f); }
 let mut pc: u32 = 0x15003d1f;
 loop { match pc {
 0x15003d1f => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x15016b9cu32); cpu.push(0x15003d28); cpu.call(t);
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15003d43; continue; }
  pc = 0x15003d2c; continue;
 }
 0x15003d2c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x15094c8cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x15003d43; continue; }
  pc = 0x15003d34; continue;
 }
 0x15003d34 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  pc = 0x15003d43; continue;
 }
 0x15003d43 => {
  let t = cpu.r32(0x15016ba0u32); cpu.push(0x15003d49); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003d4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003d4b); }
 let mut pc: u32 = 0x15003d4b;
 loop { match pc {
 0x15003d4b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v = 0xfffffff6u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15094c8cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15094c90u32, v); }
  cpu.push(0x15003d62); f_1500fbca(cpu);
  cpu.push(0x15003d67); f_15003c0a(cpu);
  { let v = 0xfffffff5u32; cpu.push(v); }
  cpu.push(0x15003d6e); f_1500fbca(cpu);
  cpu.push(0x15003d73); f_15003c0a(cpu);
  { let v = 0xfffffff4u32; cpu.push(v); }
  cpu.push(0x15003d7a); f_1500fbca(cpu);
  cpu.push(0x15003d7f); f_15003c0a(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003d94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003d94); }
 let mut pc: u32 = 0x15003d94;
 loop { match pc {
 0x15003d94 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x15003d9e); f_150034b4(cpu);
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003db2; continue; }
  pc = 0x15003da4; continue;
 }
 0x15003da4 => {
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15003db0); f_1500fc12(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x15003db2; continue;
 }
 0x15003db2 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x15003dc5; continue; }
  pc = 0x15003db6; continue;
 }
 0x15003db6 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x150175b8u32; cpu.eax = v; }
  cpu.push(0x15003dc5); f_15004c8e(cpu);
  pc = 0x15003dc5; continue;
 }
 0x15003dc5 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003dcb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003dcb); }
 let mut pc: u32 = 0x15003dcb;
 loop { match pc {
 0x15003dcb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x15016b94u32); cpu.push(0x15003dd8); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x15003de9); f_15003eb4(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003e1e; continue; }
  pc = 0x15003def; continue;
 }
 0x15003def => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x15003df6); f_15004da0(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003e03; continue; }
  pc = 0x15003dfa; continue;
 }
 0x15003dfa => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15003e01); f_15003ecc(cpu);
  pc = 0x15003e1e; continue;
 }
 0x15003e03 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x52u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x15094c94u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15094c94u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x15003e20; continue;
 }
 0x15003e1e => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  pc = 0x15003e20; continue;
 }
 0x15003e20 => {
  let t = cpu.r32(0x15016b98u32); cpu.push(0x15003e26); cpu.call(t);
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003e2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003e2c); }
 let mut pc: u32 = 0x15003e2c;
 loop { match pc {
 0x15003e2c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x15016b94u32); cpu.push(0x15003e37); cpu.call(t);
  { let v: u32 = 0x15094c94u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15094c94u32); cpu.edx = v; }
  pc = 0x15003e68; continue;
 }
 0x15003e44 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x15003e64; continue; }
  pc = 0x15003e49; continue;
 }
 0x15003e49 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x15003e57; continue; }
  pc = 0x15003e4f; continue;
 }
 0x15003e4f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x15003e57); f_15003ecc(cpu);
  pc = 0x15003e57; continue;
 }
 0x15003e57 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15003e62); f_15003ecc(cpu);
  pc = 0x15003e6c; continue;
 }
 0x15003e64 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x15003e68; continue;
 }
 0x15003e68 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x15003e44; continue; }
  pc = 0x15003e6c; continue;
 }
 0x15003e6c => {
  let t = cpu.r32(0x15016b98u32); cpu.push(0x15003e72); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003eb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003eb4); }
 let mut pc: u32 = 0x15003eb4;
 loop { match pc {
 0x15003eb4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x15003ebf); f_15004de6(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003eca; continue; }
  pc = 0x15003ec3; continue;
 }
 0x15003ec3 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x15003eca); f_15004ee0(cpu);
  pc = 0x15003eca; continue;
 }
 0x15003eca => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003ecc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003ecc); }
 let mut pc: u32 = 0x15003ecc;
 loop { match pc {
 0x15003ecc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15003f54; continue; }
  pc = 0x15003edb; continue;
 }
 0x15003edb => {
  let t = cpu.r32(0x15016b84u32); cpu.push(0x15003ee1); cpu.call(t);
  { let v: u32 = cpu.r32(0x15094c98u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x15003ef4; continue; }
  pc = 0x15003eeb; continue;
 }
 0x15003eeb => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15003ef4; continue; }
  pc = 0x15003eef; continue;
 }
 0x15003eef => {
  { cpu.sub32(cpu.esi, cpu.r32(0x8u32.wrapping_add(cpu.ecx))); }
  if cpu.cf { pc = 0x15003f0e; continue; }
  pc = 0x15003ef4; continue;
 }
 0x15003ef4 => {
  { let v: u32 = cpu.r32(0x15016ca8u32); cpu.ecx = v; }
  pc = 0x15003f07; continue;
 }
 0x15003efc => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15003f04; continue; }
  pc = 0x15003f00; continue;
 }
 0x15003f00 => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if cpu.cf { pc = 0x15003f0e; continue; }
  pc = 0x15003f04; continue;
 }
 0x15003f04 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x15003f07; continue;
 }
 0x15003f07 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x15003efc; continue; }
  pc = 0x15003f0e; continue;
 }
 0x15003f0e => {
  { let v: u32 = 0x15016ca8u32; cpu.eax = v; }
  { let v: u32 = 0x2bu32; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15003f24); f_15004fa0(cpu);
  { let v: u32 = cpu.r32(0x15016cacu32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15094c98u32, v); }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if !cpu.cf { pc = 0x15003f46; continue; }
  pc = 0x15003f34; continue;
 }
 0x15003f34 => {
  { let v: u32 = cpu.r32(0x15016cb0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x15003f46; continue; }
  pc = 0x15003f40; continue;
 }
 0x15003f40 => {
  { let v: u32 = cpu.edx; cpu.w32(0x15016cb0u32, v); }
  pc = 0x15003f46; continue;
 }
 0x15003f46 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x15094ca0u32, v); }
  let t = cpu.r32(0x15016b8cu32); cpu.push(0x15003f54); cpu.call(t);
  pc = 0x15003f54; continue;
 }
 0x15003f54 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003f5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003f5a); }
 let mut pc: u32 = 0x15003f5a;
 loop { match pc {
 0x15003e72 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15003e76 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15094c94u32); cpu.edx = v; }
  pc = 0x15003e7f; continue;
 }
 0x15003e7f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x15003e72; continue; }
  pc = 0x15003e83; continue;
 }
 0x15003e83 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x15003e94; continue; }
  pc = 0x15003e8c; continue;
 }
 0x15003e8c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x15003e94); f_15003ecc(cpu);
  pc = 0x15003e94; continue;
 }
 0x15003e94 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15003e9b); f_15003ecc(cpu);
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  pc = 0x15003e7f; continue;
 }
 0x15003f5a => {
  pc = 0x15003e76; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003f5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003f5f); }
 let mut pc: u32 = 0x15003f5f;
 loop { match pc {
 0x15003f5f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x15003f74); f_150050ab(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003f78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003f78); }
 let mut pc: u32 = 0x15003f78;
 loop { match pc {
 0x15003f78 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x15003f8a; continue; }
  pc = 0x15003f80; continue;
 }
 0x15003f80 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x15003fa8; continue; }
  pc = 0x15003f85; continue;
 }
 0x15003f85 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x15003f8a => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15003f90); cpu.call(t);
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x15003f9c; continue; }
  pc = 0x15003f97; continue;
 }
 0x15003f97 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x15003f9c => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x15003fa1; continue;
 }
 0x15003fa1 => {
  cpu.push(0x15003fa6); f_15004078(cpu);
  pc = 0x15003fbf; continue;
 }
 0x15003fa8 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15003fae); cpu.call(t);
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x15003f97; continue; }
  pc = 0x15003fb8; continue;
 }
 0x15003fb8 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  pc = 0x15003fa1; continue;
 }
 0x15003fbf => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x15003f97; continue; }
  pc = 0x15003fc4; continue;
 }
 0x15003fc4 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x15003f97; continue; }
  pc = 0x15003fc9; continue;
 }
 0x15003fc9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15003fd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15003fd1); }
 let mut pc: u32 = 0x15003fd1;
 loop { match pc {
 0x15003fd1 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15003fd7); cpu.call(t);
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x2u32); }
  if cpu.zf { pc = 0x15003fe9; continue; }
  pc = 0x15003fdd; continue;
 }
 0x15003fdd => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15003fe3); cpu.call(t);
  { cpu.sub32(cpu.r32(0x78u32.wrapping_add(cpu.eax)), 0x3u32); }
  if !cpu.zf { pc = 0x15004007; continue; }
  pc = 0x15003fe9; continue;
 }
 0x15003fe9 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15003fef); cpu.call(t);
  { cpu.sub32(cpu.r32(0x90u32.wrapping_add(cpu.eax)), 0x2u32); }
  if cpu.zf { pc = 0x1500400d; continue; }
  pc = 0x15003ff8; continue;
 }
 0x15003ff8 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15003ffe); cpu.call(t);
  { cpu.sub32(cpu.r32(0x90u32.wrapping_add(cpu.eax)), 0x3u32); }
  if cpu.zf { pc = 0x1500400d; continue; }
  pc = 0x15004007; continue;
 }
 0x15004007 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500400d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500403c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500403c); }
 let mut pc: u32 = 0x1500403c;
 loop { match pc {
 0x1500403c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15016c38u32), 0x0u8); }
  if cpu.zf { pc = 0x1500405f; continue; }
  pc = 0x15004047; continue;
 }
 0x15004047 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x15003f78u32; cpu.push(v); }
  cpu.push(0x15004053); f_1500fbc4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500405f; continue; }
  pc = 0x15004057; continue;
 }
 0x15004057 => {
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x15016c38u32, v); }
  pc = 0x1500405f; continue;
 }
 0x1500405f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15016c38u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004073(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004073); }
 let mut pc: u32 = 0x15004073;
 loop { match pc {
 0x15004073 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  f_15004078(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004078); }
 let mut pc: u32 = 0x15004078;
 loop { match pc {
 0x15004078 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15004088); cpu.call(t);
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.cf { pc = 0x1500417f; continue; }
  pc = 0x15004098; continue;
 }
 0x15004098 => {
  if (cpu.cf || cpu.zf) { pc = 0x15004189; continue; }
  pc = 0x1500409e; continue;
 }
 0x1500409e => {
  { cpu.sub32(cpu.edx, 0xcu32); }
  if (cpu.cf || cpu.zf) { pc = 0x1500419f; continue; }
  pc = 0x150040a7; continue;
 }
 0x150040a7 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x150041d6; continue;
 }
 0x1500417f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x15004195; continue; }
  pc = 0x15004184; continue;
 }
 0x15004184 => {
  pc = 0x150040a7; continue;
 }
 0x15004189 => {
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  cpu.push(0x15004193); f_150040b1(cpu);
  pc = 0x150041d4; continue;
 }
 0x15004195 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x1500419f; continue; }
  pc = 0x1500419a; continue;
 }
 0x1500419a => {
  cpu.push(0x1500419f); f_15005102(cpu);
  pc = 0x1500419f; continue;
 }
 0x1500419f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x150041c6; continue; }
  pc = 0x150041a7; continue;
 }
 0x150041a7 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x150041c6; continue; }
  pc = 0x150041ac; continue;
 }
 0x150041ac => {
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x150041c6; continue; }
  pc = 0x150041b1; continue;
 }
 0x150041b1 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150041b9); cpu.call(t);
  { let v: u32 = 0x2u32; cpu.w32(0x58u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x150041c6); cpu.call(t);
  pc = 0x150041c6; continue;
 }
 0x150041c6 => {
  cpu.push(0x150041cb); f_15003fd1(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150041d4; continue; }
  pc = 0x150041cf; continue;
 }
 0x150041cf => {
  cpu.push(0x150041d4); f_1500403c(cpu);
  pc = 0x150041d4; continue;
 }
 0x150041d4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x150041d6; continue;
 }
 0x150041d6 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150040b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150040b1); }
 let mut pc: u32 = 0x150040b1;
 loop { match pc {
 0x150040b1 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150040bd); cpu.call(t);
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x150040ee; continue; }
  pc = 0x150040c8; continue;
 }
 0x150040c8 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x150040ee; continue; }
  pc = 0x150040cd; continue;
 }
 0x150040cd => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x150040ee; continue; }
  pc = 0x150040d2; continue;
 }
 0x150040d2 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150040d8); cpu.call(t);
  { let v: u32 = 0x2u32; cpu.w32(0x68u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x150040e7); cpu.call(t);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150040ee => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150041dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150041dd); }
 let mut pc: u32 = 0x150041dd;
 loop { match pc {
 0x150041dd => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(0x2b); }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x150041e6; continue;
 }
 0x150041e6 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150041ec); cpu.call(t);
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let _ = cpu.eax; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x15016bd0u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.sub32(cpu.edx, 0x68u32); }
  if !cpu.zf { pc = 0x150041e6; continue; }
  pc = 0x15004206; continue;
 }
 0x15004206 => {
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500420c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500420c); }
 let mut pc: u32 = 0x1500420c;
 loop { match pc {
 0x1500420c => {
  if (cpu.sf != cpu.of) { pc = 0x15004250; continue; }
  pc = 0x1500420e; continue;
 }
 0x1500420e => {
  { let v: u8 = cpu.add8(cpu.r8(0x15004344u32), (cpu.edx as u8)); cpu.w8(0x15004344u32, v); }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.edx)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.adc32(cpu.eax, 0x15004290u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x15004352u32).wrapping_add((cpu.edx as u8)); cpu.w8(0x15004352u32, v); }
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x6575653u32).wrapping_add((cpu.edx as u8)); cpu.w8(0x6575653u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x220u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x238u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x240u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x23cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x1500424f; continue;
 }
 0x1500424f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x1500425b; continue;
 }
 0x15004250 => {
  cpu.unimplemented(0x15004250, "Arpl");
  cpu.unimplemented(0x15004253, "Les");
  if !cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x1500425b; continue;
 }
 0x1500425b => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x6u8); }
  if !cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x15004264; continue;
 }
 0x15004264 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3fffff73u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500436d; continue; }
  pc = 0x15004274; continue;
 }
 0x15004274 => {
  pc = cpu.r32(0x1500420cu32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1500427c => {
  { let v: u32 = 0x82u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004286 => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004290 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x81u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0xfad9u32); }
  if !cpu.zf { pc = 0x150042af; continue; }
  pc = 0x150042a5; continue;
 }
 0x150042a5 => {
  { let v: u32 = 0x88u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x150042af => {
  { cpu.sub32(cpu.ecx, 0xf1d9u32); }
  if !cpu.zf { pc = 0x150042c1; continue; }
  pc = 0x150042b7; continue;
 }
 0x150042b7 => {
  { let v: u32 = 0x8eu32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x150042c1 => {
  if !cpu.zf { pc = 0x150042cd; continue; }
  pc = 0x150042c3; continue;
 }
 0x150042c3 => {
  { let v: u32 = 0x8fu32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x150042cd => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xdbu32); }
  if cpu.zf { pc = 0x150042e1; continue; }
  pc = 0x150042d9; continue;
 }
 0x150042d9 => {
  { cpu.sub32(cpu.ecx, 0xdfu32); }
  if !cpu.zf { pc = 0x150042f7; continue; }
  pc = 0x150042e1; continue;
 }
 0x150042e1 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x30u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if !cpu.zf { pc = 0x150042f7; continue; }
  pc = 0x150042f2; continue;
 }
 0x150042f2 => {
  { let v: u32 = 0x8du32; cpu.eax = v; }
  pc = 0x150042f7; continue;
 }
 0x150042f7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15004372; continue; }
  pc = 0x15004308; continue;
 }
 0x15004308 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x30u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x15004372; continue; }
  pc = 0x15004319; continue;
 }
 0x15004319 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 13u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if !cpu.zf { pc = 0x15004372; continue; }
  pc = 0x15004344; continue;
 }
 0x15004344 => {
  { let v: u32 = 0x83u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x1500434b => {
  { let v: u32 = 0x84u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004352 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1500435f; continue; }
  pc = 0x15004358; continue;
 }
 0x15004358 => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x1500435f => {
  { let v: u32 = 0x8bu32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004366 => {
  { let v: u32 = 0x85u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x1500436d => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004372 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x150043b3; continue; }
  pc = 0x15004377; continue;
 }
 0x15004377 => {
  { let v: u8 = 0x1u8; cpu.w8(0x15094c9cu32, v); }
  cpu.push(0x15004383); f_150040b1(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150043a9; continue; }
  pc = 0x15004387; continue;
 }
 0x15004387 => {
  { cpu.sub8(cpu.r8(0x15094c9cu32), 0x0u8); }
  if cpu.zf { pc = 0x150043a9; continue; }
  pc = 0x15004390; continue;
 }
 0x15004390 => {
  cpu.push(0x15004395); f_15003279(cpu);
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x150045cd; continue;
 }
 0x150043a9 => {
  cpu.push(0x150043ae); f_15003279(cpu);
  pc = 0x150045c8; continue;
 }
 0x150043b3 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x150043bd; continue;
 }
 0x150043bd => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150043c3); cpu.call(t);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x5cu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  if !cpu.zf { pc = 0x15004414; continue; }
  pc = 0x150043cb; continue;
 }
 0x150043cb => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150043d1); cpu.call(t);
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u32); }
  if cpu.zf { pc = 0x1500441d; continue; }
  pc = 0x150043d8; continue;
 }
 0x150043d8 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150043de); cpu.call(t);
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x2u32); }
  if cpu.zf { pc = 0x1500441d; continue; }
  pc = 0x150043e5; continue;
 }
 0x150043e5 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150043eb); cpu.call(t);
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x3u32); }
  if cpu.zf { pc = 0x1500441d; continue; }
  pc = 0x150043f2; continue;
 }
 0x150043f2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x15094c9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x15004400); f_15004078(cpu);
  { cpu.sub8(cpu.r8(0x15094c9cu32), 0x0u8); }
  if cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x1500440d; continue;
 }
 0x1500440d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x150045cd; continue;
 }
 0x15004414 => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x60u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x150043bd; continue; }
  pc = 0x1500441d; continue;
 }
 0x1500441d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xc0000094u32); }
  if cpu.cf { pc = 0x15004449; continue; }
  pc = 0x15004426; continue;
 }
 0x15004426 => {
  if (cpu.cf || cpu.zf) { pc = 0x150044a7; continue; }
  pc = 0x1500442c; continue;
 }
 0x1500442c => {
  { cpu.sub32(cpu.eax, 0xc0000096u32); }
  if cpu.cf { pc = 0x150044bd; continue; }
  pc = 0x15004437; continue;
 }
 0x15004437 => {
  if (cpu.cf || cpu.zf) { pc = 0x15004491; continue; }
  pc = 0x15004439; continue;
 }
 0x15004439 => {
  { cpu.sub32(cpu.eax, 0xc00000fdu32); }
  if cpu.zf { pc = 0x150044b2; continue; }
  pc = 0x15004444; continue;
 }
 0x15004444 => {
  pc = 0x150044bd; continue;
 }
 0x15004449 => {
  { cpu.sub32(cpu.eax, 0xc0000005u32); }
  if cpu.cf { pc = 0x150044bd; continue; }
  pc = 0x15004454; continue;
 }
 0x15004454 => {
  if (cpu.cf || cpu.zf) { pc = 0x1500445f; continue; }
  pc = 0x15004456; continue;
 }
 0x15004456 => {
  { cpu.sub32(cpu.eax, 0xc000001du32); }
  if cpu.zf { pc = 0x1500449c; continue; }
  pc = 0x1500445d; continue;
 }
 0x1500445d => {
  pc = 0x150044bd; continue;
 }
 0x1500445f => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x1500446c; continue; }
  pc = 0x15004465; continue;
 }
 0x15004465 => {
  { let v: u32 = 0x150175e0u32; cpu.eax = v; }
  pc = 0x15004471; continue;
 }
 0x1500446c => {
  { let v: u32 = 0x150175eau32; cpu.eax = v; }
  pc = 0x15004471; continue;
 }
 0x15004471 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x150175f2u32; cpu.push(v); }
  { let v: u32 = 0x128u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500448c); f_1500fbbe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x150044d6; continue;
 }
 0x15004491 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017641u32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x1500449c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x15017679u32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x150044a7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x150176aeu32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x150044b2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x150176e9u32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x150044bd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1501771bu32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x150044c6 => {
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x150044d3); f_1500fbbe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x150044d6; continue;
 }
 0x150044d6 => {
  cpu.push(0x150044db); f_1500fbb8(cpu);
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150044ef; continue; }
  pc = 0x150044e1; continue;
 }
 0x150044e1 => {
  cpu.push(0x150044e6); f_15005207(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1500458d; continue; }
  pc = 0x150044ef; continue;
 }
 0x150044ef => {
  { let v = 0x15017762u32; cpu.push(v); }
  cpu.push(0x150044f9); f_1500fbb2(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500458d; continue; }
  pc = 0x15004503; continue;
 }
 0x15004503 => {
  { let v = 0x1501776du32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500450e); f_1500fbac(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x218u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500458b; continue; }
  pc = 0x15004519; continue;
 }
 0x15004519 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let _ = cpu.eax; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x1501777bu32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1501778fu32; cpu.esi = v; }
  { let v: u32 = 0x124u32.wrapping_add(cpu.esp); cpu.edi = v; }
  cpu.push(0x15004546); f_1500fbfa(cpu);
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x15004552; continue;
 }
 0x15004552 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1500456a; continue; }
  pc = 0x1500455a; continue;
 }
 0x1500455a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x15004552; continue; }
  pc = 0x1500456a; continue;
 }
 0x1500456a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.pop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2010u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x124u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x22cu32.wrapping_add(cpu.esp)); cpu.push(0x15004589); cpu.call(t);
  pc = 0x1500458d; continue;
 }
 0x1500458b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1500458d; continue;
 }
 0x1500458d => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x15004591; continue;
 }
 0x15004591 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x220u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.push(0x2b); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let _ = cpu.eax; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.pop(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x124u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x150045c8); f_1500fba6(cpu);
  pc = 0x150045c8; continue;
 }
 0x150045c8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x150045cd; continue;
 }
 0x150045cd => {
  { let v: u32 = cpu.add32(cpu.esp, 0x220u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004228(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004228); }
 let mut pc: u32 = 0x15004228;
 loop { match pc {
 0x15004228 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(0x2b); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x220u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x238u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x240u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x23cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x1500424f; continue;
 }
 0x1500424f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x1500425b; continue;
 }
 0x1500425b => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x6u8); }
  if !cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x15004264; continue;
 }
 0x15004264 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3fffff73u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500436d; continue; }
  pc = 0x15004274; continue;
 }
 0x15004274 => {
  pc = cpu.r32(0x1500420cu32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x1500427c => {
  { let v: u32 = 0x82u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004286 => {
  { let v: u32 = 0x86u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004290 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x81u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0xfad9u32); }
  if !cpu.zf { pc = 0x150042af; continue; }
  pc = 0x150042a5; continue;
 }
 0x150042a5 => {
  { let v: u32 = 0x88u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x150042af => {
  { cpu.sub32(cpu.ecx, 0xf1d9u32); }
  if !cpu.zf { pc = 0x150042c1; continue; }
  pc = 0x150042b7; continue;
 }
 0x150042b7 => {
  { let v: u32 = 0x8eu32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x150042c1 => {
  if !cpu.zf { pc = 0x150042cd; continue; }
  pc = 0x150042c3; continue;
 }
 0x150042c3 => {
  { let v: u32 = 0x8fu32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x150042cd => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xdbu32); }
  if cpu.zf { pc = 0x150042e1; continue; }
  pc = 0x150042d9; continue;
 }
 0x150042d9 => {
  { cpu.sub32(cpu.ecx, 0xdfu32); }
  if !cpu.zf { pc = 0x150042f7; continue; }
  pc = 0x150042e1; continue;
 }
 0x150042e1 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x30u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if !cpu.zf { pc = 0x150042f7; continue; }
  pc = 0x150042f2; continue;
 }
 0x150042f2 => {
  { let v: u32 = 0x8du32; cpu.eax = v; }
  pc = 0x150042f7; continue;
 }
 0x150042f7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15004372; continue; }
  pc = 0x15004308; continue;
 }
 0x15004308 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x30u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x15004372; continue; }
  pc = 0x15004319; continue;
 }
 0x15004319 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 13u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if !cpu.zf { pc = 0x15004372; continue; }
  pc = 0x15004344; continue;
 }
 0x15004344 => {
  { let v: u32 = 0x83u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x1500434b => {
  { let v: u32 = 0x84u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004352 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1500435f; continue; }
  pc = 0x15004358; continue;
 }
 0x15004358 => {
  { let v: u32 = 0x8au32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x1500435f => {
  { let v: u32 = 0x8bu32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004366 => {
  { let v: u32 = 0x85u32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x1500436d => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x15004372; continue;
 }
 0x15004372 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x150043b3; continue; }
  pc = 0x15004377; continue;
 }
 0x15004377 => {
  { let v: u8 = 0x1u8; cpu.w8(0x15094c9cu32, v); }
  cpu.push(0x15004383); f_150040b1(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150043a9; continue; }
  pc = 0x15004387; continue;
 }
 0x15004387 => {
  { cpu.sub8(cpu.r8(0x15094c9cu32), 0x0u8); }
  if cpu.zf { pc = 0x150043a9; continue; }
  pc = 0x15004390; continue;
 }
 0x15004390 => {
  cpu.push(0x15004395); f_15003279(cpu);
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x150045cd; continue;
 }
 0x150043a9 => {
  cpu.push(0x150043ae); f_15003279(cpu);
  pc = 0x150045c8; continue;
 }
 0x150043b3 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x150043bd; continue;
 }
 0x150043bd => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150043c3); cpu.call(t);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x5cu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  if !cpu.zf { pc = 0x15004414; continue; }
  pc = 0x150043cb; continue;
 }
 0x150043cb => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150043d1); cpu.call(t);
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u32); }
  if cpu.zf { pc = 0x1500441d; continue; }
  pc = 0x150043d8; continue;
 }
 0x150043d8 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150043de); cpu.call(t);
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x2u32); }
  if cpu.zf { pc = 0x1500441d; continue; }
  pc = 0x150043e5; continue;
 }
 0x150043e5 => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150043eb); cpu.call(t);
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x3u32); }
  if cpu.zf { pc = 0x1500441d; continue; }
  pc = 0x150043f2; continue;
 }
 0x150043f2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x15094c9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x15004400); f_15004078(cpu);
  { cpu.sub8(cpu.r8(0x15094c9cu32), 0x0u8); }
  if cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x1500440d; continue;
 }
 0x1500440d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x150045cd; continue;
 }
 0x15004414 => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x60u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x150043bd; continue; }
  pc = 0x1500441d; continue;
 }
 0x1500441d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xc0000094u32); }
  if cpu.cf { pc = 0x15004449; continue; }
  pc = 0x15004426; continue;
 }
 0x15004426 => {
  if (cpu.cf || cpu.zf) { pc = 0x150044a7; continue; }
  pc = 0x1500442c; continue;
 }
 0x1500442c => {
  { cpu.sub32(cpu.eax, 0xc0000096u32); }
  if cpu.cf { pc = 0x150044bd; continue; }
  pc = 0x15004437; continue;
 }
 0x15004437 => {
  if (cpu.cf || cpu.zf) { pc = 0x15004491; continue; }
  pc = 0x15004439; continue;
 }
 0x15004439 => {
  { cpu.sub32(cpu.eax, 0xc00000fdu32); }
  if cpu.zf { pc = 0x150044b2; continue; }
  pc = 0x15004444; continue;
 }
 0x15004444 => {
  pc = 0x150044bd; continue;
 }
 0x15004449 => {
  { cpu.sub32(cpu.eax, 0xc0000005u32); }
  if cpu.cf { pc = 0x150044bd; continue; }
  pc = 0x15004454; continue;
 }
 0x15004454 => {
  if (cpu.cf || cpu.zf) { pc = 0x1500445f; continue; }
  pc = 0x15004456; continue;
 }
 0x15004456 => {
  { cpu.sub32(cpu.eax, 0xc000001du32); }
  if cpu.zf { pc = 0x1500449c; continue; }
  pc = 0x1500445d; continue;
 }
 0x1500445d => {
  pc = 0x150044bd; continue;
 }
 0x1500445f => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x1500446c; continue; }
  pc = 0x15004465; continue;
 }
 0x15004465 => {
  { let v: u32 = 0x150175e0u32; cpu.eax = v; }
  pc = 0x15004471; continue;
 }
 0x1500446c => {
  { let v: u32 = 0x150175eau32; cpu.eax = v; }
  pc = 0x15004471; continue;
 }
 0x15004471 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x150175f2u32; cpu.push(v); }
  { let v: u32 = 0x128u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500448c); f_1500fbbe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x150044d6; continue;
 }
 0x15004491 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x15017641u32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x1500449c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x15017679u32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x150044a7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x150176aeu32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x150044b2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x150176e9u32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x150044bd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1501771bu32; cpu.push(v); }
  pc = 0x150044c6; continue;
 }
 0x150044c6 => {
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x150044d3); f_1500fbbe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x150044d6; continue;
 }
 0x150044d6 => {
  cpu.push(0x150044db); f_1500fbb8(cpu);
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150044ef; continue; }
  pc = 0x150044e1; continue;
 }
 0x150044e1 => {
  cpu.push(0x150044e6); f_15005207(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1500458d; continue; }
  pc = 0x150044ef; continue;
 }
 0x150044ef => {
  { let v = 0x15017762u32; cpu.push(v); }
  cpu.push(0x150044f9); f_1500fbb2(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500458d; continue; }
  pc = 0x15004503; continue;
 }
 0x15004503 => {
  { let v = 0x1501776du32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500450e); f_1500fbac(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x218u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500458b; continue; }
  pc = 0x15004519; continue;
 }
 0x15004519 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let _ = cpu.eax; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x1501777bu32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1501778fu32; cpu.esi = v; }
  { let v: u32 = 0x124u32.wrapping_add(cpu.esp); cpu.edi = v; }
  cpu.push(0x15004546); f_1500fbfa(cpu);
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.pop(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x15004552; continue;
 }
 0x15004552 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1500456a; continue; }
  pc = 0x1500455a; continue;
 }
 0x1500455a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x15004552; continue; }
  pc = 0x1500456a; continue;
 }
 0x1500456a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.pop(); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2010u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x124u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x22cu32.wrapping_add(cpu.esp)); cpu.push(0x15004589); cpu.call(t);
  pc = 0x1500458d; continue;
 }
 0x1500458b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1500458d; continue;
 }
 0x1500458d => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x150045c8; continue; }
  pc = 0x15004591; continue;
 }
 0x15004591 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x220u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { cpu.push(0x2b); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let _ = cpu.eax; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.pop(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x124u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x150045c8); f_1500fba6(cpu);
  pc = 0x150045c8; continue;
 }
 0x150045c8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x150045cd; continue;
 }
 0x150045cd => {
  { let v: u32 = cpu.add32(cpu.esp, 0x220u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150045db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150045db); }
 let mut pc: u32 = 0x150045db;
 loop { match pc {
 0x150045db => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150045e4); cpu.call(t);
  { let v: u32 = cpu.edx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150045f4); cpu.call(t);
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150045ff); cpu.call(t);
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x15004228u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15004611); cpu.call(t);
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004619(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004619); }
 let mut pc: u32 = 0x15004619;
 loop { match pc {
 0x15004619 => {
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15004620); cpu.call(t);
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500462e; continue; }
  pc = 0x15004627; continue;
 }
 0x15004627 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.fs_base), v); }
  pc = 0x1500462e; continue;
 }
 0x1500462e => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15004634); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x15004640); f_15003fd1(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15004666; continue; }
  pc = 0x15004644; continue;
 }
 0x15004644 => {
  cpu.push(0x15004649); f_1500403c(cpu);
  let t = cpu.r32(0x15016b68u32); cpu.push(0x1500464f); cpu.call(t);
  { let v: u32 = 0x2u32; cpu.w32(0x78u32.wrapping_add(cpu.eax), v); }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x1500465c); cpu.call(t);
  { let v: u32 = 0x2u32; cpu.w32(0x90u32.wrapping_add(cpu.eax), v); }
  pc = 0x15004666; continue;
 }
 0x15004666 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004668(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004668); }
 let mut pc: u32 = 0x15004668;
 loop { match pc {
 0x15004668 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(0x2b); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let _ = cpu.eax; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.pop(); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x15004688); f_15004de6(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150046a6; continue; }
  pc = 0x1500468e; continue;
 }
 0x1500468e => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(0x2b); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let _ = cpu.eax; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, false); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.pop(); }
  pc = 0x150046a6; continue;
 }
 0x150046a6 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150046b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150046b3); }
 let mut pc: u32 = 0x150046b3;
 loop { match pc {
 0x150046b3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x150046cc; continue; }
  pc = 0x150046c6; continue;
 }
 0x150046c6 => {
  { let v: u8 = 0x2du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x150046ce; continue;
 }
 0x150046cc => {
  { let v: u8 = 0x2bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x150046ce; continue;
 }
 0x150046ce => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x150046f1; continue; }
  pc = 0x150046dc; continue;
 }
 0x150046dc => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x150046f1; continue; }
  pc = 0x150046ea; continue;
 }
 0x150046ea => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x150046f1; continue;
 }
 0x150046f1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15004701; continue; }
  pc = 0x150046f5; continue;
 }
 0x150046f5 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x3u32); }
  if !cpu.zf { pc = 0x15004701; continue; }
  pc = 0x150046ff; continue;
 }
 0x150046ff => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x15004701; continue;
 }
 0x15004701 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500474d; continue; }
  pc = 0x15004709; continue;
 }
 0x15004709 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500474a; continue; }
  pc = 0x1500470d; continue;
 }
 0x1500470d => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1500470f; continue;
 }
 0x1500470f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500471a; continue; }
  pc = 0x15004714; continue;
 }
 0x15004714 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1500470f; continue;
 }
 0x1500471a => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500471a; continue; }
  pc = 0x15004748; continue;
 }
 0x15004748 => {
  pc = 0x1500474d; continue;
 }
 0x1500474a => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  pc = 0x1500474d; continue;
 }
 0x1500474d => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004756(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004756); }
 let mut pc: u32 = 0x15004756;
 loop { match pc {
 0x15004754 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15004756 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1500476b; continue; }
  pc = 0x1500475d; continue;
 }
 0x1500475d => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500476b; continue; }
  pc = 0x15004761; continue;
 }
 0x15004761 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500476b => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500477c; continue; }
  pc = 0x1500476f; continue;
 }
 0x1500476f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x1500477c; continue; }
  pc = 0x15004775; continue;
 }
 0x15004775 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1500476b; continue;
 }
 0x1500477c => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15004789; continue; }
  pc = 0x15004780; continue;
 }
 0x15004780 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15004780; continue; }
  pc = 0x15004789; continue;
 }
 0x15004789 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500479d; continue; }
  pc = 0x15004791; continue;
 }
 0x15004791 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1500479d; continue; }
  pc = 0x15004795; continue;
 }
 0x15004795 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15004791; continue; }
  pc = 0x1500479d; continue;
 }
 0x1500479d => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15004754; continue; }
  pc = 0x150047a1; continue;
 }
 0x150047a1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x150047ae; continue; }
  pc = 0x150047a7; continue;
 }
 0x150047a7 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x150047a1; continue; }
  pc = 0x150047ae; continue;
 }
 0x150047ae => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x15004754; continue; }
  pc = 0x150047b2; continue;
 }
 0x150047b2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x150047ae; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150047b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150047b9); }
 let mut pc: u32 = 0x150047b9;
 loop { match pc {
 0x150047b9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.zf { pc = 0x15004822; continue; }
  pc = 0x150047ca; continue;
 }
 0x150047ca => {
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x150047d9; continue;
 }
 0x150047d9 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x150047d9; continue; }
  pc = 0x150047e8; continue;
 }
 0x150047e8 => {
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x150047f8; continue; }
  pc = 0x150047f1; continue;
 }
 0x150047f1 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x15004816; continue;
 }
 0x150047f8 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x15004802; continue; }
  pc = 0x150047fd; continue;
 }
 0x150047fd => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x15004816; continue; }
  pc = 0x15004802; continue;
 }
 0x15004802 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x15004816; continue; }
  pc = 0x1500480c; continue;
 }
 0x1500480c => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x15004816; continue;
 }
 0x15004816 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x15004822); f_15004ee0(cpu);
  pc = 0x15004822; continue;
 }
 0x15004822 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500482a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500482a); }
 let mut pc: u32 = 0x1500482a;
 loop { match pc {
 0x1500482a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500484a); f_15005212(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500486a; continue; }
  pc = 0x1500484e; continue;
 }
 0x1500484e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x1500485d; continue;
 }
 0x15004855 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x1500485b; continue; }
  pc = 0x15004859; continue;
 }
 0x15004859 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1500485b; continue;
 }
 0x1500485b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1500485d; continue;
 }
 0x1500485d => {
  { let v: u8 = cpu.r8(0xffffffc0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { pc = 0x15004855; continue; }
  pc = 0x15004865; continue;
 }
 0x15004865 => {
  pc = 0x15004a47; continue;
 }
 0x1500486a => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x47u32); }
  if !cpu.zf { pc = 0x150048df; continue; }
  pc = 0x15004878; continue;
 }
 0x15004878 => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.esi, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x15004891; continue; }
  pc = 0x1500488d; continue;
 }
 0x1500488d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150048d8; continue; }
  pc = 0x15004891; continue;
 }
 0x15004891 => {
  { let v = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  cpu.push(0x15004899); f_150052eb(cpu);
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150048a9); f_15005313(cpu);
  cpu.push(0x150048ae); f_150030a4(cpu);
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if (cpu.sf != cpu.of) { pc = 0x150048bd; continue; }
  pc = 0x150048b9; continue;
 }
 0x150048b9 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150048c1; continue; }
  pc = 0x150048bd; continue;
 }
 0x150048bd => {
  { let v: u8 = 0x45u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x150048df; continue;
 }
 0x150048c1 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x150048ce; continue; }
  pc = 0x150048cb; continue;
 }
 0x150048cb => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x150048ce; continue;
 }
 0x150048ce => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x150048d8; continue; }
  pc = 0x150048d7; continue;
 }
 0x150048d7 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x150048d8; continue;
 }
 0x150048d8 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u8 = 0x46u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x150048df; continue;
 }
 0x150048df => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x1500490e; continue; }
  pc = 0x150048e8; continue;
 }
 0x150048e8 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x150048fa; continue; }
  pc = 0x150048f3; continue;
 }
 0x150048f3 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500490e; continue; }
  pc = 0x150048fa; continue;
 }
 0x150048fa => {
  { let v: u32 = 0x2au32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x15004909); f_15004ee0(cpu);
  pc = 0x15004a5b; continue;
 }
 0x1500490e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x1500492b; continue; }
  pc = 0x1500491a; continue;
 }
 0x1500491a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15004926; continue; }
  pc = 0x15004921; continue;
 }
 0x15004921 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x1500492b; continue;
 }
 0x15004926 => {
  if (cpu.sf == cpu.of) { pc = 0x1500492b; continue; }
  pc = 0x15004928; continue;
 }
 0x15004928 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x1500492b; continue;
 }
 0x1500492b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x15004944; continue; }
  pc = 0x15004940; continue;
 }
 0x15004940 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x15004965; continue; }
  pc = 0x15004944; continue;
 }
 0x15004944 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if cpu.zf { pc = 0x15004965; continue; }
  pc = 0x1500494d; continue;
 }
 0x1500494d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15004965; continue; }
  pc = 0x15004954; continue;
 }
 0x15004954 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15004962); f_15005390(cpu);
  { let v = cpu.st(0); cpu.wf64(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x15004965; continue;
 }
 0x15004965 => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15004984); f_150054ad(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1500499d; continue; }
  pc = 0x15004992; continue;
 }
 0x15004992 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x150049aa; continue;
 }
 0x1500499d => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x150049aa; continue; }
  pc = 0x150049a1; continue;
 }
 0x150049a1 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2bu8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x150049aa; continue;
 }
 0x150049aa => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x45u32); }
  if !cpu.zf { pc = 0x15004a1c; continue; }
  pc = 0x150049b6; continue;
 }
 0x150049b6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15004a45; continue; }
  pc = 0x150049c5; continue;
 }
 0x150049c5 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x150049d2); f_15004756(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x150049ee; continue; }
  pc = 0x150049e3; continue;
 }
 0x150049e3 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x150049ee; continue; }
  pc = 0x150049e7; continue;
 }
 0x150049e7 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x150049ee; continue;
 }
 0x150049ee => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x15004a00; continue; }
  pc = 0x150049fc; continue;
 }
 0x150049fc => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15004a06; continue; }
  pc = 0x15004a00; continue;
 }
 0x15004a00 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.eax); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x15004a06; continue;
 }
 0x15004a06 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  cpu.push(0x15004a18); f_150046b3(cpu);
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x15004a47; continue;
 }
 0x15004a1c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x15004a45; continue; }
  pc = 0x15004a30; continue;
 }
 0x15004a30 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  cpu.push(0x15004a43); f_15004756(cpu);
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  pc = 0x15004a45; continue;
 }
 0x15004a45 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x15004a47; continue;
 }
 0x15004a47 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150048fa; continue; }
  pc = 0x15004a4f; continue;
 }
 0x15004a4f => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x15004a5b); f_150047b9(cpu);
  pc = 0x15004a5b; continue;
 }
 0x15004a5b => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004a64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004a64); }
 let mut pc: u32 = 0x15004a64;
 loop { match pc {
 0x15004a64 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x15004a72; continue;
 }
 0x15004a72 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x20u32); }
  if cpu.zf { pc = 0x15004a87; continue; }
  pc = 0x15004a7d; continue;
 }
 0x15004a7d => {
  { cpu.sub32(cpu.edx, 0x9u32); }
  if (cpu.sf != cpu.of) { pc = 0x15004a8a; continue; }
  pc = 0x15004a82; continue;
 }
 0x15004a82 => {
  { cpu.sub32(cpu.edx, 0xdu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15004a8a; continue; }
  pc = 0x15004a87; continue;
 }
 0x15004a87 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x15004a72; continue;
 }
 0x15004a8a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x2bu32); }
  if cpu.zf { pc = 0x15004a9f; continue; }
  pc = 0x15004a98; continue;
 }
 0x15004a98 => {
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x15004aa1; continue; }
  pc = 0x15004a9d; continue;
 }
 0x15004a9d => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x15004a9f; continue;
 }
 0x15004a9f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x15004aa1; continue;
 }
 0x15004aa1 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = 0x30u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x15004aa8; continue;
 }
 0x15004aa8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.edx as u8) as u32; cpu.esi = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x8u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x2eu32); }
  if !cpu.zf { pc = 0x15004ac4; continue; }
  pc = 0x15004ab8; continue;
 }
 0x15004ab8 => {
  { let v: u32 = ((cpu.ecx >> 8) as u8) as u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15004af1; continue; }
  pc = 0x15004abf; continue;
 }
 0x15004abf => {
  { let v: u8 = (cpu.ecx as u8) | 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x15004aa8; continue;
 }
 0x15004ac4 => {
  { cpu.sub32(cpu.esi, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x15004af1; continue; }
  pc = 0x15004ac9; continue;
 }
 0x15004ac9 => {
  { cpu.sub32(cpu.esi, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15004af1; continue; }
  pc = 0x15004ace; continue;
 }
 0x15004ace => {
  { let v: u32 = ((cpu.ecx >> 8) as u8) as u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x15004ad8; continue; }
  pc = 0x15004ad5; continue;
 }
 0x15004ad5 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x15004ad8; continue;
 }
 0x15004ad8 => {
  { let v: u8 = ((cpu.edx >> 8) as u8) | (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x30u32); }
  if cpu.zf { pc = 0x15004aec; continue; }
  pc = 0x15004ae2; continue;
 }
 0x15004ae2 => {
  { cpu.sub32(cpu.ebx, 0x11u32); }
  if (cpu.sf == cpu.of) { pc = 0x15004aeb; continue; }
  pc = 0x15004ae7; continue;
 }
 0x15004ae7 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffc8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x15004aeb; continue;
 }
 0x15004aeb => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x15004aec; continue;
 }
 0x15004aec => {
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x15004aa8; continue;
 }
 0x15004af1 => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x15004b87; continue; }
  pc = 0x15004b03; continue;
 }
 0x15004b03 => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x65u32); }
  if cpu.zf { pc = 0x15004b17; continue; }
  pc = 0x15004b0e; continue;
 }
 0x15004b0e => {
  { cpu.sub32(cpu.edx, 0x45u32); }
  if !cpu.zf { pc = 0x15004b83; continue; }
  pc = 0x15004b17; continue;
 }
 0x15004b17 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x2bu32); }
  if cpu.zf { pc = 0x15004b35; continue; }
  pc = 0x15004b2d; continue;
 }
 0x15004b2d => {
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x15004b37; continue; }
  pc = 0x15004b32; continue;
 }
 0x15004b32 => {
  { let v: u8 = (cpu.ecx as u8) | 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x15004b35; continue;
 }
 0x15004b35 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x15004b37; continue;
 }
 0x15004b37 => {
  { let v: u8 = (cpu.ecx as u8) & 0xfbu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x15004b3a; continue;
 }
 0x15004b3a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x15004b62; continue; }
  pc = 0x15004b47; continue;
 }
 0x15004b47 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15004b62; continue; }
  pc = 0x15004b4c; continue;
 }
 0x15004b4c => {
  { cpu.sub32(cpu.esi, 0x3e8u32); }
  if (cpu.sf == cpu.of) { pc = 0x15004b5c; continue; }
  pc = 0x15004b54; continue;
 }
 0x15004b54 => {
  { let v: u32 = cpu.imul2_32(cpu.esi, 0xau32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.edx); cpu.esi = v; }
  pc = 0x15004b5c; continue;
 }
 0x15004b5c => {
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x15004b3a; continue;
 }
 0x15004b62 => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x15004b71; continue; }
  pc = 0x15004b6f; continue;
 }
 0x15004b6f => {
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x15004b71; continue;
 }
 0x15004b71 => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x15004b84; continue; }
  pc = 0x15004b7e; continue;
 }
 0x15004b7e => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x15004b84; continue;
 }
 0x15004b83 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x15004b84; continue;
 }
 0x15004b84 => {
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x15004b87; continue;
 }
 0x15004b87 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, 0x11u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15004b99; continue; }
  pc = 0x15004b8f; continue;
 }
 0x15004b8f => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x11u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  pc = 0x15004b99; continue;
 }
 0x15004b99 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15004bac; continue; }
  pc = 0x15004b9d; continue;
 }
 0x15004b9d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffc7u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x15004bac; continue; }
  pc = 0x15004ba8; continue;
 }
 0x15004ba8 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x15004b99; continue;
 }
 0x15004bac => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x15004bbb; continue; }
  pc = 0x15004bb0; continue;
 }
 0x15004bb0 => {
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x15004c3f; continue;
 }
 0x15004bbb => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xffffffc8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x15004bcc); f_1500571a(cpu);
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15004c01; continue; }
  pc = 0x15004bd7; continue;
 }
 0x15004bd7 => {
  cpu.push(0x15004bdc); f_150050d7(cpu);
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x15004bef; continue; }
  pc = 0x15004be7; continue;
 }
 0x15004be7 => {
  { let v = cpu.rf64(0x15017a54u32); cpu.fpush(v); }
  pc = 0x15004c3a; continue;
 }
 0x15004bef => {
  { let v: u32 = cpu.r32(0x15017a54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x15017a58u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x15004c3f; continue;
 }
 0x15004c01 => {
  { cpu.sub32(cpu.eax, 0xfffffeccu32); }
  if (cpu.sf == cpu.of) { pc = 0x15004c17; continue; }
  pc = 0x15004c08; continue;
 }
 0x15004c08 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.push(0x15004c0f); f_150050d7(cpu);
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x15004c3f; continue;
 }
 0x15004c17 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x15004c2c; continue; }
  pc = 0x15004c1b; continue;
 }
 0x15004c1b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15004c29); f_15005390(cpu);
  { let v = cpu.st(0); cpu.wf64(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x15004c2c; continue;
 }
 0x15004c2c => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x15004c3f; continue; }
  pc = 0x15004c37; continue;
 }
 0x15004c37 => {
  { let v = cpu.rf64(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  pc = 0x15004c3a; continue;
 }
 0x15004c3a => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x15004c3f; continue;
 }
 0x15004c3f => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x15004c4b; continue; }
  pc = 0x15004c46; continue;
 }
 0x15004c46 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x15004c4b; continue;
 }
 0x15004c4b => {
  { let v = cpu.rf64(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004c57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004c57); }
 let mut pc: u32 = 0x15004c57;
 loop { match pc {
 0x15003a09 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x15003a10); f_15004619(cpu);
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x15003a1c); f_15003831(cpu);
  let t = cpu.r32(0x15016ba4u32); cpu.push(0x15003a22); cpu.call(t);
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15003a28); f_1500fbee(cpu);
  f_15003a28(cpu); return;
 }
 0x15004c57 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x15004c67; continue;
 }
 0x15004c66 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x15004c67; continue;
 }
 0x15004c67 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x15004c66; continue; }
  pc = 0x15004c70; continue;
 }
 0x15004c70 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15004c87); f_1500fba6(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x15003a09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004c8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004c8e); }
 let mut pc: u32 = 0x15004c8e;
 loop { match pc {
 0x15004c8e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x2bu32; cpu.edx = v; }
  cpu.push(0x15004c9b); f_150057b6(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15004ca8; continue; }
  pc = 0x15004c9f; continue;
 }
 0x15004c9f => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x15004ca8); f_15004c57(cpu);
  pc = 0x15004ca8; continue;
 }
 0x15004ca8 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004cab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004cab); }
 let mut pc: u32 = 0x15004cab;
 loop { match pc {
 0x15004cab => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15016c44u32)); }
  if cpu.cf { pc = 0x15004cbe; continue; }
  pc = 0x15004cb8; continue;
 }
 0x15004cb8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15004cbe => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15004cf4; continue; }
  pc = 0x15004cc3; continue;
 }
 0x15004cc3 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15016c98u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x15004cf4; continue; }
  pc = 0x15004cd7; continue;
 }
 0x15004cd7 => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15004ce6); f_150057dc(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15004cf4; continue; }
  pc = 0x15004cea; continue;
 }
 0x15004cea => {
  { let v: u32 = cpu.r32(0x15016c98u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x20u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x15004cf4; continue;
 }
 0x15004cf4 => {
  { let v: u32 = cpu.r32(0x15016c98u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004d00); }
 let mut pc: u32 = 0x15004d00;
 loop { match pc {
 0x15004d00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x40u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15016c98u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004d15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004d15); }
 let mut pc: u32 = 0x15004d15;
 loop { match pc {
 0x15004d15 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15004d2b; continue; }
  pc = 0x15004d22; continue;
 }
 0x15004d22 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15004d29); f_15004de6(cpu);
  pc = 0x15004d89; continue;
 }
 0x15004d2b => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x15004d38; continue; }
  pc = 0x15004d2f; continue;
 }
 0x15004d2f => {
  cpu.push(0x15004d34); f_15003ecc(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x15004d89; continue;
 }
 0x15004d38 => {
  cpu.push(0x15004d3d); f_1500581d(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x15004d46); f_15005826(cpu);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15004d87; continue; }
  pc = 0x15004d4c; continue;
 }
 0x15004d4c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x15004d53); f_15004de6(cpu);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15004d7e; continue; }
  pc = 0x15004d59; continue;
 }
 0x15004d59 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.push(0x2b); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let _ = cpu.eax; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, false); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.pop(); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x15004d7c); f_15003ecc(cpu);
  pc = 0x15004d87; continue;
 }
 0x15004d7e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x15004d87); f_15005826(cpu);
  pc = 0x15004d87; continue;
 }
 0x15004d87 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x15004d89; continue;
 }
 0x15004d89 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004d8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004d8f); }
 let mut pc: u32 = 0x15004d8f;
 loop { match pc {
 0x15004d8f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004d99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004d99); }
 let mut pc: u32 = 0x15004d99;
 loop { match pc {
 0x15004d99 => {
  let t = cpu.r32(0x15016ca0u32); cpu.push(0x15004d9f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004da0); }
 let mut pc: u32 = 0x15004da0;
 loop { match pc {
 0x15004da0 => {
  let t = cpu.r32(0x15016ca4u32); cpu.push(0x15004da6); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004de6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004de6); }
 let mut pc: u32 = 0x15004de6;
 loop { match pc {
 0x15004de6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { cpu.push(0x2b); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15004dff; continue; }
  pc = 0x15004dfa; continue;
 }
 0x15004dfa => {
  { cpu.sub32(cpu.eax, 0xffffffd4u32); }
  if (cpu.cf || cpu.zf) { pc = 0x15004e06; continue; }
  pc = 0x15004dff; continue;
 }
 0x15004dff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15004ec6; continue;
 }
 0x15004e06 => {
  { let v: u32 = 0x15016ca8u32; cpu.ebp = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16) & 0xfffcu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x15016b84u32); cpu.push(0x15004e1f); cpu.call(t);
  pc = 0x15004e1f; continue;
 }
 0x15004e1f => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0xcu32); }
  if !cpu.cf { pc = 0x15004e2b; continue; }
  pc = 0x15004e26; continue;
 }
 0x15004e26 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  pc = 0x15004e2b; continue;
 }
 0x15004e2b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x15016cb0u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x15004e4b; continue; }
  pc = 0x15004e33; continue;
 }
 0x15004e33 => {
  { let v: u32 = cpu.r32(0x15016cacu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x15004e83; continue; }
  pc = 0x15004e3d; continue;
 }
 0x15004e3d => {
  { let v: u32 = cpu.ecx; cpu.w32(0x15016cb0u32, v); }
  { let v: u32 = cpu.r32(0x15016ca8u32); cpu.ecx = v; }
  pc = 0x15004e83; continue;
 }
 0x15004e4b => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x15004e3d; continue;
 }
 0x15004e4f => {
  { let v: u32 = 0x2bu32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15016cacu32, v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15004e68); f_15004ef8(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15004eb6; continue; }
  pc = 0x15004e6e; continue;
 }
 0x15004e6e => {
  { let v: u32 = cpu.r32(0x15016cb0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x15004e80; continue; }
  pc = 0x15004e7b; continue;
 }
 0x15004e7b => {
  { let v: u32 = cpu.eax; cpu.w32(0x15016cb0u32, v); }
  pc = 0x15004e80; continue;
 }
 0x15004e80 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x15004e83; continue;
 }
 0x15004e83 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x15004e4f; continue; }
  pc = 0x15004e87; continue;
 }
 0x15004e87 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esp)), 0x0u8); }
  if !cpu.zf { pc = 0x15004e98; continue; }
  pc = 0x15004e8d; continue;
 }
 0x15004e8d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15004e94); f_15005cac(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15004ead; continue; }
  pc = 0x15004e98; continue;
 }
 0x15004e98 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15004e9f); f_15005d07(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15004eb6; continue; }
  pc = 0x15004ea3; continue;
 }
 0x15004ea3 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x15004e1f; continue;
 }
 0x15004ead => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x15004e1f; continue;
 }
 0x15004eb6 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x15094ca0u32, v); }
  let t = cpu.r32(0x15016b8cu32); cpu.push(0x15004ec4); cpu.call(t);
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x15004ec6; continue;
 }
 0x15004ec6 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pop(); }
  { cpu.pop(); }
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004ee0); }
 let mut pc: u32 = 0x15004ee0;
 loop { match pc {
 0x15004ee0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x15004ef5); f_15005d10(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004ef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004ef8); }
 let mut pc: u32 = 0x15004ef8;
 loop { match pc {
 0x15004ef8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(0x2b); }
  { let _ = cpu.edx; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x15004f47; continue; }
  pc = 0x15004f02; continue;
 }
 0x15004f02 => {
  { let v: u32 = cpu.add32(cpu.eax, 0xbu32); cpu.eax = v; }
  if cpu.cf { pc = 0x15004f47; continue; }
  pc = 0x15004f07; continue;
 }
 0x15004f07 => {
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebx))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15004f47; continue; }
  pc = 0x15004f1c; continue;
 }
 0x15004f1c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15004f2b; continue; }
  pc = 0x15004f26; continue;
 }
 0x15004f26 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x15004f2b; continue;
 }
 0x15004f2b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x15004f4b; continue; }
  pc = 0x15004f31; continue;
 }
 0x15004f31 => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi & cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { pc = 0x15004f2b; continue; }
  pc = 0x15004f44; continue;
 }
 0x15004f44 => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x15004f47; continue;
 }
 0x15004f47 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x15004f96; continue;
 }
 0x15004f4b => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.ebx))); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { pc = 0x15004f79; continue; }
  pc = 0x15004f5b; continue;
 }
 0x15004f5b => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x15004f8e; continue;
 }
 0x15004f79 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x15004f8e; continue;
 }
 0x15004f8e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x15004f96; continue;
 }
 0x15004f96 => {
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15004fa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15004fa0); }
 let mut pc: u32 = 0x15004fa0;
 loop { match pc {
 0x15004f9b => {
  pc = 0x150050a6; continue;
 }
 0x15004fa0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(0x2b); }
  { let _ = cpu.edx; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x15004f9b; continue; }
  pc = 0x15004faa; continue;
 }
 0x15004faa => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x15004f9b; continue; }
  pc = 0x15004fb5; continue;
 }
 0x15004fb5 => {
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x1u32); }
  if !cpu.zf { pc = 0x15004fe2; continue; }
  pc = 0x15004fc3; continue;
 }
 0x15004fc3 => {
  { cpu.sub32(cpu.edi, cpu.r32(0xcu32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x15004fcb; continue; }
  pc = 0x15004fc8; continue;
 }
 0x15004fc8 => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x15004fcb; continue;
 }
 0x15004fcb => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x1500505e; continue;
 }
 0x15004fe2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x15004ff9; continue; }
  pc = 0x15004feb; continue;
 }
 0x15004feb => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500505e; continue; }
  pc = 0x15004ff0; continue;
 }
 0x15004ff0 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x1500505e; continue; }
  pc = 0x15004ff7; continue;
 }
 0x15004ff7 => {
  pc = 0x15005008; continue;
 }
 0x15004ff9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x1500505e; continue; }
  pc = 0x15005000; continue;
 }
 0x15005000 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.ebx))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500505e; continue; }
  pc = 0x15005008; continue;
 }
 0x15005008 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x1500503d; continue; }
  pc = 0x15005018; continue;
 }
 0x15005018 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15005025; continue; }
  pc = 0x15005023; continue;
 }
 0x15005023 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x15005025; continue;
 }
 0x15005025 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.edi = v; }
  pc = 0x15005029; continue;
 }
 0x15005029 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x1500505e; continue; }
  pc = 0x15005030; continue;
 }
 0x15005030 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1500503d; continue; }
  pc = 0x15005035; continue;
 }
 0x15005035 => {
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x15005029; continue; }
  pc = 0x1500503d; continue;
 }
 0x1500503d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x15005047; continue; }
  pc = 0x15005044; continue;
 }
 0x15005044 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  pc = 0x15005047; continue;
 }
 0x15005047 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x1500505e; continue; }
  pc = 0x1500504b; continue;
 }
 0x1500504b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x1500505e; continue; }
  pc = 0x15005052; continue;
 }
 0x15005052 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x1500505e; continue; }
  pc = 0x15005059; continue;
 }
 0x15005059 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  pc = 0x15005047; continue;
 }
 0x1500505e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.edi; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x1500507d; continue; }
  pc = 0x1500506d; continue;
 }
 0x1500506d => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x15005079; continue; }
  pc = 0x15005076; continue;
 }
 0x15005076 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x15005079; continue;
 }
 0x15005079 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x1500508e; continue;
 }
 0x1500507d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1500508e; continue;
 }
 0x1500508e => {
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.ebx))); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx))); }
  if !cpu.cf { pc = 0x1500509e; continue; }
  pc = 0x15005096; continue;
 }
 0x15005096 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebx))); }
  if (cpu.cf || cpu.zf) { pc = 0x1500509e; continue; }
  pc = 0x1500509b; continue;
 }
 0x1500509b => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1500509e; continue;
 }
 0x1500509e => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebx))); }
  if (cpu.cf || cpu.zf) { pc = 0x150050a6; continue; }
  pc = 0x150050a3; continue;
 }
 0x150050a3 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x150050a6; continue;
 }
 0x150050a6 => {
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150050ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150050ab); }
 let mut pc: u32 = 0x150050ab;
 loop { match pc {
 0x150050ab => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x150050c0); f_15005db3(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150050c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150050c4); }
 let mut pc: u32 = 0x150050c4;
 loop { match pc {
 0x150050c4 => {
  { let v: u32 = 0xdu32; cpu.eax = v; }
  f_150050c9(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150050c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150050c9); }
 let mut pc: u32 = 0x150050c9;
 loop { match pc {
 0x150050c9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150050d2); cpu.call(t);
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150050d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150050d7); }
 let mut pc: u32 = 0x150050d7;
 loop { match pc {
 0x150050d7 => {
  { let v: u32 = 0xeu32; cpu.eax = v; }
  f_150050c9(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150050ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150050ee); }
 let mut pc: u32 = 0x150050ee;
 loop { match pc {
 0x150050ee => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x150050f7); cpu.call(t);
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005102(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005102); }
 let mut pc: u32 = 0x15005102;
 loop { match pc {
 0x15005102 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x150177bcu32; cpu.eax = v; }
  cpu.push(0x15005112); f_15004c8e(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005193(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005193); }
 let mut pc: u32 = 0x15005193;
 loop { match pc {
 0x15005193 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x1500519d); cpu.call(t);
  { cpu.sub32(cpu.r32(0x15016cb8u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x150051c7; continue; }
  pc = 0x150051a6; continue;
 }
 0x150051a6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x150177d4u32; cpu.push(v); }
  cpu.push(0x150051c2); f_1500fba0(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x15016cb8u32, v); }
  pc = 0x150051c7; continue;
 }
 0x150051c7 => {
  { cpu.sub32(cpu.r32(0x15016cbcu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x150051f1; continue; }
  pc = 0x150051d0; continue;
 }
 0x150051d0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x40000000u32; cpu.push(v); }
  { let v = 0x150177dbu32; cpu.push(v); }
  cpu.push(0x150051ec); f_1500fba0(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x15016cbcu32, v); }
  pc = 0x150051f1; continue;
 }
 0x150051f1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x150051f9); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005207(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005207); }
 let mut pc: u32 = 0x15005207;
 loop { match pc {
 0x15005207 => {
  cpu.push(0x1500520c); f_15005193(cpu);
  { let v: u32 = cpu.r32(0x15016cbcu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005212(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005212); }
 let mut pc: u32 = 0x15005212;
 loop { match pc {
 0x15005212 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff0u32); }
  if !cpu.zf { pc = 0x1500529a; continue; }
  pc = 0x1500523f; continue;
 }
 0x1500523f => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x15005258; continue; }
  pc = 0x15005252; continue;
 }
 0x15005252 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x15005258; continue;
 }
 0x15005258 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx | cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.zf { pc = 0x1500528b; continue; }
  pc = 0x15005283; continue;
 }
 0x15005283 => {
  { let v: u32 = cpu.r32(0x150177e4u32); cpu.edx = v; }
  pc = 0x15005291; continue;
 }
 0x1500528b => {
  { let v: u32 = cpu.r32(0x150177e8u32); cpu.edx = v; }
  pc = 0x15005291; continue;
 }
 0x15005291 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1500529c; continue;
 }
 0x1500529a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1500529c; continue;
 }
 0x1500529c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150052eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150052eb); }
 let mut pc: u32 = 0x150052eb;
 loop { match pc {
 0x150052a6 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x150052cb; continue; }
  pc = 0x150052b9; continue;
 }
 0x150052b9 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x150052c7); f_15005dbb(cpu);
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x150052e3; continue;
 }
 0x150052cb => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if !cpu.zf { pc = 0x150052d3; continue; }
  pc = 0x150052cf; continue;
 }
 0x150052cf => {
  { cpu.fpush(std::f64::consts::LOG10_2); }
  pc = 0x150052dd; continue;
 }
 0x150052d3 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x150052db; continue; }
  pc = 0x150052d7; continue;
 }
 0x150052d7 => {
  { cpu.fpush(1.0); }
  pc = 0x150052dd; continue;
 }
 0x150052db => {
  { cpu.fpush(std::f64::consts::LN_2); }
  pc = 0x150052dd; continue;
 }
 0x150052dd => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x150052e3; continue;
 }
 0x150052e3 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150052eb => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x150052a6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005313(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005313); }
 let mut pc: u32 = 0x15005313;
 loop { match pc {
 0x15005313 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1500532d); f_15005df8(cpu);
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x15005342; continue; }
  pc = 0x15005336; continue;
 }
 0x15005336 => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x150177ecu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x15005342; continue;
 }
 0x15005342 => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500534f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500534f); }
 let mut pc: u32 = 0x1500534f;
 loop { match pc {
 0x1500534f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x15005375; continue; }
  pc = 0x15005360; continue;
 }
 0x15005360 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15005372); f_15005390(cpu);
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x15005375; continue;
 }
 0x15005375 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x1500537f); f_15005e4c(cpu);
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x15005388); f_15005e12(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005390); }
 let mut pc: u32 = 0x15005390;
 loop { match pc {
 0x15005390 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x15017a34u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x150053bb; continue; }
  pc = 0x150053ab; continue;
 }
 0x150053ab => {
  { let v: u32 = cpu.r32(0x15017a44u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x15017a48u32); cpu.eax = v; }
  pc = 0x150053d8; continue;
 }
 0x150053bb => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x15017a3cu32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x150053dc; continue; }
  pc = 0x150053ca; continue;
 }
 0x150053ca => {
  { let v: u32 = cpu.r32(0x15017a4cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x15017a50u32); cpu.eax = v; }
  pc = 0x150053d8; continue;
 }
 0x150053d8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x150053dc; continue;
 }
 0x150053dc => {
  { let v: u32 = 0x15017a5cu32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x1500541f; continue; }
  pc = 0x150053e5; continue;
 }
 0x150053e5 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500543d; continue; }
  pc = 0x150053f1; continue;
 }
 0x150053f1 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x15016cc8u32), 0x1u8); }
  if cpu.zf { pc = 0x15005411; continue; }
  pc = 0x150053fe; continue;
 }
 0x150053fe => {
  { let v = cpu.r32(0x15017a62u32); cpu.push(v); }
  { let v = cpu.r32(0x15017a5eu32); cpu.push(v); }
  cpu.push(0x1500540f); f_15006610(cpu);
  pc = 0x15005417; continue;
 }
 0x15005411 => {
  { let v = cpu.st(0) / cpu.rf64(0x15017a5eu32); cpu.set_st(0, v); }
  pc = 0x15005417; continue;
 }
 0x15005417 => {
  { let v: u32 = cpu.edx.wrapping_sub(0xd8u32); cpu.edx = v; }
  pc = 0x15005439; continue;
 }
 0x1500541f => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500543d; continue; }
  pc = 0x15005429; continue;
 }
 0x15005429 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x15017a5eu32); cpu.set_st(0, v); }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  pc = 0x15005439; continue;
 }
 0x15005439 => {
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x1500543d; continue;
 }
 0x1500543d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x1500544b; continue;
 }
 0x1500544b => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x1500545d; continue; }
  pc = 0x15005452; continue;
 }
 0x15005452 => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x2u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x1500545d; continue;
 }
 0x1500545d => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1500546e; continue; }
  pc = 0x15005461; continue;
 }
 0x15005461 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x1500544b; continue; }
  pc = 0x15005469; continue;
 }
 0x15005469 => {
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  pc = 0x1500544b; continue;
 }
 0x1500546e => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x15005493; continue; }
  pc = 0x15005472; continue;
 }
 0x15005472 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x15016cc8u32), 0x1u8); }
  if cpu.zf { pc = 0x1500548e; continue; }
  pc = 0x1500547f; continue;
 }
 0x1500547f => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x1500548c); f_15006610(cpu);
  pc = 0x1500549a; continue;
 }
 0x1500548e => {
  { let v = cpu.st(0) / cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x1500549a; continue;
 }
 0x15005493 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x1500549a; continue;
 }
 0x1500549a => {
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150054ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150054ad); }
 let mut pc: u32 = 0x150054ad;
 loop { match pc {
 0x150054ad => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x150054de); f_15005212(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150054e9; continue; }
  pc = 0x150054e2; continue;
 }
 0x150054e2 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x1500567e; continue;
 }
 0x150054e9 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x15005513; continue; }
  pc = 0x1500550b; continue;
 }
 0x1500550b => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1500563b; continue; }
  pc = 0x15005513; continue;
 }
 0x15005513 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x1500552b; continue; }
  pc = 0x1500551d; continue;
 }
 0x1500551d => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x1500552b; continue;
 }
 0x1500552b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15005539); f_150066f4(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x15005543; continue; }
  pc = 0x15005541; continue;
 }
 0x15005541 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x15005543; continue;
 }
 0x15005543 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x5u32); cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x15005563; continue; }
  pc = 0x15005561; continue;
 }
 0x15005561 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x15005563; continue;
 }
 0x15005563 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.eax, 0x46u32); }
  if !cpu.zf { pc = 0x1500556f; continue; }
  pc = 0x1500556d; continue;
 }
 0x1500556d => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x1500556f; continue;
 }
 0x1500556f => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x15005609; continue; }
  pc = 0x15005577; continue;
 }
 0x15005577 => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15005581; continue; }
  pc = 0x1500557c; continue;
 }
 0x1500557c => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x15005581; continue;
 }
 0x15005581 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15005597); f_15005df8(cpu);
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x150055ab; continue; }
  pc = 0x150055a0; continue;
 }
 0x150055a0 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x150055ab; continue; }
  pc = 0x150055a4; continue;
 }
 0x150055a4 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x150055ab; continue;
 }
 0x150055ab => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x150055bf); f_1500534f(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15005609; continue; }
  pc = 0x150055c5; continue;
 }
 0x150055c5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if cpu.zf { pc = 0x150055d3; continue; }
  pc = 0x150055cd; continue;
 }
 0x150055cd => {
  { cpu.sub8(cpu.r8(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x150055d5; continue; }
  pc = 0x150055d3; continue;
 }
 0x150055d3 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  pc = 0x150055d5; continue;
 }
 0x150055d5 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x150055e1; continue; }
  pc = 0x150055da; continue;
 }
 0x150055da => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x150055eb; continue;
 }
 0x150055e1 => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x150055eb; continue; }
  pc = 0x150055e6; continue;
 }
 0x150055e6 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x150055eb; continue;
 }
 0x150055eb => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x150055ff); f_1500534f(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15005609; continue; }
  pc = 0x15005603; continue;
 }
 0x15005603 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x15005609; continue;
 }
 0x15005609 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd6u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500563b; continue; }
  pc = 0x1500562f; continue;
 }
 0x1500562f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if !cpu.zf { pc = 0x1500563b; continue; }
  pc = 0x15005639; continue;
 }
 0x15005639 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  pc = 0x1500563b; continue;
 }
 0x1500563b => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x15005647; continue; }
  pc = 0x15005640; continue;
 }
 0x15005640 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x15005651; continue;
 }
 0x15005647 => {
  { cpu.sub32(cpu.ebx, 0x11u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15005651; continue; }
  pc = 0x1500564c; continue;
 }
 0x1500564c => {
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  pc = 0x15005651; continue;
 }
 0x15005651 => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x15005662); f_15005eb7(cpu);
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x15005663; continue;
 }
 0x15005663 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1500567b; continue; }
  pc = 0x1500566b; continue;
 }
 0x1500566b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x15005663; continue; }
  pc = 0x1500567b; continue;
 }
 0x1500567b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x1500567e; continue;
 }
 0x1500567e => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(32);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500571a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500571a); }
 let mut pc: u32 = 0x1500571a;
 loop { match pc {
 0x1500571a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x15005728; continue;
 }
 0x15005728 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1500574c; continue; }
  pc = 0x1500572e; continue;
 }
 0x1500572e => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x15005728; continue;
 }
 0x1500574c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x43300000u32; cpu.edi = v; }
  cpu.push(0x15005758); f_15005765(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005765(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005765); }
 let mut pc: u32 = 0x15005765;
 loop { match pc {
 0x15005765 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.edx); cpu.esi = v; }
  if cpu.zf { pc = 0x150057b5; continue; }
  pc = 0x1500576d; continue;
 }
 0x1500576d => {
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if !cpu.zf { pc = 0x15005789; continue; }
  pc = 0x15005775; continue;
 }
 0x15005775 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if cpu.zf { pc = 0x15005775; continue; }
  pc = 0x15005787; continue;
 }
 0x15005787 => {
  pc = 0x150057ad; continue;
 }
 0x15005789 => {
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if cpu.zf { pc = 0x150057ad; continue; }
  pc = 0x15005791; continue;
 }
 0x15005791 => {
  { let v: u32 = cpu.edi.wrapping_add(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if !cpu.zf { pc = 0x15005791; continue; }
  pc = 0x150057a5; continue;
 }
 0x150057a5 => {
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x150057ad; continue;
 }
 0x150057ad => {
  { let v: u32 = cpu.edx & 0xfffffu32; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edi); cpu.edx = v; }
  pc = 0x150057b5; continue;
 }
 0x150057b5 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150057b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150057b6); }
 let mut pc: u32 = 0x150057b6;
 loop { match pc {
 0x150057b6 => {
  { cpu.sub8(cpu.r8(0x15016cc0u32), 0x0u8); }
  if cpu.zf { pc = 0x150057d9; continue; }
  pc = 0x150057bf; continue;
 }
 0x150057bf => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.unimplemented(0x150057c7, "Int3");
  pc = 0x150057d0; continue;
 }
 0x150057d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150057d9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150057dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150057dc); }
 let mut pc: u32 = 0x150057dc;
 loop { match pc {
 0x150057dc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x150057f3); cpu.call(t);
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x150057f9); f_1500fb9a(cpu);
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x1500580f; continue; }
  pc = 0x150057fe; continue;
 }
 0x150057fe => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15005806); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500580f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15005817); cpu.call(t);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500581d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500581d); }
 let mut pc: u32 = 0x1500581d;
 loop { match pc {
 0x1500581d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x4u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005826(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005826); }
 let mut pc: u32 = 0x15005826;
 loop { match pc {
 0x15005826 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x15016b84u32); cpu.push(0x15005836); cpu.call(t);
  { let v: u32 = 0x15016ca8u32; cpu.eax = v; }
  { let v: u32 = 0x2bu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x1500584b); f_15005860(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150059ef; continue; }
  pc = 0x15005853; continue;
 }
 0x15005853 => {
  let t = cpu.r32(0x15016b8cu32); cpu.push(0x15005859); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x150059f7; continue;
 }
 0x150059ef => {
  let t = cpu.r32(0x15016b8cu32); cpu.push(0x150059f5); cpu.call(t);
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x150059f7; continue;
 }
 0x150059f7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005860); }
 let mut pc: u32 = 0x15005860;
 loop { match pc {
 0x15005860 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x7u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.cf { pc = 0x15005877; continue; }
  pc = 0x15005872; continue;
 }
 0x15005872 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x15005877; continue;
 }
 0x15005877 => {
  { cpu.sub32(cpu.eax, 0xcu32); }
  if !cpu.cf { pc = 0x15005881; continue; }
  pc = 0x1500587c; continue;
 }
 0x1500587c => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  pc = 0x15005881; continue;
 }
 0x15005881 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x1500598e; continue; }
  pc = 0x15005895; continue;
 }
 0x15005895 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x1500589d; continue;
 }
 0x1500589d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x150058b0; continue; }
  pc = 0x150058a6; continue;
 }
 0x150058a6 => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  pc = 0x150059e6; continue;
 }
 0x150058b0 => {
  { cpu.and32(cpu.ebp, 0x1u32); }
  if !cpu.zf { pc = 0x15005987; continue; }
  pc = 0x150058bc; continue;
 }
 0x150058bc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x2bu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x15016ca8u32; cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x0u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x150058f1; continue; }
  pc = 0x150058d7; continue;
 }
 0x150058d7 => {
  { let v: u32 = cpu.r32(0x15016ca8u32); cpu.esi = v; }
  pc = 0x150058ea; continue;
 }
 0x150058df => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x150058e7; continue; }
  pc = 0x150058e3; continue;
 }
 0x150058e3 => {
  { cpu.sub32(cpu.edi, cpu.edx); }
  if cpu.cf { pc = 0x150058f1; continue; }
  pc = 0x150058e7; continue;
 }
 0x150058e7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x150058ea; continue;
 }
 0x150058ea => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x150058df; continue; }
  pc = 0x150058f1; continue;
 }
 0x150058f1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { pc = 0x150058fe; continue; }
  pc = 0x150058f8; continue;
 }
 0x150058f8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x150058fe; continue;
 }
 0x150058fe => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.ecx))); }
  if cpu.cf { pc = 0x1500590d; continue; }
  pc = 0x15005902; continue;
 }
 0x15005902 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xcu32); }
  if !cpu.cf { pc = 0x15005941; continue; }
  pc = 0x1500590d; continue;
 }
 0x1500590d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x15094ca1u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.ebp, cpu.edx); }
  if cpu.cf { pc = 0x1500597c; continue; }
  pc = 0x1500593c; continue;
 }
 0x1500593c => {
  pc = 0x150059e4; continue;
 }
 0x15005941 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x15094ca1u32, v); }
  pc = 0x150059e6; continue;
 }
 0x1500597c => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  pc = 0x1500589d; continue;
 }
 0x15005987 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x150059e6; continue;
 }
 0x1500598e => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x150059e4; continue; }
  pc = 0x15005995; continue;
 }
 0x15005995 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) | 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0x15016ca8u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2bu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x150059d5; continue; }
  pc = 0x150059bb; continue;
 }
 0x150059bb => {
  { let v: u32 = cpu.r32(0x15016ca8u32); cpu.esi = v; }
  pc = 0x150059ce; continue;
 }
 0x150059c3 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x150059cb; continue; }
  pc = 0x150059c7; continue;
 }
 0x150059c7 => {
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if cpu.cf { pc = 0x150059d5; continue; }
  pc = 0x150059cb; continue;
 }
 0x150059cb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x150059ce; continue;
 }
 0x150059ce => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x150059c3; continue; }
  pc = 0x150059d5; continue;
 }
 0x150059d5 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  cpu.push(0x150059e4); f_15003ecc(cpu);
  pc = 0x150059e4; continue;
 }
 0x150059e4 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x150059e6; continue;
 }
 0x150059e6 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150059fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150059fe); }
 let mut pc: u32 = 0x150059fe;
 loop { match pc {
 0x150059fe => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15094c84u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x15005a54; continue; }
  pc = 0x15005a23; continue;
 }
 0x15005a23 => {
  { let v: u32 = cpu.r32(0x15016bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15016bccu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15005a3e); f_15004ee0(cpu);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15005a45); f_150034b4(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15005a54; continue; }
  pc = 0x15005a49; continue;
 }
 0x15005a49 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15005a52); f_1500fc30(cpu);
  pc = 0x15005aa4; continue;
 }
 0x15005a54 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15005a5d); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.w32(0xdeu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x15005a7b; continue; }
  pc = 0x15005a6a; continue;
 }
 0x15005a6a => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xcu32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfffu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8(((cpu.eax >> 8) as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x15005a7b; continue;
 }
 0x15005a7b => {
  let t = cpu.r32(0x15016b68u32); cpu.push(0x15005a81); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15005a8c); f_1500fb94(cpu);
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x15005a94); f_150045db(cpu);
  let t = cpu.r32(0x15016ba8u32); cpu.push(0x15005a9a); cpu.call(t);
  { let v: u32 = cpu.edi; cpu.eax = v; }
  let t = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.push(0x15005a9f); cpu.call(t);
  cpu.push(0x15005aa4); f_15004d99(cpu);
  pc = 0x15005aa4; continue;
 }
 0x15005aa4 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005aaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005aaa); }
 let mut pc: u32 = 0x15005aaa;
 loop { match pc {
 0x15005aaa => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(0x2b); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15016b64u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x15005ad5; continue; }
  pc = 0x15005ac0; continue;
 }
 0x15005ac0 => {
  cpu.push(0x15005ac5); f_15003468(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15005ad0; continue; }
  pc = 0x15005ac9; continue;
 }
 0x15005ac9 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x15005b76; continue;
 }
 0x15005ad0 => {
  cpu.push(0x15005ad5); f_15003578(cpu);
  pc = 0x15005ad5; continue;
 }
 0x15005ad5 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xfffu32); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x15005aea); f_1500fb8e(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v: u32 = cpu.ebp & 0xfffff000u32; cpu.ebp = v; }
  { let _ = cpu.eax; }
  { let v: u32 = 0x150177f4u32; cpu.esi = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.push(0x15005b0e); f_1500fc2a(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x15005b17); f_150067d2(cpu);
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15005b25); f_1500fb88(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x150059feu32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15005b4a); f_1500fb82(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15005b5c; continue; }
  pc = 0x15005b52; continue;
 }
 0x15005b52 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  pc = 0x15005b68; continue;
 }
 0x15005b5c => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15005b68); f_1500fc1e(cpu);
  pc = 0x15005b68; continue;
 }
 0x15005b68 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15005b72); f_1500fc30(cpu);
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x15005b76; continue;
 }
 0x15005b76 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005b7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005b7e); }
 let mut pc: u32 = 0x15005b7e;
 loop { match pc {
 0x15005b7e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x15016bacu32); cpu.push(0x15005b86); cpu.call(t);
  cpu.push(0x15005b8b); f_15004619(cpu);
  { cpu.sub32(cpu.r32(0x15094c84u32), 0x0u32); }
  if !cpu.zf { pc = 0x15005b9e; continue; }
  pc = 0x15005b94; continue;
 }
 0x15005b94 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x15005b9e); f_15003501(cpu);
  pc = 0x15005b9e; continue;
 }
 0x15005b9e => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15005ba5); f_1500fb7c(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005ba8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005ba8); }
 let mut pc: u32 = 0x15005ba8;
 loop { match pc {
 0x15005ba8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15016ca8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x15005bbe; continue;
 }
 0x15005bb5 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x15005bc2; continue; }
  pc = 0x15005bb9; continue;
 }
 0x15005bb9 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x15005bbe; continue;
 }
 0x15005bbe => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15005bb5; continue; }
  pc = 0x15005bc2; continue;
 }
 0x15005bc2 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x15005bd1; continue; }
  pc = 0x15005bcc; continue;
 }
 0x15005bcc => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x15005bd7; continue;
 }
 0x15005bd1 => {
  { let v: u32 = cpu.edx; cpu.w32(0x15016ca8u32, v); }
  pc = 0x15005bd7; continue;
 }
 0x15005bd7 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15005bde; continue; }
  pc = 0x15005bdb; continue;
 }
 0x15005bdb => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x15005bde; continue;
 }
 0x15005bde => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2cu32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005c1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005c1c); }
 let mut pc: u32 = 0x15005c1c;
 loop { match pc {
 0x15005c1c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x15016cccu32), 0x0u32); }
  if !cpu.zf { pc = 0x15005c32; continue; }
  pc = 0x15005c2b; continue;
 }
 0x15005c2b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x15005ca3; continue;
 }
 0x15005c32 => {
  { cpu.sub32(cpu.r32(0x15016b14u32), 0xfffffffeu32); }
  if cpu.zf { pc = 0x15005c2b; continue; }
  pc = 0x15005c3b; continue;
 }
 0x15005c3b => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x15005c42); f_15005cbd(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15005ca3; continue; }
  pc = 0x15005c46; continue;
 }
 0x15005c46 => {
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x1000u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x15005c59); f_1500fb76(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15005ca3; continue; }
  pc = 0x15005c5f; continue;
 }
 0x15005c5f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15005c2b; continue; }
  pc = 0x15005c6c; continue;
 }
 0x15005c6c => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x38u32); }
  if cpu.cf { pc = 0x15005c2b; continue; }
  pc = 0x15005c74; continue;
 }
 0x15005c74 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15005c7d); f_15005ba8(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x15005c9e); f_15003ecc(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x15005ca3; continue;
 }
 0x15005ca3 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005cac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005cac); }
 let mut pc: u32 = 0x15005cac;
 loop { match pc {
 0x15005cac => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x15005cb4); f_1500680e(cpu);
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15005cbb); f_15005c1c(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005cbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005cbd); }
 let mut pc: u32 = 0x15005cbd;
 loop { match pc {
 0x15005cbd => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xbu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15005d04; continue; }
  pc = 0x15005ccc; continue;
 }
 0x15005ccc => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.cf { pc = 0x15005cda; continue; }
  pc = 0x15005cd5; continue;
 }
 0x15005cd5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15005cda => {
  { let v: u32 = cpu.r32(0x15016c3cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x15005ce8; continue; }
  pc = 0x15005ce4; continue;
 }
 0x15005ce4 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x15005ce8; continue;
 }
 0x15005ce8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0xfffu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if cpu.cf { pc = 0x15005cd5; continue; }
  pc = 0x15005cf3; continue;
 }
 0x15005cf3 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  pc = 0x15005d04; continue;
 }
 0x15005d04 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005d07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005d07); }
 let mut pc: u32 = 0x15005d07;
 loop { match pc {
 0x15005d07 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005d10); }
 let mut pc: u32 = 0x15005d10;
 loop { match pc {
 0x15005d10 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005d40; continue; }
  pc = 0x15005d14; continue;
 }
 0x15005d14 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  pc = 0x15005d16; continue;
 }
 0x15005d16 => {
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x15005d23; continue; }
  pc = 0x15005d1a; continue;
 }
 0x15005d1a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15005d16; continue; }
  pc = 0x15005d23; continue;
 }
 0x15005d23 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  cpu.push(0x15005d2c); f_15005d47(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005d40; continue; }
  pc = 0x15005d32; continue;
 }
 0x15005d32 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005d40; continue; }
  pc = 0x15005d37; continue;
 }
 0x15005d37 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005d40; continue; }
  pc = 0x15005d3d; continue;
 }
 0x15005d3d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x15005d40; continue;
 }
 0x15005d40 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005d47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005d47); }
 let mut pc: u32 = 0x15005d47;
 loop { match pc {
 0x15005d47 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005db2; continue; }
  pc = 0x15005d4b; continue;
 }
 0x15005d4b => {
  { cpu.and8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x15005d57; continue; }
  pc = 0x15005d4f; continue;
 }
 0x15005d4f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15005d4b; continue; }
  pc = 0x15005d57; continue;
 }
 0x15005d57 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005d97; continue; }
  pc = 0x15005d5d; continue;
 }
 0x15005d5d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005d89; continue; }
  pc = 0x15005d60; continue;
 }
 0x15005d60 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x15005d86; continue; }
  pc = 0x15005d6e; continue;
 }
 0x15005d6e => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x15005d60; continue; }
  pc = 0x15005d83; continue;
 }
 0x15005d83 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x15005d86; continue;
 }
 0x15005d86 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x15005d89; continue;
 }
 0x15005d89 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x15005d97; continue;
 }
 0x15005d97 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005db2; continue; }
  pc = 0x15005d9d; continue;
 }
 0x15005d9d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005db2; continue; }
  pc = 0x15005da5; continue;
 }
 0x15005da5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x15005db2; continue; }
  pc = 0x15005dad; continue;
 }
 0x15005dad => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x15005db2; continue;
 }
 0x15005db2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005db3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005db3); }
 let mut pc: u32 = 0x15005db3;
 loop { match pc {
 0x15005db3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005dbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005dbb); }
 let mut pc: u32 = 0x15005dbb;
 loop { match pc {
 0x15005dbb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x15005de4; continue; }
  pc = 0x15005dcc; continue;
 }
 0x15005dcc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x15005de4; continue; }
  pc = 0x15005dda; continue;
 }
 0x15005dda => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x15005de4; continue; }
  pc = 0x15005dde; continue;
 }
 0x15005dde => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1080u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x15005de8; continue;
 }
 0x15005de4 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1040u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x15005de8; continue;
 }
 0x15005de8 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x15005df0); f_150068d3(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005df8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005df8); }
 let mut pc: u32 = 0x15005df8;
 loop { match pc {
 0x15005df8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  cpu.push(0x15005e04); f_150030a4(cpu);
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005e12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005e12); }
 let mut pc: u32 = 0x15005e12;
 loop { match pc {
 0x15005e12 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x15005e19); f_15005f21(cpu);
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x15005e24; continue;
 }
 0x15005e24 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x15005e2d; continue; }
  pc = 0x15005e29; continue;
 }
 0x15005e29 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x15005e2d; continue;
 }
 0x15005e2d => {
  if cpu.cf { pc = 0x15005e35; continue; }
  pc = 0x15005e2f; continue;
 }
 0x15005e2f => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x15005e24; continue;
 }
 0x15005e35 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x15005e38; continue;
 }
 0x15005e38 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x15005e41; continue; }
  pc = 0x15005e3d; continue;
 }
 0x15005e3d => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x15005e41; continue;
 }
 0x15005e41 => {
  if !cpu.cf { pc = 0x15005e49; continue; }
  pc = 0x15005e43; continue;
 }
 0x15005e43 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x15005e38; continue;
 }
 0x15005e49 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005e4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005e4c); }
 let mut pc: u32 = 0x15005e4c;
 loop { match pc {
 0x15005e4c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp & 0xfff00000u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ 0x100000u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 20u32); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x433u32); cpu.ebp = v; }
  if cpu.zf { pc = 0x15005eaa; continue; }
  pc = 0x15005e75; continue;
 }
 0x15005e75 => {
  if (cpu.cf || cpu.zf) { pc = 0x15005e80; continue; }
  pc = 0x15005e77; continue;
 }
 0x15005e77 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcl32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15005e77; continue; }
  pc = 0x15005e7e; continue;
 }
 0x15005e7e => {
  pc = 0x15005eaa; continue;
 }
 0x15005e80 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x15005e84; continue;
 }
 0x15005e84 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15005e84; continue; }
  pc = 0x15005e8f; continue;
 }
 0x15005e8f => {
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  if cpu.cf { pc = 0x15005eaa; continue; }
  pc = 0x15005e96; continue;
 }
 0x15005e96 => {
  if !cpu.zf { pc = 0x15005ea4; continue; }
  pc = 0x15005e98; continue;
 }
 0x15005e98 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x15005ea4; continue; }
  pc = 0x15005e9c; continue;
 }
 0x15005e9c => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x15005eaa; continue; }
  pc = 0x15005ea4; continue;
 }
 0x15005ea4 => {
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, 0x0u32); cpu.ebx = v; }
  pc = 0x15005eaa; continue;
 }
 0x15005eaa => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005eb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005eb7); }
 let mut pc: u32 = 0x15005eb7;
 loop { match pc {
 0x15005eb7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x15005ecc; continue;
 }
 0x15005ecc => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x15005edf; continue; }
  pc = 0x15005ed8; continue;
 }
 0x15005ed8 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15005ee1; continue; }
  pc = 0x15005edc; continue;
 }
 0x15005edc => {
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x15005ef7; continue;
 }
 0x15005edf => {
  { let v = cpu.edi; cpu.div32(v); }
  pc = 0x15005ee1; continue;
 }
 0x15005ee1 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edi; cpu.div32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x15005ef0, "Aam");
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x15005ef5, "Aam");
  pc = 0x15005ef7; continue;
 }
 0x15005ef7 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3030u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x3030u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x4u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x15005f1b; continue; }
  pc = 0x15005f09; continue;
 }
 0x15005f09 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x15005f1b; continue; }
  pc = 0x15005f0f; continue;
 }
 0x15005f0f => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x15005f1b; continue; }
  pc = 0x15005f15; continue;
 }
 0x15005f15 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x15005ecc; continue; }
  pc = 0x15005f1b; continue;
 }
 0x15005f1b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005f21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005f21); }
 let mut pc: u32 = 0x15005f21;
 loop { match pc {
 0x15005f21 => {
  cpu.push(0x15005f26); f_15005fd6(cpu);
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.fs_base)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.fs_base), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x15005f53, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x186u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x15005f64, "Sldt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9896u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x9896u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x15005f6a; continue; }
  pc = 0x15005f75; continue;
 }
 0x15005f6a => {
  { let v: u8 = cpu.adc8(cpu.r8(0x98u32.wrapping_add(cpu.esi)), 0x0u8); cpu.w8(0x98u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x15005f6a; continue; }
  pc = 0x15005f75; continue;
 }
 0x15005f75 => {
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x15005f83); cpu.call(0x0);
  cpu.unimplemented(0x15005f83, "In");
  { let v = cpu.esp; cpu.push(v); }
  { cpu.pop(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x15005f90); cpu.call(0x15e8a806);
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, false); }
  cpu.unimplemented(0x15005f95, "Aam");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x5af34e72u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x7au32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x7au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0xffffff8du32.wrapping_add(cpu.esi)), ((cpu.ebx >> 8) as u8)); cpu.w8(0xffffff8du32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x86f2a4c6u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x86f2a4c6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax & cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.r32(0x78u32.wrapping_add(cpu.edi)), 69u32); cpu.w32(0x78u32.wrapping_add(cpu.edi), v); }
  cpu.unimplemented(0x15005fb8, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffb3u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xe0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | 0xa7640000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x23u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x15005fc8); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005fd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005fd6); }
 let mut pc: u32 = 0x15005fd6;
 loop { match pc {
 0x15005fd6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15005fd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15005fd8); }
 let mut pc: u32 = 0x15005fd8;
 loop { match pc {
 0x15005fd8 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x15005fe0; continue;
 }
 0x15005fe0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x15006072; continue; }
  pc = 0x15005fec; continue;
 }
 0x15005fec => {
  { let v: u32 = cpu.eax ^ 0xe000000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xe000000u32); }
  if cpu.zf { pc = 0x15005ffb; continue; }
  pc = 0x15005ff8; continue;
 }
 0x15005ff8 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15005ffb => {
  { let v: u32 = cpu.shr32(cpu.eax, 28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x15017af8u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1500600a; continue; }
  pc = 0x15006007; continue;
 }
 0x15006007 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500600a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1500607c; continue; }
  pc = 0x15006015; continue;
 }
 0x15006015 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x1500607c; continue; }
  pc = 0x1500601c; continue;
 }
 0x1500601c => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1500605b; continue; }
  pc = 0x15006044; continue;
 }
 0x15006044 => {
  { let v = cpu.st(0) * cpu.rf32(0x15017b08u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x15017b08u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500605b => {
  { let v = cpu.st(0) * cpu.rf32(0x15017b0cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x15017b0cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15006072 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500607f; continue; }
  pc = 0x1500607c; continue;
 }
 0x1500607c => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500607f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500607c; continue; }
  pc = 0x1500608a; continue;
 }
 0x1500608a => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x150060c0; continue; }
  pc = 0x150060af; continue;
 }
 0x150060af => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x150060e8; continue; }
  pc = 0x150060b6; continue;
 }
 0x150060b6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x150060e8; continue; }
  pc = 0x150060be; continue;
 }
 0x150060be => {
  pc = 0x150060c8; continue;
 }
 0x150060c0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x150060e8; continue; }
  pc = 0x150060c8; continue;
 }
 0x150060c8 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15017b10u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  {  }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  pc = 0x15005fe0; continue;
 }
 0x150060e8 => {
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150060f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150060f9); }
 let mut pc: u32 = 0x150060f9;
 loop { match pc {
 0x150060f9 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150060ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150060ff); }
 let mut pc: u32 = 0x150060ff;
 loop { match pc {
 0x150060ff => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x15006102, "Int");
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500610a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500610a); }
 let mut pc: u32 = 0x1500610a;
 loop { match pc {
 0x1500610a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1500610d, "Int");
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006115(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006115); }
 let mut pc: u32 = 0x15006115;
 loop { match pc {
 0x15006115 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500611b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500611b); }
 let mut pc: u32 = 0x1500611b;
 loop { match pc {
 0x1500611b => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006121(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006121); }
 let mut pc: u32 = 0x15006121;
 loop { match pc {
 0x15006121 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006127(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006127); }
 let mut pc: u32 = 0x15006127;
 loop { match pc {
 0x15006127 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006139); f_15005fd8(cpu);
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006143(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006143); }
 let mut pc: u32 = 0x15006143;
 loop { match pc {
 0x15006143 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x15006146, "Int");
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006154); f_15005fd8(cpu);
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500615e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500615e); }
 let mut pc: u32 = 0x1500615e;
 loop { match pc {
 0x1500615e => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x15006161, "Int");
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006177); f_15005fd8(cpu);
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500617f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500617f); }
 let mut pc: u32 = 0x1500617f;
 loop { match pc {
 0x1500617f => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500618b); f_15005fd8(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500618f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500618f); }
 let mut pc: u32 = 0x1500618f;
 loop { match pc {
 0x1500618f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500619b); f_15005fd8(cpu);
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150061a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150061a3); }
 let mut pc: u32 = 0x150061a3;
 loop { match pc {
 0x150061a3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150061af); f_15005fd8(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150061b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150061b3); }
 let mut pc: u32 = 0x150061b3;
 loop { match pc {
 0x150061b3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150061c7); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150061d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150061d3); }
 let mut pc: u32 = 0x150061d3;
 loop { match pc {
 0x150061d3 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x150061d6, "Int");
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150061e6); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150061f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150061f2); }
 let mut pc: u32 = 0x150061f2;
 loop { match pc {
 0x150061f2 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x150061f5, "Int");
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500620d); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006217(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006217); }
 let mut pc: u32 = 0x15006217;
 loop { match pc {
 0x15006217 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006225); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500622b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500622b); }
 let mut pc: u32 = 0x1500622b;
 loop { match pc {
 0x1500622b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006239); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006243(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006243); }
 let mut pc: u32 = 0x15006243;
 loop { match pc {
 0x15006243 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006251); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006257(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006257); }
 let mut pc: u32 = 0x15006257;
 loop { match pc {
 0x15006257 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500626b); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006277(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006277); }
 let mut pc: u32 = 0x15006277;
 loop { match pc {
 0x15006277 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1500627a, "Int");
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500628a); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006296(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006296); }
 let mut pc: u32 = 0x15006296;
 loop { match pc {
 0x15006296 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x15006299, "Int");
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150062b1); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150062bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150062bb); }
 let mut pc: u32 = 0x150062bb;
 loop { match pc {
 0x150062bb => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150062c9); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150062cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150062cf); }
 let mut pc: u32 = 0x150062cf;
 loop { match pc {
 0x150062cf => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150062dd); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150062e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150062e7); }
 let mut pc: u32 = 0x150062e7;
 loop { match pc {
 0x150062e7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150062f5); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150062fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150062fb); }
 let mut pc: u32 = 0x150062fb;
 loop { match pc {
 0x150062fb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500630f); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500631b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500631b); }
 let mut pc: u32 = 0x1500631b;
 loop { match pc {
 0x1500631b => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1500631e, "Int");
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500632e); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500633a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500633a); }
 let mut pc: u32 = 0x1500633a;
 loop { match pc {
 0x1500633a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1500633d, "Int");
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006355); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500635f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500635f); }
 let mut pc: u32 = 0x1500635f;
 loop { match pc {
 0x1500635f => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500636d); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006373(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006373); }
 let mut pc: u32 = 0x15006373;
 loop { match pc {
 0x15006373 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006381); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500638b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500638b); }
 let mut pc: u32 = 0x1500638b;
 loop { match pc {
 0x1500638b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006399); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500639f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500639f); }
 let mut pc: u32 = 0x1500639f;
 loop { match pc {
 0x1500639f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150063b3); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150063bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150063bf); }
 let mut pc: u32 = 0x150063bf;
 loop { match pc {
 0x150063bf => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x150063c2, "Int");
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150063d2); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150063de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150063de); }
 let mut pc: u32 = 0x150063de;
 loop { match pc {
 0x150063de => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x150063e1, "Int");
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150063f9); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006403(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006403); }
 let mut pc: u32 = 0x15006403;
 loop { match pc {
 0x15006403 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006411); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006417(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006417); }
 let mut pc: u32 = 0x15006417;
 loop { match pc {
 0x15006417 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006425); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500642f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500642f); }
 let mut pc: u32 = 0x1500642f;
 loop { match pc {
 0x1500642f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500643d); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006443(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006443); }
 let mut pc: u32 = 0x15006443;
 loop { match pc {
 0x15006443 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006457); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006463(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006463); }
 let mut pc: u32 = 0x15006463;
 loop { match pc {
 0x15006463 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x15006466, "Int");
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006476); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006482(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006482); }
 let mut pc: u32 = 0x15006482;
 loop { match pc {
 0x15006482 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x15006485, "Int");
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500649d); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150064a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150064a7); }
 let mut pc: u32 = 0x150064a7;
 loop { match pc {
 0x150064a7 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150064b5); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150064bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150064bb); }
 let mut pc: u32 = 0x150064bb;
 loop { match pc {
 0x150064bb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150064c9); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150064d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150064d3); }
 let mut pc: u32 = 0x150064d3;
 loop { match pc {
 0x150064d3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150064e1); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150064e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150064e7); }
 let mut pc: u32 = 0x150064e7;
 loop { match pc {
 0x150064e7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x150064fb); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006507(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006507); }
 let mut pc: u32 = 0x15006507;
 loop { match pc {
 0x15006507 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x1500650a, "Int");
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500651a); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006526(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006526); }
 let mut pc: u32 = 0x15006526;
 loop { match pc {
 0x15006526 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  cpu.unimplemented(0x15006529, "Int");
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006541); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500654b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500654b); }
 let mut pc: u32 = 0x1500654b;
 loop { match pc {
 0x1500654b => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006559); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500655f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500655f); }
 let mut pc: u32 = 0x1500655f;
 loop { match pc {
 0x1500655f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500656d); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
