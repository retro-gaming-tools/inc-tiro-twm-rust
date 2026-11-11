#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_15006577(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006577); }
 let mut pc: u32 = 0x15006577;
 loop { match pc {
 0x15006577 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x15006585); f_15005fd8(cpu);
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500658b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500658b); }
 let mut pc: u32 = 0x1500658b;
 loop { match pc {
 0x1500658b => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x1500659a); f_15005fd8(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006610); }
 let mut pc: u32 = 0x15006610;
 loop { match pc {
 0x15006610 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff00000u32); }
  if cpu.zf { pc = 0x15006654; continue; }
  pc = 0x15006621; continue;
 }
 0x15006621 => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x15006637; continue; }
  pc = 0x1500662a; continue;
 }
 0x1500662a => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x15006633); f_1500658b(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x15006637 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x15006648); f_1500658b(cpu);
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x15006654 => {
  { let v = cpu.st(0) / cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150066f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150066f4); }
 let mut pc: u32 = 0x150066f4;
 loop { match pc {
 0x150066f4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x1500671b; continue; }
  pc = 0x15006717; continue;
 }
 0x15006717 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x15006750; continue; }
  pc = 0x1500671b; continue;
 }
 0x1500671b => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7ff0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0x800fu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx | 0x3fe0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x3feu32); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x15006750; continue;
 }
 0x15006750 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006763(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006763); }
 let mut pc: u32 = 0x15006763;
 loop { match pc {
 0x15006763 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x15006769, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x1500676e, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x150067ef; continue; }
  pc = 0x1500677e; continue;
 }
 0x1500677e => {
  if cpu.cf { pc = 0x150067f3; continue; }
  pc = 0x15006780; continue;
 }
 0x15006780 => {
  if cpu.zf { pc = 0x150067f7; continue; }
  pc = 0x15006782; continue;
 }
 0x15006782 => {
  if (cpu.cf || cpu.zf) { pc = 0x150067fb; continue; }
  pc = 0x15006784; continue;
 }
 0x15006784 => {
  if cpu.sf { pc = 0x150067ff; continue; }
  pc = 0x15006786; continue;
 }
 0x15006786 => {
  if cpu.pf() { f_15006788(cpu); return; }
  f_15006788(cpu); return;
 }
 0x150067ef => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  let t = cpu.r32(0x15016b84u32); cpu.push(0x150067f7); cpu.call(t);
  pc = 0x150067f7; continue;
 }
 0x150067f3 => {
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), ((cpu.ecx >> 8) as u8)); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x6ccc158bu32); cpu.eax = v; }
  pc = 0x150067fb; continue;
 }
 0x150067f7 => {
  { let v: u32 = cpu.r32(0x15016cccu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15016cccu32, v); }
  let t = cpu.r32(0x15016b8cu32); cpu.push(0x15006809); cpu.call(t);
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150067fb => {
  { let v: u32 = cpu.r32(0x6ccc1d89u32).wrapping_add(cpu.edx); cpu.w32(0x6ccc1d89u32, v); }
  { let v: u32 = cpu.r32(0x6b8c15ffu32).wrapping_add(cpu.edx); cpu.w32(0x6b8c15ffu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x5b5ad089u32), cpu.edx); cpu.w32(0x5b5ad089u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150067ff => {
  cpu.unimplemented(0x150067ff, "Int3");
  cpu.unimplemented(0x15006800, "Insb");
  { let v: u32 = cpu.r32(0x6b8c15ffu32).wrapping_add(cpu.edx); cpu.w32(0x6b8c15ffu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x5b5ad089u32), cpu.edx); cpu.w32(0x5b5ad089u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006788(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006788); }
 let mut pc: u32 = 0x15006788;
 loop { match pc {
 0x15006788 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x1500679e; continue;
 }
 0x1500679e => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x15006763u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500679e; continue; }
  pc = 0x150067be; continue;
 }
 0x150067be => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x150067be; continue; }
  pc = 0x150067c8; continue;
 }
 0x150067c8 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150067d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150067d2); }
 let mut pc: u32 = 0x150067d2;
 loop { match pc {
 0x150067d2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x150067e4; continue; }
  pc = 0x150067da; continue;
 }
 0x150067da => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x150067e4; continue; }
  pc = 0x150067de; continue;
 }
 0x150067de => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x150067e4; continue;
 }
 0x150067e4 => {
  cpu.push(0x150067e9); f_15006788(cpu);
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500680e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500680e); }
 let mut pc: u32 = 0x1500680e;
 loop { match pc {
 0x1500680e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x15016b84u32); cpu.push(0x15006818); cpu.call(t);
  { let v: u32 = cpu.r32(0x15016ca8u32); cpu.eax = v; }
  pc = 0x150068c2; continue;
 }
 0x150068aa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { pc = 0x150068c0; continue; }
  pc = 0x150068bb; continue;
 }
 0x150068bb => {
  cpu.push(0x150068c0); f_15006880(cpu);
  pc = 0x150068c0; continue;
 }
 0x150068c0 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x150068c2; continue;
 }
 0x150068c2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150068aa; continue; }
  pc = 0x150068c6; continue;
 }
 0x150068c6 => {
  let t = cpu.r32(0x15016b8cu32); cpu.push(0x150068cc); cpu.call(t);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006822(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006822); }
 let mut pc: u32 = 0x15006822;
 loop { match pc {
 0x15006822 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = 0x8000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  cpu.push(0x15006839); f_1500fb70(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006844; continue; }
  pc = 0x1500683d; continue;
 }
 0x1500683d => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1500687a; continue;
 }
 0x15006844 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15016cacu32)); }
  if !cpu.zf { pc = 0x15006868; continue; }
  pc = 0x1500684c; continue;
 }
 0x1500684c => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x15006858; continue; }
  pc = 0x15006850; continue;
 }
 0x15006850 => {
  { let v: u32 = cpu.esi; cpu.w32(0x15016cacu32, v); }
  pc = 0x15006868; continue;
 }
 0x15006858 => {
  { let v: u32 = cpu.r32(0x15016ca8u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x15016cb0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15016cacu32, v); }
  pc = 0x15006868; continue;
 }
 0x15006868 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15094c98u32)); }
  if !cpu.zf { pc = 0x15006878; continue; }
  pc = 0x15006870; continue;
 }
 0x15006870 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15094c98u32, v); }
  pc = 0x15006878; continue;
 }
 0x15006878 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1500687a; continue;
 }
 0x1500687a => {
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
pub fn f_15006880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006880); }
 let mut pc: u32 = 0x15006880;
 loop { match pc {
 0x15006880 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  cpu.push(0x1500688d); f_15006822(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150068a7; continue; }
  pc = 0x15006891; continue;
 }
 0x15006891 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x1500689d; continue; }
  pc = 0x15006895; continue;
 }
 0x15006895 => {
  { let v: u32 = cpu.edx; cpu.w32(0x15016ca8u32, v); }
  pc = 0x150068a0; continue;
 }
 0x1500689d => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x150068a0; continue;
 }
 0x150068a0 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x150068a7; continue; }
  pc = 0x150068a4; continue;
 }
 0x150068a4 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  pc = 0x150068a7; continue;
 }
 0x150068a7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150068d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150068d3); }
 let mut pc: u32 = 0x150068d3;
 loop { match pc {
 0x150068d3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x150068db); f_150068dd(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150068dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150068dd); }
 let mut pc: u32 = 0x150068dd;
 loop { match pc {
 0x150068dd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x15006909; continue; }
  pc = 0x15006902; continue;
 }
 0x15006902 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x15006943; continue;
 }
 0x15006909 => {
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if cpu.zf { pc = 0x15006915; continue; }
  pc = 0x1500690e; continue;
 }
 0x1500690e => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  pc = 0x15006943; continue;
 }
 0x15006915 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x15006921; continue; }
  pc = 0x1500691a; continue;
 }
 0x1500691a => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  pc = 0x15006943; continue;
 }
 0x15006921 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x1500692d; continue; }
  pc = 0x15006926; continue;
 }
 0x15006926 => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x15006943; continue;
 }
 0x1500692d => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x15006939; continue; }
  pc = 0x15006932; continue;
 }
 0x15006932 => {
  { let v: u32 = 0x6u32; cpu.esi = v; }
  pc = 0x15006943; continue;
 }
 0x15006939 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x15006943; continue; }
  pc = 0x1500693e; continue;
 }
 0x1500693e => {
  { let v: u32 = 0x5u32; cpu.esi = v; }
  pc = 0x15006943; continue;
 }
 0x15006943 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15016cccu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x15006967; continue; }
  pc = 0x1500695a; continue;
 }
 0x1500695a => {
  { let v = cpu.rf64(0x15017a54u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x150069a9; continue;
 }
 0x15006967 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x15006976; continue; }
  pc = 0x1500696c; continue;
 }
 0x1500696c => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x150069a9; continue;
 }
 0x15006976 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x1500698a; continue; }
  pc = 0x1500697b; continue;
 }
 0x1500697b => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x150069a9; continue;
 }
 0x1500698a => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x80u8); }
  if cpu.zf { pc = 0x1500699e; continue; }
  pc = 0x1500698f; continue;
 }
 0x1500698f => {
  { let v: u32 = cpu.r32(0x15017a54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x15017a58u32); cpu.eax = v; }
  pc = 0x150069a6; continue;
 }
 0x1500699e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x150069a6; continue;
 }
 0x150069a6 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x150069a9; continue;
 }
 0x150069a9 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x150069b1); f_15006a5b(cpu);
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150069b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150069b8); }
 let mut pc: u32 = 0x150069b8;
 loop { match pc {
 0x150069b8 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xc0000000u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4150017eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4147ffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15016b0du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x15006a09; continue; }
  pc = 0x150069e6; continue;
 }
 0x150069e6 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x1501786cu32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x15006a09; continue; }
  pc = 0x15006a02; continue;
 }
 0x15006a02 => {
  { let v: u8 = cpu.r8(0x15016cc8u32) | 0x1u8; cpu.w8(0x15016cc8u32, v); }
  pc = 0x15006a09; continue;
 }
 0x15006a09 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006a13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006a13); }
 let mut pc: u32 = 0x15006a13;
 loop { match pc {
 0x15006a13 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x15006a24); f_15006a8f(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15017aacu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x15006a36); f_15006aae(cpu);
  { let v: u32 = 0x150178f9u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x15006a42); f_15006aae(cpu);
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x15006a4b); f_15006aae(cpu);
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x15006a57); f_15006b3b(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006a5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006a5b); }
 let mut pc: u32 = 0x15006a5b;
 loop { match pc {
 0x15006a5b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  let t = cpu.r32(0x15016d2cu32); cpu.push(0x15006a65); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006a84; continue; }
  pc = 0x15006a69; continue;
 }
 0x15006a69 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x15006a73); f_15006a13(cpu);
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if !cpu.zf { pc = 0x15006a7f; continue; }
  pc = 0x15006a78; continue;
 }
 0x15006a78 => {
  cpu.push(0x15006a7d); f_150050c4(cpu);
  pc = 0x15006a84; continue;
 }
 0x15006a7f => {
  cpu.push(0x15006a84); f_150050d7(cpu);
  pc = 0x15006a84; continue;
 }
 0x15006a84 => {
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006a8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006a8a); }
 let mut pc: u32 = 0x15006a8a;
 loop { match pc {
 0x15006a8a => {
  pc = 0x15006bfe; continue;
 }
 0x15006bfe => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006a8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006a8f); }
 let mut pc: u32 = 0x15006a8f;
 loop { match pc {
 0x15006a8f => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.cf || cpu.zf) { pc = 0x15006a99; continue; }
  pc = 0x15006a95; continue;
 }
 0x15006a95 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15006a99 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x15016d30u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006aae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006aae); }
 let mut pc: u32 = 0x15006aae;
 loop { match pc {
 0x15006aae => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x15006ac0); cpu.call(t);
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x15006acd; continue; }
  pc = 0x15006ac6; continue;
 }
 0x15006ac6 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x15006acd); f_15006c01(cpu);
  pc = 0x15006acd; continue;
 }
 0x15006acd => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x15006ae9; continue; }
  pc = 0x15006ad7; continue;
 }
 0x15006ad7 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  pc = 0x15006ae9; continue;
 }
 0x15006ae9 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x15006aed; continue;
 }
 0x15006aed => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15006b04; continue; }
  pc = 0x15006af5; continue;
 }
 0x15006af5 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x15006afd); f_15006b3b(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x15006aed; continue; }
  pc = 0x15006b02; continue;
 }
 0x15006b02 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x15006b04; continue;
 }
 0x15006b04 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x15006b22; continue; }
  pc = 0x15006b08; continue;
 }
 0x15006b08 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15006b22; continue; }
  pc = 0x15006b19; continue;
 }
 0x15006b19 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x15006b20); f_15006c78(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x15006b22; continue;
 }
 0x15006b22 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15006b2a; continue; }
  pc = 0x15006b26; continue;
 }
 0x15006b26 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.ebp); cpu.esi = v; }
  pc = 0x15006b2a; continue;
 }
 0x15006b2a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15006b33); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
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
pub fn f_15006b3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006b3b); }
 let mut pc: u32 = 0x15006b3b;
 loop { match pc {
 0x15006b3b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x15006b4b); cpu.call(t);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x2u8); }
  if !cpu.zf { pc = 0x15006b72; continue; }
  pc = 0x15006b51; continue;
 }
 0x15006b51 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x15006b5b); f_150050c9(cpu);
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x20u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x15006b5f; continue;
 }
 0x15006b5f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15006b68); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x15006bf8; continue;
 }
 0x15006b72 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x15006b7f; continue; }
  pc = 0x15006b78; continue;
 }
 0x15006b78 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15006b7f); f_15006c01(cpu);
  pc = 0x15006b7f; continue;
 }
 0x15006b7f => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x15006bbc; continue; }
  pc = 0x15006b89; continue;
 }
 0x15006b89 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x600u32; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x15006bbc; continue; }
  pc = 0x15006b95; continue;
 }
 0x15006b95 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if !cpu.zf { pc = 0x15006bbc; continue; }
  pc = 0x15006bb1; continue;
 }
 0x15006bb1 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15006bb8); f_15006c78(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006b5f; continue; }
  pc = 0x15006bbc; continue;
 }
 0x15006bbc => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.edi); }
  if !cpu.zf { pc = 0x15006bdc; continue; }
  pc = 0x15006bd7; continue;
 }
 0x15006bd7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x15006beb; continue; }
  pc = 0x15006bdc; continue;
 }
 0x15006bdc => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15006be3); f_15006c78(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006b5f; continue; }
  pc = 0x15006beb; continue;
 }
 0x15006beb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15006bf4); cpu.call(t);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x15006bf8; continue;
 }
 0x15006bf8 => {
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
pub fn f_15006c01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006c01); }
 let mut pc: u32 = 0x15006c01;
 loop { match pc {
 0x15006c01 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x15006c0a); f_15006d62(cpu);
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x15006c36; continue; }
  pc = 0x15006c10; continue;
 }
 0x15006c10 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x15006c21; continue; }
  pc = 0x15006c18; continue;
 }
 0x15006c18 => {
  { let v: u32 = 0x86u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x15006c36; continue;
 }
 0x15006c21 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x15006c2f; continue; }
  pc = 0x15006c26; continue;
 }
 0x15006c26 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x15006c36; continue;
 }
 0x15006c2f => {
  { let v: u32 = 0x1000u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x15006c36; continue;
 }
 0x15006c36 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x15006c3e); f_15004de6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006c65; continue; }
  pc = 0x15006c45; continue;
 }
 0x15006c45 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) & 0xf8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x15006c69; continue;
 }
 0x15006c65 => {
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x8u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x15006c69; continue;
 }
 0x15006c69 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006c78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006c78); }
 let mut pc: u32 = 0x15006c78;
 loop { match pc {
 0x15006c78 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x15006c89); cpu.call(t);
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x15006d0c; continue; }
  pc = 0x15006c97; continue;
 }
 0x15006c97 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xefu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x15006d44; continue; }
  pc = 0x15006caa; continue;
 }
 0x15006caa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x15006d44; continue; }
  pc = 0x15006cb5; continue;
 }
 0x15006cb5 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  pc = 0x15006cba; continue;
 }
 0x15006cba => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x15006d44; continue; }
  pc = 0x15006cc2; continue;
 }
 0x15006cc2 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x15006d44; continue; }
  pc = 0x15006cca; continue;
 }
 0x15006cca => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x15006cd6); f_15006d93(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x15006cea; continue; }
  pc = 0x15006cdd; continue;
 }
 0x15006cdd => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x15006d06; continue;
 }
 0x15006cea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006d06; continue; }
  pc = 0x15006cee; continue;
 }
 0x15006cee => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x15006cf8); f_150050c9(cpu);
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x15006d06; continue;
 }
 0x15006d06 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  pc = 0x15006cba; continue;
 }
 0x15006d0c => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x15006d44; continue; }
  pc = 0x15006d12; continue;
 }
 0x15006d12 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)) & 0xefu8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x20u8); }
  if !cpu.zf { pc = 0x15006d44; continue; }
  pc = 0x15006d1c; continue;
 }
 0x15006d1c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15006d34; continue; }
  pc = 0x15006d23; continue;
 }
 0x15006d23 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x15006d34); f_15006e39(cpu);
  pc = 0x15006d34; continue;
 }
 0x15006d34 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x15006d44; continue; }
  pc = 0x15006d39; continue;
 }
 0x15006d39 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x20u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x15006d44; continue;
 }
 0x15006d44 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15006d59); cpu.call(t);
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
pub fn f_15006d62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006d62); }
 let mut pc: u32 = 0x15006d62;
 loop { match pc {
 0x15006d62 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x15006d8f; continue; }
  pc = 0x15006d6d; continue;
 }
 0x15006d6d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x15006d75); f_150057dc(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15006d8f; continue; }
  pc = 0x15006d79; continue;
 }
 0x15006d79 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { cpu.and8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x15006d8f; continue; }
  pc = 0x15006d87; continue;
 }
 0x15006d87 => {
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x15006d8f; continue;
 }
 0x15006d8f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006d93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006d93); }
 let mut pc: u32 = 0x15006d93;
 loop { match pc {
 0x15006d93 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x15006dae); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x15006db5); f_15004cab(cpu);
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x15006dd9; continue; }
  pc = 0x15006db9; continue;
 }
 0x15006db9 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x15006dc5); f_1500fb6a(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x15006dd9; continue; }
  pc = 0x15006dca; continue;
 }
 0x15006dca => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15006dd2); cpu.call(t);
  cpu.push(0x15006dd7); f_15006faf(cpu);
  pc = 0x15006e31; continue;
 }
 0x15006dd9 => {
  { cpu.sub32(cpu.r32(0x15016f84u32), 0x0u32); }
  if cpu.zf { pc = 0x15006e04; continue; }
  pc = 0x15006de2; continue;
 }
 0x15006de2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x15016f58u32); cpu.push(0x15006dea); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15006e04; continue; }
  pc = 0x15006dee; continue;
 }
 0x15006dee => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  let t = cpu.r32(0x15016f84u32); cpu.push(0x15006df6); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15006e00); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x15006e31; continue;
 }
 0x15006e04 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x15006e13); f_1500fba6(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15006dca; continue; }
  pc = 0x15006e17; continue;
 }
 0x15006e17 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x15006e26; continue; }
  pc = 0x15006e1c; continue;
 }
 0x15006e1c => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x15006e26); f_150050c9(cpu);
  pc = 0x15006e26; continue;
 }
 0x15006e26 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15006e2e); cpu.call(t);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x15006e31; continue;
 }
 0x15006e31 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006e39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006e39); }
 let mut pc: u32 = 0x15006e39;
 loop { match pc {
 0x15006e39 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x15006e4e); cpu.call(t);
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15006e58); f_1500fb6a(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15006e64); cpu.call(t);
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x15006e6e; continue; }
  pc = 0x15006e69; continue;
 }
 0x15006e69 => {
  cpu.push(0x15006e6e); f_15006faf(cpu);
  pc = 0x15006e6e; continue;
 }
 0x15006e6e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006e73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006e73); }
 let mut pc: u32 = 0x15006e73;
 loop { match pc {
 0x15006e73 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x15016d71u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x15016d71u32, v); }
  { let v: u8 = (cpu.edx as u8) | 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x15016d30u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x15016d71u32, v); }
  pc = 0x15006ed9; continue;
 }
 0x15006e97 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x15006ea1); f_15004de6(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006ec6; continue; }
  pc = 0x15006ea7; continue;
 }
 0x15006ea7 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x15006eb1); f_15004de6(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006ec6; continue; }
  pc = 0x15006eb7; continue;
 }
 0x15006eb7 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x15017900u32; cpu.eax = v; }
  cpu.push(0x15006ec6); f_15004c8e(cpu);
  pc = 0x15006ec6; continue;
 }
 0x15006ec6 => {
  { let v: u32 = cpu.r32(0x15094ca8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15094ca8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x15006ed9; continue;
 }
 0x15006ed9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x15006e97; continue; }
  pc = 0x15006ee0; continue;
 }
 0x15006ee0 => {
  { let v: u32 = cpu.edx; cpu.w32(0x15094ca4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006eea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006eea); }
 let mut pc: u32 = 0x15006eea;
 loop { match pc {
 0x15006eea => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x15006ef1); f_15006efb(cpu);
  pc = 0x150070af; continue;
 }
 0x150070af => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x150070b0; continue;
 }
 0x150070b0 => {
  { let v: u32 = cpu.r32(0x15094ca4u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x150070cb; continue; }
  pc = 0x150070ba; continue;
 }
 0x150070ba => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.push(0x150070c3); f_15003ecc(cpu);
  { let v: u32 = cpu.edx; cpu.w32(0x15094ca4u32, v); }
  pc = 0x150070b0; continue;
 }
 0x150070cb => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006efb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006efb); }
 let mut pc: u32 = 0x15006efb;
 loop { match pc {
 0x15006efb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x15016d30u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15094ca8u32); cpu.eax = v; }
  pc = 0x15006f48; continue;
 }
 0x15006f1d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x40u8); }
  if !cpu.zf { pc = 0x15006f40; continue; }
  pc = 0x15006f2d; continue;
 }
 0x15006f2d => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x15006f40; continue; }
  pc = 0x15006f33; continue;
 }
 0x15006f33 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.cf { pc = 0x15006f46; continue; }
  pc = 0x15006f37; continue;
 }
 0x15006f37 => {
  { cpu.sub32(cpu.eax, 0x15016d7eu32); }
  if !cpu.cf { pc = 0x15006f40; continue; }
  pc = 0x15006f3e; continue;
 }
 0x15006f3e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x15006f40; continue;
 }
 0x15006f40 => {
  cpu.push(0x15006f45); f_1500710c(cpu);
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x15006f46; continue;
 }
 0x15006f46 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x15006f48; continue;
 }
 0x15006f48 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x15006f1d; continue; }
  pc = 0x15006f4c; continue;
 }
 0x15006f4c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006f5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006f5e); }
 let mut pc: u32 = 0x15006f5e;
 loop { match pc {
 0x15006f5e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x15006f66); f_150050ee(cpu);
  { cpu.sub32(cpu.edx, 0x7bu32); }
  if !cpu.zf { pc = 0x15006f72; continue; }
  pc = 0x15006f6b; continue;
 }
 0x15006f6b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x15006fa3; continue;
 }
 0x15006f72 => {
  { cpu.sub32(cpu.edx, 0xceu32); }
  if !cpu.zf { pc = 0x15006f81; continue; }
  pc = 0x15006f7a; continue;
 }
 0x15006f7a => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  pc = 0x15006fa3; continue;
 }
 0x15006f81 => {
  { cpu.sub32(cpu.edx, 0xb7u32); }
  if !cpu.zf { pc = 0x15006f90; continue; }
  pc = 0x15006f89; continue;
 }
 0x15006f89 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  pc = 0x15006fa3; continue;
 }
 0x15006f90 => {
  { cpu.sub32(cpu.edx, 0x13u32); }
  if (cpu.cf || cpu.zf) { pc = 0x15006f9a; continue; }
  pc = 0x15006f95; continue;
 }
 0x15006f95 => {
  { let v: u32 = 0x13u32; cpu.edx = v; }
  pc = 0x15006f9a; continue;
 }
 0x15006f9a => {
  { let v: u32 = cpu.r32(0x15016f41u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 24u32); cpu.eax = v; }
  pc = 0x15006fa3; continue;
 }
 0x15006fa3 => {
  cpu.push(0x15006fa8); f_150050c9(cpu);
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15006faf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15006faf); }
 let mut pc: u32 = 0x15006faf;
 loop { match pc {
 0x15006faf => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15006fb6); f_1500fb64(cpu);
  cpu.push(0x15006fbb); f_15006f5e(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007076(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007076); }
 let mut pc: u32 = 0x15007076;
 loop { match pc {
 0x15007076 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x15094ca8u32; cpu.edx = v; }
  pc = 0x15007080; continue;
 }
 0x15007080 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150070ab; continue; }
  pc = 0x15007086; continue;
 }
 0x15007086 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if cpu.zf { pc = 0x15007091; continue; }
  pc = 0x1500708d; continue;
 }
 0x1500708d => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x15007080; continue;
 }
 0x15007091 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x15094ca4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15094ca4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x150070ab; continue;
 }
 0x150070ab => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500710c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500710c); }
 let mut pc: u32 = 0x1500710c;
 loop { match pc {
 0x1500710c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x15007114); f_15007192(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x1500711d); f_15007076(cpu);
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007121(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007121); }
 let mut pc: u32 = 0x15007121;
 loop { match pc {
 0x15007121 => {
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500712c; continue; }
  pc = 0x15007129; continue;
 }
 0x15007129 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x27u32); cpu.eax = v; }
  pc = 0x1500712c; continue;
 }
 0x1500712c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500712d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500712d); }
 let mut pc: u32 = 0x1500712d;
 loop { match pc {
 0x1500712d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x1500713a); f_1500723d(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = 0x74u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  pc = 0x15007144; continue;
 }
 0x15007144 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  cpu.push(0x1500714e); f_15007121(cpu);
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { pc = 0x15007144; continue; }
  pc = 0x15007158; continue;
 }
 0x15007158 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = 0x5fu8; cpu.w8(0x5u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x15007169); f_15007121(cpu);
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  cpu.push(0x15007176); f_15007121(cpu);
  { let v: u8 = 0x2eu8; cpu.w8(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x74u8; cpu.w8(0x9u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x6du8; cpu.w8(0xau32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x70u8; cpu.w8(0xbu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007192(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007192); }
 let mut pc: u32 = 0x15007192;
 loop { match pc {
 0x15007192 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x150071ae; continue; }
  pc = 0x150071a4; continue;
 }
 0x150071a4 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x15007234; continue;
 }
 0x150071ae => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x150071c1; continue; }
  pc = 0x150071b8; continue;
 }
 0x150071b8 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x150071bf); f_15006c78(cpu);
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x150071c1; continue;
 }
 0x150071c1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x150071ca); cpu.call(t);
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x150071d1); f_15007247(cpu);
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x150071e4; continue; }
  pc = 0x150071d6; continue;
 }
 0x150071d6 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x150071e4); f_15006e39(cpu);
  pc = 0x150071e4; continue;
 }
 0x150071e4 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x150071f2; continue; }
  pc = 0x150071e8; continue;
 }
 0x150071e8 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x150071f0); f_15007298(cpu);
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  pc = 0x150071f2; continue;
 }
 0x150071f2 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x15007207; continue; }
  pc = 0x150071f8; continue;
 }
 0x150071f8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x15007200); f_15003ecc(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x15007207; continue;
 }
 0x15007207 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x15007220; continue; }
  pc = 0x1500720d; continue;
 }
 0x1500720d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x15007219); f_1500712d(cpu);
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x15007220); f_15007307(cpu);
  pc = 0x15007220; continue;
 }
 0x15007220 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15007229); cpu.call(t);
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b78u32); cpu.push(0x15007232); cpu.call(t);
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x15007234; continue;
 }
 0x15007234 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
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
pub fn f_1500723d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500723d); }
 let mut pc: u32 = 0x1500723d;
 loop { match pc {
 0x1500723d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x15007244); f_1500fb5e(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007247(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007247); }
 let mut pc: u32 = 0x15007247;
 loop { match pc {
 0x15007247 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x15007253); f_1500730c(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x15007295; continue; }
  pc = 0x1500725a; continue;
 }
 0x1500725a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x15007263); cpu.call(t);
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x15007276; continue; }
  pc = 0x15007269; continue;
 }
 0x15007269 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x15007276; continue; }
  pc = 0x1500726f; continue;
 }
 0x1500726f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x15007276); f_15007349(cpu);
  pc = 0x15007276; continue;
 }
 0x15007276 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x1500728a; continue; }
  pc = 0x1500727c; continue;
 }
 0x1500727c => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x15007287; continue; }
  pc = 0x15007282; continue;
 }
 0x15007282 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  pc = 0x1500728a; continue;
 }
 0x15007287 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  pc = 0x1500728a; continue;
 }
 0x1500728a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15007293); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x15007295; continue;
 }
 0x15007295 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007298(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007298); }
 let mut pc: u32 = 0x15007298;
 loop { match pc {
 0x15007298 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15016f64u32); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x150072d7; continue; }
  pc = 0x150072b6; continue;
 }
 0x150072b6 => {
  let t = cpu.r32(0x15016f58u32); cpu.push(0x150072bc); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150072d7; continue; }
  pc = 0x150072c2; continue;
 }
 0x150072c2 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  let t = cpu.r32(0x15016f5cu32); cpu.push(0x150072ca); cpu.call(t);
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  let t = cpu.r32(0x15016f64u32); cpu.push(0x150072d7); cpu.call(t);
  pc = 0x150072d7; continue;
 }
 0x150072d7 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x150072f6; continue; }
  pc = 0x150072db; continue;
 }
 0x150072db => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x150072e1); f_1500fc30(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x150072f6; continue; }
  pc = 0x150072e5; continue;
 }
 0x150072e5 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  cpu.push(0x150072f4); f_150050c9(cpu);
  pc = 0x150072ff; continue;
 }
 0x150072f6 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x150072ff); f_15004d00(cpu);
  pc = 0x150072ff; continue;
 }
 0x150072ff => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
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
pub fn f_15007307(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007307); }
 let mut pc: u32 = 0x15007307;
 loop { match pc {
 0x15007307 => {
  pc = 0x15007359; continue;
 }
 0x15007359 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x15007361); f_1500fb58(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1500736d; continue; }
  pc = 0x15007365; continue;
 }
 0x15007365 => {
  cpu.push(0x1500736a); f_15006faf(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500736d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500730c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500730c); }
 let mut pc: u32 = 0x1500730c;
 loop { match pc {
 0x1500730c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15094c90u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  let t = cpu.r32(0x15016b6cu32); cpu.push(0x15007323); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1500732f); f_1500fb6a(cpu);
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1500733b; continue; }
  pc = 0x15007336; continue;
 }
 0x15007336 => {
  cpu.push(0x1500733b); f_15006faf(cpu);
  pc = 0x1500733b; continue;
 }
 0x1500733b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  let t = cpu.r32(0x15016b70u32); cpu.push(0x15007343); cpu.call(t);
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007349(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007349); }
 let mut pc: u32 = 0x15007349;
 loop { match pc {
 0x15007349 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { f_15006c78(cpu); return; }
  pc = 0x15007351; continue;
 }
 0x15007351 => {
  cpu.push(0x15007356); f_15007372(cpu);
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007372(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007372); }
 let mut pc: u32 = 0x15007372;
 loop { match pc {
 0x15007372 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x15016b7cu32); cpu.push(0x15007382); cpu.call(t);
  { let v: u32 = cpu.r32(0x15094ca8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x150073a2; continue;
 }
 0x1500738c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), cpu.ecx); }
  if cpu.zf { pc = 0x150073a0; continue; }
  pc = 0x15007394; continue;
 }
 0x15007394 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x150073a0; continue; }
  pc = 0x1500739b; continue;
 }
 0x1500739b => {
  cpu.push(0x150073a0); f_15006c78(cpu);
  pc = 0x150073a0; continue;
 }
 0x150073a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x150073a2; continue;
 }
 0x150073a2 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x1500738c; continue; }
  pc = 0x150073a6; continue;
 }
 0x150073a6 => {
  let t = cpu.r32(0x15016b80u32); cpu.push(0x150073ac); cpu.call(t);
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150073c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150073c0); }
 let mut pc: u32 = 0x150073c0;
 loop { match pc {
 0x150073c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010fb4u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x15010fa8u32), cpu.ebp); }
  if !cpu.zf { pc = 0x150073e7; continue; }
  pc = 0x150073d7; continue;
 }
 0x150073d7 => {
  { cpu.sub32(cpu.r32(0x15010fa0u32), cpu.eax); }
  if !cpu.zf { pc = 0x150073e7; continue; }
  pc = 0x150073df; continue;
 }
 0x150073df => {
  { cpu.sub32(cpu.r32(0x15010fa4u32), cpu.ebx); }
  if cpu.zf { pc = 0x15007429; continue; }
  pc = 0x150073e7; continue;
 }
 0x150073e7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010fa0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010fa4u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010fa8u32, v); }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010facu32, v); }
  { let v = (cpu.r32(0x15010fa0u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011038u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1501103cu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15010fa4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011038u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15011040u32, v); cpu.fpop(); }
  pc = 0x15007429; continue;
 }
 0x15007429 => {
  { cpu.sub32(cpu.r32(0x15010fd8u32), cpu.ecx); }
  if !cpu.zf { pc = 0x1500744e; continue; }
  pc = 0x15007431; continue;
 }
 0x15007431 => {
  { cpu.sub32(cpu.r32(0x15010fdcu32), cpu.edx); }
  if !cpu.zf { pc = 0x1500744e; continue; }
  pc = 0x15007439; continue;
 }
 0x15007439 => {
  { cpu.sub32(cpu.r32(0x15010fe8u32), cpu.esi); }
  if !cpu.zf { pc = 0x1500744e; continue; }
  pc = 0x15007441; continue;
 }
 0x15007441 => {
  { cpu.sub32(cpu.r32(0x15010fbcu32), cpu.edi); }
  if !cpu.zf { pc = 0x1500744e; continue; }
  pc = 0x15007449; continue;
 }
 0x15007449 => {
  pc = 0x15007594; continue;
 }
 0x1500744e => {
  { let v: u32 = cpu.ecx; cpu.w32(0x15010fd8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15010fdcu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15010fe8u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15010fbcu32, v); }
  { let v = (cpu.r32(0x15010fe8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15010fecu32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x15010fecu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15010ff0u32, v); cpu.fpop(); }
  { let v = cpu.rf64(0x1501000cu32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x15010fecu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15011004u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x15010ffcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15011000u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x15010fecu32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x15010ffcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15010ff4u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15010fd8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15010fe0u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15010fdcu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15010fe4u32, v); cpu.fpop(); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010fc0u32, v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010fc4u32, v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010fc8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15010fccu32, v); }
  { let v = (cpu.r32(0x15010fc8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15010fd0u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15010fccu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15010fd4u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15010fc8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x15010fe8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x15010fe8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x15010004u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x15011020u32, v); cpu.fpop(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x15011020u32)); cpu.w32(0x15011020u32, v); }
  { let v = (cpu.r32(0x15010fccu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x15010fe8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x15010fe8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x15010004u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x15011024u32, v); cpu.fpop(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x15011024u32)); cpu.w32(0x15011024u32, v); }
  { let v = (cpu.r32(0x15010fc8u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x15010fe8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x15010004u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x15011028u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15010fccu32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x15010fe8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x15010004u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x1501102cu32, v); cpu.fpop(); }
  pc = 0x15007594; continue;
 }
 0x15007594 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007596(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007596); }
 let mut pc: u32 = 0x15007596;
 loop { match pc {
 0x15007596 => {
  { cpu.pushad(); }
  cpu.push(0x1500759c); f_1500a9a7(cpu);
  cpu.push(0x150075a1); f_1500a9aa(cpu);
  cpu.push(0x150075a6); f_1500761a(cpu);
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  cpu.push(0x150075b0); f_150075c4(cpu);
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150075c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150075c4); }
 let mut pc: u32 = 0x150075c4;
 loop { match pc {
 0x150075c4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010ff8u32, v); }
  { let v = (cpu.r32(0x15010ff8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15010ffcu32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x15010ffcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15011000u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x15010fecu32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x15010ffcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15010ff4u32, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150075f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150075f6); }
 let mut pc: u32 = 0x150075f6;
 loop { match pc {
 0x150075f6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x15011014u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15011018u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1501101cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15011008u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1501100cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x15011010u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500761a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500761a); }
 let mut pc: u32 = 0x1500761a;
 loop { match pc {
 0x1500761a => {
  { let v = cpu.eax; cpu.push(v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v = cpu.fpu_control; cpu.w16(0x15010014u32, v); }
  { let v: u16 = cpu.r16(0x15010014u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfcffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xf3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0xc00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xffc0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x15010016u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500764a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500764a); }
 let mut pc: u32 = 0x1500764a;
 loop { match pc {
 0x1500764a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150414e8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x150414ecu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x150414f0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007665(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007665); }
 let mut pc: u32 = 0x15007665;
 loop { match pc {
 0x15007665 => {
  { let v: u32 = cpu.r32(0x15041c64u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007670); }
 let mut pc: u32 = 0x15007670;
 loop { match pc {
 0x15007670 => {
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  if !cpu.cf { pc = 0x15007692; continue; }
  pc = 0x15007674; continue;
 }
 0x15007674 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffc0ffc0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1f001fu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x15007692; continue;
 }
 0x15007692 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffc0ffc0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1f001fu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15007692; continue; }
  pc = 0x150076b0; continue;
 }
 0x150076b0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007700); }
 let mut pc: u32 = 0x15007700;
 loop { match pc {
 0x15007700 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15016618u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15010f30u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15010f34u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15011984u32, v); }
  { let v: u32 = cpu.r32(0x1501197cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15011980u32, v); }
  { cpu.sub32(cpu.r32(0x15010b20u32), 0x0u32); }
  if !cpu.zf { pc = 0x1500773e; continue; }
  pc = 0x15007739; continue;
 }
 0x15007739 => {
  cpu.push(0x1500773e); f_1500a9a8(cpu);
  pc = 0x1500773e; continue;
 }
 0x1500773e => {
  { let v: u32 = 0x15042218u32; cpu.w32(0x15042214u32, v); }
  { let v: u32 = 0x15042218u32; cpu.w32(0x15042214u32, v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x15010016u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x150414e8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x150414ecu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x150414f0u32, v); }
  { let v: u32 = 0x15016634u32; cpu.eax = v; }
  { let v: u32 = 0x15016658u32; cpu.ebx = v; }
  cpu.push(0x15007788); f_1500e2ab(cpu);
  { let v: u32 = cpu.r32(0x15016638u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15016644u32); cpu.ebx = v; }
  cpu.push(0x15007798); f_1500e09f(cpu);
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  { let v: u32 = 0x150166e8u32; cpu.edi = v; }
  cpu.push(0x150077a7); f_1500deaf(cpu);
  { let v: u32 = cpu.r32(0x1501664cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15016654u32); cpu.ebx = v; }
  cpu.push(0x150077b7); f_1500e09f(cpu);
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  { let v: u32 = 0x15016730u32; cpu.edi = v; }
  cpu.push(0x150077c8); f_1500ddd4(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150077ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150077ca); }
 let mut pc: u32 = 0x150077ca;
 loop { match pc {
 0x150077ca => {
  { let v = cpu.ebp; cpu.push(v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x15010014u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007801(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007801); }
 let mut pc: u32 = 0x15007801;
 loop { match pc {
 0x15007801 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15010030u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010034u32); cpu.ebx = v; }
  cpu.push(0x15007812); f_15007d1c(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007814(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007814); }
 let mut pc: u32 = 0x15007814;
 loop { match pc {
 0x15007814 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15010038u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1501003cu32); cpu.ebx = v; }
  cpu.push(0x15007825); f_15007c50(cpu);
  { let v: u32 = cpu.r32(0x15010038u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1501003cu32); cpu.ebx = v; }
  cpu.push(0x15007835); f_15007d1c(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007837(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007837); }
 let mut pc: u32 = 0x15007837;
 loop { match pc {
 0x15007837 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500785f; continue; }
  pc = 0x1500783b; continue;
 }
 0x1500783b => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15017d44u32, v); }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15017d48u32, v); }
  pc = 0x1500784a; continue;
 }
 0x1500784a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x15007850); f_15007860(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x15017d44u32)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15017d48u32)); }
  if !cpu.zf { pc = 0x1500784a; continue; }
  pc = 0x1500785f; continue;
 }
 0x1500785f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007860); }
 let mut pc: u32 = 0x15007860;
 loop { match pc {
 0x15007860 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x15007b29; continue; }
  pc = 0x1500786c; continue;
 }
 0x1500786c => {
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15010080u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1501661cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x15016620u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x15016624u32)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x15011014u32)); }
  if (cpu.sf != cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x150078a5; continue;
 }
 0x150078a5 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x15011018u32)); }
  if (cpu.sf != cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x150078b4; continue;
 }
 0x150078b4 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1501101cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x150078c3; continue;
 }
 0x150078c3 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x15011008u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x150078d4; continue;
 }
 0x150078d4 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x1501100cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x150078e3; continue;
 }
 0x150078e3 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x15011010u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x150078f2; continue;
 }
 0x150078f2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x15017d4cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15017d50u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15017d54u32, v); }
  { let v: u32 = cpu.r32(0x1501003cu32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.edx)), 0x1u16); }
  if cpu.zf { pc = 0x15007917; continue; }
  pc = 0x15007911; continue;
 }
 0x15007911 => {
  { let v: u32 = cpu.r32(0x15010034u32); cpu.ebx = v; }
  pc = 0x15007917; continue;
 }
 0x15007917 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15016638u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d4cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15016644u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d50u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15016650u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d54u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x15010ff8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x15007964; continue;
 }
 0x15007964 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x15010fe8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x15007975; continue;
 }
 0x15007975 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15016634u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d4cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15016640u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d50u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1501664cu32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d54u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x15011028u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150079c3; continue; }
  pc = 0x150079c1; continue;
 }
 0x150079c1 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x150079c3; continue;
 }
 0x150079c3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x15011020u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x150079d9; continue;
 }
 0x150079d9 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15007a1b; continue; }
  pc = 0x150079e1; continue;
 }
 0x150079e1 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x1501663cu32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d4cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15016648u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d50u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15016654u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d54u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x15007a89; continue;
 }
 0x15007a1b => {
  { let v: u32 = cpu.r32(0x1501663cu32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d4cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15016648u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d50u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15016654u32); cpu.eax = v; }
  { let v = cpu.r32(0x15017d54u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x1501102cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007a67; continue; }
  pc = 0x15007a65; continue;
 }
 0x15007a65 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x15007a67; continue;
 }
 0x15007a67 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x15011024u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x15007a7d; continue;
 }
 0x15007a7d => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15007a89; continue; }
  pc = 0x15007a85; continue;
 }
 0x15007a85 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x15007a89; continue;
 }
 0x15007a89 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x15010ff8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007aa3; continue; }
  pc = 0x15007a96; continue;
 }
 0x15007a96 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x15010fe8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x15007aa7; continue; }
  pc = 0x15007aa3; continue;
 }
 0x15007aa3 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x2u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x15007aa7; continue;
 }
 0x15007aa7 => {
  { let v: u32 = cpu.r32(0x15010080u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebx)), 0x1u16); }
  if cpu.zf { pc = 0x15007b02; continue; }
  pc = 0x15007ae3; continue;
 }
 0x15007ae3 => {
  { let v: u32 = cpu.r32(0x15010034u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x15019cbcu32); }
  if (cpu.sf == cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x15007af8; continue;
 }
 0x15007af8 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15010034u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x15007b1b; continue;
 }
 0x15007b02 => {
  { let v: u32 = cpu.r32(0x1501003cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x150296bcu32); }
  if (cpu.sf == cpu.of) { pc = 0x15007b7e; continue; }
  pc = 0x15007b13; continue;
 }
 0x15007b13 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x1501003cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x15007b1b; continue;
 }
 0x15007b1b => {
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebx)), 0x2u16); }
  if cpu.zf { pc = 0x15007b77; continue; }
  pc = 0x15007b29; continue;
 }
 0x15007b29 => {
  { cpu.sub32(cpu.r32(0x70u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x15007b77; continue; }
  pc = 0x15007b2f; continue;
 }
 0x15007b2f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x15007b33; continue;
 }
 0x15007b33 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x40u32.wrapping_add(cpu.esi); cpu.esi = v; }
  cpu.push(0x15007b42); f_1500cfca(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.edx))); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esi); cpu.esi = v; }
  cpu.push(0x15007b67); f_1500dbbf(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15007b6f); f_15007860(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15007b33; continue; }
  pc = 0x15007b76; continue;
 }
 0x15007b76 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x15007b77; continue;
 }
 0x15007b77 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15007b7e => {
  pc = 0x15007b1b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007c50); }
 let mut pc: u32 = 0x15007c50;
 loop { match pc {
 0x15007c50 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15007cca; continue; }
  f_15007c59(cpu); return;
 }
 0x15007cca => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007c59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007c59); }
 let mut pc: u32 = 0x15007c59;
 loop { match pc {
 0x15007c59 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x24u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15007ccb; continue; }
  pc = 0x15007c62; continue;
 }
 0x15007c62 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x15007c71; continue;
 }
 0x15007c71 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007c89; continue; }
  pc = 0x15007c78; continue;
 }
 0x15007c78 => {
  if cpu.zf { pc = 0x15007c7f; continue; }
  pc = 0x15007c7a; continue;
 }
 0x15007c7a => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x15007c71; continue;
 }
 0x15007c7f => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if !cpu.cf { pc = 0x15007c89; continue; }
  pc = 0x15007c84; continue;
 }
 0x15007c84 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x15007c71; continue;
 }
 0x15007c89 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.edi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007ca1; continue; }
  pc = 0x15007c90; continue;
 }
 0x15007c90 => {
  if cpu.zf { pc = 0x15007c97; continue; }
  pc = 0x15007c92; continue;
 }
 0x15007c92 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x15007c89; continue;
 }
 0x15007c97 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.edi))); }
  if !cpu.cf { pc = 0x15007ca1; continue; }
  pc = 0x15007c9c; continue;
 }
 0x15007c9c => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x15007c89; continue;
 }
 0x15007ca1 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007caf; continue; }
  pc = 0x15007ca5; continue;
 }
 0x15007ca5 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); let b = cpu.edi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x15007caf; continue;
 }
 0x15007caf => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15007c71; continue; }
  pc = 0x15007cb3; continue;
 }
 0x15007cb3 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x15007cc0; continue; }
  pc = 0x15007cb7; continue;
 }
 0x15007cb7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x15007cbf); f_15007c59(cpu);
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x15007cc0; continue;
 }
 0x15007cc0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x15007cca; continue; }
  pc = 0x15007cc5; continue;
 }
 0x15007cc5 => {
  cpu.push(0x15007cca); f_15007c59(cpu);
  pc = 0x15007cca; continue;
 }
 0x15007cca => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15007ccb => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x15007ccd; continue;
 }
 0x15007ccd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x15007cf2; continue; }
  pc = 0x15007cdf; continue;
 }
 0x15007cdf => {
  if cpu.zf { pc = 0x15007ce7; continue; }
  pc = 0x15007ce1; continue;
 }
 0x15007ce1 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { pc = 0x15007ccd; continue; }
  pc = 0x15007ce5; continue;
 }
 0x15007ce5 => {
  pc = 0x15007d1b; continue;
 }
 0x15007ce7 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if cpu.cf { pc = 0x15007cf2; continue; }
  pc = 0x15007cec; continue;
 }
 0x15007cec => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { pc = 0x15007ccd; continue; }
  pc = 0x15007cf0; continue;
 }
 0x15007cf0 => {
  pc = 0x15007d1b; continue;
 }
 0x15007cf2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x15007cfb; continue;
 }
 0x15007cfb => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15007d13; continue; }
  pc = 0x15007d02; continue;
 }
 0x15007d02 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007cfb; continue; }
  pc = 0x15007d0c; continue;
 }
 0x15007d0c => {
  if !cpu.zf { pc = 0x15007d13; continue; }
  pc = 0x15007d0e; continue;
 }
 0x15007d0e => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15007cfb; continue; }
  pc = 0x15007d13; continue;
 }
 0x15007d13 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { pc = 0x15007ccd; continue; }
  pc = 0x15007d1b; continue;
 }
 0x15007d1b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007d1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007d1c); }
 let mut pc: u32 = 0x15007d1c;
 loop { match pc {
 0x15007d1c => {
  { cpu.sub32(cpu.r32(0x15010b20u32), 0x0u32); }
  if cpu.zf { pc = 0x15007d35; continue; }
  pc = 0x15007d25; continue;
 }
 0x15007d25 => {
  { cpu.sub8(cpu.r8(0x15016618u32), 0x0u8); }
  if !cpu.zf { pc = 0x15007d35; continue; }
  pc = 0x15007d2e; continue;
 }
 0x15007d2e => {
  { let v: u8 = 0x1u8; cpu.w8(0x15016618u32, v); }
  pc = 0x15007d35; continue;
 }
 0x15007d35 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x15007dff; continue; }
  pc = 0x15007d3d; continue;
 }
 0x15007d3d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15010070u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010064u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010068u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1501006cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010074u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010078u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15010084u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15007df2; continue; }
  pc = 0x15007d8a; continue;
 }
 0x15007d8a => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010080u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15010070u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1501012cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1501002cu32, v); }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = 0x15041c64u32; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x5dc0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15007dc9; continue; }
  pc = 0x15007db7; continue;
 }
 0x15007db7 => {
  { let v: u32 = 0x15041c64u32; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x13880u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15007dc9; continue; }
  pc = 0x15007dc4; continue;
 }
 0x15007dc4 => {
  { let v: u32 = 0x15041c64u32; cpu.eax = v; }
  pc = 0x15007dc9; continue;
 }
 0x15007dc9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x15041c64u32, v); }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x32u32); cpu.esi = v; }
  pc = 0x15007dd7; continue;
 }
 0x15007dd7 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x15007df2; continue; }
  pc = 0x15007de1; continue;
 }
 0x15007de1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x1501008cu32.wrapping_add(cpu.ebx)); cpu.push(0x15007dec); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x15007dd7; continue;
 }
 0x15007df2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x15007d3d; continue; }
  pc = 0x15007dff; continue;
 }
 0x15007dff => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007e29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007e29); }
 let mut pc: u32 = 0x15007e29;
 loop { match pc {
 0x15007e29 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x15041c64u32)); cpu.w32(0x15041c64u32, v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  f_15007e31(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007e31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007e31); }
 let mut pc: u32 = 0x15007e31;
 loop { match pc {
 0x15007e31 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x15007e42; continue; }
  pc = 0x15007e3b; continue;
 }
 0x15007e3b => {
  cpu.push(0x15007e40); f_15007e8c(cpu);
  pc = 0x15007e47; continue;
 }
 0x15007e42 => {
  cpu.push(0x15007e47); f_15007e48(cpu);
  pc = 0x15007e47; continue;
 }
 0x15007e47 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007e48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007e48); }
 let mut pc: u32 = 0x15007e48;
 loop { match pc {
 0x15007e48 => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x15007e68; continue; }
  pc = 0x15007e55; continue;
 }
 0x15007e55 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010044u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15087f8cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x15007e75; continue;
 }
 0x15007e68 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010044u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15087f8cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  pc = 0x15007e75; continue;
 }
 0x15007e75 => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0xfffffffau32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { pc = 0x15007e7f; continue; }
  pc = 0x15007e7d; continue;
 }
 0x15007e7d => {
  { let a = cpu.ebx; let b = cpu.edx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.edx = v; } }
  pc = 0x15007e7f; continue;
 }
 0x15007e7f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x15007e85); f_15007e31(cpu);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.push(0x15007e8b); f_15007e31(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007e8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007e8c); }
 let mut pc: u32 = 0x15007e8c;
 loop { match pc {
 0x15007e8c => {
  { let v: u32 = cpu.r32(0x150414e8u32).wrapping_add(cpu.eax); cpu.w32(0x150414e8u32, v); }
  pc = 0x15007e92; continue;
 }
 0x15007e92 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15007eb0; continue; }
  pc = 0x15007e9b; continue;
 }
 0x15007e9b => {
  if cpu.zf { pc = 0x15007ec2; continue; }
  pc = 0x15007e9d; continue;
 }
 0x15007e9d => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010044u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15087f8cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x15007ebd; continue;
 }
 0x15007eb0 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010044u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15087f8cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  pc = 0x15007ebd; continue;
 }
 0x15007ebd => {
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { pc = 0x15007eda; continue; }
  pc = 0x15007ec2; continue;
 }
 0x15007ec2 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x150414ecu32)); cpu.w32(0x150414ecu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1501002cu32)); cpu.ebx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.push(0x15007ed9); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x15007eda; continue;
 }
 0x15007eda => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x15007e92; continue; }
  pc = 0x15007ee6; continue;
 }
 0x15007ee6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15007f7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15007f7e); }
 let mut pc: u32 = 0x15007f7e;
 loop { match pc {
 0x15007f7e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008b40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008b40); }
 let mut pc: u32 = 0x15008b40;
 loop { match pc {
 0x15008b40 => {
  { let v: u32 = 0x0u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008b4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008b4b); }
 let mut pc: u32 = 0x15008b4b;
 loop { match pc {
 0x15008b4b => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.esi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008b66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008b66); }
 let mut pc: u32 = 0x15008b66;
 loop { match pc {
 0x15008b66 => {
  { let v: u32 = 0x150166e8u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008b71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008b71); }
 let mut pc: u32 = 0x15008b71;
 loop { match pc {
 0x15008b71 => {
  { let v: u32 = 0x1501670cu32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008b7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008b7c); }
 let mut pc: u32 = 0x15008b7c;
 loop { match pc {
 0x15008b7c => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15008b8c; continue; }
  pc = 0x15008b89; continue;
 }
 0x15008b89 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x15008b8c; continue;
 }
 0x15008b8c => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008b9f); f_1500da56(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008baa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008baa); }
 let mut pc: u32 = 0x15008baa;
 loop { match pc {
 0x15008baa => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15008bba; continue; }
  pc = 0x15008bb7; continue;
 }
 0x15008bb7 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x15008bba; continue;
 }
 0x15008bba => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008bd1); f_1500da56(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008bdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008bdc); }
 let mut pc: u32 = 0x15008bdc;
 loop { match pc {
 0x15008bdc => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15008bec; continue; }
  pc = 0x15008be9; continue;
 }
 0x15008be9 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x15008bec; continue;
 }
 0x15008bec => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008c03); f_1500d8f3(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008c0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008c0e); }
 let mut pc: u32 = 0x15008c0e;
 loop { match pc {
 0x15008c0e => {
  { let v: u32 = 0x15016634u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008c19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008c19); }
 let mut pc: u32 = 0x15008c19;
 loop { match pc {
 0x15008c19 => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.esi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008c31); f_1500dbbf(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008c47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008c47); }
 let mut pc: u32 = 0x15008c47;
 loop { match pc {
 0x15008c47 => {
  { let v: u32 = 0x150166e8u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008c52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008c52); }
 let mut pc: u32 = 0x15008c52;
 loop { match pc {
 0x15008c52 => {
  { let v: u32 = 0x1501670cu32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008c5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008c5d); }
 let mut pc: u32 = 0x15008c5d;
 loop { match pc {
 0x15008c5d => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15008c6d; continue; }
  pc = 0x15008c6a; continue;
 }
 0x15008c6a => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x15008c6d; continue;
 }
 0x15008c6d => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008c81); f_1500deaf(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008c8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008c8c); }
 let mut pc: u32 = 0x15008c8c;
 loop { match pc {
 0x15008c8c => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15008c9c; continue; }
  pc = 0x15008c99; continue;
 }
 0x15008c99 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x15008c9c; continue;
 }
 0x15008c9c => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x15008cd9; continue; }
  pc = 0x15008cb0; continue;
 }
 0x15008cb0 => {
  { let v: u32 = 0x150166a0u32; cpu.edi = v; }
  cpu.push(0x15008cba); f_1500da56(cpu);
  { let v: u32 = 0x150166a0u32; cpu.esi = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008cce); f_1500dbbf(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008cd9 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x150166a0u32; cpu.edi = v; }
  cpu.push(0x15008ce6); f_1500d8b9(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008cf6); f_1500deaf(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008d01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008d01); }
 let mut pc: u32 = 0x15008d01;
 loop { match pc {
 0x15008d01 => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15008d11; continue; }
  pc = 0x15008d0e; continue;
 }
 0x15008d0e => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x15008d11; continue;
 }
 0x15008d11 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x15008d4e; continue; }
  pc = 0x15008d25; continue;
 }
 0x15008d25 => {
  { let v: u32 = 0x150166a0u32; cpu.edi = v; }
  cpu.push(0x15008d2f); f_1500d8f3(cpu);
  { let v: u32 = 0x150166a0u32; cpu.esi = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008d43); f_1500dbbf(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008d4e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x150166a0u32; cpu.edi = v; }
  cpu.push(0x15008d5b); f_1500d8b9(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008d6b); f_1500deaf(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008d76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008d76); }
 let mut pc: u32 = 0x15008d76;
 loop { match pc {
 0x15008d76 => {
  pc = 0x1500d653; continue;
 }
 0x1500d653 => {
  { cpu.sub32(cpu.r32(0x15010060u32), 0x0u32); }
  if cpu.zf { pc = 0x1500d70e; continue; }
  pc = 0x1500d660; continue;
 }
 0x1500d660 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010058u32, v); }
  { let v: u32 = cpu.r32(0x15010044u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = 0x15088f34u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15010060u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508218cu32, v); }
  pc = 0x1500d690; continue;
 }
 0x1500d690 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1508218cu32), cpu.edi); }
  if !cpu.zf { pc = 0x1500d690; continue; }
  pc = 0x1500d70c; continue;
 }
 0x1500d70c => {
  pc = 0x1500d737; continue;
 }
 0x1500d70e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010058u32, v); }
  { let v: u32 = cpu.r32(0x15010044u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = 0x15088f34u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { pc = 0x1500d737; continue; }
  pc = 0x1500d735; continue;
 }
 0x1500d735 => {
  { cpu.movs(2, false); }
  pc = 0x1500d737; continue;
 }
 0x1500d737 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008d7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008d7b); }
 let mut pc: u32 = 0x15008d7b;
 loop { match pc {
 0x15008d7b => {
  pc = 0x1500d55f; continue;
 }
 0x1500d55f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010054u32, v); }
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15010060u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508218cu32, v); }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x1500d622; continue; }
  pc = 0x1500d59a; continue;
 }
 0x1500d59a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010064u32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010068u32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1501006cu32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1508218cu32), cpu.edi); }
  if !cpu.zf { pc = 0x1500d59a; continue; }
  pc = 0x1500d620; continue;
 }
 0x1500d620 => {
  pc = 0x1500d652; continue;
 }
 0x1500d622 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010064u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x15010068u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1501006cu32)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1508218cu32), cpu.edi); }
  if !cpu.zf { pc = 0x1500d622; continue; }
  pc = 0x1500d652; continue;
 }
 0x1500d652 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008d80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008d80); }
 let mut pc: u32 = 0x15008d80;
 loop { match pc {
 0x15008d80 => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x15008d90; continue; }
  pc = 0x15008d8d; continue;
 }
 0x15008d8d => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x15008d90; continue;
 }
 0x15008d90 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15010060u32); cpu.ebx = v; }
  { let v: u32 = 0x150102f0u32; cpu.esi = v; }
  cpu.push(0x15008da3); f_1500cfca(cpu);
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15010064u32).wrapping_add(cpu.eax); cpu.w32(0x15010064u32, v); }
  { let v: u32 = cpu.r32(0x15010068u32).wrapping_add(cpu.ebx); cpu.w32(0x15010068u32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x1501006cu32), cpu.edx); cpu.w32(0x1501006cu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x150166a0u32; cpu.edi = v; }
  cpu.push(0x15008dcd); f_1500da56(cpu);
  { let v: u32 = cpu.r32(0x15010060u32); cpu.esi = v; }
  { let v: u32 = 0x150371b8u32; cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x150166a0u32; cpu.esi = v; }
  { let v: u32 = 0x150371b8u32; cpu.ebx = v; }
  { let v: u32 = 0x150166c4u32; cpu.edi = v; }
  cpu.push(0x15008df3); f_1500dbbf(cpu);
  { let v: u32 = 0x150166c4u32; cpu.w32(0x15010060u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008dfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008dfe); }
 let mut pc: u32 = 0x15008dfe;
 loop { match pc {
 0x15007e00 => {
  { cpu.sub32(cpu.r32(0x15010088u32), 0x0u32); }
  if !cpu.zf { pc = 0x15007e13; continue; }
  pc = 0x15007e09; continue;
 }
 0x15007e09 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1501002cu32), 0xb0u32); cpu.w32(0x1501002cu32, v); }
  pc = 0x15007e13; continue;
 }
 0x15007e13 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008dfe => {
  pc = 0x15007e00; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008e03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008e03); }
 let mut pc: u32 = 0x15008e03;
 loop { match pc {
 0x15008e03 => {
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15008e1a; continue; }
  pc = 0x15008e0e; continue;
 }
 0x15008e0e => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010080u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x2eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x15008e1a; continue;
 }
 0x15008e1a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008e2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008e2f); }
 let mut pc: u32 = 0x15008e2f;
 loop { match pc {
 0x15008e2f => {
  { let v: u32 = cpu.r32(0x15010080u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15010068u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15008e5c; continue; }
  pc = 0x15008e49; continue;
 }
 0x15008e49 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x15008e56; continue; }
  pc = 0x15008e50; continue;
 }
 0x15008e50 => {
  pc = 0x1500a990; continue;
 }
 0x15008e56 => {
  pc = 0x1500a990; continue;
 }
 0x15008e5c => {
  pc = 0x1500a990; continue;
 }
 0x1500a990 => {
  { let v: u32 = 0x0u32; cpu.w32(0x15011064u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008e62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008e62); }
 let mut pc: u32 = 0x15008e62;
 loop { match pc {
 0x15008e62 => {
  pc = 0x1500a99b; continue;
 }
 0x1500a99b => {
  { let v: u32 = 0x6u32; cpu.w32(0x15011064u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008e67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008e67); }
 let mut pc: u32 = 0x15008e67;
 loop { match pc {
 0x15008e67 => {
  pc = 0x1500a9a6; continue;
 }
 0x1500a9a6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008eab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008eab); }
 let mut pc: u32 = 0x15008eab;
 loop { match pc {
 0x15008e6c => {
  { let v: u32 = cpu.r32(0x15010068u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x15008e81; continue;
 }
 0x15008e81 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x15008e91; continue; }
  pc = 0x15008e85; continue;
 }
 0x15008e85 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15008e81; continue; }
  pc = 0x15008e8e; continue;
 }
 0x15008e8e => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x15008e91; continue;
 }
 0x15008e91 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15008eaa; continue; }
  pc = 0x15008e9e; continue;
 }
 0x15008e9e => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010080u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x2eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x15008eaa; continue;
 }
 0x15008eaa => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008eab => {
  { let v: u32 = 0x150102b0u32; cpu.edi = v; }
  pc = 0x15008e6c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008eb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008eb2); }
 let mut pc: u32 = 0x15008eb2;
 loop { match pc {
 0x15008e6c => {
  { let v: u32 = cpu.r32(0x15010068u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x15008e81; continue;
 }
 0x15008e81 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x15008e91; continue; }
  pc = 0x15008e85; continue;
 }
 0x15008e85 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15008e81; continue; }
  pc = 0x15008e8e; continue;
 }
 0x15008e8e => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x15008e91; continue;
 }
 0x15008e91 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15008eaa; continue; }
  pc = 0x15008e9e; continue;
 }
 0x15008e9e => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010080u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x2eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x15008eaa; continue;
 }
 0x15008eaa => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008eb2 => {
  { let v: u32 = 0x150102c0u32; cpu.edi = v; }
  pc = 0x15008e6c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008eb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008eb9); }
 let mut pc: u32 = 0x15008eb9;
 loop { match pc {
 0x15008e6c => {
  { let v: u32 = cpu.r32(0x15010068u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x15008e81; continue;
 }
 0x15008e81 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x15008e91; continue; }
  pc = 0x15008e85; continue;
 }
 0x15008e85 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15008e81; continue; }
  pc = 0x15008e8e; continue;
 }
 0x15008e8e => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x15008e91; continue;
 }
 0x15008e91 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15008eaa; continue; }
  pc = 0x15008e9e; continue;
 }
 0x15008e9e => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010080u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x2eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x15008eaa; continue;
 }
 0x15008eaa => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008eb9 => {
  { let v: u32 = 0x150102d0u32; cpu.edi = v; }
  pc = 0x15008e6c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008ec0); }
 let mut pc: u32 = 0x15008ec0;
 loop { match pc {
 0x15008e6c => {
  { let v: u32 = cpu.r32(0x15010068u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x15008e81; continue;
 }
 0x15008e81 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x15008e91; continue; }
  pc = 0x15008e85; continue;
 }
 0x15008e85 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x15008e81; continue; }
  pc = 0x15008e8e; continue;
 }
 0x15008e8e => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x15008e91; continue;
 }
 0x15008e91 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15008eaa; continue; }
  pc = 0x15008e9e; continue;
 }
 0x15008e9e => {
  { let v: u32 = cpu.eax; cpu.w32(0x15010080u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x2eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x15008eaa; continue;
 }
 0x15008eaa => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008ec0 => {
  { let v: u32 = 0x150102e0u32; cpu.edi = v; }
  pc = 0x15008e6c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008ede(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008ede); }
 let mut pc: u32 = 0x15008ede;
 loop { match pc {
 0x15007e00 => {
  { cpu.sub32(cpu.r32(0x15010088u32), 0x0u32); }
  if !cpu.zf { pc = 0x15007e13; continue; }
  pc = 0x15007e09; continue;
 }
 0x15007e09 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1501002cu32), 0xb0u32); cpu.w32(0x1501002cu32, v); }
  pc = 0x15007e13; continue;
 }
 0x15007e13 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008ec7 => {
  { let v: u32 = cpu.r32(0x15010068u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x15008edd; continue; }
  pc = 0x15008ed8; continue;
 }
 0x15008ed8 => {
  pc = 0x15007e00; continue;
 }
 0x15008edd => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008ede => {
  { let v: u32 = 0x150102b0u32; cpu.edi = v; }
  pc = 0x15008ec7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008ee5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008ee5); }
 let mut pc: u32 = 0x15008ee5;
 loop { match pc {
 0x15007e00 => {
  { cpu.sub32(cpu.r32(0x15010088u32), 0x0u32); }
  if !cpu.zf { pc = 0x15007e13; continue; }
  pc = 0x15007e09; continue;
 }
 0x15007e09 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1501002cu32), 0xb0u32); cpu.w32(0x1501002cu32, v); }
  pc = 0x15007e13; continue;
 }
 0x15007e13 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008ec7 => {
  { let v: u32 = cpu.r32(0x15010068u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x15008edd; continue; }
  pc = 0x15008ed8; continue;
 }
 0x15008ed8 => {
  pc = 0x15007e00; continue;
 }
 0x15008edd => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008ee5 => {
  { let v: u32 = 0x150102c0u32; cpu.edi = v; }
  pc = 0x15008ec7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008eec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008eec); }
 let mut pc: u32 = 0x15008eec;
 loop { match pc {
 0x15007e00 => {
  { cpu.sub32(cpu.r32(0x15010088u32), 0x0u32); }
  if !cpu.zf { pc = 0x15007e13; continue; }
  pc = 0x15007e09; continue;
 }
 0x15007e09 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1501002cu32), 0xb0u32); cpu.w32(0x1501002cu32, v); }
  pc = 0x15007e13; continue;
 }
 0x15007e13 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008ec7 => {
  { let v: u32 = cpu.r32(0x15010068u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x15008edd; continue; }
  pc = 0x15008ed8; continue;
 }
 0x15008ed8 => {
  pc = 0x15007e00; continue;
 }
 0x15008edd => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008eec => {
  { let v: u32 = 0x150102d0u32; cpu.edi = v; }
  pc = 0x15008ec7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008ef3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008ef3); }
 let mut pc: u32 = 0x15008ef3;
 loop { match pc {
 0x15007e00 => {
  { cpu.sub32(cpu.r32(0x15010088u32), 0x0u32); }
  if !cpu.zf { pc = 0x15007e13; continue; }
  pc = 0x15007e09; continue;
 }
 0x15007e09 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1501002cu32), 0xb0u32); cpu.w32(0x1501002cu32, v); }
  pc = 0x15007e13; continue;
 }
 0x15007e13 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008ec7 => {
  { let v: u32 = cpu.r32(0x15010068u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x15008edd; continue; }
  pc = 0x15008ed8; continue;
 }
 0x15008ed8 => {
  pc = 0x15007e00; continue;
 }
 0x15008edd => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008ef3 => {
  { let v: u32 = 0x150102e0u32; cpu.edi = v; }
  pc = 0x15008ec7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008efa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008efa); }
 let mut pc: u32 = 0x15008efa;
 loop { match pc {
 0x15008efa => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.cf { pc = 0x15008f11; continue; }
  pc = 0x15008eff; continue;
 }
 0x15008eff => {
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if !cpu.cf { pc = 0x15008f11; continue; }
  pc = 0x15008f04; continue;
 }
 0x15008f04 => {
  { let v: u32 = cpu.r32(0x150102a0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15008f11 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008f17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008f17); }
 let mut pc: u32 = 0x15008f17;
 loop { match pc {
 0x15008f17 => {
  { let v: u32 = 0x30u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.imul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x15010064u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15010060u32); cpu.esi = v; }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { pc = 0x15008f3c; continue; }
  pc = 0x15008f3a; continue;
 }
 0x15008f3a => {
  { cpu.movs(2, false); }
  pc = 0x15008f3c; continue;
 }
 0x15008f3c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008f3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008f3d); }
 let mut pc: u32 = 0x15008f3d;
 loop { match pc {
 0x15008f3d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150102f0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x150102f4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x150102f8u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010040u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010044u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010048u32, v); }
  { let v: u32 = cpu.r32(0x150102fcu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15010060u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15010060u32); cpu.esi = v; }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { pc = 0x15008f90; continue; }
  pc = 0x15008f8e; continue;
 }
 0x15008f8e => {
  { cpu.movs(2, false); }
  pc = 0x15008f90; continue;
 }
 0x15008f90 => {
  { let v: u32 = 0x15010064u32; cpu.esi = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edi; cpu.w32(0x150102fcu32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15008fa3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15008fa3); }
 let mut pc: u32 = 0x15008fa3;
 loop { match pc {
 0x15008fa3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010040u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010044u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010048u32, v); }
  { let v: u32 = cpu.r32(0x150102fcu32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x34u32); cpu.esi = v; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x15010060u32, v); }
  { let v: u32 = cpu.r32(0x15010060u32); cpu.edi = v; }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { pc = 0x15008fe6; continue; }
  pc = 0x15008fe4; continue;
 }
 0x15008fe4 => {
  { cpu.movs(2, false); }
  pc = 0x15008fe6; continue;
 }
 0x15008fe6 => {
  { let v: u32 = 0x15010064u32; cpu.edi = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.sub32(cpu.r32(0x150102fcu32), 0x34u32); cpu.w32(0x150102fcu32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009000); }
 let mut pc: u32 = 0x15009000;
 loop { match pc {
 0x15009000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010064u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010068u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1501006cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15010084u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2eu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x15009030; continue;
 }
 0x15009030 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x1500904d; continue; }
  pc = 0x15009037; continue;
 }
 0x15009037 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x1501008cu32.wrapping_add(cpu.ebx)); cpu.push(0x15009046); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x15009030; continue;
 }
 0x1500904d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150090a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150090a0); }
 let mut pc: u32 = 0x150090a0;
 loop { match pc {
 0x150090a0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x150371e8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x150371ecu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x150371f0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x150371f4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x150371f8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15037208u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150090c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150090c8); }
 let mut pc: u32 = 0x150090c8;
 loop { match pc {
 0x150090c8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150371f4u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150371fcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15037200u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037204u32, v); }
  { cpu.sub32(cpu.r32(0x150371f4u32), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009104; continue; }
  pc = 0x150090f8; continue;
 }
 0x150090f8 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150090fe); f_1500e045(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x150371f4u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x15009104; continue;
 }
 0x15009104 => {
  { let v: u32 = cpu.r32(0x150371fcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15037200u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15037204u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150371e8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x150371ecu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x150371f0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15037208u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500913a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500913a); }
 let mut pc: u32 = 0x1500913a;
 loop { match pc {
 0x1500913a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15037380u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1503735cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x15037354u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037350u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15037358u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15009239; continue; }
  pc = 0x15009165; continue;
 }
 0x15009165 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1500916a; continue;
 }
 0x1500916a => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 1u32); cpu.edi = v; }
  if cpu.zf { pc = 0x15009190; continue; }
  pc = 0x15009172; continue;
 }
 0x15009172 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.r32(0x15037354u32); cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x150371ecu32)); cpu.eax = v; }
  if (cpu.sf != cpu.of) { pc = 0x1500918c; continue; }
  pc = 0x15009188; continue;
 }
 0x15009188 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x1500916a; continue;
 }
 0x1500918c => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x1500916a; continue;
 }
 0x15009190 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037364u32, v); }
  { let v = cpu.r32(0x15037354u32); cpu.imul32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x15037360u32, v); }
  pc = 0x150091aa; continue;
 }
 0x150091aa => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x150371ecu32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150091b7; continue; }
  pc = 0x150091b5; continue;
 }
 0x150091b5 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x150091b7; continue;
 }
 0x150091b7 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x15037350u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150091e1; continue; }
  pc = 0x150091bf; continue;
 }
 0x150091bf => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15037358u32); cpu.ebx = v; }
  cpu.push(0x150091cc); f_150092dc(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150091d8; continue; }
  pc = 0x150091d2; continue;
 }
 0x150091d2 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x15037368u32, v); }
  pc = 0x150091d8; continue;
 }
 0x150091d8 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x15037354u32)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x150091aa; continue; }
  pc = 0x150091e1; continue;
 }
 0x150091e1 => {
  { let v: u32 = cpu.r32(0x15037360u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15037364u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x15037354u32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x1503735cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x15009239; continue; }
  pc = 0x150091fc; continue;
 }
 0x150091fc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x150371ecu32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009209; continue; }
  pc = 0x15009207; continue;
 }
 0x15009207 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x15009209; continue;
 }
 0x15009209 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x15037350u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009239; continue; }
  pc = 0x15009211; continue;
 }
 0x15009211 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15037358u32); cpu.ebx = v; }
  cpu.push(0x1500921e); f_150092dc(cpu);
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500922a; continue; }
  pc = 0x15009224; continue;
 }
 0x15009224 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x15037368u32, v); }
  pc = 0x1500922a; continue;
 }
 0x1500922a => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x15037354u32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x1503735cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x150091fc; continue; }
  pc = 0x15009239; continue;
 }
 0x15009239 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15037380u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x1500924c; continue; }
  pc = 0x15009244; continue;
 }
 0x15009244 => {
  { let v: u32 = cpu.r32(0x15037368u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1500924c; continue;
 }
 0x1500924c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500924e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500924e); }
 let mut pc: u32 = 0x1500924e;
 loop { match pc {
 0x1500924e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15037380u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500929e; continue; }
  pc = 0x1500925e; continue;
 }
 0x1500925e => {
  { let v: u32 = cpu.edx; cpu.w32(0x15037370u32, v); }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1503736cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15037374u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15037378u32, v); }
  pc = 0x15009279; continue;
 }
 0x15009279 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15037374u32); cpu.ebx = v; }
  cpu.push(0x15009285); f_150092dc(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15009290; continue; }
  pc = 0x1500928a; continue;
 }
 0x1500928a => {
  { let v: u32 = cpu.esi; cpu.w32(0x1503737cu32, v); }
  pc = 0x15009290; continue;
 }
 0x15009290 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x15037370u32)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1503736cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15009279; continue; }
  pc = 0x1500929e; continue;
 }
 0x1500929e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15037380u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x150092c3; continue; }
  pc = 0x150092a9; continue;
 }
 0x150092a9 => {
  { let v: u32 = cpu.r32(0x1503737cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037378u32)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0x15037370u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15037380u32); cpu.eax = v; }
  pc = 0x150092c3; continue;
 }
 0x150092c3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150092c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150092c5); }
 let mut pc: u32 = 0x150092c5;
 loop { match pc {
 0x150092c5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15037380u32, v); }
  cpu.push(0x150092d5); f_150092dc(cpu);
  { let v: u32 = cpu.r32(0x15037380u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150092dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150092dc); }
 let mut pc: u32 = 0x150092dc;
 loop { match pc {
 0x150092dc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15013f08u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x15009477; continue; }
  pc = 0x150092ee; continue;
 }
 0x150092ee => {
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebp)), 0x4u16); }
  if cpu.zf { pc = 0x15009386; continue; }
  pc = 0x150092fa; continue;
 }
 0x150092fa => {
  { let v: u32 = cpu.r32(0x150371f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150371f0u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009477; continue; }
  pc = 0x15009316; continue;
 }
 0x15009316 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150371f0u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x15009477; continue; }
  pc = 0x1500932c; continue;
 }
 0x1500932c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150371ecu32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009477; continue; }
  pc = 0x15009342; continue;
 }
 0x15009342 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150371ecu32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x15009477; continue; }
  pc = 0x15009358; continue;
 }
 0x15009358 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150371e8u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009477; continue; }
  pc = 0x1500936e; continue;
 }
 0x1500936e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150371e8u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x15009477; continue; }
  pc = 0x15009384; continue;
 }
 0x15009384 => {
  pc = 0x150093db; continue;
 }
 0x15009386 => {
  { let v: u32 = cpu.r32(0x150371f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x150371ecu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009477; continue; }
  pc = 0x150093a3; continue;
 }
 0x150093a3 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x15009477; continue; }
  pc = 0x150093ab; continue;
 }
 0x150093ab => {
  { let v: u32 = cpu.r32(0x150371e8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009477; continue; }
  pc = 0x150093bb; continue;
 }
 0x150093bb => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x15009477; continue; }
  pc = 0x150093c3; continue;
 }
 0x150093c3 => {
  { let v: u32 = cpu.r32(0x150371f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009477; continue; }
  pc = 0x150093d3; continue;
 }
 0x150093d3 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x15009477; continue; }
  pc = 0x150093db; continue;
 }
 0x150093db => {
  { let v: u32 = cpu.r32(0x150371f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x15010080u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15010084u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x150371dcu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1503720cu32, v); }
  { cpu.sub32(cpu.r32(0x150371dcu32), 0x0u32); }
  if cpu.zf { pc = 0x1500947d; continue; }
  pc = 0x15009405; continue;
 }
 0x15009405 => {
  { let v: u32 = cpu.r32(0x15010084u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1503721cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15037220u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037224u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15010064u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010068u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1501006cu32, v); }
  { let v: u32 = cpu.r32(0x15010080u32); cpu.esi = v; }
  { let v: u32 = 0x2au32.wrapping_add(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x15009477; continue; }
  pc = 0x15009444; continue;
 }
 0x15009444 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.esi = v; }
  pc = 0x15009446; continue;
 }
 0x15009446 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x15009461; continue; }
  pc = 0x15009450; continue;
 }
 0x15009450 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x1501008cu32.wrapping_add(cpu.ebx)); cpu.push(0x1500945b); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x15009446; continue;
 }
 0x15009461 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x15009477; continue; }
  pc = 0x15009467; continue;
 }
 0x15009467 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150371e0u32, v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.push(0x15009476); f_15009488(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15009477 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500947d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15037380u32, v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009488(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009488); }
 let mut pc: u32 = 0x15009488;
 loop { match pc {
 0x15009488 => {
  { let v: u32 = cpu.r32(0x150371e0u32); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x15037388u32, v); }
  cpu.push(0x1500949d); f_1500949e(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500949e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500949e); }
 let mut pc: u32 = 0x1500949e;
 loop { match pc {
 0x1500949e => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x150094bf; continue; }
  pc = 0x150094a8; continue;
 }
 0x150094a8 => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x150094b7); f_1500949e(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x150094bd); f_1500949e(cpu);
  pc = 0x150094e4; continue;
 }
 0x150094bf => {
  { let v: u32 = cpu.eax; cpu.w32(0x15037384u32, v); }
  pc = 0x150094c4; continue;
 }
 0x150094c4 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x150094ca); f_150094ea(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150094d4; continue; }
  pc = 0x150094cf; continue;
 }
 0x150094cf => {
  { let v: u32 = cpu.eax; cpu.w32(0x15037388u32, v); }
  pc = 0x150094d4; continue;
 }
 0x150094d4 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x15037384u32)); cpu.w32(0x15037384u32, v); }
  if !cpu.zf { pc = 0x150094c4; continue; }
  pc = 0x150094e4; continue;
 }
 0x150094e4 => {
  { let v: u32 = cpu.r32(0x15037388u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150094ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150094ea); }
 let mut pc: u32 = 0x150094ea;
 loop { match pc {
 0x150094ea => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x150371dcu32), 0x2u32); }
  if cpu.zf { pc = 0x15009545; continue; }
  pc = 0x150094f4; continue;
 }
 0x150094f4 => {
  { let v: u32 = cpu.r32(0x150371e8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150371ecu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x150371f0u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15037210u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15037214u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037218u32, v); }
  cpu.push(0x1500951b); f_15009714(cpu);
  { cpu.sub32(cpu.eax, cpu.r32(0x150371f4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150095c8; continue; }
  pc = 0x15009527; continue;
 }
 0x15009527 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x150371f8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x150095c8; continue; }
  pc = 0x15009533; continue;
 }
 0x15009533 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x1500953b); f_1500979a(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150095c8; continue; }
  pc = 0x15009543; continue;
 }
 0x15009543 => {
  pc = 0x1500958b; continue;
 }
 0x15009545 => {
  { let v: u32 = cpu.r32(0x150371e8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150371ecu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x150371f0u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15037210u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15037214u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037218u32, v); }
  cpu.push(0x1500956c); f_150095cf(cpu);
  if cpu.zf { pc = 0x150095c8; continue; }
  pc = 0x1500956e; continue;
 }
 0x1500956e => {
  { let v: u32 = cpu.eax; cpu.w32(0x15037210u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15037214u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037218u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x15009587); f_1500979a(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x150095c8; continue; }
  pc = 0x1500958b; continue;
 }
 0x1500958b => {
  { let v: u32 = cpu.r32(0x15037208u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if (cpu.sf == cpu.of) { pc = 0x150095bc; continue; }
  pc = 0x15009595; continue;
 }
 0x15009595 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x15037234u32, v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v: u32 = 0x1503721cu32; cpu.esi = v; }
  { let v: u32 = 0x15037238u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.inc32(cpu.r32(0x15037208u32)); cpu.w32(0x15037208u32, v); }
  pc = 0x150095bc; continue;
 }
 0x150095bc => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15037380u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x150095c8 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150095cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150095cf); }
 let mut pc: u32 = 0x150095cf;
 loop { match pc {
 0x150095cf => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150095dc; continue; }
  pc = 0x150095da; continue;
 }
 0x150095da => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x150095dc; continue;
 }
 0x150095dc => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x15088f34u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x150095fe; continue; }
  pc = 0x150095f8; continue;
 }
 0x150095f8 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x150095fe; continue;
 }
 0x150095fe => {
  { let v: u32 = cpu.eax; cpu.w32(0x15037228u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1503722cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037230u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150371fcu32); cpu.eax = v; }
  { let v = cpu.r32(0x15037228u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15037200u32); cpu.eax = v; }
  { let v = cpu.r32(0x1503722cu32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15037204u32); cpu.eax = v; }
  { let v = cpu.r32(0x15037230u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.fpush(1.0); }
  { let v: u32 = cpu.eax; cpu.w32(0x15037398u32, v); }
  { let v = (cpu.r32(0x15037398u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x15037210u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1503721cu32)); cpu.eax = v; }
  { let v = cpu.r32(0x15037228u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15037214u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037220u32)); cpu.eax = v; }
  { let v = cpu.r32(0x1503722cu32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15037218u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037224u32)); cpu.eax = v; }
  { let v = cpu.r32(0x15037230u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15037398u32, v); }
  { let v = (cpu.r32(0x15037398u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x15037398u32, v); }
  { let v: u32 = cpu.r32(0x15037398u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3f800000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500970f; continue; }
  pc = 0x150096bc; continue;
 }
 0x150096bc => {
  { let v = (cpu.r32(0x150371fcu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x15037210u32); cpu.eax = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x1503738cu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15037200u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x15037214u32); cpu.ebx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x15037390u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15037204u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x15037218u32); cpu.ecx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x15037394u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1503738cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x15037390u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15037394u32)); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.or32(cpu.edx, 0xffffffffu32); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500970f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009714(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009714); }
 let mut pc: u32 = 0x15009714;
 loop { match pc {
 0x15009714 => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009720; continue; }
  pc = 0x1500971e; continue;
 }
 0x1500971e => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x15009720; continue;
 }
 0x15009720 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x15088f34u32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x15009742; continue; }
  pc = 0x1500973c; continue;
 }
 0x1500973c => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x15009742; continue;
 }
 0x15009742 => {
  { let v: u32 = cpu.eax; cpu.w32(0x15037228u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1503722cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15037230u32, v); }
  { let v: u32 = cpu.r32(0x15037210u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1503721cu32)); cpu.eax = v; }
  { let v = cpu.r32(0x15037228u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15037214u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037220u32)); cpu.eax = v; }
  { let v = cpu.r32(0x1503722cu32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15037218u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037224u32)); cpu.eax = v; }
  { let v = cpu.r32(0x15037230u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500979a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500979a); }
 let mut pc: u32 = 0x1500979a;
 loop { match pc {
 0x1500979a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x150373c0u32; cpu.edi = v; }
  cpu.push(0x150097a5); f_1500993d(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15037210u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x150371f4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x150373a8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1500981e; continue; }
  pc = 0x150097b9; continue;
 }
 0x150097b9 => {
  { let v: u32 = cpu.r32(0x15037210u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x150371f4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x150373b4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500981e; continue; }
  pc = 0x150097cc; continue;
 }
 0x150097cc => {
  { let v: u32 = cpu.r32(0x15037214u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x150371f4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x150373acu32)); }
  if (cpu.sf != cpu.of) { pc = 0x1500981e; continue; }
  pc = 0x150097df; continue;
 }
 0x150097df => {
  { let v: u32 = cpu.r32(0x15037214u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x150371f4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x150373b8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500981e; continue; }
  pc = 0x150097f2; continue;
 }
 0x150097f2 => {
  { let v: u32 = cpu.r32(0x15037218u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x150371f4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x150373b0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1500981e; continue; }
  pc = 0x15009805; continue;
 }
 0x15009805 => {
  { let v: u32 = cpu.r32(0x15037218u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x150371f4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x150373bcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500981e; continue; }
  pc = 0x15009818; continue;
 }
 0x15009818 => {
  cpu.push(0x1500981d); f_15009821(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500981e => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009821(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009821); }
 let mut pc: u32 = 0x15009821;
 loop { match pc {
 0x15009821 => {
  { let v: u32 = cpu.r32(0x15037228u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1503722cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15037230u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009838; continue; }
  pc = 0x15009836; continue;
 }
 0x15009836 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x15009838; continue;
 }
 0x15009838 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500983e; continue; }
  pc = 0x1500983c; continue;
 }
 0x1500983c => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x1500983e; continue;
 }
 0x1500983e => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009844; continue; }
  pc = 0x15009842; continue;
 }
 0x15009842 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x15009844; continue;
 }
 0x15009844 => {
  { let v: u32 = 0x150373c0u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x150098a3; continue; }
  pc = 0x15009852; continue;
 }
 0x15009852 => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x150098a3; continue; }
  pc = 0x15009856; continue;
 }
 0x15009856 => {
  { let v: u32 = cpu.r32(0x15037230u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  pc = 0x1500985f; continue;
 }
 0x1500985f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037210u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x15037214u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037210u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x15037214u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x15009936; continue; }
  pc = 0x15009898; continue;
 }
 0x15009898 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500985f; continue; }
  pc = 0x1500989e; continue;
 }
 0x1500989e => {
  pc = 0x15009939; continue;
 }
 0x150098a3 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x150098ef; continue; }
  pc = 0x150098a7; continue;
 }
 0x150098a7 => {
  { let v: u32 = cpu.r32(0x1503722cu32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  { let v: u32 = !cpu.edi; cpu.edi = v; }
  pc = 0x150098b2; continue;
 }
 0x150098b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037210u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x15037218u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037210u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x15037218u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x15009936; continue; }
  pc = 0x150098e7; continue;
 }
 0x150098e7 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x150098b2; continue; }
  pc = 0x150098ed; continue;
 }
 0x150098ed => {
  pc = 0x15009939; continue;
 }
 0x150098ef => {
  { let v: u32 = cpu.r32(0x15037228u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  pc = 0x150098f8; continue;
 }
 0x150098f8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037214u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x15037218u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15037214u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x15037218u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x15009936; continue; }
  pc = 0x1500992e; continue;
 }
 0x1500992e => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x150098f8; continue; }
  pc = 0x15009934; continue;
 }
 0x15009934 => {
  pc = 0x15009939; continue;
 }
 0x15009936 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x15009939 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500993d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500993d); }
 let mut pc: u32 = 0x1500993d;
 loop { match pc {
 0x1500993d => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0xeu32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150373a8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x150373acu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x150373b0u32, v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150373b4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x150373b8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x150373bcu32, v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x15009973; continue;
 }
 0x15009973 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x150373a8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500999a; continue; }
  pc = 0x15009995; continue;
 }
 0x15009995 => {
  { let v: u32 = cpu.eax; cpu.w32(0x150373a8u32, v); }
  pc = 0x1500999a; continue;
 }
 0x1500999a => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x150373acu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150099a8; continue; }
  pc = 0x150099a2; continue;
 }
 0x150099a2 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x150373acu32, v); }
  pc = 0x150099a8; continue;
 }
 0x150099a8 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x150373b0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x150099b6; continue; }
  pc = 0x150099b0; continue;
 }
 0x150099b0 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x150373b0u32, v); }
  pc = 0x150099b6; continue;
 }
 0x150099b6 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x150373b4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x150099c3; continue; }
  pc = 0x150099be; continue;
 }
 0x150099be => {
  { let v: u32 = cpu.eax; cpu.w32(0x150373b4u32, v); }
  pc = 0x150099c3; continue;
 }
 0x150099c3 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x150373b8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x150099d1; continue; }
  pc = 0x150099cb; continue;
 }
 0x150099cb => {
  { let v: u32 = cpu.ebx; cpu.w32(0x150373b8u32, v); }
  pc = 0x150099d1; continue;
 }
 0x150099d1 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x150373bcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x150099df; continue; }
  pc = 0x150099d9; continue;
 }
 0x150099d9 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x150373bcu32, v); }
  pc = 0x150099df; continue;
 }
 0x150099df => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15009973; continue; }
  pc = 0x150099ed; continue;
 }
 0x150099ed => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_150099f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x150099f8); }
 let mut pc: u32 = 0x150099f8;
 loop { match pc {
 0x150099f8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x15037238u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15037208u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x15009a42; continue; }
  pc = 0x15009a08; continue;
 }
 0x15009a08 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15009a08; continue; }
  pc = 0x15009a42; continue;
 }
 0x15009a42 => {
  { let v: u32 = cpu.r32(0x15037208u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009a49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009a49); }
 let mut pc: u32 = 0x15009a49;
 loop { match pc {
 0x15009a49 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009b20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009b20); }
 let mut pc: u32 = 0x15009b20;
 loop { match pc {
 0x15009b20 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x15041510u32; cpu.w32(0x1504150cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x150416c0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x15009b3c; continue;
 }
 0x15009b3c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009b7f; continue; }
  pc = 0x15009b4a; continue;
 }
 0x15009b4a => {
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fecu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15009ba5; continue; }
  pc = 0x15009b52; continue;
 }
 0x15009b52 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009b73; continue; }
  pc = 0x15009b5f; continue;
 }
 0x15009b5f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x15009bc9; continue; }
  pc = 0x15009b67; continue;
 }
 0x15009b67 => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009b71); f_15009be0(cpu);
  pc = 0x15009bc9; continue;
 }
 0x15009b73 => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009b7d); f_15009be0(cpu);
  pc = 0x15009bc9; continue;
 }
 0x15009b7f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009bc9; continue; }
  pc = 0x15009b87; continue;
 }
 0x15009b87 => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009b91); f_15009be0(cpu);
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x15009bc9; continue; }
  pc = 0x15009b99; continue;
 }
 0x15009b99 => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009ba3); f_15009be0(cpu);
  pc = 0x15009bc9; continue;
 }
 0x15009ba5 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15009bc9; continue; }
  pc = 0x15009bad; continue;
 }
 0x15009bad => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009bb7); f_15009be0(cpu);
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15009bc9; continue; }
  pc = 0x15009bbf; continue;
 }
 0x15009bbf => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009bc9); f_15009be0(cpu);
  pc = 0x15009bc9; continue;
 }
 0x15009bc9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15009b3c; continue; }
  pc = 0x15009bd7; continue;
 }
 0x15009bd7 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009be0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009be0); }
 let mut pc: u32 = 0x15009be0;
 loop { match pc {
 0x15009be0 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1504150cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1504150cu32).wrapping_add(0x24u32); cpu.w32(0x1504150cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010ffcu32)); }
  if cpu.zf { pc = 0x15009c3e; continue; }
  pc = 0x15009c0d; continue;
 }
 0x15009c0d => {
  { let v: u32 = cpu.r32(0x15010ff0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x15009c79; continue;
 }
 0x15009c3e => {
  { let v: u32 = cpu.r32(0x15011000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010ff4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010ff4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x15009c79; continue;
 }
 0x15009c79 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009c7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009c7d); }
 let mut pc: u32 = 0x15009c7d;
 loop { match pc {
 0x15009c7d => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x15041510u32; cpu.w32(0x1504150cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x150416c0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x15009c99; continue;
 }
 0x15009c99 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009cdc; continue; }
  pc = 0x15009ca7; continue;
 }
 0x15009ca7 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fecu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15009d02; continue; }
  pc = 0x15009caf; continue;
 }
 0x15009caf => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009cd0; continue; }
  pc = 0x15009cbc; continue;
 }
 0x15009cbc => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x15009d26; continue; }
  pc = 0x15009cc4; continue;
 }
 0x15009cc4 => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009cce); f_15009d3d(cpu);
  pc = 0x15009d26; continue;
 }
 0x15009cd0 => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009cda); f_15009d3d(cpu);
  pc = 0x15009d26; continue;
 }
 0x15009cdc => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009d26; continue; }
  pc = 0x15009ce4; continue;
 }
 0x15009ce4 => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009cee); f_15009d3d(cpu);
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x15009d26; continue; }
  pc = 0x15009cf6; continue;
 }
 0x15009cf6 => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009d00); f_15009d3d(cpu);
  pc = 0x15009d26; continue;
 }
 0x15009d02 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15009d26; continue; }
  pc = 0x15009d0a; continue;
 }
 0x15009d0a => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009d14); f_15009d3d(cpu);
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15009d26; continue; }
  pc = 0x15009d1c; continue;
 }
 0x15009d1c => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009d26); f_15009d3d(cpu);
  pc = 0x15009d26; continue;
 }
 0x15009d26 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15009c99; continue; }
  pc = 0x15009d34; continue;
 }
 0x15009d34 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009d3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009d3d); }
 let mut pc: u32 = 0x15009d3d;
 loop { match pc {
 0x15009d3d => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1504150cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1504150cu32).wrapping_add(0x24u32); cpu.w32(0x1504150cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010ffcu32)); }
  if cpu.zf { pc = 0x15009d9b; continue; }
  pc = 0x15009d6a; continue;
 }
 0x15009d6a => {
  { let v: u32 = cpu.r32(0x15010ff0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x15009dd6; continue;
 }
 0x15009d9b => {
  { let v: u32 = cpu.r32(0x15011000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010ff4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010ff4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x15009dd6; continue;
 }
 0x15009dd6 => {
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009de8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009de8); }
 let mut pc: u32 = 0x15009de8;
 loop { match pc {
 0x15009de8 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x15041510u32; cpu.w32(0x1504150cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x150416c0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x15009e04; continue;
 }
 0x15009e04 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009e47; continue; }
  pc = 0x15009e12; continue;
 }
 0x15009e12 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fecu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15009e6d; continue; }
  pc = 0x15009e1a; continue;
 }
 0x15009e1a => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009e3b; continue; }
  pc = 0x15009e27; continue;
 }
 0x15009e27 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x15009e91; continue; }
  pc = 0x15009e2f; continue;
 }
 0x15009e2f => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009e39); f_15009ea8(cpu);
  pc = 0x15009e91; continue;
 }
 0x15009e3b => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009e45); f_15009ea8(cpu);
  pc = 0x15009e91; continue;
 }
 0x15009e47 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x15009e91; continue; }
  pc = 0x15009e4f; continue;
 }
 0x15009e4f => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009e59); f_15009ea8(cpu);
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x15009e91; continue; }
  pc = 0x15009e61; continue;
 }
 0x15009e61 => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009e6b); f_15009ea8(cpu);
  pc = 0x15009e91; continue;
 }
 0x15009e6d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fecu32)); }
  if (cpu.sf != cpu.of) { pc = 0x15009e91; continue; }
  pc = 0x15009e75; continue;
 }
 0x15009e75 => {
  { let v: u32 = 0x15010fecu32; cpu.eax = v; }
  cpu.push(0x15009e7f); f_15009ea8(cpu);
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010ffcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x15009e91; continue; }
  pc = 0x15009e87; continue;
 }
 0x15009e87 => {
  { let v: u32 = 0x15010ffcu32; cpu.eax = v; }
  cpu.push(0x15009e91); f_15009ea8(cpu);
  pc = 0x15009e91; continue;
 }
 0x15009e91 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15009e04; continue; }
  pc = 0x15009e9f; continue;
 }
 0x15009e9f => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009ea8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009ea8); }
 let mut pc: u32 = 0x15009ea8;
 loop { match pc {
 0x15009ea8 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1504150cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1504150cu32).wrapping_add(0x24u32); cpu.w32(0x1504150cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010ffcu32)); }
  if cpu.zf { pc = 0x15009f06; continue; }
  pc = 0x15009ed5; continue;
 }
 0x15009ed5 => {
  { let v: u32 = cpu.r32(0x15010ff0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x15009f41; continue;
 }
 0x15009f06 => {
  { let v: u32 = cpu.r32(0x15011000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010ff4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010ff4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x15009f41; continue;
 }
 0x15009f41 => {
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_15009f61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x15009f61); }
 let mut pc: u32 = 0x15009f61;
 loop { match pc {
 0x15009f61 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x150417e4u32; cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x15041994u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x15009f7c; continue;
 }
 0x15009f7c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fe0u32)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x15009fbf; continue; }
  pc = 0x15009f89; continue;
 }
 0x15009f89 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe0u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15009fa5; continue; }
  pc = 0x15009f96; continue;
 }
 0x15009f96 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15009f7c; continue; }
  pc = 0x15009fa0; continue;
 }
 0x15009fa0 => {
  pc = 0x1500a025; continue;
 }
 0x15009fa5 => {
  if (cpu.sf != cpu.of) { pc = 0x15009fb3; continue; }
  pc = 0x15009fa7; continue;
 }
 0x15009fa7 => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x15009fb1); f_1500a0f7(cpu);
  pc = 0x1500a017; continue;
 }
 0x15009fb3 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x15009fbd); f_1500a0f7(cpu);
  pc = 0x1500a017; continue;
 }
 0x15009fbf => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe0u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x15009fe3; continue; }
  pc = 0x15009fc7; continue;
 }
 0x15009fc7 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x15009fd7; continue; }
  pc = 0x15009fcb; continue;
 }
 0x15009fcb => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x15009fd5); f_1500a0f7(cpu);
  pc = 0x1500a017; continue;
 }
 0x15009fd7 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x15009fe1); f_1500a0f7(cpu);
  pc = 0x1500a017; continue;
 }
 0x15009fe3 => {
  if (cpu.sf != cpu.of) { pc = 0x15009fff; continue; }
  pc = 0x15009fe5; continue;
 }
 0x15009fe5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500a017; continue; }
  pc = 0x15009fe9; continue;
 }
 0x15009fe9 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x15009ff3); f_1500a0f7(cpu);
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x15009ffd); f_1500a0f7(cpu);
  pc = 0x1500a017; continue;
 }
 0x15009fff => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a017; continue; }
  pc = 0x1500a003; continue;
 }
 0x1500a003 => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a00d); f_1500a0f7(cpu);
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a017); f_1500a0f7(cpu);
  pc = 0x1500a017; continue;
 }
 0x1500a017 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x15009f7c; continue; }
  pc = 0x1500a025; continue;
 }
 0x1500a025 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500a0f4; continue; }
  pc = 0x1500a030; continue;
 }
 0x1500a030 => {
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x1500a041; continue;
 }
 0x1500a041 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fe4u32)); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a085; continue; }
  pc = 0x1500a04f; continue;
 }
 0x1500a04f => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a06b; continue; }
  pc = 0x1500a05c; continue;
 }
 0x1500a05c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a041; continue; }
  pc = 0x1500a066; continue;
 }
 0x1500a066 => {
  pc = 0x1500a0eb; continue;
 }
 0x1500a06b => {
  if (cpu.sf != cpu.of) { pc = 0x1500a079; continue; }
  pc = 0x1500a06d; continue;
 }
 0x1500a06d => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a077); f_1500a15a(cpu);
  pc = 0x1500a0dd; continue;
 }
 0x1500a079 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a083); f_1500a15a(cpu);
  pc = 0x1500a0dd; continue;
 }
 0x1500a085 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a0a9; continue; }
  pc = 0x1500a08d; continue;
 }
 0x1500a08d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a09d; continue; }
  pc = 0x1500a091; continue;
 }
 0x1500a091 => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a09b); f_1500a15a(cpu);
  pc = 0x1500a0dd; continue;
 }
 0x1500a09d => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a0a7); f_1500a15a(cpu);
  pc = 0x1500a0dd; continue;
 }
 0x1500a0a9 => {
  if (cpu.sf != cpu.of) { pc = 0x1500a0c5; continue; }
  pc = 0x1500a0ab; continue;
 }
 0x1500a0ab => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500a0dd; continue; }
  pc = 0x1500a0af; continue;
 }
 0x1500a0af => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a0b9); f_1500a15a(cpu);
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a0c3); f_1500a15a(cpu);
  pc = 0x1500a0dd; continue;
 }
 0x1500a0c5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a0dd; continue; }
  pc = 0x1500a0c9; continue;
 }
 0x1500a0c9 => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a0d3); f_1500a15a(cpu);
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a0dd); f_1500a15a(cpu);
  pc = 0x1500a0dd; continue;
 }
 0x1500a0dd => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a041; continue; }
  pc = 0x1500a0eb; continue;
 }
 0x1500a0eb => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500a0f4 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a0f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a0f7); }
 let mut pc: u32 = 0x1500a0f7;
 loop { match pc {
 0x1500a0f7 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150417e0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150417e0u32).wrapping_add(0x24u32); cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a15a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a15a); }
 let mut pc: u32 = 0x1500a15a;
 loop { match pc {
 0x1500a15a => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150417e0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150417e0u32).wrapping_add(0x24u32); cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a1bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a1bf); }
 let mut pc: u32 = 0x1500a1bf;
 loop { match pc {
 0x1500a1bf => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x150417e4u32; cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x15041994u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x1500a1da; continue;
 }
 0x1500a1da => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fe0u32)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a21d; continue; }
  pc = 0x1500a1e7; continue;
 }
 0x1500a1e7 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe0u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a203; continue; }
  pc = 0x1500a1f4; continue;
 }
 0x1500a1f4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a1da; continue; }
  pc = 0x1500a1fe; continue;
 }
 0x1500a1fe => {
  pc = 0x1500a283; continue;
 }
 0x1500a203 => {
  if (cpu.sf != cpu.of) { pc = 0x1500a211; continue; }
  pc = 0x1500a205; continue;
 }
 0x1500a205 => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a20f); f_1500a355(cpu);
  pc = 0x1500a275; continue;
 }
 0x1500a211 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a21b); f_1500a355(cpu);
  pc = 0x1500a275; continue;
 }
 0x1500a21d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe0u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a241; continue; }
  pc = 0x1500a225; continue;
 }
 0x1500a225 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a235; continue; }
  pc = 0x1500a229; continue;
 }
 0x1500a229 => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a233); f_1500a355(cpu);
  pc = 0x1500a275; continue;
 }
 0x1500a235 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a23f); f_1500a355(cpu);
  pc = 0x1500a275; continue;
 }
 0x1500a241 => {
  if (cpu.sf != cpu.of) { pc = 0x1500a25d; continue; }
  pc = 0x1500a243; continue;
 }
 0x1500a243 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500a275; continue; }
  pc = 0x1500a247; continue;
 }
 0x1500a247 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a251); f_1500a355(cpu);
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a25b); f_1500a355(cpu);
  pc = 0x1500a275; continue;
 }
 0x1500a25d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a275; continue; }
  pc = 0x1500a261; continue;
 }
 0x1500a261 => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a26b); f_1500a355(cpu);
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a275); f_1500a355(cpu);
  pc = 0x1500a275; continue;
 }
 0x1500a275 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a1da; continue; }
  pc = 0x1500a283; continue;
 }
 0x1500a283 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500a352; continue; }
  pc = 0x1500a28e; continue;
 }
 0x1500a28e => {
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x1500a29f; continue;
 }
 0x1500a29f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fe4u32)); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a2e3; continue; }
  pc = 0x1500a2ad; continue;
 }
 0x1500a2ad => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a2c9; continue; }
  pc = 0x1500a2ba; continue;
 }
 0x1500a2ba => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a29f; continue; }
  pc = 0x1500a2c4; continue;
 }
 0x1500a2c4 => {
  pc = 0x1500a349; continue;
 }
 0x1500a2c9 => {
  if (cpu.sf != cpu.of) { pc = 0x1500a2d7; continue; }
  pc = 0x1500a2cb; continue;
 }
 0x1500a2cb => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a2d5); f_1500a3c6(cpu);
  pc = 0x1500a33b; continue;
 }
 0x1500a2d7 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a2e1); f_1500a3c6(cpu);
  pc = 0x1500a33b; continue;
 }
 0x1500a2e3 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a307; continue; }
  pc = 0x1500a2eb; continue;
 }
 0x1500a2eb => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a2fb; continue; }
  pc = 0x1500a2ef; continue;
 }
 0x1500a2ef => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a2f9); f_1500a3c6(cpu);
  pc = 0x1500a33b; continue;
 }
 0x1500a2fb => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a305); f_1500a3c6(cpu);
  pc = 0x1500a33b; continue;
 }
 0x1500a307 => {
  if (cpu.sf != cpu.of) { pc = 0x1500a323; continue; }
  pc = 0x1500a309; continue;
 }
 0x1500a309 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500a33b; continue; }
  pc = 0x1500a30d; continue;
 }
 0x1500a30d => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a317); f_1500a3c6(cpu);
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a321); f_1500a3c6(cpu);
  pc = 0x1500a33b; continue;
 }
 0x1500a323 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a33b; continue; }
  pc = 0x1500a327; continue;
 }
 0x1500a327 => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a331); f_1500a3c6(cpu);
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a33b); f_1500a3c6(cpu);
  pc = 0x1500a33b; continue;
 }
 0x1500a33b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a29f; continue; }
  pc = 0x1500a349; continue;
 }
 0x1500a349 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500a352 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a355(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a355); }
 let mut pc: u32 = 0x1500a355;
 loop { match pc {
 0x1500a355 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150417e0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150417e0u32).wrapping_add(0x24u32); cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a3c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a3c6); }
 let mut pc: u32 = 0x1500a3c6;
 loop { match pc {
 0x1500a3c6 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150417e0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150417e0u32).wrapping_add(0x24u32); cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a439(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a439); }
 let mut pc: u32 = 0x1500a439;
 loop { match pc {
 0x1500a439 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x150417e4u32; cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x15041994u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x1500a454; continue;
 }
 0x1500a454 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fe0u32)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a497; continue; }
  pc = 0x1500a461; continue;
 }
 0x1500a461 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe0u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a47d; continue; }
  pc = 0x1500a46e; continue;
 }
 0x1500a46e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a454; continue; }
  pc = 0x1500a478; continue;
 }
 0x1500a478 => {
  pc = 0x1500a4fd; continue;
 }
 0x1500a47d => {
  if (cpu.sf != cpu.of) { pc = 0x1500a48b; continue; }
  pc = 0x1500a47f; continue;
 }
 0x1500a47f => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a489); f_1500a5cf(cpu);
  pc = 0x1500a4ef; continue;
 }
 0x1500a48b => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a495); f_1500a5cf(cpu);
  pc = 0x1500a4ef; continue;
 }
 0x1500a497 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe0u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a4bb; continue; }
  pc = 0x1500a49f; continue;
 }
 0x1500a49f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a4af; continue; }
  pc = 0x1500a4a3; continue;
 }
 0x1500a4a3 => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a4ad); f_1500a5cf(cpu);
  pc = 0x1500a4ef; continue;
 }
 0x1500a4af => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a4b9); f_1500a5cf(cpu);
  pc = 0x1500a4ef; continue;
 }
 0x1500a4bb => {
  if (cpu.sf != cpu.of) { pc = 0x1500a4d7; continue; }
  pc = 0x1500a4bd; continue;
 }
 0x1500a4bd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500a4ef; continue; }
  pc = 0x1500a4c1; continue;
 }
 0x1500a4c1 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a4cb); f_1500a5cf(cpu);
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a4d5); f_1500a5cf(cpu);
  pc = 0x1500a4ef; continue;
 }
 0x1500a4d7 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a4ef; continue; }
  pc = 0x1500a4db; continue;
 }
 0x1500a4db => {
  { let v: u32 = 0x15010fe0u32; cpu.eax = v; }
  cpu.push(0x1500a4e5); f_1500a5cf(cpu);
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a4ef); f_1500a5cf(cpu);
  pc = 0x1500a4ef; continue;
 }
 0x1500a4ef => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a454; continue; }
  pc = 0x1500a4fd; continue;
 }
 0x1500a4fd => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500a5cc; continue; }
  pc = 0x1500a508; continue;
 }
 0x1500a508 => {
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x1500a519; continue;
 }
 0x1500a519 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fe4u32)); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a55d; continue; }
  pc = 0x1500a527; continue;
 }
 0x1500a527 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a543; continue; }
  pc = 0x1500a534; continue;
 }
 0x1500a534 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a519; continue; }
  pc = 0x1500a53e; continue;
 }
 0x1500a53e => {
  pc = 0x1500a5c3; continue;
 }
 0x1500a543 => {
  if (cpu.sf != cpu.of) { pc = 0x1500a551; continue; }
  pc = 0x1500a545; continue;
 }
 0x1500a545 => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a54f); f_1500a64e(cpu);
  pc = 0x1500a5b5; continue;
 }
 0x1500a551 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a55b); f_1500a64e(cpu);
  pc = 0x1500a5b5; continue;
 }
 0x1500a55d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500a581; continue; }
  pc = 0x1500a565; continue;
 }
 0x1500a565 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a575; continue; }
  pc = 0x1500a569; continue;
 }
 0x1500a569 => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a573); f_1500a64e(cpu);
  pc = 0x1500a5b5; continue;
 }
 0x1500a575 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a57f); f_1500a64e(cpu);
  pc = 0x1500a5b5; continue;
 }
 0x1500a581 => {
  if (cpu.sf != cpu.of) { pc = 0x1500a59d; continue; }
  pc = 0x1500a583; continue;
 }
 0x1500a583 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500a5b5; continue; }
  pc = 0x1500a587; continue;
 }
 0x1500a587 => {
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a591); f_1500a64e(cpu);
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a59b); f_1500a64e(cpu);
  pc = 0x1500a5b5; continue;
 }
 0x1500a59d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500a5b5; continue; }
  pc = 0x1500a5a1; continue;
 }
 0x1500a5a1 => {
  { let v: u32 = 0x15010fe4u32; cpu.eax = v; }
  cpu.push(0x1500a5ab); f_1500a64e(cpu);
  { let v: u32 = 0x15011044u32; cpu.eax = v; }
  cpu.push(0x1500a5b5); f_1500a64e(cpu);
  pc = 0x1500a5b5; continue;
 }
 0x1500a5b5 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a519; continue; }
  pc = 0x1500a5c3; continue;
 }
 0x1500a5c3 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500a5cc => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a5cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a5cf); }
 let mut pc: u32 = 0x1500a5cf;
 loop { match pc {
 0x1500a5cf => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150417e0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150417e0u32).wrapping_add(0x24u32); cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a64e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a64e); }
 let mut pc: u32 = 0x1500a64e;
 loop { match pc {
 0x1500a64e => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150417e0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150417e0u32).wrapping_add(0x24u32); cpu.w32(0x150417e0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a6cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a6cf); }
 let mut pc: u32 = 0x1500a6cf;
 loop { match pc {
 0x1500a6cf => {
  { let v: u32 = cpu.r32(0x15010fe8u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500a985; continue; }
  pc = 0x1500a6dd; continue;
 }
 0x1500a6dd => {
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x15041be8u32, v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010fecu32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011048u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x10u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011048u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x15010fe0u32); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x15041bd4u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x15041bd4u32), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500a97f; continue; }
  pc = 0x1500a725; continue;
 }
 0x1500a725 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x15010fd4u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x15010fe4u32); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x15041bd8u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x15041bd8u32), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500a97f; continue; }
  pc = 0x1500a749; continue;
 }
 0x1500a749 => {
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041c1cu32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x15041c1cu32), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x1500a97f; continue; }
  pc = 0x1500a768; continue;
 }
 0x1500a768 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x15010fd4u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041c20u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x15041c20u32), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x1500a97f; continue; }
  pc = 0x1500a788; continue;
 }
 0x1500a788 => {
  { let v: u32 = cpu.r32(0x15041bd4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c40u32, v); }
  { let v: u32 = cpu.r32(0x15041c1cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041bf8u32, v); }
  { let v: u32 = cpu.r32(0x15041bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041bfcu32, v); }
  { let v: u32 = cpu.r32(0x15041c20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c44u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3800000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041becu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c58u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3800000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c10u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c34u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3800000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041bf0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c14u32, v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3800000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c5cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c38u32, v); }
  { cpu.sub32(cpu.r32(0x15041bd4u32), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500a84b; continue; }
  pc = 0x1500a801; continue;
 }
 0x1500a801 => {
  { cpu.fpush(0.0); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bd4u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x15041c1cu32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bd4u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = 0x0u32; cpu.w32(0x15041bd4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15041c40u32, v); }
  { let v = cpu.rf32(0x15041c34u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041becu32); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x15041becu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041becu32, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041c58u32, v); cpu.fpop(); }
  pc = 0x1500a84b; continue;
 }
 0x1500a84b => {
  { cpu.sub32(cpu.r32(0x15041bd8u32), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500a89e; continue; }
  pc = 0x1500a854; continue;
 }
 0x1500a854 => {
  { cpu.fpush(0.0); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bd8u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x15041c20u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bd8u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = 0x0u32; cpu.w32(0x15041bd8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x15041bfcu32, v); }
  { let v = cpu.rf32(0x15041c38u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bf0u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x15041bf0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041bf0u32, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041c14u32, v); cpu.fpop(); }
  pc = 0x1500a89e; continue;
 }
 0x1500a89e => {
  { let v: u32 = cpu.r32(0x15010fe0u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x15041bf8u32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500a8ef; continue; }
  pc = 0x1500a8ab; continue;
 }
 0x1500a8ab => {
  { let v = cpu.rf32(0x15010fe0u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bd4u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x15041c1cu32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bd4u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041bf8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c1cu32, v); }
  { let v = cpu.rf32(0x15041c34u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041becu32); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x15041becu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041c10u32, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041c34u32, v); cpu.fpop(); }
  pc = 0x1500a8ef; continue;
 }
 0x1500a8ef => {
  { let v: u32 = cpu.r32(0x15010fe4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x15041c20u32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500a940; continue; }
  pc = 0x1500a8fc; continue;
 }
 0x1500a8fc => {
  { let v = cpu.rf32(0x15010fe4u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bd8u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x15041c20u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bd8u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c20u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c44u32, v); }
  { let v = cpu.rf32(0x15041c38u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x15041bf0u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x15041bf0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041c5cu32, v); }
  { let v = cpu.st(0); cpu.wf32(0x15041c38u32, v); cpu.fpop(); }
  pc = 0x1500a940; continue;
 }
 0x1500a940 => {
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x1501104cu32); cpu.set_st(0, v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x15041be8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x15041be8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c0cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c30u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15041c54u32, v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = 0x15011050u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500a97f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x1500a985; continue;
 }
 0x1500a985 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a9a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a9a7); }
 let mut pc: u32 = 0x1500a9a7;
 loop { match pc {
 0x1500a9a7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a9a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a9a8); }
 let mut pc: u32 = 0x1500a9a8;
 loop { match pc {
 0x1500a9a8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a9a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a9a9); }
 let mut pc: u32 = 0x1500a9a9;
 loop { match pc {
 0x1500a9a9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a9aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a9aa); }
 let mut pc: u32 = 0x1500a9aa;
 loop { match pc {
 0x1500a9aa => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500a9ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500a9ac); }
 let mut pc: u32 = 0x1500a9ac;
 loop { match pc {
 0x1500a9ac => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500a9c4; continue;
 }
 0x1500a9c4 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500a9c4; continue; }
  pc = 0x1500a9f5; continue;
 }
 0x1500a9f5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500aa4a; continue; }
  pc = 0x1500aa0c; continue;
 }
 0x1500aa0c => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500aa10; continue;
 }
 0x1500aa10 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500aa10; continue; }
  pc = 0x1500aa1f; continue;
 }
 0x1500aa1f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15011064u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500aa4a; continue;
 }
 0x1500aa4a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500aa4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500aa4b); }
 let mut pc: u32 = 0x1500aa4b;
 loop { match pc {
 0x1500aa4b => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500aa63; continue;
 }
 0x1500aa63 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500aa63; continue; }
  pc = 0x1500aa94; continue;
 }
 0x1500aa94 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500aaa2); f_15009f61(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500aaf0; continue; }
  pc = 0x1500aaa4; continue;
 }
 0x1500aaa4 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500aaf0; continue; }
  pc = 0x1500aab2; continue;
 }
 0x1500aab2 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500aab6; continue;
 }
 0x1500aab6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500aab6; continue; }
  pc = 0x1500aac5; continue;
 }
 0x1500aac5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15011064u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500aaf0; continue;
 }
 0x1500aaf0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500aaf1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500aaf1); }
 let mut pc: u32 = 0x1500aaf1;
 loop { match pc {
 0x1500aaf1 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500ab09; continue;
 }
 0x1500ab09 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ab09; continue; }
  pc = 0x1500ab31; continue;
 }
 0x1500ab31 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500ab3f); f_15009b20(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ab8d; continue; }
  pc = 0x1500ab41; continue;
 }
 0x1500ab41 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500ab8d; continue; }
  pc = 0x1500ab4f; continue;
 }
 0x1500ab4f => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500ab53; continue;
 }
 0x1500ab53 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ab53; continue; }
  pc = 0x1500ab62; continue;
 }
 0x1500ab62 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15011064u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500ab8d; continue;
 }
 0x1500ab8d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ab8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ab8e); }
 let mut pc: u32 = 0x1500ab8e;
 loop { match pc {
 0x1500ab8e => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500aba6; continue;
 }
 0x1500aba6 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500aba6; continue; }
  pc = 0x1500abce; continue;
 }
 0x1500abce => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500abdc); f_15009b20(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ac31; continue; }
  pc = 0x1500abde; continue;
 }
 0x1500abde => {
  cpu.push(0x1500abe3); f_15009f61(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ac31; continue; }
  pc = 0x1500abe5; continue;
 }
 0x1500abe5 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500ac31; continue; }
  pc = 0x1500abf3; continue;
 }
 0x1500abf3 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500abf7; continue;
 }
 0x1500abf7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500abf7; continue; }
  pc = 0x1500ac06; continue;
 }
 0x1500ac06 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15011064u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500ac31; continue;
 }
 0x1500ac31 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ac32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ac32); }
 let mut pc: u32 = 0x1500ac32;
 loop { match pc {
 0x1500ac32 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500ac4a; continue;
 }
 0x1500ac4a => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ac4a; continue; }
  pc = 0x1500ac7b; continue;
 }
 0x1500ac7b => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500ac82; continue;
 }
 0x1500ac82 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ac82; continue; }
  pc = 0x1500ac90; continue;
 }
 0x1500ac90 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x150374f8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041504u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500acf5; continue; }
  pc = 0x1500acb8; continue;
 }
 0x1500acb8 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500acbc; continue;
 }
 0x1500acbc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500acbc; continue; }
  pc = 0x1500accb; continue;
 }
 0x1500accb => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500acf5; continue;
 }
 0x1500acf5 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500acf6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500acf6); }
 let mut pc: u32 = 0x1500acf6;
 loop { match pc {
 0x1500acf6 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500ad0e; continue;
 }
 0x1500ad0e => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ad0e; continue; }
  pc = 0x1500ad3f; continue;
 }
 0x1500ad3f => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500ad46; continue;
 }
 0x1500ad46 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ad46; continue; }
  pc = 0x1500ad54; continue;
 }
 0x1500ad54 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x150374f8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041504u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500ad73); f_1500a1bf(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500adc0; continue; }
  pc = 0x1500ad75; continue;
 }
 0x1500ad75 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500adc0; continue; }
  pc = 0x1500ad83; continue;
 }
 0x1500ad83 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500ad87; continue;
 }
 0x1500ad87 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ad87; continue; }
  pc = 0x1500ad96; continue;
 }
 0x1500ad96 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500adc0; continue;
 }
 0x1500adc0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500adc1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500adc1); }
 let mut pc: u32 = 0x1500adc1;
 loop { match pc {
 0x1500adc1 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500add9; continue;
 }
 0x1500add9 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500add9; continue; }
  pc = 0x1500ae01; continue;
 }
 0x1500ae01 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500ae08; continue;
 }
 0x1500ae08 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ae08; continue; }
  pc = 0x1500ae16; continue;
 }
 0x1500ae16 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x150374f8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041504u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500ae35); f_15009c7d(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ae82; continue; }
  pc = 0x1500ae37; continue;
 }
 0x1500ae37 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500ae82; continue; }
  pc = 0x1500ae45; continue;
 }
 0x1500ae45 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500ae49; continue;
 }
 0x1500ae49 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ae49; continue; }
  pc = 0x1500ae58; continue;
 }
 0x1500ae58 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500ae82; continue;
 }
 0x1500ae82 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ae83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ae83); }
 let mut pc: u32 = 0x1500ae83;
 loop { match pc {
 0x1500ae83 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500ae9b; continue;
 }
 0x1500ae9b => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ae9b; continue; }
  pc = 0x1500aec3; continue;
 }
 0x1500aec3 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500aeca; continue;
 }
 0x1500aeca => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500aeca; continue; }
  pc = 0x1500aed8; continue;
 }
 0x1500aed8 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x150374f8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15041504u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500aef7); f_15009c7d(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500af4b; continue; }
  pc = 0x1500aef9; continue;
 }
 0x1500aef9 => {
  cpu.push(0x1500aefe); f_1500a1bf(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500af4b; continue; }
  pc = 0x1500af00; continue;
 }
 0x1500af00 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500af4b; continue; }
  pc = 0x1500af0e; continue;
 }
 0x1500af0e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500af12; continue;
 }
 0x1500af12 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500af12; continue; }
  pc = 0x1500af21; continue;
 }
 0x1500af21 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500af4b; continue;
 }
 0x1500af4b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500af4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500af4c); }
 let mut pc: u32 = 0x1500af4c;
 loop { match pc {
 0x1500af4c => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500af64; continue;
 }
 0x1500af64 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500af64; continue; }
  pc = 0x1500af95; continue;
 }
 0x1500af95 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500af9e; continue;
 }
 0x1500af9e => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500af9e; continue; }
  pc = 0x1500afb2; continue;
 }
 0x1500afb2 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x1500c7ab; continue;
 }
 0x1500c7ab => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c7b2; continue;
 }
 0x1500c7b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010f2cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c7b2; continue; }
  pc = 0x1500c7f6; continue;
 }
 0x1500c7f6 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15011068u32), cpu.ebx); }
  if cpu.zf { pc = 0x1500c821; continue; }
  pc = 0x1500c804; continue;
 }
 0x1500c804 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15011068u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  cpu.push(0x1500c815); f_1500cb4e(cpu);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  let t = cpu.r32(0x15010f98u32); cpu.push(0x1500c820); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1500c821; continue;
 }
 0x1500c821 => {
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c82d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500afce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500afce); }
 let mut pc: u32 = 0x1500afce;
 loop { match pc {
 0x1500afce => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500afe6; continue;
 }
 0x1500afe6 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500afe6; continue; }
  pc = 0x1500b017; continue;
 }
 0x1500b017 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b020; continue;
 }
 0x1500b020 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b020; continue; }
  pc = 0x1500b034; continue;
 }
 0x1500b034 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b04f); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b056; continue; }
  pc = 0x1500b051; continue;
 }
 0x1500b051 => {
  pc = 0x1500c7ab; continue;
 }
 0x1500b056 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c7ab => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c7b2; continue;
 }
 0x1500c7b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010f2cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c7b2; continue; }
  pc = 0x1500c7f6; continue;
 }
 0x1500c7f6 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15011068u32), cpu.ebx); }
  if cpu.zf { pc = 0x1500c821; continue; }
  pc = 0x1500c804; continue;
 }
 0x1500c804 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15011068u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  cpu.push(0x1500c815); f_1500cb4e(cpu);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  let t = cpu.r32(0x15010f98u32); cpu.push(0x1500c820); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1500c821; continue;
 }
 0x1500c821 => {
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c82d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b057(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b057); }
 let mut pc: u32 = 0x1500b057;
 loop { match pc {
 0x1500b057 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b06f; continue;
 }
 0x1500b06f => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b06f; continue; }
  pc = 0x1500b097; continue;
 }
 0x1500b097 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b0a0; continue;
 }
 0x1500b0a0 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b0a0; continue; }
  pc = 0x1500b0b4; continue;
 }
 0x1500b0b4 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b0cf); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b0d6; continue; }
  pc = 0x1500b0d1; continue;
 }
 0x1500b0d1 => {
  pc = 0x1500c7ab; continue;
 }
 0x1500b0d6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c7ab => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c7b2; continue;
 }
 0x1500c7b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010f2cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c7b2; continue; }
  pc = 0x1500c7f6; continue;
 }
 0x1500c7f6 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15011068u32), cpu.ebx); }
  if cpu.zf { pc = 0x1500c821; continue; }
  pc = 0x1500c804; continue;
 }
 0x1500c804 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15011068u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  cpu.push(0x1500c815); f_1500cb4e(cpu);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  let t = cpu.r32(0x15010f98u32); cpu.push(0x1500c820); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1500c821; continue;
 }
 0x1500c821 => {
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c82d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b0d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b0d7); }
 let mut pc: u32 = 0x1500b0d7;
 loop { match pc {
 0x1500b0d7 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b0ef; continue;
 }
 0x1500b0ef => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b0ef; continue; }
  pc = 0x1500b117; continue;
 }
 0x1500b117 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b120; continue;
 }
 0x1500b120 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b120; continue; }
  pc = 0x1500b134; continue;
 }
 0x1500b134 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b14f); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b15d; continue; }
  pc = 0x1500b151; continue;
 }
 0x1500b151 => {
  cpu.push(0x1500b156); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b15d; continue; }
  pc = 0x1500b158; continue;
 }
 0x1500b158 => {
  pc = 0x1500c7ab; continue;
 }
 0x1500b15d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c7ab => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c7b2; continue;
 }
 0x1500c7b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010f2cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c7b2; continue; }
  pc = 0x1500c7f6; continue;
 }
 0x1500c7f6 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15011068u32), cpu.ebx); }
  if cpu.zf { pc = 0x1500c821; continue; }
  pc = 0x1500c804; continue;
 }
 0x1500c804 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15011068u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  cpu.push(0x1500c815); f_1500cb4e(cpu);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  let t = cpu.r32(0x15010f98u32); cpu.push(0x1500c820); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1500c821; continue;
 }
 0x1500c821 => {
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c82d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b15e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b15e); }
 let mut pc: u32 = 0x1500b15e;
 loop { match pc {
 0x1500b15e => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b176; continue;
 }
 0x1500b176 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b176; continue; }
  pc = 0x1500b19e; continue;
 }
 0x1500b19e => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x15080db8u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b1a9; continue;
 }
 0x1500b1a9 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1508ae7cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b1a9; continue; }
  pc = 0x1500b1c4; continue;
 }
 0x1500b1c4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b1dd); f_1500c5c8(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b1de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b1de); }
 let mut pc: u32 = 0x1500b1de;
 loop { match pc {
 0x1500b1de => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b1f6; continue;
 }
 0x1500b1f6 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b1f6; continue; }
  pc = 0x1500b21e; continue;
 }
 0x1500b21e => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x15080db8u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b229; continue;
 }
 0x1500b229 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1508ae7cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b229; continue; }
  pc = 0x1500b244; continue;
 }
 0x1500b244 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b25d); f_1500c5c8(cpu);
  cpu.push(0x1500b262); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b264; continue; }
  pc = 0x1500b264; continue;
 }
 0x1500b264 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b265(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b265); }
 let mut pc: u32 = 0x1500b265;
 loop { match pc {
 0x1500b265 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b27d; continue;
 }
 0x1500b27d => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b27d; continue; }
  pc = 0x1500b2a5; continue;
 }
 0x1500b2a5 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x15080db8u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b2b0; continue;
 }
 0x1500b2b0 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1508ae7cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b2b0; continue; }
  pc = 0x1500b2cb; continue;
 }
 0x1500b2cb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b2e4); f_1500c5c8(cpu);
  cpu.push(0x1500b2e9); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b2eb; continue; }
  pc = 0x1500b2eb; continue;
 }
 0x1500b2eb => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b2ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b2ec); }
 let mut pc: u32 = 0x1500b2ec;
 loop { match pc {
 0x1500b2ec => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b304; continue;
 }
 0x1500b304 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b304; continue; }
  pc = 0x1500b32c; continue;
 }
 0x1500b32c => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x15080db8u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b337; continue;
 }
 0x1500b337 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1508ae7cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b337; continue; }
  pc = 0x1500b352; continue;
 }
 0x1500b352 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b36b); f_1500c5c8(cpu);
  cpu.push(0x1500b370); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b379; continue; }
  pc = 0x1500b372; continue;
 }
 0x1500b372 => {
  cpu.push(0x1500b377); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b379; continue; }
  pc = 0x1500b379; continue;
 }
 0x1500b379 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b37a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b37a); }
 let mut pc: u32 = 0x1500b37a;
 loop { match pc {
 0x1500b37a => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b392; continue;
 }
 0x1500b392 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b392; continue; }
  pc = 0x1500b3ba; continue;
 }
 0x1500b3ba => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b3c3; continue;
 }
 0x1500b3c3 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b3c3; continue; }
  pc = 0x1500b3d7; continue;
 }
 0x1500b3d7 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15011034u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500b3f7; continue;
 }
 0x1500b3f7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x1500b409; continue; }
  pc = 0x1500b401; continue;
 }
 0x1500b401 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500b3f7; continue; }
  pc = 0x1500b404; continue;
 }
 0x1500b404 => {
  pc = 0x1500c647; continue;
 }
 0x1500b409 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500b456; continue; }
  pc = 0x1500b417; continue;
 }
 0x1500b417 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500b41b; continue;
 }
 0x1500b41b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b41b; continue; }
  pc = 0x1500b42a; continue;
 }
 0x1500b42a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500b456; continue;
 }
 0x1500b456 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c647 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b457(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b457); }
 let mut pc: u32 = 0x1500b457;
 loop { match pc {
 0x1500b457 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b46f; continue;
 }
 0x1500b46f => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b46f; continue; }
  pc = 0x1500b497; continue;
 }
 0x1500b497 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b4a0; continue;
 }
 0x1500b4a0 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b4a0; continue; }
  pc = 0x1500b4b4; continue;
 }
 0x1500b4b4 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b4cf); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b53a; continue; }
  pc = 0x1500b4d1; continue;
 }
 0x1500b4d1 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15011034u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500b4db; continue;
 }
 0x1500b4db => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x1500b4ed; continue; }
  pc = 0x1500b4e5; continue;
 }
 0x1500b4e5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500b4db; continue; }
  pc = 0x1500b4e8; continue;
 }
 0x1500b4e8 => {
  pc = 0x1500c647; continue;
 }
 0x1500b4ed => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500b53a; continue; }
  pc = 0x1500b4fb; continue;
 }
 0x1500b4fb => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500b4ff; continue;
 }
 0x1500b4ff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b4ff; continue; }
  pc = 0x1500b50e; continue;
 }
 0x1500b50e => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500b53a; continue;
 }
 0x1500b53a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c647 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b53b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b53b); }
 let mut pc: u32 = 0x1500b53b;
 loop { match pc {
 0x1500b53b => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b553; continue;
 }
 0x1500b553 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b553; continue; }
  pc = 0x1500b57b; continue;
 }
 0x1500b57b => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b584; continue;
 }
 0x1500b584 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b584; continue; }
  pc = 0x1500b598; continue;
 }
 0x1500b598 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b5b3); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b61e; continue; }
  pc = 0x1500b5b5; continue;
 }
 0x1500b5b5 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15011034u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500b5bf; continue;
 }
 0x1500b5bf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x1500b5d1; continue; }
  pc = 0x1500b5c9; continue;
 }
 0x1500b5c9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500b5bf; continue; }
  pc = 0x1500b5cc; continue;
 }
 0x1500b5cc => {
  pc = 0x1500c647; continue;
 }
 0x1500b5d1 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500b61e; continue; }
  pc = 0x1500b5df; continue;
 }
 0x1500b5df => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500b5e3; continue;
 }
 0x1500b5e3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b5e3; continue; }
  pc = 0x1500b5f2; continue;
 }
 0x1500b5f2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500b61e; continue;
 }
 0x1500b61e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c647 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b61f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b61f); }
 let mut pc: u32 = 0x1500b61f;
 loop { match pc {
 0x1500b61f => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b637; continue;
 }
 0x1500b637 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b637; continue; }
  pc = 0x1500b65f; continue;
 }
 0x1500b65f => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b668; continue;
 }
 0x1500b668 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b668; continue; }
  pc = 0x1500b67c; continue;
 }
 0x1500b67c => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b697); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b709; continue; }
  pc = 0x1500b699; continue;
 }
 0x1500b699 => {
  cpu.push(0x1500b69e); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b709; continue; }
  pc = 0x1500b6a0; continue;
 }
 0x1500b6a0 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15011034u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500b6aa; continue;
 }
 0x1500b6aa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x1500b6bc; continue; }
  pc = 0x1500b6b4; continue;
 }
 0x1500b6b4 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500b6aa; continue; }
  pc = 0x1500b6b7; continue;
 }
 0x1500b6b7 => {
  pc = 0x1500c647; continue;
 }
 0x1500b6bc => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500b709; continue; }
  pc = 0x1500b6ca; continue;
 }
 0x1500b6ca => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500b6ce; continue;
 }
 0x1500b6ce => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b6ce; continue; }
  pc = 0x1500b6dd; continue;
 }
 0x1500b6dd => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500b709; continue;
 }
 0x1500b709 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c647 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b70a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b70a); }
 let mut pc: u32 = 0x1500b70a;
 loop { match pc {
 0x1500b70a => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b722; continue;
 }
 0x1500b722 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b722; continue; }
  pc = 0x1500b753; continue;
 }
 0x1500b753 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b75c; continue;
 }
 0x1500b75c => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b75c; continue; }
  pc = 0x1500b770; continue;
 }
 0x1500b770 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500b7d3; continue; }
  pc = 0x1500b794; continue;
 }
 0x1500b794 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500b798; continue;
 }
 0x1500b798 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b798; continue; }
  pc = 0x1500b7a7; continue;
 }
 0x1500b7a7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x2u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500b7d3; continue;
 }
 0x1500b7d3 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b7d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b7d4); }
 let mut pc: u32 = 0x1500b7d4;
 loop { match pc {
 0x1500b7d4 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b7ec; continue;
 }
 0x1500b7ec => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b7ec; continue; }
  pc = 0x1500b81d; continue;
 }
 0x1500b81d => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b826; continue;
 }
 0x1500b826 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b826; continue; }
  pc = 0x1500b83a; continue;
 }
 0x1500b83a => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b855); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b8a4; continue; }
  pc = 0x1500b857; continue;
 }
 0x1500b857 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500b8a4; continue; }
  pc = 0x1500b865; continue;
 }
 0x1500b865 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500b869; continue;
 }
 0x1500b869 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b869; continue; }
  pc = 0x1500b878; continue;
 }
 0x1500b878 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x2u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500b8a4; continue;
 }
 0x1500b8a4 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b8a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b8a5); }
 let mut pc: u32 = 0x1500b8a5;
 loop { match pc {
 0x1500b8a5 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b8bd; continue;
 }
 0x1500b8bd => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b8bd; continue; }
  pc = 0x1500b8e5; continue;
 }
 0x1500b8e5 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b8ee; continue;
 }
 0x1500b8ee => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b8ee; continue; }
  pc = 0x1500b902; continue;
 }
 0x1500b902 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b91d); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500b96c; continue; }
  pc = 0x1500b91f; continue;
 }
 0x1500b91f => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500b96c; continue; }
  pc = 0x1500b92d; continue;
 }
 0x1500b92d => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500b931; continue;
 }
 0x1500b931 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b931; continue; }
  pc = 0x1500b940; continue;
 }
 0x1500b940 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x2u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500b96c; continue;
 }
 0x1500b96c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500b96d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500b96d); }
 let mut pc: u32 = 0x1500b96d;
 loop { match pc {
 0x1500b96d => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500b985; continue;
 }
 0x1500b985 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b985; continue; }
  pc = 0x1500b9ad; continue;
 }
 0x1500b9ad => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500b9b6; continue;
 }
 0x1500b9b6 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500b9b6; continue; }
  pc = 0x1500b9ca; continue;
 }
 0x1500b9ca => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500b9e5); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ba3b; continue; }
  pc = 0x1500b9e7; continue;
 }
 0x1500b9e7 => {
  cpu.push(0x1500b9ec); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ba3b; continue; }
  pc = 0x1500b9ee; continue;
 }
 0x1500b9ee => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500ba3b; continue; }
  pc = 0x1500b9fc; continue;
 }
 0x1500b9fc => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500ba00; continue;
 }
 0x1500ba00 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ba00; continue; }
  pc = 0x1500ba0f; continue;
 }
 0x1500ba0f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x2u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500ba3b; continue;
 }
 0x1500ba3b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ba3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ba3c); }
 let mut pc: u32 = 0x1500ba3c;
 loop { match pc {
 0x1500ba3c => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500ba54; continue;
 }
 0x1500ba54 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ba54; continue; }
  pc = 0x1500ba85; continue;
 }
 0x1500ba85 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500ba8e; continue;
 }
 0x1500ba8e => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ba8e; continue; }
  pc = 0x1500baa2; continue;
 }
 0x1500baa2 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500bb05; continue; }
  pc = 0x1500bac6; continue;
 }
 0x1500bac6 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500baca; continue;
 }
 0x1500baca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500baca; continue; }
  pc = 0x1500bad9; continue;
 }
 0x1500bad9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500bb05; continue;
 }
 0x1500bb05 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500bb06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500bb06); }
 let mut pc: u32 = 0x1500bb06;
 loop { match pc {
 0x1500bb06 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500bb1e; continue;
 }
 0x1500bb1e => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bb1e; continue; }
  pc = 0x1500bb4f; continue;
 }
 0x1500bb4f => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500bb58; continue;
 }
 0x1500bb58 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bb58; continue; }
  pc = 0x1500bb6c; continue;
 }
 0x1500bb6c => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500bb87); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500bbd6; continue; }
  pc = 0x1500bb89; continue;
 }
 0x1500bb89 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500bbd6; continue; }
  pc = 0x1500bb97; continue;
 }
 0x1500bb97 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500bb9b; continue;
 }
 0x1500bb9b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bb9b; continue; }
  pc = 0x1500bbaa; continue;
 }
 0x1500bbaa => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500bbd6; continue;
 }
 0x1500bbd6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500bbd7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500bbd7); }
 let mut pc: u32 = 0x1500bbd7;
 loop { match pc {
 0x1500bbd7 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500bbef; continue;
 }
 0x1500bbef => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bbef; continue; }
  pc = 0x1500bc17; continue;
 }
 0x1500bc17 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500bc20; continue;
 }
 0x1500bc20 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bc20; continue; }
  pc = 0x1500bc34; continue;
 }
 0x1500bc34 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500bc4f); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500bc9e; continue; }
  pc = 0x1500bc51; continue;
 }
 0x1500bc51 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500bc9e; continue; }
  pc = 0x1500bc5f; continue;
 }
 0x1500bc5f => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500bc63; continue;
 }
 0x1500bc63 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bc63; continue; }
  pc = 0x1500bc72; continue;
 }
 0x1500bc72 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500bc9e; continue;
 }
 0x1500bc9e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500bc9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500bc9f); }
 let mut pc: u32 = 0x1500bc9f;
 loop { match pc {
 0x1500bc9f => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500bcb7; continue;
 }
 0x1500bcb7 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bcb7; continue; }
  pc = 0x1500bcdf; continue;
 }
 0x1500bcdf => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500bce8; continue;
 }
 0x1500bce8 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bce8; continue; }
  pc = 0x1500bcfc; continue;
 }
 0x1500bcfc => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500bd17); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500bd6d; continue; }
  pc = 0x1500bd19; continue;
 }
 0x1500bd19 => {
  cpu.push(0x1500bd1e); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500bd6d; continue; }
  pc = 0x1500bd20; continue;
 }
 0x1500bd20 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500bd6d; continue; }
  pc = 0x1500bd2e; continue;
 }
 0x1500bd2e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500bd32; continue;
 }
 0x1500bd32 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500bd32; continue; }
  pc = 0x1500bd41; continue;
 }
 0x1500bd41 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500bd6d; continue;
 }
 0x1500bd6d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500bd6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500bd6e); }
 let mut pc: u32 = 0x1500bd6e;
 loop { match pc {
 0x1500bd6e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500be5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500be5a); }
 let mut pc: u32 = 0x1500be5a;
 loop { match pc {
 0x1500be5a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500bf4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500bf4d); }
 let mut pc: u32 = 0x1500bf4d;
 loop { match pc {
 0x1500bf4d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c037(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c037); }
 let mut pc: u32 = 0x1500c037;
 loop { match pc {
 0x1500c037 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c128(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c128); }
 let mut pc: u32 = 0x1500c128;
 loop { match pc {
 0x1500c128 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500c140; continue;
 }
 0x1500c140 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c140; continue; }
  pc = 0x1500c168; continue;
 }
 0x1500c168 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500c171; continue;
 }
 0x1500c171 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c171; continue; }
  pc = 0x1500c185; continue;
 }
 0x1500c185 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x1500c7ab; continue;
 }
 0x1500c7ab => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c7b2; continue;
 }
 0x1500c7b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010f2cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c7b2; continue; }
  pc = 0x1500c7f6; continue;
 }
 0x1500c7f6 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15011068u32), cpu.ebx); }
  if cpu.zf { pc = 0x1500c821; continue; }
  pc = 0x1500c804; continue;
 }
 0x1500c804 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15011068u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  cpu.push(0x1500c815); f_1500cb4e(cpu);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  let t = cpu.r32(0x15010f98u32); cpu.push(0x1500c820); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1500c821; continue;
 }
 0x1500c821 => {
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c82d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c1a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c1a1); }
 let mut pc: u32 = 0x1500c1a1;
 loop { match pc {
 0x1500c1a1 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500c1b9; continue;
 }
 0x1500c1b9 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c1b9; continue; }
  pc = 0x1500c1e1; continue;
 }
 0x1500c1e1 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500c1ea; continue;
 }
 0x1500c1ea => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c1ea; continue; }
  pc = 0x1500c1fe; continue;
 }
 0x1500c1fe => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500c219); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c220; continue; }
  pc = 0x1500c21b; continue;
 }
 0x1500c21b => {
  pc = 0x1500c7ab; continue;
 }
 0x1500c220 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c7ab => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c7b2; continue;
 }
 0x1500c7b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010f2cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c7b2; continue; }
  pc = 0x1500c7f6; continue;
 }
 0x1500c7f6 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15011068u32), cpu.ebx); }
  if cpu.zf { pc = 0x1500c821; continue; }
  pc = 0x1500c804; continue;
 }
 0x1500c804 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15011068u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  cpu.push(0x1500c815); f_1500cb4e(cpu);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  let t = cpu.r32(0x15010f98u32); cpu.push(0x1500c820); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1500c821; continue;
 }
 0x1500c821 => {
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c82d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c221(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c221); }
 let mut pc: u32 = 0x1500c221;
 loop { match pc {
 0x1500c221 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500c239; continue;
 }
 0x1500c239 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c239; continue; }
  pc = 0x1500c261; continue;
 }
 0x1500c261 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500c26a; continue;
 }
 0x1500c26a => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c26a; continue; }
  pc = 0x1500c27e; continue;
 }
 0x1500c27e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500c299); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c2a0; continue; }
  pc = 0x1500c29b; continue;
 }
 0x1500c29b => {
  pc = 0x1500c7ab; continue;
 }
 0x1500c2a0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c7ab => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c7b2; continue;
 }
 0x1500c7b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010f2cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c7b2; continue; }
  pc = 0x1500c7f6; continue;
 }
 0x1500c7f6 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15011068u32), cpu.ebx); }
  if cpu.zf { pc = 0x1500c821; continue; }
  pc = 0x1500c804; continue;
 }
 0x1500c804 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15011068u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  cpu.push(0x1500c815); f_1500cb4e(cpu);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  let t = cpu.r32(0x15010f98u32); cpu.push(0x1500c820); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1500c821; continue;
 }
 0x1500c821 => {
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c82d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c2a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c2a1); }
 let mut pc: u32 = 0x1500c2a1;
 loop { match pc {
 0x1500c2a1 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500c2b9; continue;
 }
 0x1500c2b9 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c2b9; continue; }
  pc = 0x1500c2e1; continue;
 }
 0x1500c2e1 => {
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x1500c2ea; continue;
 }
 0x1500c2ea => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c2ea; continue; }
  pc = 0x1500c2fe; continue;
 }
 0x1500c2fe => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500c319); f_15009de8(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c327; continue; }
  pc = 0x1500c31b; continue;
 }
 0x1500c31b => {
  cpu.push(0x1500c320); f_1500a439(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c327; continue; }
  pc = 0x1500c322; continue;
 }
 0x1500c322 => {
  pc = 0x1500c7ab; continue;
 }
 0x1500c327 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500c7ab => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c7b2; continue;
 }
 0x1500c7b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010f2cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x24u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x28u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c7b2; continue; }
  pc = 0x1500c7f6; continue;
 }
 0x1500c7f6 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x15011068u32), cpu.ebx); }
  if cpu.zf { pc = 0x1500c821; continue; }
  pc = 0x1500c804; continue;
 }
 0x1500c804 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15011068u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  cpu.push(0x1500c815); f_1500cb4e(cpu);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  let t = cpu.r32(0x15010f98u32); cpu.push(0x1500c820); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1500c821; continue;
 }
 0x1500c821 => {
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c82d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c328(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c328); }
 let mut pc: u32 = 0x1500c328;
 loop { match pc {
 0x1500c328 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500c347; continue;
 }
 0x1500c347 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c347; continue; }
  pc = 0x1500c378; continue;
 }
 0x1500c378 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c3cc; continue; }
  pc = 0x1500c38f; continue;
 }
 0x1500c38f => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500c393; continue;
 }
 0x1500c393 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c393; continue; }
  pc = 0x1500c3a2; continue;
 }
 0x1500c3a2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500c3cc; continue;
 }
 0x1500c3cc => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c3cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c3cd); }
 let mut pc: u32 = 0x1500c3cd;
 loop { match pc {
 0x1500c3cd => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500c3ec; continue;
 }
 0x1500c3ec => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x15084110u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15084118u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c3ec; continue; }
  pc = 0x1500c41d; continue;
 }
 0x1500c41d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500c42b); f_15009f61(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c478; continue; }
  pc = 0x1500c42d; continue;
 }
 0x1500c42d => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c478; continue; }
  pc = 0x1500c43b; continue;
 }
 0x1500c43b => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500c43f; continue;
 }
 0x1500c43f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c43f; continue; }
  pc = 0x1500c44e; continue;
 }
 0x1500c44e => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500c478; continue;
 }
 0x1500c478 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c479(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c479); }
 let mut pc: u32 = 0x1500c479;
 loop { match pc {
 0x1500c479 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500c498; continue;
 }
 0x1500c498 => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c498; continue; }
  pc = 0x1500c4c0; continue;
 }
 0x1500c4c0 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500c4ce); f_15009b20(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c51b; continue; }
  pc = 0x1500c4d0; continue;
 }
 0x1500c4d0 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c51b; continue; }
  pc = 0x1500c4de; continue;
 }
 0x1500c4de => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500c4e2; continue;
 }
 0x1500c4e2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c4e2; continue; }
  pc = 0x1500c4f1; continue;
 }
 0x1500c4f1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500c51b; continue;
 }
 0x1500c51b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c51c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c51c); }
 let mut pc: u32 = 0x1500c51c;
 loop { match pc {
 0x1500c51c => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x15080da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1500c53b; continue;
 }
 0x1500c53b => {
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c53b; continue; }
  pc = 0x1500c563; continue;
 }
 0x1500c563 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011888u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x1500c571); f_15009b20(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c5c5; continue; }
  pc = 0x1500c573; continue;
 }
 0x1500c573 => {
  cpu.push(0x1500c578); f_15009f61(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c5c5; continue; }
  pc = 0x1500c57a; continue;
 }
 0x1500c57a => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c5c5; continue; }
  pc = 0x1500c588; continue;
 }
 0x1500c588 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500c58c; continue;
 }
 0x1500c58c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c58c; continue; }
  pc = 0x1500c59b; continue;
 }
 0x1500c59b => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500c5c5; continue;
 }
 0x1500c5c5 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c5c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c5c6); }
 let mut pc: u32 = 0x1500c5c6;
 loop { match pc {
 0x1500c5c6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c5c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c5c7); }
 let mut pc: u32 = 0x1500c5c7;
 loop { match pc {
 0x1500c5c7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c5c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c5c8); }
 let mut pc: u32 = 0x1500c5c8;
 loop { match pc {
 0x1500c5c8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x1500c5ce; continue;
 }
 0x1500c5ce => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.edi))); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x2000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500c5e8; continue; }
  pc = 0x1500c5e1; continue;
 }
 0x1500c5e1 => {
  { cpu.sub32(cpu.eax, 0xffffe000u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c5ef; continue; }
  pc = 0x1500c5e8; continue;
 }
 0x1500c5e8 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.edi)).wrapping_add(0x4000u32); cpu.w32(0x3cu32.wrapping_add(cpu.edi), v); }
  pc = 0x1500c5ef; continue;
 }
 0x1500c5ef => {
  { cpu.sub32(cpu.ebx, 0x2000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500c5ff; continue; }
  pc = 0x1500c5f7; continue;
 }
 0x1500c5f7 => {
  { cpu.sub32(cpu.ebx, 0xffffe000u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c606; continue; }
  pc = 0x1500c5ff; continue;
 }
 0x1500c5ff => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.edi)).wrapping_add(0x4000u32); cpu.w32(0x40u32.wrapping_add(cpu.edi), v); }
  pc = 0x1500c606; continue;
 }
 0x1500c606 => {
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c5ce; continue; }
  pc = 0x1500c60c; continue;
 }
 0x1500c60c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.edi))); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x2000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500c626; continue; }
  pc = 0x1500c61f; continue;
 }
 0x1500c61f => {
  { cpu.sub32(cpu.eax, 0xffffe000u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c62d; continue; }
  pc = 0x1500c626; continue;
 }
 0x1500c626 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)).wrapping_add(0x4000u32); cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x1500c62d; continue;
 }
 0x1500c62d => {
  { cpu.sub32(cpu.ebx, 0x2000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500c63d; continue; }
  pc = 0x1500c635; continue;
 }
 0x1500c635 => {
  { cpu.sub32(cpu.ebx, 0xffffe000u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c644; continue; }
  pc = 0x1500c63d; continue;
 }
 0x1500c63d => {
  { let v: u32 = cpu.add32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0x4000u32); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x1500c644; continue;
 }
 0x1500c644 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c648(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c648); }
 let mut pc: u32 = 0x1500c648;
 loop { match pc {
 0x1500c648 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1500c82eu32; cpu.w32(0x15041c6cu32, v); }
  let t = cpu.r32(0x15010f5cu32); cpu.push(0x1500c659); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0x15041c68u32, v); }
  cpu.push(0x1500c668); f_1500c747(cpu);
  let t = cpu.r32(0x15010f60u32); cpu.push(0x1500c66e); cpu.call(t);
  { let v: u32 = 0x1500c8b0u32; cpu.w32(0x15041c6cu32, v); }
  let t = cpu.r32(0x15010f64u32); cpu.push(0x1500c67e); cpu.call(t);
  { let v: u32 = 0x5u32; cpu.w32(0x15041c68u32, v); }
  cpu.push(0x1500c68d); f_1500c747(cpu);
  let t = cpu.r32(0x15010f68u32); cpu.push(0x1500c693); cpu.call(t);
  { let v: u32 = 0x1500c7abu32; cpu.w32(0x15041c6cu32, v); }
  let t = cpu.r32(0x15010f84u32); cpu.push(0x1500c6a3); cpu.call(t);
  { let v: u32 = 0x4u32; cpu.w32(0x15041c68u32, v); }
  cpu.push(0x1500c6b2); f_1500c747(cpu);
  let t = cpu.r32(0x15010f88u32); cpu.push(0x1500c6b8); cpu.call(t);
  let t = cpu.r32(0x15010f4cu32); cpu.push(0x1500c6be); cpu.call(t);
  { let v: u32 = 0x1500c82eu32; cpu.w32(0x15041c6cu32, v); }
  let t = cpu.r32(0x15010f5cu32); cpu.push(0x1500c6ce); cpu.call(t);
  { let v: u32 = 0x6u32; cpu.w32(0x15041c68u32, v); }
  cpu.push(0x1500c6dd); f_1500c747(cpu);
  let t = cpu.r32(0x15010f60u32); cpu.push(0x1500c6e3); cpu.call(t);
  let t = cpu.r32(0x15010f6cu32); cpu.push(0x1500c6e9); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0x15041c68u32, v); }
  cpu.push(0x1500c6f8); f_1500c747(cpu);
  let t = cpu.r32(0x15010f70u32); cpu.push(0x1500c6fe); cpu.call(t);
  { let v: u32 = 0x1500c7abu32; cpu.w32(0x15041c6cu32, v); }
  let t = cpu.r32(0x15010f74u32); cpu.push(0x1500c70e); cpu.call(t);
  { let v: u32 = 0x2u32; cpu.w32(0x15041c68u32, v); }
  cpu.push(0x1500c71d); f_1500c747(cpu);
  let t = cpu.r32(0x15010f78u32); cpu.push(0x1500c723); cpu.call(t);
  let t = cpu.r32(0x15010f7cu32); cpu.push(0x1500c729); cpu.call(t);
  { let v: u32 = 0x3u32; cpu.w32(0x15041c68u32, v); }
  cpu.push(0x1500c738); f_1500c747(cpu);
  let t = cpu.r32(0x15010f80u32); cpu.push(0x1500c73e); cpu.call(t);
  let t = cpu.r32(0x15010f48u32); cpu.push(0x1500c744); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c746(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c746); }
 let mut pc: u32 = 0x1500c746;
 loop { match pc {
 0x1500c746 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c747(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c747); }
 let mut pc: u32 = 0x1500c747;
 loop { match pc {
 0x1500c747 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.esi = v; }
  pc = 0x1500c74d; continue;
 }
 0x1500c74d => {
  { cpu.sub32(cpu.esi, 0x15042218u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c7aa; continue; }
  pc = 0x1500c755; continue;
 }
 0x1500c755 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x24u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x15041c68u32), cpu.edx); }
  if !cpu.zf { pc = 0x1500c74d; continue; }
  pc = 0x1500c76e; continue;
 }
 0x1500c76e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x150118ecu32; cpu.edx = v; }
  pc = 0x1500c776; continue;
 }
 0x1500c776 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x24u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c776; continue; }
  pc = 0x1500c781; continue;
 }
 0x1500c781 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x150414f8u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15080d9cu32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15041504u32, v); }
  { let v: u32 = 0x150118ecu32; cpu.esi = v; }
  let t = cpu.r32(0x15041c6cu32); cpu.push(0x1500c7a7); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1500c74d; continue;
 }
 0x1500c7aa => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c82e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c82e); }
 let mut pc: u32 = 0x1500c82e;
 loop { match pc {
 0x1500c82e => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1500c835; continue;
 }
 0x1500c835 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c835; continue; }
  pc = 0x1500c863; continue;
 }
 0x1500c863 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx & 0xf800u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 11u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15011174u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7e0u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x150111f4u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x150112f4u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = v; }
  let t = cpu.r32(0x15010f94u32); cpu.push(0x1500c8a2); cpu.call(t);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c8af); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c8b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c8b0); }
 let mut pc: u32 = 0x1500c8b0;
 loop { match pc {
 0x1500c8b0 => {
  { let v: u32 = 0x15041c70u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x1500c8b8; continue;
 }
 0x1500c8b8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1501103cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15011040u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x15011004u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x15011070u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x15011070u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15011074u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011074u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15011074u32.wrapping_add(cpu.edx.wrapping_mul(8))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x3cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500c8b8; continue; }
  pc = 0x1500c932; continue;
 }
 0x1500c932 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = 0x15041c70u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x15010f90u32); cpu.push(0x1500c93f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c940); }
 let mut pc: u32 = 0x1500c940;
 loop { match pc {
 0x1500c940 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1501661cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x15016620u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x15016624u32)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010064u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010068u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1501006cu32, v); }
  { let v: u32 = 0x15010064u32; cpu.esi = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  cpu.push(0x1500c97c); f_1500cfca(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c9e9; continue; }
  pc = 0x1500c985; continue;
 }
 0x1500c985 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15080d9cu32, v); }
  cpu.push(0x1500c99a); f_1500a6cf(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500c9e9; continue; }
  pc = 0x1500c99c; continue;
 }
 0x1500c99c => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500c9e9; continue; }
  pc = 0x1500c9aa; continue;
 }
 0x1500c9aa => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500c9ae; continue;
 }
 0x1500c9ae => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500c9ae; continue; }
  pc = 0x1500c9bd; continue;
 }
 0x1500c9bd => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x2u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500c9e9; continue;
 }
 0x1500c9e9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500c9eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500c9eb); }
 let mut pc: u32 = 0x1500c9eb;
 loop { match pc {
 0x1500c9eb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1501661cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x15016620u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x15016624u32)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010064u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010068u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1501006cu32, v); }
  { let v: u32 = 0x15010064u32; cpu.esi = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  cpu.push(0x1500ca27); f_1500cfca(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ca94; continue; }
  pc = 0x1500ca30; continue;
 }
 0x1500ca30 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15080d9cu32, v); }
  cpu.push(0x1500ca45); f_1500a6cf(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ca94; continue; }
  pc = 0x1500ca47; continue;
 }
 0x1500ca47 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500ca94; continue; }
  pc = 0x1500ca55; continue;
 }
 0x1500ca55 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500ca59; continue;
 }
 0x1500ca59 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ca59; continue; }
  pc = 0x1500ca68; continue;
 }
 0x1500ca68 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500ca94; continue;
 }
 0x1500ca94 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ca96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ca96); }
 let mut pc: u32 = 0x1500ca96;
 loop { match pc {
 0x1500ca96 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1501661cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x15016620u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x15016624u32)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010064u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15010068u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1501006cu32, v); }
  { let v: u32 = 0x15010064u32; cpu.esi = v; }
  { let v: u32 = 0x15016634u32; cpu.ebx = v; }
  cpu.push(0x1500cad2); f_1500cfca(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500cb3f; continue; }
  pc = 0x1500cadb; continue;
 }
 0x1500cadb => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15080d9cu32, v); }
  cpu.push(0x1500caf0); f_1500a6cf(cpu);
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500cb3f; continue; }
  pc = 0x1500caf2; continue;
 }
 0x1500caf2 => {
  { let v: u32 = cpu.r32(0x15042214u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x15080a18u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500cb3f; continue; }
  pc = 0x1500cb00; continue;
 }
 0x1500cb00 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500cb04; continue;
 }
 0x1500cb04 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500cb04; continue; }
  pc = 0x1500cb13; continue;
 }
 0x1500cb13 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150414f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15041504u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x15042214u32, v); }
  pc = 0x1500cb3f; continue;
 }
 0x1500cb3f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500cb41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500cb41); }
 let mut pc: u32 = 0x1500cb41;
 loop { match pc {
 0x1500cb41 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15080d9cu32, v); }
  cpu.push(0x1500cb4c); f_1500cb4e(cpu);
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500cb4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500cb4e); }
 let mut pc: u32 = 0x1500cb4e;
 loop { match pc {
 0x1500cb4e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1501197cu32)); cpu.w32(0x1501197cu32, v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1501197cu32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15011994u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub16(cpu.r16(0x15011998u32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if !cpu.zf { pc = 0x1500cc1b; continue; }
  pc = 0x1500cb77; continue;
 }
 0x1500cb77 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1501197cu32); cpu.edx = v; }
  { let v: u32 = 0x15010b2cu32; cpu.esi = v; }
  pc = 0x1500cb84; continue;
 }
 0x1500cb84 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500cb8e; continue; }
  pc = 0x1500cb88; continue;
 }
 0x1500cb88 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x1500cb8e; continue;
 }
 0x1500cb8e => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x15010b28u32), cpu.ecx); }
  if !cpu.zf { pc = 0x1500cb84; continue; }
  pc = 0x1500cb9a; continue;
 }
 0x1500cb9a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x1500cbaf; continue; }
  pc = 0x1500cba0; continue;
 }
 0x1500cba0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x15011998u32.wrapping_add(cpu.edx), v); }
  pc = 0x1500cbaf; continue;
 }
 0x1500cbaf => {
  { let v: u32 = cpu.eax; cpu.w32(0x1501106cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x15011998u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x15011988u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x1500cbd7; continue; }
  pc = 0x1500cbcc; continue;
 }
 0x1500cbcc => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  cpu.push(0x1500cbd7); f_1500e390(cpu);
  pc = 0x1500cbd7; continue;
 }
 0x1500cbd7 => {
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011988u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x15011984u32)); cpu.w32(0x15011984u32, v); }
  let t = cpu.r32(0x15010f9cu32); cpu.push(0x1500cbf7); cpu.call(t);
  { let v: u32 = cpu.r32(0x1501106cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010b2cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x15011980u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500cc41; continue; }
  pc = 0x1500cc0d; continue;
 }
 0x1500cc0d => {
  { let v: u32 = cpu.r32(0x1501197cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010b2cu32.wrapping_add(cpu.eax), v); }
  pc = 0x1500cc41; continue;
 }
 0x1500cc1b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15080d9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x15011998u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1501197cu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1501106cu32, v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15010b2cu32.wrapping_add(cpu.eax), v); }
  pc = 0x1500cc41; continue;
 }
 0x1500cc41 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500cc79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500cc79); }
 let mut pc: u32 = 0x1500cc79;
 loop { match pc {
 0x1500cc79 => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x15010fc0u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500ccd3; continue; }
  pc = 0x1500cc82; continue;
 }
 0x1500cc82 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fc4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1500ccd3; continue; }
  pc = 0x1500cc8a; continue;
 }
 0x1500cc8a => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x15010fe8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ccd3; continue; }
  pc = 0x1500cc92; continue;
 }
 0x1500cc92 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15010fb4u32); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x15010fb8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 12u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1000u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v = cpu.r32(0x15010fe8u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500ccd3; continue; }
  pc = 0x1500cccc; continue;
 }
 0x1500cccc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500ccd3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ccd7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ccd7); }
 let mut pc: u32 = 0x1500ccd7;
 loop { match pc {
 0x1500ccd7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ccd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ccd8); }
 let mut pc: u32 = 0x1500ccd8;
 loop { match pc {
 0x1500ccd8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ccd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ccd9); }
 let mut pc: u32 = 0x1500ccd9;
 loop { match pc {
 0x1500ccd9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ccdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ccdb); }
 let mut pc: u32 = 0x1500ccdb;
 loop { match pc {
 0x1500ccdb => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ccdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ccdd); }
 let mut pc: u32 = 0x1500ccdd;
 loop { match pc {
 0x1500ccdd => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ccde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ccde); }
 let mut pc: u32 = 0x1500ccde;
 loop { match pc {
 0x1500ccde => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ccdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ccdf); }
 let mut pc: u32 = 0x1500ccdf;
 loop { match pc {
 0x1500ccdf => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ccf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ccf0); }
 let mut pc: u32 = 0x1500ccf0;
 loop { match pc {
 0x1500ccf0 => {
  { let v: u32 = 0x1501661cu32; cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = 0x15016634u32; cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500cd07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500cd07); }
 let mut pc: u32 = 0x1500cd07;
 loop { match pc {
 0x1500cd07 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010058u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500cdda; continue; }
  pc = 0x1500cd1c; continue;
 }
 0x1500cd1c => {
  { let v: u32 = cpu.r32(0x15010060u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15010044u32); cpu.ecx = v; }
  { let v: u32 = 0x15087f90u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508218cu32, v); }
  pc = 0x1500cd39; continue;
 }
 0x1500cd39 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { let v = cpu.r32(0x1501006cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x15010068u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x15010064u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.sub32(cpu.r32(0x1508218cu32), cpu.edi); }
  if !cpu.zf { pc = 0x1500cd39; continue; }
  pc = 0x1500cdda; continue;
 }
 0x1500cdda => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500cddb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500cddb); }
 let mut pc: u32 = 0x1500cddb;
 loop { match pc {
 0x1500cddb => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010050u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1500ce59; continue; }
  pc = 0x1500cdec; continue;
 }
 0x1500cdec => {
  { let v: u32 = cpu.r32(0x15010060u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15010048u32); cpu.ecx = v; }
  { let v: u32 = 0x1508a6a8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1500ce06); f_1500d43b(cpu);
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x1500ce0c; continue;
 }
 0x1500ce0c => {
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x1501679cu32); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x150167a0u32); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x150167a4u32); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  cpu.unimplemented(0x1500ce38, "Shrd");
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x1500ce44; continue; }
  pc = 0x1500ce42; continue;
 }
 0x1500ce42 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x1500ce44; continue;
 }
 0x1500ce44 => {
  { let v: u32 = cpu.eax.wrapping_add(0x300u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x3fffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ce54; continue; }
  pc = 0x1500ce4f; continue;
 }
 0x1500ce4f => {
  { let v: u32 = 0x3fffu32; cpu.eax = v; }
  pc = 0x1500ce54; continue;
 }
 0x1500ce54 => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ce0c; continue; }
  pc = 0x1500ce59; continue;
 }
 0x1500ce59 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ce5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ce5a); }
 let mut pc: u32 = 0x1500ce5a;
 loop { match pc {
 0x1500ce5a => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1501005cu32, v); }
  { let v: u32 = cpu.r32(0x1501004cu32); cpu.ecx = v; }
  { let v: u32 = 0x1508ae7cu32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508218cu32, v); }
  pc = 0x1500ce7c; continue;
 }
 0x1500ce7c => {
  { let v: u32 = 0x150166a0u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15011954u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15011958u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1501195cu32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x15011954u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011958u32); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x1500cf06); f_1500e09f(cpu);
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x1501195cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15011958u32); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x1500cf26); f_1500e09f(cpu);
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1508218cu32), cpu.edi); }
  if !cpu.zf { pc = 0x1500ce7c; continue; }
  pc = 0x1500cf49; continue;
 }
 0x1500cf49 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500cf4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500cf4a); }
 let mut pc: u32 = 0x1500cf4a;
 loop { match pc {
 0x1500cf4a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500cfca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500cfca); }
 let mut pc: u32 = 0x1500cfca;
 loop { match pc {
 0x1500cfca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500d09c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500d09c); }
 let mut pc: u32 = 0x1500d09c;
 loop { match pc {
 0x1500d09c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500d109(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500d109); }
 let mut pc: u32 = 0x1500d109;
 loop { match pc {
 0x1500d109 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082110u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082114u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082118u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508211cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082120u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082124u32, v); }
  { let v: u32 = 0x15082110u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x15082120u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x15010fe8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500d1f7; continue; }
  pc = 0x1500d1b9; continue;
 }
 0x1500d1b9 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x15082128u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1508211cu32)); cpu.eax = v; }
  { let v = cpu.r32(0x15010fe8u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15010fc8u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x15082124u32)); cpu.eax = v; }
  { let v = cpu.r32(0x15010fe8u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x15010fccu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500d1f7 => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500d1fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500d1fe); }
 let mut pc: u32 = 0x1500d1fe;
 loop { match pc {
 0x1500d1fe => {
  { cpu.sub32(cpu.r32(0x15010060u32), 0x15016634u32); }
  if cpu.zf { pc = 0x1500d314; continue; }
  pc = 0x1500d20e; continue;
 }
 0x1500d20e => {
  { let v: u32 = cpu.r32(0x15010060u32); cpu.eax = v; }
  { let v: u32 = 0x15082138u32; cpu.ebx = v; }
  cpu.push(0x1500d21d); f_1500e2ab(cpu);
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010054u32, v); }
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = 0x15082138u32; cpu.ebx = v; }
  { let v = (cpu.r32(0x15010064u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1508212cu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x15010068u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15082130u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x1501006cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15082134u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x15010fe8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010054u32); cpu.ecx = v; }
  pc = 0x1500d274; continue;
 }
 0x1500d274 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x1508212cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x15082130u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x20u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(7); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x15082134u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500d2f7; continue; }
  pc = 0x1500d2d3; continue;
 }
 0x1500d2d3 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010fecu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x15010fd4u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x1500d2f7; continue;
 }
 0x1500d2f7 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500d274; continue; }
  pc = 0x1500d313; continue;
 }
 0x1500d313 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500d314 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15010054u32, v); }
  { let v: u32 = cpu.r32(0x15010040u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x1508410cu32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = 0x150821c8u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15010084u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1501661cu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508215cu32, v); }
  { let v = (cpu.r32(0x1508215cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15016620u32)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x1508215cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x15082160u32, v); }
  { let v = (cpu.r32(0x15082160u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x15016624u32)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x15082160u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x15082164u32, v); }
  { let v = (cpu.r32(0x15082164u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x15082164u32, v); cpu.fpop(); }
  { let v: u32 = 0x15016658u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15010fe8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15010054u32); cpu.ecx = v; }
  pc = 0x1500d39b; continue;
 }
 0x1500d39b => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15082164u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x15082160u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x1508215cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x20u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(7); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1500d41e; continue; }
  pc = 0x1500d3fa; continue;
 }
 0x1500d3fa => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0) * cpu.rf32(0x15010fecu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x15010fd0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x15010fd4u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x1500d41e; continue;
 }
 0x1500d41e => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500d39b; continue; }
  pc = 0x1500d43a; continue;
 }
 0x1500d43a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500d43b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500d43b); }
 let mut pc: u32 = 0x1500d43b;
 loop { match pc {
 0x1500d43b => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508218cu32, v); }
  pc = 0x1500d445; continue;
 }
 0x1500d445 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0xcu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  cpu.unimplemented(0x1500d469, "Shrd");
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  cpu.unimplemented(0x1500d497, "Shrd");
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x14u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  cpu.unimplemented(0x1500d4c5, "Shrd");
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1508218cu32), cpu.edi); }
  if !cpu.zf { pc = 0x1500d445; continue; }
  pc = 0x1500d4e0; continue;
 }
 0x1500d4e0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500d8b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500d8b9); }
 let mut pc: u32 = 0x1500d8b9;
 loop { match pc {
 0x1500d8b9 => {
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3fffu32; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500d8f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500d8f3); }
 let mut pc: u32 = 0x1500d8f3;
 loop { match pc {
 0x1500d8f3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xfffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082174u32, v); }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082180u32, v); }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082178u32, v); }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082184u32, v); }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508217cu32, v); }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082188u32, v); }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082178u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082178u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082180u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082178u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082180u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082180u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082184u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082184u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082180u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082184u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500da56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500da56); }
 let mut pc: u32 = 0x1500da56;
 loop { match pc {
 0x1500da56 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xfffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082174u32, v); }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082180u32, v); }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082178u32, v); }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082184u32, v); }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508217cu32, v); }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082188u32, v); }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082178u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082180u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082184u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082178u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082180u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508217cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082184u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082188u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082180u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082178u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082174u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082180u32); cpu.eax = v; }
  { let v = cpu.r32(0x15082184u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500dbbf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500dbbf); }
 let mut pc: u32 = 0x1500dbbf;
 loop { match pc {
 0x1500dbbf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ddd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ddd4); }
 let mut pc: u32 = 0x1500ddd4;
 loop { match pc {
 0x1500ddd4 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x15082178u32, v); }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082184u32, v); }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082178u32); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082184u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500deaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500deaf); }
 let mut pc: u32 = 0x1500deaf;
 loop { match pc {
 0x1500deaf => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1508217cu32, v); }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082188u32, v); }
  { let v: u32 = cpu.r32(0x1508217cu32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082188u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1508217cu32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082188u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1508217cu32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082188u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1508217cu32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082188u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1508217cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082188u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1508217cu32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15082188u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500df84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500df84); }
 let mut pc: u32 = 0x1500df84;
 loop { match pc {
 0x1500df84 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1500df8a); f_1500dfdd(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x1500df9a); f_1500dfdd(cpu);
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500dfdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500dfdd); }
 let mut pc: u32 = 0x1500dfdd;
 loop { match pc {
 0x1500dfdd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x1500dff8; continue;
 }
 0x1500dff8 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x1500e003; continue; }
  pc = 0x1500dffc; continue;
 }
 0x1500dffc => {
  { cpu.sub32(cpu.eax, 0x3fffffffu32); }
  if cpu.cf { pc = 0x1500e00d; continue; }
  pc = 0x1500e003; continue;
 }
 0x1500e003 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1500dff8; continue;
 }
 0x1500e00d => {
  cpu.push(0x1500e012); f_1500e07c(cpu);
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x1500e01f; continue; }
  pc = 0x1500e01a; continue;
 }
 0x1500e01a => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x1500e01f; continue;
 }
 0x1500e01f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e045(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e045); }
 let mut pc: u32 = 0x1500e045;
 loop { match pc {
 0x1500e045 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x1500e05e; continue;
 }
 0x1500e05e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x1500e069; continue; }
  pc = 0x1500e062; continue;
 }
 0x1500e062 => {
  { cpu.sub32(cpu.eax, 0x3fffffffu32); }
  if cpu.cf { pc = 0x1500e073; continue; }
  pc = 0x1500e069; continue;
 }
 0x1500e069 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1500e05e; continue;
 }
 0x1500e073 => {
  cpu.push(0x1500e078); f_1500e07c(cpu);
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e07c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e07c); }
 let mut pc: u32 = 0x1500e07c;
 loop { match pc {
 0x1500e07c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x10000000u32; cpu.ebx = v; }
  pc = 0x1500e086; continue;
 }
 0x1500e086 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x1500e094; continue; }
  pc = 0x1500e090; continue;
 }
 0x1500e090 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  pc = 0x1500e094; continue;
 }
 0x1500e094 => {
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  if !cpu.zf { pc = 0x1500e086; continue; }
  pc = 0x1500e099; continue;
 }
 0x1500e099 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e09f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e09f); }
 let mut pc: u32 = 0x1500e09f;
 loop { match pc {
 0x1500e09f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500e0aa; continue; }
  pc = 0x1500e0a8; continue;
 }
 0x1500e0a8 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x1500e0aa; continue;
 }
 0x1500e0aa => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500e0b0; continue; }
  pc = 0x1500e0ae; continue;
 }
 0x1500e0ae => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x1500e0b0; continue;
 }
 0x1500e0b0 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u8 = ((cpu.zf || cpu.sf != cpu.of)) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500e0b8; continue; }
  pc = 0x1500e0b7; continue;
 }
 0x1500e0b7 => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x1500e0b8; continue;
 }
 0x1500e0b8 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1500e0eb; continue; }
  pc = 0x1500e0bc; continue;
 }
 0x1500e0bc => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1508e620u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x1500e0dc; continue; }
  pc = 0x1500e0d5; continue;
 }
 0x1500e0d5 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x400u32); cpu.eax = v; }
  pc = 0x1500e0dc; continue;
 }
 0x1500e0dc => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.ebp); cpu.esi = v; }
  if (cpu.sf == cpu.of) { pc = 0x1500e0e2; continue; }
  pc = 0x1500e0e0; continue;
 }
 0x1500e0e0 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x1500e0e2; continue;
 }
 0x1500e0e2 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x1500e0eb; continue; }
  pc = 0x1500e0e6; continue;
 }
 0x1500e0e6 => {
  { let v: u32 = cpu.eax.wrapping_add(0x800u32); cpu.eax = v; }
  pc = 0x1500e0eb; continue;
 }
 0x1500e0eb => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e0f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e0f2); }
 let mut pc: u32 = 0x1500e0f2;
 loop { match pc {
 0x1500e0f2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082190u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082194u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15082198u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508219cu32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150821a0u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150821a4u32, v); }
  { let v: u32 = cpu.r32(0x15082190u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15082194u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15082198u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1500e14e); f_1500e07c(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15082190u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15082190u32, v); }
  { let v: u32 = cpu.r32(0x15082194u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15082194u32, v); }
  { let v: u32 = cpu.r32(0x15082198u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15082198u32, v); }
  { let v: u32 = cpu.r32(0x1508219cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x150821a0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x150821a4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1500e1bc); f_1500e07c(cpu);
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1508219cu32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1508219cu32, v); }
  { let v: u32 = cpu.r32(0x150821a0u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x150821a0u32, v); }
  { let v: u32 = cpu.r32(0x150821a4u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x150821a4u32, v); }
  { let v: u32 = cpu.r32(0x15082198u32); cpu.eax = v; }
  { let v = cpu.r32(0x150821a0u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15082194u32); cpu.eax = v; }
  { let v = cpu.r32(0x150821a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150821a8u32, v); }
  { let v: u32 = cpu.r32(0x15082190u32); cpu.eax = v; }
  { let v = cpu.r32(0x150821a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15082198u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508219cu32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150821acu32, v); }
  { let v: u32 = cpu.r32(0x15082194u32); cpu.eax = v; }
  { let v = cpu.r32(0x1508219cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15082190u32); cpu.eax = v; }
  { let v = cpu.r32(0x150821a0u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150821b0u32, v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15082190u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1508219cu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x150821a8u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082194u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x150821a0u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x150821acu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x15082198u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x150821a4u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x150821b0u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e2ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e2ab); }
 let mut pc: u32 = 0x1500e2ab;
 loop { match pc {
 0x1500e2ab => {
  { let v = cpu.rf32(0x15011968u32); cpu.fpush(v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x10u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x10u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x1cu32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x18u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x14u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x20u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e2fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e2fa); }
 let mut pc: u32 = 0x1500e2fa;
 loop { match pc {
 0x1500e2fa => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1508be20u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e308(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e308); }
 let mut pc: u32 = 0x1500e308;
 loop { match pc {
 0x1500e308 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1508be20u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1508be20u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.sar16((cpu.edx as u16), 15u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.idiv16(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e345(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e345); }
 let mut pc: u32 = 0x1500e345;
 loop { match pc {
 0x1500e345 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1508c620u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e353(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e353); }
 let mut pc: u32 = 0x1500e353;
 loop { match pc {
 0x1500e353 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1508c620u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1508c620u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.sar16((cpu.edx as u16), 15u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.idiv16(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e390); }
 let mut pc: u32 = 0x1500e390;
 loop { match pc {
 0x1500e390 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1501197cu32)); cpu.w32(0x1501197cu32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1501196cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1501197cu32); cpu.edx = v; }
  pc = 0x1500e3a5; continue;
 }
 0x1500e3a5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1500e3d8; continue; }
  pc = 0x1500e3ab; continue;
 }
 0x1500e3ab => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500e3bb; continue; }
  pc = 0x1500e3b0; continue;
 }
 0x1500e3b0 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x1500e3bb; continue; }
  pc = 0x1500e3b6; continue;
 }
 0x1500e3b6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x1500e3bb; continue;
 }
 0x1500e3bb => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x150821c0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x1500e3a5; continue; }
  pc = 0x1500e3c7; continue;
 }
 0x1500e3c7 => {
  { let v: u32 = cpu.r32(0x1501196cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1500e3da; continue;
 }
 0x1500e3d8 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x1500e3da; continue;
 }
 0x1500e3da => {
  { let v: u32 = cpu.r32(0x1501196cu32); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15011988u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x15010b24u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x150821c4u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150821b4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x150821b4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x15011978u32); cpu.push(0x1500e416); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1500e425; continue; }
  pc = 0x1500e41d; continue;
 }
 0x1500e41d => {
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x150821b4u32); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500e425 => {
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e490); }
 let mut pc: u32 = 0x1500e490;
 loop { match pc {
 0x1500e490 => {
  { let v: u32 = 0x1500e7a0u32; cpu.w32(0x150167c0u32, v); }
  pc = 0x1500e4a8; continue;
 }
 0x1500e4a8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x4e534956u32); }
  if !cpu.zf { pc = 0x1500e5a2; continue; }
  pc = 0x1500e4b5; continue;
 }
 0x1500e4b5 => {
  { let v: u32 = cpu.edi; cpu.w32(0x1508ee24u32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x150167c4u32)); }
  if cpu.zf { pc = 0x1500e4c8; continue; }
  pc = 0x1500e4c3; continue;
 }
 0x1500e4c3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x150167c4u32, v); }
  pc = 0x1500e4c8; continue;
 }
 0x1500e4c8 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150167c8u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150167ccu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee30u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee34u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x15016868u32), cpu.eax); }
  if cpu.zf { pc = 0x1500e526; continue; }
  pc = 0x1500e4ff; continue;
 }
 0x1500e4ff => {
  { let v: u32 = cpu.eax; cpu.w32(0x15016868u32, v); }
  { cpu.pushad(); }
  { let v: u32 = 0x1501686cu32; cpu.esi = v; }
  { let v: u32 = 0x150902e8u32; cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500e515); f_1500ee12(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x150168acu32; cpu.esi = v; }
  { let v: u32 = 0x150904e8u32; cpu.edi = v; }
  cpu.push(0x1500e525); f_1500ee12(cpu);
  { cpu.popad(); }
  pc = 0x1500e526; continue;
 }
 0x1500e526 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x150167b8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150167d0u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u8 = 0x20u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = 0x150900f0u32; cpu.edi = v; }
  cpu.push(0x1500e54e); f_1500eb71(cpu);
  { cpu.pushad(); }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x1508fde8u32; cpu.ecx = v; }
  { let v: u32 = 0x1508fee8u32; cpu.edx = v; }
  { let v: u32 = 0x1508ffecu32; cpu.ebp = v; }
  { let v: u32 = 0x150900f0u32; cpu.esi = v; }
  cpu.push(0x1500e56d); f_1500ec52(cpu);
  { cpu.popad(); }
  if cpu.zf { pc = 0x1500e5a2; continue; }
  pc = 0x1500e570; continue;
 }
 0x1500e570 => {
  { let v: u32 = 0x1508fd64u32; cpu.edi = v; }
  cpu.push(0x1500e57a); f_1500eb71(cpu);
  { cpu.pushad(); }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x1508fa5cu32; cpu.ecx = v; }
  { let v: u32 = 0x1508fb5cu32; cpu.edx = v; }
  { let v: u32 = 0x1508fc60u32; cpu.ebp = v; }
  { let v: u32 = 0x1508fd64u32; cpu.esi = v; }
  cpu.push(0x1500e599); f_1500ec52(cpu);
  { cpu.popad(); }
  if cpu.zf { pc = 0x1500e5a2; continue; }
  pc = 0x1500e59c; continue;
 }
 0x1500e59c => {
  let t = cpu.r32(0x150167d0u32); cpu.push(0x1500e5a2); cpu.call(t);
  pc = 0x1500e5a2; continue;
 }
 0x1500e5a2 => {
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e49c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e49c); }
 let mut pc: u32 = 0x1500e49c;
 loop { match pc {
 0x1500e49c => {
  { let v: u32 = 0x1500e6c3u32; cpu.w32(0x150167c0u32, v); }
  pc = 0x1500e4a8; continue;
 }
 0x1500e4a8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x4e534956u32); }
  if !cpu.zf { pc = 0x1500e5a2; continue; }
  pc = 0x1500e4b5; continue;
 }
 0x1500e4b5 => {
  { let v: u32 = cpu.edi; cpu.w32(0x1508ee24u32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x150167c4u32)); }
  if cpu.zf { pc = 0x1500e4c8; continue; }
  pc = 0x1500e4c3; continue;
 }
 0x1500e4c3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x150167c4u32, v); }
  pc = 0x1500e4c8; continue;
 }
 0x1500e4c8 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150167c8u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150167ccu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee30u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee34u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x15016868u32), cpu.eax); }
  if cpu.zf { pc = 0x1500e526; continue; }
  pc = 0x1500e4ff; continue;
 }
 0x1500e4ff => {
  { let v: u32 = cpu.eax; cpu.w32(0x15016868u32, v); }
  { cpu.pushad(); }
  { let v: u32 = 0x1501686cu32; cpu.esi = v; }
  { let v: u32 = 0x150902e8u32; cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1500e515); f_1500ee12(cpu);
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x150168acu32; cpu.esi = v; }
  { let v: u32 = 0x150904e8u32; cpu.edi = v; }
  cpu.push(0x1500e525); f_1500ee12(cpu);
  { cpu.popad(); }
  pc = 0x1500e526; continue;
 }
 0x1500e526 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x150167b8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x150167d0u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u8 = 0x20u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = 0x150900f0u32; cpu.edi = v; }
  cpu.push(0x1500e54e); f_1500eb71(cpu);
  { cpu.pushad(); }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x1508fde8u32; cpu.ecx = v; }
  { let v: u32 = 0x1508fee8u32; cpu.edx = v; }
  { let v: u32 = 0x1508ffecu32; cpu.ebp = v; }
  { let v: u32 = 0x150900f0u32; cpu.esi = v; }
  cpu.push(0x1500e56d); f_1500ec52(cpu);
  { cpu.popad(); }
  if cpu.zf { pc = 0x1500e5a2; continue; }
  pc = 0x1500e570; continue;
 }
 0x1500e570 => {
  { let v: u32 = 0x1508fd64u32; cpu.edi = v; }
  cpu.push(0x1500e57a); f_1500eb71(cpu);
  { cpu.pushad(); }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x1508fa5cu32; cpu.ecx = v; }
  { let v: u32 = 0x1508fb5cu32; cpu.edx = v; }
  { let v: u32 = 0x1508fc60u32; cpu.ebp = v; }
  { let v: u32 = 0x1508fd64u32; cpu.esi = v; }
  cpu.push(0x1500e599); f_1500ec52(cpu);
  { cpu.popad(); }
  if cpu.zf { pc = 0x1500e5a2; continue; }
  pc = 0x1500e59c; continue;
 }
 0x1500e59c => {
  let t = cpu.r32(0x150167d0u32); cpu.push(0x1500e5a2); cpu.call(t);
  pc = 0x1500e5a2; continue;
 }
 0x1500e5a2 => {
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e5a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e5a4); }
 let mut pc: u32 = 0x1500e5a4;
 loop { match pc {
 0x1500e5a4 => {
  { let v: u32 = cpu.r32(0x1508ee30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150167c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1508ee38u32, v); }
  { let v: u32 = cpu.r32(0x1508ee34u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee2cu32, v); }
  pc = 0x1500e5c9; continue;
 }
 0x1500e5c9 => {
  { let v: u32 = cpu.r32(0x1508ee30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee28u32, v); }
  pc = 0x1500e5d3; continue;
 }
 0x1500e5d3 => {
  { let v: u32 = 0x1508ee5cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee44u32); cpu.eax = v; }
  cpu.push(0x1500e5e2); f_1500e9f7(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee44u32, v); }
  { let v: u32 = 0x1508ef5cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee48u32); cpu.eax = v; }
  cpu.push(0x1500e5f6); f_1500e9f7(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee48u32, v); }
  { let v: u32 = 0x1508f05cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee4cu32); cpu.eax = v; }
  cpu.push(0x1500e60a); f_1500e87d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee4cu32, v); }
  { let v: u32 = 0x1508f15cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee50u32); cpu.eax = v; }
  cpu.push(0x1500e61e); f_1500e87d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee50u32, v); }
  { let v: u32 = 0x1508f25cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee54u32); cpu.eax = v; }
  cpu.push(0x1500e632); f_1500e87d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee54u32, v); }
  { let v: u32 = 0x1508f35cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee58u32); cpu.eax = v; }
  cpu.push(0x1500e646); f_1500e87d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee58u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1508ee5cu32; cpu.esi = v; }
  cpu.push(0x1500e658); f_1500ee7a(cpu);
  { let v: u32 = 0x1508ef5cu32; cpu.esi = v; }
  cpu.push(0x1500e662); f_1500ee7a(cpu);
  { let v: u32 = 0x1508f05cu32; cpu.esi = v; }
  cpu.push(0x1500e66c); f_1500ee7a(cpu);
  { let v: u32 = 0x1508f15cu32; cpu.esi = v; }
  cpu.push(0x1500e676); f_1500ee7a(cpu);
  { let v: u32 = 0x1508f25cu32; cpu.esi = v; }
  cpu.push(0x1500e680); f_1500ee7a(cpu);
  { let v: u32 = 0x1508f35cu32; cpu.esi = v; }
  cpu.push(0x1500e68a); f_1500ee7a(cpu);
  { let v: u32 = cpu.r32(0x1508ee24u32); cpu.edi = v; }
  cpu.push(0x1500e695); f_1500f12e(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1508ee24u32).wrapping_add(0x20u32); cpu.w32(0x1508ee24u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1508ee28u32)); cpu.w32(0x1508ee28u32, v); }
  if !cpu.zf { pc = 0x1500e5d3; continue; }
  pc = 0x1500e6ab; continue;
 }
 0x1500e6ab => {
  { let v: u32 = cpu.r32(0x1508ee38u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x1508ee24u32), cpu.eax); cpu.w32(0x1508ee24u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1508ee2cu32)); cpu.w32(0x1508ee2cu32, v); }
  if !cpu.zf { pc = 0x1500e5c9; continue; }
  pc = 0x1500e6c2; continue;
 }
 0x1500e6c2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e6c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e6c3); }
 let mut pc: u32 = 0x1500e6c3;
 loop { match pc {
 0x1500e6c3 => {
  { let v: u32 = cpu.r32(0x1508ee30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1508ee34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1508ee30u32).wrapping_add(cpu.eax); cpu.w32(0x1508ee30u32, v); }
  { let v: u32 = cpu.r32(0x1508ee34u32).wrapping_add(cpu.ebx); cpu.w32(0x1508ee34u32, v); }
  { let v: u32 = cpu.r32(0x1508ee30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150167c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1508ee38u32, v); }
  { let v: u32 = cpu.r32(0x1508ee34u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee2cu32, v); }
  pc = 0x1500e6ff; continue;
 }
 0x1500e6ff => {
  { let v: u32 = cpu.r32(0x1508ee30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee28u32, v); }
  pc = 0x1500e709; continue;
 }
 0x1500e709 => {
  { let v: u32 = 0x1508ee5cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee44u32); cpu.eax = v; }
  cpu.push(0x1500e718); f_1500e9f7(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee44u32, v); }
  { let v: u32 = 0x1508ef5cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee48u32); cpu.eax = v; }
  cpu.push(0x1500e72c); f_1500e9f7(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee48u32, v); }
  { let v: u32 = 0x1508f05cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee4cu32); cpu.eax = v; }
  cpu.push(0x1500e740); f_1500e87d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee4cu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1508ee5cu32; cpu.esi = v; }
  cpu.push(0x1500e752); f_1500ee7a(cpu);
  { let v: u32 = 0x1508ef5cu32; cpu.esi = v; }
  cpu.push(0x1500e75c); f_1500ee7a(cpu);
  { let v: u32 = 0x1508f05cu32; cpu.esi = v; }
  cpu.push(0x1500e766); f_1500ee7a(cpu);
  { let v: u32 = 0x1508f45cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1508ee24u32); cpu.edi = v; }
  cpu.push(0x1500e776); f_1500f3f0(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1508ee24u32).wrapping_add(0x10u32); cpu.w32(0x1508ee24u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1508ee28u32)); cpu.w32(0x1508ee28u32, v); }
  if !cpu.zf { pc = 0x1500e709; continue; }
  pc = 0x1500e788; continue;
 }
 0x1500e788 => {
  { let v: u32 = cpu.r32(0x1508ee38u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x1508ee24u32), cpu.eax); cpu.w32(0x1508ee24u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1508ee2cu32)); cpu.w32(0x1508ee2cu32, v); }
  if !cpu.zf { pc = 0x1500e6ff; continue; }
  pc = 0x1500e79f; continue;
 }
 0x1500e79f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e7a0); }
 let mut pc: u32 = 0x1500e7a0;
 loop { match pc {
 0x1500e7a0 => {
  { let v: u32 = cpu.r32(0x1508ee30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1508ee34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1508ee30u32).wrapping_add(cpu.eax); cpu.w32(0x1508ee30u32, v); }
  { let v: u32 = cpu.r32(0x1508ee34u32).wrapping_add(cpu.ebx); cpu.w32(0x1508ee34u32, v); }
  { let v: u32 = cpu.r32(0x1508ee30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x150167c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1508ee38u32, v); }
  { let v: u32 = cpu.r32(0x1508ee34u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee2cu32, v); }
  pc = 0x1500e7dc; continue;
 }
 0x1500e7dc => {
  { let v: u32 = cpu.r32(0x1508ee30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee28u32, v); }
  pc = 0x1500e7e6; continue;
 }
 0x1500e7e6 => {
  { let v: u32 = 0x1508ee5cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee44u32); cpu.eax = v; }
  cpu.push(0x1500e7f5); f_1500e9f7(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee44u32, v); }
  { let v: u32 = 0x1508ef5cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee48u32); cpu.eax = v; }
  cpu.push(0x1500e809); f_1500e9f7(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee48u32, v); }
  { let v: u32 = 0x1508f05cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1508ee4cu32); cpu.eax = v; }
  cpu.push(0x1500e81d); f_1500e87d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0x1508ee4cu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1508ee5cu32; cpu.esi = v; }
  cpu.push(0x1500e82f); f_1500ee7a(cpu);
  { let v: u32 = 0x1508ef5cu32; cpu.esi = v; }
  cpu.push(0x1500e839); f_1500ee7a(cpu);
  { let v: u32 = 0x1508f05cu32; cpu.esi = v; }
  cpu.push(0x1500e843); f_1500ee7a(cpu);
  { let v: u32 = 0x1508f45cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1508ee24u32); cpu.edi = v; }
  cpu.push(0x1500e853); f_1500f66f(cpu);
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1508ee24u32).wrapping_add(0x8u32); cpu.w32(0x1508ee24u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1508ee28u32)); cpu.w32(0x1508ee28u32, v); }
  if !cpu.zf { pc = 0x1500e7e6; continue; }
  pc = 0x1500e865; continue;
 }
 0x1500e865 => {
  { let v: u32 = cpu.r32(0x1508ee38u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x1508ee24u32), cpu.eax); cpu.w32(0x1508ee24u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1508ee2cu32)); cpu.w32(0x1508ee2cu32, v); }
  if !cpu.zf { pc = 0x1500e7dc; continue; }
  pc = 0x1500e87c; continue;
 }
 0x1500e87c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e87d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e87d); }
 let mut pc: u32 = 0x1500e87d;
 loop { match pc {
 0x1500e87d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x150902e8u32; cpu.edx = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500e89a; continue; }
  pc = 0x1500e888; continue;
 }
 0x1500e888 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500e89a; continue;
 }
 0x1500e89a => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x6u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 6u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 26u32); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1508ee3cu32, v); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0xcu8); }
  if (cpu.sf == cpu.of) { pc = 0x1500e8c6; continue; }
  pc = 0x1500e8b4; continue;
 }
 0x1500e8b4 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500e8c6; continue;
 }
 0x1500e8c6 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0xcu8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 12u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1508ee3cu32)); }
  if cpu.zf { pc = 0x1500e9de; continue; }
  pc = 0x1500e8e7; continue;
 }
 0x1500e8e7 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500e8fe; continue; }
  pc = 0x1500e8ec; continue;
 }
 0x1500e8ec => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500e8fe; continue;
 }
 0x1500e8fe => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1508fa5cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x74u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500e93a; continue; }
  pc = 0x1500e910; continue;
 }
 0x1500e910 => {
  { let v: u8 = cpu.r8(0x1508fd64u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500e91a; continue;
 }
 0x1500e91a => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x3au32); cpu.eax = v; }
  if cpu.zf { pc = 0x1500e925; continue; }
  pc = 0x1500e91f; continue;
 }
 0x1500e91f => {
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 14u32); cpu.eax = v; }
  pc = 0x1500e925; continue;
 }
 0x1500e925 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1508ee3cu32)); }
  if !cpu.zf { pc = 0x1500e8e7; continue; }
  pc = 0x1500e935; continue;
 }
 0x1500e935 => {
  pc = 0x1500e9de; continue;
 }
 0x1500e93a => {
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500e98f; continue; }
  pc = 0x1500e941; continue;
 }
 0x1500e941 => {
  { let v: u8 = cpu.r8(0x1508fd64u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500e94b; continue;
 }
 0x1500e94b => {
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), (cpu.eax as u8)); }
  if (cpu.sf == cpu.of) { pc = 0x1500e964; continue; }
  pc = 0x1500e952; continue;
 }
 0x1500e952 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500e964; continue;
 }
 0x1500e964 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 14u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x1508ee3cu32)); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { pc = 0x1500e8e7; continue; }
  pc = 0x1500e98d; continue;
 }
 0x1500e98d => {
  pc = 0x1500e9de; continue;
 }
 0x1500e98f => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 8u32); cpu.ebp = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500e9ac; continue; }
  pc = 0x1500e99a; continue;
 }
 0x1500e99a => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500e9ac; continue;
 }
 0x1500e9ac => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1500e9c1; continue; }
  pc = 0x1500e9b2; continue;
 }
 0x1500e9b2 => {
  { let v: u8 = cpu.r8(0x1508fbdfu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf != cpu.of) { pc = 0x1500e9d0; continue; }
  pc = 0x1500e9bf; continue;
 }
 0x1500e9bf => {
  pc = 0x1500e9ac; continue;
 }
 0x1500e9c1 => {
  { let v: u8 = cpu.r8(0x1508fadbu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf != cpu.of) { pc = 0x1500e9d0; continue; }
  pc = 0x1500e9ce; continue;
 }
 0x1500e9ce => {
  pc = 0x1500e9ac; continue;
 }
 0x1500e9d0 => {
  { cpu.sub32(cpu.eax, 0x74u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500e91a; continue; }
  pc = 0x1500e9d9; continue;
 }
 0x1500e9d9 => {
  pc = 0x1500e94b; continue;
 }
 0x1500e9de => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500e9f1; continue; }
  pc = 0x1500e9e3; continue;
 }
 0x1500e9e3 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x1500e9e5; continue;
 }
 0x1500e9e5 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x1500e9e5; continue; }
  pc = 0x1500e9f1; continue;
 }
 0x1500e9f1 => {
  { let v: u32 = cpu.r32(0x1508ee3cu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500e9f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500e9f7); }
 let mut pc: u32 = 0x1500e9f7;
 loop { match pc {
 0x1500e9f7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x150904e8u32; cpu.edx = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500ea14; continue; }
  pc = 0x1500ea02; continue;
 }
 0x1500ea02 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ea14; continue;
 }
 0x1500ea14 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x6u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 6u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 26u32); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1508ee40u32, v); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0xcu8); }
  if (cpu.sf == cpu.of) { pc = 0x1500ea40; continue; }
  pc = 0x1500ea2e; continue;
 }
 0x1500ea2e => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ea40; continue;
 }
 0x1500ea40 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0xcu8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 12u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1508ee40u32)); }
  if cpu.zf { pc = 0x1500eb58; continue; }
  pc = 0x1500ea61; continue;
 }
 0x1500ea61 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500ea78; continue; }
  pc = 0x1500ea66; continue;
 }
 0x1500ea66 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ea78; continue;
 }
 0x1500ea78 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1508fde8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x74u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500eab4; continue; }
  pc = 0x1500ea8a; continue;
 }
 0x1500ea8a => {
  { let v: u8 = cpu.r8(0x150900f0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ea94; continue;
 }
 0x1500ea94 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x3au32); cpu.eax = v; }
  if cpu.zf { pc = 0x1500ea9f; continue; }
  pc = 0x1500ea99; continue;
 }
 0x1500ea99 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 14u32); cpu.eax = v; }
  pc = 0x1500ea9f; continue;
 }
 0x1500ea9f => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1508ee40u32)); }
  if !cpu.zf { pc = 0x1500ea61; continue; }
  pc = 0x1500eaaf; continue;
 }
 0x1500eaaf => {
  pc = 0x1500eb58; continue;
 }
 0x1500eab4 => {
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf == cpu.of) { pc = 0x1500eb09; continue; }
  pc = 0x1500eabb; continue;
 }
 0x1500eabb => {
  { let v: u8 = cpu.r8(0x150900f0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500eac5; continue;
 }
 0x1500eac5 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), (cpu.eax as u8)); }
  if (cpu.sf == cpu.of) { pc = 0x1500eade; continue; }
  pc = 0x1500eacc; continue;
 }
 0x1500eacc => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500eade; continue;
 }
 0x1500eade => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 14u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x1508ee40u32)); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { pc = 0x1500ea61; continue; }
  pc = 0x1500eb07; continue;
 }
 0x1500eb07 => {
  pc = 0x1500eb58; continue;
 }
 0x1500eb09 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 8u32); cpu.ebp = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500eb26; continue; }
  pc = 0x1500eb14; continue;
 }
 0x1500eb14 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500eb26; continue;
 }
 0x1500eb26 => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1500eb3b; continue; }
  pc = 0x1500eb2c; continue;
 }
 0x1500eb2c => {
  { let v: u8 = cpu.r8(0x1508ff6bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf != cpu.of) { pc = 0x1500eb4a; continue; }
  pc = 0x1500eb39; continue;
 }
 0x1500eb39 => {
  pc = 0x1500eb26; continue;
 }
 0x1500eb3b => {
  { let v: u8 = cpu.r8(0x1508fe67u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf != cpu.of) { pc = 0x1500eb4a; continue; }
  pc = 0x1500eb48; continue;
 }
 0x1500eb48 => {
  pc = 0x1500eb26; continue;
 }
 0x1500eb4a => {
  { cpu.sub32(cpu.eax, 0x74u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ea94; continue; }
  pc = 0x1500eb53; continue;
 }
 0x1500eb53 => {
  pc = 0x1500eac5; continue;
 }
 0x1500eb58 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1500eb6b; continue; }
  pc = 0x1500eb5d; continue;
 }
 0x1500eb5d => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x1500eb5f; continue;
 }
 0x1500eb5f => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x1500eb5f; continue; }
  pc = 0x1500eb6b; continue;
 }
 0x1500eb6b => {
  { let v: u32 = cpu.r32(0x1508ee40u32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500eb71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500eb71); }
 let mut pc: u32 = 0x1500eb71;
 loop { match pc {
 0x1500eb71 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0xcu8); }
  if (cpu.sf == cpu.of) { pc = 0x1500eb88; continue; }
  pc = 0x1500eb76; continue;
 }
 0x1500eb76 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500eb88; continue;
 }
 0x1500eb88 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0xcu8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 12u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500eb98; continue;
 }
 0x1500eb98 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500ebaf; continue; }
  pc = 0x1500eb9d; continue;
 }
 0x1500eb9d => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ebaf; continue;
 }
 0x1500ebaf => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1509018cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x150902d4u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ebc6; continue;
 }
 0x1500ebc6 => {
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500ebdc; continue; }
  pc = 0x1500ebca; continue;
 }
 0x1500ebca => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x1500ec1b; continue; }
  pc = 0x1500ebce; continue;
 }
 0x1500ebce => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.and8(((cpu.edx >> 8) as u8), 0xfu8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1500eb98; continue; }
  pc = 0x1500ebda; continue;
 }
 0x1500ebda => {
  pc = 0x1500ec50; continue;
 }
 0x1500ebdc => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 8u32); cpu.ebp = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500ebf9; continue; }
  pc = 0x1500ebe7; continue;
 }
 0x1500ebe7 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ebf9; continue;
 }
 0x1500ebf9 => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1500ec0c; continue; }
  pc = 0x1500ebff; continue;
 }
 0x1500ebff => {
  { let v: u8 = cpu.r8(0x1509029fu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x11u32); }
  if (cpu.sf != cpu.of) { pc = 0x1500ec19; continue; }
  pc = 0x1500ec0a; continue;
 }
 0x1500ec0a => {
  pc = 0x1500ebf9; continue;
 }
 0x1500ec0c => {
  { let v: u8 = cpu.r8(0x1509027bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x11u32); }
  if (cpu.sf != cpu.of) { pc = 0x1500ec19; continue; }
  pc = 0x1500ec17; continue;
 }
 0x1500ec17 => {
  pc = 0x1500ebf9; continue;
 }
 0x1500ec19 => {
  pc = 0x1500ebc6; continue;
 }
 0x1500ec1b => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x5u8); }
  if (cpu.sf == cpu.of) { pc = 0x1500ec32; continue; }
  pc = 0x1500ec20; continue;
 }
 0x1500ec20 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ec32; continue;
 }
 0x1500ec32 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x5u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 5u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 27u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1500ec41; continue;
 }
 0x1500ec41 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1500ec41; continue; }
  pc = 0x1500ec48; continue;
 }
 0x1500ec48 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1500eb98; continue; }
  pc = 0x1500ec50; continue;
 }
 0x1500ec50 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ec52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ec52); }
 let mut pc: u32 = 0x1500ec52;
 loop { match pc {
 0x1500ec52 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xc0u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15090174u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x15090184u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x15090178u32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1509017cu32, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x15090180u32, v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x15090174u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x1500ec90; continue;
 }
 0x1500ec90 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500edd2; continue; }
  pc = 0x1500ec9a; continue;
 }
 0x1500ec9a => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1500eca1; continue; }
  pc = 0x1500ec9e; continue;
 }
 0x1500ec9e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  pc = 0x1500eca1; continue;
 }
 0x1500eca1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500ec90; continue; }
  pc = 0x1500eca5; continue;
 }
 0x1500eca5 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x15090174u32)); cpu.esi = v; }
  { let v: u32 = 0x44u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x1500ecb9; continue;
 }
 0x1500ecb9 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500ecb9; continue; }
  pc = 0x1500ecc8; continue;
 }
 0x1500ecc8 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1509017cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15090174u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { cpu.stos(2, true); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.r32(0x15090180u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15090174u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { cpu.stos(2, true); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.r32(0x15090174u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x15090188u32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x1500ed04; continue;
 }
 0x1500ed04 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1500edba; continue; }
  pc = 0x1500ed0e; continue;
 }
 0x1500ed0e => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x15090184u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500ed51; continue; }
  pc = 0x1500ed16; continue;
 }
 0x1500ed16 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.w32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x15090184u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edi, (cpu.ecx & 0xff)); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x15090178u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x1500ed47; continue; }
  pc = 0x1500ed43; continue;
 }
 0x1500ed43 => {
  { cpu.stos(1, false); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ed4f; continue; }
  pc = 0x1500ed47; continue;
 }
 0x1500ed47 => {
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(2, true); }
  if !cpu.cf { pc = 0x1500ed4f; continue; }
  pc = 0x1500ed4e; continue;
 }
 0x1500ed4e => {
  { cpu.stos(1, false); }
  pc = 0x1500ed4f; continue;
 }
 0x1500ed4f => {
  pc = 0x1500edba; continue;
 }
 0x1500ed51 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.w32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x15090184u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x15090178u32)); cpu.edi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1500edb8; continue; }
  pc = 0x1500ed77; continue;
 }
 0x1500ed77 => {
  { let v: u32 = cpu.r32(0x15090188u32); cpu.ecx = v; }
  pc = 0x1500ed7d; continue;
 }
 0x1500ed7d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x1500ed99; continue; }
  pc = 0x1500ed82; continue;
 }
 0x1500ed82 => {
  { let v: u32 = cpu.r32(0x1509017cu32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x15090180u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1500ed99; continue;
 }
 0x1500ed99 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x15090180u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x1500edad; continue; }
  pc = 0x1500eda7; continue;
 }
 0x1500eda7 => {
  { let v: u32 = cpu.r32(0x1509017cu32); cpu.edi = v; }
  pc = 0x1500edad; continue;
 }
 0x1500edad => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { pc = 0x1500ed7d; continue; }
  pc = 0x1500edb2; continue;
 }
 0x1500edb2 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x15090188u32, v); }
  pc = 0x1500edb8; continue;
 }
 0x1500edb8 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1500edba; continue;
 }
 0x1500edba => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x15090174u32)); }
  if !cpu.zf { pc = 0x1500ed04; continue; }
  pc = 0x1500edc8; continue;
 }
 0x1500edc8 => {
  { let v: u32 = cpu.esp.wrapping_add(0xc0u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1500edd2 => {
  { let v: u32 = cpu.esp.wrapping_add(0xc0u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500eddb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500eddb); }
 let mut pc: u32 = 0x1500eddb;
 loop { match pc {
 0x1500eddb => {
  { cpu.pushad(); }
  { let v: u32 = 0x150167d4u32; cpu.esi = v; }
  { let v: u32 = 0x150902d4u32; cpu.edi = v; }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x11u32; cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x1509018cu32; cpu.ecx = v; }
  { let v: u32 = 0x1509028cu32; cpu.edx = v; }
  { let v: u32 = 0x150902b0u32; cpu.ebp = v; }
  { let v: u32 = 0x150902d4u32; cpu.esi = v; }
  cpu.push(0x1500ee10); f_1500ec52(cpu);
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ee12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ee12); }
 let mut pc: u32 = 0x1500ee12;
 loop { match pc {
 0x1500ee12 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.eax, 0x32u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500ee2e; continue; }
  pc = 0x1500ee18; continue;
 }
 0x1500ee18 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500ee21; continue; }
  pc = 0x1500ee1c; continue;
 }
 0x1500ee1c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1500ee21; continue;
 }
 0x1500ee21 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x1388u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  pc = 0x1500ee35; continue;
 }
 0x1500ee2e => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x1500ee35; continue;
 }
 0x1500ee35 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x1500ee39; continue;
 }
 0x1500ee39 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x32u32); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1500ee53; continue; }
  pc = 0x1500ee4e; continue;
 }
 0x1500ee4e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1500ee53; continue;
 }
 0x1500ee53 => {
  { cpu.sub32(cpu.eax, 0xffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1500ee5f; continue; }
  pc = 0x1500ee5a; continue;
 }
 0x1500ee5a => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x1500ee5f; continue;
 }
 0x1500ee5f => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x150167e8u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x150168ecu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(8)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if !cpu.zf { pc = 0x1500ee39; continue; }
  pc = 0x1500ee78; continue;
 }
 0x1500ee78 => {
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500ee7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500ee7a); }
 let mut pc: u32 = 0x1500ee7a;
 loop { match pc {
 0x1500ee7a => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x150906e8u32, v); }
  pc = 0x1500ee83; continue;
 }
 0x1500ee83 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x40u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500eef2; continue; }
  pc = 0x1500ee8b; continue;
 }
 0x1500ee8b => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x60u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500eef2; continue; }
  pc = 0x1500ee90; continue;
 }
 0x1500ee90 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x80u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500eef2; continue; }
  pc = 0x1500ee98; continue;
 }
 0x1500ee98 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0xa0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500eef2; continue; }
  pc = 0x1500eea0; continue;
 }
 0x1500eea0 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0xc0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500eef2; continue; }
  pc = 0x1500eea8; continue;
 }
 0x1500eea8 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0xe0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500eef2; continue; }
  pc = 0x1500eeb0; continue;
 }
 0x1500eeb0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x600u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x620u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x640u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x660u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x680u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x6a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x6c0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x6e0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x150906e8u32)); }
  if !cpu.zf { pc = 0x1500ee83; continue; }
  pc = 0x1500eeed; continue;
 }
 0x1500eeed => {
  pc = 0x1500f00d; continue;
 }
 0x1500eef2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x150906ecu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x150906f0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x150906f4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x150906f8u32, v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xe0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0x1d9u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x115u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0xfffffd63u32); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x150906ecu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x150906ecu32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x600u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x6e0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x150906f0u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x150906f0u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x620u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x6c0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x150906f4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x150906f4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x640u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x6a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x150906f8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x150906f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x680u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x660u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x150906e8u32)); }
  if !cpu.zf { pc = 0x1500ee83; continue; }
  pc = 0x1500f00d; continue;
 }
 0x1500f00d => {
  { let v: u32 = cpu.esi.wrapping_sub(0xfffffa20u32); cpu.esi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x150906e8u32, v); }
  pc = 0x1500f01f; continue;
 }
 0x1500f01f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x150906ecu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x150906f0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x150906f4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x150906f8u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0x1d9u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x115u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0xfffffd63u32); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x150906ecu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x150906ecu32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x150906f0u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x150906f0u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x150906f4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x150906f4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x150906f8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x150906f8u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x150906e8u32)); }
  if !cpu.zf { pc = 0x1500f01f; continue; }
  pc = 0x1500f12d; continue;
 }
 0x1500f12d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f12e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f12e); }
 let mut pc: u32 = 0x1500f12e;
 loop { match pc {
 0x1500f12e => {
  { let v: u32 = cpu.edi; cpu.w32(0x15090700u32, v); }
  { let v: u32 = 0x1508f45cu32; cpu.w32(0x150906fcu32, v); }
  { let v: u32 = 0x1508f65cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15090700u32); cpu.edi = v; }
  cpu.push(0x1500f14e); f_1500f18e(cpu);
  { let v: u32 = 0x1508f4dcu32; cpu.w32(0x150906fcu32, v); }
  { let v: u32 = 0x1508f85cu32; cpu.esi = v; }
  cpu.push(0x1500f162); f_1500f18e(cpu);
  { let v: u32 = 0x1508f46cu32; cpu.w32(0x150906fcu32, v); }
  { let v: u32 = 0x1508f75cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15090700u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  cpu.push(0x1500f17f); f_1500f18e(cpu);
  { let v: u32 = 0x1508f4ecu32; cpu.w32(0x150906fcu32, v); }
  { let v: u32 = 0x1508f95cu32; cpu.esi = v; }
  f_1500f18e(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f18e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f18e); }
 let mut pc: u32 = 0x1500f18e;
 loop { match pc {
 0x1500f18e => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15090704u32, v); }
  pc = 0x1500f199; continue;
 }
 0x1500f199 => {
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x200u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x204u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f3ca => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f218(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f218); }
 let mut pc: u32 = 0x1500f218;
 loop { match pc {
 0x1500f199 => {
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x200u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x204u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f218 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x204u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f3ca => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f29e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f29e); }
 let mut pc: u32 = 0x1500f29e;
 loop { match pc {
 0x1500f199 => {
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x200u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x204u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f29e => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f3ca => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f324(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f324); }
 let mut pc: u32 = 0x1500f324;
 loop { match pc {
 0x1500f199 => {
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x200u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x204u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f324 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f3ca => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f3aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f3aa); }
 let mut pc: u32 = 0x1500f3aa;
 loop { match pc {
 0x1500f199 => {
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x200u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x204u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x150906fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f3aa => {
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x150906fcu32).wrapping_add(0x20u32); cpu.w32(0x150906fcu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167ccu32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090704u32)); }
  if !cpu.zf { pc = 0x1500f199; continue; }
  pc = 0x1500f3ca; continue;
 }
 0x1500f3ca => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f3f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f3f0); }
 let mut pc: u32 = 0x1500f3f0;
 loop { match pc {
 0x1500f3f0 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x15090708u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1509070cu32, v); }
  pc = 0x1500f401; continue;
 }
 0x1500f401 => {
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x204u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x200u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x208u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x214u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x210u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x21cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x15090708u32).wrapping_add(0x20u32); cpu.w32(0x15090708u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167c8u32)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x15090708u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x1509070cu32)); }
  if !cpu.zf { pc = 0x1500f401; continue; }
  pc = 0x1500f66e; continue;
 }
 0x1500f66e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f66f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f66f); }
 let mut pc: u32 = 0x1500f66f;
 loop { match pc {
 0x1500f66f => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15090710u32, v); }
  pc = 0x1500f67a; continue;
 }
 0x1500f67a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x200u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x104u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x204u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x120u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x220u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x24u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x124u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x224u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x208u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10cu32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x20cu32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x28u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x128u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x228u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x12cu32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x22cu32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x210u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x114u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x214u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x30u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x130u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x230u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x34u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x134u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x234u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11cu32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x21cu32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x38u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x138u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x238u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3cu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x13cu32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x23cu32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x15093114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15094114u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x15092114u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x15093914u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15092914u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x150167c8u32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x15090710u32)); }
  if !cpu.zf { pc = 0x1500f67a; continue; }
  pc = 0x1500f861; continue;
 }
 0x1500f861 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f862(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f862); }
 let mut pc: u32 = 0x1500f862;
 loop { match pc {
 0x1500f862 => {
  { cpu.pushad(); }
  { let v: u32 = 0x15016828u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x1500f86f; continue;
 }
 0x1500f86f => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500f86f; continue; }
  pc = 0x1500f886; continue;
 }
 0x1500f886 => {
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f888(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f888); }
 let mut pc: u32 = 0x1500f888;
 loop { match pc {
 0x1500f888 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0x700u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 8u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x15091914u32; cpu.edi = v; }
  cpu.push(0x1500f8ae); f_1500f8cf(cpu);
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = 0x15091114u32; cpu.edi = v; }
  cpu.push(0x1500f8be); f_1500f8cf(cpu);
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x15090914u32; cpu.edi = v; }
  cpu.push(0x1500f8cc); f_1500f8cf(cpu);
  { cpu.popad(); }
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f8cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f8cf); }
 let mut pc: u32 = 0x1500f8cf;
 loop { match pc {
 0x1500f8cf => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x200u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1500f8ec; continue;
 }
 0x1500f8ec => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x150169ecu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.imul32(v); }
  { let v: u32 = 0xffu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x100u32); }
  if !cpu.zf { pc = 0x1500f8ec; continue; }
  pc = 0x1500f909; continue;
 }
 0x1500f909 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f911(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f911); }
 let mut pc: u32 = 0x1500f911;
 loop { match pc {
 0x1500f911 => {
  { cpu.pushad(); }
  { let v: u32 = 0x100u32; cpu.w32(0x15094724u32, v); }
  { let v = (cpu.r32(0x15094724u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0x15094714u32, v); cpu.fpop(); }
  { let v: u32 = 0x100u32; cpu.w32(0x15094724u32, v); }
  { let v = (cpu.r32(0x15094724u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0x1509471cu32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v: u32 = 0x15092114u32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  cpu.push(0x1500f94e); f_1500f9b5(cpu);
  { let v = cpu.rf64(0x15016aecu32); cpu.fpush(v); }
  { let v: u32 = 0x15092914u32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = 0x15090914u32; cpu.esi = v; }
  cpu.push(0x1500f968); f_1500f9b5(cpu);
  { let v = cpu.rf64(0x15016af4u32); cpu.fpush(v); }
  { let v: u32 = 0x15093114u32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = 0x15091114u32; cpu.esi = v; }
  cpu.push(0x1500f982); f_1500f9b5(cpu);
  { let v = cpu.rf64(0x15016afcu32); cpu.fpush(v); }
  { let v: u32 = 0x15093914u32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x1500f999); f_1500f9b5(cpu);
  { let v = cpu.rf64(0x15016b04u32); cpu.fpush(v); }
  { let v: u32 = 0x15094114u32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = 0x15091914u32; cpu.esi = v; }
  cpu.push(0x1500f9b3); f_1500f9b5(cpu);
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500f9b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500f9b5); }
 let mut pc: u32 = 0x1500f9b5;
 loop { match pc {
 0x1500f9b5 => {
  { let v = cpu.st(0) * cpu.rf64(0x15094714u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) / cpu.rf64(0x1509471cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x15094724u32, v); }
  { let v = (cpu.r32(0x15094724u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x1500f9d7; continue;
 }
 0x1500f9d7 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 2u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500f9d7; continue; }
  pc = 0x1500f9e4; continue;
 }
 0x1500f9e4 => {
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  pc = 0x1500f9e9; continue;
 }
 0x1500f9e9 => {
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 2u32); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x1500f9e9; continue; }
  pc = 0x1500f9f8; continue;
 }
 0x1500f9f8 => {
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x1500f9ff; continue;
 }
 0x1500f9ff => {
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 2u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.ebx, 0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x1500f9ff; continue; }
  pc = 0x1500fa0c; continue;
 }
 0x1500fa0c => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fa11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fa11); }
 let mut pc: u32 = 0x1500fa11;
 loop { match pc {
 0x1500fa11 => {
  { cpu.pushad(); }
  cpu.push(0x1500fa17); f_1500f888(cpu);
  cpu.push(0x1500fa1c); f_1500f911(cpu);
  { let v: u32 = 0x150902e8u32; cpu.edi = v; }
  cpu.push(0x1500fa26); f_1500f862(cpu);
  { let v: u32 = 0x150904e8u32; cpu.edi = v; }
  cpu.push(0x1500fa30); f_1500f862(cpu);
  { let v: u32 = 0x150904e8u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1508ee44u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1508ee48u32, v); }
  { let v: u32 = 0x150902e8u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1508ee4cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1508ee50u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1508ee54u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1508ee58u32, v); }
  cpu.push(0x1500fa63); f_1500eddb(cpu);
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1508ee5cu32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1508ef5cu32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1508f05cu32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1508f15cu32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1508f25cu32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x1508f35cu32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.popad(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fab0); }
 let mut pc: u32 = 0x1500fab0;
 loop { match pc {
 0x1500fab0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x1500fab5; continue;
 }
 0x1500fab5 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1500fabe; continue; }
  pc = 0x1500fab9; continue;
 }
 0x1500fab9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1500fab5; continue;
 }
 0x1500fabe => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fabf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fabf); }
 let mut pc: u32 = 0x1500fabf;
 loop { match pc {
 0x1500fabf => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let a = cpu.r32(0x70u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fac7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fac7); }
 let mut pc: u32 = 0x1500fac7;
 loop { match pc {
 0x1500fac7 => {
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x1500faca; continue;
 }
 0x1500faca => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1500fad8; continue; }
  pc = 0x1500face; continue;
 }
 0x1500face => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.zf { pc = 0x1500fad6; continue; }
  pc = 0x1500fad2; continue;
 }
 0x1500fad2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x1500faca; continue;
 }
 0x1500fad6 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1500fad8; continue;
 }
 0x1500fad8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fad9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fad9); }
 let mut pc: u32 = 0x1500fad9;
 loop { match pc {
 0x1500fad9 => {
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x1500fadc; continue;
 }
 0x1500fadc => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1500faec; continue; }
  pc = 0x1500fae0; continue;
 }
 0x1500fae0 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); }
  if cpu.zf { pc = 0x1500fae8; continue; }
  pc = 0x1500fae4; continue;
 }
 0x1500fae4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x1500fadc; continue;
 }
 0x1500fae8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1500faec; continue;
 }
 0x1500faec => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb58); }
 let mut pc: u32 = 0x1500fb58;
 loop { match pc {
 0x1500fb58 => {
  pc = cpu.r32(0x15096124u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb5e); }
 let mut pc: u32 = 0x1500fb5e;
 loop { match pc {
 0x1500fb5e => {
  pc = cpu.r32(0x15096134u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb64); }
 let mut pc: u32 = 0x1500fb64;
 loop { match pc {
 0x1500fb64 => {
  pc = cpu.r32(0x15096148u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb6a); }
 let mut pc: u32 = 0x1500fb6a;
 loop { match pc {
 0x1500fb6a => {
  pc = cpu.r32(0x15096170u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb70); }
 let mut pc: u32 = 0x1500fb70;
 loop { match pc {
 0x1500fb70 => {
  pc = cpu.r32(0x1509618cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb76); }
 let mut pc: u32 = 0x1500fb76;
 loop { match pc {
 0x1500fb76 => {
  pc = cpu.r32(0x15096188u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb7c); }
 let mut pc: u32 = 0x1500fb7c;
 loop { match pc {
 0x1500fb7c => {
  pc = cpu.r32(0x1509612cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb82); }
 let mut pc: u32 = 0x1500fb82;
 loop { match pc {
 0x1500fb82 => {
  pc = cpu.r32(0x15096120u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb88); }
 let mut pc: u32 = 0x1500fb88;
 loop { match pc {
 0x1500fb88 => {
  pc = cpu.r32(0x15096114u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb8e); }
 let mut pc: u32 = 0x1500fb8e;
 loop { match pc {
 0x1500fb8e => {
  pc = cpu.r32(0x1509613cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb94); }
 let mut pc: u32 = 0x1500fb94;
 loop { match pc {
 0x1500fb94 => {
  pc = cpu.r32(0x1509616cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fb9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fb9a); }
 let mut pc: u32 = 0x1500fb9a;
 loop { match pc {
 0x1500fb9a => {
  pc = cpu.r32(0x15096144u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fba0); }
 let mut pc: u32 = 0x1500fba0;
 loop { match pc {
 0x1500fba0 => {
  pc = cpu.r32(0x15096118u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fba6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fba6); }
 let mut pc: u32 = 0x1500fba6;
 loop { match pc {
 0x1500fba6 => {
  pc = cpu.r32(0x15096194u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbac); }
 let mut pc: u32 = 0x1500fbac;
 loop { match pc {
 0x1500fbac => {
  pc = cpu.r32(0x15096154u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbb2); }
 let mut pc: u32 = 0x1500fbb2;
 loop { match pc {
 0x1500fbb2 => {
  pc = cpu.r32(0x15096160u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbb8); }
 let mut pc: u32 = 0x1500fbb8;
 loop { match pc {
 0x1500fbb8 => {
  pc = cpu.r32(0x15096074u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbbe); }
 let mut pc: u32 = 0x1500fbbe;
 loop { match pc {
 0x1500fbbe => {
  pc = cpu.r32(0x1509607cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbc4); }
 let mut pc: u32 = 0x1500fbc4;
 loop { match pc {
 0x1500fbc4 => {
  pc = cpu.r32(0x15096168u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbca); }
 let mut pc: u32 = 0x1500fbca;
 loop { match pc {
 0x1500fbca => {
  pc = cpu.r32(0x15096158u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbdc); }
 let mut pc: u32 = 0x1500fbdc;
 loop { match pc {
 0x1500fbdc => {
  pc = cpu.r32(0x1509615cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbe2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbe2); }
 let mut pc: u32 = 0x1500fbe2;
 loop { match pc {
 0x1500fbe2 => {
  pc = cpu.r32(0x15096130u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbe8); }
 let mut pc: u32 = 0x1500fbe8;
 loop { match pc {
 0x1500fbe8 => {
  pc = cpu.r32(0x15096140u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbee); }
 let mut pc: u32 = 0x1500fbee;
 loop { match pc {
 0x1500fbee => {
  pc = cpu.r32(0x15096128u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbf4); }
 let mut pc: u32 = 0x1500fbf4;
 loop { match pc {
 0x1500fbf4 => {
  pc = cpu.r32(0x15096078u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fbfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fbfa); }
 let mut pc: u32 = 0x1500fbfa;
 loop { match pc {
 0x1500fbfa => {
  pc = cpu.r32(0x1509614cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc00); }
 let mut pc: u32 = 0x1500fc00;
 loop { match pc {
 0x1500fc00 => {
  pc = cpu.r32(0x1509617cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc06); }
 let mut pc: u32 = 0x1500fc06;
 loop { match pc {
 0x1500fc06 => {
  pc = cpu.r32(0x15096184u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc0c); }
 let mut pc: u32 = 0x1500fc0c;
 loop { match pc {
 0x1500fc0c => {
  pc = cpu.r32(0x15096178u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc12); }
 let mut pc: u32 = 0x1500fc12;
 loop { match pc {
 0x1500fc12 => {
  pc = cpu.r32(0x15096180u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc18); }
 let mut pc: u32 = 0x1500fc18;
 loop { match pc {
 0x1500fc18 => {
  pc = cpu.r32(0x15096164u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc1e); }
 let mut pc: u32 = 0x1500fc1e;
 loop { match pc {
 0x1500fc1e => {
  pc = cpu.r32(0x15096190u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc24); }
 let mut pc: u32 = 0x1500fc24;
 loop { match pc {
 0x1500fc24 => {
  pc = cpu.r32(0x1509611cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc2a); }
 let mut pc: u32 = 0x1500fc2a;
 loop { match pc {
 0x1500fc2a => {
  pc = cpu.r32(0x15096138u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fc30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fc30); }
 let mut pc: u32 = 0x1500fc30;
 loop { match pc {
 0x1500fc30 => {
  pc = cpu.r32(0x15096110u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fea0); }
 let mut pc: u32 = 0x1500fea0;
 loop { match pc {
 0x1500fea0 => {
  pc = cpu.r32(0x1509655cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fea6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fea6); }
 let mut pc: u32 = 0x1500fea6;
 loop { match pc {
 0x1500fea6 => {
  pc = cpu.r32(0x15096434u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500feac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500feac); }
 let mut pc: u32 = 0x1500feac;
 loop { match pc {
 0x1500feac => {
  pc = cpu.r32(0x15096420u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500feb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500feb2); }
 let mut pc: u32 = 0x1500feb2;
 loop { match pc {
 0x1500feb2 => {
  pc = cpu.r32(0x1509641cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500feb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500feb8); }
 let mut pc: u32 = 0x1500feb8;
 loop { match pc {
 0x1500feb8 => {
  pc = cpu.r32(0x150963f8u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500febe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500febe); }
 let mut pc: u32 = 0x1500febe;
 loop { match pc {
 0x1500febe => {
  pc = cpu.r32(0x15096428u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fec4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fec4); }
 let mut pc: u32 = 0x1500fec4;
 loop { match pc {
 0x1500fec4 => {
  pc = cpu.r32(0x15096528u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500feca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500feca); }
 let mut pc: u32 = 0x1500feca;
 loop { match pc {
 0x1500feca => {
  pc = cpu.r32(0x1509647cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fed0); }
 let mut pc: u32 = 0x1500fed0;
 loop { match pc {
 0x1500fed0 => {
  pc = cpu.r32(0x15096598u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fed6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fed6); }
 let mut pc: u32 = 0x1500fed6;
 loop { match pc {
 0x1500fed6 => {
  pc = cpu.r32(0x15096474u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fee2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fee2); }
 let mut pc: u32 = 0x1500fee2;
 loop { match pc {
 0x1500fee2 => {
  pc = cpu.r32(0x15096424u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1500fee8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1500fee8); }
 let mut pc: u32 = 0x1500fee8;
 loop { match pc {
 0x1500fee8 => {
  pc = cpu.r32(0x150964d4u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
