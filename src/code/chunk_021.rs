#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_12018616(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018616); }
 let mut pc: u32 = 0x12018616;
 loop { match pc {
 0x12018616 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018620); f_12018572(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018624(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018624); }
 let mut pc: u32 = 0x12018624;
 loop { match pc {
 0x12018624 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x12018634; continue;
 }
 0x12018634 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018652; continue; }
  pc = 0x1201863c; continue;
 }
 0x1201863c => {
  { let v = 0x8u32; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018648); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201864b; continue;
 }
 0x1201864b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12018667; continue; }
  pc = 0x1201864f; continue;
 }
 0x1201864f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x12018634; continue;
 }
 0x12018652 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  pc = 0x1201864b; continue;
 }
 0x12018667 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if cpu.zf { pc = 0x12018678; continue; }
  pc = 0x12018673; continue;
 }
 0x12018673 => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if !cpu.zf { pc = 0x1201867d; continue; }
  pc = 0x12018678; continue;
 }
 0x12018678 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1201867d; continue;
 }
 0x1201867d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x12018687; continue;
 }
 0x12018687 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120186ce; continue; }
  pc = 0x12018690; continue;
 }
 0x12018690 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12018698); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201869b; continue;
 }
 0x1201869b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120186df; continue; }
  pc = 0x1201869f; continue;
 }
 0x1201869f => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120186ba); f_12018734(cpu);
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12018687; continue;
 }
 0x120186ce => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  pc = 0x1201869b; continue;
 }
 0x120186df => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.edi, 0x2du32); }
  if !cpu.zf { pc = 0x120186fb; continue; }
  pc = 0x120186ec; continue;
 }
 0x120186ec => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120186fb => {
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
pub fn f_12018703(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018703); }
 let mut pc: u32 = 0x12018703;
 loop { match pc {
 0x12018703 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12018720; continue; }
  pc = 0x12018714; continue;
 }
 0x12018714 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x12018720; continue; }
  pc = 0x1201871a; continue;
 }
 0x1201871a => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x12018714; continue; }
  pc = 0x12018720; continue;
 }
 0x12018720 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x12018732; continue; }
  pc = 0x12018724; continue;
 }
 0x12018724 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x12018732; continue; }
  pc = 0x1201872a; continue;
 }
 0x1201872a => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x12018732; continue;
 }
 0x12018732 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018734(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018734); }
 let mut pc: u32 = 0x12018734;
 loop { match pc {
 0x12018734 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1201874d; continue; }
  pc = 0x12018744; continue;
 }
 0x12018744 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x1201874d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.mul32(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018768(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018768); }
 let mut pc: u32 = 0x12018768;
 loop { match pc {
 0x12018768 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x12018770); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201877d); f_12002423(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x12018789); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018790); }
 let mut pc: u32 = 0x12018790;
 loop { match pc {
 0x12018790 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203b128u32); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x120187f5; continue; }
  pc = 0x1201879f; continue;
 }
 0x1201879f => {
  { let v: u32 = cpu.r32(0x12041140u32); cpu.edi = v; }
  pc = 0x120187a5; continue;
 }
 0x120187a5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x120187b8); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x120187fc; continue; }
  pc = 0x120187be; continue;
 }
 0x120187be => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x120187c4); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12018806; continue; }
  pc = 0x120187cd; continue;
 }
 0x120187cd => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  let t = cpu.edi; cpu.push(0x120187de); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12018810; continue; }
  pc = 0x120187e2; continue;
 }
 0x120187e2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x120187ed); f_1201881a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x120187a5; continue; }
  pc = 0x120187f5; continue;
 }
 0x120187f5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120187fc => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018806 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018810 => {
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
pub fn f_1201881a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201881a); }
 let mut pc: u32 = 0x1201881a;
 loop { match pc {
 0x1201881a => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if cpu.zf { pc = 0x12018a5f; continue; }
  pc = 0x1201882c; continue;
 }
 0x1201882c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x3du32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12018838); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x12018a5f; continue; }
  pc = 0x12018845; continue;
 }
 0x12018845 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), cpu.ebp); }
  if cpu.zf { pc = 0x12018a5f; continue; }
  pc = 0x1201884f; continue;
 }
 0x1201884f => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203b120u32); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x1203b124u32), cpu.ecx); }
  if !cpu.zf { pc = 0x12018877; continue; }
  pc = 0x12018869; continue;
 }
 0x12018869 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201886f); f_12018ac8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b120u32, v); }
  pc = 0x12018877; continue;
 }
 0x12018877 => {
  { cpu.sub32(cpu.r32(0x1203b120u32), 0x0u32); }
  if !cpu.zf { pc = 0x12018927; continue; }
  pc = 0x12018884; continue;
 }
 0x12018884 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x120188ae; continue; }
  pc = 0x1201888b; continue;
 }
 0x1201888b => {
  { cpu.sub32(cpu.r32(0x1203b128u32), 0x0u32); }
  if cpu.zf { pc = 0x120188ae; continue; }
  pc = 0x12018894; continue;
 }
 0x12018894 => {
  cpu.push(0x12018899); f_12018790(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12018927; continue; }
  pc = 0x120188a1; continue;
 }
 0x120188a1 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120188ae => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x120188bf; continue; }
  pc = 0x120188b5; continue;
 }
 0x120188b5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120188bf => {
  { cpu.sub32(cpu.r32(0x1203b120u32), 0x0u32); }
  if !cpu.zf { pc = 0x120188f3; continue; }
  pc = 0x120188c8; continue;
 }
 0x120188c8 => {
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x120188cf); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b120u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120188e8; continue; }
  pc = 0x120188db; continue;
 }
 0x120188db => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120188e8 => {
  { let v: u32 = cpu.r32(0x1203b120u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x120188f3; continue;
 }
 0x120188f3 => {
  { cpu.sub32(cpu.r32(0x1203b128u32), 0x0u32); }
  if !cpu.zf { pc = 0x12018927; continue; }
  pc = 0x120188fc; continue;
 }
 0x120188fc => {
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x12018903); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203b128u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201891c; continue; }
  pc = 0x1201890f; continue;
 }
 0x1201890f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201891c => {
  { let v: u32 = cpu.r32(0x1203b128u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12018927; continue;
 }
 0x12018927 => {
  { let v: u32 = cpu.r32(0x1203b120u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201893c); f_12018a6c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x12018a11; continue; }
  pc = 0x12018949; continue;
 }
 0x12018949 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x12018a11; continue; }
  pc = 0x12018952; continue;
 }
 0x12018952 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12018993; continue; }
  pc = 0x12018959; continue;
 }
 0x12018959 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018964); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x1201897b; continue; }
  pc = 0x1201896c; continue;
 }
 0x1201896c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201896c; continue; }
  pc = 0x1201897b; continue;
 }
 0x1201897b => {
  { let v: u32 = cpu.shl32(cpu.edi, 2u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12018985); f_12001cbe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201899a; continue; }
  pc = 0x1201898c; continue;
 }
 0x1201898c => {
  { let v: u32 = cpu.eax; cpu.w32(0x1203b120u32, v); }
  pc = 0x1201899a; continue;
 }
 0x12018993 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4)), v); }
  pc = 0x1201899a; continue;
 }
 0x1201899a => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12018a07; continue; }
  pc = 0x120189a1; continue;
 }
 0x120189a1 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x120189b7); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12018a07; continue; }
  pc = 0x120189c0; continue;
 }
 0x120189c0 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120410e8u32); cpu.push(0x120189fe); cpu.call(t);
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12018a04); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12018a07; continue;
 }
 0x12018a07 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018a11 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x12018a55; continue; }
  pc = 0x12018a18; continue;
 }
 0x12018a18 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x12018a1e; continue; }
  pc = 0x12018a1c; continue;
 }
 0x12018a1c => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x12018a1e; continue;
 }
 0x12018a1e => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12018a2c); f_12001cbe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12018a40; continue; }
  pc = 0x12018a33; continue;
 }
 0x12018a33 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018a40 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1201898c; continue;
 }
 0x12018a55 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018a5f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
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
pub fn f_12018a6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018a6c); }
 let mut pc: u32 = 0x12018a6c;
 loop { match pc {
 0x12018a6c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203b120u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if cpu.zf { pc = 0x12018aa8; continue; }
  pc = 0x12018a7a; continue;
 }
 0x12018a7a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x12018a82; continue;
 }
 0x12018a82 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12018a8c); f_120059ae(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12018aa0; continue; }
  pc = 0x12018a93; continue;
 }
 0x12018a93 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3du8); }
  if cpu.zf { pc = 0x12018ab9; continue; }
  pc = 0x12018a9c; continue;
 }
 0x12018a9c => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x12018ab9; continue; }
  pc = 0x12018aa0; continue;
 }
 0x12018aa0 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x12018a82; continue; }
  pc = 0x12018aa8; continue;
 }
 0x12018aa8 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1203b120u32)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018ab9 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1203b120u32)); cpu.edi = v; }
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
pub fn f_12018ac8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018ac8); }
 let mut pc: u32 = 0x12018ac8;
 loop { match pc {
 0x12018ac8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if !cpu.zf { pc = 0x12018adb; continue; }
  pc = 0x12018ad5; continue;
 }
 0x12018ad5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018adb => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x12018aee; continue; }
  pc = 0x12018ae3; continue;
 }
 0x12018ae3 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x12018ae3; continue; }
  pc = 0x12018aee; continue;
 }
 0x12018aee => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018afb); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x12018b10; continue; }
  pc = 0x12018b06; continue;
 }
 0x12018b06 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x12018b0d); f_12012684(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12018b10; continue;
 }
 0x12018b10 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x12018b30; continue; }
  pc = 0x12018b15; continue;
 }
 0x12018b15 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.push(0x12018b25); f_12003cb0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x12018b15; continue; }
  pc = 0x12018b30; continue;
 }
 0x12018b30 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018b3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018b3c); }
 let mut pc: u32 = 0x12018b3c;
 loop { match pc {
 0x12018b3c => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018b58; continue; }
  pc = 0x12018b45; continue;
 }
 0x12018b45 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x103u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018b54); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018b58 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x103u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018b6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018b6e); }
 let mut pc: u32 = 0x12018b6e;
 loop { match pc {
 0x12018b6e => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018b87; continue; }
  pc = 0x12018b77; continue;
 }
 0x12018b77 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018b83); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018b87 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018b9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018b9b); }
 let mut pc: u32 = 0x12018b9b;
 loop { match pc {
 0x12018b9b => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018bb4; continue; }
  pc = 0x12018ba4; continue;
 }
 0x12018ba4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018bb0); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018bb4 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018bc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018bc8); }
 let mut pc: u32 = 0x12018bc8;
 loop { match pc {
 0x12018bc8 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018be1; continue; }
  pc = 0x12018bd1; continue;
 }
 0x12018bd1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018bdd); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018be1 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x4u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018bf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018bf5); }
 let mut pc: u32 = 0x12018bf5;
 loop { match pc {
 0x12018bf5 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018c11; continue; }
  pc = 0x12018bfe; continue;
 }
 0x12018bfe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018c0d); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018c11 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018c27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018c27); }
 let mut pc: u32 = 0x12018c27;
 loop { match pc {
 0x12018c27 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018c40; continue; }
  pc = 0x12018c30; continue;
 }
 0x12018c30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018c3c); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018c40 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x8u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018c54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018c54); }
 let mut pc: u32 = 0x12018c54;
 loop { match pc {
 0x12018c54 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018c6d; continue; }
  pc = 0x12018c5d; continue;
 }
 0x12018c5d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018c69); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018c6d => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x10u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018c81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018c81); }
 let mut pc: u32 = 0x12018c81;
 loop { match pc {
 0x12018c81 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018c9d; continue; }
  pc = 0x12018c8a; continue;
 }
 0x12018c8a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x107u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018c99); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018c9d => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x107u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018cb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018cb3); }
 let mut pc: u32 = 0x12018cb3;
 loop { match pc {
 0x12018cb3 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018ccf; continue; }
  pc = 0x12018cbc; continue;
 }
 0x12018cbc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x157u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018ccb); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018ccf => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x157u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018ce5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018ce5); }
 let mut pc: u32 = 0x12018ce5;
 loop { match pc {
 0x12018ce5 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018d01; continue; }
  pc = 0x12018cee; continue;
 }
 0x12018cee => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x117u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018cfd); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018d01 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x117u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018d17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018d17); }
 let mut pc: u32 = 0x12018d17;
 loop { match pc {
 0x12018d17 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018d30; continue; }
  pc = 0x12018d20; continue;
 }
 0x12018d20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x20u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12018d2c); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018d30 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x20u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018d44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018d44); }
 let mut pc: u32 = 0x12018d44;
 loop { match pc {
 0x12018d44 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x80u32); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018d55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018d55); }
 let mut pc: u32 = 0x12018d55;
 loop { match pc {
 0x12018d55 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018d5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018d5d); }
 let mut pc: u32 = 0x12018d5d;
 loop { match pc {
 0x12018d5d => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  { let v = cpu.esi; cpu.push(v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018d86; continue; }
  pc = 0x12018d67; continue;
 }
 0x12018d67 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x103u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12018d76); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12018d79; continue;
 }
 0x12018d79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12018d9d; continue; }
  pc = 0x12018d7d; continue;
 }
 0x12018d7d => {
  { cpu.sub32(cpu.esi, 0x5fu32); }
  if cpu.zf { pc = 0x12018d9d; continue; }
  pc = 0x12018d82; continue;
 }
 0x12018d82 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018d86 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x103u32; cpu.eax = v; }
  pc = 0x12018d79; continue;
 }
 0x12018d9d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018da4); }
 let mut pc: u32 = 0x12018da4;
 loop { match pc {
 0x12018da4 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  { let v = cpu.esi; cpu.push(v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12018dcd; continue; }
  pc = 0x12018dae; continue;
 }
 0x12018dae => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x107u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12018dbd); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12018dc0; continue;
 }
 0x12018dc0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12018de4; continue; }
  pc = 0x12018dc4; continue;
 }
 0x12018dc4 => {
  { cpu.sub32(cpu.esi, 0x5fu32); }
  if cpu.zf { pc = 0x12018de4; continue; }
  pc = 0x12018dc9; continue;
 }
 0x12018dc9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018dcd => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x107u32; cpu.eax = v; }
  pc = 0x12018dc0; continue;
 }
 0x12018de4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018deb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018deb); }
 let mut pc: u32 = 0x12018deb;
 loop { match pc {
 0x12018deb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018e07); f_1202f5f9(cpu);
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x3cu32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018e22); f_12018e6e(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018e26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018e26); }
 let mut pc: u32 = 0x12018e26;
 loop { match pc {
 0x12018e26 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018e43); f_1202f5f9(cpu);
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018e5b); f_12018e6e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x12018e69; continue; }
  pc = 0x12018e66; continue;
 }
 0x12018e66 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12018e69; continue;
 }
 0x12018e69 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018e6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018e6e); }
 let mut pc: u32 = 0x12018e6e;
 loop { match pc {
 0x12018e6e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12018e75); f_12003ac7(cpu);
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.eax)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  if !cpu.zf { pc = 0x12018e96; continue; }
  pc = 0x12018e7d; continue;
 }
 0x12018e7d => {
  { let v = 0x15du32; cpu.push(v); }
  cpu.push(0x12018e87); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12018e96; continue; }
  pc = 0x12018e91; continue;
 }
 0x12018e91 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018e96 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x15bu32); }
  if (cpu.sf != cpu.of) { pc = 0x12018eae; continue; }
  pc = 0x12018ea9; continue;
 }
 0x12018ea9 => {
  { let v: u32 = 0x15bu32; cpu.eax = v; }
  pc = 0x12018eae; continue;
 }
 0x12018eae => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12018eb5); f_12019c3a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2du32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018ed8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018ed8); }
 let mut pc: u32 = 0x12018ed8;
 loop { match pc {
 0x12018ed8 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12018ef6); f_1202f5f9(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if (cpu.sf != cpu.of) { pc = 0x12018f73; continue; }
  pc = 0x12018f08; continue;
 }
 0x12018f08 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x12018f73; continue; }
  pc = 0x12018f0f; continue;
 }
 0x12018f0f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12018f21); f_1202f9b8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x12018f24; continue;
 }
 0x12018f24 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x12018f3b; continue; }
  pc = 0x12018f2b; continue;
 }
 0x12018f2b => {
  { let v: u8 = cpu.r8(0x1203bf00u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x12018f3b; continue; }
  pc = 0x12018f35; continue;
 }
 0x12018f35 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x12018f2b; continue; }
  pc = 0x12018f3b; continue;
 }
 0x12018f3b => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x12018f6b; continue; }
  pc = 0x12018f43; continue;
 }
 0x12018f43 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x12018f53; continue; }
  pc = 0x12018f48; continue;
 }
 0x12018f48 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x65u8); }
  if cpu.zf { pc = 0x12018f53; continue; }
  pc = 0x12018f4d; continue;
 }
 0x12018f4d => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x12018f48; continue; }
  pc = 0x12018f53; continue;
 }
 0x12018f53 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x30u8); }
  if !cpu.zf { pc = 0x12018f61; continue; }
  pc = 0x12018f5b; continue;
 }
 0x12018f5b => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x30u8); }
  if cpu.zf { pc = 0x12018f5b; continue; }
  pc = 0x12018f61; continue;
 }
 0x12018f61 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x12018f61; continue; }
  pc = 0x12018f6b; continue;
 }
 0x12018f6b => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12018f73 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12018f86); f_1202f857(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x12018f24; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018f8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018f8b); }
 let mut pc: u32 = 0x12018f8b;
 loop { match pc {
 0x12018f8b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12018fa3); f_120039ed(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x14u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x12018fbe); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018fc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018fc5); }
 let mut pc: u32 = 0x12018fc5;
 loop { match pc {
 0x12018fc5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12018fdd); f_12015c4b(cpu);
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x28u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x12018ff8); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12018fff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12018fff); }
 let mut pc: u32 = 0x12018fff;
 loop { match pc {
 0x12018fff => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12019017); f_12015c7a(cpu);
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x28u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x12019032); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019039(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019039); }
 let mut pc: u32 = 0x12019039;
 loop { match pc {
 0x12019039 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x50u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12019056); f_12015c98(cpu);
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x12019071); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x50u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019078); }
 let mut pc: u32 = 0x12019078;
 loop { match pc {
 0x12019078 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x50u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12019095); f_12015d75(cpu);
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x120190b0); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x50u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120190b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120190b7); }
 let mut pc: u32 = 0x120190b7;
 loop { match pc {
 0x120190b7 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x120190d9; continue; }
  pc = 0x120190c3; continue;
 }
 0x120190c3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120190d9 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  if cpu.zf { pc = 0x12019117; continue; }
  pc = 0x120190e2; continue;
 }
 0x120190e2 => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x120190ee); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120190f1; continue;
 }
 0x120190f1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x1201910a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019121; continue; }
  pc = 0x1201910e; continue;
 }
 0x1201910e => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019117 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x120190f1; continue;
 }
 0x12019121 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x12019144; continue; }
  pc = 0x12019127; continue;
 }
 0x12019127 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x1201913d; continue; }
  pc = 0x12019137; continue;
 }
 0x12019137 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x12019144; continue; }
  pc = 0x1201913d; continue;
 }
 0x1201913d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x12019127; continue; }
  pc = 0x12019144; continue;
 }
 0x12019144 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12019158; continue; }
  pc = 0x12019148; continue;
 }
 0x12019148 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x1201914f); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019158 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019164(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019164); }
 let mut pc: u32 = 0x12019164;
 loop { match pc {
 0x12019164 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201918b; continue; }
  pc = 0x1201916f; continue;
 }
 0x1201916f => {
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x1201917a; continue;
 }
 0x1201917a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x1201917a; continue; }
  pc = 0x1201918b; continue;
 }
 0x1201918b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201918e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201918e); }
 let mut pc: u32 = 0x1201918e;
 loop { match pc {
 0x1201918e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019196(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019196); }
 let mut pc: u32 = 0x12019196;
 loop { match pc {
 0x12019196 => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x120191b5; continue; }
  pc = 0x120191a1; continue;
 }
 0x120191a1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x12019201; continue; }
  pc = 0x120191aa; continue;
 }
 0x120191aa => {
  { cpu.sub32(cpu.eax, 0x7au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12019201; continue; }
  pc = 0x120191af; continue;
 }
 0x120191af => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120191b5 => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  if cpu.zf { pc = 0x120191ef; continue; }
  pc = 0x120191be; continue;
 }
 0x120191be => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x120191ca); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120191cd; continue;
 }
 0x120191cd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120191d7); f_12019204(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x120191f9; continue; }
  pc = 0x120191e0; continue;
 }
 0x120191e0 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x120191e7); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120191ef => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x120191cd; continue;
 }
 0x120191f9 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12019201; continue;
 }
 0x12019201 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019204(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019204); }
 let mut pc: u32 = 0x12019204;
 loop { match pc {
 0x12019204 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  if !cpu.zf { pc = 0x12019229; continue; }
  pc = 0x12019215; continue;
 }
 0x12019215 => {
  { cpu.sub32(cpu.ebx, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x12019222; continue; }
  pc = 0x1201921a; continue;
 }
 0x1201921a => {
  { cpu.sub32(cpu.ebx, 0x7au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12019222; continue; }
  pc = 0x1201921f; continue;
 }
 0x1201921f => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x20u32); cpu.ebx = v; }
  pc = 0x12019222; continue;
 }
 0x12019222 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019229 => {
  { cpu.sub32(cpu.ebx, 0x100u32); }
  if (cpu.sf == cpu.of) { pc = 0x12019261; continue; }
  pc = 0x12019231; continue;
 }
 0x12019231 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12019250; continue; }
  pc = 0x1201923a; continue;
 }
 0x1201923a => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x12019242); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12019245; continue;
 }
 0x12019245 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019261; continue; }
  pc = 0x12019249; continue;
 }
 0x12019249 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019250 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  pc = 0x12019245; continue;
 }
 0x12019261 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x120192b3; continue; }
  pc = 0x12019273; continue;
 }
 0x12019273 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x9u32.wrapping_add(cpu.esp), v); }
  pc = 0x12019285; continue;
 }
 0x12019285 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1a8u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x200u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x120192a5); f_120170bd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120192c3; continue; }
  pc = 0x120192ac; continue;
 }
 0x120192ac => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120192b3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x9u32.wrapping_add(cpu.esp), v); }
  pc = 0x12019285; continue;
 }
 0x120192c3 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x120192d3; continue; }
  pc = 0x120192c8; continue;
 }
 0x120192c8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120192d3 => {
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
pub fn f_120192e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120192e9); }
 let mut pc: u32 = 0x120192e9;
 loop { match pc {
 0x120192e9 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12019308); cpu.call(t);
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12019312); f_12018572(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019316(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019316); }
 let mut pc: u32 = 0x12019316;
 loop { match pc {
 0x12019316 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12019335); cpu.call(t);
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201933f); f_12018572(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019343(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019343); }
 let mut pc: u32 = 0x12019343;
 loop { match pc {
 0x12019343 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x50u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x14u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x12041140u32); cpu.push(0x12019362); cpu.call(t);
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201936c); f_12018624(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019370); }
 let mut pc: u32 = 0x12019370;
 loop { match pc {
 0x12019370 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x1201937e; continue;
 }
 0x1201937e => {
  { cpu.sub32(cpu.r32(0x1203befcu32), cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201939c; continue; }
  pc = 0x12019386; continue;
 }
 0x12019386 => {
  { let v = 0x8u32; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019392); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12019395; continue;
 }
 0x12019395 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120193b1; continue; }
  pc = 0x12019399; continue;
 }
 0x12019399 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1201937e; continue;
 }
 0x1201939c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  pc = 0x12019395; continue;
 }
 0x120193b1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120193cf); f_1200576e(cpu);
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120193db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120193db); }
 let mut pc: u32 = 0x120193db;
 loop { match pc {
 0x120193db => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x8000u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120193f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120193f3); }
 let mut pc: u32 = 0x120193f3;
 loop { match pc {
 0x120193f3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x103u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019402); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019406(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019406); }
 let mut pc: u32 = 0x12019406;
 loop { match pc {
 0x12019406 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019412); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019416(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019416); }
 let mut pc: u32 = 0x12019416;
 loop { match pc {
 0x12019416 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019422); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019426(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019426); }
 let mut pc: u32 = 0x12019426;
 loop { match pc {
 0x12019426 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019432); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019436(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019436); }
 let mut pc: u32 = 0x12019436;
 loop { match pc {
 0x12019436 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019445); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019449(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019449); }
 let mut pc: u32 = 0x12019449;
 loop { match pc {
 0x12019449 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019455); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019459(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019459); }
 let mut pc: u32 = 0x12019459;
 loop { match pc {
 0x12019459 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019465); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019469(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019469); }
 let mut pc: u32 = 0x12019469;
 loop { match pc {
 0x12019469 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x107u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019478); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201947c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201947c); }
 let mut pc: u32 = 0x1201947c;
 loop { match pc {
 0x1201947c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x157u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201948b); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201948f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201948f); }
 let mut pc: u32 = 0x1201948f;
 loop { match pc {
 0x1201948f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x117u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201949e); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120194a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120194a2); }
 let mut pc: u32 = 0x120194a2;
 loop { match pc {
 0x120194a2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x20u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120194ae); f_12004b0f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120194b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120194b2); }
 let mut pc: u32 = 0x120194b2;
 loop { match pc {
 0x120194b2 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.esp)), 0x80u16); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120194c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120194c2); }
 let mut pc: u32 = 0x120194c2;
 loop { match pc {
 0x120194c2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x12019547; continue; }
  pc = 0x120194cb; continue;
 }
 0x120194cb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x12019547; continue; }
  pc = 0x120194d1; continue;
 }
 0x120194d1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12019547; continue; }
  pc = 0x120194d9; continue;
 }
 0x120194d9 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x12019527; continue; }
  pc = 0x120194ea; continue;
 }
 0x120194ea => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12019520; continue; }
  pc = 0x120194f3; continue;
 }
 0x120194f3 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12019520; continue; }
  pc = 0x120194fb; continue;
 }
 0x120194fb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203befcu32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x12019515); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12019520; continue; }
  pc = 0x12019519; continue;
 }
 0x12019519 => {
  { let v: u32 = cpu.r32(0x1203befcu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019520 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019527 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x1201953c); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfffffffeu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019547 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201954b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201954b); }
 let mut pc: u32 = 0x1201954b;
 loop { match pc {
 0x1201954b => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x12019591; continue; }
  pc = 0x12019556; continue;
 }
 0x12019556 => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x12019562); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12019565; continue;
 }
 0x12019565 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12019579); f_120195a6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1201959b; continue; }
  pc = 0x12019582; continue;
 }
 0x12019582 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x12019589); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019591 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12019565; continue;
 }
 0x1201959b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120195a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120195a6); }
 let mut pc: u32 = 0x120195a6;
 loop { match pc {
 0x120195a6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x120195c1; continue; }
  pc = 0x120195b8; continue;
 }
 0x120195b8 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x120195c1; continue; }
  pc = 0x120195bc; continue;
 }
 0x120195bc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120195c1 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12019695; continue; }
  pc = 0x120195c9; continue;
 }
 0x120195c9 => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  if cpu.zf { pc = 0x12019671; continue; }
  pc = 0x120195d6; continue;
 }
 0x120195d6 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x120195ed); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120195f7; continue; }
  pc = 0x120195f1; continue;
 }
 0x120195f1 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120195f7 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x120195fd); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x7au32); }
  if cpu.zf { pc = 0x12019617; continue; }
  pc = 0x12019602; continue;
 }
 0x12019602 => {
  cpu.push(0x12019607); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2au32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019617 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x12019640; continue; }
  pc = 0x12019620; continue;
 }
 0x12019620 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x12019640; continue; }
  pc = 0x12019627; continue;
 }
 0x12019627 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x12019638; continue; }
  pc = 0x12019637; continue;
 }
 0x12019637 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x12019638; continue;
 }
 0x12019638 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019620; continue; }
  pc = 0x12019640; continue;
 }
 0x12019640 => {
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edi); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x12019654); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120196e8; continue; }
  pc = 0x1201965c; continue;
 }
 0x1201965c => {
  cpu.push(0x12019661); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2au32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019671 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x120196e8; continue; }
  pc = 0x12019675; continue;
 }
 0x12019675 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x12019679; continue;
 }
 0x12019679 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x120196e8; continue; }
  pc = 0x12019688; continue;
 }
 0x12019688 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12019679; continue; }
  pc = 0x12019690; continue;
 }
 0x12019690 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019695 => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x120196b5; continue; }
  pc = 0x1201969e; continue;
 }
 0x1201969e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120196b5 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x120196ce); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120196e7; continue; }
  pc = 0x120196d2; continue;
 }
 0x120196d2 => {
  cpu.push(0x120196d7); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2au32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120196e7 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x120196e8; continue;
 }
 0x120196e8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120196ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120196ed); }
 let mut pc: u32 = 0x120196ed;
 loop { match pc {
 0x120196ed => {
  { cpu.sub32(cpu.r32(0x1203f014u32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.zf { pc = 0x12019733; continue; }
  pc = 0x120196f8; continue;
 }
 0x120196f8 => {
  { let v = 0x13u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x12019704); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12019707; continue;
 }
 0x12019707 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201971b); f_12019748(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1201973d; continue; }
  pc = 0x12019724; continue;
 }
 0x12019724 => {
  { let v = 0x13u32; cpu.push(v); }
  cpu.push(0x1201972b); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019733 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  pc = 0x12019707; continue;
 }
 0x1201973d => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1203f018u32)); cpu.w32(0x1203f018u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019748(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019748); }
 let mut pc: u32 = 0x12019748;
 loop { match pc {
 0x12019748 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12019848; continue; }
  pc = 0x12019756; continue;
 }
 0x12019756 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12019848; continue; }
  pc = 0x12019762; continue;
 }
 0x12019762 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1201977a; continue; }
  pc = 0x12019768; continue;
 }
 0x12019768 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12019775; continue; }
  pc = 0x12019770; continue;
 }
 0x12019770 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12019775; continue;
 }
 0x12019775 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201977a => {
  { cpu.sub32(cpu.r32(0x1203c1a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x1201979a; continue; }
  pc = 0x12019783; continue;
 }
 0x12019783 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12019792; continue; }
  pc = 0x1201978b; continue;
 }
 0x1201978b => {
  { let v: u16 = (cpu.eax as u8) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12019792; continue;
 }
 0x12019792 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201979a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(2))), 0x80u8); }
  if cpu.zf { pc = 0x1201980b; continue; }
  pc = 0x120197aa; continue;
 }
 0x120197aa => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120197e2; continue; }
  pc = 0x120197b3; continue;
 }
 0x120197b3 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x120197e8; continue; }
  pc = 0x120197bb; continue;
 }
 0x120197bb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203befcu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x120197de); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120197f0; continue; }
  pc = 0x120197e2; continue;
 }
 0x120197e2 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), cpu.edi); }
  pc = 0x120197e8; continue;
 }
 0x120197e8 => {
  if (!cpu.cf && !cpu.zf) { pc = 0x120197f8; continue; }
  pc = 0x120197ea; continue;
 }
 0x120197ea => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x120197f8; continue; }
  pc = 0x120197f0; continue;
 }
 0x120197f0 => {
  { let v: u32 = cpu.r32(0x1203befcu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120197f8 => {
  cpu.push(0x120197fd); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x2au32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201980b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203c1b8u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041158u32); cpu.push(0x12019829); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019840; continue; }
  pc = 0x1201982d; continue;
 }
 0x1201982d => {
  cpu.push(0x12019832); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x2au32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019840 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019848 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201984d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201984d); }
 let mut pc: u32 = 0x1201984d;
 loop { match pc {
 0x1201984d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12019863); f_12019867(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019867(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019867); }
 let mut pc: u32 = 0x12019867;
 loop { match pc {
 0x12019867 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x1201987e; continue;
 }
 0x1201987e => {
  { cpu.sub32(cpu.r32(0x1203befcu32), cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201989e; continue; }
  pc = 0x12019886; continue;
 }
 0x12019886 => {
  { let v = 0x8u32; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019892); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12019895; continue;
 }
 0x12019895 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120198b3; continue; }
  pc = 0x12019899; continue;
 }
 0x12019899 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1201987e; continue;
 }
 0x1201989e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  pc = 0x12019895; continue;
 }
 0x120198b3 => {
  { cpu.sub8((cpu.ebx as u8), 0x2du8); }
  if !cpu.zf { pc = 0x12019987; continue; }
  pc = 0x120198bc; continue;
 }
 0x120198bc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x2u32; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x120198c6; continue;
 }
 0x120198c6 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x12019abd; continue; }
  pc = 0x120198d1; continue;
 }
 0x120198d1 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x1u32); }
  if cpu.zf { pc = 0x12019abd; continue; }
  pc = 0x120198dc; continue;
 }
 0x120198dc => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x24u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12019abd; continue; }
  pc = 0x120198e7; continue;
 }
 0x120198e7 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x120198ff; continue; }
  pc = 0x120198ee; continue;
 }
 0x120198ee => {
  { cpu.sub8((cpu.ebx as u8), 0x30u8); }
  if cpu.zf { pc = 0x12019998; continue; }
  pc = 0x120198f7; continue;
 }
 0x120198f7 => {
  { let v: u32 = 0xau32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  pc = 0x120198ff; continue;
 }
 0x120198ff => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x10u32); }
  if !cpu.zf { pc = 0x1201991a; continue; }
  pc = 0x12019906; continue;
 }
 0x12019906 => {
  { cpu.sub8((cpu.ebx as u8), 0x30u8); }
  if !cpu.zf { pc = 0x1201991a; continue; }
  pc = 0x1201990b; continue;
 }
 0x1201990b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x78u8); }
  if cpu.zf { pc = 0x12019915; continue; }
  pc = 0x12019911; continue;
 }
 0x12019911 => {
  { cpu.sub8((cpu.eax as u8), 0x58u8); }
  if !cpu.zf { pc = 0x1201991a; continue; }
  pc = 0x12019915; continue;
 }
 0x12019915 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1201991a; continue;
 }
 0x1201991a => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x12019929; continue;
 }
 0x12019929 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x120199bc; continue; }
  pc = 0x12019936; continue;
 }
 0x12019936 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019942); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x12019945; continue;
 }
 0x12019945 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120199d4; continue; }
  pc = 0x1201994d; continue;
 }
 0x1201994d => {
  { let v: u32 = (cpu.ebx as u8) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  pc = 0x12019953; continue;
 }
 0x12019953 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x12019a2d; continue; }
  pc = 0x1201995d; continue;
 }
 0x1201995d => {
  { let v: u32 = cpu.esi | 0x8u32; cpu.esi = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { pc = 0x12019a1e; continue; }
  pc = 0x1201996a; continue;
 }
 0x1201996a => {
  if !cpu.zf { pc = 0x1201997f; continue; }
  pc = 0x1201996c; continue;
 }
 0x1201996c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.div32(v); }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x12019a1e; continue; }
  pc = 0x1201997f; continue;
 }
 0x1201997f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi | 0x4u32; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x12019929; continue;
 }
 0x12019987 => {
  { cpu.sub8((cpu.ebx as u8), 0x2bu8); }
  if !cpu.zf { pc = 0x1201998f; continue; }
  pc = 0x1201998c; continue;
 }
 0x1201998c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1201998f; continue;
 }
 0x1201998f => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x120198c6; continue;
 }
 0x12019998 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x78u8); }
  if cpu.zf { pc = 0x120199af; continue; }
  pc = 0x1201999e; continue;
 }
 0x1201999e => {
  { cpu.sub8((cpu.eax as u8), 0x58u8); }
  if cpu.zf { pc = 0x120199af; continue; }
  pc = 0x120199a2; continue;
 }
 0x120199a2 => {
  { let v: u32 = 0x8u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  pc = 0x120198ff; continue;
 }
 0x120199af => {
  { let v: u32 = 0x10u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  pc = 0x120198ff; continue;
 }
 0x120199bc => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  pc = 0x12019945; continue;
 }
 0x120199d4 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12019a07; continue; }
  pc = 0x120199dd; continue;
 }
 0x120199dd => {
  { let v = 0x103u32; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120199ec); f_12016124(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x120199ef; continue;
 }
 0x120199ef => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12019a2d; continue; }
  pc = 0x120199f3; continue;
 }
 0x120199f3 => {
  { let v: u32 = (cpu.ebx as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120199fc); f_12019196(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffc9u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x12019953; continue;
 }
 0x12019a07 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x103u32; cpu.eax = v; }
  pc = 0x120199ef; continue;
 }
 0x12019a1e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x20u32.wrapping_add(cpu.esp))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  pc = 0x12019929; continue;
 }
 0x12019a2d => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.esi, 0x8u32); }
  if !cpu.zf { pc = 0x12019a5e; continue; }
  pc = 0x12019a36; continue;
 }
 0x12019a36 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12019a42; continue; }
  pc = 0x12019a3e; continue;
 }
 0x12019a3e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x12019a42; continue;
 }
 0x12019a42 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x12019a44; continue;
 }
 0x12019a44 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12019a4a; continue; }
  pc = 0x12019a48; continue;
 }
 0x12019a48 => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12019a4a; continue;
 }
 0x12019a4a => {
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x12019a54; continue; }
  pc = 0x12019a52; continue;
 }
 0x12019a52 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x12019a54; continue;
 }
 0x12019a54 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019a5e => {
  { cpu.and32(cpu.esi, 0x4u32); }
  if !cpu.zf { pc = 0x12019a89; continue; }
  pc = 0x12019a66; continue;
 }
 0x12019a66 => {
  { cpu.and32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x12019aa1; continue; }
  pc = 0x12019a6e; continue;
 }
 0x12019a6e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x2u32); cpu.eax = v; }
  if cpu.zf { pc = 0x12019a7d; continue; }
  pc = 0x12019a75; continue;
 }
 0x12019a75 => {
  { cpu.sub32(cpu.ebp, 0x80000000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12019a89; continue; }
  pc = 0x12019a7d; continue;
 }
 0x12019a7d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019aa1; continue; }
  pc = 0x12019a81; continue;
 }
 0x12019a81 => {
  { cpu.sub32(cpu.ebp, 0x7fffffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12019aa1; continue; }
  pc = 0x12019a89; continue;
 }
 0x12019a89 => {
  cpu.push(0x12019a8e); f_120133b4(cpu);
  { cpu.and32(cpu.esi, 0x1u32); }
  { let v: u32 = 0x22u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x12019aa7; continue; }
  pc = 0x12019a9c; continue;
 }
 0x12019a9c => {
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  pc = 0x12019aa1; continue;
 }
 0x12019aa1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x12019a44; continue;
 }
 0x12019aa7 => {
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x12019ab6; continue; }
  pc = 0x12019aaf; continue;
 }
 0x12019aaf => {
  { let v: u32 = 0x80000000u32; cpu.ebp = v; }
  pc = 0x12019aa1; continue;
 }
 0x12019ab6 => {
  { let v: u32 = 0x7fffffffu32; cpu.ebp = v; }
  pc = 0x12019aa1; continue;
 }
 0x12019abd => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12019acb; continue; }
  pc = 0x12019ac5; continue;
 }
 0x12019ac5 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12019acb; continue;
 }
 0x12019acb => {
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
pub fn f_12019ad5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019ad5); }
 let mut pc: u32 = 0x12019ad5;
 loop { match pc {
 0x12019ad5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12019aeb); f_12019867(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019aef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019aef); }
 let mut pc: u32 = 0x12019aef;
 loop { match pc {
 0x12019aef => {
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019b0b); f_12004b0f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12019b28; continue; }
  pc = 0x12019b12; continue;
 }
 0x12019b12 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019b21); f_12004b0f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019b12; continue; }
  pc = 0x12019b28; continue;
 }
 0x12019b28 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12019b2e); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019b40); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.ebp as u16)); }
  if cpu.zf { pc = 0x12019b71; continue; }
  pc = 0x12019b4c; continue;
 }
 0x12019b4c => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x12019b4e; continue;
 }
 0x12019b4e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019b5e); f_12015d95(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12019b71; continue; }
  pc = 0x12019b65; continue;
 }
 0x12019b65 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0x0u16); }
  if !cpu.zf { pc = 0x12019b4e; continue; }
  pc = 0x12019b71; continue;
 }
 0x12019b71 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12019b8b); f_1200576e(cpu);
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12019b9a); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x12019bad; continue; }
  pc = 0x12019ba5; continue;
 }
 0x12019ba5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12019bad; continue;
 }
 0x12019bad => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x240u32); }
  if cpu.zf { pc = 0x12019bd6; continue; }
  pc = 0x12019bb6; continue;
 }
 0x12019bb6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x12019bca; continue; }
  pc = 0x12019bc4; continue;
 }
 0x12019bc4 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x12019bca; continue;
 }
 0x12019bca => {
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019bd6 => {
  { cpu.and8((cpu.eax as u8), 0x81u8); }
  if cpu.zf { pc = 0x12019c0e; continue; }
  pc = 0x12019bda; continue;
 }
 0x12019bda => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x2du16); }
  if !cpu.zf { pc = 0x12019bf9; continue; }
  pc = 0x12019be0; continue;
 }
 0x12019be0 => {
  { let v = cpu.rf64(0x1203dbd0u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x12019bec; continue;
 }
 0x12019bec => {
  cpu.push(0x12019bf1); f_120133b4(cpu);
  { let v: u32 = 0x22u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12019bca; continue;
 }
 0x12019bf9 => {
  { let v: u32 = cpu.r32(0x1203dbd4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203dbd0u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x12019bec; continue;
 }
 0x12019c0e => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x12019c2a; continue; }
  pc = 0x12019c13; continue;
 }
 0x12019c13 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x12019c22); f_120133b4(cpu);
  { let v: u32 = 0x22u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12019bca; continue;
 }
 0x12019c2a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x12019bca; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019c3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019c3a); }
 let mut pc: u32 = 0x12019c3a;
 loop { match pc {
 0x12019c3a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12019c67; continue; }
  pc = 0x12019c59; continue;
 }
 0x12019c59 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x12019c92; continue; }
  pc = 0x12019c60; continue;
 }
 0x12019c60 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x12019c63; continue;
 }
 0x12019c63 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x12019c59; continue; }
  pc = 0x12019c67; continue;
 }
 0x12019c67 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x12019c85; continue; }
  pc = 0x12019c6e; continue;
 }
 0x12019c6e => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x35u8); }
  if (cpu.sf != cpu.of) { pc = 0x12019c85; continue; }
  pc = 0x12019c74; continue;
 }
 0x12019c74 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x39u8); }
  if !cpu.zf { pc = 0x12019c83; continue; }
  pc = 0x12019c7a; continue;
 }
 0x12019c7a => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x39u8); }
  if cpu.zf { pc = 0x12019c7a; continue; }
  pc = 0x12019c83; continue;
 }
 0x12019c83 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x12019c85; continue;
 }
 0x12019c85 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x31u8); }
  if !cpu.zf { pc = 0x12019c97; continue; }
  pc = 0x12019c8a; continue;
 }
 0x12019c8a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019c92 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x12019c63; continue;
 }
 0x12019c97 => {
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
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019cbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019cbb); }
 let mut pc: u32 = 0x12019cbb;
 loop { match pc {
 0x12019cbb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019cc4; continue; }
  pc = 0x12019cc3; continue;
 }
 0x12019cc3 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x12019cc4; continue;
 }
 0x12019cc4 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204111cu32); cpu.push(0x12019ccb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019ccc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019ccc); }
 let mut pc: u32 = 0x12019ccc;
 loop { match pc {
 0x12019ccc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e4u32); cpu.push(0x12019cdc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019cdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019cdd); }
 let mut pc: u32 = 0x12019cdd;
 loop { match pc {
 0x12019cdd => {
  { let v: u32 = cpu.esp.wrapping_sub(0x140u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x148u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041198u32); cpu.push(0x12019cfa); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  if !cpu.zf { pc = 0x12019d6f; continue; }
  pc = 0x12019d01; continue;
 }
 0x12019d01 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12019d07); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12019d1e; continue; }
  pc = 0x12019d0f; continue;
 }
 0x12019d0f => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x12019e10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12019e00u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x12019d1e => {
  cpu.push(0x12019d23); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019d39 => {
  cpu.push(0x12019d3e); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019d54 => {
  cpu.push(0x12019d59); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019d6f => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x12019dee; continue; }
  pc = 0x12019d79; continue;
 }
 0x12019d79 => {
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12019d86; continue;
 }
 0x12019d86 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019d90); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019da0); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019db0); f_12019f7c(cpu);
  { let v: u32 = 0x40u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
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
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019dee => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x12019d86; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019e00); }
 let mut pc: u32 = 0x12019e00;
 loop { match pc {
 0x12019e00 => {
  { cpu.sub32(cpu.r32(0x9d541201u32.wrapping_add(cpu.ebp)), cpu.ebx); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.r32(0x9d1e1201u32.wrapping_add(cpu.ebp)), cpu.ebx); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
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
pub fn f_12019e21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019e21); }
 let mut pc: u32 = 0x12019e21;
 loop { match pc {
 0x12019e21 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x140u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
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
pub fn f_12019f48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019f48); }
 let mut pc: u32 = 0x12019f48;
 loop { match pc {
 0x12019f48 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x50042444u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x12019f64); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019f79; continue; }
  pc = 0x12019f68; continue;
 }
 0x12019f68 => {
  cpu.push(0x12019f6d); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019f79 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019f59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019f59); }
 let mut pc: u32 = 0x12019f59;
 loop { match pc {
 0x12019f59 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x12019f64); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12019f79; continue; }
  pc = 0x12019f68; continue;
 }
 0x12019f68 => {
  cpu.push(0x12019f6d); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019f79 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12019f7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12019f7c); }
 let mut pc: u32 = 0x12019f7c;
 loop { match pc {
 0x12019f7c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x12019f97; continue; }
  pc = 0x12019f88; continue;
 }
 0x12019f88 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x12019f97; continue; }
  pc = 0x12019f8e; continue;
 }
 0x12019f8e => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019f97 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x12019fa3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12019ffc; continue; }
  pc = 0x12019fa7; continue;
 }
 0x12019fa7 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x12019fb7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12019ffc; continue; }
  pc = 0x12019fbb; continue;
 }
 0x12019fbb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12019ff8); f_1202ef0d(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x34u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12019ffc => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a005(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a005); }
 let mut pc: u32 = 0x1201a005;
 loop { match pc {
 0x1201a005 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x140u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x148u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041198u32); cpu.push(0x1201a022); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  if !cpu.zf { pc = 0x1201a097; continue; }
  pc = 0x1201a029; continue;
 }
 0x1201a029 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a02f); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201a046; continue; }
  pc = 0x1201a037; continue;
 }
 0x1201a037 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1201a158u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1201a148u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1201a046 => {
  cpu.push(0x1201a04b); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a061 => {
  cpu.push(0x1201a066); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a07c => {
  cpu.push(0x1201a081); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a097 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x1201a132; continue; }
  pc = 0x1201a0a5; continue;
 }
 0x1201a0a5 => {
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201a0b2; continue;
 }
 0x1201a0b2 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a0bc); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a0cc); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a0dc); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a0f2); f_12018734(cpu);
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x30u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.edi = v; }
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
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x140u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a132 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201a0b2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a148(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a148); }
 let mut pc: u32 = 0x1201a148;
 loop { match pc {
 0x1201a148 => {
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0xa07c1201u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0xa0461201u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
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
pub fn f_1201a169(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a169); }
 let mut pc: u32 = 0x1201a169;
 loop { match pc {
 0x1201a169 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x140u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
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
pub fn f_1201a2a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a2a0); }
 let mut pc: u32 = 0x1201a2a0;
 loop { match pc {
 0x1201a2a0 => {
  { let v: u32 = 0xd71201a1u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xa1bd1201u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1201a1u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  f_1201a2c1(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a2b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a2b0); }
 let mut pc: u32 = 0x1201a2b0;
 loop { match pc {
 0x1201a2b0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x83042444u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1201a2c6, "In");
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x85u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8(cpu.r8(0x28u32.wrapping_add(cpu.ebp)), 51u32); cpu.w8(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.ror8(cpu.r8(0x510c244cu32.wrapping_add(cpu.ebx)), (cpu.ecx & 0xff)); cpu.w8(0x510c244cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x120410d8u32); cpu.push(0x1201a2e6); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201a31d; continue; }
  pc = 0x1201a2ea; continue;
 }
 0x1201a2ea => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a2f0); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a31d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a2c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a2c1); }
 let mut pc: u32 = 0x1201a2c1;
 loop { match pc {
 0x1201a2c1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201a2f4; continue; }
  pc = 0x1201a2cc; continue;
 }
 0x1201a2cc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1201a2ce; continue;
 }
 0x1201a2ce => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x120410d8u32); cpu.push(0x1201a2e6); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201a31d; continue; }
  pc = 0x1201a2ea; continue;
 }
 0x1201a2ea => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a2f0); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a2f4 => {
  { cpu.sub32(cpu.eax, 0x1au32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201a302; continue; }
  pc = 0x1201a2f9; continue;
 }
 0x1201a2f9 => {
  { let v: u32 = 0x57u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a302 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x40u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x5cu8; cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201a2ce; continue;
 }
 0x1201a31d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a323(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a323); }
 let mut pc: u32 = 0x1201a323;
 loop { match pc {
 0x1201a323 => {
  pc = cpu.r32(0x120410d4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a329(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a329); }
 let mut pc: u32 = 0x1201a329;
 loop { match pc {
 0x1201a329 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120410d0u32); cpu.push(0x1201a334); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a335(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a335); }
 let mut pc: u32 = 0x1201a335;
 loop { match pc {
 0x1201a335 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x250u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041190u32); cpu.push(0x1201a34d); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x1201a3bc; continue; }
  pc = 0x1201a354; continue;
 }
 0x1201a354 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a35a); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201a371; continue; }
  pc = 0x1201a362; continue;
 }
 0x1201a362 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1201a450u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1201a440u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1201a371 => {
  cpu.push(0x1201a376); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a38a => {
  cpu.push(0x1201a38f); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a3a3 => {
  cpu.push(0x1201a3a8); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a3bc => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x1201a426; continue; }
  pc = 0x1201a3c6; continue;
 }
 0x1201a3c6 => {
  { let v: u32 = cpu.r32(0x260u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a3d3; continue;
 }
 0x1201a3d3 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a3dd); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a3ed); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a3fd); f_12019f7c(cpu);
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201a418); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a426 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x260u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a3d3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a440); }
 let mut pc: u32 = 0x1201a440;
 loop { match pc {
 0x1201a440 => {
  { let v: u8 = cpu.r8(0xa3a31201u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0xa3711201u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
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
pub fn f_1201a461(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a461); }
 let mut pc: u32 = 0x1201a461;
 loop { match pc {
 0x1201a461 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x250u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
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
pub fn f_1201a560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a560); }
 let mut pc: u32 = 0x1201a560;
 loop { match pc {
 0x1201a560 => {
  { let v: u8 = 0xa4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xa4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
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
pub fn f_1201a581(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a581); }
 let mut pc: u32 = 0x1201a581;
 loop { match pc {
 0x1201a581 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x250u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
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
pub fn f_1201a6a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a6a0); }
 let mut pc: u32 = 0x1201a6a0;
 loop { match pc {
 0x1201a6a0 => {
  cpu.unimplemented(0x1201a6a0, "Salc");
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1201a6a4, "Out");
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.unimplemented(0x1201a6a8, "Salc");
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1201a5u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  f_1201a6c1(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a6b0); }
 let mut pc: u32 = 0x1201a6b0;
 loop { match pc {
 0x1201a6b0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x8104244cu32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1201a6c6, "In");
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x56002444u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x56002444u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041194u32); cpu.push(0x1201a6d8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201a741; continue; }
  pc = 0x1201a6dc; continue;
 }
 0x1201a6dc => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a6e2); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201a6f9; continue; }
  pc = 0x1201a6ea; continue;
 }
 0x1201a6ea => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1201a7f0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1201a7e0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1201a6f9 => {
  cpu.push(0x1201a6fe); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a711 => {
  cpu.push(0x1201a716); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a729 => {
  cpu.push(0x1201a72e); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a741 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x1201a7c2; continue; }
  pc = 0x1201a74b; continue;
 }
 0x1201a74b => {
  { let v: u32 = cpu.r32(0x25cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a758; continue;
 }
 0x1201a758 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a762); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a772); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a782); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a798); f_12018734(cpu);
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x24u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x18u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201a7b5); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a7c2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x25cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a758; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a6c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a6c1); }
 let mut pc: u32 = 0x1201a6c1;
 loop { match pc {
 0x1201a6c1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x250u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041194u32); cpu.push(0x1201a6d8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201a741; continue; }
  pc = 0x1201a6dc; continue;
 }
 0x1201a6dc => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a6e2); cpu.call(t);
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201a6f9; continue; }
  pc = 0x1201a6ea; continue;
 }
 0x1201a6ea => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1201a7f0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x1201a7e0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x1201a6f9 => {
  cpu.push(0x1201a6fe); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a711 => {
  cpu.push(0x1201a716); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a729 => {
  cpu.push(0x1201a72e); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a741 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x80u32); }
  if !cpu.zf { pc = 0x1201a7c2; continue; }
  pc = 0x1201a74b; continue;
 }
 0x1201a74b => {
  { let v: u32 = cpu.r32(0x25cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a758; continue;
 }
 0x1201a758 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a762); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a772); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a782); f_12019f7c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a798); f_12018734(cpu);
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x24u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x18u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201a7b5); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x250u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a7c2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x25cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1201a758; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a7e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a7e0); }
 let mut pc: u32 = 0x1201a7e0;
 loop { match pc {
 0x1201a7e0 => {
  { let v: u32 = cpu.adc32(cpu.r32(0xa7291201u32.wrapping_add(cpu.edi)), cpu.esp); cpu.w32(0xa7291201u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0xa6f91201u32.wrapping_add(cpu.edi)), cpu.esp); cpu.w32(0xa6f91201u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
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
pub fn f_1201a801(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a801); }
 let mut pc: u32 = 0x1201a801;
 loop { match pc {
 0x1201a801 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
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
pub fn f_1201a850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a850); }
 let mut pc: u32 = 0x1201a850;
 loop { match pc {
 0x1201a850 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10cu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120410c4u32); cpu.push(0x1201a861); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201a8d9; continue; }
  pc = 0x1201a865; continue;
 }
 0x1201a865 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x105u32; cpu.push(v); }
  let t = cpu.r32(0x120410c8u32); cpu.push(0x1201a875); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201a8d9; continue; }
  pc = 0x1201a879; continue;
 }
 0x1201a879 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x5cu8); }
  if cpu.zf { pc = 0x1201a887; continue; }
  pc = 0x1201a880; continue;
 }
 0x1201a880 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2fu8); }
  if !cpu.zf { pc = 0x1201a89a; continue; }
  pc = 0x1201a887; continue;
 }
 0x1201a887 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1201a89a; continue; }
  pc = 0x1201a891; continue;
 }
 0x1201a891 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a89a => {
  { let v: u8 = 0x3du8; cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a8ab); f_1202243a(cpu);
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x3au8; cpu.w8(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x120410e8u32); cpu.push(0x1201a8cc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201a8d9; continue; }
  pc = 0x1201a8d0; continue;
 }
 0x1201a8d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a8d9 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a8df); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a8e5); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a8f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a8f4); }
 let mut pc: u32 = 0x1201a8f4;
 loop { match pc {
 0x1201a8f4 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x104u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  let t = cpu.r32(0x120410c8u32); cpu.push(0x1201a90d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201a92a; continue; }
  pc = 0x1201a911; continue;
 }
 0x1201a911 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.esp)), 0x3au8); }
  if !cpu.zf { pc = 0x1201a92a; continue; }
  pc = 0x1201a918; continue;
 }
 0x1201a918 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a924); f_12019196(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x1201a92a; continue;
 }
 0x1201a92a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x104u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a934(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a934); }
 let mut pc: u32 = 0x1201a934;
 loop { match pc {
 0x1201a934 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x1201a9a4; continue; }
  pc = 0x1201a941; continue;
 }
 0x1201a941 => {
  { cpu.sub32(cpu.ebx, 0x1fu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201a9a4; continue; }
  pc = 0x1201a946; continue;
 }
 0x1201a946 => {
  { let v = 0xcu32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x40u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x1201a950); f_1200420b(cpu);
  { let v: u8 = 0x3au8; cpu.w8(0x9u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xau32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120410c4u32); cpu.push(0x1201a96c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201a981; continue; }
  pc = 0x1201a970; continue;
 }
 0x1201a970 => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201a977); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a981 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a987); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a98d); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201a997); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a9a4 => {
  cpu.push(0x1201a9a9); f_120133b4(cpu);
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201a9b4); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0xfu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201a9c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201a9c4); }
 let mut pc: u32 = 0x1201a9c4;
 loop { match pc {
 0x1201a9c4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x120410ccu32); cpu.push(0x1201a9d0); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201a9eb; continue; }
  pc = 0x1201a9d5; continue;
 }
 0x1201a9d5 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201a9db); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201a9e1); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201a9eb => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x80u8); }
  if cpu.zf { pc = 0x1201aa17; continue; }
  pc = 0x1201a9f2; continue;
 }
 0x1201a9f2 => {
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  pc = 0x1201a9f5; continue;
 }
 0x1201a9f5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x120410c0u32); cpu.push(0x1201a9fd); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201aa1c; continue; }
  pc = 0x1201aa01; continue;
 }
 0x1201aa01 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201aa07); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201aa0d); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201aa17 => {
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  pc = 0x1201a9f5; continue;
 }
 0x1201aa1c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201aa20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201aa20); }
 let mut pc: u32 = 0x1201aa20;
 loop { match pc {
 0x1201aa20 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201aae4; continue; }
  pc = 0x1201aa33; continue;
 }
 0x1201aa33 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1201aae4; continue; }
  pc = 0x1201aa3d; continue;
 }
 0x1201aa3d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201aaa8; continue; }
  pc = 0x1201aa45; continue;
 }
 0x1201aa45 => {
  { let v = 0x104u32; cpu.push(v); }
  cpu.push(0x1201aa4f); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1201aa6d; continue; }
  pc = 0x1201aa58; continue;
 }
 0x1201aa58 => {
  cpu.push(0x1201aa5d); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201aa6d => {
  { let v: u32 = 0x104u32; cpu.edi = v; }
  pc = 0x1201aa72; continue;
 }
 0x1201aa72 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x120410bcu32); cpu.ebp = v; }
  let t = cpu.ebp; cpu.push(0x1201aa82); cpu.call(t);
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.cf { pc = 0x1201aab0; continue; }
  pc = 0x1201aa86; continue;
 }
 0x1201aa86 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201aa93; continue; }
  pc = 0x1201aa8a; continue;
 }
 0x1201aa8a => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201aa90); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201aa93; continue;
 }
 0x1201aa93 => {
  cpu.push(0x1201aa98); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x22u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201aaa8 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x1201aa72; continue;
 }
 0x1201aab0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201aada; continue; }
  pc = 0x1201aab4; continue;
 }
 0x1201aab4 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201aac1; continue; }
  pc = 0x1201aab8; continue;
 }
 0x1201aab8 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201aabe); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201aac1; continue;
 }
 0x1201aac1 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201aac7); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201aacd); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201aada => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201aae4 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201aaf3); f_1201aafe(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
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
pub fn f_1201aafe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201aafe); }
 let mut pc: u32 = 0x1201aafe;
 loop { match pc {
 0x1201aafe => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201ab06); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1201ab1a); f_1201ab5f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201ab26); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ab2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ab2d); }
 let mut pc: u32 = 0x1201ab2d;
 loop { match pc {
 0x1201ab2d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201ab35); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201ab4c); f_1201ab5f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201ab58); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ab5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ab5f); }
 let mut pc: u32 = 0x1201ab5f;
 loop { match pc {
 0x1201ab5f => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201ac28; continue; }
  pc = 0x1201ab77; continue;
 }
 0x1201ab77 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201ab7d); f_1201ac95(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201aba6; continue; }
  pc = 0x1201ab84; continue;
 }
 0x1201ab84 => {
  cpu.push(0x1201ab89); f_120133bd(cpu);
  { let v: u32 = 0xfu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ab94); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201aba6 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x40u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x11u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x2eu8; cpu.w8(0x12u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120410bcu32); cpu.push(0x1201abd6); cpu.call(t);
  pc = 0x1201abd6; continue;
 }
 0x1201abd6 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201ac89; continue; }
  pc = 0x1201abde; continue;
 }
 0x1201abde => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x104u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201ac89; continue; }
  pc = 0x1201abea; continue;
 }
 0x1201abea => {
  { let v: u32 = cpu.r32(0x120u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x1201ac3a; continue; }
  pc = 0x1201abf5; continue;
 }
 0x1201abf5 => {
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201ac02; continue; }
  pc = 0x1201ac00; continue;
 }
 0x1201ac00 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x1201ac02; continue;
 }
 0x1201ac02 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ac08); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x1201ac5a; continue; }
  pc = 0x1201ac11; continue;
 }
 0x1201ac11 => {
  cpu.push(0x1201ac16); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ac28 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  let t = cpu.r32(0x120410c8u32); cpu.push(0x1201ac38); cpu.call(t);
  pc = 0x1201abd6; continue;
 }
 0x1201ac3a => {
  { cpu.sub32(cpu.eax, cpu.r32(0x124u32.wrapping_add(cpu.esp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201ac5a; continue; }
  pc = 0x1201ac43; continue;
 }
 0x1201ac43 => {
  cpu.push(0x1201ac48); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x22u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ac5a => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edi = v; }
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
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ac89 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ac95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ac95); }
 let mut pc: u32 = 0x1201ac95;
 loop { match pc {
 0x1201ac95 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201aca9; continue; }
  pc = 0x1201aca0; continue;
 }
 0x1201aca0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201aca9 => {
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x5cu8; cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x40u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120410b8u32); cpu.push(0x1201acc9); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201acdb; continue; }
  pc = 0x1201accd; continue;
 }
 0x1201accd => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1201acdb; continue; }
  pc = 0x1201acd2; continue;
 }
 0x1201acd2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201acdb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ace1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ace1); }
 let mut pc: u32 = 0x1201ace1;
 loop { match pc {
 0x1201ace1 => {
  pc = cpu.r32(0x120410b4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ace7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ace7); }
 let mut pc: u32 = 0x1201ace7;
 loop { match pc {
 0x1201ace7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120410b0u32); cpu.push(0x1201acf4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201ad03; continue; }
  pc = 0x1201acfd; continue;
 }
 0x1201acfd => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201ad03); cpu.call(t);
  pc = 0x1201ad03; continue;
 }
 0x1201ad03 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201ad16; continue; }
  pc = 0x1201ad07; continue;
 }
 0x1201ad07 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ad0d); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ad16 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ad19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ad19); }
 let mut pc: u32 = 0x1201ad19;
 loop { match pc {
 0x1201ad19 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120410acu32); cpu.push(0x1201ad24); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201ad33; continue; }
  pc = 0x1201ad2d; continue;
 }
 0x1201ad2d => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201ad33); cpu.call(t);
  pc = 0x1201ad33; continue;
 }
 0x1201ad33 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201ad46; continue; }
  pc = 0x1201ad37; continue;
 }
 0x1201ad37 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ad3d); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ad46 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ad49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ad49); }
 let mut pc: u32 = 0x1201ad49;
 loop { match pc {
 0x1201ad49 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), 0x3au8); }
  if !cpu.zf { pc = 0x1201ad65; continue; }
  pc = 0x1201ad62; continue;
 }
 0x1201ad62 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.edx); cpu.eax = v; }
  pc = 0x1201ad65; continue;
 }
 0x1201ad65 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201ad71; continue; }
  pc = 0x1201ad6c; continue;
 }
 0x1201ad6c => {
  { cpu.sub8((cpu.ebx as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201ad77; continue; }
  pc = 0x1201ad71; continue;
 }
 0x1201ad71 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1201ad84; continue; }
  pc = 0x1201ad77; continue;
 }
 0x1201ad77 => {
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if !cpu.zf { pc = 0x1201ad84; continue; }
  pc = 0x1201ad7c; continue;
 }
 0x1201ad7c => {
  { let v: u16 = 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x1201ad88; continue; }
  pc = 0x1201ad84; continue;
 }
 0x1201ad84 => {
  { let v: u16 = 0x4040u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x1201ad88; continue;
 }
 0x1201ad88 => {
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { let v = 0x2eu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x80u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { let v: u16 = (cpu.esi as u16) | (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x1201ada5); f_12005b48(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201adfa; continue; }
  pc = 0x1201adae; continue;
 }
 0x1201adae => {
  { let v = 0x12036528u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201adb9); f_12013a4c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201adf6; continue; }
  pc = 0x1201adc0; continue;
 }
 0x1201adc0 => {
  { let v = 0x12036520u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201adcb); f_12013a4c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201adf6; continue; }
  pc = 0x1201add2; continue;
 }
 0x1201add2 => {
  { let v = 0x12036518u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201addd); f_12013a4c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201adf6; continue; }
  pc = 0x1201ade4; continue;
 }
 0x1201ade4 => {
  { let v = 0x12036510u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201adef); f_12013a4c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201adfa; continue; }
  pc = 0x1201adf6; continue;
 }
 0x1201adf6 => {
  { let v: u16 = (cpu.esi as u16) | 0x40u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x1201adfa; continue;
 }
 0x1201adfa => {
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x1c0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.esi as u16) | (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x1c0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.esi as u16), (cpu.eax as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ae1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ae1d); }
 let mut pc: u32 = 0x1201ae1d;
 loop { match pc {
 0x1201ae1d => {
  { let v: u32 = cpu.esp.wrapping_sub(0x25cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x268u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x12036534u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201ae39); f_12004309(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201ae66; continue; }
  pc = 0x1201ae40; continue;
 }
 0x1201ae40 => {
  cpu.push(0x1201ae45); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ae50); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ae66 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x3au8); }
  if !cpu.zf { pc = 0x1201afaf; continue; }
  pc = 0x1201ae70; continue;
 }
 0x1201ae70 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1201aea2; continue; }
  pc = 0x1201ae76; continue;
 }
 0x1201ae76 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1201aea2; continue; }
  pc = 0x1201ae7c; continue;
 }
 0x1201ae7c => {
  cpu.push(0x1201ae81); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ae8c); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201aea2 => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201aeab); f_120223ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x1201aeb1; continue;
 }
 0x1201aeb1 => {
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12041198u32); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201aebf); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x1201afe1; continue; }
  pc = 0x1201aeca; continue;
 }
 0x1201aeca => {
  { let v = 0x12036530u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201aed5); f_12004309(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201afbb; continue; }
  pc = 0x1201aee0; continue;
 }
 0x1201aee0 => {
  { let v: u32 = 0x168u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201aef3); f_1201aa20(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201afbb; continue; }
  pc = 0x1201af00; continue;
 }
 0x1201af00 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x1201af24; continue; }
  pc = 0x1201af13; continue;
 }
 0x1201af13 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201af19); f_1201b1d8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201afbb; continue; }
  pc = 0x1201af24; continue;
 }
 0x1201af24 => {
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x120410b8u32); cpu.push(0x1201af2b); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201afbb; continue; }
  pc = 0x1201af34; continue;
 }
 0x1201af34 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x274u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x7bcu32; cpu.push(v); }
  cpu.push(0x1201af64); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201af70; continue;
 }
 0x1201af70 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201af7b); f_1201ad49(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201afaf => {
  cpu.push(0x1201afb4); f_1201a8f4(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x1201aeb1; continue;
 }
 0x1201afbb => {
  cpu.push(0x1201afc0); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201afcb); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201afe1 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201aff1); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b1b2; continue; }
  pc = 0x1201aff9; continue;
 }
 0x1201aff9 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201b009); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b1b2; continue; }
  pc = 0x1201b011; continue;
 }
 0x1201b011 => {
  { let v: u32 = cpu.r32(0x274u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b055); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201b069; continue; }
  pc = 0x1201b062; continue;
 }
 0x1201b062 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201b0d9; continue; }
  pc = 0x1201b069; continue;
 }
 0x1201b069 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201b079); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b18c; continue; }
  pc = 0x1201b081; continue;
 }
 0x1201b081 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201b091); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b18c; continue; }
  pc = 0x1201b099; continue;
 }
 0x1201b099 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b0d6); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201b0d9; continue;
 }
 0x1201b0d9 => {
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x1201b0fc; continue; }
  pc = 0x1201b0e3; continue;
 }
 0x1201b0e3 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201b0fc; continue; }
  pc = 0x1201b0ea; continue;
 }
 0x1201b0ea => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1201b0ed; continue;
 }
 0x1201b0ed => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201b0f7); cpu.call(t);
  pc = 0x1201af70; continue;
 }
 0x1201b0fc => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201b10c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b166; continue; }
  pc = 0x1201b110; continue;
 }
 0x1201b110 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201b120); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b166; continue; }
  pc = 0x1201b124; continue;
 }
 0x1201b124 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b161); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201b0ed; continue;
 }
 0x1201b166 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b16c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b172); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201b17c); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b18c => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b192); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b198); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201b1a2); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b1b2 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b1b8); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b1be); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201b1c8); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b1d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b1d8); }
 let mut pc: u32 = 0x1201b1d8;
 loop { match pc {
 0x1201b1d8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if cpu.cf { pc = 0x1201b245; continue; }
  pc = 0x1201b1f0; continue;
 }
 0x1201b1f0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201b1fa; continue; }
  pc = 0x1201b1f6; continue;
 }
 0x1201b1f6 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201b245; continue; }
  pc = 0x1201b1fa; continue;
 }
 0x1201b1fa => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201b205; continue; }
  pc = 0x1201b201; continue;
 }
 0x1201b201 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201b245; continue; }
  pc = 0x1201b205; continue;
 }
 0x1201b205 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  pc = 0x1201b208; continue;
 }
 0x1201b208 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1201b218; continue; }
  pc = 0x1201b210; continue;
 }
 0x1201b210 => {
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201b218; continue; }
  pc = 0x1201b214; continue;
 }
 0x1201b214 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201b208; continue; }
  pc = 0x1201b218; continue;
 }
 0x1201b218 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x1201b245; continue; }
  pc = 0x1201b21d; continue;
 }
 0x1201b21d => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x1201b245; continue; }
  pc = 0x1201b223; continue;
 }
 0x1201b223 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1201b233; continue; }
  pc = 0x1201b22b; continue;
 }
 0x1201b22b => {
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201b233; continue; }
  pc = 0x1201b22f; continue;
 }
 0x1201b22f => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201b223; continue; }
  pc = 0x1201b233; continue;
 }
 0x1201b233 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x1201b23e; continue; }
  pc = 0x1201b238; continue;
 }
 0x1201b238 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x1201b245; continue; }
  pc = 0x1201b23e; continue;
 }
 0x1201b23e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b245 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b249(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b249); }
 let mut pc: u32 = 0x1201b249;
 loop { match pc {
 0x1201b249 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x25cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x268u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x12036534u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201b265); f_12004309(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b292; continue; }
  pc = 0x1201b26c; continue;
 }
 0x1201b26c => {
  cpu.push(0x1201b271); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201b27c); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b292 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x3au8); }
  if !cpu.zf { pc = 0x1201b3f2; continue; }
  pc = 0x1201b29c; continue;
 }
 0x1201b29c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1201b2ce; continue; }
  pc = 0x1201b2a2; continue;
 }
 0x1201b2a2 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1201b2ce; continue; }
  pc = 0x1201b2a8; continue;
 }
 0x1201b2a8 => {
  cpu.push(0x1201b2ad); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201b2b8); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b2ce => {
  { let v: u32 = (cpu.eax as u8) as i8 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b2d7); f_120223ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x1201b2dd; continue;
 }
 0x1201b2dd => {
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12041198u32); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201b2eb); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x1201b424; continue; }
  pc = 0x1201b2f6; continue;
 }
 0x1201b2f6 => {
  { let v = 0x12036530u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201b301); f_12004309(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b3fe; continue; }
  pc = 0x1201b30c; continue;
 }
 0x1201b30c => {
  { let v: u32 = 0x168u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b31f); f_1201aa20(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201b3fe; continue; }
  pc = 0x1201b32c; continue;
 }
 0x1201b32c => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x1201b350; continue; }
  pc = 0x1201b33f; continue;
 }
 0x1201b33f => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201b345); f_1201b61b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b3fe; continue; }
  pc = 0x1201b350; continue;
 }
 0x1201b350 => {
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x120410b8u32); cpu.push(0x1201b357); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201b3fe; continue; }
  pc = 0x1201b360; continue;
 }
 0x1201b360 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x274u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x7bcu32; cpu.push(v); }
  cpu.push(0x1201b390); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201b39c; continue;
 }
 0x1201b39c => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b3a7); f_1201ad49(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b3c4); f_12018734(cpu);
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x48u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b3f2 => {
  cpu.push(0x1201b3f7); f_1201a8f4(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x1201b2dd; continue;
 }
 0x1201b3fe => {
  cpu.push(0x1201b403); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201b40e); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b424 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201b434); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b5f5; continue; }
  pc = 0x1201b43c; continue;
 }
 0x1201b43c => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201b44c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b5f5; continue; }
  pc = 0x1201b454; continue;
 }
 0x1201b454 => {
  { let v: u32 = cpu.r32(0x274u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b498); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201b4ac; continue; }
  pc = 0x1201b4a5; continue;
 }
 0x1201b4a5 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201b51c; continue; }
  pc = 0x1201b4ac; continue;
 }
 0x1201b4ac => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201b4bc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b5cf; continue; }
  pc = 0x1201b4c4; continue;
 }
 0x1201b4c4 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201b4d4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b5cf; continue; }
  pc = 0x1201b4dc; continue;
 }
 0x1201b4dc => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b519); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201b51c; continue;
 }
 0x1201b51c => {
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x1201b53f; continue; }
  pc = 0x1201b526; continue;
 }
 0x1201b526 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201b53f; continue; }
  pc = 0x1201b52d; continue;
 }
 0x1201b52d => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1201b530; continue;
 }
 0x1201b530 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201b53a); cpu.call(t);
  pc = 0x1201b39c; continue;
 }
 0x1201b53f => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201b54f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b5a9; continue; }
  pc = 0x1201b553; continue;
 }
 0x1201b553 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201b563); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b5a9; continue; }
  pc = 0x1201b567; continue;
 }
 0x1201b567 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b5a4); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201b530; continue;
 }
 0x1201b5a9 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b5af); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b5b5); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201b5bf); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b5cf => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b5d5); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b5db); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201b5e5); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b5f5 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b5fb); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b601); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201b60b); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x25cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b61b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b61b); }
 let mut pc: u32 = 0x1201b61b;
 loop { match pc {
 0x1201b61b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if cpu.cf { pc = 0x1201b688; continue; }
  pc = 0x1201b633; continue;
 }
 0x1201b633 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201b63d; continue; }
  pc = 0x1201b639; continue;
 }
 0x1201b639 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201b688; continue; }
  pc = 0x1201b63d; continue;
 }
 0x1201b63d => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201b648; continue; }
  pc = 0x1201b644; continue;
 }
 0x1201b644 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201b688; continue; }
  pc = 0x1201b648; continue;
 }
 0x1201b648 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  pc = 0x1201b64b; continue;
 }
 0x1201b64b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1201b65b; continue; }
  pc = 0x1201b653; continue;
 }
 0x1201b653 => {
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201b65b; continue; }
  pc = 0x1201b657; continue;
 }
 0x1201b657 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201b64b; continue; }
  pc = 0x1201b65b; continue;
 }
 0x1201b65b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x1201b688; continue; }
  pc = 0x1201b660; continue;
 }
 0x1201b660 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x1201b688; continue; }
  pc = 0x1201b666; continue;
 }
 0x1201b666 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1201b676; continue; }
  pc = 0x1201b66e; continue;
 }
 0x1201b66e => {
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if cpu.zf { pc = 0x1201b676; continue; }
  pc = 0x1201b672; continue;
 }
 0x1201b672 => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201b666; continue; }
  pc = 0x1201b676; continue;
 }
 0x1201b676 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x1201b681; continue; }
  pc = 0x1201b67b; continue;
 }
 0x1201b67b => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x1201b688; continue; }
  pc = 0x1201b681; continue;
 }
 0x1201b681 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b688 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b68c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b68c); }
 let mut pc: u32 = 0x1201b68c;
 loop { match pc {
 0x1201b68c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120410a8u32); cpu.push(0x1201b697); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201b6a6; continue; }
  pc = 0x1201b6a0; continue;
 }
 0x1201b6a0 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b6a6); cpu.call(t);
  pc = 0x1201b6a6; continue;
 }
 0x1201b6a6 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b6b9; continue; }
  pc = 0x1201b6aa; continue;
 }
 0x1201b6aa => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b6b0); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b6b9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b6bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b6bc); }
 let mut pc: u32 = 0x1201b6bc;
 loop { match pc {
 0x1201b6bc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b6c6); f_1201b68c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b6ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b6ca); }
 let mut pc: u32 = 0x1201b6ca;
 loop { match pc {
 0x1201b6ca => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120410a4u32); cpu.push(0x1201b6d5); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201b6ef; continue; }
  pc = 0x1201b6da; continue;
 }
 0x1201b6da => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b6e0); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b6e6); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b6ef => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1201b716; continue; }
  pc = 0x1201b6f3; continue;
 }
 0x1201b6f3 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x2u8); }
  if cpu.zf { pc = 0x1201b716; continue; }
  pc = 0x1201b6fa; continue;
 }
 0x1201b6fa => {
  cpu.push(0x1201b6ff); f_120133b4(cpu);
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201b70a); f_120133bd(cpu);
  { let v: u32 = 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b716 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b719(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b719); }
 let mut pc: u32 = 0x1201b719;
 loop { match pc {
 0x1201b719 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x214u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204109cu32); cpu.push(0x1201b72a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b7ab; continue; }
  pc = 0x1201b72e; continue;
 }
 0x1201b72e => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x105u32; cpu.push(v); }
  let t = cpu.r32(0x120410a0u32); cpu.push(0x1201b73e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b7ab; continue; }
  pc = 0x1201b742; continue;
 }
 0x1201b742 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0x5cu16); }
  if cpu.zf { pc = 0x1201b752; continue; }
  pc = 0x1201b74a; continue;
 }
 0x1201b74a => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esp)), 0x2fu16); }
  if !cpu.zf { pc = 0x1201b767; continue; }
  pc = 0x1201b752; continue;
 }
 0x1201b752 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esp)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x1201b767; continue; }
  pc = 0x1201b75e; continue;
 }
 0x1201b75e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x214u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b767 => {
  { let v: u16 = 0x3du16; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b778); f_120161b5(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = 0x3au16; cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041134u32); cpu.push(0x1201b79e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201b7ab; continue; }
  pc = 0x1201b7a2; continue;
 }
 0x1201b7a2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x214u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b7ab => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b7b1); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b7b7); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x214u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b7c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b7c6); }
 let mut pc: u32 = 0x1201b7c6;
 loop { match pc {
 0x1201b7c6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x120410a4u32); cpu.push(0x1201b7d2); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201b7ed; continue; }
  pc = 0x1201b7d7; continue;
 }
 0x1201b7d7 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b7dd); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b7e3); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b7ed => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esp)), 0x80u8); }
  if cpu.zf { pc = 0x1201b819; continue; }
  pc = 0x1201b7f4; continue;
 }
 0x1201b7f4 => {
  { let v: u32 = cpu.eax & 0xfffffffeu32; cpu.eax = v; }
  pc = 0x1201b7f7; continue;
 }
 0x1201b7f7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041098u32); cpu.push(0x1201b7ff); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201b81e; continue; }
  pc = 0x1201b803; continue;
 }
 0x1201b803 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b809); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b80f); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b819 => {
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  pc = 0x1201b7f7; continue;
 }
 0x1201b81e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b822(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b822); }
 let mut pc: u32 = 0x1201b822;
 loop { match pc {
 0x1201b822 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201b8e7; continue; }
  pc = 0x1201b835; continue;
 }
 0x1201b835 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1201b8e7; continue; }
  pc = 0x1201b840; continue;
 }
 0x1201b840 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201b8ab; continue; }
  pc = 0x1201b848; continue;
 }
 0x1201b848 => {
  { let v = 0x208u32; cpu.push(v); }
  cpu.push(0x1201b852); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1201b870; continue; }
  pc = 0x1201b85b; continue;
 }
 0x1201b85b => {
  cpu.push(0x1201b860); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b870 => {
  { let v: u32 = 0x104u32; cpu.edi = v; }
  pc = 0x1201b875; continue;
 }
 0x1201b875 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041094u32); cpu.ebp = v; }
  let t = cpu.ebp; cpu.push(0x1201b885); cpu.call(t);
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.cf { pc = 0x1201b8b3; continue; }
  pc = 0x1201b889; continue;
 }
 0x1201b889 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201b896; continue; }
  pc = 0x1201b88d; continue;
 }
 0x1201b88d => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201b893); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201b896; continue;
 }
 0x1201b896 => {
  cpu.push(0x1201b89b); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x22u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b8ab => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x1201b875; continue;
 }
 0x1201b8b3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201b8dd; continue; }
  pc = 0x1201b8b7; continue;
 }
 0x1201b8b7 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201b8c4; continue; }
  pc = 0x1201b8bb; continue;
 }
 0x1201b8bb => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201b8c1); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201b8c4; continue;
 }
 0x1201b8c4 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201b8ca); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201b8d0); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b8dd => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b8e7 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201b8f6); f_1201b901(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
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
pub fn f_1201b901(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b901); }
 let mut pc: u32 = 0x1201b901;
 loop { match pc {
 0x1201b901 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201b909); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1201b91d); f_1201b962(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201b929); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b930); }
 let mut pc: u32 = 0x1201b930;
 loop { match pc {
 0x1201b930 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201b938); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201b94f); f_1201b962(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x1201b95b); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201b962(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201b962); }
 let mut pc: u32 = 0x1201b962;
 loop { match pc {
 0x1201b962 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x214u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x21cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1201ba2f; continue; }
  pc = 0x1201b978; continue;
 }
 0x1201b978 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201b97e); f_1201ac95(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201b9a5; continue; }
  pc = 0x1201b985; continue;
 }
 0x1201b985 => {
  cpu.push(0x1201b98a); f_120133bd(cpu);
  { let v: u32 = 0xfu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201b995); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x214u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201b9a5 => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(0x40u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u16 = 0x3au16; cpu.w16(0xau32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x2eu16; cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xeu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041094u32); cpu.push(0x1201b9dd); cpu.call(t);
  pc = 0x1201b9dd; continue;
 }
 0x1201b9dd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201ba75; continue; }
  pc = 0x1201b9e5; continue;
 }
 0x1201b9e5 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x104u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201ba75; continue; }
  pc = 0x1201b9f1; continue;
 }
 0x1201b9f1 => {
  { let v: u32 = cpu.r32(0x220u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1201ba41; continue; }
  pc = 0x1201b9fc; continue;
 }
 0x1201b9fc => {
  { let v: u32 = cpu.r32(0x224u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201ba09; continue; }
  pc = 0x1201ba07; continue;
 }
 0x1201ba07 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x1201ba09; continue;
 }
 0x1201ba09 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ba11); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1201ba5f; continue; }
  pc = 0x1201ba1a; continue;
 }
 0x1201ba1a => {
  cpu.push(0x1201ba1f); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x214u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ba2f => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x104u32; cpu.push(v); }
  let t = cpu.r32(0x120410a0u32); cpu.push(0x1201ba3f); cpu.call(t);
  pc = 0x1201b9dd; continue;
 }
 0x1201ba41 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x224u32.wrapping_add(cpu.esp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201ba5f; continue; }
  pc = 0x1201ba4a; continue;
 }
 0x1201ba4a => {
  cpu.push(0x1201ba4f); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x22u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x214u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ba5f => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201ba6a); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x214u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ba75 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x214u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ba7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ba7f); }
 let mut pc: u32 = 0x1201ba7f;
 loop { match pc {
 0x1201ba7f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041090u32); cpu.push(0x1201ba8c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201ba9b; continue; }
  pc = 0x1201ba95; continue;
 }
 0x1201ba95 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201ba9b); cpu.call(t);
  pc = 0x1201ba9b; continue;
 }
 0x1201ba9b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201baae; continue; }
  pc = 0x1201ba9f; continue;
 }
 0x1201ba9f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201baa5); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201baae => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201bab1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201bab1); }
 let mut pc: u32 = 0x1201bab1;
 loop { match pc {
 0x1201bab1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204108cu32); cpu.push(0x1201babc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201bacb; continue; }
  pc = 0x1201bac5; continue;
 }
 0x1201bac5 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201bacb); cpu.call(t);
  pc = 0x1201bacb; continue;
 }
 0x1201bacb => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bade; continue; }
  pc = 0x1201bacf; continue;
 }
 0x1201bacf => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bad5); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bade => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201bae1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201bae1); }
 let mut pc: u32 = 0x1201bae1;
 loop { match pc {
 0x1201bae1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201baeb); f_1201bab1(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201baef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201baef); }
 let mut pc: u32 = 0x1201baef;
 loop { match pc {
 0x1201baef => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041088u32); cpu.push(0x1201baff); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201bb0e; continue; }
  pc = 0x1201bb08; continue;
 }
 0x1201bb08 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201bb0e); cpu.call(t);
  pc = 0x1201bb0e; continue;
 }
 0x1201bb0e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bb21; continue; }
  pc = 0x1201bb12; continue;
 }
 0x1201bb12 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bb18); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bb21 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201bb24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201bb24); }
 let mut pc: u32 = 0x1201bb24;
 loop { match pc {
 0x1201bb24 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041084u32); cpu.push(0x1201bb2f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201bb3e; continue; }
  pc = 0x1201bb38; continue;
 }
 0x1201bb38 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201bb3e); cpu.call(t);
  pc = 0x1201bb3e; continue;
 }
 0x1201bb3e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bb51; continue; }
  pc = 0x1201bb42; continue;
 }
 0x1201bb42 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bb48); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bb51 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201bb54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201bb54); }
 let mut pc: u32 = 0x1201bb54;
 loop { match pc {
 0x1201bb54 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.edi)), 0x3au16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201bb70; continue; }
  pc = 0x1201bb6d; continue;
 }
 0x1201bb6d => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.eax = v; }
  pc = 0x1201bb70; continue;
 }
 0x1201bb70 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201bb7f; continue; }
  pc = 0x1201bb79; continue;
 }
 0x1201bb79 => {
  { cpu.sub16((cpu.ecx as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201bb86; continue; }
  pc = 0x1201bb7f; continue;
 }
 0x1201bb7f => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x1201bb94; continue; }
  pc = 0x1201bb86; continue;
 }
 0x1201bb86 => {
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if !cpu.zf { pc = 0x1201bb94; continue; }
  pc = 0x1201bb8b; continue;
 }
 0x1201bb8b => {
  { let v: u16 = 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x1201bb98; continue; }
  pc = 0x1201bb94; continue;
 }
 0x1201bb94 => {
  { let v: u16 = 0x4040u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x1201bb98; continue;
 }
 0x1201bb98 => {
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v = 0x2eu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x80u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { let v: u16 = (cpu.esi as u16) | (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x1201bbb5); f_1202e1f5(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201bc0a; continue; }
  pc = 0x1201bbbe; continue;
 }
 0x1201bbbe => {
  { let v = 0x1203655cu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201bbc9); f_120155e2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bc06; continue; }
  pc = 0x1201bbd0; continue;
 }
 0x1201bbd0 => {
  { let v = 0x12036550u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201bbdb); f_120155e2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bc06; continue; }
  pc = 0x1201bbe2; continue;
 }
 0x1201bbe2 => {
  { let v = 0x12036544u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201bbed); f_120155e2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bc06; continue; }
  pc = 0x1201bbf4; continue;
 }
 0x1201bbf4 => {
  { let v = 0x12036538u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201bbff); f_120155e2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201bc0a; continue; }
  pc = 0x1201bc06; continue;
 }
 0x1201bc06 => {
  { let v: u16 = (cpu.esi as u16) | 0x40u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x1201bc0a; continue;
 }
 0x1201bc0a => {
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x1c0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.esi as u16) | (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x1c0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.esi as u16), (cpu.eax as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201bc2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201bc2c); }
 let mut pc: u32 = 0x1201bc2c;
 loop { match pc {
 0x1201bc2c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x470u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x47cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x12036570u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201bc48); f_120154e6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bc75; continue; }
  pc = 0x1201bc4f; continue;
 }
 0x1201bc4f => {
  cpu.push(0x1201bc54); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201bc5f); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bc75 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), 0x3au16); }
  if !cpu.zf { pc = 0x1201bdbc; continue; }
  pc = 0x1201bc80; continue;
 }
 0x1201bc80 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1201bcb5; continue; }
  pc = 0x1201bc88; continue;
 }
 0x1201bc88 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x1201bcb5; continue; }
  pc = 0x1201bc8f; continue;
 }
 0x1201bc8f => {
  cpu.push(0x1201bc94); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201bc9f); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bcb5 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bcbb); f_12015e7f(cpu);
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x60u32); cpu.edi = v; }
  pc = 0x1201bcc4; continue;
 }
 0x1201bcc4 => {
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041190u32); cpu.push(0x1201bcd0); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  if !cpu.zf { pc = 0x1201bdee; continue; }
  pc = 0x1201bcdb; continue;
 }
 0x1201bcdb => {
  { let v = 0x12036568u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201bce6); f_120154e6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bdc8; continue; }
  pc = 0x1201bcf1; continue;
 }
 0x1201bcf1 => {
  { let v: u32 = 0x278u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bd04); f_1201b822(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201bdc8; continue; }
  pc = 0x1201bd11; continue;
 }
 0x1201bd11 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201bd17); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x1201bd30; continue; }
  pc = 0x1201bd1f; continue;
 }
 0x1201bd1f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201bd25); f_1201bfe5(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bdc8; continue; }
  pc = 0x1201bd30; continue;
 }
 0x1201bd30 => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x12041080u32); cpu.push(0x1201bd37); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201bdc8; continue; }
  pc = 0x1201bd40; continue;
 }
 0x1201bd40 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x488u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x54u32.wrapping_add(cpu.esp), v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x7bcu32; cpu.push(v); }
  cpu.push(0x1201bd71); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201bd7d; continue;
 }
 0x1201bd7d => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bd88); f_1201bb54(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bdbc => {
  cpu.push(0x1201bdc1); f_1201a8f4(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x1201bcc4; continue;
 }
 0x1201bdc8 => {
  cpu.push(0x1201bdcd); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201bdd8); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bdee => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201bdfe); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bfbf; continue; }
  pc = 0x1201be06; continue;
 }
 0x1201be06 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201be16); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bfbf; continue; }
  pc = 0x1201be1e; continue;
 }
 0x1201be1e => {
  { let v: u32 = cpu.r32(0x488u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201be62); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201be76; continue; }
  pc = 0x1201be6f; continue;
 }
 0x1201be6f => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201bee6; continue; }
  pc = 0x1201be76; continue;
 }
 0x1201be76 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201be86); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bf99; continue; }
  pc = 0x1201be8e; continue;
 }
 0x1201be8e => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201be9e); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bf99; continue; }
  pc = 0x1201bea6; continue;
 }
 0x1201bea6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bee3); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201bee6; continue;
 }
 0x1201bee6 => {
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x1201bf09; continue; }
  pc = 0x1201bef0; continue;
 }
 0x1201bef0 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201bf09; continue; }
  pc = 0x1201bef7; continue;
 }
 0x1201bef7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1201befa; continue;
 }
 0x1201befa => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201bf04); cpu.call(t);
  pc = 0x1201bd7d; continue;
 }
 0x1201bf09 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201bf19); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bf73; continue; }
  pc = 0x1201bf1d; continue;
 }
 0x1201bf1d => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201bf2d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201bf73; continue; }
  pc = 0x1201bf31; continue;
 }
 0x1201bf31 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bf6e); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201befa; continue;
 }
 0x1201bf73 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201bf79); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bf7f); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201bf89); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bf99 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201bf9f); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bfa5); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201bfaf); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201bfbf => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201bfc5); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201bfcb); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201bfd5); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201bfe5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201bfe5); }
 let mut pc: u32 = 0x1201bfe5;
 loop { match pc {
 0x1201bfe5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201bff0); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if cpu.cf { pc = 0x1201c06b; continue; }
  pc = 0x1201bff8; continue;
 }
 0x1201bff8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201c007; continue; }
  pc = 0x1201c001; continue;
 }
 0x1201c001 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201c06b; continue; }
  pc = 0x1201c007; continue;
 }
 0x1201c007 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201c017; continue; }
  pc = 0x1201c011; continue;
 }
 0x1201c011 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201c06b; continue; }
  pc = 0x1201c017; continue;
 }
 0x1201c017 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x1201c01a; continue;
 }
 0x1201c01a => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1201c032; continue; }
  pc = 0x1201c026; continue;
 }
 0x1201c026 => {
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201c032; continue; }
  pc = 0x1201c02c; continue;
 }
 0x1201c02c => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201c01a; continue; }
  pc = 0x1201c032; continue;
 }
 0x1201c032 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1201c06b; continue; }
  pc = 0x1201c038; continue;
 }
 0x1201c038 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1201c06b; continue; }
  pc = 0x1201c03f; continue;
 }
 0x1201c03f => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1201c057; continue; }
  pc = 0x1201c04b; continue;
 }
 0x1201c04b => {
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201c057; continue; }
  pc = 0x1201c051; continue;
 }
 0x1201c051 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201c03f; continue; }
  pc = 0x1201c057; continue;
 }
 0x1201c057 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1201c064; continue; }
  pc = 0x1201c05d; continue;
 }
 0x1201c05d => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x1201c06b; continue; }
  pc = 0x1201c064; continue;
 }
 0x1201c064 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c06b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c06f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c06f); }
 let mut pc: u32 = 0x1201c06f;
 loop { match pc {
 0x1201c06f => {
  { let v: u32 = cpu.esp.wrapping_sub(0x470u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x47cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x12036570u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201c08b); f_120154e6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c0b8; continue; }
  pc = 0x1201c092; continue;
 }
 0x1201c092 => {
  cpu.push(0x1201c097); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201c0a2); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c0b8 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), 0x3au16); }
  if !cpu.zf { pc = 0x1201c216; continue; }
  pc = 0x1201c0c3; continue;
 }
 0x1201c0c3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1201c0f8; continue; }
  pc = 0x1201c0cb; continue;
 }
 0x1201c0cb => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x1201c0f8; continue; }
  pc = 0x1201c0d2; continue;
 }
 0x1201c0d2 => {
  cpu.push(0x1201c0d7); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201c0e2); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c0f8 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c0fe); f_12015e7f(cpu);
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x60u32); cpu.edi = v; }
  pc = 0x1201c107; continue;
 }
 0x1201c107 => {
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041190u32); cpu.push(0x1201c113); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  if !cpu.zf { pc = 0x1201c248; continue; }
  pc = 0x1201c11e; continue;
 }
 0x1201c11e => {
  { let v = 0x12036568u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201c129); f_120154e6(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c222; continue; }
  pc = 0x1201c134; continue;
 }
 0x1201c134 => {
  { let v: u32 = 0x278u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x104u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c147); f_1201b822(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201c222; continue; }
  pc = 0x1201c154; continue;
 }
 0x1201c154 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201c15a); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x1201c173; continue; }
  pc = 0x1201c162; continue;
 }
 0x1201c162 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201c168); f_1201c43f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c222; continue; }
  pc = 0x1201c173; continue;
 }
 0x1201c173 => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x12041080u32); cpu.push(0x1201c17a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201c222; continue; }
  pc = 0x1201c183; continue;
 }
 0x1201c183 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x488u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x54u32.wrapping_add(cpu.esp), v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x7bcu32; cpu.push(v); }
  cpu.push(0x1201c1b4); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201c1c0; continue;
 }
 0x1201c1c0 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c1cb); f_1201bb54(cpu);
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c1e8); f_12018734(cpu);
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x48u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c216 => {
  cpu.push(0x1201c21b); f_1201a8f4(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x1201c107; continue;
 }
 0x1201c222 => {
  cpu.push(0x1201c227); f_120133b4(cpu);
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201c232); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c248 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201c258); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c419; continue; }
  pc = 0x1201c260; continue;
 }
 0x1201c260 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201c270); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c419; continue; }
  pc = 0x1201c278; continue;
 }
 0x1201c278 => {
  { let v: u32 = cpu.r32(0x488u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c2bc); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201c2d0; continue; }
  pc = 0x1201c2c9; continue;
 }
 0x1201c2c9 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201c340; continue; }
  pc = 0x1201c2d0; continue;
 }
 0x1201c2d0 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201c2e0); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c3f3; continue; }
  pc = 0x1201c2e8; continue;
 }
 0x1201c2e8 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201c2f8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c3f3; continue; }
  pc = 0x1201c300; continue;
 }
 0x1201c300 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c33d); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201c340; continue;
 }
 0x1201c340 => {
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x1201c363; continue; }
  pc = 0x1201c34a; continue;
 }
 0x1201c34a => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201c363; continue; }
  pc = 0x1201c351; continue;
 }
 0x1201c351 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1201c354; continue;
 }
 0x1201c354 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201c35e); cpu.call(t);
  pc = 0x1201c1c0; continue;
 }
 0x1201c363 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410dcu32); cpu.push(0x1201c373); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c3cd; continue; }
  pc = 0x1201c377; continue;
 }
 0x1201c377 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120410e0u32); cpu.push(0x1201c387); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c3cd; continue; }
  pc = 0x1201c38b; continue;
 }
 0x1201c38b => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c3c8); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201c354; continue;
 }
 0x1201c3cd => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201c3d3); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c3d9); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201c3e3); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c3f3 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201c3f9); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c3ff); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201c409); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c419 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201c41f); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c425); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x120411a0u32); cpu.push(0x1201c42f); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x470u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c43f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c43f); }
 let mut pc: u32 = 0x1201c43f;
 loop { match pc {
 0x1201c43f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201c44a); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if cpu.cf { pc = 0x1201c4c5; continue; }
  pc = 0x1201c452; continue;
 }
 0x1201c452 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201c461; continue; }
  pc = 0x1201c45b; continue;
 }
 0x1201c45b => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201c4c5; continue; }
  pc = 0x1201c461; continue;
 }
 0x1201c461 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201c471; continue; }
  pc = 0x1201c46b; continue;
 }
 0x1201c46b => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201c4c5; continue; }
  pc = 0x1201c471; continue;
 }
 0x1201c471 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x1201c474; continue;
 }
 0x1201c474 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1201c48c; continue; }
  pc = 0x1201c480; continue;
 }
 0x1201c480 => {
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201c48c; continue; }
  pc = 0x1201c486; continue;
 }
 0x1201c486 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201c474; continue; }
  pc = 0x1201c48c; continue;
 }
 0x1201c48c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1201c4c5; continue; }
  pc = 0x1201c492; continue;
 }
 0x1201c492 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1201c4c5; continue; }
  pc = 0x1201c499; continue;
 }
 0x1201c499 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1201c4b1; continue; }
  pc = 0x1201c4a5; continue;
 }
 0x1201c4a5 => {
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201c4b1; continue; }
  pc = 0x1201c4ab; continue;
 }
 0x1201c4ab => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201c499; continue; }
  pc = 0x1201c4b1; continue;
 }
 0x1201c4b1 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1201c4be; continue; }
  pc = 0x1201c4b7; continue;
 }
 0x1201c4b7 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x1201c4c5; continue; }
  pc = 0x1201c4be; continue;
 }
 0x1201c4be => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c4c5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c4c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c4c9); }
 let mut pc: u32 = 0x1201c4c9;
 loop { match pc {
 0x1201c4c9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1204107cu32); cpu.push(0x1201c4d9); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201c4e8; continue; }
  pc = 0x1201c4e2; continue;
 }
 0x1201c4e2 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201c4e8); cpu.call(t);
  pc = 0x1201c4e8; continue;
 }
 0x1201c4e8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c4fb; continue; }
  pc = 0x1201c4ec; continue;
 }
 0x1201c4ec => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c4f2); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c4fb => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c4fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c4fe); }
 let mut pc: u32 = 0x1201c4fe;
 loop { match pc {
 0x1201c4fe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12036578u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x1201c58c; continue; }
  pc = 0x1201c52b; continue;
 }
 0x1201c52b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c538); f_1201c7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201c542; continue;
 }
 0x1201c542 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c58c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1201c542; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c553(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c553); }
 let mut pc: u32 = 0x1201c553;
 loop { match pc {
 0x1201c553 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3ffffffbu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c567(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c567); }
 let mut pc: u32 = 0x1201c567;
 loop { match pc {
 0x1201c542 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c567 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  { let v = 0x12036584u32; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x1201c577); f_12011935(cpu);
  { let v = 0x12037068u32; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c585); f_12011239(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1201c542; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c590); }
 let mut pc: u32 = 0x1201c590;
 loop { match pc {
 0x1201c590 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x120365a8u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x3cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x1201c5dc; continue; }
  pc = 0x1201c5bd; continue;
 }
 0x1201c5bd => {
  { let v = 0x120365d8u32; cpu.push(v); }
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x1201c5ca); f_1201172d(cpu);
  { let v = 0x12037088u32; cpu.push(v); }
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c5d8); f_12011239(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1201c602; continue;
 }
 0x1201c5dc => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041218u32); cpu.push(0x1201c5f4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201c613; continue; }
  pc = 0x1201c5f8; continue;
 }
 0x1201c5f8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201c602; continue;
 }
 0x1201c602 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c613 => {
  { let v = 0x120365b4u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x1201c620); f_12011935(cpu);
  { let v = 0x12037068u32; cpu.push(v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c62e); f_12011239(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201c602; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c639(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c639); }
 let mut pc: u32 = 0x1201c639;
 loop { match pc {
 0x1201c639 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3ffffffbu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c64d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c64d); }
 let mut pc: u32 = 0x1201c64d;
 loop { match pc {
 0x1201c5d8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1201c602; continue;
 }
 0x1201c602 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c64d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  { let v = 0x12036584u32; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x1201c65d); f_12011935(cpu);
  { let v = 0x12037068u32; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c66b); f_12011239(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201c5d8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c677(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c677); }
 let mut pc: u32 = 0x1201c677;
 loop { match pc {
 0x1201c677 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x12036600u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201c6b7; continue; }
  pc = 0x1201c6a4; continue;
 }
 0x1201c6a4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1201c6a6; continue;
 }
 0x1201c6a6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c6b7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201c6c4); f_1201c7a0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201c6cf); f_1201c590(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x1201c71e; continue; }
  pc = 0x1201c6e9; continue;
 }
 0x1201c6e9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201c6f9); f_1201c7ba(cpu);
  pc = 0x1201c6f9; continue;
 }
 0x1201c6f9 => {
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201c73b; continue; }
  pc = 0x1201c700; continue;
 }
 0x1201c700 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201c70a); f_1201cb98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201c712; continue;
 }
 0x1201c712 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1201c6a6; continue;
 }
 0x1201c71e => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x1201c734; continue; }
  pc = 0x1201c72d; continue;
 }
 0x1201c72d => {
  cpu.push(0x1201c732); f_1201c827(cpu);
  pc = 0x1201c6f9; continue;
 }
 0x1201c734 => {
  cpu.push(0x1201c739); f_1201c9b7(cpu);
  pc = 0x1201c6f9; continue;
 }
 0x1201c73b => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1201c712; continue; }
  pc = 0x1201c748; continue;
 }
 0x1201c748 => {
  { let v: u32 = 0x1203660cu32; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x1201c75b); f_12011521(cpu);
  { let v = 0x120370c8u32; cpu.push(v); }
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c769); f_12011239(cpu);
  pc = 0x1201c712; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c76b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c76b); }
 let mut pc: u32 = 0x1201c76b;
 loop { match pc {
 0x1201c76b => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3ffffffbu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c77f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c77f); }
 let mut pc: u32 = 0x1201c77f;
 loop { match pc {
 0x1201c6a6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c712 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1201c6a6; continue;
 }
 0x1201c763 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201c769); f_12011239(cpu);
  pc = 0x1201c712; continue;
 }
 0x1201c77f => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x12036584u32; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x1201c796); f_12011935(cpu);
  { let v = 0x12037068u32; cpu.push(v); }
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  pc = 0x1201c763; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c7a0); }
 let mut pc: u32 = 0x1201c7a0;
 loop { match pc {
 0x1201c7a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1201c7b9; continue; }
  pc = 0x1201c7b5; continue;
 }
 0x1201c7b5 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  pc = 0x1201c7b9; continue;
 }
 0x1201c7b9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c7ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c7ba); }
 let mut pc: u32 = 0x1201c7ba;
 loop { match pc {
 0x1201c7ba => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x1201c819; continue; }
  pc = 0x1201c7cf; continue;
 }
 0x1201c7cf => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x1201c7d3; continue;
 }
 0x1201c7d3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x1201c803; continue; }
  pc = 0x1201c7d9; continue;
 }
 0x1201c7d9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  pc = 0x1201c7e1; continue;
 }
 0x1201c7e1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x1201c820; continue; }
  pc = 0x1201c7e7; continue;
 }
 0x1201c7e7 => {
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1201c7fd; continue; }
  pc = 0x1201c7eb; continue;
 }
 0x1201c7eb => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x1u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x1201c820; continue; }
  pc = 0x1201c7f3; continue;
 }
 0x1201c7f3 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1201c7e1; continue; }
  pc = 0x1201c7fd; continue;
 }
 0x1201c7fd => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x1201c7ff; continue;
 }
 0x1201c7ff => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201c809; continue; }
  pc = 0x1201c803; continue;
 }
 0x1201c803 => {
  { cpu.and8(cpu.r8(0x14u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x1201c81b; continue; }
  pc = 0x1201c809; continue;
 }
 0x1201c809 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201c7d3; continue; }
  pc = 0x1201c819; continue;
 }
 0x1201c819 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1201c81b; continue;
 }
 0x1201c81b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c820 => {
  { let v: u32 = cpu.sbb32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, 0xffffffffu32); cpu.esi = v; }
  pc = 0x1201c7ff; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c827(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c827); }
 let mut pc: u32 = 0x1201c827;
 loop { match pc {
 0x1201c827 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  { let v = cpu.ebp; cpu.push(v); }
  if (cpu.cf || cpu.zf) { pc = 0x1201c927; continue; }
  pc = 0x1201c84e; continue;
 }
 0x1201c84e => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x1201c852; continue;
 }
 0x1201c852 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x30u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1201c898; continue; }
  pc = 0x1201c864; continue;
 }
 0x1201c864 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  pc = 0x1201c86e; continue;
 }
 0x1201c86e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x1201c990; continue; }
  pc = 0x1201c878; continue;
 }
 0x1201c878 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1201c892; continue; }
  pc = 0x1201c87c; continue;
 }
 0x1201c87c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x1201c990; continue; }
  pc = 0x1201c888; continue;
 }
 0x1201c888 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1201c86e; continue; }
  pc = 0x1201c892; continue;
 }
 0x1201c892 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1201c894; continue;
 }
 0x1201c894 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201c90a; continue; }
  pc = 0x1201c898; continue;
 }
 0x1201c898 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201c90a; continue; }
  pc = 0x1201c8aa; continue;
 }
 0x1201c8aa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x1201c8e2; continue; }
  pc = 0x1201c8b0; continue;
 }
 0x1201c8b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x1201c8b8; continue;
 }
 0x1201c8b8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x1201c99a; continue; }
  pc = 0x1201c8c2; continue;
 }
 0x1201c8c2 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1201c8dc; continue; }
  pc = 0x1201c8c6; continue;
 }
 0x1201c8c6 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x1201c99a; continue; }
  pc = 0x1201c8d2; continue;
 }
 0x1201c8d2 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1201c8b8; continue; }
  pc = 0x1201c8dc; continue;
 }
 0x1201c8dc => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x1201c8de; continue;
 }
 0x1201c8de => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1201c8fd; continue; }
  pc = 0x1201c8e2; continue;
 }
 0x1201c8e2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201c8f0); f_1201cb98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x2cu32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1201c9a4; continue; }
  pc = 0x1201c8fd; continue;
 }
 0x1201c8fd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201c8aa; continue; }
  pc = 0x1201c90a; continue;
 }
 0x1201c90a => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201c852; continue; }
  pc = 0x1201c927; continue;
 }
 0x1201c927 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201c986; continue; }
  pc = 0x1201c938; continue;
 }
 0x1201c938 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x30u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1201c96e; continue; }
  pc = 0x1201c942; continue;
 }
 0x1201c942 => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  pc = 0x1201c94c; continue;
 }
 0x1201c94c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x1201c9b0; continue; }
  pc = 0x1201c952; continue;
 }
 0x1201c952 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1201c968; continue; }
  pc = 0x1201c956; continue;
 }
 0x1201c956 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x1201c9b0; continue; }
  pc = 0x1201c95e; continue;
 }
 0x1201c95e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1201c94c; continue; }
  pc = 0x1201c968; continue;
 }
 0x1201c968 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x1201c96a; continue;
 }
 0x1201c96a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1201c976; continue; }
  pc = 0x1201c96e; continue;
 }
 0x1201c96e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x1201c988; continue; }
  pc = 0x1201c976; continue;
 }
 0x1201c976 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201c938; continue; }
  pc = 0x1201c986; continue;
 }
 0x1201c986 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1201c988; continue;
 }
 0x1201c988 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c990 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x1201c894; continue;
 }
 0x1201c99a => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x1201c8de; continue;
 }
 0x1201c9a4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201c9b0 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x1201c96a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201c9b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201c9b7); }
 let mut pc: u32 = 0x1201c9b7;
 loop { match pc {
 0x1201c9b7 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.ecx); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  if (cpu.cf || cpu.zf) { pc = 0x1201caf7; continue; }
  pc = 0x1201c9df; continue;
 }
 0x1201c9df => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x1201c9e3; continue;
 }
 0x1201c9e3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), cpu.eax); }
  if cpu.zf { pc = 0x1201ca29; continue; }
  pc = 0x1201c9f1; continue;
 }
 0x1201c9f1 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x1201c9fb; continue;
 }
 0x1201c9fb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x1201cb0a; continue; }
  pc = 0x1201ca05; continue;
 }
 0x1201ca05 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1201ca1f; continue; }
  pc = 0x1201ca09; continue;
 }
 0x1201ca09 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x1u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x1201cb0a; continue; }
  pc = 0x1201ca15; continue;
 }
 0x1201ca15 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1201c9fb; continue; }
  pc = 0x1201ca1f; continue;
 }
 0x1201ca1f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1201ca21; continue;
 }
 0x1201ca21 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201cada; continue; }
  pc = 0x1201ca29; continue;
 }
 0x1201ca29 => {
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201cada; continue; }
  pc = 0x1201ca42; continue;
 }
 0x1201ca42 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), cpu.ecx); }
  if cpu.zf { pc = 0x1201ca80; continue; }
  pc = 0x1201ca4c; continue;
 }
 0x1201ca4c => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  pc = 0x1201ca56; continue;
 }
 0x1201ca56 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x1201cb14; continue; }
  pc = 0x1201ca60; continue;
 }
 0x1201ca60 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1201ca7a; continue; }
  pc = 0x1201ca64; continue;
 }
 0x1201ca64 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x1201cb14; continue; }
  pc = 0x1201ca70; continue;
 }
 0x1201ca70 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1201ca56; continue; }
  pc = 0x1201ca7a; continue;
 }
 0x1201ca7a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x1201ca7c; continue;
 }
 0x1201ca7c => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1201cac6; continue; }
  pc = 0x1201ca80; continue;
 }
 0x1201ca80 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201ca8a); f_1201cb98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x34u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x1201cac6; continue; }
  pc = 0x1201ca93; continue;
 }
 0x1201ca93 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201cac2; continue; }
  pc = 0x1201ca9a; continue;
 }
 0x1201ca9a => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201caa4); f_1201cb98(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201cab9); f_1201cb98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), cpu.eax); }
  if !cpu.zf { pc = 0x1201cb1e; continue; }
  pc = 0x1201cac2; continue;
 }
 0x1201cac2 => {
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201cac6; continue;
 }
 0x1201cac6 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201ca42; continue; }
  pc = 0x1201cada; continue;
 }
 0x1201cada => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.esp))); cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)).wrapping_add(0x4u32); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201c9e3; continue; }
  pc = 0x1201caf7; continue;
 }
 0x1201caf7 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201cb28; continue; }
  pc = 0x1201cafe; continue;
 }
 0x1201cafe => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201cb0a => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x1201ca21; continue;
 }
 0x1201cb14 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x1201ca7c; continue;
 }
 0x1201cb1e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201cb28 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201cb87; continue; }
  pc = 0x1201cb39; continue;
 }
 0x1201cb39 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), cpu.edi); }
  if cpu.zf { pc = 0x1201cb6f; continue; }
  pc = 0x1201cb43; continue;
 }
 0x1201cb43 => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  pc = 0x1201cb4d; continue;
 }
 0x1201cb4d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x1201cb91; continue; }
  pc = 0x1201cb53; continue;
 }
 0x1201cb53 => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1201cb69; continue; }
  pc = 0x1201cb57; continue;
 }
 0x1201cb57 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x1u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x1201cb91; continue; }
  pc = 0x1201cb5f; continue;
 }
 0x1201cb5f => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1201cb4d; continue; }
  pc = 0x1201cb69; continue;
 }
 0x1201cb69 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x1201cb6b; continue;
 }
 0x1201cb6b => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1201cb77; continue; }
  pc = 0x1201cb6f; continue;
 }
 0x1201cb6f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x1201cb89; continue; }
  pc = 0x1201cb77; continue;
 }
 0x1201cb77 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201cb39; continue; }
  pc = 0x1201cb87; continue;
 }
 0x1201cb87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1201cb89; continue;
 }
 0x1201cb89 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201cb91 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x1201cb6b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cb98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cb98); }
 let mut pc: u32 = 0x1201cb98;
 loop { match pc {
 0x1201cb98 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x1201cbb5; continue; }
  pc = 0x1201cba6; continue;
 }
 0x1201cba6 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  pc = 0x1201cbb5; continue;
 }
 0x1201cbb5 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cbb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cbb9); }
 let mut pc: u32 = 0x1201cbb9;
 loop { match pc {
 0x1201cbb9 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201cc43; continue; }
  pc = 0x1201cbc8; continue;
 }
 0x1201cbc8 => {
  { cpu.sub32(cpu.esi, 0xfffffffeu32); }
  if cpu.zf { pc = 0x1201cc43; continue; }
  pc = 0x1201cbcd; continue;
 }
 0x1201cbcd => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041074u32); cpu.push(0x1201cbd6); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201cc0a; continue; }
  pc = 0x1201cbda; continue;
 }
 0x1201cbda => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x12041078u32); cpu.push(0x1201cbe6); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201cc0a; continue; }
  pc = 0x1201cbea; continue;
 }
 0x1201cbea => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x1201cbec; continue;
 }
 0x1201cbec => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x1201cbf3); cpu.call(t);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201cc01; continue; }
  pc = 0x1201cbfb; continue;
 }
 0x1201cbfb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201cc01; continue;
 }
 0x1201cc01 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201cc0a => {
  { let v: u32 = cpu.r32(0x120411ccu32); cpu.edi = v; }
  let t = cpu.edi; cpu.push(0x1201cc12); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x6u32); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  if !cpu.zf { pc = 0x1201cc36; continue; }
  pc = 0x1201cc19; continue;
 }
 0x1201cc19 => {
  cpu.push(0x1201cc1e); f_120133b4(cpu);
  { let v: u32 = 0xau32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201cc29); f_120133bd(cpu);
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201cc2b; continue;
 }
 0x1201cc2b => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x1201cbec; continue;
 }
 0x1201cc36 => {
  let t = cpu.edi; cpu.push(0x1201cc38); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201cc3e); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201cc2b; continue;
 }
 0x1201cc43 => {
  cpu.push(0x1201cc48); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cc5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cc5a); }
 let mut pc: u32 = 0x1201cc5a;
 loop { match pc {
 0x1201cc5a => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201cc6b); f_1201ccdf(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cc6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cc6f); }
 let mut pc: u32 = 0x1201cc6f;
 loop { match pc {
 0x1201cc6f => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x1201cc73; continue;
 }
 0x1201cc73 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201cc73; continue; }
  pc = 0x1201cc7d; continue;
 }
 0x1201cc7d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201cc8f); f_1201ccdf(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cc93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cc93); }
 let mut pc: u32 = 0x1201cc93;
 loop { match pc {
 0x1201cc93 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201cca2); f_1201cf64(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cca6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cca6); }
 let mut pc: u32 = 0x1201cca6;
 loop { match pc {
 0x1201cca6 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1201ccaa; continue;
 }
 0x1201ccaa => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1201ccaa; continue; }
  pc = 0x1201ccb4; continue;
 }
 0x1201ccb4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201ccc6); f_1201cf79(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ccca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ccca); }
 let mut pc: u32 = 0x1201ccca;
 loop { match pc {
 0x1201ccca => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201ccdb); f_1201ccdf(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ccdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ccdf); }
 let mut pc: u32 = 0x1201ccdf;
 loop { match pc {
 0x1201ccdf => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ccf6); f_12005b48(cpu);
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201cd07); f_12005b48(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201ce19; continue; }
  pc = 0x1201cd12; continue;
 }
 0x1201cd12 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201cdb4; continue; }
  pc = 0x1201cd1a; continue;
 }
 0x1201cd1a => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x3au32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201cd26); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201cdb4; continue; }
  pc = 0x1201cd33; continue;
 }
 0x1201cd33 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201cd4b); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201cd63; continue; }
  pc = 0x1201cd56; continue;
 }
 0x1201cd56 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201cd63 => {
  { let v: u16 = cpu.r16(0x12036620u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x12036620u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x1201cdb4; continue;
 }
 0x1201cdb4 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v = 0x2eu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201cdc4); f_12005b48(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201ce25; continue; }
  pc = 0x1201cdcb; continue;
 }
 0x1201cdcb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201cdd7); f_1201a801(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201cdfa; continue; }
  pc = 0x1201cddf; continue;
 }
 0x1201cddf => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201cdf3); f_1201cef8(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1201cdfa; continue;
 }
 0x1201cdfa => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x20u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1201ce0d; continue; }
  pc = 0x1201ce04; continue;
 }
 0x1201ce04 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ce0a); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201ce0d; continue;
 }
 0x1201ce0d => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ce19 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1201ce21; continue; }
  pc = 0x1201ce1d; continue;
 }
 0x1201ce1d => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201cdb4; continue; }
  pc = 0x1201ce21; continue;
 }
 0x1201ce21 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x1201cdb4; continue;
 }
 0x1201ce25 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201ce3d); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1201ce53; continue; }
  pc = 0x1201ce46; continue;
 }
 0x1201ce46 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ce53 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1203d08cu32; cpu.ebp = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201ce92; continue;
 }
 0x1201ce92 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x1201cebe); f_1201a801(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201cedf; continue; }
  pc = 0x1201cec6; continue;
 }
 0x1201cec6 => {
  { let v: u32 = cpu.ebp.wrapping_sub(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1203d080u32); }
  if !cpu.cf { pc = 0x1201ce92; continue; }
  pc = 0x1201ced1; continue;
 }
 0x1201ced1 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201ced7); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201cdfa; continue;
 }
 0x1201cedf => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201ceef); f_1201cef8(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1201ced1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cef8); }
 let mut pc: u32 = 0x1201cef8;
 loop { match pc {
 0x1201cef8 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201cf1a); f_1201e394(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201cf2c; continue; }
  pc = 0x1201cf22; continue;
 }
 0x1201cf22 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201cf2c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1201cf3e); f_1201e11c(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201cf4d); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201cf5a); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cf64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cf64); }
 let mut pc: u32 = 0x1201cf64;
 loop { match pc {
 0x1201cf64 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201cf75); f_1201cf79(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201cf79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201cf79); }
 let mut pc: u32 = 0x1201cf79;
 loop { match pc {
 0x1201cf79 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201cf9c); f_1201ccdf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.push(0x1201cfa4); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if !cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201cfad; continue;
 }
 0x1201cfad => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201cfb9); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201cfc4; continue;
 }
 0x1201cfc4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201cfd0); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201cfdb; continue;
 }
 0x1201cfdb => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1201cfee; continue; }
  pc = 0x1201cfe4; continue;
 }
 0x1201cfe4 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.eax)), 0x3au8); }
  if cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201cfee; continue;
 }
 0x1201cfee => {
  { let v = 0x12036628u32; cpu.push(v); }
  cpu.push(0x1201cff8); f_12018768(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201d005; continue;
 }
 0x1201d005 => {
  { let v = 0x104u32; cpu.push(v); }
  cpu.push(0x1201d00f); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201d020; continue;
 }
 0x1201d020 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x1201d043; continue; }
  pc = 0x1201d025; continue;
 }
 0x1201d025 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x3bu8); }
  if cpu.zf { pc = 0x1201d043; continue; }
  pc = 0x1201d02c; continue;
 }
 0x1201d02c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x102u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.cf || cpu.zf) { pc = 0x1201d043; continue; }
  pc = 0x1201d039; continue;
 }
 0x1201d039 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x1201d025; continue; }
  pc = 0x1201d043; continue;
 }
 0x1201d043 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x1201d09b; continue; }
  pc = 0x1201d054; continue;
 }
 0x1201d054 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d060); f_12005b48(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x1201d09f; continue; }
  pc = 0x1201d067; continue;
 }
 0x1201d067 => {
  { let v: u32 = 0x12036624u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x1201d09f; continue;
 }
 0x1201d09b => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201d067; continue; }
  pc = 0x1201d09f; continue;
 }
 0x1201d09f => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x104u32); }
  if !cpu.cf { pc = 0x1201d188; continue; }
  pc = 0x1201d0cf; continue;
 }
 0x1201d0cf => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d114); f_1201ccdf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.push(0x1201d11c); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if cpu.zf { pc = 0x1201d178; continue; }
  pc = 0x1201d121; continue;
 }
 0x1201d121 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d12d); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1201d14b; continue; }
  pc = 0x1201d136; continue;
 }
 0x1201d136 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d142); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201d14b; continue;
 }
 0x1201d14b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x5cu32; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d158); f_1200236d(cpu);
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1201d178; continue; }
  pc = 0x1201d164; continue;
 }
 0x1201d164 => {
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d16c); f_1200236d(cpu);
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201d178; continue;
 }
 0x1201d178 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x1201d188; continue; }
  pc = 0x1201d17d; continue;
 }
 0x1201d17d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d020; continue; }
  pc = 0x1201d188; continue;
 }
 0x1201d188 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201d19c; continue; }
  pc = 0x1201d18f; continue;
 }
 0x1201d18f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d199); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201d19c; continue;
 }
 0x1201d19c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
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
pub fn f_1201d1a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d1a9); }
 let mut pc: u32 = 0x1201d1a9;
 loop { match pc {
 0x1201d1a9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d1ca; continue; }
  pc = 0x1201d1b1; continue;
 }
 0x1201d1b1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201d1de; continue; }
  pc = 0x1201d1bd; continue;
 }
 0x1201d1bd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411ecu32); cpu.push(0x1201d1c9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d1ca => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201d1de; continue; }
  pc = 0x1201d1d1; continue;
 }
 0x1201d1d1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411ecu32); cpu.push(0x1201d1dd); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d1de => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d1e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d1e1); }
 let mut pc: u32 = 0x1201d1e1;
 loop { match pc {
 0x1201d1e1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204115cu32); cpu.push(0x1201d1ec); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d1ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d1ed); }
 let mut pc: u32 = 0x1201d1ed;
 loop { match pc {
 0x1201d1ed => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041070u32); cpu.push(0x1201d1f8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d202; continue; }
  pc = 0x1201d1fc; continue;
 }
 0x1201d1fc => {
  pc = cpu.r32(0x120411ccu32); continue;
 }
 0x1201d202 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d205(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d205); }
 let mut pc: u32 = 0x1201d205;
 loop { match pc {
 0x1201d205 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d21b); f_1201d2a3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d21f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d21f); }
 let mut pc: u32 = 0x1201d21f;
 loop { match pc {
 0x1201d21f => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x1201d223; continue;
 }
 0x1201d223 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201d223; continue; }
  pc = 0x1201d22d; continue;
 }
 0x1201d22d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d244); f_1201d2a3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d248(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d248); }
 let mut pc: u32 = 0x1201d248;
 loop { match pc {
 0x1201d248 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d25c); f_1201d535(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d260); }
 let mut pc: u32 = 0x1201d260;
 loop { match pc {
 0x1201d260 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x1201d264; continue;
 }
 0x1201d264 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201d264; continue; }
  pc = 0x1201d26e; continue;
 }
 0x1201d26e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d285); f_1201d54f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d289(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d289); }
 let mut pc: u32 = 0x1201d289;
 loop { match pc {
 0x1201d289 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d29f); f_1201d2a3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d2a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d2a3); }
 let mut pc: u32 = 0x1201d2a3;
 loop { match pc {
 0x1201d2a3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d2ba); f_12005b48(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d2cb); f_12005b48(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d3e2; continue; }
  pc = 0x1201d2d6; continue;
 }
 0x1201d2d6 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201d378; continue; }
  pc = 0x1201d2de; continue;
 }
 0x1201d2de => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x3au32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d2ea); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201d378; continue; }
  pc = 0x1201d2f7; continue;
 }
 0x1201d2f7 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d30f); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d327; continue; }
  pc = 0x1201d31a; continue;
 }
 0x1201d31a => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d327 => {
  { let v: u16 = cpu.r16(0x12036620u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x12036620u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x1201d378; continue;
 }
 0x1201d378 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v = 0x2eu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d388); f_12005b48(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201d3ee; continue; }
  pc = 0x1201d38f; continue;
 }
 0x1201d38f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d39b); f_1201a801(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201d3c3; continue; }
  pc = 0x1201d3a3; continue;
 }
 0x1201d3a3 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d3bc); f_1201d4c6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1201d3c3; continue;
 }
 0x1201d3c3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x24u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1201d3d6; continue; }
  pc = 0x1201d3cd; continue;
 }
 0x1201d3cd => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d3d3); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201d3d6; continue;
 }
 0x1201d3d6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d3e2 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1201d3ea; continue; }
  pc = 0x1201d3e6; continue;
 }
 0x1201d3e6 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201d378; continue; }
  pc = 0x1201d3ea; continue;
 }
 0x1201d3ea => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x1201d378; continue;
 }
 0x1201d3ee => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d406); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1201d41c; continue; }
  pc = 0x1201d40f; continue;
 }
 0x1201d40f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d41c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1203d09cu32; cpu.ebp = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201d45b; continue;
 }
 0x1201d45b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x1201d487); f_1201a801(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201d4a8; continue; }
  pc = 0x1201d48f; continue;
 }
 0x1201d48f => {
  { let v: u32 = cpu.ebp.wrapping_sub(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1203d090u32); }
  if !cpu.cf { pc = 0x1201d45b; continue; }
  pc = 0x1201d49a; continue;
 }
 0x1201d49a => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201d4a0); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201d3c3; continue;
 }
 0x1201d4a8 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d4bd); f_1201d4c6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1201d49a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d4c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d4c6); }
 let mut pc: u32 = 0x1201d4c6;
 loop { match pc {
 0x1201d4c6 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d4e8); f_1201e394(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201d4fa; continue; }
  pc = 0x1201d4f0; continue;
 }
 0x1201d4f0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d4fa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d50f); f_1201e11c(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d51e); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d52b); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d535(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d535); }
 let mut pc: u32 = 0x1201d535;
 loop { match pc {
 0x1201d535 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d54b); f_1201d54f(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d54f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d54f); }
 let mut pc: u32 = 0x1201d54f;
 loop { match pc {
 0x1201d54f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d571); f_1201d2a3(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201d72b; continue; }
  pc = 0x1201d581; continue;
 }
 0x1201d581 => {
  cpu.push(0x1201d586); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if !cpu.zf { pc = 0x1201d72b; continue; }
  pc = 0x1201d58f; continue;
 }
 0x1201d58f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d59b); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d72b; continue; }
  pc = 0x1201d5a6; continue;
 }
 0x1201d5a6 => {
  { let v = 0x12036628u32; cpu.push(v); }
  cpu.push(0x1201d5b0); f_12018768(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x1201d72b; continue; }
  pc = 0x1201d5bd; continue;
 }
 0x1201d5bd => {
  { let v = 0x104u32; cpu.push(v); }
  cpu.push(0x1201d5c7); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201d738; continue; }
  pc = 0x1201d5d4; continue;
 }
 0x1201d5d4 => {
  { let v = 0x103u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201d5e0); f_12024d45(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201d72b; continue; }
  pc = 0x1201d5ed; continue;
 }
 0x1201d5ed => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x1201d72b; continue; }
  pc = 0x1201d5f6; continue;
 }
 0x1201d5f6 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x1201d64e; continue; }
  pc = 0x1201d60d; continue;
 }
 0x1201d60d => {
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201d615); f_12005b48(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x1201d652; continue; }
  pc = 0x1201d61c; continue;
 }
 0x1201d61c => {
  { let v: u32 = 0x12036624u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x1201d652; continue;
 }
 0x1201d64e => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x1201d61c; continue; }
  pc = 0x1201d652; continue;
 }
 0x1201d652 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x104u32); }
  if !cpu.cf { pc = 0x1201d72b; continue; }
  pc = 0x1201d680; continue;
 }
 0x1201d680 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d6c4); f_1201d2a3(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201d72b; continue; }
  pc = 0x1201d6d0; continue;
 }
 0x1201d6d0 => {
  cpu.push(0x1201d6d5); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if cpu.zf { pc = 0x1201d5d4; continue; }
  pc = 0x1201d6de; continue;
 }
 0x1201d6de => {
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201d6e6); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x1201d6fc; continue; }
  pc = 0x1201d6ed; continue;
 }
 0x1201d6ed => {
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201d6f5); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x1201d72b; continue; }
  pc = 0x1201d6fc; continue;
 }
 0x1201d6fc => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d707); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1201d5d4; continue; }
  pc = 0x1201d715; continue;
 }
 0x1201d715 => {
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d71d); f_1200236d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1201d5d4; continue; }
  pc = 0x1201d72b; continue;
 }
 0x1201d72b => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201d738; continue; }
  pc = 0x1201d72f; continue;
 }
 0x1201d72f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201d735); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201d738; continue;
 }
 0x1201d738 => {
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
pub fn f_1201d744(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d744); }
 let mut pc: u32 = 0x1201d744;
 loop { match pc {
 0x1201d744 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d755); f_1201d7c9(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d759(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d759); }
 let mut pc: u32 = 0x1201d759;
 loop { match pc {
 0x1201d759 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x1201d75d; continue;
 }
 0x1201d75d => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201d75d; continue; }
  pc = 0x1201d767; continue;
 }
 0x1201d767 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d779); f_1201d7c9(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d77d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d77d); }
 let mut pc: u32 = 0x1201d77d;
 loop { match pc {
 0x1201d77d => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d78c); f_1201d9ca(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d790); }
 let mut pc: u32 = 0x1201d790;
 loop { match pc {
 0x1201d790 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x1201d794; continue;
 }
 0x1201d794 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201d794; continue; }
  pc = 0x1201d79e; continue;
 }
 0x1201d79e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201d7b0); f_1201d9df(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d7b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d7b4); }
 let mut pc: u32 = 0x1201d7b4;
 loop { match pc {
 0x1201d7b4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d7c5); f_1201d7c9(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d7c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d7c9); }
 let mut pc: u32 = 0x1201d7c9;
 loop { match pc {
 0x1201d7c9 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d7e0); f_1202e1f5(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d7f1); f_1202e1f5(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201d8bd; continue; }
  pc = 0x1201d7fc; continue;
 }
 0x1201d7fc => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201d864; continue; }
  pc = 0x1201d800; continue;
 }
 0x1201d800 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x3au32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d80c); f_120158ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201d864; continue; }
  pc = 0x1201d815; continue;
 }
 0x1201d815 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d81f); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d82f); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201d845; continue; }
  pc = 0x1201d838; continue;
 }
 0x1201d838 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d845 => {
  { let v = 0x12036630u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d850); f_120154c4(cpu);
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d861); f_12015493(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201d864; continue;
 }
 0x1201d864 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = 0x2eu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201d874); f_1202e1f5(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201d8c9; continue; }
  pc = 0x1201d87b; continue;
 }
 0x1201d87b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d883); f_1201b6ca(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201d8a2; continue; }
  pc = 0x1201d88b; continue;
 }
 0x1201d88b => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d89b); f_1201d95e(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1201d8a2; continue;
 }
 0x1201d8a2 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), cpu.esi); }
  if cpu.zf { pc = 0x1201d8b1; continue; }
  pc = 0x1201d8a8; continue;
 }
 0x1201d8a8 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d8ae); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201d8b1; continue;
 }
 0x1201d8b1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d8bd => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201d8c5; continue; }
  pc = 0x1201d8c1; continue;
 }
 0x1201d8c1 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201d864; continue; }
  pc = 0x1201d8c5; continue;
 }
 0x1201d8c5 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x1201d864; continue;
 }
 0x1201d8c9 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d8cf); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xau32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d8df); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201d8f5; continue; }
  pc = 0x1201d8e8; continue;
 }
 0x1201d8e8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d8f5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x1203d0acu32; cpu.ebx = v; }
  cpu.push(0x1201d901); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d90a); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ebp = v; }
  pc = 0x1201d910; continue;
 }
 0x1201d910 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201d919); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201d924); f_1201b6ca(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201d945; continue; }
  pc = 0x1201d92c; continue;
 }
 0x1201d92c => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1203d0a0u32); }
  if !cpu.cf { pc = 0x1201d910; continue; }
  pc = 0x1201d937; continue;
 }
 0x1201d937 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201d93d); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201d8a2; continue;
 }
 0x1201d945 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201d955); f_1201d95e(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1201d937; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d95e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d95e); }
 let mut pc: u32 = 0x1201d95e;
 loop { match pc {
 0x1201d95e => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d980); f_1201e9c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201d992; continue; }
  pc = 0x1201d988; continue;
 }
 0x1201d988 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201d992 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x1201d9a4); f_1201e72e(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d9b3); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201d9c0); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d9ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d9ca); }
 let mut pc: u32 = 0x1201d9ca;
 loop { match pc {
 0x1201d9ca => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201d9db); f_1201d9df(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201d9df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201d9df); }
 let mut pc: u32 = 0x1201d9df;
 loop { match pc {
 0x1201d9df => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201d9fc); f_1201d7c9(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.push(0x1201da04); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if !cpu.zf { pc = 0x1201db41; continue; }
  pc = 0x1201da0d; continue;
 }
 0x1201da0d => {
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201da15); f_120158ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201db41; continue; }
  pc = 0x1201da20; continue;
 }
 0x1201da20 => {
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201da28); f_120158ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201db41; continue; }
  pc = 0x1201da33; continue;
 }
 0x1201da33 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1201da43; continue; }
  pc = 0x1201da38; continue;
 }
 0x1201da38 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), 0x3au16); }
  if cpu.zf { pc = 0x1201db41; continue; }
  pc = 0x1201da43; continue;
 }
 0x1201da43 => {
  { let v = 0x1203663cu32; cpu.push(v); }
  cpu.push(0x1201da4d); f_12024199(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201db41; continue; }
  pc = 0x1201da5a; continue;
 }
 0x1201da5a => {
  { let v = 0x208u32; cpu.push(v); }
  cpu.push(0x1201da64); f_12004858(cpu);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201db4e; continue; }
  pc = 0x1201da73; continue;
 }
 0x1201da73 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if cpu.zf { pc = 0x1201da9c; continue; }
  pc = 0x1201da79; continue;
 }
 0x1201da79 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3bu16); }
  if cpu.zf { pc = 0x1201da9c; continue; }
  pc = 0x1201da82; continue;
 }
 0x1201da82 => {
  { let v: u32 = 0x408u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.cf || cpu.zf) { pc = 0x1201da9c; continue; }
  pc = 0x1201da8c; continue;
 }
 0x1201da8c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x1201da79; continue; }
  pc = 0x1201da9c; continue;
 }
 0x1201da9c => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201dac4; continue; }
  pc = 0x1201dab0; continue;
 }
 0x1201dab0 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if cpu.zf { pc = 0x1201dac4; continue; }
  pc = 0x1201dab6; continue;
 }
 0x1201dab6 => {
  { let v = 0x12036638u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201dac1); f_12015493(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201dac4; continue;
 }
 0x1201dac4 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201daca); f_12003c98(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201dad7); f_12003c98(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)).wrapping_add(cpu.eax); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x104u32); }
  if !cpu.cf { pc = 0x1201db41; continue; }
  pc = 0x1201dae8; continue;
 }
 0x1201dae8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201daef); f_12015493(cpu);
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201db02); f_1201d7c9(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.push(0x1201db0a); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if cpu.zf { pc = 0x1201db2e; continue; }
  pc = 0x1201db0f; continue;
 }
 0x1201db0f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201db1e; continue; }
  pc = 0x1201db18; continue;
 }
 0x1201db18 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201db41; continue; }
  pc = 0x1201db1e; continue;
 }
 0x1201db1e => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201db2e; continue; }
  pc = 0x1201db28; continue;
 }
 0x1201db28 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201db41; continue; }
  pc = 0x1201db2e; continue;
 }
 0x1201db2e => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if cpu.zf { pc = 0x1201db41; continue; }
  pc = 0x1201db34; continue;
 }
 0x1201db34 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201da73; continue; }
  pc = 0x1201db41; continue;
 }
 0x1201db41 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201db4e; continue; }
  pc = 0x1201db45; continue;
 }
 0x1201db45 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201db4b); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201db4e; continue;
 }
 0x1201db4e => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
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
pub fn f_1201db5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201db5b); }
 let mut pc: u32 = 0x1201db5b;
 loop { match pc {
 0x1201db5b => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201db71); f_1201dbf9(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201db75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201db75); }
 let mut pc: u32 = 0x1201db75;
 loop { match pc {
 0x1201db75 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x1201db79; continue;
 }
 0x1201db79 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201db79; continue; }
  pc = 0x1201db83; continue;
 }
 0x1201db83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201db9a); f_1201dbf9(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201db9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201db9e); }
 let mut pc: u32 = 0x1201db9e;
 loop { match pc {
 0x1201db9e => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201dbb2); f_1201de07(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201dbb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201dbb6); }
 let mut pc: u32 = 0x1201dbb6;
 loop { match pc {
 0x1201dbb6 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x1201dbba; continue;
 }
 0x1201dbba => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201dbba; continue; }
  pc = 0x1201dbc4; continue;
 }
 0x1201dbc4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201dbdb); f_1201de21(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201dbdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201dbdf); }
 let mut pc: u32 = 0x1201dbdf;
 loop { match pc {
 0x1201dbdf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201dbf5); f_1201dbf9(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201dbf9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201dbf9); }
 let mut pc: u32 = 0x1201dbf9;
 loop { match pc {
 0x1201dbf9 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x5cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201dc10); f_1202e1f5(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201dc21); f_1202e1f5(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201dcf2; continue; }
  pc = 0x1201dc2c; continue;
 }
 0x1201dc2c => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201dc94; continue; }
  pc = 0x1201dc30; continue;
 }
 0x1201dc30 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x3au32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201dc3c); f_120158ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201dc94; continue; }
  pc = 0x1201dc45; continue;
 }
 0x1201dc45 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201dc4f); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201dc5f); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201dc75; continue; }
  pc = 0x1201dc68; continue;
 }
 0x1201dc68 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201dc75 => {
  { let v = 0x12036630u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201dc80); f_120154c4(cpu);
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201dc91); f_12015493(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201dc94; continue;
 }
 0x1201dc94 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = 0x2eu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201dca4); f_1202e1f5(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201dcfe; continue; }
  pc = 0x1201dcab; continue;
 }
 0x1201dcab => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201dcb3); f_1201b6ca(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201dcd7; continue; }
  pc = 0x1201dcbb; continue;
 }
 0x1201dcbb => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201dcd0); f_1201dd98(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1201dcd7; continue;
 }
 0x1201dcd7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1cu32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1201dce6; continue; }
  pc = 0x1201dcdd; continue;
 }
 0x1201dcdd => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201dce3); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201dce6; continue;
 }
 0x1201dce6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201dcf2 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201dcfa; continue; }
  pc = 0x1201dcf6; continue;
 }
 0x1201dcf6 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201dc94; continue; }
  pc = 0x1201dcfa; continue;
 }
 0x1201dcfa => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x1201dc94; continue;
 }
 0x1201dcfe => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201dd04); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xau32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201dd14); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201dd2a; continue; }
  pc = 0x1201dd1d; continue;
 }
 0x1201dd1d => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201dd2a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x1203d0bcu32; cpu.ebx = v; }
  cpu.push(0x1201dd36); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201dd3f); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ebp = v; }
  pc = 0x1201dd45; continue;
 }
 0x1201dd45 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201dd4e); f_120154c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201dd59); f_1201b6ca(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201dd7a; continue; }
  pc = 0x1201dd61; continue;
 }
 0x1201dd61 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1203d0b0u32); }
  if !cpu.cf { pc = 0x1201dd45; continue; }
  pc = 0x1201dd6c; continue;
 }
 0x1201dd6c => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201dd72); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201dcd7; continue;
 }
 0x1201dd7a => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201dd8f); f_1201dd98(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1201dd6c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201dd98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201dd98); }
 let mut pc: u32 = 0x1201dd98;
 loop { match pc {
 0x1201dd98 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ddba); f_1201e9c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201ddcc; continue; }
  pc = 0x1201ddc2; continue;
 }
 0x1201ddc2 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ddcc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201dde1); f_1201e72e(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201ddf0); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ddfd); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201de07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201de07); }
 let mut pc: u32 = 0x1201de07;
 loop { match pc {
 0x1201de07 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201de1d); f_1201de21(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201de21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201de21); }
 let mut pc: u32 = 0x1201de21;
 loop { match pc {
 0x1201de21 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201de43); f_1201dbf9(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201df6d; continue; }
  pc = 0x1201de53; continue;
 }
 0x1201de53 => {
  cpu.push(0x1201de58); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if !cpu.zf { pc = 0x1201df6d; continue; }
  pc = 0x1201de61; continue;
 }
 0x1201de61 => {
  { let v = 0x2fu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201de69); f_120158ad(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201df6d; continue; }
  pc = 0x1201de74; continue;
 }
 0x1201de74 => {
  { let v = 0x1203663cu32; cpu.push(v); }
  cpu.push(0x1201de7e); f_12024199(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x1201df6d; continue; }
  pc = 0x1201de8b; continue;
 }
 0x1201de8b => {
  { let v = 0x208u32; cpu.push(v); }
  cpu.push(0x1201de95); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201df7a; continue; }
  pc = 0x1201dea2; continue;
 }
 0x1201dea2 => {
  { let v = 0x103u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201deae); f_12024dc9(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1201df6d; continue; }
  pc = 0x1201debb; continue;
 }
 0x1201debb => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if cpu.zf { pc = 0x1201df6d; continue; }
  pc = 0x1201dec5; continue;
 }
 0x1201dec5 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201decb); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201deef; continue; }
  pc = 0x1201dedb; continue;
 }
 0x1201dedb => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if cpu.zf { pc = 0x1201deef; continue; }
  pc = 0x1201dee1; continue;
 }
 0x1201dee1 => {
  { let v = 0x12036638u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201deec); f_12015493(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201deef; continue;
 }
 0x1201deef => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201def5); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201df00); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x104u32); }
  if !cpu.cf { pc = 0x1201df6d; continue; }
  pc = 0x1201df0d; continue;
 }
 0x1201df0d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201df14); f_12015493(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1201df2c); f_1201dbf9(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201df6d; continue; }
  pc = 0x1201df38; continue;
 }
 0x1201df38 => {
  cpu.push(0x1201df3d); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if cpu.zf { pc = 0x1201dea2; continue; }
  pc = 0x1201df46; continue;
 }
 0x1201df46 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201df55; continue; }
  pc = 0x1201df4f; continue;
 }
 0x1201df4f => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if !cpu.zf { pc = 0x1201df6d; continue; }
  pc = 0x1201df55; continue;
 }
 0x1201df55 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5cu16); }
  if cpu.zf { pc = 0x1201dea2; continue; }
  pc = 0x1201df63; continue;
 }
 0x1201df63 => {
  { cpu.sub16((cpu.eax as u16), 0x2fu16); }
  if cpu.zf { pc = 0x1201dea2; continue; }
  pc = 0x1201df6d; continue;
 }
 0x1201df6d => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201df7a; continue; }
  pc = 0x1201df71; continue;
 }
 0x1201df71 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201df77); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201df7a; continue;
 }
 0x1201df7a => {
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
pub fn f_1201df86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201df86); }
 let mut pc: u32 = 0x1201df86;
 loop { match pc {
 0x1201df86 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x12036678u32; cpu.push(v); }
  cpu.push(0x1201df94); f_12024199(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201dfcc; continue; }
  pc = 0x1201dfa3; continue;
 }
 0x1201dfa3 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201dfb1; continue; }
  pc = 0x1201dfaa; continue;
 }
 0x1201dfaa => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201dfb1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201dfbd); f_1201b6ca(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201dfcc => {
  { let v: u32 = 0x12036670u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x1201e01f; continue; }
  pc = 0x1201dfe7; continue;
 }
 0x1201dfe7 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1201dffa); f_1201dbf9(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201e018; continue; }
  pc = 0x1201e004; continue;
 }
 0x1201e004 => {
  cpu.push(0x1201e009); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if cpu.zf { pc = 0x1201e01f; continue; }
  pc = 0x1201e00e; continue;
 }
 0x1201e00e => {
  cpu.push(0x1201e013); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0xdu32); }
  if cpu.zf { pc = 0x1201e01f; continue; }
  pc = 0x1201e018; continue;
 }
 0x1201e018 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e01f => {
  { cpu.and8(cpu.r8(0x1203b105u32), 0x80u8); }
  { let v: u32 = 0x12036658u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201e032; continue; }
  pc = 0x1201e02d; continue;
 }
 0x1201e02d => {
  { let v: u32 = 0x12036648u32; cpu.eax = v; }
  pc = 0x1201e032; continue;
 }
 0x1201e032 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1201e049); f_1201de21(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201e051(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201e051); }
 let mut pc: u32 = 0x1201e051;
 loop { match pc {
 0x1201e051 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x120366a0u32; cpu.push(v); }
  cpu.push(0x1201e05f); f_12018768(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201e097; continue; }
  pc = 0x1201e06e; continue;
 }
 0x1201e06e => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e07c; continue; }
  pc = 0x1201e075; continue;
 }
 0x1201e075 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e07c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e088); f_1201a801(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e097 => {
  { let v: u32 = 0x1203669cu32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x1201e0ea; continue; }
  pc = 0x1201e0b2; continue;
 }
 0x1201e0b2 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1201e0c5); f_1201d2a3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201e0e3; continue; }
  pc = 0x1201e0cf; continue;
 }
 0x1201e0cf => {
  cpu.push(0x1201e0d4); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x2u32); }
  if cpu.zf { pc = 0x1201e0ea; continue; }
  pc = 0x1201e0d9; continue;
 }
 0x1201e0d9 => {
  cpu.push(0x1201e0de); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0xdu32); }
  if cpu.zf { pc = 0x1201e0ea; continue; }
  pc = 0x1201e0e3; continue;
 }
 0x1201e0e3 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e0ea => {
  { cpu.and8(cpu.r8(0x1203b105u32), 0x80u8); }
  { let v: u32 = 0x12036690u32; cpu.eax = v; }
  if !cpu.zf { pc = 0x1201e0fd; continue; }
  pc = 0x1201e0f8; continue;
 }
 0x1201e0f8 => {
  { let v: u32 = 0x12036688u32; cpu.eax = v; }
  pc = 0x1201e0fd; continue;
 }
 0x1201e0fd => {
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1201e114); f_1201d54f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201e11c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201e11c); }
 let mut pc: u32 = 0x1201e11c;
 loop { match pc {
 0x1201e11c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x64u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e13d; continue; }
  pc = 0x1201e136; continue;
 }
 0x1201e136 => {
  pc = cpu.r32(0x1201e380u32.wrapping_add(cpu.esi.wrapping_mul(4))); continue;
 }
 0x1201e13d => {
  cpu.push(0x1201e142); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e14d); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e160 => {
  { let v: u8 = 0x1u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e165; continue;
 }
 0x1201e165 => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e170; continue;
 }
 0x1201e170 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x1201e18b; continue; }
  pc = 0x1201e176; continue;
 }
 0x1201e176 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1201e176; continue; }
  pc = 0x1201e17c; continue;
 }
 0x1201e17c => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x1201e170; continue; }
  pc = 0x1201e184; continue;
 }
 0x1201e184 => {
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x1201e170; continue;
 }
 0x1201e18b => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x44u32; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203f010u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if cpu.zf { pc = 0x1201e1d4; continue; }
  pc = 0x1201e1aa; continue;
 }
 0x1201e1aa => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x1201e1ac; continue;
 }
 0x1201e1ac => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1))), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x1201e1d4; continue; }
  pc = 0x1201e1ce; continue;
 }
 0x1201e1ce => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1201e1ac; continue; }
  pc = 0x1201e1d4; continue;
 }
 0x1201e1d4 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x66u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x66u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e1f2); f_12002393(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201e247; continue; }
  pc = 0x1201e210; continue;
 }
 0x1201e210 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp & 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(8)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if !cpu.zf { pc = 0x1201e2fe; continue; }
  pc = 0x1201e237; continue;
 }
 0x1201e237 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201e23e; continue;
 }
 0x1201e23e => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201e210; continue; }
  pc = 0x1201e247; continue;
 }
 0x1201e247 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esp)), 0x0u8); }
  if cpu.zf { pc = 0x1201e283; continue; }
  pc = 0x1201e24e; continue;
 }
 0x1201e24e => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x1201e267; continue; }
  pc = 0x1201e262; continue;
 }
 0x1201e262 => {
  { let v: u32 = 0x3u32; cpu.edi = v; }
  pc = 0x1201e267; continue;
 }
 0x1201e267 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201e27b; continue; }
  pc = 0x1201e26b; continue;
 }
 0x1201e26b => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x1201e26b; continue; }
  pc = 0x1201e27b; continue;
 }
 0x1201e27b => {
  { let v: u32 = 0x8u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e283; continue;
 }
 0x1201e283 => {
  cpu.push(0x1201e288); f_120133bd(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  cpu.push(0x1201e28f); f_120133b4(cpu);
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x1204106cu32); cpu.push(0x1201e2cd); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201e2d5); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e2e1); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201e30c; continue; }
  pc = 0x1201e2e8; continue;
 }
 0x1201e2e8 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201e2ee); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e2fe => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201e23e; continue;
 }
 0x1201e30c => {
  { cpu.sub32(cpu.esi, 0x2u32); }
  if !cpu.zf { pc = 0x1201e332; continue; }
  pc = 0x1201e311; continue;
 }
 0x1201e311 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1201e318); f_120123bf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201e31b; continue;
 }
 0x1201e31b => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x1201e326); cpu.call(t);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e332 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201e360; continue; }
  pc = 0x1201e336; continue;
 }
 0x1201e336 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041074u32); cpu.push(0x1201e343); cpu.call(t);
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041078u32); cpu.push(0x1201e353); cpu.call(t);
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x1201e35e); cpu.call(t);
  pc = 0x1201e31b; continue;
 }
 0x1201e360 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if !cpu.zf { pc = 0x1201e37a; continue; }
  pc = 0x1201e369; continue;
 }
 0x1201e369 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x1201e370); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e31b; continue;
 }
 0x1201e37a => {
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e31b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201e380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201e380); }
 let mut pc: u32 = 0x1201e380;
 loop { match pc {
 0x1201e380 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x1201e384; continue; }
  pc = 0x1201e383; continue;
 }
 0x1201e383 => {
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1201e388; continue;
 }
 0x1201e384 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x1201e388; continue; }
  pc = 0x1201e387; continue;
 }
 0x1201e387 => {
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1201e38c; continue;
 }
 0x1201e388 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x1201e38c; continue; }
  pc = 0x1201e38b; continue;
 }
 0x1201e38b => {
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1201e390; continue;
 }
 0x1201e38c => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x1201e390; continue; }
  pc = 0x1201e38f; continue;
 }
 0x1201e38f => {
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0xffffffe1u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  f_1201e394(cpu); return;
 }
 0x1201e390 => {
  { cpu.pushad(); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { f_1201e394(cpu); return; }
  pc = 0x1201e393; continue;
 }
 0x1201e393 => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x56530cecu32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x1201e3ca; continue; }
  pc = 0x1201e3ab; continue;
 }
 0x1201e3ab => {
  { cpu.sub32(cpu.esi, 0x7fffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e3ca; continue; }
  pc = 0x1201e3b3; continue;
 }
 0x1201e3b3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e3ab; continue; }
  pc = 0x1201e3ca; continue;
 }
 0x1201e3ca => {
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.esi, 0x7fffu32); }
  if cpu.cf { pc = 0x1201e403; continue; }
  pc = 0x1201e3d6; continue;
 }
 0x1201e3d6 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201e3e5); f_120133b4(cpu);
  { let v: u32 = 0x7u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e3f0); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xau32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e403 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201e409); f_12004858(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x1201e443; continue; }
  pc = 0x1201e416; continue;
 }
 0x1201e416 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201e425); f_120133b4(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e430); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e443 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201e47f; continue; }
  pc = 0x1201e44b; continue;
 }
 0x1201e44b => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1201e477; continue; }
  pc = 0x1201e458; continue;
 }
 0x1201e458 => {
  { cpu.sub32(cpu.esi, 0x7fffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e477; continue; }
  pc = 0x1201e460; continue;
 }
 0x1201e460 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e458; continue; }
  pc = 0x1201e477; continue;
 }
 0x1201e477 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x1201e588; continue; }
  pc = 0x1201e47f; continue;
 }
 0x1201e47f => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1201e489; continue;
 }
 0x1201e489 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  if !cpu.zf { pc = 0x1201e6f8; continue; }
  pc = 0x1201e499; continue;
 }
 0x1201e499 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1201e49a; continue;
 }
 0x1201e49a => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x1201e4dd; continue; }
  pc = 0x1201e49f; continue;
 }
 0x1201e49f => {
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e49f; continue; }
  pc = 0x1201e4dd; continue;
 }
 0x1201e4dd => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  { let v: u8 = 0x0u8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  if cpu.zf { pc = 0x1201e553; continue; }
  pc = 0x1201e4ee; continue;
 }
 0x1201e4ee => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1201e553; continue; }
  pc = 0x1201e518; continue;
 }
 0x1201e518 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e518; continue; }
  pc = 0x1201e553; continue;
 }
 0x1201e553 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1201e566; continue; }
  pc = 0x1201e557; continue;
 }
 0x1201e557 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if !cpu.zf { pc = 0x1201e563; continue; }
  pc = 0x1201e55f; continue;
 }
 0x1201e55f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1201e563; continue;
 }
 0x1201e563 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1201e566; continue;
 }
 0x1201e566 => {
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e571); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1203b1e8u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e588 => {
  { cpu.sub32(cpu.r32(0x1203b1e8u32), 0x0u32); }
  if !cpu.zf { pc = 0x1201e5ac; continue; }
  pc = 0x1201e591; continue;
 }
 0x1201e591 => {
  cpu.push(0x1201e596); f_1200414b(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1203b1e8u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201e5ac; continue; }
  pc = 0x1201e59f; continue;
 }
 0x1201e59f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e5ac => {
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1201e5eb; continue; }
  pc = 0x1201e5be; continue;
 }
 0x1201e5be => {
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x3du8); }
  if cpu.zf { pc = 0x1201e5eb; continue; }
  pc = 0x1201e5cd; continue;
 }
 0x1201e5cd => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)).wrapping_add(cpu.ecx); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x1201e5be; continue; }
  pc = 0x1201e5eb; continue;
 }
 0x1201e5eb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))), 0x3du8); }
  if !cpu.zf { pc = 0x1201e641; continue; }
  pc = 0x1201e5fe; continue;
 }
 0x1201e5fe => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1201e600; continue;
 }
 0x1201e600 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x1203b1e8u32)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1201e641; continue; }
  pc = 0x1201e60f; continue;
 }
 0x1201e60f => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.edi)), 0x3au8); }
  if !cpu.zf { pc = 0x1201e641; continue; }
  pc = 0x1201e615; continue;
 }
 0x1201e615 => {
  { cpu.sub8(cpu.r8(0x3u32.wrapping_add(cpu.edi)), 0x3du8); }
  if !cpu.zf { pc = 0x1201e641; continue; }
  pc = 0x1201e61b; continue;
 }
 0x1201e61b => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x3du8); }
  if cpu.zf { pc = 0x1201e600; continue; }
  pc = 0x1201e641; continue;
 }
 0x1201e641 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ffeu32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201e6a5; continue; }
  pc = 0x1201e669; continue;
 }
 0x1201e669 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e675); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201e687); f_120133b4(cpu);
  { let v: u32 = 0x7u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e692); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xau32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e6a5 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201e6ab); f_12004858(cpu);
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x1201e489; continue; }
  pc = 0x1201e6bc; continue;
 }
 0x1201e6bc => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e6c8); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201e6da); f_120133b4(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e6e5); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e6f8 => {
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x1201e49a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201e394(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201e394); }
 let mut pc: u32 = 0x1201e394;
 loop { match pc {
 0x1201e394 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x1201e3ca; continue; }
  pc = 0x1201e3ab; continue;
 }
 0x1201e3ab => {
  { cpu.sub32(cpu.esi, 0x7fffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e3ca; continue; }
  pc = 0x1201e3b3; continue;
 }
 0x1201e3b3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e3ab; continue; }
  pc = 0x1201e3ca; continue;
 }
 0x1201e3ca => {
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.esi, 0x7fffu32); }
  if cpu.cf { pc = 0x1201e403; continue; }
  pc = 0x1201e3d6; continue;
 }
 0x1201e3d6 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201e3e5); f_120133b4(cpu);
  { let v: u32 = 0x7u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e3f0); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xau32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e403 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201e409); f_12004858(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x1201e443; continue; }
  pc = 0x1201e416; continue;
 }
 0x1201e416 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201e425); f_120133b4(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e430); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e443 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201e47f; continue; }
  pc = 0x1201e44b; continue;
 }
 0x1201e44b => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1201e477; continue; }
  pc = 0x1201e458; continue;
 }
 0x1201e458 => {
  { cpu.sub32(cpu.esi, 0x7fffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e477; continue; }
  pc = 0x1201e460; continue;
 }
 0x1201e460 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e458; continue; }
  pc = 0x1201e477; continue;
 }
 0x1201e477 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x1201e588; continue; }
  pc = 0x1201e47f; continue;
 }
 0x1201e47f => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1201e489; continue;
 }
 0x1201e489 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  if !cpu.zf { pc = 0x1201e6f8; continue; }
  pc = 0x1201e499; continue;
 }
 0x1201e499 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1201e49a; continue;
 }
 0x1201e49a => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x1201e4dd; continue; }
  pc = 0x1201e49f; continue;
 }
 0x1201e49f => {
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e49f; continue; }
  pc = 0x1201e4dd; continue;
 }
 0x1201e4dd => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  { let v: u8 = 0x0u8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  if cpu.zf { pc = 0x1201e553; continue; }
  pc = 0x1201e4ee; continue;
 }
 0x1201e4ee => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1201e553; continue; }
  pc = 0x1201e518; continue;
 }
 0x1201e518 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e518; continue; }
  pc = 0x1201e553; continue;
 }
 0x1201e553 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1201e566; continue; }
  pc = 0x1201e557; continue;
 }
 0x1201e557 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.edx); }
  if !cpu.zf { pc = 0x1201e563; continue; }
  pc = 0x1201e55f; continue;
 }
 0x1201e55f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1201e563; continue;
 }
 0x1201e563 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1201e566; continue;
 }
 0x1201e566 => {
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e571); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1203b1e8u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e588 => {
  { cpu.sub32(cpu.r32(0x1203b1e8u32), 0x0u32); }
  if !cpu.zf { pc = 0x1201e5ac; continue; }
  pc = 0x1201e591; continue;
 }
 0x1201e591 => {
  cpu.push(0x1201e596); f_1200414b(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1203b1e8u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201e5ac; continue; }
  pc = 0x1201e59f; continue;
 }
 0x1201e59f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e5ac => {
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1201e5eb; continue; }
  pc = 0x1201e5be; continue;
 }
 0x1201e5be => {
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x3du8); }
  if cpu.zf { pc = 0x1201e5eb; continue; }
  pc = 0x1201e5cd; continue;
 }
 0x1201e5cd => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)).wrapping_add(cpu.ecx); cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x1201e5be; continue; }
  pc = 0x1201e5eb; continue;
 }
 0x1201e5eb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))), 0x3du8); }
  if !cpu.zf { pc = 0x1201e641; continue; }
  pc = 0x1201e5fe; continue;
 }
 0x1201e5fe => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1201e600; continue;
 }
 0x1201e600 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x1203b1e8u32)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1201e641; continue; }
  pc = 0x1201e60f; continue;
 }
 0x1201e60f => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.edi)), 0x3au8); }
  if !cpu.zf { pc = 0x1201e641; continue; }
  pc = 0x1201e615; continue;
 }
 0x1201e615 => {
  { cpu.sub8(cpu.r8(0x3u32.wrapping_add(cpu.edi)), 0x3du8); }
  if !cpu.zf { pc = 0x1201e641; continue; }
  pc = 0x1201e61b; continue;
 }
 0x1201e61b => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203b1e8u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x3du8); }
  if cpu.zf { pc = 0x1201e600; continue; }
  pc = 0x1201e641; continue;
 }
 0x1201e641 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ffeu32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201e6a5; continue; }
  pc = 0x1201e669; continue;
 }
 0x1201e669 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e675); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201e687); f_120133b4(cpu);
  { let v: u32 = 0x7u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e692); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xau32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e6a5 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201e6ab); f_12004858(cpu);
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x1201e489; continue; }
  pc = 0x1201e6bc; continue;
 }
 0x1201e6bc => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e6c8); f_12005adb(cpu);
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201e6da); f_120133b4(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e6e5); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e6f8 => {
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x1201e49a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201e72e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201e72e); }
 let mut pc: u32 = 0x1201e72e;
 loop { match pc {
 0x1201e72e => {
  { let v: u32 = cpu.esp.wrapping_sub(0x64u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e74f; continue; }
  pc = 0x1201e748; continue;
 }
 0x1201e748 => {
  pc = cpu.r32(0x1201e9b0u32.wrapping_add(cpu.esi.wrapping_mul(4))); continue;
 }
 0x1201e74f => {
  cpu.push(0x1201e754); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201e75f); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e772 => {
  { let v: u8 = 0x1u8; cpu.w8(0x13u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e777; continue;
 }
 0x1201e777 => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e782; continue;
 }
 0x1201e782 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1201e7a5; continue; }
  pc = 0x1201e78a; continue;
 }
 0x1201e78a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ecx)), 0x0u16); }
  if !cpu.zf { pc = 0x1201e78a; continue; }
  pc = 0x1201e793; continue;
 }
 0x1201e793 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x1201e782; continue; }
  pc = 0x1201e79c; continue;
 }
 0x1201e79c => {
  { let v: u16 = 0x20u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1201e782; continue;
 }
 0x1201e7a5 => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x44u32; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203f010u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if cpu.zf { pc = 0x1201e7ee; continue; }
  pc = 0x1201e7c4; continue;
 }
 0x1201e7c4 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x1201e7c6; continue;
 }
 0x1201e7c6 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1))), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x1201e7ee; continue; }
  pc = 0x1201e7e8; continue;
 }
 0x1201e7e8 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1201e7c6; continue; }
  pc = 0x1201e7ee; continue;
 }
 0x1201e7ee => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x66u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x66u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e80c); f_12002393(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201e861; continue; }
  pc = 0x1201e82a; continue;
 }
 0x1201e82a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp & 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(8)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if !cpu.zf { pc = 0x1201e920; continue; }
  pc = 0x1201e851; continue;
 }
 0x1201e851 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201e858; continue;
 }
 0x1201e858 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201e82a; continue; }
  pc = 0x1201e861; continue;
 }
 0x1201e861 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esp)), 0x0u8); }
  if cpu.zf { pc = 0x1201e89d; continue; }
  pc = 0x1201e868; continue;
 }
 0x1201e868 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x1201e881; continue; }
  pc = 0x1201e87c; continue;
 }
 0x1201e87c => {
  { let v: u32 = 0x3u32; cpu.edi = v; }
  pc = 0x1201e881; continue;
 }
 0x1201e881 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201e895; continue; }
  pc = 0x1201e885; continue;
 }
 0x1201e885 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x1201e885; continue; }
  pc = 0x1201e895; continue;
 }
 0x1201e895 => {
  { let v: u32 = 0x8u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e89d; continue;
 }
 0x1201e89d => {
  cpu.push(0x1201e8a2); f_120133bd(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  cpu.push(0x1201e8a9); f_120133b4(cpu);
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x84u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)) | 0x400u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x12041068u32); cpu.push(0x1201e8ef); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201e8f7); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201e903); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1201e92e; continue; }
  pc = 0x1201e90a; continue;
 }
 0x1201e90a => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201e910); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x64u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201e920 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201e858; continue;
 }
 0x1201e92e => {
  { cpu.sub32(cpu.esi, 0x2u32); }
  if !cpu.zf { pc = 0x1201e954; continue; }
  pc = 0x1201e933; continue;
 }
 0x1201e933 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x1201e93a); f_120123bf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201e93d; continue;
 }
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
 0x1201e954 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1201e982; continue; }
  pc = 0x1201e958; continue;
 }
 0x1201e958 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041074u32); cpu.push(0x1201e965); cpu.call(t);
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041078u32); cpu.push(0x1201e975); cpu.call(t);
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x1201e980); cpu.call(t);
  pc = 0x1201e93d; continue;
 }
 0x1201e982 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if !cpu.zf { pc = 0x1201e99c; continue; }
  pc = 0x1201e98b; continue;
 }
 0x1201e98b => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x1201e992); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e93d; continue;
 }
 0x1201e99c => {
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201e93d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201e9c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201e9c4); }
 let mut pc: u32 = 0x1201e9c4;
 loop { match pc {
 0x1201e9c4 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if cpu.zf { pc = 0x1201e9fa; continue; }
  pc = 0x1201e9d9; continue;
 }
 0x1201e9d9 => {
  { cpu.sub32(cpu.esi, 0x7fffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201e9fa; continue; }
  pc = 0x1201e9e1; continue;
 }
 0x1201e9e1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201e9ee); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x1201e9d9; continue; }
  pc = 0x1201e9fa; continue;
 }
 0x1201e9fa => {
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.esi, 0x3fffu32); }
  if cpu.cf { pc = 0x1201ea33; continue; }
  pc = 0x1201ea06; continue;
 }
 0x1201ea06 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201ea15); f_120133b4(cpu);
  { let v: u32 = 0x7u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ea20); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xau32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ea33 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(2)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ea40); f_12004858(cpu);
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  if !cpu.zf { pc = 0x1201ea7a; continue; }
  pc = 0x1201ea4d; continue;
 }
 0x1201ea4d => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1201ea5c); f_120133b4(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ea67); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ea7a => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201eabb; continue; }
  pc = 0x1201ea81; continue;
 }
 0x1201ea81 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if cpu.zf { pc = 0x1201eab0; continue; }
  pc = 0x1201ea8f; continue;
 }
 0x1201ea8f => {
  { cpu.sub32(cpu.esi, 0x7fffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201eab0; continue; }
  pc = 0x1201ea97; continue;
 }
 0x1201ea97 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201eaa4); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x1201ea8f; continue; }
  pc = 0x1201eab0; continue;
 }
 0x1201eab0 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201ebba; continue; }
  pc = 0x1201eabb; continue;
 }
 0x1201eabb => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1201eacd; continue;
 }
 0x1201eacd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201ed04; continue; }
  pc = 0x1201eadd; continue;
 }
 0x1201eadd => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x1201eae0; continue;
 }
 0x1201eae0 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x1201eb11; continue; }
  pc = 0x1201eae5; continue;
 }
 0x1201eae5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201eaee); f_120154c4(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201eafe); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = 0x20u16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x1201eae5; continue; }
  pc = 0x1201eb11; continue;
 }
 0x1201eb11 => {
  { let v: u16 = 0x0u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  if cpu.zf { pc = 0x1201eb7f; continue; }
  pc = 0x1201eb29; continue;
 }
 0x1201eb29 => {
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edi); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1203b1ecu32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x1201eb7f; continue; }
  pc = 0x1201eb5a; continue;
 }
 0x1201eb5a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201eb63); f_120154c4(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201eb73); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1201eb5a; continue; }
  pc = 0x1201eb7f; continue;
 }
 0x1201eb7f => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1201eb98; continue; }
  pc = 0x1201eb83; continue;
 }
 0x1201eb83 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.ebx); }
  if !cpu.zf { pc = 0x1201eb93; continue; }
  pc = 0x1201eb8b; continue;
 }
 0x1201eb8b => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  pc = 0x1201eb93; continue;
 }
 0x1201eb93 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1201eb98; continue;
 }
 0x1201eb98 => {
  { let v: u32 = cpu.r32(0x1203b1ecu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201eba3); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1203b1ecu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ebba => {
  { cpu.sub32(cpu.r32(0x1203b1ecu32), 0x0u32); }
  if !cpu.zf { pc = 0x1201ebde; continue; }
  pc = 0x1201ebc3; continue;
 }
 0x1201ebc3 => {
  cpu.push(0x1201ebc8); f_1201443f(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1203b1ecu32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201ebde; continue; }
  pc = 0x1201ebd1; continue;
 }
 0x1201ebd1 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ebde => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203b1ecu32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x1201ec16; continue; }
  pc = 0x1201ebea; continue;
 }
 0x1201ebea => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1203b1ecu32)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x3du16); }
  if cpu.zf { pc = 0x1201ec16; continue; }
  pc = 0x1201ebfd; continue;
 }
 0x1201ebfd => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ec03); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1203b1ecu32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(2))), 0x0u16); }
  if !cpu.zf { pc = 0x1201ebea; continue; }
  pc = 0x1201ec16; continue;
 }
 0x1201ec16 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1203b1ecu32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(2))), 0x3du16); }
  if !cpu.zf { pc = 0x1201ec62; continue; }
  pc = 0x1201ec24; continue;
 }
 0x1201ec24 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1203b1ecu32)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x1201ec62; continue; }
  pc = 0x1201ec38; continue;
 }
 0x1201ec38 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.eax)), 0x3au16); }
  if !cpu.zf { pc = 0x1201ec62; continue; }
  pc = 0x1201ec3f; continue;
 }
 0x1201ec3f => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.eax)), 0x3du16); }
  if !cpu.zf { pc = 0x1201ec62; continue; }
  pc = 0x1201ec46; continue;
 }
 0x1201ec46 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ec4f); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1203b1ecu32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x3du16); }
  if cpu.zf { pc = 0x1201ec24; continue; }
  pc = 0x1201ec62; continue;
 }
 0x1201ec62 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ec70); f_12003c98(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3fffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201ecb7; continue; }
  pc = 0x1201ec83; continue;
 }
 0x1201ec83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ec8b); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x1201ec99); f_120133b4(cpu);
  { let v: u32 = 0x7u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201eca4); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xau32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ecb7 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201ecbf); f_12004858(cpu);
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  if !cpu.zf { pc = 0x1201eacd; continue; }
  pc = 0x1201ecd0; continue;
 }
 0x1201ecd0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ecd8); f_12005adb(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x1201ece6); f_120133b4(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ecf1); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ed04 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201ed0b); f_120154c4(cpu);
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201ed21); f_12003c98(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.esi = v; }
  pc = 0x1201eae0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ed2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ed2d); }
 let mut pc: u32 = 0x1201ed2d;
 loop { match pc {
 0x1201ed2d => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffe0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1201ed45; continue; }
  pc = 0x1201ed3c; continue;
 }
 0x1201ed3c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ed45 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1201edae; continue; }
  pc = 0x1201ed49; continue;
 }
 0x1201ed49 => {
  { let v: u32 = cpu.edi.wrapping_add(0xfu32); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xfffffff0u32; cpu.edi = v; }
  pc = 0x1201ed4f; continue;
 }
 0x1201ed4f => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x1201ed56); f_1200420b(cpu);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201ed6d); f_1200589a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201edb5; continue; }
  pc = 0x1201ed74; continue;
 }
 0x1201ed74 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1203bc5cu32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201ed9b; continue; }
  pc = 0x1201ed7e; continue;
 }
 0x1201ed7e => {
  { let v: u32 = cpu.shr32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201ed92); f_120021f8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201ed9b; continue; }
  pc = 0x1201ed99; continue;
 }
 0x1201ed99 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x1201ed9b; continue;
 }
 0x1201ed9b => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x1201eda2); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201edae => {
  { let v: u32 = 0x10u32; cpu.edi = v; }
  pc = 0x1201ed4f; continue;
 }
 0x1201edb5 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x1201edbc); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203ef08u32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041130u32); cpu.push(0x1201edcf); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201edd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201edd6); }
 let mut pc: u32 = 0x1201edd6;
 loop { match pc {
 0x1201edd6 => {
  cpu.push(0x1201eddb); f_120133b4(cpu);
  { let v: u32 = 0x28u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ede7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ede7); }
 let mut pc: u32 = 0x1201ede7;
 loop { match pc {
 0x1201ede7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v: u32 = 0xfffffffeu32; cpu.esi = v; }
  cpu.push(0x1201edf4); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x1201edfc); f_120136d5(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1201ee05; continue; }
  pc = 0x1201ee00; continue;
 }
 0x1201ee00 => {
  { let v: u32 = 0xfffffffcu32; cpu.esi = v; }
  pc = 0x1201ee05; continue;
 }
 0x1201ee05 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x1201ee0c); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203ef08u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041064u32); cpu.push(0x1201ee1f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201ee4d; continue; }
  pc = 0x1201ee23; continue;
 }
 0x1201ee23 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201ee29); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x78u32); }
  if !cpu.zf { pc = 0x1201ee48; continue; }
  pc = 0x1201ee2e; continue;
 }
 0x1201ee2e => {
  cpu.push(0x1201ee33); f_120133bd(cpu);
  { let v: u32 = 0x78u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ee3e); f_120133b4(cpu);
  { let v: u32 = 0x28u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ee48 => {
  { let v: u32 = 0xfffffffcu32; cpu.esi = v; }
  pc = 0x1201ee4d; continue;
 }
 0x1201ee4d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ee51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ee51); }
 let mut pc: u32 = 0x1201ee51;
 loop { match pc {
 0x1201ee51 => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x1201ee58); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x20u32; cpu.push(v); }
  cpu.push(0x1201ee62); f_12005a2b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x1201ee6c); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203ef08u32); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041060u32); cpu.push(0x1201ee7d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201eea8; continue; }
  pc = 0x1201ee81; continue;
 }
 0x1201ee81 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201ee87); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x78u32); }
  if !cpu.zf { pc = 0x1201eea2; continue; }
  pc = 0x1201ee8c; continue;
 }
 0x1201ee8c => {
  cpu.push(0x1201ee91); f_120133bd(cpu);
  { let v: u32 = 0x78u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ee9c); f_120133b4(cpu);
  { let v: u32 = 0x28u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201eea2; continue;
 }
 0x1201eea2 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201eea8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201eeab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201eeab); }
 let mut pc: u32 = 0x1201eeab;
 loop { match pc {
 0x1201eeab => {
  cpu.push(0x1201eeb0); f_120133b4(cpu);
  { let v: u32 = 0x28u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201eeb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201eeb9); }
 let mut pc: u32 = 0x1201eeb9;
 loop { match pc {
 0x1201eeb9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x120366a8u32; cpu.push(v); }
  { let v = 0x1201424cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v: u32 = 0xfffffffeu32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffcau32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffc9u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x1201ef52; continue; }
  pc = 0x1201eefb; continue;
 }
 0x1201eefb => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef08u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1204105cu32); cpu.push(0x1201ef0c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201f016; continue; }
  pc = 0x1201ef14; continue;
 }
 0x1201ef14 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201ef1a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x78u32); }
  if !cpu.zf { pc = 0x1201ef4b; continue; }
  pc = 0x1201ef1f; continue;
 }
 0x1201ef1f => {
  cpu.push(0x1201ef24); f_120133bd(cpu);
  { let v: u32 = 0x78u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201ef2f); f_120133b4(cpu);
  { let v: u32 = 0x28u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xfffffffbu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201ef3a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ef4b => {
  { let v: u32 = 0xfffffffdu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201ef52 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x1u32); }
  if !cpu.zf { pc = 0x1201ef7b; continue; }
  pc = 0x1201ef5b; continue;
 }
 0x1201ef5b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203ef08u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041064u32); cpu.push(0x1201ef6a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201ef75; continue; }
  pc = 0x1201ef6e; continue;
 }
 0x1201ef6e => {
  { let v: u32 = 0xfffffffcu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201ef75 => {
  { let v: u16 = 0x4u16; cpu.w16(0xffffffcau32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201ef7b; continue;
 }
 0x1201ef7b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203ef08u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204105cu32); cpu.push(0x1201ef93); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201ef9e; continue; }
  pc = 0x1201ef97; continue;
 }
 0x1201ef97 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201ef9e; continue;
 }
 0x1201ef9e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x1201f006; continue; }
  pc = 0x1201efab; continue;
 }
 0x1201efab => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201efb1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x103u32); }
  if !cpu.zf { pc = 0x1201efd7; continue; }
  pc = 0x1201efbb; continue;
 }
 0x1201efbb => {
  { let v: u32 = 0xfffffffbu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201efd7 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x78u32); }
  if !cpu.zf { pc = 0x1201effc; continue; }
  pc = 0x1201efdd; continue;
 }
 0x1201efdd => {
  cpu.push(0x1201efe2); f_120133bd(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201efec); f_120133b4(cpu);
  { let v: u32 = 0x28u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xfffffffbu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201effc => {
  { let v: u32 = 0xfffffffcu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201f006 => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if !cpu.zf { pc = 0x1201f016; continue; }
  pc = 0x1201f00c; continue;
 }
 0x1201f00c => {
  { let v: u32 = 0xfffffffcu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201f016 => {
  { cpu.and8(cpu.r8(0xffffffcau32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x1201ef7b; continue; }
  pc = 0x1201f020; continue;
 }
 0x1201f020 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0xffffffcau32.wrapping_add(cpu.ebp)), 0x4u8); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  if cpu.zf { pc = 0x1201f043; continue; }
  pc = 0x1201f034; continue;
 }
 0x1201f034 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1201f03b; continue;
 }
 0x1201f03b => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201f043 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201f03b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201efc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201efc5); }
 let mut pc: u32 = 0x1201efc5;
 loop { match pc {
 0x1201efc5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201efcb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201efcb); }
 let mut pc: u32 = 0x1201efcb;
 loop { match pc {
 0x1201ef3a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201ef7b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203ef08u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204105cu32); cpu.push(0x1201ef93); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201ef9e; continue; }
  pc = 0x1201ef97; continue;
 }
 0x1201ef97 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201ef9e; continue;
 }
 0x1201ef9e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x1201f006; continue; }
  pc = 0x1201efab; continue;
 }
 0x1201efab => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201efb1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x103u32); }
  if !cpu.zf { pc = 0x1201efd7; continue; }
  pc = 0x1201efbb; continue;
 }
 0x1201efbb => {
  { let v: u32 = 0xfffffffbu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201efcb => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201ef9e; continue;
 }
 0x1201efd7 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x78u32); }
  if !cpu.zf { pc = 0x1201effc; continue; }
  pc = 0x1201efdd; continue;
 }
 0x1201efdd => {
  cpu.push(0x1201efe2); f_120133bd(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201efec); f_120133b4(cpu);
  { let v: u32 = 0x28u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xfffffffbu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201effc => {
  { let v: u32 = 0xfffffffcu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201f006 => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if !cpu.zf { pc = 0x1201f016; continue; }
  pc = 0x1201f00c; continue;
 }
 0x1201f00c => {
  { let v: u32 = 0xfffffffcu32; cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201f016 => {
  { cpu.and8(cpu.r8(0xffffffcau32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x1201ef7b; continue; }
  pc = 0x1201f020; continue;
 }
 0x1201f020 => {
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0xffffffcau32.wrapping_add(cpu.ebp)), 0x4u8); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  if cpu.zf { pc = 0x1201f043; continue; }
  pc = 0x1201f034; continue;
 }
 0x1201f034 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1201f03b; continue;
 }
 0x1201f03b => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1201ef3a; continue;
 }
 0x1201f043 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201f03b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f04f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f04f); }
 let mut pc: u32 = 0x1201f04f;
 loop { match pc {
 0x1201f04f => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1201f05d); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x1203d410u32), 0xfffffffeu32); }
  { let v: u32 = 0x2u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  if !cpu.zf { pc = 0x1201f07b; continue; }
  pc = 0x1201f076; continue;
 }
 0x1201f076 => {
  cpu.push(0x1201f07b); f_12021789(cpu);
  pc = 0x1201f07b; continue;
 }
 0x1201f07b => {
  { cpu.sub32(cpu.r32(0x1203d410u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201f098; continue; }
  pc = 0x1201f084; continue;
 }
 0x1201f084 => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1201f08b); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f098 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041050u32); cpu.push(0x1201f0aa); cpu.call(t);
  { let v = 0x7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041054u32); cpu.ebx = v; }
  let t = cpu.ebx; cpu.push(0x1201f0bb); cpu.call(t);
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041058u32); cpu.push(0x1201f0d5); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201f0db; continue; }
  pc = 0x1201f0d9; continue;
 }
 0x1201f0d9 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x1201f0db; continue;
 }
 0x1201f0db => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1201f0f7; continue; }
  pc = 0x1201f0df; continue;
 }
 0x1201f0df => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0xdu8); }
  if !cpu.zf { pc = 0x1201f119; continue; }
  pc = 0x1201f0ea; continue;
 }
 0x1201f0ea => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201f0f7; continue;
 }
 0x1201f0f7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x1201f105); cpu.call(t);
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1201f10c); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f119 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x1201f138; continue; }
  pc = 0x1201f123; continue;
 }
 0x1201f123 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.eax)), 0xdu8); }
  if !cpu.zf { pc = 0x1201f138; continue; }
  pc = 0x1201f129; continue;
 }
 0x1201f129 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201f0f7; continue;
 }
 0x1201f138 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x1201f14f; continue; }
  pc = 0x1201f13f; continue;
 }
 0x1201f13f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0xau8); }
  if !cpu.zf { pc = 0x1201f14f; continue; }
  pc = 0x1201f145; continue;
 }
 0x1201f145 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1201f0f7; continue;
 }
 0x1201f14f => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201f0f7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f15c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f15c); }
 let mut pc: u32 = 0x1201f15c;
 loop { match pc {
 0x1201f15c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x1201f1b1; continue; }
  pc = 0x1201f16a; continue;
 }
 0x1201f16a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201f1b1; continue; }
  pc = 0x1201f18a; continue;
 }
 0x1201f18a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f190); f_12016b72(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f19e); f_1201f1c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f1a9); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f1b1 => {
  cpu.push(0x1201f1b6); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f1c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f1c4); }
 let mut pc: u32 = 0x1201f1c4;
 loop { match pc {
 0x1201f1c4 => {
  { let v: u32 = 0x1008u32; cpu.eax = v; }
  cpu.push(0x1201f1ce); f_12012390(cpu);
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1018u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f1e4); f_1201677e(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201f2ea; continue; }
  pc = 0x1201f1f4; continue;
 }
 0x1201f1f4 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f1fd); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201f2ea; continue; }
  pc = 0x1201f209; continue;
 }
 0x1201f209 => {
  { let v: u32 = cpu.r32(0x1020u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201f2a6; continue; }
  pc = 0x1201f21c; continue;
 }
 0x1201f21c => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v = 0x8000u32; cpu.push(v); }
  { cpu.stos(4, true); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f234); f_12021177(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201f23b; continue;
 }
 0x1201f23b => {
  { let v: u32 = 0x1000u32; cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1201f246; continue; }
  pc = 0x1201f244; continue;
 }
 0x1201f244 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x1201f246; continue;
 }
 0x1201f246 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f252); f_120163e4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201f28d; continue; }
  pc = 0x1201f25c; continue;
 }
 0x1201f25c => {
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edi); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201f23b; continue; }
  pc = 0x1201f262; continue;
 }
 0x1201f262 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f26d); f_12021177(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201f270; continue;
 }
 0x1201f270 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f27d); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1008u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f28d => {
  cpu.push(0x1201f292); f_120133bd(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x5u32); }
  if !cpu.zf { pc = 0x1201f2a2; continue; }
  pc = 0x1201f297; continue;
 }
 0x1201f297 => {
  cpu.push(0x1201f29c); f_120133b4(cpu);
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201f2a2; continue;
 }
 0x1201f2a2 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x1201f262; continue;
 }
 0x1201f2a6 => {
  if (cpu.sf == cpu.of) { pc = 0x1201f270; continue; }
  pc = 0x1201f2a8; continue;
 }
 0x1201f2a8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f2b1); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f2ba); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204104cu32); cpu.push(0x1201f2c4); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201f270; continue; }
  pc = 0x1201f2ce; continue;
 }
 0x1201f2ce => {
  cpu.push(0x1201f2d3); f_120133b4(cpu);
  { let v: u32 = 0xdu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f2de); f_120133bd(cpu);
  { let v: u32 = cpu.eax; cpu.edi = v; }
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201f2e6); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201f270; continue;
 }
 0x1201f2ea => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1008u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f2fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f2fa); }
 let mut pc: u32 = 0x1201f2fa;
 loop { match pc {
 0x1201f2fa => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1201f309); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203d414u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xfffffffeu32); }
  if !cpu.zf { pc = 0x1201f322; continue; }
  pc = 0x1201f317; continue;
 }
 0x1201f317 => {
  cpu.push(0x1201f31c); f_120217a9(cpu);
  { let v: u32 = cpu.r32(0x1203d414u32); cpu.ebx = v; }
  pc = 0x1201f322; continue;
 }
 0x1201f322 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201f34d; continue; }
  pc = 0x1201f327; continue;
 }
 0x1201f327 => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x12041048u32); cpu.push(0x1201f349); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201f352; continue; }
  pc = 0x1201f34d; continue;
 }
 0x1201f34d => {
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  pc = 0x1201f352; continue;
 }
 0x1201f352 => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1201f359); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f365(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f365); }
 let mut pc: u32 = 0x1201f365;
 loop { match pc {
 0x1201f365 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x302u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1201f379); f_12020718(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f37d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f37d); }
 let mut pc: u32 = 0x1201f37d;
 loop { match pc {
 0x1201f37d => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201f49c; continue; }
  pc = 0x1201f394; continue;
 }
 0x1201f394 => {
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
  if cpu.zf { pc = 0x1201f49c; continue; }
  pc = 0x1201f3ba; continue;
 }
 0x1201f3ba => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f3c0); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x1201f3ce); f_12016800(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x1201f401; continue; }
  pc = 0x1201f3d5; continue;
 }
 0x1201f3d5 => {
  cpu.push(0x1201f3da); f_120133b4(cpu);
  { let v: u32 = 0x18u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f3e5); f_120133bd(cpu);
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f3f1); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f401 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x120411fcu32); cpu.ebp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201f414); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f41b); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201f421); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041044u32); cpu.push(0x1201f428); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201f460; continue; }
  pc = 0x1201f42c; continue;
 }
 0x1201f42c => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201f432); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1201f434; continue;
 }
 0x1201f434 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201f43a); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f443); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1201f472; continue; }
  pc = 0x1201f44a; continue;
 }
 0x1201f44a => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1201f450); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f460 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1201f46d); f_12016941(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1201f434; continue;
 }
 0x1201f472 => {
  { let v: u8 = (cpu.ebx as u8) & 0xefu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f49c => {
  cpu.push(0x1201f4a1); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f4ac); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f4bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f4bf); }
 let mut pc: u32 = 0x1201f4bf;
 loop { match pc {
 0x1201f4bf => {
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x1201f695; continue; }
  pc = 0x1201f4d6; continue;
 }
 0x1201f4d6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201f695; continue; }
  pc = 0x1201f4fc; continue;
 }
 0x1201f4fc => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x800u32); }
  if !cpu.cf { pc = 0x1201f695; continue; }
  pc = 0x1201f50c; continue;
 }
 0x1201f50c => {
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1201f539; continue; }
  pc = 0x1201f514; continue;
 }
 0x1201f514 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201f51a); f_1201f6b8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f539; continue; }
  pc = 0x1201f521; continue;
 }
 0x1201f521 => {
  cpu.push(0x1201f526); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f539 => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x1201f58d; continue; }
  pc = 0x1201f53d; continue;
 }
 0x1201f53d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f543); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x1201f547; continue;
 }
 0x1201f547 => {
  cpu.push(0x1201f54c); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201f54f; continue;
 }
 0x1201f54f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x1u8); }
  if !cpu.zf { pc = 0x1201f59b; continue; }
  pc = 0x1201f558; continue;
 }
 0x1201f558 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f55e); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201f567); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x1201f56f); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f57a); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f58d => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1201f54f; continue; }
  pc = 0x1201f58f; continue;
 }
 0x1201f58f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201f595); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x1201f547; continue;
 }
 0x1201f59b => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if !cpu.zf { pc = 0x1201f5bb; continue; }
  pc = 0x1201f59f; continue;
 }
 0x1201f59f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f5a5); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201f5ae); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f5bb => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1203ef10u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201f5f1; continue; }
  pc = 0x1201f5e8; continue;
 }
 0x1201f5e8 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201f5ee); f_1201667d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201f5f1; continue;
 }
 0x1201f5f1 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411fcu32); cpu.push(0x1201f602); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f609); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x120411fcu32); cpu.push(0x1201f613); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041044u32); cpu.push(0x1201f61a); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201f650; continue; }
  pc = 0x1201f61e; continue;
 }
 0x1201f61e => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201f624); cpu.call(t);
  pc = 0x1201f624; continue;
 }
 0x1201f624 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f662; continue; }
  pc = 0x1201f628; continue;
 }
 0x1201f628 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201f62e); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f637); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201f640); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f650 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201f65b); f_12016941(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1201f624; continue;
 }
 0x1201f662 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xefu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  cpu.push(0x1201f67f); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1201f688); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f695 => {
  cpu.push(0x1201f69a); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f6a5); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f6b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f6b8); }
 let mut pc: u32 = 0x1201f6b8;
 loop { match pc {
 0x1201f6b8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1203f010u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1201f72b; continue; }
  pc = 0x1201f6c8; continue;
 }
 0x1201f6c8 => {
  { let v: u32 = 0x1203ef10u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  pc = 0x1201f6d2; continue;
 }
 0x1201f6d2 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1201f720; continue; }
  pc = 0x1201f6d7; continue;
 }
 0x1201f6d7 => {
  { let v = 0x480u32; cpu.push(v); }
  cpu.push(0x1201f6e1); f_12004858(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f732; continue; }
  pc = 0x1201f6e8; continue;
 }
 0x1201f6e8 => {
  { let v: u32 = 0x480u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x1203f010u32).wrapping_add(cpu.ebx); cpu.w32(0x1203f010u32, v); }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x1201f720; continue; }
  pc = 0x1201f6fa; continue;
 }
 0x1201f6fa => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.edx = v; }
  pc = 0x1201f701; continue;
 }
 0x1201f701 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x24u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffe1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x480u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1201f701; continue; }
  pc = 0x1201f720; continue;
 }
 0x1201f720 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1203f010u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1201f6d2; continue; }
  pc = 0x1201f72b; continue;
 }
 0x1201f72b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f732 => {
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
pub fn f_1201f73c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f73c); }
 let mut pc: u32 = 0x1201f73c;
 loop { match pc {
 0x1201f73c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x1201f7c7; continue; }
  pc = 0x1201f74a; continue;
 }
 0x1201f74a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201f7c7; continue; }
  pc = 0x1201f76a; continue;
 }
 0x1201f76a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f770); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f77d); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201f7c0; continue; }
  pc = 0x1201f787; continue;
 }
 0x1201f787 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f791); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1201f7c0; continue; }
  pc = 0x1201f799; continue;
 }
 0x1201f799 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { pc = 0x1201f7b0; continue; }
  pc = 0x1201f79d; continue;
 }
 0x1201f79d => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x1201f7a2; continue;
 }
 0x1201f7a2 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f7a8); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f7b0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x1201f7bb); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1201f7a2; continue;
 }
 0x1201f7c0 => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x1201f7a2; continue;
 }
 0x1201f7c7 => {
  cpu.push(0x1201f7cc); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f7d7); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f7e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f7e5); }
 let mut pc: u32 = 0x1201f7e5;
 loop { match pc {
 0x1201f7e5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201f866; continue; }
  pc = 0x1201f7f4; continue;
 }
 0x1201f7f4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201f866; continue; }
  pc = 0x1201f814; continue;
 }
 0x1201f814 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f81a); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f827); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1201f845; continue; }
  pc = 0x1201f831; continue;
 }
 0x1201f831 => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x1201f836; continue;
 }
 0x1201f836 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f83c); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f845 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f84f); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x1201f836; continue; }
  pc = 0x1201f858; continue;
 }
 0x1201f858 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f861); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1201f836; continue;
 }
 0x1201f866 => {
  cpu.push(0x1201f86b); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f876); f_120133bd(cpu);
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
pub fn f_1201f885(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f885); }
 let mut pc: u32 = 0x1201f885;
 loop { match pc {
 0x1201f885 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x1201f944; continue; }
  pc = 0x1201f899; continue;
 }
 0x1201f899 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201f944; continue; }
  pc = 0x1201f8bd; continue;
 }
 0x1201f8bd => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f8c3); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f8d2); f_1202059d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x1201f90a; continue; }
  pc = 0x1201f8e6; continue;
 }
 0x1201f8e6 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), cpu.eax); }
  if !cpu.zf { pc = 0x1201f90a; continue; }
  pc = 0x1201f8ec; continue;
 }
 0x1201f8ec => {
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201f8f4; continue;
 }
 0x1201f8f4 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f8fa); f_12016bd3(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201f90a => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f916); f_1202059d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), cpu.edx); }
  if !cpu.zf { pc = 0x1201f92d; continue; }
  pc = 0x1201f927; continue;
 }
 0x1201f927 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), cpu.eax); }
  if cpu.zf { pc = 0x1201f8f4; continue; }
  pc = 0x1201f92d; continue;
 }
 0x1201f92d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f93f); f_1202059d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x1201f8f4; continue;
 }
 0x1201f944 => {
  cpu.push(0x1201f949); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1201f954); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201f966(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201f966); }
 let mut pc: u32 = 0x1201f966;
 loop { match pc {
 0x1201f966 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x1201fc7a; continue; }
  pc = 0x1201f985; continue;
 }
 0x1201f985 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201fc7a; continue; }
  pc = 0x1201f9ab; continue;
 }
 0x1201f9ab => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201f9b1); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041170u32); cpu.push(0x1201f9c0); cpu.call(t);
  { let v: u32 = cpu.eax & 0xffff7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1201fa8b; continue; }
  pc = 0x1201f9ce; continue;
 }
 0x1201f9ce => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x1201fa07; continue; }
  pc = 0x1201f9d3; continue;
 }
 0x1201f9d3 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x1201fa02; continue; }
  pc = 0x1201f9d8; continue;
 }
 0x1201f9d8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201f9f1; continue; }
  pc = 0x1201f9dc; continue;
 }
 0x1201f9dc => {
  cpu.push(0x1201f9e1); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201f9e7; continue;
 }
 0x1201f9e7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201fa3b; continue;
 }
 0x1201f9f1 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201f9f7); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201f9fd); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201f9e7; continue;
 }
 0x1201fa02 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x1201fa50; continue; }
  pc = 0x1201fa07; continue;
 }
 0x1201fa07 => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = 0x2000u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fa11; continue;
 }
 0x1201fa11 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  { let v: u16 = 0x1u16; cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { pc = 0x1201fa5c; continue; }
  pc = 0x1201fa38; continue;
 }
 0x1201fa38 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fa3b; continue;
 }
 0x1201fa3b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201fa41); f_12016bd3(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201fa50 => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = 0x1000u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fa11; continue;
 }
 0x1201fa5c => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x1204103cu32); cpu.push(0x1201fa75); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201fa82; continue; }
  pc = 0x1201fa79; continue;
 }
 0x1201fa79 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fa3b; continue;
 }
 0x1201fa82 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fa3b; continue;
 }
 0x1201fa8b => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041040u32); cpu.push(0x1201fab8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201fad0; continue; }
  pc = 0x1201fabc; continue;
 }
 0x1201fabc => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201fac2); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201fac8); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201f9e7; continue;
 }
 0x1201fad0 => {
  { cpu.and8(cpu.r8(0x30u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x1201fc02; continue; }
  pc = 0x1201fadb; continue;
 }
 0x1201fadb => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) | 0x124u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fae1; continue;
 }
 0x1201fae1 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x44u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x120410dcu32); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x1201faf3); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f9e7; continue; }
  pc = 0x1201fafb; continue;
 }
 0x1201fafb => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x120410e0u32); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201fb0d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f9e7; continue; }
  pc = 0x1201fb15; continue;
 }
 0x1201fb15 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201fb52); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201fb66; continue; }
  pc = 0x1201fb5f; continue;
 }
 0x1201fb5f => {
  { cpu.sub32(cpu.r32(0x40u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201fbce; continue; }
  pc = 0x1201fb66; continue;
 }
 0x1201fb66 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x1201fb72); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f9e7; continue; }
  pc = 0x1201fb7a; continue;
 }
 0x1201fb7a => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201fb86); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f9e7; continue; }
  pc = 0x1201fb8e; continue;
 }
 0x1201fb8e => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201fbcb); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201fbce; continue;
 }
 0x1201fbce => {
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { pc = 0x1201fc0d; continue; }
  pc = 0x1201fbd8; continue;
 }
 0x1201fbd8 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201fc0d; continue; }
  pc = 0x1201fbdf; continue;
 }
 0x1201fbdf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  pc = 0x1201fbe2; continue;
 }
 0x1201fbe2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)) | 0x80u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fa3b; continue;
 }
 0x1201fc02 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) | 0x1b6u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fae1; continue;
 }
 0x1201fc0d => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x1201fc19); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f9e7; continue; }
  pc = 0x1201fc21; continue;
 }
 0x1201fc21 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201fc2d); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201f9e7; continue; }
  pc = 0x1201fc35; continue;
 }
 0x1201fc35 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201fc72); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201fbe2; continue;
 }
 0x1201fc7a => {
  cpu.push(0x1201fc7f); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201fc92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201fc92); }
 let mut pc: u32 = 0x1201fc92;
 loop { match pc {
 0x1201fc92 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x1201ffd4; continue; }
  pc = 0x1201fcb1; continue;
 }
 0x1201fcb1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1201ffd4; continue; }
  pc = 0x1201fcd7; continue;
 }
 0x1201fcd7 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201fcdd); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041170u32); cpu.push(0x1201fcec); cpu.call(t);
  { let v: u32 = cpu.eax & 0xffff7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1201fdcd; continue; }
  pc = 0x1201fcfa; continue;
 }
 0x1201fcfa => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x1201fd33; continue; }
  pc = 0x1201fcff; continue;
 }
 0x1201fcff => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x1201fd2e; continue; }
  pc = 0x1201fd04; continue;
 }
 0x1201fd04 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201fd1d; continue; }
  pc = 0x1201fd08; continue;
 }
 0x1201fd08 => {
  cpu.push(0x1201fd0d); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1201fd13; continue;
 }
 0x1201fd13 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x1201fd6d; continue;
 }
 0x1201fd1d => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201fd23); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201fd29); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201fd13; continue;
 }
 0x1201fd2e => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x1201fd82; continue; }
  pc = 0x1201fd33; continue;
 }
 0x1201fd33 => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = 0x2000u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fd3d; continue;
 }
 0x1201fd3d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  { let v: u16 = 0x1u16; cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { pc = 0x1201fd8e; continue; }
  pc = 0x1201fd64; continue;
 }
 0x1201fd64 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  pc = 0x1201fd6d; continue;
 }
 0x1201fd6d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1201fd73); f_12016bd3(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1201fd82 => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = 0x1000u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fd3d; continue;
 }
 0x1201fd8e => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x1204103cu32); cpu.push(0x1201fda7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201fdba; continue; }
  pc = 0x1201fdab; continue;
 }
 0x1201fdab => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fd6d; continue;
 }
 0x1201fdba => {
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fd6d; continue;
 }
 0x1201fdcd => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041040u32); cpu.push(0x1201fdfa); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1201fe12; continue; }
  pc = 0x1201fdfe; continue;
 }
 0x1201fdfe => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x1201fe04); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201fe0a); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x1201fd13; continue;
 }
 0x1201fe12 => {
  { cpu.and8(cpu.r8(0x30u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x1201ff5c; continue; }
  pc = 0x1201fe1d; continue;
 }
 0x1201fe1d => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) | 0x124u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fe23; continue;
 }
 0x1201fe23 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x44u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x120410dcu32); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x1201fe35); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201fd13; continue; }
  pc = 0x1201fe3d; continue;
 }
 0x1201fe3d => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x120410e0u32); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201fe4f); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201fd13; continue; }
  pc = 0x1201fe57; continue;
 }
 0x1201fe57 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201fe94); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201fea8; continue; }
  pc = 0x1201fea1; continue;
 }
 0x1201fea1 => {
  { cpu.sub32(cpu.r32(0x40u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1201ff10; continue; }
  pc = 0x1201fea8; continue;
 }
 0x1201fea8 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x1201feb4); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201fd13; continue; }
  pc = 0x1201febc; continue;
 }
 0x1201febc => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201fec8); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201fd13; continue; }
  pc = 0x1201fed0; continue;
 }
 0x1201fed0 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ff0d); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201ff10; continue;
 }
 0x1201ff10 => {
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { pc = 0x1201ff67; continue; }
  pc = 0x1201ff1a; continue;
 }
 0x1201ff1a => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x1201ff67; continue; }
  pc = 0x1201ff21; continue;
 }
 0x1201ff21 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  pc = 0x1201ff24; continue;
 }
 0x1201ff24 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ff37); f_12018734(cpu);
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x54u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edi)) | 0x80u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1201fd6d; continue;
 }
 0x1201ff5c => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) | 0x1b6u16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1201fe23; continue;
 }
 0x1201ff67 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebx; cpu.push(0x1201ff73); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201fd13; continue; }
  pc = 0x1201ff7b; continue;
 }
 0x1201ff7b => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1201ff87); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1201fd13; continue; }
  pc = 0x1201ff8f; continue;
 }
 0x1201ff8f => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x26u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1201ffcc); f_1202ef0d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  pc = 0x1201ff24; continue;
 }
 0x1201ffd4 => {
  cpu.push(0x1201ffd9); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1201ffec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1201ffec); }
 let mut pc: u32 = 0x1201ffec;
 loop { match pc {
 0x1201ffec => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1201fff4); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x1201fffc); f_1202002c(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12020005); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202000c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202000c); }
 let mut pc: u32 = 0x1202000c;
 loop { match pc {
 0x1202000c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12020014); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x1202001c); f_1202011f(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12020025); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202002c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202002c); }
 let mut pc: u32 = 0x1202002c;
 loop { match pc {
 0x1202002c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203d408u32), 0xffffffffu32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if cpu.zf { pc = 0x12020055; continue; }
  pc = 0x1202003c; continue;
 }
 0x1202003c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x1203d408u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1203d408u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020055 => {
  { cpu.sub32(cpu.r32(0x1203d410u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1202006b; continue; }
  pc = 0x1202005e; continue;
 }
 0x1202005e => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202006b => {
  { cpu.sub32(cpu.r32(0x1203d410u32), 0xfffffffeu32); }
  if !cpu.zf { pc = 0x12020079; continue; }
  pc = 0x12020074; continue;
 }
 0x12020074 => {
  cpu.push(0x12020079); f_12021789(cpu);
  pc = 0x12020079; continue;
 }
 0x12020079 => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  let t = cpu.r32(0x12041050u32); cpu.push(0x1202008d); cpu.call(t);
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  let t = cpu.r32(0x12041054u32); cpu.push(0x120200a1); cpu.call(t);
  { let v: u32 = cpu.r32(0x12041038u32); cpu.esi = v; }
  pc = 0x120200a7; continue;
 }
 0x120200a7 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x120200bb); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12020118; continue; }
  pc = 0x120200bf; continue;
 }
 0x120200bf => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), cpu.edi); }
  if cpu.zf { pc = 0x12020118; continue; }
  pc = 0x120200c5; continue;
 }
 0x120200c5 => {
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), (cpu.ebp as u16)); }
  if !cpu.zf { pc = 0x120200a7; continue; }
  pc = 0x120200cc; continue;
 }
 0x120200cc => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esp)), cpu.edi); }
  if cpu.zf { pc = 0x120200a7; continue; }
  pc = 0x120200d2; continue;
 }
 0x120200d2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x120200fc; continue; }
  pc = 0x120200dc; continue;
 }
 0x120200dc => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120200e6); f_120202be(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120200a7; continue; }
  pc = 0x120200ed; continue;
 }
 0x120200ed => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1203d408u32, v); }
  pc = 0x120200fc; continue;
 }
 0x120200fc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041054u32); cpu.push(0x1202010e); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020118 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  pc = 0x120200fc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202011f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202011f); }
 let mut pc: u32 = 0x1202011f;
 loop { match pc {
 0x1202011f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203d408u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1202013b; continue; }
  pc = 0x1202012b; continue;
 }
 0x1202012b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1203d408u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202013b => {
  cpu.push(0x12020140); f_1202002c(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  if cpu.zf { pc = 0x12020159; continue; }
  pc = 0x12020147; continue;
 }
 0x12020147 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1202014d); f_12020e15(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x12020159; continue; }
  pc = 0x12020155; continue;
 }
 0x12020155 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020159 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020160); }
 let mut pc: u32 = 0x12020160;
 loop { match pc {
 0x12020160 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12020168); f_1200420b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x12020170); f_12020180(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12020179); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020180); }
 let mut pc: u32 = 0x12020180;
 loop { match pc {
 0x12020180 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1203d408u32), 0xffffffffu32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.zf { pc = 0x1202019f; continue; }
  pc = 0x12020191; continue;
 }
 0x12020191 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202019f => {
  { cpu.sub32(cpu.r32(0x1203d410u32), 0xfffffffeu32); }
  if !cpu.zf { pc = 0x120201ad; continue; }
  pc = 0x120201a8; continue;
 }
 0x120201a8 => {
  cpu.push(0x120201ad); f_12021789(cpu);
  pc = 0x120201ad; continue;
 }
 0x120201ad => {
  { cpu.sub32(cpu.r32(0x1203d410u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x12020268; continue; }
  pc = 0x120201ba; continue;
 }
 0x120201ba => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041030u32); cpu.push(0x120201cb); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12020268; continue; }
  pc = 0x120201d3; continue;
 }
 0x120201d3 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x12020268; continue; }
  pc = 0x120201dd; continue;
 }
 0x120201dd => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffffcu32; cpu.eax = v; }
  cpu.push(0x120201ef); f_12012390(cpu);
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x12020268; continue; }
  pc = 0x120201f5; continue;
 }
 0x120201f5 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203d410u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x12041034u32); cpu.push(0x1202020b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1202024f; continue; }
  pc = 0x1202020f; continue;
 }
 0x1202020f => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1202024f; continue; }
  pc = 0x12020215; continue;
 }
 0x12020215 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1202024f; continue; }
  pc = 0x1202021d; continue;
 }
 0x1202021d => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1202024f; continue; }
  pc = 0x12020223; continue;
 }
 0x12020223 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x12020225; continue;
 }
 0x12020225 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x1u16); }
  if !cpu.zf { pc = 0x12020244; continue; }
  pc = 0x1202022b; continue;
 }
 0x1202022b => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ebx); }
  if cpu.zf { pc = 0x12020244; continue; }
  pc = 0x12020232; continue;
 }
 0x12020232 => {
  { cpu.sub8(cpu.r8(0xeu32.wrapping_add(cpu.esi)), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x1202025a; continue; }
  pc = 0x12020237; continue;
 }
 0x12020237 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1202023d); f_120202be(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1202025a; continue; }
  pc = 0x12020244; continue;
 }
 0x12020244 => {
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12020225; continue; }
  pc = 0x1202024f; continue;
 }
 0x1202024f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202025a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020268 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020273(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020273); }
 let mut pc: u32 = 0x12020273;
 loop { match pc {
 0x12020273 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x1202027b); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12020288); f_1202029b(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12020294); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202029b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202029b); }
 let mut pc: u32 = 0x1202029b;
 loop { match pc {
 0x1202029b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x120202b8; continue; }
  pc = 0x120202a4; continue;
 }
 0x120202a4 => {
  { cpu.sub32(cpu.r32(0x1203d408u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x120202b8; continue; }
  pc = 0x120202ad; continue;
 }
 0x120202ad => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1203d408u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120202b8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120202be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120202be); }
 let mut pc: u32 = 0x120202be;
 loop { match pc {
 0x120202be => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, 0x100u32); }
  if cpu.zf { pc = 0x12020339; continue; }
  pc = 0x120202cf; continue;
 }
 0x120202cf => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = 0x1203d0c0u32; cpu.edx = v; }
  pc = 0x120202dc; continue;
 }
 0x120202dc => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edx)), (cpu.ecx as u16)); }
  if cpu.zf { pc = 0x120202f0; continue; }
  pc = 0x120202e1; continue;
 }
 0x120202e1 => {
  { let v: u32 = cpu.edx.wrapping_add(0xau32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x1203d138u32); }
  if cpu.cf { pc = 0x120202dc; continue; }
  pc = 0x120202ed; continue;
 }
 0x120202ed => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120202f0 => {
  { cpu.and32(cpu.esi, 0x3u32); }
  if cpu.zf { pc = 0x12020305; continue; }
  pc = 0x120202f8; continue;
 }
 0x120202f8 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x1203d0c8u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020305 => {
  { cpu.and32(cpu.esi, 0xcu32); }
  if cpu.zf { pc = 0x1202031a; continue; }
  pc = 0x1202030d; continue;
 }
 0x1202030d => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x1203d0c6u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202031a => {
  { cpu.and32(cpu.esi, 0x10u32); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.eax = v; }
  if cpu.zf { pc = 0x1202032f; continue; }
  pc = 0x12020325; continue;
 }
 0x12020325 => {
  { let v: u32 = 0x1203d0c4u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202032f => {
  { let v: u32 = 0x1203d0c2u32.wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020339 => {
  { cpu.and32(cpu.esi, 0x3u32); }
  if cpu.zf { pc = 0x12020364; continue; }
  pc = 0x12020341; continue;
 }
 0x12020341 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1203d146u32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  pc = 0x1202034e; continue;
 }
 0x1202034e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x12020359; continue; }
  pc = 0x12020354; continue;
 }
 0x12020354 => {
  { cpu.sub8((cpu.ecx as u8), 0xe0u8); }
  if !cpu.zf { pc = 0x1202035f; continue; }
  pc = 0x12020359; continue;
 }
 0x12020359 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x12020361; continue; }
  pc = 0x1202035f; continue;
 }
 0x1202035f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x12020361; continue;
 }
 0x12020361 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020364 => {
  { cpu.and32(cpu.esi, 0xcu32); }
  if cpu.zf { pc = 0x1202037b; continue; }
  pc = 0x1202036c; continue;
 }
 0x1202036c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1203d144u32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  pc = 0x1202034e; continue;
 }
 0x1202037b => {
  { cpu.and32(cpu.esi, 0x10u32); }
  if cpu.zf { pc = 0x12020392; continue; }
  pc = 0x12020383; continue;
 }
 0x12020383 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1203d142u32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  pc = 0x1202034e; continue;
 }
 0x12020392 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1203d140u32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  pc = 0x1202034e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120203a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120203a1); }
 let mut pc: u32 = 0x120203a1;
 loop { match pc {
 0x120203a1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x120203a8, "In");
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120203aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120203aa); }
 let mut pc: u32 = 0x120203aa;
 loop { match pc {
 0x120203aa => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x120203af, "In");
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120203b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120203b2); }
 let mut pc: u32 = 0x120203b2;
 loop { match pc {
 0x120203b2 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x120203b7, "In");
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120203b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120203b9); }
 let mut pc: u32 = 0x120203b9;
 loop { match pc {
 0x120203b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x120204ec; continue; }
  pc = 0x120203cd; continue;
 }
 0x120203cd => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x120204ec; continue; }
  pc = 0x120203f1; continue;
 }
 0x120203f1 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120203f7); f_12016b72(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12020404); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12020421; continue; }
  pc = 0x1202040e; continue;
 }
 0x1202040e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12020414); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020421 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x1u32); }
  if cpu.zf { pc = 0x12020434; continue; }
  pc = 0x12020428; continue;
 }
 0x12020428 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x3u32); }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if !cpu.zf { pc = 0x12020439; continue; }
  pc = 0x12020434; continue;
 }
 0x12020434 => {
  { let v: u32 = 0x9u32; cpu.edi = v; }
  pc = 0x12020439; continue;
 }
 0x12020439 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), cpu.ebx); }
  if !cpu.zf { pc = 0x12020467; continue; }
  pc = 0x12020441; continue;
 }
 0x12020441 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1202044f); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041028u32); cpu.push(0x12020459); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12020485; continue; }
  pc = 0x1202045d; continue;
 }
 0x1202045d => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12020463); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x12020485; continue;
 }
 0x12020467 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12020477); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204102cu32); cpu.push(0x12020481); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1202045d; continue; }
  pc = 0x12020485; continue;
 }
 0x12020485 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1202049b; continue; }
  pc = 0x12020489; continue;
 }
 0x12020489 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1202049b; continue; }
  pc = 0x1202048d; continue;
 }
 0x1202048d => {
  { let v = 0x3e8u32; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  let t = cpu.r32(0x1204111cu32); cpu.push(0x12020499); cpu.call(t);
  pc = 0x12020439; continue;
 }
 0x1202049b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120204a1); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x120204e5; continue; }
  pc = 0x120204a8; continue;
 }
 0x120204a8 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x1u32); }
  if cpu.zf { pc = 0x120204c9; continue; }
  pc = 0x120204af; continue;
 }
 0x120204af => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x3u32); }
  if cpu.zf { pc = 0x120204c9; continue; }
  pc = 0x120204b6; continue;
 }
 0x120204b6 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x120204bc); f_12013341(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120204c9 => {
  cpu.push(0x120204ce); f_120133b4(cpu);
  { let v: u32 = 0x24u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120204d9); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120204e5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120204ec => {
  cpu.push(0x120204f1); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120204fc); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202050c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202050c); }
 let mut pc: u32 = 0x1202050c;
 loop { match pc {
 0x1202050c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x1202057b; continue; }
  pc = 0x1202051c; continue;
 }
 0x1202051c => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x1202057b; continue; }
  pc = 0x1202053c; continue;
 }
 0x1202053c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12020542); f_12016b72(cpu);
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1202055a); f_1202059d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1202056b); f_12016bd3(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202057b => {
  cpu.push(0x12020580); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1202058b); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202059d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202059d); }
 let mut pc: u32 = 0x1202059d;
 loop { match pc {
 0x1202059d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x120205bb); f_12016a74(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x120205da; continue; }
  pc = 0x120205c3; continue;
 }
 0x120205c3 => {
  cpu.push(0x120205c8); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120205da => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x12041124u32); cpu.push(0x120205f0); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12020618; continue; }
  pc = 0x120205f9; continue;
 }
 0x120205f9 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x120205ff); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12020618; continue; }
  pc = 0x12020603; continue;
 }
 0x12020603 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12020609); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020618 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi & 0x1fu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1))) & 0xfdu8; cpu.w8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020641(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020641); }
 let mut pc: u32 = 0x12020641;
 loop { match pc {
 0x12020641 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  cpu.push(0x12020652); f_1201292c(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x1202065f; continue; }
  pc = 0x12020658; continue;
 }
 0x12020658 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x12020658; continue; }
  pc = 0x1202065f; continue;
 }
 0x1202065f => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  if cpu.cf { pc = 0x120206a0; continue; }
  pc = 0x12020664; continue;
 }
 0x12020664 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1202066b); f_12022012(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120206a0; continue; }
  pc = 0x12020672; continue;
 }
 0x12020672 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x58u8); }
  if !cpu.zf { pc = 0x12020711; continue; }
  pc = 0x1202067c; continue;
 }
 0x1202067c => {
  { cpu.sub32(cpu.ebx, 0x5u32); }
  if (cpu.sf == cpu.of) { pc = 0x120206a0; continue; }
  pc = 0x12020681; continue;
 }
 0x12020681 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.ebp, cpu.edi); }
  if !cpu.cf { pc = 0x12020664; continue; }
  pc = 0x120206a0; continue;
 }
 0x120206a0 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), 0x58u8); }
  if !cpu.zf { pc = 0x12020711; continue; }
  pc = 0x120206a6; continue;
 }
 0x120206a6 => {
  { cpu.sub32(cpu.ebx, 0x5u32); }
  if (cpu.sf != cpu.of) { pc = 0x12020711; continue; }
  pc = 0x120206ab; continue;
 }
 0x120206ab => {
  { let v: u32 = 0x62u32; cpu.ebx = v; }
  { let v: u8 = 0x61u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x120206b9); f_120133b4(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  cpu.push(0x120206c0); f_120133b4(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x120206c6; continue;
 }
 0x120206c6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120206ce); f_1201a801(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120206df; continue; }
  pc = 0x120206d5; continue;
 }
 0x120206d5 => {
  cpu.push(0x120206da); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0xdu32); }
  if !cpu.zf { pc = 0x120206f5; continue; }
  pc = 0x120206df; continue;
 }
 0x120206df => {
  cpu.push(0x120206e4); f_120133b4(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, 0x7bu32); }
  if cpu.zf { pc = 0x12020703; continue; }
  pc = 0x120206ef; continue;
 }
 0x120206ef => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x120206c6; continue;
 }
 0x120206f5 => {
  cpu.push(0x120206fa); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020703 => {
  cpu.push(0x12020708); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020711 => {
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
pub fn f_12020718(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020718); }
 let mut pc: u32 = 0x12020718;
 loop { match pc {
 0x12020718 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1202072e); f_12020732(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020732(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020732); }
 let mut pc: u32 = 0x12020732;
 loop { match pc {
 0x12020732 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xcu32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.and32(cpu.esi, 0x80u32); }
  if cpu.zf { pc = 0x120207ac; continue; }
  pc = 0x12020753; continue;
 }
 0x12020753 => {
  { let v: u8 = 0x10u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x12020759; continue;
 }
 0x12020759 => {
  { cpu.and32(cpu.esi, 0x8000u32); }
  if !cpu.zf { pc = 0x12020778; continue; }
  pc = 0x12020761; continue;
 }
 0x12020761 => {
  { cpu.and32(cpu.esi, 0x4000u32); }
  if !cpu.zf { pc = 0x12020775; continue; }
  pc = 0x12020769; continue;
 }
 0x12020769 => {
  { cpu.sub32(cpu.r32(0x1203bc84u32), 0x8000u32); }
  if cpu.zf { pc = 0x12020778; continue; }
  pc = 0x12020775; continue;
 }
 0x12020775 => {
  { let v: u8 = (cpu.ebx as u8) | 0x80u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12020778; continue;
 }
 0x12020778 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  if cpu.zf { pc = 0x120207b8; continue; }
  pc = 0x1202077f; continue;
 }
 0x1202077f => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x120207db; continue; }
  pc = 0x12020784; continue;
 }
 0x12020784 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x120207e5; continue; }
  pc = 0x12020789; continue;
 }
 0x12020789 => {
  cpu.push(0x1202078e); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12020799); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120207ac => {
  { let v: u32 = 0x1u32; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12020759; continue;
 }
 0x120207b8 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x120207c0; continue;
 }
 0x120207c0 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120207ef; continue; }
  pc = 0x120207cc; continue;
 }
 0x120207cc => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x12020b80u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12020b6cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x120207db => {
  { let v: u32 = 0x40000000u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x120207c0; continue;
 }
 0x120207e5 => {
  { let v: u32 = 0xc0000000u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x120207c0; continue;
 }
 0x120207ef => {
  cpu.push(0x120207f4); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120207ff); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020812 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1202081a; continue;
 }
 0x1202081a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x700u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12020873; continue; }
  pc = 0x12020828; continue;
 }
 0x12020828 => {
  if cpu.zf { pc = 0x12020956; continue; }
  pc = 0x1202082e; continue;
 }
 0x1202082e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120208a9; continue; }
  pc = 0x12020832; continue;
 }
 0x12020832 => {
  cpu.push(0x12020837); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12020842); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020855 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1202081a; continue;
 }
 0x1202085f => {
  { let v: u32 = 0x2u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1202081a; continue;
 }
 0x12020869 => {
  { let v: u32 = 0x3u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1202081a; continue;
 }
 0x12020873 => {
  { cpu.sub32(cpu.eax, 0x300u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12020891; continue; }
  pc = 0x1202087a; continue;
 }
 0x1202087a => {
  if cpu.zf { pc = 0x12020963; continue; }
  pc = 0x12020880; continue;
 }
 0x12020880 => {
  { cpu.sub32(cpu.eax, 0x200u32); }
  if !cpu.zf { pc = 0x12020832; continue; }
  pc = 0x12020887; continue;
 }
 0x12020887 => {
  { let v: u32 = 0x5u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x120208b1; continue;
 }
 0x12020891 => {
  { cpu.sub32(cpu.eax, 0x500u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12020933; continue; }
  pc = 0x1202089c; continue;
 }
 0x1202089c => {
  if cpu.zf { pc = 0x12020949; continue; }
  pc = 0x120208a2; continue;
 }
 0x120208a2 => {
  { cpu.sub32(cpu.eax, 0x400u32); }
  if !cpu.zf { pc = 0x12020832; continue; }
  pc = 0x120208a9; continue;
 }
 0x120208a9 => {
  { let v: u32 = 0x3u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x120208b1; continue;
 }
 0x120208b1 => {
  { cpu.and32(cpu.esi, 0x100u32); }
  { let v: u32 = 0x80u32; cpu.ebp = v; }
  if cpu.zf { pc = 0x120208d2; continue; }
  pc = 0x120208be; continue;
 }
 0x120208be => {
  { let v: u32 = cpu.r32(0x1203b100u32); cpu.eax = v; }
  { let v: u32 = !cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.ebp, cpu.eax); }
  if !cpu.zf { pc = 0x120208d2; continue; }
  pc = 0x120208cd; continue;
 }
 0x120208cd => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x120208d2; continue;
 }
 0x120208d2 => {
  { cpu.and32(cpu.esi, 0x40u32); }
  if cpu.zf { pc = 0x120208e8; continue; }
  pc = 0x120208da; continue;
 }
 0x120208da => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)) | 0x10000u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp | 0x4000000u32; cpu.ebp = v; }
  pc = 0x120208e8; continue;
 }
 0x120208e8 => {
  { cpu.and32(cpu.esi, 0x1000u32); }
  if cpu.zf { pc = 0x120208f6; continue; }
  pc = 0x120208f0; continue;
 }
 0x120208f0 => {
  { let v: u32 = cpu.ebp | 0x100u32; cpu.ebp = v; }
  pc = 0x120208f6; continue;
 }
 0x120208f6 => {
  { cpu.and32(cpu.esi, 0x20u32); }
  if cpu.zf { pc = 0x12020970; continue; }
  pc = 0x120208fe; continue;
 }
 0x120208fe => {
  { let v: u32 = cpu.ebp | 0x8000000u32; cpu.ebp = v; }
  pc = 0x12020904; continue;
 }
 0x12020904 => {
  cpu.push(0x12020909); f_12016800(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x12020980; continue; }
  pc = 0x12020910; continue;
 }
 0x12020910 => {
  cpu.push(0x12020915); f_120133b4(cpu);
  { let v: u32 = 0x18u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12020920); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020933 => {
  { cpu.sub32(cpu.eax, 0x600u32); }
  if cpu.zf { pc = 0x12020887; continue; }
  pc = 0x1202093e; continue;
 }
 0x1202093e => {
  { cpu.sub32(cpu.eax, 0x700u32); }
  if !cpu.zf { pc = 0x12020832; continue; }
  pc = 0x12020949; continue;
 }
 0x12020949 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x120208b1; continue;
 }
 0x12020956 => {
  { let v: u32 = 0x4u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x120208b1; continue;
 }
 0x12020963 => {
  { let v: u32 = 0x2u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x120208b1; continue;
 }
 0x12020970 => {
  { cpu.and32(cpu.esi, 0x10u32); }
  if cpu.zf { pc = 0x12020904; continue; }
  pc = 0x12020978; continue;
 }
 0x12020978 => {
  { let v: u32 = cpu.ebp | 0x10000000u32; cpu.ebp = v; }
  pc = 0x12020904; continue;
 }
 0x12020980 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041024u32); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x120209a4); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  if !cpu.zf { pc = 0x120209d0; continue; }
  pc = 0x120209ab; continue;
 }
 0x120209ab => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x120209b1); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120209b7); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120209c0); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120209d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x12041170u32); cpu.push(0x120209d7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12020a07; continue; }
  pc = 0x120209db; continue;
 }
 0x120209db => {
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x120209e2); cpu.call(t);
  let t = cpu.r32(0x120411ccu32); cpu.push(0x120209e8); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120209ee); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120209f7); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020a07 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x12020aab; continue; }
  pc = 0x12020a10; continue;
 }
 0x12020a10 => {
  { let v: u8 = (cpu.ebx as u8) | 0x40u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12020a13; continue;
 }
 0x12020a13 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020a1d); f_12016941(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x48u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x12020b3b; continue; }
  pc = 0x12020a52; continue;
 }
 0x12020a52 => {
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if cpu.zf { pc = 0x12020b3b; continue; }
  pc = 0x12020a5b; continue;
 }
 0x12020a5b => {
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x12020b3b; continue; }
  pc = 0x12020a67; continue;
 }
 0x12020a67 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020a71); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12020abc; continue; }
  pc = 0x12020a7b; continue;
 }
 0x12020a7b => {
  cpu.push(0x12020a80); f_120133bd(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x83u32); }
  if cpu.zf { pc = 0x12020b3b; continue; }
  pc = 0x12020a8c; continue;
 }
 0x12020a8c => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020a92); f_1201660f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020a9b); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020aab => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x12020a13; continue; }
  pc = 0x12020ab4; continue;
 }
 0x12020ab4 => {
  { let v: u8 = (cpu.ebx as u8) | 0x8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x12020a13; continue;
 }
 0x12020abc => {
  { let v: u32 = 0x13u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020ace); f_12020edf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12020b0a; continue; }
  pc = 0x12020ad5; continue;
 }
 0x12020ad5 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esp)), 0x1au8); }
  if !cpu.zf { pc = 0x12020b0a; continue; }
  pc = 0x12020adc; continue;
 }
 0x12020adc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020ae3); f_1201f1c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12020b0a; continue; }
  pc = 0x12020aeb; continue;
 }
 0x12020aeb => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020af1); f_1201660f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020afa); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020b0a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020b14); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12020b3b; continue; }
  pc = 0x12020b1c; continue;
 }
 0x12020b1c => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020b22); f_1201660f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020b2b); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020b3b => {
  { cpu.sub8(cpu.r8(0x14u32.wrapping_add(cpu.esp)), 0x0u8); }
  if !cpu.zf { pc = 0x12020b56; continue; }
  pc = 0x12020b42; continue;
 }
 0x12020b42 => {
  { cpu.and32(cpu.esi, 0x8u32); }
  if cpu.zf { pc = 0x12020b56; continue; }
  pc = 0x12020b4a; continue;
 }
 0x12020b4a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))) | 0x20u8; cpu.w8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x12020b56; continue;
 }
 0x12020b56 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12020b5c); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020b6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020b6c); }
 let mut pc: u32 = 0x12020b6c;
 loop { match pc {
 0x12020b6c => {
  { let v: u8 = cpu.adc8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x8u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x7ef1202u32); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
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
pub fn f_12020bb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020bb1); }
 let mut pc: u32 = 0x12020bb1;
 loop { match pc {
 0x12020bb1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x12020bbc, "Out");
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020bbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020bbe); }
 let mut pc: u32 = 0x12020bbe;
 loop { match pc {
 0x12020bbe => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x12020bc8, "Out");
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020bcb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020bcb); }
 let mut pc: u32 = 0x12020bcb;
 loop { match pc {
 0x12020bcb => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.unimplemented(0x12020bd4, "Out");
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020bd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020bd6); }
 let mut pc: u32 = 0x12020bd6;
 loop { match pc {
 0x12020bd6 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.and32(cpu.edi, 0x80u32); cpu.edi = v; }
  if !cpu.zf { pc = 0x12020c05; continue; }
  pc = 0x12020bfd; continue;
 }
 0x12020bfd => {
  { let v: u32 = 0x1u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  pc = 0x12020c05; continue;
 }
 0x12020c05 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041020u32); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x12020c21); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12020c41; continue; }
  pc = 0x12020c25; continue;
 }
 0x12020c25 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12020c2b); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12020c31); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020c41 => {
  cpu.push(0x12020c46); f_12016800(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x12020dd6; continue; }
  pc = 0x12020c57; continue;
 }
 0x12020c57 => {
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = 0x89u8; cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  cpu.push(0x12020c76); f_12016800(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x12020dab; continue; }
  pc = 0x12020c84; continue;
 }
 0x12020c84 => {
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x80u8); }
  { let v: u8 = 0x89u8; cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  if !cpu.zf { pc = 0x12020cb4; continue; }
  pc = 0x12020ca3; continue;
 }
 0x12020ca3 => {
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x12020cf1; continue; }
  pc = 0x12020ca8; continue;
 }
 0x12020ca8 => {
  { cpu.sub32(cpu.r32(0x1203bc84u32), 0x8000u32); }
  if !cpu.zf { pc = 0x12020cf1; continue; }
  pc = 0x12020cb4; continue;
 }
 0x12020cb4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))) & 0x7fu8; cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))) & 0x7fu8; cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x12020cf1; continue;
 }
 0x12020cf1 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x12020d32; continue; }
  pc = 0x12020cf5; continue;
 }
 0x12020cf5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))) | 0x10u8; cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))) | 0x10u8; cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x12020d32; continue;
 }
 0x12020d32 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12020d3f); f_12016941(cpu);
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12020d50); f_12016941(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  cpu.push(0x12020d58); f_120133b4(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x12020d67); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12020d6a; continue;
 }
 0x12020d6a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12020d72); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x12020d75; continue;
 }
 0x12020d75 => {
  cpu.push(0x12020d7a); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x12020de3; continue; }
  pc = 0x12020d7f; continue;
 }
 0x12020d7f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x120411bcu32); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x12020d8c); cpu.call(t);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x12020d93); cpu.call(t);
  cpu.push(0x12020d98); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020dab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  cpu.push(0x12020dce); f_120133b4(cpu);
  { let v: u32 = 0x18u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12020d6a; continue;
 }
 0x12020dd6 => {
  cpu.push(0x12020ddb); f_120133b4(cpu);
  { let v: u32 = 0x18u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x12020d75; continue;
 }
 0x12020de3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020ded(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020ded); }
 let mut pc: u32 = 0x12020ded;
 loop { match pc {
 0x12020ded => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12020df5); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12020e02); f_12020e15(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x12020e0e); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020e15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020e15); }
 let mut pc: u32 = 0x12020e15;
 loop { match pc {
 0x12020e15 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1203d414u32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ecx, 0xfffffffeu32); }
  if !cpu.zf { pc = 0x12020e36; continue; }
  pc = 0x12020e2b; continue;
 }
 0x12020e2b => {
  cpu.push(0x12020e30); f_120217a9(cpu);
  { let v: u32 = cpu.r32(0x1203d414u32); cpu.ecx = v; }
  pc = 0x12020e36; continue;
 }
 0x12020e36 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x12020e5e; continue; }
  pc = 0x12020e3b; continue;
 }
 0x12020e3b => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x7u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x12041048u32); cpu.push(0x12020e50); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12020e5e; continue; }
  pc = 0x12020e54; continue;
 }
 0x12020e54 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020e5e => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020e67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020e67); }
 let mut pc: u32 = 0x12020e67;
 loop { match pc {
 0x12020e67 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1203f010u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x12020ec1; continue; }
  pc = 0x12020e75; continue;
 }
 0x12020e75 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x12020ec1; continue; }
  pc = 0x12020e95; continue;
 }
 0x12020e95 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12020e9b); f_12016b72(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12020eae); f_12020edf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12020eb9); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020ec1 => {
  cpu.push(0x12020ec6); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12020ed1); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12020edf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12020edf); }
 let mut pc: u32 = 0x12020edf;
 loop { match pc {
 0x12020edf => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if cpu.zf { pc = 0x12021105; continue; }
  pc = 0x12020efe; continue;
 }
 0x12020efe => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x12021105; continue; }
  pc = 0x12020f2a; continue;
 }
 0x12020f2a => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax)), 0x48u8); }
  if cpu.zf { pc = 0x12020f4b; continue; }
  pc = 0x12020f30; continue;
 }
 0x12020f30 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x12020f4b; continue; }
  pc = 0x12020f37; continue;
 }
 0x12020f37 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0xau8; cpu.w8(0x5u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x12020f4b; continue;
 }
 0x12020f4b => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x1204101cu32); cpu.push(0x12020f61); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12020fb6; continue; }
  pc = 0x12020f65; continue;
 }
 0x12020f65 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12020f6b); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x5u32); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  if !cpu.zf { pc = 0x12020f91; continue; }
  pc = 0x12020f72; continue;
 }
 0x12020f72 => {
  cpu.push(0x12020f77); f_120133b4(cpu);
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12020f82); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020f91 => {
  { cpu.sub32(cpu.esi, 0x6du32); }
  if !cpu.zf { pc = 0x12020fa0; continue; }
  pc = 0x12020f96; continue;
 }
 0x12020f96 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020fa0 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12020fa6); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12020fb6 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)).wrapping_add(cpu.eax); cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if cpu.zf { pc = 0x120210af; continue; }
  pc = 0x12020fd0; continue;
 }
 0x12020fd0 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x12021014; continue; }
  pc = 0x12020fd7; continue;
 }
 0x12020fd7 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0xau8); }
  if !cpu.zf { pc = 0x12021014; continue; }
  pc = 0x12020fe0; continue;
 }
 0x12020fe0 => {
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x12020fe3; continue;
 }
 0x12020fe3 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if !cpu.cf { pc = 0x120210a7; continue; }
  pc = 0x12020ffd; continue;
 }
 0x12020ffd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.zf { pc = 0x120210f0; continue; }
  pc = 0x12021007; continue;
 }
 0x12021007 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x12021019; continue; }
  pc = 0x1202100b; continue;
 }
 0x1202100b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1202100e; continue;
 }
 0x1202100e => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1202109d; continue;
 }
 0x12021014 => {
  { let v: u8 = (cpu.ecx as u8) & 0xfbu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x12020fe3; continue;
 }
 0x12021019 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x12021038; continue; }
  pc = 0x12021022; continue;
 }
 0x12021022 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xau8); }
  if !cpu.zf { pc = 0x12021032; continue; }
  pc = 0x1202102a; continue;
 }
 0x1202102a => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = 0xau8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1202100e; continue;
 }
 0x12021032 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1202100e; continue;
 }
 0x12021038 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1204101cu32); cpu.push(0x1202105c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1202106a; continue; }
  pc = 0x12021060; continue;
 }
 0x12021060 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12021066); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x1202106a; continue;
 }
 0x1202106a => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x120210e8; continue; }
  pc = 0x12021071; continue;
 }
 0x12021071 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x120210e8; continue; }
  pc = 0x12021078; continue;
 }
 0x12021078 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x48u8); }
  if cpu.zf { pc = 0x120210bb; continue; }
  pc = 0x12021082; continue;
 }
 0x12021082 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esp)), 0xau8); }
  if !cpu.zf { pc = 0x1202108e; continue; }
  pc = 0x12021089; continue;
 }
 0x12021089 => {
  { let v: u8 = 0xau8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1202100e; continue;
 }
 0x1202108e => {
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x1202109d; continue;
 }
 0x1202109d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x1cu32.wrapping_add(cpu.esp))); }
  if cpu.cf { pc = 0x12020ffd; continue; }
  pc = 0x120210a7; continue;
 }
 0x120210a7 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x28u32.wrapping_add(cpu.esp))); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x120210af; continue;
 }
 0x120210af => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120210bb => {
  { cpu.sub32(cpu.edi, cpu.r32(0x28u32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x120210d0; continue; }
  pc = 0x120210c1; continue;
 }
 0x120210c1 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esp)), 0xau8); }
  if !cpu.zf { pc = 0x120210d0; continue; }
  pc = 0x120210c8; continue;
 }
 0x120210c8 => {
  { let v: u8 = 0xau8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1202100e; continue;
 }
 0x120210d0 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120210de); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esp)), 0xau8); }
  if cpu.zf { pc = 0x1202109d; continue; }
  pc = 0x120210e8; continue;
 }
 0x120210e8 => {
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1202100e; continue;
 }
 0x120210f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x120210a7; continue; }
  pc = 0x120210fe; continue;
 }
 0x120210fe => {
  { let v: u8 = (cpu.ecx as u8) | 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x120210a7; continue;
 }
 0x12021105 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
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
pub fn f_1202110f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202110f); }
 let mut pc: u32 = 0x1202110f;
 loop { match pc {
 0x1202110f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1203f010u32)); }
  if !cpu.cf { pc = 0x12021164; continue; }
  pc = 0x1202111d; continue;
 }
 0x1202111d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u8); }
  if cpu.zf { pc = 0x12021164; continue; }
  pc = 0x1202113d; continue;
 }
 0x1202113d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12021143); f_12016b72(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12021151); f_12021177(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1202115c); f_12016bd3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021164 => {
  cpu.push(0x12021169); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021177(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021177); }
 let mut pc: u32 = 0x12021177;
 loop { match pc {
 0x12021177 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203ef10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8000u32); }
  if !cpu.zf { pc = 0x120211c2; continue; }
  pc = 0x120211ac; continue;
 }
 0x120211ac => {
  { let v: u8 = (cpu.ebx as u8) & 0x7fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x120211af; continue;
 }
 0x120211af => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x4000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4000u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120211c2 => {
  { cpu.sub32(cpu.eax, 0x4000u32); }
  if !cpu.zf { pc = 0x120211ce; continue; }
  pc = 0x120211c9; continue;
 }
 0x120211c9 => {
  { let v: u8 = (cpu.ebx as u8) | 0x80u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x120211af; continue;
 }
 0x120211ce => {
  cpu.push(0x120211d3); f_120133b4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120211e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120211e0); }
 let mut pc: u32 = 0x120211e0;
 loop { match pc {
 0x120211e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120211ee); f_12016706(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120211f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120211f2); }
 let mut pc: u32 = 0x120211f2;
 loop { match pc {
 0x120211f2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12021202); f_1202050c(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021206(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021206); }
 let mut pc: u32 = 0x12021206;
 loop { match pc {
 0x12021206 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x302u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1202121a); f_120212ee(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202121e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202121e); }
 let mut pc: u32 = 0x1202121e;
 loop { match pc {
 0x1202121e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x1202122f); f_1201292c(cpu);
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.esi as u16)); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  if cpu.zf { pc = 0x1202123f; continue; }
  pc = 0x12021236; continue;
 }
 0x12021236 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0x0u16); }
  if !cpu.zf { pc = 0x12021236; continue; }
  pc = 0x1202123f; continue;
 }
 0x1202123f => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x2u32); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0x58u16); }
  if !cpu.zf { pc = 0x120212e7; continue; }
  pc = 0x1202124c; continue;
 }
 0x1202124c => {
  { cpu.sub32(cpu.esi, 0x5u32); }
  if (cpu.sf == cpu.of) { pc = 0x12021275; continue; }
  pc = 0x12021251; continue;
 }
 0x12021251 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0xau32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x30u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0x58u16); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  if cpu.zf { pc = 0x1202124c; continue; }
  pc = 0x12021275; continue;
 }
 0x12021275 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0x58u16); }
  if !cpu.zf { pc = 0x120212e7; continue; }
  pc = 0x1202127b; continue;
 }
 0x1202127b => {
  { cpu.sub32(cpu.esi, 0x5u32); }
  if (cpu.sf != cpu.of) { pc = 0x120212e7; continue; }
  pc = 0x12021280; continue;
 }
 0x12021280 => {
  { let v: u16 = 0x61u16; cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x62u32; cpu.esi = v; }
  cpu.push(0x1202128f); f_120133b4(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  cpu.push(0x12021296); f_120133b4(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1202129c; continue;
 }
 0x1202129c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120212a4); f_1201b6ca(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120212b5; continue; }
  pc = 0x120212ab; continue;
 }
 0x120212ab => {
  cpu.push(0x120212b0); f_120133b4(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0xdu32); }
  if !cpu.zf { pc = 0x120212cb; continue; }
  pc = 0x120212b5; continue;
 }
 0x120212b5 => {
  cpu.push(0x120212ba); f_120133b4(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, 0x7bu32); }
  if cpu.zf { pc = 0x120212d9; continue; }
  pc = 0x120212c5; continue;
 }
 0x120212c5 => {
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1202129c; continue;
 }
 0x120212cb => {
  cpu.push(0x120212d0); f_120133b4(cpu);
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120212d9 => {
  cpu.push(0x120212de); f_120133b4(cpu);
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120212e7 => {
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
pub fn f_120212ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120212ee); }
 let mut pc: u32 = 0x120212ee;
 loop { match pc {
 0x120212ee => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12021304); f_12021308(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021308(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021308); }
 let mut pc: u32 = 0x12021308;
 loop { match pc {
 0x12021308 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xcu32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.and32(cpu.esi, 0x80u32); }
  if cpu.zf { pc = 0x12021382; continue; }
  pc = 0x12021329; continue;
 }
 0x12021329 => {
  { let v: u8 = 0x10u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x1202132f; continue;
 }
 0x1202132f => {
  { cpu.and32(cpu.esi, 0x8000u32); }
  if !cpu.zf { pc = 0x1202134e; continue; }
  pc = 0x12021337; continue;
 }
 0x12021337 => {
  { cpu.and32(cpu.esi, 0x4000u32); }
  if !cpu.zf { pc = 0x1202134b; continue; }
  pc = 0x1202133f; continue;
 }
 0x1202133f => {
  { cpu.sub32(cpu.r32(0x1203bc84u32), 0x8000u32); }
  if cpu.zf { pc = 0x1202134e; continue; }
  pc = 0x1202134b; continue;
 }
 0x1202134b => {
  { let v: u8 = (cpu.ebx as u8) | 0x80u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1202134e; continue;
 }
 0x1202134e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  if cpu.zf { pc = 0x1202138e; continue; }
  pc = 0x12021355; continue;
 }
 0x12021355 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x120213b1; continue; }
  pc = 0x1202135a; continue;
 }
 0x1202135a => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x120213bb; continue; }
  pc = 0x1202135f; continue;
 }
 0x1202135f => {
  cpu.push(0x12021364); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1202136f); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021382 => {
  { let v: u32 = 0x1u32; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1202132f; continue;
 }
 0x1202138e => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x12021396; continue;
 }
 0x12021396 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120213c5; continue; }
  pc = 0x120213a2; continue;
 }
 0x120213a2 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x12021758u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = cpu.r32(0x12021744u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x120213b1 => {
  { let v: u32 = 0x40000000u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x12021396; continue;
 }
 0x120213bb => {
  { let v: u32 = 0xc0000000u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x12021396; continue;
 }
 0x120213c5 => {
  cpu.push(0x120213ca); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120213d5); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120213e8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x120213f0; continue;
 }
 0x120213f0 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x700u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12021449; continue; }
  pc = 0x120213fe; continue;
 }
 0x120213fe => {
  if cpu.zf { pc = 0x1202152c; continue; }
  pc = 0x12021404; continue;
 }
 0x12021404 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1202147f; continue; }
  pc = 0x12021408; continue;
 }
 0x12021408 => {
  cpu.push(0x1202140d); f_120133b4(cpu);
  { let v: u32 = 0x16u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x12021418); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202142b => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x120213f0; continue;
 }
 0x12021435 => {
  { let v: u32 = 0x2u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x120213f0; continue;
 }
 0x1202143f => {
  { let v: u32 = 0x3u32; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x120213f0; continue;
 }
 0x12021449 => {
  { cpu.sub32(cpu.eax, 0x300u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12021467; continue; }
  pc = 0x12021450; continue;
 }
 0x12021450 => {
  if cpu.zf { pc = 0x12021539; continue; }
  pc = 0x12021456; continue;
 }
 0x12021456 => {
  { cpu.sub32(cpu.eax, 0x200u32); }
  if !cpu.zf { pc = 0x12021408; continue; }
  pc = 0x1202145d; continue;
 }
 0x1202145d => {
  { let v: u32 = 0x5u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x12021487; continue;
 }
 0x12021467 => {
  { cpu.sub32(cpu.eax, 0x500u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12021509; continue; }
  pc = 0x12021472; continue;
 }
 0x12021472 => {
  if cpu.zf { pc = 0x1202151f; continue; }
  pc = 0x12021478; continue;
 }
 0x12021478 => {
  { cpu.sub32(cpu.eax, 0x400u32); }
  if !cpu.zf { pc = 0x12021408; continue; }
  pc = 0x1202147f; continue;
 }
 0x1202147f => {
  { let v: u32 = 0x3u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x12021487; continue;
 }
 0x12021487 => {
  { cpu.and32(cpu.esi, 0x100u32); }
  { let v: u32 = 0x80u32; cpu.ebp = v; }
  if cpu.zf { pc = 0x120214a8; continue; }
  pc = 0x12021494; continue;
 }
 0x12021494 => {
  { let v: u32 = cpu.r32(0x1203b100u32); cpu.eax = v; }
  { let v: u32 = !cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.ebp, cpu.eax); }
  if !cpu.zf { pc = 0x120214a8; continue; }
  pc = 0x120214a3; continue;
 }
 0x120214a3 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x120214a8; continue;
 }
 0x120214a8 => {
  { cpu.and32(cpu.esi, 0x40u32); }
  if cpu.zf { pc = 0x120214be; continue; }
  pc = 0x120214b0; continue;
 }
 0x120214b0 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)) | 0x10000u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp | 0x4000000u32; cpu.ebp = v; }
  pc = 0x120214be; continue;
 }
 0x120214be => {
  { cpu.and32(cpu.esi, 0x1000u32); }
  if cpu.zf { pc = 0x120214cc; continue; }
  pc = 0x120214c6; continue;
 }
 0x120214c6 => {
  { let v: u32 = cpu.ebp | 0x100u32; cpu.ebp = v; }
  pc = 0x120214cc; continue;
 }
 0x120214cc => {
  { cpu.and32(cpu.esi, 0x20u32); }
  if cpu.zf { pc = 0x12021546; continue; }
  pc = 0x120214d4; continue;
 }
 0x120214d4 => {
  { let v: u32 = cpu.ebp | 0x8000000u32; cpu.ebp = v; }
  pc = 0x120214da; continue;
 }
 0x120214da => {
  cpu.push(0x120214df); f_12016800(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  if !cpu.zf { pc = 0x12021556; continue; }
  pc = 0x120214e6; continue;
 }
 0x120214e6 => {
  cpu.push(0x120214eb); f_120133b4(cpu);
  { let v: u32 = 0x18u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x120214f6); f_120133bd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021509 => {
  { cpu.sub32(cpu.eax, 0x600u32); }
  if cpu.zf { pc = 0x1202145d; continue; }
  pc = 0x12021514; continue;
 }
 0x12021514 => {
  { cpu.sub32(cpu.eax, 0x700u32); }
  if !cpu.zf { pc = 0x12021408; continue; }
  pc = 0x1202151f; continue;
 }
 0x1202151f => {
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x12021487; continue;
 }
 0x1202152c => {
  { let v: u32 = 0x4u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x12021487; continue;
 }
 0x12021539 => {
  { let v: u32 = 0x2u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x12021487; continue;
 }
 0x12021546 => {
  { cpu.and32(cpu.esi, 0x10u32); }
  if cpu.zf { pc = 0x120214da; continue; }
  pc = 0x1202154e; continue;
 }
 0x1202154e => {
  { let v: u32 = cpu.ebp | 0x10000000u32; cpu.ebp = v; }
  pc = 0x120214da; continue;
 }
 0x12021556 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12041018u32); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.ebp; cpu.push(0x1202157a); cpu.call(t);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  if !cpu.zf { pc = 0x120215a6; continue; }
  pc = 0x12021581; continue;
 }
 0x12021581 => {
  let t = cpu.r32(0x120411ccu32); cpu.push(0x12021587); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1202158d); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12021596); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120215a6 => {
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x12041170u32); cpu.push(0x120215ad); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120215dd; continue; }
  pc = 0x120215b1; continue;
 }
 0x120215b1 => {
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x120411bcu32); cpu.push(0x120215b8); cpu.call(t);
  let t = cpu.r32(0x120411ccu32); cpu.push(0x120215be); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120215c4); f_12013341(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120215cd); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120215dd => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x12021681; continue; }
  pc = 0x120215e6; continue;
 }
 0x120215e6 => {
  { let v: u8 = (cpu.ebx as u8) | 0x40u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x120215e9; continue;
 }
 0x120215e9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120215f3); f_12016941(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffffe7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x1203ef10u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x48u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  if !cpu.zf { pc = 0x12021714; continue; }
  pc = 0x12021628; continue;
 }
 0x12021628 => {
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if cpu.zf { pc = 0x12021714; continue; }
  pc = 0x12021631; continue;
 }
 0x12021631 => {
  { cpu.and32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x12021714; continue; }
  pc = 0x1202163d; continue;
 }
 0x1202163d => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12021647); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12021692; continue; }
  pc = 0x12021651; continue;
 }
 0x12021651 => {
  cpu.push(0x12021656); f_120133bd(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x83u32); }
  if cpu.zf { pc = 0x12021714; continue; }
  pc = 0x12021662; continue;
 }
 0x12021662 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12021668); f_1201660f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12021671); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021681 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x120215e9; continue; }
  pc = 0x1202168a; continue;
 }
 0x1202168a => {
  { let v: u8 = (cpu.ebx as u8) | 0x8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x120215e9; continue;
 }
 0x12021692 => {
  { let v: u32 = 0x12u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x16u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120216a6); f_12020edf(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120216e3; continue; }
  pc = 0x120216ad; continue;
 }
 0x120216ad => {
  { cpu.sub16(cpu.r16(0x12u32.wrapping_add(cpu.esp)), 0x1au16); }
  if !cpu.zf { pc = 0x120216e3; continue; }
  pc = 0x120216b5; continue;
 }
 0x120216b5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120216bc); f_1201f1c4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x120216e3; continue; }
  pc = 0x120216c4; continue;
 }
 0x120216c4 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120216ca); f_1201660f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120216d3); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120216e3 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120216ed); f_1201677e(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x12021714; continue; }
  pc = 0x120216f5; continue;
 }
 0x120216f5 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x120216fb); f_1201660f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12021704); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021714 => {
  { cpu.sub8(cpu.r8(0x14u32.wrapping_add(cpu.esp)), 0x0u8); }
  if !cpu.zf { pc = 0x1202172f; continue; }
  pc = 0x1202171b; continue;
 }
 0x1202171b => {
  { cpu.and32(cpu.esi, 0x8u32); }
  if cpu.zf { pc = 0x1202172f; continue; }
  pc = 0x12021723; continue;
 }
 0x12021723 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))) | 0x20u8; cpu.w8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x1202172f; continue;
 }
 0x1202172f => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12021735); f_12016bd3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021744(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021744); }
 let mut pc: u32 = 0x12021744;
 loop { match pc {
 0x12021744 => {
  cpu.push(0x12021749); cpu.call(0x3d14195c);
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(((cpu.edx >> 8) as u8), cpu.r8(0x3f120214u32)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
  f_12021789(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021758(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021758); }
 let mut pc: u32 = 0x12021758;
 loop { match pc {
 0x12021758 => {
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
  f_12021789(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021789(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021789); }
 let mut pc: u32 = 0x12021789;
 loop { match pc {
 0x12021789 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0xc0000000u32; cpu.push(v); }
  { let v = 0x120366b4u32; cpu.push(v); }
  let t = cpu.r32(0x12041024u32); cpu.push(0x120217a3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203d410u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120217a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120217a9); }
 let mut pc: u32 = 0x120217a9;
 loop { match pc {
 0x120217a9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x40000000u32; cpu.push(v); }
  { let v = 0x120366bcu32; cpu.push(v); }
  let t = cpu.r32(0x12041024u32); cpu.push(0x120217c3); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1203d414u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120217c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120217c9); }
 let mut pc: u32 = 0x120217c9;
 loop { match pc {
 0x120217c9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120217d7); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120217db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120217db); }
 let mut pc: u32 = 0x120217db;
 loop { match pc {
 0x120217db => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120217e9); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120217ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120217ed); }
 let mut pc: u32 = 0x120217ed;
 loop { match pc {
 0x120217ed => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120217fb); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120217ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120217ff); }
 let mut pc: u32 = 0x120217ff;
 loop { match pc {
 0x120217ff => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x107u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12021810); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021814(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021814); }
 let mut pc: u32 = 0x12021814;
 loop { match pc {
 0x12021814 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x103u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12021825); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021829(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021829); }
 let mut pc: u32 = 0x12021829;
 loop { match pc {
 0x12021829 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x117u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1202183a); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202183e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202183e); }
 let mut pc: u32 = 0x1202183e;
 loop { match pc {
 0x1202183e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x157u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1202184f); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021853(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021853); }
 let mut pc: u32 = 0x12021853;
 loop { match pc {
 0x12021853 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12021861); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021865(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021865); }
 let mut pc: u32 = 0x12021865;
 loop { match pc {
 0x12021865 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12021873); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021877(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021877); }
 let mut pc: u32 = 0x12021877;
 loop { match pc {
 0x12021877 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12021885); f_120218b3(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021889(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021889); }
 let mut pc: u32 = 0x12021889;
 loop { match pc {
 0x12021889 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  if !cpu.zf { pc = 0x120218b0; continue; }
  pc = 0x12021895; continue;
 }
 0x12021895 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120218a3); f_120218b3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120218b0; continue; }
  pc = 0x120218aa; continue;
 }
 0x120218aa => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120218b0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120218b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120218b3); }
 let mut pc: u32 = 0x120218b3;
 loop { match pc {
 0x120218b3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1203bc89u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.esp)), cpu.ecx); }
  if !cpu.zf { pc = 0x120218e7; continue; }
  pc = 0x120218c7; continue;
 }
 0x120218c7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120218e0; continue; }
  pc = 0x120218d4; continue;
 }
 0x120218d4 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1203bf12u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & cpu.eax; cpu.ecx = v; }
  pc = 0x120218e0; continue;
 }
 0x120218e0 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x120218e7; continue; }
  pc = 0x120218e4; continue;
 }
 0x120218e4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120218e7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120218ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120218ed); }
 let mut pc: u32 = 0x120218ed;
 loop { match pc {
 0x120218ed => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12021961; continue; }
  pc = 0x120218fc; continue;
 }
 0x120218fc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.zf { pc = 0x12021985; continue; }
  pc = 0x1202191a; continue;
 }
 0x1202191a => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12021934); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12021941; continue; }
  pc = 0x1202193b; continue;
 }
 0x1202193b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021941 => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x1202195b; continue; }
  pc = 0x12021949; continue;
 }
 0x12021949 => {
  { cpu.and16(cpu.r16(0x4u32.wrapping_add(cpu.esp)), 0x107u16); }
  if cpu.zf { pc = 0x1202195b; continue; }
  pc = 0x12021952; continue;
 }
 0x12021952 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202195b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021961 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and16(cpu.r16(0x1203bf12u32.wrapping_add(cpu.eax.wrapping_mul(2))), 0x107u16); }
  if !cpu.zf { pc = 0x12021980; continue; }
  pc = 0x12021971; continue;
 }
 0x12021971 => {
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x12021980; continue; }
  pc = 0x1202197a; continue;
 }
 0x1202197a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021980 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12021985; continue;
 }
 0x12021985 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021989); }
 let mut pc: u32 = 0x12021989;
 loop { match pc {
 0x12021989 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x120219fd; continue; }
  pc = 0x12021998; continue;
 }
 0x12021998 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.zf { pc = 0x12021a21; continue; }
  pc = 0x120219b6; continue;
 }
 0x120219b6 => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x120219d0); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120219dd; continue; }
  pc = 0x120219d7; continue;
 }
 0x120219d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120219dd => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x120219f7; continue; }
  pc = 0x120219e5; continue;
 }
 0x120219e5 => {
  { cpu.and16(cpu.r16(0x4u32.wrapping_add(cpu.esp)), 0x103u16); }
  if cpu.zf { pc = 0x120219f7; continue; }
  pc = 0x120219ee; continue;
 }
 0x120219ee => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120219f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120219fd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and16(cpu.r16(0x1203bf12u32.wrapping_add(cpu.eax.wrapping_mul(2))), 0x103u16); }
  if !cpu.zf { pc = 0x12021a1c; continue; }
  pc = 0x12021a0d; continue;
 }
 0x12021a0d => {
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x12021a1c; continue; }
  pc = 0x12021a16; continue;
 }
 0x12021a16 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021a1c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12021a21; continue;
 }
 0x12021a21 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021a25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021a25); }
 let mut pc: u32 = 0x12021a25;
 loop { match pc {
 0x12021a25 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.edx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12021a9b; continue; }
  pc = 0x12021a34; continue;
 }
 0x12021a34 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x1203bd8cu32), cpu.eax); }
  if cpu.zf { pc = 0x12021abf; continue; }
  pc = 0x12021a50; continue;
 }
 0x12021a50 => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12021a70); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12021a7d; continue; }
  pc = 0x12021a77; continue;
 }
 0x12021a77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021a7d => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x12021a95; continue; }
  pc = 0x12021a85; continue;
 }
 0x12021a85 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x12021a95; continue; }
  pc = 0x12021a8c; continue;
 }
 0x12021a8c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021a95 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021a9b => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12021ab0; continue; }
  pc = 0x12021aa4; continue;
 }
 0x12021aa4 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12021aac); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021ab0 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  pc = 0x12021abf; continue;
 }
 0x12021abf => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021ac3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021ac3); }
 let mut pc: u32 = 0x12021ac3;
 loop { match pc {
 0x12021ac3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12021b37; continue; }
  pc = 0x12021ad2; continue;
 }
 0x12021ad2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.zf { pc = 0x12021b5b; continue; }
  pc = 0x12021af0; continue;
 }
 0x12021af0 => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12021b0a); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12021b17; continue; }
  pc = 0x12021b11; continue;
 }
 0x12021b11 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021b17 => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x12021b31; continue; }
  pc = 0x12021b1f; continue;
 }
 0x12021b1f => {
  { cpu.and16(cpu.r16(0x4u32.wrapping_add(cpu.esp)), 0x117u16); }
  if cpu.zf { pc = 0x12021b31; continue; }
  pc = 0x12021b28; continue;
 }
 0x12021b28 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021b31 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021b37 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and16(cpu.r16(0x1203bf12u32.wrapping_add(cpu.eax.wrapping_mul(2))), 0x117u16); }
  if !cpu.zf { pc = 0x12021b56; continue; }
  pc = 0x12021b47; continue;
 }
 0x12021b47 => {
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x12021b56; continue; }
  pc = 0x12021b50; continue;
 }
 0x12021b50 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021b56 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12021b5b; continue;
 }
 0x12021b5b => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021b5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021b5f); }
 let mut pc: u32 = 0x12021b5f;
 loop { match pc {
 0x12021b5f => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  if !cpu.zf { pc = 0x12021b83; continue; }
  pc = 0x12021b6b; continue;
 }
 0x12021b6b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x829fu32); }
  if cpu.cf { pc = 0x12021b83; continue; }
  pc = 0x12021b76; continue;
 }
 0x12021b76 => {
  { cpu.sub32(cpu.eax, 0x82f1u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12021b83; continue; }
  pc = 0x12021b7d; continue;
 }
 0x12021b7d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021b83 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021b86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021b86); }
 let mut pc: u32 = 0x12021b86;
 loop { match pc {
 0x12021b86 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  if !cpu.zf { pc = 0x12021bb1; continue; }
  pc = 0x12021b92; continue;
 }
 0x12021b92 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8340u32); }
  if cpu.cf { pc = 0x12021bb1; continue; }
  pc = 0x12021b9d; continue;
 }
 0x12021b9d => {
  { cpu.sub32(cpu.eax, 0x8396u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12021bb1; continue; }
  pc = 0x12021ba4; continue;
 }
 0x12021ba4 => {
  { cpu.sub32(cpu.eax, 0x837fu32); }
  if cpu.zf { pc = 0x12021bb1; continue; }
  pc = 0x12021bab; continue;
 }
 0x12021bab => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021bb1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021bb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021bb4); }
 let mut pc: u32 = 0x12021bb4;
 loop { match pc {
 0x12021bb4 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  if !cpu.zf { pc = 0x12021bdf; continue; }
  pc = 0x12021bc0; continue;
 }
 0x12021bc0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8141u32); }
  if cpu.cf { pc = 0x12021bdf; continue; }
  pc = 0x12021bcb; continue;
 }
 0x12021bcb => {
  { cpu.sub32(cpu.eax, 0x81acu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12021bdf; continue; }
  pc = 0x12021bd2; continue;
 }
 0x12021bd2 => {
  { cpu.sub32(cpu.eax, 0x817fu32); }
  if cpu.zf { pc = 0x12021bdf; continue; }
  pc = 0x12021bd9; continue;
 }
 0x12021bd9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021bdf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021be2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021be2); }
 let mut pc: u32 = 0x12021be2;
 loop { match pc {
 0x12021be2 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x12021c0f; continue; }
  pc = 0x12021bef; continue;
 }
 0x12021bef => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12021bf9); f_12021c85(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12021c0f; continue; }
  pc = 0x12021c00; continue;
 }
 0x12021c00 => {
  { cpu.sub32(cpu.esi, 0x889fu32); }
  if !cpu.cf { pc = 0x12021c0f; continue; }
  pc = 0x12021c08; continue;
 }
 0x12021c08 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021c0f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021c13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021c13); }
 let mut pc: u32 = 0x12021c13;
 loop { match pc {
 0x12021c13 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x12021c48; continue; }
  pc = 0x12021c20; continue;
 }
 0x12021c20 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12021c2a); f_12021c85(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12021c48; continue; }
  pc = 0x12021c31; continue;
 }
 0x12021c31 => {
  { cpu.sub32(cpu.esi, 0x889fu32); }
  if cpu.cf { pc = 0x12021c48; continue; }
  pc = 0x12021c39; continue;
 }
 0x12021c39 => {
  { cpu.sub32(cpu.esi, 0x9872u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12021c48; continue; }
  pc = 0x12021c41; continue;
 }
 0x12021c41 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021c48 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021c4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021c4c); }
 let mut pc: u32 = 0x12021c4c;
 loop { match pc {
 0x12021c4c => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x12021c81; continue; }
  pc = 0x12021c59; continue;
 }
 0x12021c59 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12021c63); f_12021c85(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12021c81; continue; }
  pc = 0x12021c6a; continue;
 }
 0x12021c6a => {
  { cpu.sub32(cpu.esi, 0x989fu32); }
  if cpu.cf { pc = 0x12021c81; continue; }
  pc = 0x12021c72; continue;
 }
 0x12021c72 => {
  { cpu.sub32(cpu.esi, 0xeaa4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12021c81; continue; }
  pc = 0x12021c7a; continue;
 }
 0x12021c7a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021c81 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021c85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021c85); }
 let mut pc: u32 = 0x12021c85;
 loop { match pc {
 0x12021c85 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x12021ca9; continue; }
  pc = 0x12021c96; continue;
 }
 0x12021c96 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x8u8); }
  if cpu.zf { pc = 0x12021ca9; continue; }
  pc = 0x12021ca3; continue;
 }
 0x12021ca3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021ca9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021cac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021cac); }
 let mut pc: u32 = 0x12021cac;
 loop { match pc {
 0x12021cac => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12021d1e; continue; }
  pc = 0x12021cbb; continue;
 }
 0x12021cbb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.zf { pc = 0x12021d33; continue; }
  pc = 0x12021cd9; continue;
 }
 0x12021cd9 => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12021cf3); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12021d00; continue; }
  pc = 0x12021cfa; continue;
 }
 0x12021cfa => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021d00 => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x12021d18; continue; }
  pc = 0x12021d08; continue;
 }
 0x12021d08 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x2u8); }
  if cpu.zf { pc = 0x12021d18; continue; }
  pc = 0x12021d0f; continue;
 }
 0x12021d0f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021d18 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021d1e => {
  { cpu.sub32(cpu.ecx, 0x61u32); }
  if cpu.cf { pc = 0x12021d31; continue; }
  pc = 0x12021d23; continue;
 }
 0x12021d23 => {
  { cpu.sub32(cpu.ecx, 0x7au32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12021d31; continue; }
  pc = 0x12021d28; continue;
 }
 0x12021d28 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021d31 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x12021d33; continue;
 }
 0x12021d33 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021d37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021d37); }
 let mut pc: u32 = 0x12021d37;
 loop { match pc {
 0x12021d37 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12021dab; continue; }
  pc = 0x12021d46; continue;
 }
 0x12021d46 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.zf { pc = 0x12021dcf; continue; }
  pc = 0x12021d64; continue;
 }
 0x12021d64 => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12021d7e); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12021d8b; continue; }
  pc = 0x12021d85; continue;
 }
 0x12021d85 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021d8b => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x12021da5; continue; }
  pc = 0x12021d93; continue;
 }
 0x12021d93 => {
  { cpu.and16(cpu.r16(0x4u32.wrapping_add(cpu.esp)), 0x157u16); }
  if cpu.zf { pc = 0x12021da5; continue; }
  pc = 0x12021d9c; continue;
 }
 0x12021d9c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021da5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021dab => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and16(cpu.r16(0x1203bf12u32.wrapping_add(cpu.eax.wrapping_mul(2))), 0x157u16); }
  if !cpu.zf { pc = 0x12021dca; continue; }
  pc = 0x12021dbb; continue;
 }
 0x12021dbb => {
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x12021dca; continue; }
  pc = 0x12021dc4; continue;
 }
 0x12021dc4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021dca => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12021dcf; continue;
 }
 0x12021dcf => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021dd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021dd3); }
 let mut pc: u32 = 0x12021dd3;
 loop { match pc {
 0x12021dd3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12021e45; continue; }
  pc = 0x12021de2; continue;
 }
 0x12021de2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.zf { pc = 0x12021e67; continue; }
  pc = 0x12021e00; continue;
 }
 0x12021e00 => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12021e1a); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12021e27; continue; }
  pc = 0x12021e21; continue;
 }
 0x12021e21 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021e27 => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x12021e3f; continue; }
  pc = 0x12021e2f; continue;
 }
 0x12021e2f => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x12021e3f; continue; }
  pc = 0x12021e36; continue;
 }
 0x12021e36 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021e3f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021e45 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bf12u32.wrapping_add(cpu.eax.wrapping_mul(2))), 0x10u8); }
  if !cpu.zf { pc = 0x12021e62; continue; }
  pc = 0x12021e53; continue;
 }
 0x12021e53 => {
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x12021e62; continue; }
  pc = 0x12021e5c; continue;
 }
 0x12021e5c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021e62 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x12021e67; continue;
 }
 0x12021e67 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021e6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021e6b); }
 let mut pc: u32 = 0x12021e6b;
 loop { match pc {
 0x12021e6b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.edx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12021ee1; continue; }
  pc = 0x12021e7a; continue;
 }
 0x12021e7a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x1203bd8cu32), cpu.eax); }
  if cpu.zf { pc = 0x12021f05; continue; }
  pc = 0x12021e96; continue;
 }
 0x12021e96 => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12021eb6); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12021ec3; continue; }
  pc = 0x12021ebd; continue;
 }
 0x12021ebd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021ec3 => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x12021edb; continue; }
  pc = 0x12021ecb; continue;
 }
 0x12021ecb => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x8u8); }
  if cpu.zf { pc = 0x12021edb; continue; }
  pc = 0x12021ed2; continue;
 }
 0x12021ed2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021edb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021ee1 => {
  { cpu.sub32(cpu.r32(0x1203befcu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12021ef6; continue; }
  pc = 0x12021eea; continue;
 }
 0x12021eea => {
  { let v = 0x8u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12021ef2); f_12016124(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021ef6 => {
  { let v: u32 = cpu.r32(0x1203bf08u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x8u32; cpu.eax = v; }
  pc = 0x12021f05; continue;
 }
 0x12021f05 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021f09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021f09); }
 let mut pc: u32 = 0x12021f09;
 loop { match pc {
 0x12021f09 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12021f7b; continue; }
  pc = 0x12021f18; continue;
 }
 0x12021f18 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.zf { pc = 0x12021f90; continue; }
  pc = 0x12021f36; continue;
 }
 0x12021f36 => {
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x12021f50); f_1200337d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12021f5d; continue; }
  pc = 0x12021f57; continue;
 }
 0x12021f57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021f5d => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x12021f75; continue; }
  pc = 0x12021f65; continue;
 }
 0x12021f65 => {
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x12021f75; continue; }
  pc = 0x12021f6c; continue;
 }
 0x12021f6c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021f75 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021f7b => {
  { cpu.sub32(cpu.ecx, 0x41u32); }
  if cpu.cf { pc = 0x12021f8e; continue; }
  pc = 0x12021f80; continue;
 }
 0x12021f80 => {
  { cpu.sub32(cpu.ecx, 0x5au32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12021f8e; continue; }
  pc = 0x12021f85; continue;
 }
 0x12021f85 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021f8e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x12021f90; continue;
 }
 0x12021f90 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12021f94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12021f94); }
 let mut pc: u32 = 0x12021f94;
 loop { match pc {
 0x12021f94 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x12021fa4; continue; }
  pc = 0x12021f9f; continue;
 }
 0x12021f9f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021fa4 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12021fab); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.cf { pc = 0x12021fe2; continue; }
  pc = 0x12021fba; continue;
 }
 0x12021fba => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  pc = 0x12021fbf; continue;
 }
 0x12021fbf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x12021fe2; continue; }
  pc = 0x12021fc5; continue;
 }
 0x12021fc5 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ebx)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12021fdd; continue; }
  pc = 0x12021fd1; continue;
 }
 0x12021fd1 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if cpu.zf { pc = 0x12021ff1; continue; }
  pc = 0x12021fd8; continue;
 }
 0x12021fd8 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x12022003; continue; }
  pc = 0x12021fdd; continue;
 }
 0x12021fdd => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.cf { pc = 0x12021fbf; continue; }
  pc = 0x12021fe2; continue;
 }
 0x12021fe2 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12021fe9); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12021ff1 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12021ff8); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022003 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x1202200a); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12022012(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12022012); }
 let mut pc: u32 = 0x12022012;
 loop { match pc {
 0x12022012 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  if !cpu.zf { pc = 0x12022022; continue; }
  pc = 0x1202201d; continue;
 }
 0x1202201d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022022 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12022029); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.cf { pc = 0x1202205e; continue; }
  pc = 0x12022038; continue;
 }
 0x12022038 => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  pc = 0x1202203d; continue;
 }
 0x1202203d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x1202205e; continue; }
  pc = 0x12022043; continue;
 }
 0x12022043 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ebx)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12022059; continue; }
  pc = 0x1202204f; continue;
 }
 0x1202204f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x1202206d; continue; }
  pc = 0x12022054; continue;
 }
 0x12022054 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1202207f; continue; }
  pc = 0x12022059; continue;
 }
 0x12022059 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.cf { pc = 0x1202203d; continue; }
  pc = 0x1202205e; continue;
 }
 0x1202205e => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12022065); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202206d => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12022074); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202207f => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12022086); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202208e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202208e); }
 let mut pc: u32 = 0x1202208e;
 loop { match pc {
 0x1202208e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  if cpu.zf { pc = 0x120220a1; continue; }
  pc = 0x1202209f; continue;
 }
 0x1202209f => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120220a1 => {
  { cpu.sub32(cpu.eax, 0x20u32); }
  if cpu.cf { pc = 0x120220b9; continue; }
  pc = 0x120220a6; continue;
 }
 0x120220a6 => {
  { cpu.sub32(cpu.eax, 0x7eu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120220b9; continue; }
  pc = 0x120220ab; continue;
 }
 0x120220ab => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1203d3d8u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120220b9 => {
  { cpu.sub32(cpu.eax, 0xdcu32); }
  if !cpu.zf { pc = 0x120220c7; continue; }
  pc = 0x120220c0; continue;
 }
 0x120220c0 => {
  { let v: u32 = 0x838fu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120220c7 => {
  { cpu.sub32(cpu.eax, 0xa1u32); }
  if cpu.cf { pc = 0x12022101; continue; }
  pc = 0x120220ce; continue;
 }
 0x120220ce => {
  { cpu.sub32(cpu.eax, 0xdfu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12022101; continue; }
  pc = 0x120220d5; continue;
 }
 0x120220d5 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1203d4d8u32), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x12022101; continue; }
  pc = 0x120220df; continue;
 }
 0x120220df => {
  { let v: u32 = 0x1203d4d8u32; cpu.edx = v; }
  pc = 0x120220e4; continue;
 }
 0x120220e4 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.zf { pc = 0x120220f7; continue; }
  pc = 0x120220ec; continue;
 }
 0x120220ec => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x120220e4; continue; }
  pc = 0x120220f5; continue;
 }
 0x120220f5 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120220f7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1203d4dau32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x12022101; continue;
 }
 0x12022101 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12022103(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12022103); }
 let mut pc: u32 = 0x12022103;
 loop { match pc {
 0x12022103 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if cpu.zf { pc = 0x1202211c; continue; }
  pc = 0x12022113; continue;
 }
 0x12022113 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202211c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x1203d418u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x12022127; continue;
 }
 0x12022127 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x1202217e; continue; }
  pc = 0x12022130; continue;
 }
 0x12022130 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x1203d4d4u32); }
  if (cpu.cf || cpu.zf) { pc = 0x12022127; continue; }
  pc = 0x1202213c; continue;
 }
 0x1202213c => {
  { cpu.sub32(cpu.eax, 0x8396u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1202219d; continue; }
  pc = 0x12022143; continue;
 }
 0x12022143 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x1203d4d8u32), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1202219d; continue; }
  pc = 0x1202214d; continue;
 }
 0x1202214d => {
  { let v: u32 = 0x1203d4dau32; cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x12022154; continue;
 }
 0x12022154 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.ebp); cpu.edi = v; }
  if cpu.zf { pc = 0x12022186; continue; }
  pc = 0x12022160; continue;
 }
 0x12022160 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x12022170; continue; }
  pc = 0x12022164; continue;
 }
 0x12022164 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x12022170; continue; }
  pc = 0x12022168; continue;
 }
 0x12022168 => {
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.ecx); cpu.edi = v; }
  if cpu.sf { pc = 0x12022194; continue; }
  pc = 0x12022170; continue;
 }
 0x12022170 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0xfffffffeu32.wrapping_add(cpu.esi)), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x12022154; continue; }
  pc = 0x12022179; continue;
 }
 0x12022179 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202217e => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022186 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x1203d4d8u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022194 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1203d4d8u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1202219d; continue;
 }
 0x1202219d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120221a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120221a2); }
 let mut pc: u32 = 0x120221a2;
 loop { match pc {
 0x120221a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x120221fa; continue; }
  pc = 0x120221ac; continue;
 }
 0x120221ac => {
  pc = cpu.r32(0x120221fcu32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x120221b3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1203bc89u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x120221c9; continue; }
  pc = 0x120221c3; continue;
 }
 0x120221c3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120221c9 => {
  { cpu.and16(cpu.r16(0x1203bf12u32.wrapping_add(cpu.ecx.wrapping_mul(2))), 0x157u16); }
  if !cpu.zf { pc = 0x120221df; continue; }
  pc = 0x120221d5; continue;
 }
 0x120221d5 => {
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  if !cpu.zf { pc = 0x120221df; continue; }
  pc = 0x120221d9; continue;
 }
 0x120221d9 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120221df => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120221e2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffffdu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x120221fa; continue;
 }
 0x120221fa => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120221fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120221fc); }
 let mut pc: u32 = 0x120221fc;
 loop { match pc {
 0x120221fc => {
  { let v: u8 = 0x21u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x21u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x12022227; continue; }
  pc = 0x12022206; continue;
 }
 0x12022206 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x21u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  f_1202220c(cpu); return;
 }
 0x12022227 => {
  { let v: u32 = cpu.r32(0x81c30148u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ecx); cpu.w32(0x81c30148u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, 0x1203bd8cu32); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffff8bu32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202220c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202220c); }
 let mut pc: u32 = 0x1202220c;
 loop { match pc {
 0x1202220c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.edx)), 0x4u8); }
  if cpu.zf { pc = 0x1202222b; continue; }
  pc = 0x12022225; continue;
 }
 0x12022225 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  pc = 0x1202222b; continue;
 }
 0x1202222b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202222c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202222c); }
 let mut pc: u32 = 0x1202222c;
 loop { match pc {
 0x1202222c => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  if cpu.zf { pc = 0x1202223d; continue; }
  pc = 0x12022238; continue;
 }
 0x12022238 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202223d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x21u32); }
  if cpu.cf { pc = 0x12022292; continue; }
  pc = 0x12022252; continue;
 }
 0x12022252 => {
  { cpu.sub32(cpu.eax, 0x7eu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12022292; continue; }
  pc = 0x12022257; continue;
 }
 0x12022257 => {
  { cpu.sub32(cpu.ecx, 0x21u32); }
  if cpu.cf { pc = 0x12022292; continue; }
  pc = 0x1202225c; continue;
 }
 0x1202225c => {
  { cpu.sub32(cpu.ecx, 0x7eu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12022292; continue; }
  pc = 0x12022261; continue;
 }
 0x12022261 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1202228d; continue; }
  pc = 0x12022265; continue;
 }
 0x12022265 => {
  { cpu.sub32(cpu.ecx, 0x5fu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12022288; continue; }
  pc = 0x1202226a; continue;
 }
 0x1202226a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1fu32); cpu.ecx = v; }
  pc = 0x1202226d; continue;
 }
 0x1202226d => {
  { let v: u32 = cpu.eax.wrapping_sub(0x21u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x81u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x9fu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12022282; continue; }
  pc = 0x1202227f; continue;
 }
 0x1202227f => {
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  pc = 0x12022282; continue;
 }
 0x12022282 => {
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022288 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x20u32); cpu.ecx = v; }
  pc = 0x1202226d; continue;
 }
 0x1202228d => {
  { let v: u32 = cpu.ecx.wrapping_add(0x7eu32); cpu.ecx = v; }
  pc = 0x1202226d; continue;
 }
 0x12022292 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12022295(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12022295); }
 let mut pc: u32 = 0x12022295;
 loop { match pc {
 0x12022295 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x3a4u32); }
  if cpu.zf { pc = 0x120222a6; continue; }
  pc = 0x120222a1; continue;
 }
 0x120222a1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120222a6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ecx)), 0x4u8); }
  if cpu.zf { pc = 0x12022337; continue; }
  pc = 0x120222c3; continue;
 }
 0x120222c3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x12022337; continue; }
  pc = 0x120222d0; continue;
 }
 0x120222d0 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffffc0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xc1u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x9fu32); }
  if cpu.cf { pc = 0x12022318; continue; }
  pc = 0x120222ea; continue;
 }
 0x120222ea => {
  { let v: u32 = cpu.eax.wrapping_add(0x11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x7eu32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 9u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  pc = 0x120222f5; continue;
 }
 0x120222f5 => {
  { cpu.sub32(cpu.eax, 0x7e7eu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12022315; continue; }
  pc = 0x120222fc; continue;
 }
 0x120222fc => {
  { cpu.sub32(cpu.eax, 0x2121u32); }
  if cpu.cf { pc = 0x12022315; continue; }
  pc = 0x12022303; continue;
 }
 0x12022303 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x7eu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12022315; continue; }
  pc = 0x12022310; continue;
 }
 0x12022310 => {
  { cpu.sub32(cpu.ecx, 0x21u32); }
  if !cpu.cf { pc = 0x12022339; continue; }
  pc = 0x12022315; continue;
 }
 0x12022315 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022318 => {
  { let v: u32 = 0x7eu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 9u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2100u32); cpu.eax = v; }
  { let v: u32 = cpu.edx | cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x120222f5; continue;
 }
 0x12022337 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x12022339; continue;
 }
 0x12022339 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202233a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202233a); }
 let mut pc: u32 = 0x1202233a;
 loop { match pc {
 0x1202233a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1203bc89u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12022352(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12022352); }
 let mut pc: u32 = 0x12022352;
 loop { match pc {
 0x12022352 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1202235d); f_12021b86(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12022384; continue; }
  pc = 0x12022364; continue;
 }
 0x12022364 => {
  { cpu.sub32(cpu.esi, 0x8393u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x12022384; continue; }
  pc = 0x1202236c; continue;
 }
 0x1202236c => {
  { cpu.sub32(cpu.esi, 0x837fu32); }
  if !cpu.cf { pc = 0x1202237e; continue; }
  pc = 0x12022374; continue;
 }
 0x12022374 => {
  { let v: u32 = cpu.sub32(cpu.esi, 0xa1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202237e => {
  { let v: u32 = cpu.sub32(cpu.esi, 0xa2u32); cpu.esi = v; }
  pc = 0x12022384; continue;
 }
 0x12022384 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12022388(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12022388); }
 let mut pc: u32 = 0x12022388;
 loop { match pc {
 0x12022388 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x12022393); f_12021b5f(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x120223a9; continue; }
  pc = 0x1202239a; continue;
 }
 0x1202239a => {
  { let v: u32 = cpu.esi.wrapping_add(0xa1u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x837fu32); }
  if cpu.cf { pc = 0x120223a9; continue; }
  pc = 0x120223a8; continue;
 }
 0x120223a8 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x120223a9; continue;
 }
 0x120223a9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120223ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120223ad); }
 let mut pc: u32 = 0x120223ad;
 loop { match pc {
 0x120223ad => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x12022421; continue; }
  pc = 0x120223bd; continue;
 }
 0x120223bd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x7u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x120223d9; continue; }
  pc = 0x120223d2; continue;
 }
 0x120223d2 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120223d9 => {
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120223fd); f_120170bd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1202240b; continue; }
  pc = 0x12022404; continue;
 }
 0x12022404 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202240b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022421 => {
  { cpu.sub32(cpu.ebx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x12022433; continue; }
  pc = 0x12022426; continue;
 }
 0x12022426 => {
  { cpu.sub32(cpu.ebx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x12022433; continue; }
  pc = 0x1202242b; continue;
 }
 0x1202242b => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022433 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202243a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202243a); }
 let mut pc: u32 = 0x1202243a;
 loop { match pc {
 0x1202243a => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x120224ae; continue; }
  pc = 0x1202244a; continue;
 }
 0x1202244a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x7u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x12022466; continue; }
  pc = 0x1202245f; continue;
 }
 0x1202245f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022466 => {
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x200u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1202248a); f_120170bd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x12022498; continue; }
  pc = 0x12022491; continue;
 }
 0x12022491 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022498 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120224ae => {
  { cpu.sub32(cpu.ebx, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x120224c0; continue; }
  pc = 0x120224b3; continue;
 }
 0x120224b3 => {
  { cpu.sub32(cpu.ebx, 0x7au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x120224c0; continue; }
  pc = 0x120224b8; continue;
 }
 0x120224b8 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120224c0 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120224c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120224c7); }
 let mut pc: u32 = 0x120224c7;
 loop { match pc {
 0x120224c7 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x120224d9; continue; }
  pc = 0x120224d3; continue;
 }
 0x120224d3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x120224d9 => {
  { let v = 0x19u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  cpu.push(0x120224e5); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x120224f0; continue;
 }
 0x120224f0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1202251a; continue; }
  pc = 0x120224f6; continue;
 }
 0x120224f6 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x120224fe); f_120221a2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x120224f0; continue; }
  pc = 0x1202250a; continue;
 }
 0x1202250a => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12022511); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202251a => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12022521); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202252d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202252d); }
 let mut pc: u32 = 0x1202252d;
 loop { match pc {
 0x1202252d => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x12022554; continue; }
  pc = 0x1202253e; continue;
 }
 0x1202253e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12022624; continue; }
  pc = 0x12022549; continue;
 }
 0x12022549 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.ecx, 0x3u32); }
  if !cpu.zf { pc = 0x1202253e; continue; }
  pc = 0x12022554; continue;
 }
 0x12022554 => {
  { let v: u32 = 0x7efefeffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, 0x81010100u32); }
  if cpu.zf { pc = 0x12022592; continue; }
  pc = 0x1202256e; continue;
 }
 0x1202256e => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12022624; continue; }
  pc = 0x12022576; continue;
 }
 0x12022576 => {
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x1202261b; continue; }
  pc = 0x1202257e; continue;
 }
 0x1202257e => {
  { cpu.and32(cpu.edx, 0xff0000u32); }
  if cpu.zf { pc = 0x1202260e; continue; }
  pc = 0x1202258a; continue;
 }
 0x1202258a => {
  { cpu.and32(cpu.edx, 0xff000000u32); }
  if cpu.zf { pc = 0x12022606; continue; }
  pc = 0x12022592; continue;
 }
 0x12022592 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x12022554; continue;
 }
 0x12022606 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202260e => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202261b => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022624 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12022599(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12022599); }
 let mut pc: u32 = 0x12022599;
 loop { match pc {
 0x12022532 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x12022554; continue; }
  pc = 0x1202253e; continue;
 }
 0x1202253e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12022624; continue; }
  pc = 0x12022549; continue;
 }
 0x12022549 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.ecx, 0x3u32); }
  if !cpu.zf { pc = 0x1202253e; continue; }
  pc = 0x12022554; continue;
 }
 0x12022554 => {
  { let v: u32 = 0x7efefeffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, 0x81010100u32); }
  if cpu.zf { pc = 0x12022592; continue; }
  pc = 0x1202256e; continue;
 }
 0x1202256e => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x12022624; continue; }
  pc = 0x12022576; continue;
 }
 0x12022576 => {
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x1202261b; continue; }
  pc = 0x1202257e; continue;
 }
 0x1202257e => {
  { cpu.and32(cpu.edx, 0xff0000u32); }
  if cpu.zf { pc = 0x1202260e; continue; }
  pc = 0x1202258a; continue;
 }
 0x1202258a => {
  { cpu.and32(cpu.edx, 0xff000000u32); }
  if cpu.zf { pc = 0x12022606; continue; }
  pc = 0x12022592; continue;
 }
 0x12022592 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x12022554; continue;
 }
 0x12022599 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.ecx, 0x3u32); }
  if cpu.zf { pc = 0x120225b5; continue; }
  pc = 0x120225a6; continue;
 }
 0x120225a6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x120225e6; continue; }
  pc = 0x120225ad; continue;
 }
 0x120225ad => {
  { cpu.and32(cpu.ecx, 0x3u32); }
  if !cpu.zf { pc = 0x120225a6; continue; }
  pc = 0x120225b5; continue;
 }
 0x120225b5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x7efefeffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, 0x81010100u32); }
  if cpu.zf { pc = 0x120225b5; continue; }
  pc = 0x120225cd; continue;
 }
 0x120225cd => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x120225fe; continue; }
  pc = 0x120225d4; continue;
 }
 0x120225d4 => {
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x120225f6; continue; }
  pc = 0x120225d8; continue;
 }
 0x120225d8 => {
  { cpu.and32(cpu.eax, 0xff0000u32); }
  if cpu.zf { pc = 0x120225ee; continue; }
  pc = 0x120225df; continue;
 }
 0x120225df => {
  { cpu.and32(cpu.eax, 0xff000000u32); }
  if !cpu.zf { pc = 0x120225b5; continue; }
  pc = 0x120225e6; continue;
 }
 0x120225e6 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  pc = 0x12022532; continue;
 }
 0x120225ee => {
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  pc = 0x12022532; continue;
 }
 0x120225f6 => {
  { let v: u32 = 0xfffffffdu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  pc = 0x12022532; continue;
 }
 0x120225fe => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  pc = 0x12022532; continue;
 }
 0x12022606 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202260e => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202261b => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022624 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202262c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202262c); }
 let mut pc: u32 = 0x1202262c;
 loop { match pc {
 0x1202262c => {
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1202264d); f_1200353a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1202265a; continue; }
  pc = 0x12022654; continue;
 }
 0x12022654 => {
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202265a => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202265e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202265e); }
 let mut pc: u32 = 0x1202265e;
 loop { match pc {
 0x1202265e => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x0u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  if !cpu.zf { pc = 0x12022682; continue; }
  pc = 0x1202266b; continue;
 }
 0x1202266b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1202267a); f_120174ac(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022682 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12022689); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x120226f6; continue; }
  pc = 0x12022697; continue;
 }
 0x12022697 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x120226a3; continue;
 }
 0x120226a3 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x120226d9; continue; }
  pc = 0x120226a9; continue;
 }
 0x120226a9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x120226d5; continue; }
  pc = 0x120226b7; continue;
 }
 0x120226b7 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x120226c3; continue; }
  pc = 0x120226bb; continue;
 }
 0x120226bb => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x120226d9; continue; }
  pc = 0x120226c3; continue;
 }
 0x120226c3 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x120226d9; continue; }
  pc = 0x120226cb; continue;
 }
 0x120226cb => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x120226cd; continue;
 }
 0x120226cd => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x120226d9; continue; }
  pc = 0x120226d3; continue;
 }
 0x120226d3 => {
  pc = 0x120226a9; continue;
 }
 0x120226d5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x120226cd; continue; }
  pc = 0x120226d9; continue;
 }
 0x120226d9 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x120226f6; continue; }
  pc = 0x120226de; continue;
 }
 0x120226de => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.eax)), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x120226f0; continue; }
  pc = 0x120226ea; continue;
 }
 0x120226ea => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x120226f6; continue; }
  pc = 0x120226f0; continue;
 }
 0x120226f0 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x120226a3; continue; }
  pc = 0x120226f6; continue;
 }
 0x120226f6 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x120226fd); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1202270b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1202270b); }
 let mut pc: u32 = 0x1202270b;
 loop { match pc {
 0x1202270b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x12022718); f_12004c16(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12022720); f_12004858(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x1202272e; continue; }
  pc = 0x12022725; continue;
 }
 0x12022725 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1202272c); f_12003b1e(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x1202272e; continue;
 }
 0x1202272e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12022731(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12022731); }
 let mut pc: u32 = 0x12022731;
 loop { match pc {
 0x12022731 => {
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x12022752); f_1200353a(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1202275f; continue; }
  pc = 0x12022759; continue;
 }
 0x12022759 => {
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1202275f => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_12022763(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x12022763); }
 let mut pc: u32 = 0x12022763;
 loop { match pc {
 0x12022763 => {
  { cpu.sub32(cpu.r32(0x1203bd8cu32), 0x0u32); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x12022783; continue; }
  pc = 0x1202276e; continue;
 }
 0x1202276e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022783 => {
  { let v = 0x19u32; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  cpu.push(0x1202278c); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x120227b6; continue; }
  pc = 0x12022798; continue;
 }
 0x12022798 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x1202279d; continue;
 }
 0x1202279d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x120227af; continue; }
  pc = 0x120227a9; continue;
 }
 0x120227a9 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x120227b6; continue; }
  pc = 0x120227af; continue;
 }
 0x120227af => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x1202279d; continue; }
  pc = 0x120227b6; continue;
 }
 0x120227b6 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x120227bd); f_120040fd(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_120227c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x120227c5); }
 let mut pc: u32 = 0x120227c5;
 loop { match pc {
 0x120227c5 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x120227d2); f_1200420b(cpu);
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x12022833; continue; }
  pc = 0x120227e0; continue;
 }
 0x120227e0 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  pc = 0x120227e5; continue;
 }
 0x120227e5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1203bc89u32.wrapping_add(cpu.ecx)), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x12022846; continue; }
  pc = 0x120227f3; continue;
 }
 0x120227f3 => {
  { let v: u32 = cpu.r32(0x1203bd8cu32); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1203bd90u32); cpu.edx = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x12022814); f_120170bd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x12022852; continue; }
  pc = 0x1202281b; continue;
 }
 0x1202281b => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1202282d; continue; }
  pc = 0x12022826; continue;
 }
 0x12022826 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1202282b; continue;
 }
 0x1202282b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1202282d; continue;
 }
 0x1202282d => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x120227e5; continue; }
  pc = 0x12022833; continue;
 }
 0x12022833 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x1202283a); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x12022846 => {
  { cpu.sub8((cpu.eax as u8), 0x41u8); }
  if cpu.cf { pc = 0x1202282d; continue; }
  pc = 0x1202284a; continue;
 }
 0x1202284a => {
  { cpu.sub8((cpu.eax as u8), 0x5au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1202282d; continue; }
  pc = 0x1202284e; continue;
 }
 0x1202284e => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1202282b; continue;
 }
 0x12022852 => {
  { let v = 0x19u32; cpu.push(v); }
  cpu.push(0x12022859); f_120040fd(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
