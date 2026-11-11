#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_00482f95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482f95); }
 let mut pc: u32 = 0x482f95;
 loop { match pc {
 0x482f95 => {
  { let v: u32 = 0xffffffa8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482f9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482f9e); }
 let mut pc: u32 = 0x482f9e;
 loop { match pc {
 0x482f9e => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482fa7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482fa7); }
 let mut pc: u32 = 0x482fa7;
 loop { match pc {
 0x482fa7 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00482fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x482fb0); }
 let mut pc: u32 = 0x482fb0;
 loop { match pc {
 0x482fb0 => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483079(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483079); }
 let mut pc: u32 = 0x483079;
 loop { match pc {
 0x483079 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x5256f0u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483082(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483082); }
 let mut pc: u32 = 0x483082;
 loop { match pc {
 0x483082 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048308b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48308b); }
 let mut pc: u32 = 0x48308b;
 loop { match pc {
 0x48308b => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483094(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483094); }
 let mut pc: u32 = 0x483094;
 loop { match pc {
 0x483094 => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048315f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48315f); }
 let mut pc: u32 = 0x48315f;
 loop { match pc {
 0x48315f => {
  { let v: u32 = 0xffffffb8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x5256f0u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483168(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483168); }
 let mut pc: u32 = 0x483168;
 loop { match pc {
 0x483168 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483171(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483171); }
 let mut pc: u32 = 0x483171;
 loop { match pc {
 0x483171 => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048317a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48317a); }
 let mut pc: u32 = 0x48317a;
 loop { match pc {
 0x48317a => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  pc = cpu.r32(0x52571cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004831aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4831aa); }
 let mut pc: u32 = 0x4831aa;
 loop { match pc {
 0x4831aa => {
  { let v: u32 = 0x4b8608u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004831b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4831b4); }
 let mut pc: u32 = 0x4831b4;
 loop { match pc {
 0x482e02 => {
  { let v: u32 = 0x4b178cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4831b4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x482e02; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004831bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4831bc); }
 let mut pc: u32 = 0x4831bc;
 loop { match pc {
 0x4831bc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x4831c4); f_004831da(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x4831d4; continue; }
  pc = 0x4831cb; continue;
 }
 0x4831cb => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4831d1); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4831d4; continue;
 }
 0x4831d4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004831da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4831da); }
 let mut pc: u32 = 0x4831da;
 loop { match pc {
 0x4831da => {
  { let v: u32 = 0x483220u32; cpu.eax = v; }
  cpu.push(0x4831e4); f_004aea58(cpu);
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x4b17a4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x483207; continue; }
  pc = 0x4831fe; continue;
 }
 0x4831fe => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x483204); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x483207; continue;
 }
 0x483207 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483213); f_0048322a(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483220); }
 let mut pc: u32 = 0x483220;
 loop { match pc {
 0x483220 => {
  { let v: u32 = 0x4b86a0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048322a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48322a); }
 let mut pc: u32 = 0x48322a;
 loop { match pc {
 0x482e02 => {
  { let v: u32 = 0x4b178cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48322a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x482e02; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483232(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483232); }
 let mut pc: u32 = 0x483232;
 loop { match pc {
 0x483232 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048323a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48323a); }
 let mut pc: u32 = 0x48323a;
 loop { match pc {
 0x48323a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ecx)), cpu.eax); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483244); }
 let mut pc: u32 = 0x483244;
 loop { match pc {
 0x483244 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x483277; continue; }
  pc = 0x48325c; continue;
 }
 0x48325c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x48326c); f_004aea64(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0xcu32.wrapping_add(cpu.edi)), cpu.esi); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x483277; continue;
 }
 0x483277 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483280); }
 let mut pc: u32 = 0x483280;
 loop { match pc {
 0x483280 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x48328b); f_0048329a(cpu);
  cpu.push(0x483290); f_004832bc(cpu);
  pc = 0x483295; continue;
 }
 0x483295 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048329a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48329a); }
 let mut pc: u32 = 0x48329a;
 loop { match pc {
 0x48329a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f68f4u32; cpu.push(v); }
  { let v: u32 = 0x524080u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4832b2); cpu.call(t);
  pc = 0x4832b7; continue;
 }
 0x4832b7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004832bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4832bc); }
 let mut pc: u32 = 0x4832bc;
 loop { match pc {
 0x4832bc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4832d9u32; cpu.push(v); }
  cpu.push(0x4832cc); f_004ae7f0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4832d4; continue;
 }
 0x4832d4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004832d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4832d9); }
 let mut pc: u32 = 0x4832d9;
 loop { match pc {
 0x4832d9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x524080u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4832ea); cpu.call(t);
  pc = 0x4832ef; continue;
 }
 0x4832ef => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004832f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4832f4); }
 let mut pc: u32 = 0x4832f4;
 loop { match pc {
 0x4832f4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4832ff); f_0048330e(cpu);
  cpu.push(0x483304); f_004833bd(cpu);
  pc = 0x483309; continue;
 }
 0x483309 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048330e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48330e); }
 let mut pc: u32 = 0x48330e;
 loop { match pc {
 0x48330e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f68fcu32; cpu.push(v); }
  { let v: u32 = 0x524048u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x483326); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f6904u32; cpu.push(v); }
  { let v: u32 = 0x524050u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x483339); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f690cu32; cpu.push(v); }
  { let v: u32 = 0x524058u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x48334c); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f6918u32; cpu.push(v); }
  { let v: u32 = 0x524060u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x48335f); cpu.call(t);
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f6924u32; cpu.push(v); }
  { let v: u32 = 0x524068u32; cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x483372); cpu.call(t);
  { let v: u32 = 0x524048u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x28u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x52571cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5256ecu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v: u32 = 0x524048u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x28u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4833b3); f_004aea70(cpu);
  pc = 0x4833b8; continue;
 }
 0x4833b8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004833bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4833bd); }
 let mut pc: u32 = 0x4833bd;
 loop { match pc {
 0x4833bd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x4833dau32; cpu.push(v); }
  cpu.push(0x4833cd); f_004ae7f0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4833d5; continue;
 }
 0x4833d5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004833da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4833da); }
 let mut pc: u32 = 0x4833da;
 loop { match pc {
 0x4833da => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x52571cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x524048u32; cpu.push(v); }
  cpu.push(0x4833f4); f_004aeb10(cpu);
  pc = 0x4833f9; continue;
 }
 0x4833f9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004833fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4833fe); }
 let mut pc: u32 = 0x4833fe;
 loop { match pc {
 0x4833fe => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x483411; continue;
 }
 0x483411 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483418(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483418); }
 let mut pc: u32 = 0x483418;
 loop { match pc {
 0x483418 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x48342b; continue;
 }
 0x48342b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483432(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483432); }
 let mut pc: u32 = 0x483432;
 loop { match pc {
 0x483432 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x483445; continue;
 }
 0x483445 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048344c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48344c); }
 let mut pc: u32 = 0x48344c;
 loop { match pc {
 0x48344c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48345d; continue;
 }
 0x48345d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483464(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483464); }
 let mut pc: u32 = 0x483464;
 loop { match pc {
 0x483464 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4834c5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x483495); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4834a9; continue;
 }
 0x4834a9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4834cf; continue;
 }
 0x4834cf => {
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
pub fn f_004834b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4834b8); }
 let mut pc: u32 = 0x4834b8;
 loop { match pc {
 0x4834b8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4834c4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004834c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4834c5); }
 let mut pc: u32 = 0x4834c5;
 loop { match pc {
 0x4834c5 => {
  { let v: u32 = 0x4b86c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004834e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4834e0); }
 let mut pc: u32 = 0x4834e0;
 loop { match pc {
 0x4834e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x4834f6; continue;
 }
 0x4834f6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004834fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4834fb); }
 let mut pc: u32 = 0x4834fb;
 loop { match pc {
 0x4834fb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x483512; continue;
 }
 0x483512 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483517(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483517); }
 let mut pc: u32 = 0x483517;
 loop { match pc {
 0x483517 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48352b); f_004834e0(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x483536); f_004834e0(cpu);
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x483553; continue; }
  pc = 0x483549; continue;
 }
 0x483549 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x483555; continue;
 }
 0x483553 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x483555; continue;
 }
 0x483555 => {
  pc = 0x48355a; continue;
 }
 0x48355a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483561(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483561); }
 let mut pc: u32 = 0x483561;
 loop { match pc {
 0x483561 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x483672u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x483589); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x483631; continue; }
  pc = 0x4835a0; continue;
 }
 0x4835a0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b17f4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4835b4; continue;
 }
 0x4835b4 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4835c4); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b17d4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4835d7; continue;
 }
 0x4835d7 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4835f6; continue;
 }
 0x4835f6 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b17e4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x483608; continue;
 }
 0x483608 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48362c; continue;
 }
 0x48362c => {
  pc = 0x483638; continue;
 }
 0x483631 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x483638; continue;
 }
 0x483638 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5257a4u32); cpu.push(0x483658); cpu.call(t);
  pc = 0x48365d; continue;
 }
 0x48365d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48367c; continue;
 }
 0x48367c => {
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
pub fn f_004835e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4835e9); }
 let mut pc: u32 = 0x4835e9;
 loop { match pc {
 0x4835e9 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4835f5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483617(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483617); }
 let mut pc: u32 = 0x483617;
 loop { match pc {
 0x483617 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  cpu.push(0x483622); f_00485560(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483623(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483623); }
 let mut pc: u32 = 0x483623;
 loop { match pc {
 0x483623 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48362b); f_004851f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483665(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483665); }
 let mut pc: u32 = 0x483665;
 loop { match pc {
 0x483665 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48366e); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483672(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483672); }
 let mut pc: u32 = 0x483672;
 loop { match pc {
 0x483672 => {
  { let v: u32 = 0x4b86f0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048368b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48368b); }
 let mut pc: u32 = 0x48368b;
 loop { match pc {
 0x48368b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4836be; continue; }
  pc = 0x4836ad; continue;
 }
 0x4836ad => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4836b9); cpu.call(t);
  pc = 0x4836be; continue;
 }
 0x4836be => {
  pc = 0x4836c3; continue;
 }
 0x4836c3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004836c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4836c8); }
 let mut pc: u32 = 0x4836c8;
 loop { match pc {
 0x4836c8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4836db; continue;
 }
 0x4836db => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483745(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483745); }
 let mut pc: u32 = 0x483745;
 loop { match pc {
 0x483745 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48374d); f_0042c400(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483769(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483769); }
 let mut pc: u32 = 0x483769;
 loop { match pc {
 0x483769 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x483780; continue;
 }
 0x483780 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483785(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483785); }
 let mut pc: u32 = 0x483785;
 loop { match pc {
 0x483785 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x48379f); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x4837a7; continue;
 }
 0x4837a7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004837ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4837ae); }
 let mut pc: u32 = 0x4837ae;
 loop { match pc {
 0x4837ae => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x4837c3); cpu.call(t);
  pc = 0x4837c8; continue;
 }
 0x4837c8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004837cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4837cd); }
 let mut pc: u32 = 0x4837cd;
 loop { match pc {
 0x4837cd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x483955u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x483802); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48380e; continue;
 }
 0x48380e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x483827; continue;
 }
 0x483827 => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48382e; continue;
 }
 0x48382e => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x483837); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x483916; continue; }
  pc = 0x48383f; continue;
 }
 0x48383f => {
  pc = 0x483844; continue;
 }
 0x483844 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x483852; continue;
 }
 0x483852 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x483863); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x483908; continue; }
  pc = 0x48386b; continue;
 }
 0x48386b => {
  pc = 0x483870; continue;
 }
 0x483870 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48387e; continue;
 }
 0x48387e => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x48388f); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x4838a1); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4838ae; continue; }
  pc = 0x4838a9; continue;
 }
 0x4838a9 => {
  pc = 0x4838b3; continue;
 }
 0x4838ae => {
  pc = 0x483908; continue;
 }
 0x4838b3 => {
  pc = 0x4838c2; continue;
 }
 0x4838c2 => {
  pc = 0x4838c7; continue;
 }
 0x4838c7 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x4838ee; continue; }
  pc = 0x4838e6; continue;
 }
 0x4838e6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4838ee; continue;
 }
 0x4838ee => {
  pc = 0x4838f3; continue;
 }
 0x4838f3 => {
  { let v: u32 = cpu.or32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483900); f_0048394c(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48395f; continue;
 }
 0x483908 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x483911); cpu.call(t);
  pc = 0x48382e; continue;
 }
 0x483916 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x483924; continue;
 }
 0x483924 => {
  { let v: u32 = cpu.or32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483931); f_0048394c(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48395f; continue;
 }
 0x48395f => {
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
pub fn f_0048381d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48381d); }
 let mut pc: u32 = 0x48381d;
 loop { match pc {
 0x48381d => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x483826); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483939(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483939); }
 let mut pc: u32 = 0x483939;
 loop { match pc {
 0x483939 => {
  { cpu.and8(cpu.r8(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48394b; continue; }
  pc = 0x483943; continue;
 }
 0x483943 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48394b); f_00405cd0(cpu);
  pc = 0x48394b; continue;
 }
 0x48394b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048394c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48394c); }
 let mut pc: u32 = 0x48394c;
 loop { match pc {
 0x48394c => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x483954); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483955(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483955); }
 let mut pc: u32 = 0x483955;
 loop { match pc {
 0x483955 => {
  { let v: u32 = 0x4b8758u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483970); }
 let mut pc: u32 = 0x483970;
 loop { match pc {
 0x483970 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x483af9u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x34u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x4839a5); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4839b1; continue;
 }
 0x4839b1 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4839ca; continue;
 }
 0x4839ca => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4839d1; continue;
 }
 0x4839d1 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x4839da); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x483aba; continue; }
  pc = 0x4839e2; continue;
 }
 0x4839e2 => {
  pc = 0x4839e7; continue;
 }
 0x4839e7 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4839f5; continue;
 }
 0x4839f5 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x483a06); cpu.call(t);
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x483aac; continue; }
  pc = 0x483a0f; continue;
 }
 0x483a0f => {
  pc = 0x483a14; continue;
 }
 0x483a14 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483a22; continue;
 }
 0x483a22 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.push(0x483a33); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x483a45); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x483a52; continue; }
  pc = 0x483a4d; continue;
 }
 0x483a4d => {
  pc = 0x483a57; continue;
 }
 0x483a52 => {
  pc = 0x483aac; continue;
 }
 0x483a57 => {
  pc = 0x483a66; continue;
 }
 0x483a66 => {
  pc = 0x483a6b; continue;
 }
 0x483a6b => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x483a92; continue; }
  pc = 0x483a8a; continue;
 }
 0x483a8a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x483a92; continue;
 }
 0x483a92 => {
  pc = 0x483a97; continue;
 }
 0x483a97 => {
  { let v: u32 = cpu.or32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483aa4); f_00483af0(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x483b03; continue;
 }
 0x483aac => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x483ab5); cpu.call(t);
  pc = 0x4839d1; continue;
 }
 0x483aba => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x483ac8; continue;
 }
 0x483ac8 => {
  { let v: u32 = cpu.or32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483ad5); f_00483af0(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x483b03; continue;
 }
 0x483b03 => {
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
pub fn f_004839c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4839c0); }
 let mut pc: u32 = 0x4839c0;
 loop { match pc {
 0x4839c0 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x4839c9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483add(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483add); }
 let mut pc: u32 = 0x483add;
 loop { match pc {
 0x483add => {
  { cpu.and8(cpu.r8(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x483aef; continue; }
  pc = 0x483ae7; continue;
 }
 0x483ae7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x483aef); f_00405cd0(cpu);
  pc = 0x483aef; continue;
 }
 0x483aef => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483af0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483af0); }
 let mut pc: u32 = 0x483af0;
 loop { match pc {
 0x483af0 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x483af8); f_0042c5c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483af9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483af9); }
 let mut pc: u32 = 0x483af9;
 loop { match pc {
 0x483af9 => {
  { let v: u32 = 0x4b8790u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483b14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483b14); }
 let mut pc: u32 = 0x483b14;
 loop { match pc {
 0x483b14 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525794u32); cpu.push(0x483b2d); cpu.call(t);
  pc = 0x483b32; continue;
 }
 0x483b32 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483b39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483b39); }
 let mut pc: u32 = 0x483b39;
 loop { match pc {
 0x483b39 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x483cb0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x483b63); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483b78; continue;
 }
 0x483b75 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483b78; continue;
 }
 0x483b78 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x483ca2; continue; }
  pc = 0x483b84; continue;
 }
 0x483b84 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6a10u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x483b9b); f_004af4ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x483ba4); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x483bb9; continue; }
  pc = 0x483bb4; continue;
 }
 0x483bb4 => {
  pc = 0x483ca2; continue;
 }
 0x483bb9 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x483bcf; continue; }
  pc = 0x483bc9; continue;
 }
 0x483bc9 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x483bcf; continue;
 }
 0x483bcf => {
  pc = 0x483bd4; continue;
 }
 0x483bd4 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x483be8); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x483c61; continue; }
  pc = 0x483bfc; continue;
 }
 0x483bfc => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x483c11); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x483c33; continue; }
  pc = 0x483c29; continue;
 }
 0x483c29 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)).wrapping_add(0x1u32); cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x483c33; continue;
 }
 0x483c33 => {
  pc = 0x483c38; continue;
 }
 0x483c38 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483c41; continue;
 }
 0x483c41 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483c5c; continue;
 }
 0x483c5c => {
  pc = 0x483c68; continue;
 }
 0x483c61 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483c68; continue;
 }
 0x483c68 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x483c7f); cpu.call(t);
  pc = 0x483c91; continue;
 }
 0x483c91 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x483c9d); f_00483ca7(cpu);
  pc = 0x483b75; continue;
 }
 0x483ca2 => {
  pc = 0x483cba; continue;
 }
 0x483cba => {
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
pub fn f_00483c50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483c50); }
 let mut pc: u32 = 0x483c50;
 loop { match pc {
 0x483c50 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x483c5b); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483c84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483c84); }
 let mut pc: u32 = 0x483c84;
 loop { match pc {
 0x483c84 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x483c8d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483ca7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483ca7); }
 let mut pc: u32 = 0x483ca7;
 loop { match pc {
 0x483ca7 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x483caf); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483cb0); }
 let mut pc: u32 = 0x483cb0;
 loop { match pc {
 0x483cb0 => {
  { let v: u32 = 0x4b87c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483cca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483cca); }
 let mut pc: u32 = 0x483cca;
 loop { match pc {
 0x483cca => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x483cdf); cpu.call(t);
  pc = 0x483ce4; continue;
 }
 0x483ce4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483ce9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483ce9); }
 let mut pc: u32 = 0x483ce9;
 loop { match pc {
 0x483ce9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x483d0e; continue;
 }
 0x483d0e => {
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
pub fn f_00483d16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483d16); }
 let mut pc: u32 = 0x483d16;
 loop { match pc {
 0x483d16 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x483d42; continue;
 }
 0x483d42 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483d4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483d4c); }
 let mut pc: u32 = 0x483d4c;
 loop { match pc {
 0x483d4c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x483d8f; continue; }
  pc = 0x483d65; continue;
 }
 0x483d65 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x483d8f; continue; }
  pc = 0x483d7e; continue;
 }
 0x483d7e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x483d8a); cpu.call(t);
  pc = 0x483d8f; continue;
 }
 0x483d8f => {
  pc = 0x483d94; continue;
 }
 0x483d94 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483d99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483d99); }
 let mut pc: u32 = 0x483d99;
 loop { match pc {
 0x483d99 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x483de4; continue; }
  pc = 0x483dc8; continue;
 }
 0x483dc8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x483dd9); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x483dee; continue;
 }
 0x483de4 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x483dee; continue;
 }
 0x483dee => {
  pc = 0x483df3; continue;
 }
 0x483df3 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483dfd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483dfd); }
 let mut pc: u32 = 0x483dfd;
 loop { match pc {
 0x483dfd => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if cpu.zf { pc = 0x483e95; continue; }
  pc = 0x483e15; continue;
 }
 0x483e15 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x483e4c; continue; }
  pc = 0x483e22; continue;
 }
 0x483e22 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x483e4c; continue; }
  pc = 0x483e3b; continue;
 }
 0x483e3b => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x483e47); cpu.call(t);
  pc = 0x483e4c; continue;
 }
 0x483e4c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x483e8b; continue; }
  pc = 0x483e6f; continue;
 }
 0x483e6f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.push(0x483e80); cpu.call(t);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x483e95; continue;
 }
 0x483e8b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x483e95; continue;
 }
 0x483e95 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x483e9d; continue;
 }
 0x483e9d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483ea4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483ea4); }
 let mut pc: u32 = 0x483ea4;
 loop { match pc {
 0x483ea4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x483eba; continue;
 }
 0x483eba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483ebf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483ebf); }
 let mut pc: u32 = 0x483ebf;
 loop { match pc {
 0x483ebf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  pc = 0x483ed6; continue;
 }
 0x483ed6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483edb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483edb); }
 let mut pc: u32 = 0x483edb;
 loop { match pc {
 0x483edb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x483ef2; continue;
 }
 0x483ef2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483ef7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483ef7); }
 let mut pc: u32 = 0x483ef7;
 loop { match pc {
 0x483ef7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x483f49; continue; }
  pc = 0x483f10; continue;
 }
 0x483f10 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), cpu.ecx); }
  if cpu.zf { pc = 0x483f49; continue; }
  pc = 0x483f1f; continue;
 }
 0x483f1f => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x483f49; continue; }
  pc = 0x483f38; continue;
 }
 0x483f38 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x483f44); cpu.call(t);
  pc = 0x483f49; continue;
 }
 0x483f49 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x483f57; continue;
 }
 0x483f57 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483f5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483f5e); }
 let mut pc: u32 = 0x483f5e;
 loop { match pc {
 0x483f5e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x483f72); f_00483ebf(cpu);
  { let v = cpu.st(0); cpu.wf32(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x483f7d); f_00483ebf(cpu);
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x483f9a; continue; }
  pc = 0x483f90; continue;
 }
 0x483f90 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x483f9c; continue;
 }
 0x483f9a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x483f9c; continue;
 }
 0x483f9c => {
  pc = 0x483fa1; continue;
 }
 0x483fa1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483fa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483fa8); }
 let mut pc: u32 = 0x483fa8;
 loop { match pc {
 0x483fa8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x483fbe; continue;
 }
 0x483fbe => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483fc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483fc3); }
 let mut pc: u32 = 0x483fc3;
 loop { match pc {
 0x483fc3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483fd4; continue;
 }
 0x483fd4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483fd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483fd9); }
 let mut pc: u32 = 0x483fd9;
 loop { match pc {
 0x483fd9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x483fea; continue;
 }
 0x483fea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00483ff1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x483ff1); }
 let mut pc: u32 = 0x483ff1;
 loop { match pc {
 0x483ff1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484002; continue;
 }
 0x484002 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484007(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484007); }
 let mut pc: u32 = 0x484007;
 loop { match pc {
 0x484007 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484018; continue;
 }
 0x484018 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048401d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48401d); }
 let mut pc: u32 = 0x48401d;
 loop { match pc {
 0x48401d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48402e; continue;
 }
 0x48402e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484033(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484033); }
 let mut pc: u32 = 0x484033;
 loop { match pc {
 0x484033 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x484158u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x484062; continue;
 }
 0x484062 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x484075); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x484085); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  cpu.push(0x484096); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4840a5; continue;
 }
 0x4840a5 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x5cu32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4840b5); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b184cu32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  pc = 0x4840c8; continue;
 }
 0x4840c8 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4840e7; continue;
 }
 0x4840e7 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1834u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4840ff); f_00482784(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  pc = 0x48410e; continue;
 }
 0x48410e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x484162; continue;
 }
 0x484162 => {
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
pub fn f_00484088(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484088); }
 let mut pc: u32 = 0x484088;
 loop { match pc {
 0x484088 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  cpu.push(0x484096); f_0048272e(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4840a5; continue;
 }
 0x4840a5 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x5cu32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x4840b5); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b184cu32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  pc = 0x4840c8; continue;
 }
 0x4840c8 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4840e7; continue;
 }
 0x4840e7 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1834u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4840ff); f_00482784(cpu);
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  pc = 0x48410e; continue;
 }
 0x48410e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x484162; continue;
 }
 0x484162 => {
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
pub fn f_004840da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4840da); }
 let mut pc: u32 = 0x4840da;
 loop { match pc {
 0x4840da => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x5cu32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4840e6); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048411d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48411d); }
 let mut pc: u32 = 0x48411d;
 loop { match pc {
 0x48411d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x5cu32); cpu.ecx = v; }
  cpu.push(0x484128); f_00485690(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484129(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484129); }
 let mut pc: u32 = 0x484129;
 loop { match pc {
 0x484129 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x484134); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484135(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484135); }
 let mut pc: u32 = 0x484135;
 loop { match pc {
 0x484135 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x484141); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484142(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484142); }
 let mut pc: u32 = 0x484142;
 loop { match pc {
 0x484142 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48414e); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048414f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48414f); }
 let mut pc: u32 = 0x48414f;
 loop { match pc {
 0x48414f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x484157); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484158(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484158); }
 let mut pc: u32 = 0x484158;
 loop { match pc {
 0x484158 => {
  { let v: u32 = 0x4b8800u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484200); }
 let mut pc: u32 = 0x484200;
 loop { match pc {
 0x484200 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x484209); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048427e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48427e); }
 let mut pc: u32 = 0x48427e;
 loop { match pc {
 0x48427e => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x484286); f_004854c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048432c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48432c); }
 let mut pc: u32 = 0x48432c;
 loop { match pc {
 0x48432c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x484473u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484356); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484446; continue;
 }
 0x48436d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48437a); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x484391); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48439d); f_00484469(cpu);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4843aa); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4843c1); cpu.call(t);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4843cd); f_0048445f(cpu);
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4843dc; continue;
 }
 0x4843d9 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4843dc; continue;
 }
 0x4843dc => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.cf { pc = 0x484430; continue; }
  pc = 0x4843e6; continue;
 }
 0x4843e6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4843f5; continue;
 }
 0x4843f2 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4843f5; continue;
 }
 0x4843f5 => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.cf { pc = 0x48442b; continue; }
  pc = 0x4843ff; continue;
 }
 0x4843ff => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  cpu.push(0x484412); f_00482a72(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48441e); f_004af810(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  pc = 0x4843f2; continue;
 }
 0x48442b => {
  pc = 0x4843d9; continue;
 }
 0x484430 => {
  pc = 0x484455; continue;
 }
 0x484435 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x48447d; continue;
 }
 0x484446 => {
  { cpu.sub32(cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48436d; continue; }
  pc = 0x484450; continue;
 }
 0x484450 => {
  pc = 0x484435; continue;
 }
 0x484455 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x48447d; continue;
 }
 0x48447d => {
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
pub fn f_0048445f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48445f); }
 let mut pc: u32 = 0x48445f;
 loop { match pc {
 0x48445f => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x484468); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484469(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484469); }
 let mut pc: u32 = 0x484469;
 loop { match pc {
 0x484469 => {
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x484472); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484473(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484473); }
 let mut pc: u32 = 0x484473;
 loop { match pc {
 0x484473 => {
  { let v: u32 = 0x4b8880u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048448e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48448e); }
 let mut pc: u32 = 0x48448e;
 loop { match pc {
 0x48448e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4844a5); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4844b8); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4844cb); f_004af8f4(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4844dd; continue;
 }
 0x4844da => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4844dd; continue;
 }
 0x4844dd => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.cf { pc = 0x48452c; continue; }
  pc = 0x4844e7; continue;
 }
 0x4844e7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4844f6; continue;
 }
 0x4844f3 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4844f6; continue;
 }
 0x4844f6 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.cf { pc = 0x484527; continue; }
  pc = 0x484500; continue;
 }
 0x484500 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x14u32); cpu.ecx = v; }
  cpu.push(0x484513); f_00482a72(cpu);
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48451f); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x4844f3; continue;
 }
 0x484527 => {
  pc = 0x4844da; continue;
 }
 0x48452c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x484536; continue;
 }
 0x484536 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048453d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48453d); }
 let mut pc: u32 = 0x48453d;
 loop { match pc {
 0x48453d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4845f1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48456c; continue;
 }
 0x48456c => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x48457f); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b184cu32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x484592; continue;
 }
 0x484592 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4845b1; continue;
 }
 0x4845b1 => {
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b185cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x4845cd; continue;
 }
 0x4845cd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4845fb; continue;
 }
 0x4845fb => {
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
pub fn f_004845a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4845a4); }
 let mut pc: u32 = 0x4845a4;
 loop { match pc {
 0x4845a4 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4845b0); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004845dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4845dc); }
 let mut pc: u32 = 0x4845dc;
 loop { match pc {
 0x4845dc => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4845e7); f_00485690(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004845e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4845e8); }
 let mut pc: u32 = 0x4845e8;
 loop { match pc {
 0x4845e8 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4845f0); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004845f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4845f1); }
 let mut pc: u32 = 0x4845f1;
 loop { match pc {
 0x4845f1 => {
  { let v: u32 = 0x4b88b0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048460a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48460a); }
 let mut pc: u32 = 0x48460a;
 loop { match pc {
 0x48460a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4846cau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b185cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48463f; continue;
 }
 0x48463f => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x48464b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x484696; continue; }
  pc = 0x484653; continue;
 }
 0x484653 => {
  pc = 0x484658; continue;
 }
 0x484658 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x484685; continue; }
  pc = 0x484674; continue;
 }
 0x484674 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x484680); cpu.call(t);
  pc = 0x484685; continue;
 }
 0x484685 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x484691); cpu.call(t);
  pc = 0x48463f; continue;
 }
 0x484696 => {
  pc = 0x48469b; continue;
 }
 0x48469b => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4846a4); f_004846b5(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4846b0); f_004846c1(cpu);
  pc = 0x4846d4; continue;
 }
 0x4846d4 => {
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
pub fn f_004846b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4846b5); }
 let mut pc: u32 = 0x4846b5;
 loop { match pc {
 0x4846b5 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x4846c0); f_00485690(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004846c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4846c1); }
 let mut pc: u32 = 0x4846c1;
 loop { match pc {
 0x4846c1 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4846c9); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004846ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4846ca); }
 let mut pc: u32 = 0x4846ca;
 loop { match pc {
 0x4846ca => {
  { let v: u32 = 0x4b88e8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048471d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48471d); }
 let mut pc: u32 = 0x48471d;
 loop { match pc {
 0x48471d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4849a5u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x48u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484747); f_004af7c8(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484970; continue;
 }
 0x48475e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484767); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48477c; continue;
 }
 0x484779 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48477c; continue;
 }
 0x48477c => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x48483a; continue; }
  pc = 0x484788; continue;
 }
 0x484788 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6a78u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48479f); f_004af4ed(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4847a8); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4847bf; continue; }
  pc = 0x4847b8; continue;
 }
 0x4847b8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4849af; continue;
 }
 0x4847bf => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4847c6); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4847ff; continue; }
  pc = 0x4847dd; continue;
 }
 0x4847dd => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4847e6); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x4847f4; continue;
 }
 0x4847f4 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484806; continue;
 }
 0x4847ff => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484806; continue;
 }
 0x484806 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x484823); cpu.call(t);
  pc = 0x484835; continue;
 }
 0x484835 => {
  pc = 0x484779; continue;
 }
 0x48483a => {
  pc = 0x484989; continue;
 }
 0x48483f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484848); f_004af7ec(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48495a; continue; }
  pc = 0x484858; continue;
 }
 0x484858 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484865); f_004af837(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48487e; continue;
 }
 0x48487b => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48487e; continue;
 }
 0x48487e => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x48494e; continue; }
  pc = 0x48488a; continue;
 }
 0x48488a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4f6a78u32; cpu.push(v); }
  { let v = 0x4f6a30u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4848a5); f_004af552(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4848ae); f_004ae836(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4848d9; continue; }
  pc = 0x4848be; continue;
 }
 0x4848be => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4848d1); f_0048499b(cpu);
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4849af; continue;
 }
 0x4848d9 => {
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x4848e0); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x484916; continue; }
  pc = 0x4848f4; continue;
 }
 0x4848f4 => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x4848fd); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x48490b; continue;
 }
 0x48490b => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48491d; continue;
 }
 0x484916 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48491d; continue;
 }
 0x48491d => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x484937); cpu.call(t);
  pc = 0x484949; continue;
 }
 0x484949 => {
  pc = 0x48487b; continue;
 }
 0x48494e => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48495a); f_0048499b(cpu);
  pc = 0x48495a; continue;
 }
 0x48495a => {
  pc = 0x484989; continue;
 }
 0x48495f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4849af; continue;
 }
 0x484970 => {
  { cpu.sub32(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48475e; continue; }
  pc = 0x48497a; continue;
 }
 0x48497a => {
  { cpu.sub32(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x48483f; continue; }
  pc = 0x484984; continue;
 }
 0x484984 => {
  pc = 0x48495f; continue;
 }
 0x484989 => {
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x484991); f_00484b08(cpu);
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4849af; continue;
 }
 0x4849af => {
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
pub fn f_00484828(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484828); }
 let mut pc: u32 = 0x484828;
 loop { match pc {
 0x484828 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484831); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048493c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48493c); }
 let mut pc: u32 = 0x48493c;
 loop { match pc {
 0x48493c => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484945); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048499b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48499b); }
 let mut pc: u32 = 0x48499b;
 loop { match pc {
 0x48499b => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4849a4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004849a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4849a5); }
 let mut pc: u32 = 0x4849a5;
 loop { match pc {
 0x4849a5 => {
  { let v: u32 = 0x4b8918u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004849c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4849c0); }
 let mut pc: u32 = 0x4849c0;
 loop { match pc {
 0x4849c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x484aedu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4849ec); f_004af8d2(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52578cu32); cpu.push(0x4849fb); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484a05); f_004af8e3(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x484a1f); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484a2b; continue;
 }
 0x484a2b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484a47; continue;
 }
 0x484a47 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484a4e; continue;
 }
 0x484a4e => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x484a57); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x484ac9; continue; }
  pc = 0x484a5f; continue;
 }
 0x484a5f => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x484a91; continue; }
  pc = 0x484a69; continue;
 }
 0x484a69 => {
  pc = 0x484a6e; continue;
 }
 0x484a6e => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.push(0x484a8c); cpu.call(t);
  pc = 0x484abb; continue;
 }
 0x484a91 => {
  pc = 0x484a96; continue;
 }
 0x484a96 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.push(0x484ab4); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484abb; continue;
 }
 0x484abb => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x484ac4); cpu.call(t);
  pc = 0x484a4e; continue;
 }
 0x484ac9 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484adc); f_00484ae4(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x484af7; continue;
 }
 0x484af7 => {
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
pub fn f_00484a3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484a3d); }
 let mut pc: u32 = 0x484a3d;
 loop { match pc {
 0x484a3d => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x484a46); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484ae4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484ae4); }
 let mut pc: u32 = 0x484ae4;
 loop { match pc {
 0x484ae4 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x484aec); f_004854c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484aed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484aed); }
 let mut pc: u32 = 0x484aed;
 loop { match pc {
 0x484aed => {
  { let v: u32 = 0x4b8950u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484b08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484b08); }
 let mut pc: u32 = 0x484b08;
 loop { match pc {
 0x484b08 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4851cau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xdcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x484b38); cpu.call(t);
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b184cu32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484b4b; continue;
 }
 0x484b4b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484b6a; continue;
 }
 0x484b6a => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x484b7a); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b184cu32; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484b8a; continue;
 }
 0x484b8a => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff68u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484ba6; continue;
 }
 0x484ba6 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525794u32); cpu.push(0x484bbd); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484bc4; continue;
 }
 0x484bc4 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48510f; continue; }
  pc = 0x484bce; continue;
 }
 0x484bce => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x484bd7); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x48510f; continue; }
  pc = 0x484bdf; continue;
 }
 0x484bdf => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525794u32); cpu.push(0x484bf3); cpu.call(t);
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x484c00); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484c09; continue;
 }
 0x484c09 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484c25; continue;
 }
 0x484c25 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484c29; continue;
 }
 0x484c29 => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x484c32); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x485101; continue; }
  pc = 0x484c3a; continue;
 }
 0x484c3a => {
  pc = 0x484c3f; continue;
 }
 0x484c3f => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525730u32); cpu.push(0x484c52); cpu.call(t);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x524080u32; cpu.push(v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x484c66); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x484c73; continue; }
  pc = 0x484c6e; continue;
 }
 0x484c6e => {
  pc = 0x484c78; continue;
 }
 0x484c73 => {
  pc = 0x484de6; continue;
 }
 0x484c78 => {
  pc = 0x484c8a; continue;
 }
 0x484c8a => {
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x484d13; continue; }
  pc = 0x484c9a; continue;
 }
 0x484c9a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4f6930u32; cpu.push(v); }
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x484cac); cpu.call(t);
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff98u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f8u32); cpu.push(0x484cba); cpu.call(t);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x525728u32); cpu.push(0x484cc5); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  let t = cpu.r32(0x5256f4u32); cpu.push(0x484ccd); cpu.call(t);
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484cd6); f_00485191(cpu);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x484cdf); f_0048519b(cpu);
  pc = 0x484ce4; continue;
 }
 0x484ce4 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x484d0e; continue; }
  pc = 0x484cfd; continue;
 }
 0x484cfd => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x484d09); cpu.call(t);
  pc = 0x484d0e; continue;
 }
 0x484d0e => {
  pc = 0x484dd8; continue;
 }
 0x484d13 => {
  pc = 0x484d18; continue;
 }
 0x484d18 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x484d2c; continue;
 }
 0x484d2c => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff50u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x484d3f); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x484d8c; continue; }
  pc = 0x484d59; continue;
 }
 0x484d59 => {
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x484d65); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff50u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x484d7b; continue;
 }
 0x484d7b => {
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff58u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484d96; continue;
 }
 0x484d8c => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff58u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484d96; continue;
 }
 0x484d96 => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff58u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff54u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff54u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x484dbc); cpu.call(t);
  pc = 0x484dd1; continue;
 }
 0x484dd1 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484dd8; continue;
 }
 0x484dd8 => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x484de1); cpu.call(t);
  pc = 0x4850f3; continue;
 }
 0x484de6 => {
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525738u32); cpu.push(0x484df9); cpu.call(t);
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484e02; continue;
 }
 0x484e02 => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484e1e; continue;
 }
 0x484e1e => {
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484e22; continue;
 }
 0x484e22 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x484e2b); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x48502c; continue; }
  pc = 0x484e33; continue;
 }
 0x484e33 => {
  pc = 0x484e38; continue;
 }
 0x484e38 => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x52572cu32); cpu.push(0x484e59); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x484e66; continue; }
  pc = 0x484e61; continue;
 }
 0x484e61 => {
  pc = 0x484e6b; continue;
 }
 0x484e66 => {
  pc = 0x48501e; continue;
 }
 0x484e6b => {
  pc = 0x484e7d; continue;
 }
 0x484e7d => {
  pc = 0x484e82; continue;
 }
 0x484e82 => {
  pc = 0x484e87; continue;
 }
 0x484e87 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff40u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff3cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x484ea9); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x484ef6; continue; }
  pc = 0x484ec3; continue;
 }
 0x484ec3 => {
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x484ecf); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff40u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x484ee5; continue;
 }
 0x484ee5 => {
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff48u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484f00; continue;
 }
 0x484ef6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff48u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484f00; continue;
 }
 0x484f00 => {
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff48u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff44u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff44u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff3cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x484f23); cpu.call(t);
  pc = 0x484f38; continue;
 }
 0x484f38 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x484f49; continue;
 }
 0x484f49 => {
  pc = 0x484f4e; continue;
 }
 0x484f4e => {
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff7cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ecx), v); }
  pc = 0x484f64; continue;
 }
 0x484f64 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff2cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x484f77); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff38u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x484fc4; continue; }
  pc = 0x484f91; continue;
 }
 0x484f91 => {
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x484f9d); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x484fb3; continue;
 }
 0x484fb3 => {
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff34u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484fce; continue;
 }
 0x484fc4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff34u32.wrapping_add(cpu.ebp), v); }
  pc = 0x484fce; continue;
 }
 0x484fce => {
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff34u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff30u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff30u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x484ff4); cpu.call(t);
  pc = 0x485009; continue;
 }
 0x485009 => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x485012); cpu.call(t);
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48502c; continue;
 }
 0x48501e => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525734u32); cpu.push(0x485027); cpu.call(t);
  pc = 0x484e22; continue;
 }
 0x48502c => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52573cu32); cpu.push(0x485035); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4850ea; continue; }
  pc = 0x48503d; continue;
 }
 0x48503d => {
  pc = 0x485042; continue;
 }
 0x485042 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff1cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x485055); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4850a2; continue; }
  pc = 0x48506f; continue;
 }
 0x48506f => {
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x48507b); cpu.call(t);
  { let v: u32 = cpu.r32(0xffffff1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x485091; continue;
 }
 0x485091 => {
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff24u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4850ac; continue;
 }
 0x4850a2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff24u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4850ac; continue;
 }
 0x4850ac => {
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256e4u32); cpu.push(0x4850cc); cpu.call(t);
  pc = 0x4850e1; continue;
 }
 0x4850e1 => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52570cu32); cpu.push(0x4850ea); cpu.call(t);
  pc = 0x4850ea; continue;
 }
 0x4850ea => {
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4850f3); f_00485188(cpu);
  pc = 0x4850f3; continue;
 }
 0x4850f3 => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4850fc); f_004851a5(cpu);
  pc = 0x484c29; continue;
 }
 0x485101 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48510a); f_004851af(cpu);
  pc = 0x484bc4; continue;
 }
 0x48510f => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x485118); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x48516e; continue; }
  pc = 0x485120; continue;
 }
 0x485120 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52579cu32); cpu.push(0x485129); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x48516e; continue; }
  pc = 0x485131; continue;
 }
 0x485131 => {
  pc = 0x485136; continue;
 }
 0x485136 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x485160; continue; }
  pc = 0x48514f; continue;
 }
 0x48514f => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x48515b); cpu.call(t);
  pc = 0x485160; continue;
 }
 0x485160 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525790u32); cpu.push(0x485169); cpu.call(t);
  pc = 0x485120; continue;
 }
 0x48516e => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485177); f_004851b8(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485183); f_004851c1(cpu);
  pc = 0x4851d4; continue;
 }
 0x4851d4 => {
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
pub fn f_00484b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484b60); }
 let mut pc: u32 = 0x484b60;
 loop { match pc {
 0x484b60 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x484b69); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484b9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484b9c); }
 let mut pc: u32 = 0x484b9c;
 loop { match pc {
 0x484b9c => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x484ba5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484c1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484c1b); }
 let mut pc: u32 = 0x484c1b;
 loop { match pc {
 0x484c1b => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x484c24); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484dc1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484dc1); }
 let mut pc: u32 = 0x484dc1;
 loop { match pc {
 0x484dc1 => {
  { let v: u32 = cpu.r32(0xffffff5cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484dcd); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484e14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484e14); }
 let mut pc: u32 = 0x484e14;
 loop { match pc {
 0x484e14 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x484e1d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484f28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484f28); }
 let mut pc: u32 = 0x484f28;
 loop { match pc {
 0x484f28 => {
  { let v: u32 = cpu.r32(0xffffff4cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x484f34); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00484ff9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x484ff9); }
 let mut pc: u32 = 0x484ff9;
 loop { match pc {
 0x484ff9 => {
  { let v: u32 = cpu.r32(0xffffff38u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x485005); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004850d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4850d1); }
 let mut pc: u32 = 0x4850d1;
 loop { match pc {
 0x4850d1 => {
  { let v: u32 = cpu.r32(0xffffff28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4850dd); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485188(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485188); }
 let mut pc: u32 = 0x485188;
 loop { match pc {
 0x485188 => {
  { let v: u32 = 0xffffffb0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x485190); f_004854c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485191(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485191); }
 let mut pc: u32 = 0x485191;
 loop { match pc {
 0x485191 => {
  { let v: u32 = 0xffffff98u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x5256f0u32); cpu.push(0x48519a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048519b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48519b); }
 let mut pc: u32 = 0x48519b;
 loop { match pc {
 0x48519b => {
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4851a4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004851a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4851a5); }
 let mut pc: u32 = 0x4851a5;
 loop { match pc {
 0x4851a5 => {
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4851ae); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004851af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4851af); }
 let mut pc: u32 = 0x4851af;
 loop { match pc {
 0x4851af => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4851b7); f_004854c0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004851b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4851b8); }
 let mut pc: u32 = 0x4851b8;
 loop { match pc {
 0x4851b8 => {
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4851c0); f_00485690(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004851c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4851c1); }
 let mut pc: u32 = 0x4851c1;
 loop { match pc {
 0x4851c1 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  cpu.push(0x4851c9); f_00485690(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004851ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4851ca); }
 let mut pc: u32 = 0x4851ca;
 loop { match pc {
 0x4851ca => {
  { let v: u32 = 0x4b8980u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004851f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4851f0); }
 let mut pc: u32 = 0x4851f0;
 loop { match pc {
 0x4851f0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b17f4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48520a; continue;
 }
 0x48520a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485210); }
 let mut pc: u32 = 0x485210;
 loop { match pc {
 0x485210 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b17f4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48522a; continue;
 }
 0x48522a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x485240; continue; }
  pc = 0x485234; continue;
 }
 0x485234 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48523d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x485240; continue;
 }
 0x485240 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x485248; continue;
 }
 0x485248 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485250); }
 let mut pc: u32 = 0x485250;
 loop { match pc {
 0x485250 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4852ceu32; cpu.push(v); }
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
  pc = 0x485281; continue;
 }
 0x485281 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48528a); f_0048529b(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485296); f_004852a7(cpu);
  pc = 0x4852b0; continue;
 }
 0x4852b0 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4852c6; continue; }
  pc = 0x4852ba; continue;
 }
 0x4852ba => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4852c3); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4852c6; continue;
 }
 0x4852c6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4852d8; continue;
 }
 0x4852d8 => {
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
pub fn f_0048529b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48529b); }
 let mut pc: u32 = 0x48529b;
 loop { match pc {
 0x48529b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  cpu.push(0x4852a6); f_00485560(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004852a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4852a7); }
 let mut pc: u32 = 0x4852a7;
 loop { match pc {
 0x4852a7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4852af); f_004851f0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004852ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4852ce); }
 let mut pc: u32 = 0x4852ce;
 loop { match pc {
 0x4852ce => {
  { let v: u32 = 0x4b8a18u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004852f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4852f0); }
 let mut pc: u32 = 0x4852f0;
 loop { match pc {
 0x4852f0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x485355u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48531d; continue;
 }
 0x48531d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485329); f_0048532e(cpu);
  pc = 0x485337; continue;
 }
 0x485337 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48534d; continue; }
  pc = 0x485341; continue;
 }
 0x485341 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48534a); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x48534d; continue;
 }
 0x48534d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48535f; continue;
 }
 0x48535f => {
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
pub fn f_0048532e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48532e); }
 let mut pc: u32 = 0x48532e;
 loop { match pc {
 0x48532e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x485336); f_0042c400(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485355(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485355); }
 let mut pc: u32 = 0x485355;
 loop { match pc {
 0x485355 => {
  { let v: u32 = 0x4b8a48u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485370); }
 let mut pc: u32 = 0x485370;
 loop { match pc {
 0x485370 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48538a; continue;
 }
 0x48538a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485390); }
 let mut pc: u32 = 0x485390;
 loop { match pc {
 0x485390 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b181cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4853aa; continue;
 }
 0x4853aa => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4853c0; continue; }
  pc = 0x4853b4; continue;
 }
 0x4853b4 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4853bd); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4853c0; continue;
 }
 0x4853c0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4853c8; continue;
 }
 0x4853c8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004853d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4853d0); }
 let mut pc: u32 = 0x4853d0;
 loop { match pc {
 0x4853d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48549bu32; cpu.push(v); }
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
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48540d; continue;
 }
 0x48540d => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485416); f_00485442(cpu);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48541f); f_0048544e(cpu);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485428); f_0048545a(cpu);
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485431); f_00485467(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48543d); f_00485474(cpu);
  pc = 0x48547d; continue;
 }
 0x48547d => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x485493; continue; }
  pc = 0x485487; continue;
 }
 0x485487 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x485490); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x485493; continue;
 }
 0x485493 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4854a5; continue;
 }
 0x4854a5 => {
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
pub fn f_00485442(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485442); }
 let mut pc: u32 = 0x485442;
 loop { match pc {
 0x485442 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x5cu32); cpu.ecx = v; }
  cpu.push(0x48544d); f_00485690(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048544e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48544e); }
 let mut pc: u32 = 0x48544e;
 loop { match pc {
 0x48544e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x485459); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048545a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48545a); }
 let mut pc: u32 = 0x48545a;
 loop { match pc {
 0x48545a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485466); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485467(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485467); }
 let mut pc: u32 = 0x485467;
 loop { match pc {
 0x485467 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485473); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485474); }
 let mut pc: u32 = 0x485474;
 loop { match pc {
 0x485474 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48547c); f_00485370(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048549b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48549b); }
 let mut pc: u32 = 0x48549b;
 loop { match pc {
 0x48549b => {
  { let v: u32 = 0x4b8a70u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004854c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4854c0); }
 let mut pc: u32 = 0x4854c0;
 loop { match pc {
 0x4854c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x485508u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4854ed; continue;
 }
 0x4854ed => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4854f9); f_004854fe(cpu);
  pc = 0x485512; continue;
 }
 0x485512 => {
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
pub fn f_004854fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4854fe); }
 let mut pc: u32 = 0x4854fe;
 loop { match pc {
 0x4854fe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525798u32); cpu.push(0x485507); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485508(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485508); }
 let mut pc: u32 = 0x485508;
 loop { match pc {
 0x485508 => {
  { let v: u32 = 0x4b8ab8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485520); }
 let mut pc: u32 = 0x485520;
 loop { match pc {
 0x485520 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x485534); f_0048460a(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48554a; continue; }
  pc = 0x48553e; continue;
 }
 0x48553e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x485547); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x48554a; continue;
 }
 0x48554a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x485552; continue;
 }
 0x485552 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485560); }
 let mut pc: u32 = 0x485560;
 loop { match pc {
 0x485560 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4855bau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b17d4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x48559a); cpu.call(t);
  pc = 0x48559f; continue;
 }
 0x48559f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4855ab); f_004855b0(cpu);
  pc = 0x4855c4; continue;
 }
 0x4855c4 => {
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
pub fn f_004855b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4855b0); }
 let mut pc: u32 = 0x4855b0;
 loop { match pc {
 0x4855b0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4855b9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004855ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4855ba); }
 let mut pc: u32 = 0x4855ba;
 loop { match pc {
 0x4855ba => {
  { let v: u32 = 0x4b8ae0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004855e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4855e0); }
 let mut pc: u32 = 0x4855e0;
 loop { match pc {
 0x4855e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48566du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x485608); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x485644; continue; }
  pc = 0x48561f; continue;
 }
 0x48561f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x485628); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x485639; continue;
 }
 0x485639 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48564b; continue;
 }
 0x485644 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48564b; continue;
 }
 0x48564b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x485677; continue;
 }
 0x485677 => {
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
pub fn f_00485660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485660); }
 let mut pc: u32 = 0x485660;
 loop { match pc {
 0x485660 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x485669); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048566d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48566d); }
 let mut pc: u32 = 0x48566d;
 loop { match pc {
 0x48566d => {
  { let v: u32 = 0x4b8b08u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485690); }
 let mut pc: u32 = 0x485690;
 loop { match pc {
 0x485690 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4856eau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b184cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4856ca); cpu.call(t);
  pc = 0x4856cf; continue;
 }
 0x4856cf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4856db); f_004856e0(cpu);
  pc = 0x4856f4; continue;
 }
 0x4856f4 => {
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
pub fn f_004856e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4856e0); }
 let mut pc: u32 = 0x4856e0;
 loop { match pc {
 0x4856e0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4856e9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004856ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4856ea); }
 let mut pc: u32 = 0x4856ea;
 loop { match pc {
 0x4856ea => {
  { let v: u32 = 0x4b8b30u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485710); }
 let mut pc: u32 = 0x485710;
 loop { match pc {
 0x485710 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48579du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x485738); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x485774; continue; }
  pc = 0x48574f; continue;
 }
 0x48574f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x485758); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x485769; continue;
 }
 0x485769 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48577b; continue;
 }
 0x485774 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48577b; continue;
 }
 0x48577b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4857a7; continue;
 }
 0x4857a7 => {
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
pub fn f_00485790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485790); }
 let mut pc: u32 = 0x485790;
 loop { match pc {
 0x485790 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x485799); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048579d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48579d); }
 let mut pc: u32 = 0x48579d;
 loop { match pc {
 0x48579d => {
  { let v: u32 = 0x4b8b58u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004857c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4857c0); }
 let mut pc: u32 = 0x4857c0;
 loop { match pc {
 0x4857c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x485838u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b17d4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x4857fa); cpu.call(t);
  pc = 0x4857ff; continue;
 }
 0x4857ff => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48580b); f_00485810(cpu);
  pc = 0x48581a; continue;
 }
 0x48581a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x485830; continue; }
  pc = 0x485824; continue;
 }
 0x485824 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48582d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x485830; continue;
 }
 0x485830 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x485842; continue;
 }
 0x485842 => {
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
pub fn f_00485810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485810); }
 let mut pc: u32 = 0x485810;
 loop { match pc {
 0x485810 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x485819); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485838(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485838); }
 let mut pc: u32 = 0x485838;
 loop { match pc {
 0x485838 => {
  { let v: u32 = 0x4b8b80u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485860); }
 let mut pc: u32 = 0x485860;
 loop { match pc {
 0x485860 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4858d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b184cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x48589a); cpu.call(t);
  pc = 0x48589f; continue;
 }
 0x48589f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4858ab); f_004858b0(cpu);
  pc = 0x4858ba; continue;
 }
 0x4858ba => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4858d0; continue; }
  pc = 0x4858c4; continue;
 }
 0x4858c4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4858cd); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4858d0; continue;
 }
 0x4858d0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4858e2; continue;
 }
 0x4858e2 => {
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
pub fn f_004858b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4858b0); }
 let mut pc: u32 = 0x4858b0;
 loop { match pc {
 0x4858b0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4858b9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004858d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4858d8); }
 let mut pc: u32 = 0x4858d8;
 loop { match pc {
 0x4858d8 => {
  { let v: u32 = 0x4b8ba8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485900); }
 let mut pc: u32 = 0x485900;
 loop { match pc {
 0x485900 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486845u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xb8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x24u32; cpu.push(v); }
  cpu.push(0x485928); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48594f; continue; }
  pc = 0x48593f; continue;
 }
 0x48593f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x485947); f_00498911(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485956; continue;
 }
 0x48594f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485956; continue;
 }
 0x485956 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4f68f0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x485a56; continue; }
  pc = 0x485974; continue;
 }
 0x485974 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524124u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x524210u32, v); }
  pc = 0x485991; continue;
 }
 0x485991 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x524214u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x4859a4); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b18acu32; cpu.w32(0x524210u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6a98u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x4859bf); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4859c9); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x4859dc); cpu.call(t);
  { let v: u8 = 0x4u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x524214u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4859ed); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4859f6); f_00485a1d(cpu);
  { let v = 0x524210u32; cpu.push(v); }
  cpu.push(0x485a00); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485a0c); f_00485a27(cpu);
  pc = 0x485a11; continue;
 }
 0x485a11 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485a49; continue;
 }
 0x485a49 => {
  { let v = 0x487a82u32; cpu.push(v); }
  cpu.push(0x485a53); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x485a56; continue;
 }
 0x485a56 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x485b47; continue; }
  pc = 0x485a65; continue;
 }
 0x485a65 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524124u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x524258u32, v); }
  pc = 0x485a82; continue;
 }
 0x485a82 => {
  { let v: u32 = 0x5u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x52425cu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x485a95); cpu.call(t);
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b188cu32; cpu.w32(0x524258u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6a78u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x485ab0); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x485aba); cpu.call(t);
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x485acd); cpu.call(t);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x52425cu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x485ade); cpu.call(t);
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485ae7); f_00485b0e(cpu);
  { let v = 0x524258u32; cpu.push(v); }
  cpu.push(0x485af1); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x6u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485afd); f_00485b18(cpu);
  pc = 0x485b02; continue;
 }
 0x485b02 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485b3a; continue;
 }
 0x485b3a => {
  { let v = 0x4879f1u32; cpu.push(v); }
  cpu.push(0x485b44); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x485b47; continue;
 }
 0x485b47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x485c38; continue; }
  pc = 0x485b56; continue;
 }
 0x485b56 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524124u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x5240f8u32, v); }
  pc = 0x485b73; continue;
 }
 0x485b73 => {
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5240fcu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x485b86); cpu.call(t);
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b193cu32; cpu.w32(0x5240f8u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6e50u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x485ba1); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x485bab); cpu.call(t);
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x485bbe); cpu.call(t);
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5240fcu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x485bcf); cpu.call(t);
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485bd8); f_00485bff(cpu);
  { let v = 0x5240f8u32; cpu.push(v); }
  cpu.push(0x485be2); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485bee); f_00485c09(cpu);
  pc = 0x485bf3; continue;
 }
 0x485bf3 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485c2b; continue;
 }
 0x485c2b => {
  { let v = 0x487960u32; cpu.push(v); }
  cpu.push(0x485c35); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x485c38; continue;
 }
 0x485c38 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x485d29; continue; }
  pc = 0x485c47; continue;
 }
 0x485c47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524124u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x524220u32, v); }
  pc = 0x485c64; continue;
 }
 0x485c64 => {
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x524224u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x485c77); cpu.call(t);
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b192cu32; cpu.w32(0x524220u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6e30u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x485c92); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x485c9c); cpu.call(t);
  { let v: u8 = 0xfu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x485caf); cpu.call(t);
  { let v: u8 = 0x10u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x524224u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x485cc0); cpu.call(t);
  { let v: u8 = 0xfu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485cc9); f_00485cf0(cpu);
  { let v = 0x524220u32; cpu.push(v); }
  cpu.push(0x485cd3); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485cdf); f_00485cfa(cpu);
  pc = 0x485ce4; continue;
 }
 0x485ce4 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485d1c; continue;
 }
 0x485d1c => {
  { let v = 0x4878cfu32; cpu.push(v); }
  cpu.push(0x485d26); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x485d29; continue;
 }
 0x485d29 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if !cpu.zf { pc = 0x485e1a; continue; }
  pc = 0x485d38; continue;
 }
 0x485d38 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524124u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x524118u32, v); }
  pc = 0x485d55; continue;
 }
 0x485d55 => {
  { let v: u32 = 0x11u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x52411cu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x485d68); cpu.call(t);
  { let v: u8 = 0x12u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b190cu32; cpu.w32(0x524118u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6e10u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x485d83); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x485d8d); cpu.call(t);
  { let v: u8 = 0x13u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x485da0); cpu.call(t);
  { let v: u8 = 0x14u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x52411cu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x485db1); cpu.call(t);
  { let v: u8 = 0x13u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485dba); f_00485de1(cpu);
  { let v = 0x524118u32; cpu.push(v); }
  cpu.push(0x485dc4); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x12u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485dd0); f_00485deb(cpu);
  pc = 0x485dd5; continue;
 }
 0x485dd5 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485e0d; continue;
 }
 0x485e0d => {
  { let v = 0x48783eu32; cpu.push(v); }
  cpu.push(0x485e17); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x485e1a; continue;
 }
 0x485e1a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if !cpu.zf { pc = 0x485f0b; continue; }
  pc = 0x485e29; continue;
 }
 0x485e29 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524124u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x524190u32, v); }
  pc = 0x485e46; continue;
 }
 0x485e46 => {
  { let v: u32 = 0x15u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x524194u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x485e59); cpu.call(t);
  { let v: u8 = 0x16u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b18fcu32; cpu.w32(0x524190u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6de8u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x485e74); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff94u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x485e7e); cpu.call(t);
  { let v: u8 = 0x17u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffff8cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff94u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x485e91); cpu.call(t);
  { let v: u8 = 0x18u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x524194u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x485ea2); cpu.call(t);
  { let v: u8 = 0x17u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485eab); f_00485ed2(cpu);
  { let v = 0x524190u32; cpu.push(v); }
  cpu.push(0x485eb5); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x16u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485ec1); f_00485edc(cpu);
  pc = 0x485ec6; continue;
 }
 0x485ec6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485efe; continue;
 }
 0x485efe => {
  { let v = 0x4877adu32; cpu.push(v); }
  cpu.push(0x485f08); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x485f0b; continue;
 }
 0x485f0b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x486002; continue; }
  pc = 0x485f1a; continue;
 }
 0x485f1a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524124u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x524148u32, v); }
  pc = 0x485f37; continue;
 }
 0x485f37 => {
  { let v: u32 = 0x19u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x52414cu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x485f4a); cpu.call(t);
  { let v: u8 = 0x1au8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b18ecu32; cpu.w32(0x524148u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6dc8u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x485f65); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff84u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x485f6f); cpu.call(t);
  { let v: u8 = 0x1bu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff84u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x485f85); cpu.call(t);
  { let v: u8 = 0x1cu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x52414cu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x485f96); cpu.call(t);
  { let v: u8 = 0x1bu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485f9f); f_00485fc6(cpu);
  { let v = 0x524148u32; cpu.push(v); }
  cpu.push(0x485fa9); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x1au8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x485fb5); f_00485fd3(cpu);
  pc = 0x485fba; continue;
 }
 0x485fba => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x485ff5; continue;
 }
 0x485ff5 => {
  { let v = 0x48771cu32; cpu.push(v); }
  cpu.push(0x485fff); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486002; continue;
 }
 0x486002 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x486102; continue; }
  pc = 0x486011; continue;
 }
 0x486011 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524124u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524124u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x524180u32, v); }
  pc = 0x48602e; continue;
 }
 0x48602e => {
  { let v: u32 = 0x1du32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x524184u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x486041); cpu.call(t);
  { let v: u8 = 0x1eu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b18dcu32; cpu.w32(0x524180u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6da0u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48605c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff74u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x486069); cpu.call(t);
  { let v: u8 = 0x1fu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffff6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff74u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x486082); cpu.call(t);
  { let v: u8 = 0x20u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x524184u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x486093); cpu.call(t);
  { let v: u8 = 0x1fu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48609c); f_004860c3(cpu);
  { let v = 0x524180u32; cpu.push(v); }
  cpu.push(0x4860a6); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x1eu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4860b2); f_004860d0(cpu);
  pc = 0x4860b7; continue;
 }
 0x4860b7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4860f5; continue;
 }
 0x4860f5 => {
  { let v = 0x48768bu32; cpu.push(v); }
  cpu.push(0x4860ff); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486102; continue;
 }
 0x486102 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x486202; continue; }
  pc = 0x486111; continue;
 }
 0x486111 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524168u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x5241d0u32, v); }
  pc = 0x48612e; continue;
 }
 0x48612e => {
  { let v: u32 = 0x21u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5241d4u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x486141); cpu.call(t);
  { let v: u8 = 0x22u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b18ccu32; cpu.w32(0x5241d0u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6d80u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48615c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff64u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x486169); cpu.call(t);
  { let v: u8 = 0x23u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff64u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x486182); cpu.call(t);
  { let v: u8 = 0x24u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5241d4u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x486193); cpu.call(t);
  { let v: u8 = 0x23u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48619c); f_004861c3(cpu);
  { let v = 0x5241d0u32; cpu.push(v); }
  cpu.push(0x4861a6); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x22u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4861b2); f_004861d0(cpu);
  pc = 0x4861b7; continue;
 }
 0x4861b7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4861f5; continue;
 }
 0x4861f5 => {
  { let v = 0x4875fau32; cpu.push(v); }
  cpu.push(0x4861ff); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486202; continue;
 }
 0x486202 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x486302; continue; }
  pc = 0x486211; continue;
 }
 0x486211 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524168u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x5241c0u32, v); }
  pc = 0x48622e; continue;
 }
 0x48622e => {
  { let v: u32 = 0x25u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5241c4u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x486241); cpu.call(t);
  { let v: u8 = 0x26u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b18bcu32; cpu.w32(0x5241c0u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6d50u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48625c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x486269); cpu.call(t);
  { let v: u8 = 0x27u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffff4cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x486282); cpu.call(t);
  { let v: u8 = 0x28u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5241c4u32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x486293); cpu.call(t);
  { let v: u8 = 0x27u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48629c); f_004862c3(cpu);
  { let v = 0x5241c0u32; cpu.push(v); }
  cpu.push(0x4862a6); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x26u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4862b2); f_004862d0(cpu);
  pc = 0x4862b7; continue;
 }
 0x4862b7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4862f5; continue;
 }
 0x4862f5 => {
  { let v = 0x487569u32; cpu.push(v); }
  cpu.push(0x4862ff); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486302; continue;
 }
 0x486302 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x486402; continue; }
  pc = 0x486311; continue;
 }
 0x486311 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524168u32, v); }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x5240e8u32, v); }
  pc = 0x48632e; continue;
 }
 0x48632e => {
  { let v: u32 = 0x29u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5240ecu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x486341); cpu.call(t);
  { let v: u8 = 0x2au8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4b189cu32; cpu.w32(0x5240e8u32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f6d30u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48635c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff44u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x486369); cpu.call(t);
  { let v: u8 = 0x2bu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff44u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x486382); cpu.call(t);
  { let v: u8 = 0x2cu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5240ecu32; cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x486393); cpu.call(t);
  { let v: u8 = 0x2bu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48639c); f_004863c3(cpu);
  { let v = 0x5240e8u32; cpu.push(v); }
  cpu.push(0x4863a6); f_004af36d(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = 0x2au8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4863b2); f_004863d0(cpu);
  pc = 0x4863b7; continue;
 }
 0x4863b7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4863f5; continue;
 }
 0x4863f5 => {
  { let v = 0x4874d8u32; cpu.push(v); }
  cpu.push(0x4863ff); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486402; continue;
 }
 0x486402 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x486436; continue; }
  pc = 0x486411; continue;
 }
 0x486411 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524168u32, v); }
  { let v: u32 = 0x5241e0u32; cpu.ecx = v; }
  cpu.push(0x486429); f_00488970(cpu);
  { let v = 0x487447u32; cpu.push(v); }
  cpu.push(0x486433); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486436; continue;
 }
 0x486436 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if !cpu.zf { pc = 0x48646a; continue; }
  pc = 0x486445; continue;
 }
 0x486445 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524168u32, v); }
  { let v: u32 = 0x524230u32; cpu.ecx = v; }
  cpu.push(0x48645d); f_00488bb0(cpu);
  { let v = 0x4873b6u32; cpu.push(v); }
  cpu.push(0x486467); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x48646a; continue;
 }
 0x48646a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if !cpu.zf { pc = 0x48649e; continue; }
  pc = 0x486479; continue;
 }
 0x486479 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524168u32, v); }
  { let v: u32 = 0x5241f0u32; cpu.ecx = v; }
  cpu.push(0x486491); f_00488da0(cpu);
  { let v = 0x487325u32; cpu.push(v); }
  cpu.push(0x48649b); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x48649e; continue;
 }
 0x48649e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x4864d2; continue; }
  pc = 0x4864ad; continue;
 }
 0x4864ad => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524168u32, v); }
  { let v: u32 = 0x524088u32; cpu.ecx = v; }
  cpu.push(0x4864c5); f_00488fe0(cpu);
  { let v = 0x487294u32; cpu.push(v); }
  cpu.push(0x4864cf); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4864d2; continue;
 }
 0x4864d2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x486506; continue; }
  pc = 0x4864e1; continue;
 }
 0x4864e1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524168u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524168u32, v); }
  { let v: u32 = 0x5241b0u32; cpu.ecx = v; }
  cpu.push(0x4864f9); f_004891d0(cpu);
  { let v = 0x487203u32; cpu.push(v); }
  cpu.push(0x486503); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486506; continue;
 }
 0x486506 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x48653a; continue; }
  pc = 0x486515; continue;
 }
 0x486515 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52419cu32, v); }
  { let v: u32 = 0x524200u32; cpu.ecx = v; }
  cpu.push(0x48652d); f_00489410(cpu);
  { let v = 0x487172u32; cpu.push(v); }
  cpu.push(0x486537); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x48653a; continue;
 }
 0x48653a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x486570; continue; }
  pc = 0x486549; continue;
 }
 0x486549 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52419cu32, v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x524158u32; cpu.ecx = v; }
  cpu.push(0x486563); f_00489620(cpu);
  { let v = 0x4870ccu32; cpu.push(v); }
  cpu.push(0x48656d); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486570; continue;
 }
 0x486570 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x4865a6; continue; }
  pc = 0x48657f; continue;
 }
 0x48657f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52419cu32, v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x5241a0u32; cpu.ecx = v; }
  cpu.push(0x486599); f_00489ca0(cpu);
  { let v = 0x487026u32; cpu.push(v); }
  cpu.push(0x4865a3); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4865a6; continue;
 }
 0x4865a6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x4865dc; continue; }
  pc = 0x4865b5; continue;
 }
 0x4865b5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52419cu32, v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x5240c0u32; cpu.ecx = v; }
  cpu.push(0x4865cf); f_0048a2e0(cpu);
  { let v = 0x486f80u32; cpu.push(v); }
  cpu.push(0x4865d9); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4865dc; continue;
 }
 0x4865dc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if !cpu.zf { pc = 0x486612; continue; }
  pc = 0x4865eb; continue;
 }
 0x4865eb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52419cu32, v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x524278u32; cpu.ecx = v; }
  cpu.push(0x486605); f_0048aa20(cpu);
  { let v = 0x486edau32; cpu.push(v); }
  cpu.push(0x48660f); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486612; continue;
 }
 0x486612 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if !cpu.zf { pc = 0x486648; continue; }
  pc = 0x486621; continue;
 }
 0x486621 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52419cu32, v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v: u32 = 0x524268u32; cpu.ecx = v; }
  cpu.push(0x48663b); f_0048b160(cpu);
  { let v = 0x486e34u32; cpu.push(v); }
  cpu.push(0x486645); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486648; continue;
 }
 0x486648 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x48667e; continue; }
  pc = 0x486657; continue;
 }
 0x486657 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52419cu32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x524170u32; cpu.ecx = v; }
  cpu.push(0x486671); f_0048b810(cpu);
  { let v = 0x486d8eu32; cpu.push(v); }
  cpu.push(0x48667b); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x48667e; continue;
 }
 0x48667e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x4866b4; continue; }
  pc = 0x48668d; continue;
 }
 0x48668d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x52419cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52419cu32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x524298u32; cpu.ecx = v; }
  cpu.push(0x4866a7); f_0048be80(cpu);
  { let v = 0x486ce8u32; cpu.push(v); }
  cpu.push(0x4866b1); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4866b4; continue;
 }
 0x4866b4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x4866ea; continue; }
  pc = 0x4866c3; continue;
 }
 0x4866c3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524250u32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x524138u32; cpu.ecx = v; }
  cpu.push(0x4866dd); f_0048c4f0(cpu);
  { let v = 0x486c42u32; cpu.push(v); }
  cpu.push(0x4866e7); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4866ea; continue;
 }
 0x4866ea => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x486720; continue; }
  pc = 0x4866f9; continue;
 }
 0x4866f9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524250u32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x524098u32; cpu.ecx = v; }
  cpu.push(0x486713); f_0048cb60(cpu);
  { let v = 0x486b9cu32; cpu.push(v); }
  cpu.push(0x48671d); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486720; continue;
 }
 0x486720 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x486756; continue; }
  pc = 0x48672f; continue;
 }
 0x48672f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524250u32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x5240d0u32; cpu.ecx = v; }
  cpu.push(0x486749); f_0048d1f0(cpu);
  { let v = 0x486af6u32; cpu.push(v); }
  cpu.push(0x486753); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x486756; continue;
 }
 0x486756 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x48678c; continue; }
  pc = 0x486765; continue;
 }
 0x486765 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524250u32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x524240u32; cpu.ecx = v; }
  cpu.push(0x48677f); f_0048d860(cpu);
  { let v = 0x486a50u32; cpu.push(v); }
  cpu.push(0x486789); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x48678c; continue;
 }
 0x48678c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if !cpu.zf { pc = 0x4867c2; continue; }
  pc = 0x48679b; continue;
 }
 0x48679b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524250u32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x524128u32; cpu.ecx = v; }
  cpu.push(0x4867b5); f_0048ded0(cpu);
  { let v = 0x4869aau32; cpu.push(v); }
  cpu.push(0x4867bf); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4867c2; continue;
 }
 0x4867c2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if !cpu.zf { pc = 0x4867f8; continue; }
  pc = 0x4867d1; continue;
 }
 0x4867d1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524250u32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x524288u32; cpu.ecx = v; }
  cpu.push(0x4867eb); f_0048e560(cpu);
  { let v = 0x486904u32; cpu.push(v); }
  cpu.push(0x4867f5); f_004ae7f0(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4867f8; continue;
 }
 0x4867f8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x48682e; continue; }
  pc = 0x486807; continue;
 }
 0x486807 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524250u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524250u32, v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x524108u32; cpu.ecx = v; }
  cpu.push(0x486821); f_0048ebf0(cpu);
  { let v = 0x48685eu32; cpu.push(v); }
  cpu.push(0x48682b); f_004ae7f0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x48682e; continue;
 }
 0x48682e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x48684f; continue;
 }
 0x48684f => {
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
pub fn f_00485a1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485a1d); }
 let mut pc: u32 = 0x485a1d;
 loop { match pc {
 0x485a1d => {
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485a26); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485a27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485a27); }
 let mut pc: u32 = 0x485a27;
 loop { match pc {
 0x485a27 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485a30); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485a31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485a31); }
 let mut pc: u32 = 0x485a31;
 loop { match pc {
 0x485a31 => {
  { let v: u32 = 0x524214u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485a3d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485a3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485a3e); }
 let mut pc: u32 = 0x485a3e;
 loop { match pc {
 0x485a3e => {
  { let v: u32 = 0x524210u32; cpu.ecx = v; }
  cpu.push(0x485a48); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485b0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485b0e); }
 let mut pc: u32 = 0x485b0e;
 loop { match pc {
 0x485b0e => {
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485b17); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485b18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485b18); }
 let mut pc: u32 = 0x485b18;
 loop { match pc {
 0x485b18 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485b21); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485b22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485b22); }
 let mut pc: u32 = 0x485b22;
 loop { match pc {
 0x485b22 => {
  { let v: u32 = 0x52425cu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485b2e); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485b2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485b2f); }
 let mut pc: u32 = 0x485b2f;
 loop { match pc {
 0x485b2f => {
  { let v: u32 = 0x524258u32; cpu.ecx = v; }
  cpu.push(0x485b39); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485bff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485bff); }
 let mut pc: u32 = 0x485bff;
 loop { match pc {
 0x485bff => {
  { let v: u32 = 0xffffffbcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485c08); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485c09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485c09); }
 let mut pc: u32 = 0x485c09;
 loop { match pc {
 0x485c09 => {
  { let v: u32 = 0xffffffc4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485c12); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485c13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485c13); }
 let mut pc: u32 = 0x485c13;
 loop { match pc {
 0x485c13 => {
  { let v: u32 = 0x5240fcu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485c1f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485c20); }
 let mut pc: u32 = 0x485c20;
 loop { match pc {
 0x485c20 => {
  { let v: u32 = 0x5240f8u32; cpu.ecx = v; }
  cpu.push(0x485c2a); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485cf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485cf0); }
 let mut pc: u32 = 0x485cf0;
 loop { match pc {
 0x485cf0 => {
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485cf9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485cfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485cfa); }
 let mut pc: u32 = 0x485cfa;
 loop { match pc {
 0x485cfa => {
  { let v: u32 = 0xffffffb4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485d03); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485d04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485d04); }
 let mut pc: u32 = 0x485d04;
 loop { match pc {
 0x485d04 => {
  { let v: u32 = 0x524224u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485d10); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485d11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485d11); }
 let mut pc: u32 = 0x485d11;
 loop { match pc {
 0x485d11 => {
  { let v: u32 = 0x524220u32; cpu.ecx = v; }
  cpu.push(0x485d1b); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485de1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485de1); }
 let mut pc: u32 = 0x485de1;
 loop { match pc {
 0x485de1 => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485dea); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485deb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485deb); }
 let mut pc: u32 = 0x485deb;
 loop { match pc {
 0x485deb => {
  { let v: u32 = 0xffffffa4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485df4); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485df5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485df5); }
 let mut pc: u32 = 0x485df5;
 loop { match pc {
 0x485df5 => {
  { let v: u32 = 0x52411cu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485e01); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485e02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485e02); }
 let mut pc: u32 = 0x485e02;
 loop { match pc {
 0x485e02 => {
  { let v: u32 = 0x524118u32; cpu.ecx = v; }
  cpu.push(0x485e0c); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485ed2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485ed2); }
 let mut pc: u32 = 0x485ed2;
 loop { match pc {
 0x485ed2 => {
  { let v: u32 = 0xffffff8cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485edb); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485edc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485edc); }
 let mut pc: u32 = 0x485edc;
 loop { match pc {
 0x485edc => {
  { let v: u32 = 0xffffff94u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485ee5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485ee6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485ee6); }
 let mut pc: u32 = 0x485ee6;
 loop { match pc {
 0x485ee6 => {
  { let v: u32 = 0x524194u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485ef2); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485ef3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485ef3); }
 let mut pc: u32 = 0x485ef3;
 loop { match pc {
 0x485ef3 => {
  { let v: u32 = 0x524190u32; cpu.ecx = v; }
  cpu.push(0x485efd); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485fc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485fc6); }
 let mut pc: u32 = 0x485fc6;
 loop { match pc {
 0x485fc6 => {
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485fd2); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485fd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485fd3); }
 let mut pc: u32 = 0x485fd3;
 loop { match pc {
 0x485fd3 => {
  { let v: u32 = 0xffffff84u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485fdc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485fdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485fdd); }
 let mut pc: u32 = 0x485fdd;
 loop { match pc {
 0x485fdd => {
  { let v: u32 = 0x52414cu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x485fe9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00485fea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x485fea); }
 let mut pc: u32 = 0x485fea;
 loop { match pc {
 0x485fea => {
  { let v: u32 = 0x524148u32; cpu.ecx = v; }
  cpu.push(0x485ff4); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004860c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4860c3); }
 let mut pc: u32 = 0x4860c3;
 loop { match pc {
 0x4860c3 => {
  { let v: u32 = 0xffffff6cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4860cf); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004860d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4860d0); }
 let mut pc: u32 = 0x4860d0;
 loop { match pc {
 0x4860d0 => {
  { let v: u32 = 0xffffff74u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4860dc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004860dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4860dd); }
 let mut pc: u32 = 0x4860dd;
 loop { match pc {
 0x4860dd => {
  { let v: u32 = 0x524184u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4860e9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004860ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4860ea); }
 let mut pc: u32 = 0x4860ea;
 loop { match pc {
 0x4860ea => {
  { let v: u32 = 0x524180u32; cpu.ecx = v; }
  cpu.push(0x4860f4); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004861c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4861c3); }
 let mut pc: u32 = 0x4861c3;
 loop { match pc {
 0x4861c3 => {
  { let v: u32 = 0xffffff5cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4861cf); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004861d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4861d0); }
 let mut pc: u32 = 0x4861d0;
 loop { match pc {
 0x4861d0 => {
  { let v: u32 = 0xffffff64u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4861dc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004861dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4861dd); }
 let mut pc: u32 = 0x4861dd;
 loop { match pc {
 0x4861dd => {
  { let v: u32 = 0x5241d4u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4861e9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004861ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4861ea); }
 let mut pc: u32 = 0x4861ea;
 loop { match pc {
 0x4861ea => {
  { let v: u32 = 0x5241d0u32; cpu.ecx = v; }
  cpu.push(0x4861f4); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004862c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4862c3); }
 let mut pc: u32 = 0x4862c3;
 loop { match pc {
 0x4862c3 => {
  { let v: u32 = 0xffffff4cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4862cf); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004862d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4862d0); }
 let mut pc: u32 = 0x4862d0;
 loop { match pc {
 0x4862d0 => {
  { let v: u32 = 0xffffff54u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4862dc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004862dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4862dd); }
 let mut pc: u32 = 0x4862dd;
 loop { match pc {
 0x4862dd => {
  { let v: u32 = 0x5241c4u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4862e9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004862ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4862ea); }
 let mut pc: u32 = 0x4862ea;
 loop { match pc {
 0x4862ea => {
  { let v: u32 = 0x5241c0u32; cpu.ecx = v; }
  cpu.push(0x4862f4); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004863c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4863c3); }
 let mut pc: u32 = 0x4863c3;
 loop { match pc {
 0x4863c3 => {
  { let v: u32 = 0xffffff3cu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4863cf); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004863d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4863d0); }
 let mut pc: u32 = 0x4863d0;
 loop { match pc {
 0x4863d0 => {
  { let v: u32 = 0xffffff44u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4863dc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004863dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4863dd); }
 let mut pc: u32 = 0x4863dd;
 loop { match pc {
 0x4863dd => {
  { let v: u32 = 0x5240ecu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4863e9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004863ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4863ea); }
 let mut pc: u32 = 0x4863ea;
 loop { match pc {
 0x4863ea => {
  { let v: u32 = 0x5240e8u32; cpu.ecx = v; }
  cpu.push(0x4863f4); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486838(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486838); }
 let mut pc: u32 = 0x486838;
 loop { match pc {
 0x486838 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x486841); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486845(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486845); }
 let mut pc: u32 = 0x486845;
 loop { match pc {
 0x486845 => {
  { let v: u32 = 0x4b8bd0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048685e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48685e); }
 let mut pc: u32 = 0x48685e;
 loop { match pc {
 0x48685e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4868ecu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b194cu32; cpu.w32(0x524108u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x4868b0; continue; }
  pc = 0x48689b; continue;
 }
 0x48689b => {
  { let v = 0x524108u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x4868b0); cpu.call(t);
  pc = 0x4868b0; continue;
 }
 0x4868b0 => {
  pc = 0x4868b5; continue;
 }
 0x4868b5 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4868be); f_004868cf(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4868ca); f_004868dc(cpu);
  pc = 0x4868e7; continue;
 }
 0x4868e7 => {
  pc = 0x4868f6; continue;
 }
 0x4868f6 => {
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
pub fn f_004868cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4868cf); }
 let mut pc: u32 = 0x4868cf;
 loop { match pc {
 0x4868cf => {
  { let v: u32 = 0x524110u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4868db); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004868dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4868dc); }
 let mut pc: u32 = 0x4868dc;
 loop { match pc {
 0x4868dc => {
  { let v: u32 = 0x524108u32; cpu.ecx = v; }
  cpu.push(0x4868e6); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004868ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4868ec); }
 let mut pc: u32 = 0x4868ec;
 loop { match pc {
 0x4868ec => {
  { let v: u32 = 0x4b8d58u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486904(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486904); }
 let mut pc: u32 = 0x486904;
 loop { match pc {
 0x486904 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486992u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b198cu32; cpu.w32(0x524288u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486956; continue; }
  pc = 0x486941; continue;
 }
 0x486941 => {
  { let v = 0x524288u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486956); cpu.call(t);
  pc = 0x486956; continue;
 }
 0x486956 => {
  pc = 0x48695b; continue;
 }
 0x48695b => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486964); f_00486975(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486970); f_00486982(cpu);
  pc = 0x48698d; continue;
 }
 0x48698d => {
  pc = 0x48699c; continue;
 }
 0x48699c => {
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
pub fn f_00486975(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486975); }
 let mut pc: u32 = 0x486975;
 loop { match pc {
 0x486975 => {
  { let v: u32 = 0x524290u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486981); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486982(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486982); }
 let mut pc: u32 = 0x486982;
 loop { match pc {
 0x486982 => {
  { let v: u32 = 0x524288u32; cpu.ecx = v; }
  cpu.push(0x48698c); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486992(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486992); }
 let mut pc: u32 = 0x486992;
 loop { match pc {
 0x486992 => {
  { let v: u32 = 0x4b8d88u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004869aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4869aa); }
 let mut pc: u32 = 0x4869aa;
 loop { match pc {
 0x4869aa => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486a38u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b19acu32; cpu.w32(0x524128u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x4869fc; continue; }
  pc = 0x4869e7; continue;
 }
 0x4869e7 => {
  { let v = 0x524128u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x4869fc); cpu.call(t);
  pc = 0x4869fc; continue;
 }
 0x4869fc => {
  pc = 0x486a01; continue;
 }
 0x486a01 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486a0a); f_00486a1b(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486a16); f_00486a28(cpu);
  pc = 0x486a33; continue;
 }
 0x486a33 => {
  pc = 0x486a42; continue;
 }
 0x486a42 => {
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
pub fn f_00486a1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486a1b); }
 let mut pc: u32 = 0x486a1b;
 loop { match pc {
 0x486a1b => {
  { let v: u32 = 0x524130u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486a27); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486a28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486a28); }
 let mut pc: u32 = 0x486a28;
 loop { match pc {
 0x486a28 => {
  { let v: u32 = 0x524128u32; cpu.ecx = v; }
  cpu.push(0x486a32); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486a38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486a38); }
 let mut pc: u32 = 0x486a38;
 loop { match pc {
 0x486a38 => {
  { let v: u32 = 0x4b8db8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486a50); }
 let mut pc: u32 = 0x486a50;
 loop { match pc {
 0x486a50 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486adeu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b19ccu32; cpu.w32(0x524240u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486aa2; continue; }
  pc = 0x486a8d; continue;
 }
 0x486a8d => {
  { let v = 0x524240u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486aa2); cpu.call(t);
  pc = 0x486aa2; continue;
 }
 0x486aa2 => {
  pc = 0x486aa7; continue;
 }
 0x486aa7 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486ab0); f_00486ac1(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486abc); f_00486ace(cpu);
  pc = 0x486ad9; continue;
 }
 0x486ad9 => {
  pc = 0x486ae8; continue;
 }
 0x486ae8 => {
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
pub fn f_00486ac1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486ac1); }
 let mut pc: u32 = 0x486ac1;
 loop { match pc {
 0x486ac1 => {
  { let v: u32 = 0x524248u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486acd); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486ace(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486ace); }
 let mut pc: u32 = 0x486ace;
 loop { match pc {
 0x486ace => {
  { let v: u32 = 0x524240u32; cpu.ecx = v; }
  cpu.push(0x486ad8); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486ade(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486ade); }
 let mut pc: u32 = 0x486ade;
 loop { match pc {
 0x486ade => {
  { let v: u32 = 0x4b8de8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486af6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486af6); }
 let mut pc: u32 = 0x486af6;
 loop { match pc {
 0x486af6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486b84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b19ecu32; cpu.w32(0x5240d0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486b48; continue; }
  pc = 0x486b33; continue;
 }
 0x486b33 => {
  { let v = 0x5240d0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486b48); cpu.call(t);
  pc = 0x486b48; continue;
 }
 0x486b48 => {
  pc = 0x486b4d; continue;
 }
 0x486b4d => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486b56); f_00486b67(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486b62); f_00486b74(cpu);
  pc = 0x486b7f; continue;
 }
 0x486b7f => {
  pc = 0x486b8e; continue;
 }
 0x486b8e => {
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
pub fn f_00486b67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486b67); }
 let mut pc: u32 = 0x486b67;
 loop { match pc {
 0x486b67 => {
  { let v: u32 = 0x5240d8u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486b73); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486b74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486b74); }
 let mut pc: u32 = 0x486b74;
 loop { match pc {
 0x486b74 => {
  { let v: u32 = 0x5240d0u32; cpu.ecx = v; }
  cpu.push(0x486b7e); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486b84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486b84); }
 let mut pc: u32 = 0x486b84;
 loop { match pc {
 0x486b84 => {
  { let v: u32 = 0x4b8e18u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486b9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486b9c); }
 let mut pc: u32 = 0x486b9c;
 loop { match pc {
 0x486b9c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486c2au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1a0cu32; cpu.w32(0x524098u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486bee; continue; }
  pc = 0x486bd9; continue;
 }
 0x486bd9 => {
  { let v = 0x524098u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486bee); cpu.call(t);
  pc = 0x486bee; continue;
 }
 0x486bee => {
  pc = 0x486bf3; continue;
 }
 0x486bf3 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486bfc); f_00486c0d(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486c08); f_00486c1a(cpu);
  pc = 0x486c25; continue;
 }
 0x486c25 => {
  pc = 0x486c34; continue;
 }
 0x486c34 => {
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
pub fn f_00486c0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486c0d); }
 let mut pc: u32 = 0x486c0d;
 loop { match pc {
 0x486c0d => {
  { let v: u32 = 0x5240a0u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486c19); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486c1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486c1a); }
 let mut pc: u32 = 0x486c1a;
 loop { match pc {
 0x486c1a => {
  { let v: u32 = 0x524098u32; cpu.ecx = v; }
  cpu.push(0x486c24); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486c2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486c2a); }
 let mut pc: u32 = 0x486c2a;
 loop { match pc {
 0x486c2a => {
  { let v: u32 = 0x4b8e48u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486c42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486c42); }
 let mut pc: u32 = 0x486c42;
 loop { match pc {
 0x486c42 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486cd0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1a2cu32; cpu.w32(0x524138u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486c94; continue; }
  pc = 0x486c7f; continue;
 }
 0x486c7f => {
  { let v = 0x524138u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486c94); cpu.call(t);
  pc = 0x486c94; continue;
 }
 0x486c94 => {
  pc = 0x486c99; continue;
 }
 0x486c99 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486ca2); f_00486cb3(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486cae); f_00486cc0(cpu);
  pc = 0x486ccb; continue;
 }
 0x486ccb => {
  pc = 0x486cda; continue;
 }
 0x486cda => {
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
pub fn f_00486cb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486cb3); }
 let mut pc: u32 = 0x486cb3;
 loop { match pc {
 0x486cb3 => {
  { let v: u32 = 0x524140u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486cbf); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486cc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486cc0); }
 let mut pc: u32 = 0x486cc0;
 loop { match pc {
 0x486cc0 => {
  { let v: u32 = 0x524138u32; cpu.ecx = v; }
  cpu.push(0x486cca); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486cd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486cd0); }
 let mut pc: u32 = 0x486cd0;
 loop { match pc {
 0x486cd0 => {
  { let v: u32 = 0x4b8e78u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486ce8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486ce8); }
 let mut pc: u32 = 0x486ce8;
 loop { match pc {
 0x486ce8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486d76u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1a4cu32; cpu.w32(0x524298u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486d3a; continue; }
  pc = 0x486d25; continue;
 }
 0x486d25 => {
  { let v = 0x524298u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486d3a); cpu.call(t);
  pc = 0x486d3a; continue;
 }
 0x486d3a => {
  pc = 0x486d3f; continue;
 }
 0x486d3f => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486d48); f_00486d59(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486d54); f_00486d66(cpu);
  pc = 0x486d71; continue;
 }
 0x486d71 => {
  pc = 0x486d80; continue;
 }
 0x486d80 => {
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
pub fn f_00486d59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486d59); }
 let mut pc: u32 = 0x486d59;
 loop { match pc {
 0x486d59 => {
  { let v: u32 = 0x5242a0u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486d65); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486d66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486d66); }
 let mut pc: u32 = 0x486d66;
 loop { match pc {
 0x486d66 => {
  { let v: u32 = 0x524298u32; cpu.ecx = v; }
  cpu.push(0x486d70); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486d76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486d76); }
 let mut pc: u32 = 0x486d76;
 loop { match pc {
 0x486d76 => {
  { let v: u32 = 0x4b8ea8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486d8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486d8e); }
 let mut pc: u32 = 0x486d8e;
 loop { match pc {
 0x486d8e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486e1cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1a6cu32; cpu.w32(0x524170u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486de0; continue; }
  pc = 0x486dcb; continue;
 }
 0x486dcb => {
  { let v = 0x524170u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486de0); cpu.call(t);
  pc = 0x486de0; continue;
 }
 0x486de0 => {
  pc = 0x486de5; continue;
 }
 0x486de5 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486dee); f_00486dff(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486dfa); f_00486e0c(cpu);
  pc = 0x486e17; continue;
 }
 0x486e17 => {
  pc = 0x486e26; continue;
 }
 0x486e26 => {
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
pub fn f_00486dff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486dff); }
 let mut pc: u32 = 0x486dff;
 loop { match pc {
 0x486dff => {
  { let v: u32 = 0x524178u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486e0b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486e0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486e0c); }
 let mut pc: u32 = 0x486e0c;
 loop { match pc {
 0x486e0c => {
  { let v: u32 = 0x524170u32; cpu.ecx = v; }
  cpu.push(0x486e16); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486e1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486e1c); }
 let mut pc: u32 = 0x486e1c;
 loop { match pc {
 0x486e1c => {
  { let v: u32 = 0x4b8ed8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486e34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486e34); }
 let mut pc: u32 = 0x486e34;
 loop { match pc {
 0x486e34 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486ec2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1a8cu32; cpu.w32(0x524268u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486e86; continue; }
  pc = 0x486e71; continue;
 }
 0x486e71 => {
  { let v = 0x524268u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486e86); cpu.call(t);
  pc = 0x486e86; continue;
 }
 0x486e86 => {
  pc = 0x486e8b; continue;
 }
 0x486e8b => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486e94); f_00486ea5(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486ea0); f_00486eb2(cpu);
  pc = 0x486ebd; continue;
 }
 0x486ebd => {
  pc = 0x486ecc; continue;
 }
 0x486ecc => {
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
pub fn f_00486ea5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486ea5); }
 let mut pc: u32 = 0x486ea5;
 loop { match pc {
 0x486ea5 => {
  { let v: u32 = 0x524270u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486eb1); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486eb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486eb2); }
 let mut pc: u32 = 0x486eb2;
 loop { match pc {
 0x486eb2 => {
  { let v: u32 = 0x524268u32; cpu.ecx = v; }
  cpu.push(0x486ebc); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486ec2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486ec2); }
 let mut pc: u32 = 0x486ec2;
 loop { match pc {
 0x486ec2 => {
  { let v: u32 = 0x4b8f08u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486eda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486eda); }
 let mut pc: u32 = 0x486eda;
 loop { match pc {
 0x486eda => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x486f68u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1aacu32; cpu.w32(0x524278u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486f2c; continue; }
  pc = 0x486f17; continue;
 }
 0x486f17 => {
  { let v = 0x524278u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486f2c); cpu.call(t);
  pc = 0x486f2c; continue;
 }
 0x486f2c => {
  pc = 0x486f31; continue;
 }
 0x486f31 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486f3a); f_00486f4b(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486f46); f_00486f58(cpu);
  pc = 0x486f63; continue;
 }
 0x486f63 => {
  pc = 0x486f72; continue;
 }
 0x486f72 => {
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
pub fn f_00486f4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486f4b); }
 let mut pc: u32 = 0x486f4b;
 loop { match pc {
 0x486f4b => {
  { let v: u32 = 0x524280u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486f57); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486f58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486f58); }
 let mut pc: u32 = 0x486f58;
 loop { match pc {
 0x486f58 => {
  { let v: u32 = 0x524278u32; cpu.ecx = v; }
  cpu.push(0x486f62); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486f68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486f68); }
 let mut pc: u32 = 0x486f68;
 loop { match pc {
 0x486f68 => {
  { let v: u32 = 0x4b8f38u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486f80); }
 let mut pc: u32 = 0x486f80;
 loop { match pc {
 0x486f80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48700eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1accu32; cpu.w32(0x5240c0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x486fd2; continue; }
  pc = 0x486fbd; continue;
 }
 0x486fbd => {
  { let v = 0x5240c0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x486fd2); cpu.call(t);
  pc = 0x486fd2; continue;
 }
 0x486fd2 => {
  pc = 0x486fd7; continue;
 }
 0x486fd7 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486fe0); f_00486ff1(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x486fec); f_00486ffe(cpu);
  pc = 0x487009; continue;
 }
 0x487009 => {
  pc = 0x487018; continue;
 }
 0x487018 => {
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
pub fn f_00486ff1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486ff1); }
 let mut pc: u32 = 0x486ff1;
 loop { match pc {
 0x486ff1 => {
  { let v: u32 = 0x5240c8u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x486ffd); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00486ffe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x486ffe); }
 let mut pc: u32 = 0x486ffe;
 loop { match pc {
 0x486ffe => {
  { let v: u32 = 0x5240c0u32; cpu.ecx = v; }
  cpu.push(0x487008); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048700e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48700e); }
 let mut pc: u32 = 0x48700e;
 loop { match pc {
 0x48700e => {
  { let v: u32 = 0x4b8f68u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487026(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487026); }
 let mut pc: u32 = 0x487026;
 loop { match pc {
 0x487026 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4870b4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1aecu32; cpu.w32(0x5241a0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x487078; continue; }
  pc = 0x487063; continue;
 }
 0x487063 => {
  { let v = 0x5241a0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x487078); cpu.call(t);
  pc = 0x487078; continue;
 }
 0x487078 => {
  pc = 0x48707d; continue;
 }
 0x48707d => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487086); f_00487097(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487092); f_004870a4(cpu);
  pc = 0x4870af; continue;
 }
 0x4870af => {
  pc = 0x4870be; continue;
 }
 0x4870be => {
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
pub fn f_00487097(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487097); }
 let mut pc: u32 = 0x487097;
 loop { match pc {
 0x487097 => {
  { let v: u32 = 0x5241a8u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4870a3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004870a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4870a4); }
 let mut pc: u32 = 0x4870a4;
 loop { match pc {
 0x4870a4 => {
  { let v: u32 = 0x5241a0u32; cpu.ecx = v; }
  cpu.push(0x4870ae); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004870b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4870b4); }
 let mut pc: u32 = 0x4870b4;
 loop { match pc {
 0x4870b4 => {
  { let v: u32 = 0x4b8f98u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004870cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4870cc); }
 let mut pc: u32 = 0x4870cc;
 loop { match pc {
 0x4870cc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48715au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1b0cu32; cpu.w32(0x524158u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x48711e; continue; }
  pc = 0x487109; continue;
 }
 0x487109 => {
  { let v = 0x524158u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(0x48711e); cpu.call(t);
  pc = 0x48711e; continue;
 }
 0x48711e => {
  pc = 0x487123; continue;
 }
 0x487123 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48712c); f_0048713d(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487138); f_0048714a(cpu);
  pc = 0x487155; continue;
 }
 0x487155 => {
  pc = 0x487164; continue;
 }
 0x487164 => {
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
pub fn f_0048713d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48713d); }
 let mut pc: u32 = 0x48713d;
 loop { match pc {
 0x48713d => {
  { let v: u32 = 0x524160u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487149); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048714a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48714a); }
 let mut pc: u32 = 0x48714a;
 loop { match pc {
 0x48714a => {
  { let v: u32 = 0x524158u32; cpu.ecx = v; }
  cpu.push(0x487154); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048715a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48715a); }
 let mut pc: u32 = 0x48715a;
 loop { match pc {
 0x48715a => {
  { let v: u32 = 0x4b8fc8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487172(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487172); }
 let mut pc: u32 = 0x487172;
 loop { match pc {
 0x487172 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4871ebu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1b2cu32; cpu.w32(0x524200u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524200u32; cpu.push(v); }
  cpu.push(0x4871ac); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4871b4; continue;
 }
 0x4871b4 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4871bd); f_004871ce(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4871c9); f_004871db(cpu);
  pc = 0x4871e6; continue;
 }
 0x4871e6 => {
  pc = 0x4871f5; continue;
 }
 0x4871f5 => {
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
pub fn f_004871ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4871ce); }
 let mut pc: u32 = 0x4871ce;
 loop { match pc {
 0x4871ce => {
  { let v: u32 = 0x524204u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4871da); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004871db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4871db); }
 let mut pc: u32 = 0x4871db;
 loop { match pc {
 0x4871db => {
  { let v: u32 = 0x524200u32; cpu.ecx = v; }
  cpu.push(0x4871e5); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004871eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4871eb); }
 let mut pc: u32 = 0x4871eb;
 loop { match pc {
 0x4871eb => {
  { let v: u32 = 0x4b8ff8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487203(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487203); }
 let mut pc: u32 = 0x487203;
 loop { match pc {
 0x487203 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48727cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1b3cu32; cpu.w32(0x5241b0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x5241b0u32; cpu.push(v); }
  cpu.push(0x48723d); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487245; continue;
 }
 0x487245 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48724e); f_0048725f(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48725a); f_0048726c(cpu);
  pc = 0x487277; continue;
 }
 0x487277 => {
  pc = 0x487286; continue;
 }
 0x487286 => {
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
pub fn f_0048725f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48725f); }
 let mut pc: u32 = 0x48725f;
 loop { match pc {
 0x48725f => {
  { let v: u32 = 0x5241b4u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48726b); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048726c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48726c); }
 let mut pc: u32 = 0x48726c;
 loop { match pc {
 0x48726c => {
  { let v: u32 = 0x5241b0u32; cpu.ecx = v; }
  cpu.push(0x487276); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048727c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48727c); }
 let mut pc: u32 = 0x48727c;
 loop { match pc {
 0x48727c => {
  { let v: u32 = 0x4b9028u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487294(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487294); }
 let mut pc: u32 = 0x487294;
 loop { match pc {
 0x487294 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48730du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1b4cu32; cpu.w32(0x524088u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524088u32; cpu.push(v); }
  cpu.push(0x4872ce); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4872d6; continue;
 }
 0x4872d6 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4872df); f_004872f0(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4872eb); f_004872fd(cpu);
  pc = 0x487308; continue;
 }
 0x487308 => {
  pc = 0x487317; continue;
 }
 0x487317 => {
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
pub fn f_004872f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4872f0); }
 let mut pc: u32 = 0x4872f0;
 loop { match pc {
 0x4872f0 => {
  { let v: u32 = 0x52408cu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4872fc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004872fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4872fd); }
 let mut pc: u32 = 0x4872fd;
 loop { match pc {
 0x4872fd => {
  { let v: u32 = 0x524088u32; cpu.ecx = v; }
  cpu.push(0x487307); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048730d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48730d); }
 let mut pc: u32 = 0x48730d;
 loop { match pc {
 0x48730d => {
  { let v: u32 = 0x4b9058u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487325(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487325); }
 let mut pc: u32 = 0x487325;
 loop { match pc {
 0x487325 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48739eu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1b5cu32; cpu.w32(0x5241f0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x5241f0u32; cpu.push(v); }
  cpu.push(0x48735f); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487367; continue;
 }
 0x487367 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487370); f_00487381(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48737c); f_0048738e(cpu);
  pc = 0x487399; continue;
 }
 0x487399 => {
  pc = 0x4873a8; continue;
 }
 0x4873a8 => {
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
pub fn f_00487381(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487381); }
 let mut pc: u32 = 0x487381;
 loop { match pc {
 0x487381 => {
  { let v: u32 = 0x5241f4u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48738d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048738e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48738e); }
 let mut pc: u32 = 0x48738e;
 loop { match pc {
 0x48738e => {
  { let v: u32 = 0x5241f0u32; cpu.ecx = v; }
  cpu.push(0x487398); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048739e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48739e); }
 let mut pc: u32 = 0x48739e;
 loop { match pc {
 0x48739e => {
  { let v: u32 = 0x4b9088u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004873b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4873b6); }
 let mut pc: u32 = 0x4873b6;
 loop { match pc {
 0x4873b6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48742fu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1b6cu32; cpu.w32(0x524230u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524230u32; cpu.push(v); }
  cpu.push(0x4873f0); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4873f8; continue;
 }
 0x4873f8 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487401); f_00487412(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48740d); f_0048741f(cpu);
  pc = 0x48742a; continue;
 }
 0x48742a => {
  pc = 0x487439; continue;
 }
 0x487439 => {
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
pub fn f_00487412(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487412); }
 let mut pc: u32 = 0x487412;
 loop { match pc {
 0x487412 => {
  { let v: u32 = 0x524234u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48741e); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048741f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48741f); }
 let mut pc: u32 = 0x48741f;
 loop { match pc {
 0x48741f => {
  { let v: u32 = 0x524230u32; cpu.ecx = v; }
  cpu.push(0x487429); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048742f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48742f); }
 let mut pc: u32 = 0x48742f;
 loop { match pc {
 0x48742f => {
  { let v: u32 = 0x4b90b8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487447(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487447); }
 let mut pc: u32 = 0x487447;
 loop { match pc {
 0x487447 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4874c0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b1b7cu32; cpu.w32(0x5241e0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x5241e0u32; cpu.push(v); }
  cpu.push(0x487481); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487489; continue;
 }
 0x487489 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487492); f_004874a3(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48749e); f_004874b0(cpu);
  pc = 0x4874bb; continue;
 }
 0x4874bb => {
  pc = 0x4874ca; continue;
 }
 0x4874ca => {
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
pub fn f_004874a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4874a3); }
 let mut pc: u32 = 0x4874a3;
 loop { match pc {
 0x4874a3 => {
  { let v: u32 = 0x5241e4u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4874af); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
