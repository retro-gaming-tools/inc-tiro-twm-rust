#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_004874b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4874b0); }
 let mut pc: u32 = 0x4874b0;
 loop { match pc {
 0x4874b0 => {
  { let v: u32 = 0x5241e0u32; cpu.ecx = v; }
  cpu.push(0x4874ba); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004874c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4874c0); }
 let mut pc: u32 = 0x4874c0;
 loop { match pc {
 0x4874c0 => {
  { let v: u32 = 0x4b90e8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004874d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4874d8); }
 let mut pc: u32 = 0x4874d8;
 loop { match pc {
 0x4874d8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487551u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b189cu32; cpu.w32(0x5240e8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x5240e8u32; cpu.push(v); }
  cpu.push(0x487512); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x48751a; continue;
 }
 0x48751a => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487523); f_00487534(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48752f); f_00487541(cpu);
  pc = 0x48754c; continue;
 }
 0x48754c => {
  pc = 0x48755b; continue;
 }
 0x48755b => {
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
pub fn f_00487534(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487534); }
 let mut pc: u32 = 0x487534;
 loop { match pc {
 0x487534 => {
  { let v: u32 = 0x5240ecu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487540); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487541(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487541); }
 let mut pc: u32 = 0x487541;
 loop { match pc {
 0x487541 => {
  { let v: u32 = 0x5240e8u32; cpu.ecx = v; }
  cpu.push(0x48754b); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487551(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487551); }
 let mut pc: u32 = 0x487551;
 loop { match pc {
 0x487551 => {
  { let v: u32 = 0x4b9118u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487569(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487569); }
 let mut pc: u32 = 0x487569;
 loop { match pc {
 0x487569 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4875e2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b18bcu32; cpu.w32(0x5241c0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x5241c0u32; cpu.push(v); }
  cpu.push(0x4875a3); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4875ab; continue;
 }
 0x4875ab => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4875b4); f_004875c5(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4875c0); f_004875d2(cpu);
  pc = 0x4875dd; continue;
 }
 0x4875dd => {
  pc = 0x4875ec; continue;
 }
 0x4875ec => {
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
pub fn f_004875c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4875c5); }
 let mut pc: u32 = 0x4875c5;
 loop { match pc {
 0x4875c5 => {
  { let v: u32 = 0x5241c4u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4875d1); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004875d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4875d2); }
 let mut pc: u32 = 0x4875d2;
 loop { match pc {
 0x4875d2 => {
  { let v: u32 = 0x5241c0u32; cpu.ecx = v; }
  cpu.push(0x4875dc); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004875e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4875e2); }
 let mut pc: u32 = 0x4875e2;
 loop { match pc {
 0x4875e2 => {
  { let v: u32 = 0x4b9148u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004875fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4875fa); }
 let mut pc: u32 = 0x4875fa;
 loop { match pc {
 0x4875fa => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487673u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b18ccu32; cpu.w32(0x5241d0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x5241d0u32; cpu.push(v); }
  cpu.push(0x487634); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x48763c; continue;
 }
 0x48763c => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487645); f_00487656(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487651); f_00487663(cpu);
  pc = 0x48766e; continue;
 }
 0x48766e => {
  pc = 0x48767d; continue;
 }
 0x48767d => {
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
pub fn f_00487656(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487656); }
 let mut pc: u32 = 0x487656;
 loop { match pc {
 0x487656 => {
  { let v: u32 = 0x5241d4u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487662); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487663(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487663); }
 let mut pc: u32 = 0x487663;
 loop { match pc {
 0x487663 => {
  { let v: u32 = 0x5241d0u32; cpu.ecx = v; }
  cpu.push(0x48766d); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487673(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487673); }
 let mut pc: u32 = 0x487673;
 loop { match pc {
 0x487673 => {
  { let v: u32 = 0x4b9178u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048768b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48768b); }
 let mut pc: u32 = 0x48768b;
 loop { match pc {
 0x48768b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487704u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b18dcu32; cpu.w32(0x524180u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524180u32; cpu.push(v); }
  cpu.push(0x4876c5); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4876cd; continue;
 }
 0x4876cd => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4876d6); f_004876e7(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4876e2); f_004876f4(cpu);
  pc = 0x4876ff; continue;
 }
 0x4876ff => {
  pc = 0x48770e; continue;
 }
 0x48770e => {
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
pub fn f_004876e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4876e7); }
 let mut pc: u32 = 0x4876e7;
 loop { match pc {
 0x4876e7 => {
  { let v: u32 = 0x524184u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4876f3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004876f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4876f4); }
 let mut pc: u32 = 0x4876f4;
 loop { match pc {
 0x4876f4 => {
  { let v: u32 = 0x524180u32; cpu.ecx = v; }
  cpu.push(0x4876fe); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487704(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487704); }
 let mut pc: u32 = 0x487704;
 loop { match pc {
 0x487704 => {
  { let v: u32 = 0x4b91a8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048771c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48771c); }
 let mut pc: u32 = 0x48771c;
 loop { match pc {
 0x48771c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487795u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b18ecu32; cpu.w32(0x524148u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524148u32; cpu.push(v); }
  cpu.push(0x487756); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x48775e; continue;
 }
 0x48775e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487767); f_00487778(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487773); f_00487785(cpu);
  pc = 0x487790; continue;
 }
 0x487790 => {
  pc = 0x48779f; continue;
 }
 0x48779f => {
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
pub fn f_00487778(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487778); }
 let mut pc: u32 = 0x487778;
 loop { match pc {
 0x487778 => {
  { let v: u32 = 0x52414cu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487784); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487785(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487785); }
 let mut pc: u32 = 0x487785;
 loop { match pc {
 0x487785 => {
  { let v: u32 = 0x524148u32; cpu.ecx = v; }
  cpu.push(0x48778f); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487795(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487795); }
 let mut pc: u32 = 0x487795;
 loop { match pc {
 0x487795 => {
  { let v: u32 = 0x4b91d8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004877ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4877ad); }
 let mut pc: u32 = 0x4877ad;
 loop { match pc {
 0x4877ad => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487826u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b18fcu32; cpu.w32(0x524190u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524190u32; cpu.push(v); }
  cpu.push(0x4877e7); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4877ef; continue;
 }
 0x4877ef => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4877f8); f_00487809(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487804); f_00487816(cpu);
  pc = 0x487821; continue;
 }
 0x487821 => {
  pc = 0x487830; continue;
 }
 0x487830 => {
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
pub fn f_00487809(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487809); }
 let mut pc: u32 = 0x487809;
 loop { match pc {
 0x487809 => {
  { let v: u32 = 0x524194u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487815); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487816(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487816); }
 let mut pc: u32 = 0x487816;
 loop { match pc {
 0x487816 => {
  { let v: u32 = 0x524190u32; cpu.ecx = v; }
  cpu.push(0x487820); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487826(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487826); }
 let mut pc: u32 = 0x487826;
 loop { match pc {
 0x487826 => {
  { let v: u32 = 0x4b9208u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048783e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48783e); }
 let mut pc: u32 = 0x48783e;
 loop { match pc {
 0x48783e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4878b7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b190cu32; cpu.w32(0x524118u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524118u32; cpu.push(v); }
  cpu.push(0x487878); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487880; continue;
 }
 0x487880 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487889); f_0048789a(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487895); f_004878a7(cpu);
  pc = 0x4878b2; continue;
 }
 0x4878b2 => {
  pc = 0x4878c1; continue;
 }
 0x4878c1 => {
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
pub fn f_0048789a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48789a); }
 let mut pc: u32 = 0x48789a;
 loop { match pc {
 0x48789a => {
  { let v: u32 = 0x52411cu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4878a6); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004878a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4878a7); }
 let mut pc: u32 = 0x4878a7;
 loop { match pc {
 0x4878a7 => {
  { let v: u32 = 0x524118u32; cpu.ecx = v; }
  cpu.push(0x4878b1); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004878b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4878b7); }
 let mut pc: u32 = 0x4878b7;
 loop { match pc {
 0x4878b7 => {
  { let v: u32 = 0x4b9238u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004878cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4878cf); }
 let mut pc: u32 = 0x4878cf;
 loop { match pc {
 0x4878cf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487948u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b192cu32; cpu.w32(0x524220u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524220u32; cpu.push(v); }
  cpu.push(0x487909); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487911; continue;
 }
 0x487911 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48791a); f_0048792b(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487926); f_00487938(cpu);
  pc = 0x487943; continue;
 }
 0x487943 => {
  pc = 0x487952; continue;
 }
 0x487952 => {
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
pub fn f_0048792b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48792b); }
 let mut pc: u32 = 0x48792b;
 loop { match pc {
 0x48792b => {
  { let v: u32 = 0x524224u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487937); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487938); }
 let mut pc: u32 = 0x487938;
 loop { match pc {
 0x487938 => {
  { let v: u32 = 0x524220u32; cpu.ecx = v; }
  cpu.push(0x487942); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487948(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487948); }
 let mut pc: u32 = 0x487948;
 loop { match pc {
 0x487948 => {
  { let v: u32 = 0x4b9268u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487960); }
 let mut pc: u32 = 0x487960;
 loop { match pc {
 0x487960 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4879d9u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b193cu32; cpu.w32(0x5240f8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x5240f8u32; cpu.push(v); }
  cpu.push(0x48799a); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4879a2; continue;
 }
 0x4879a2 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4879ab); f_004879bc(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4879b7); f_004879c9(cpu);
  pc = 0x4879d4; continue;
 }
 0x4879d4 => {
  pc = 0x4879e3; continue;
 }
 0x4879e3 => {
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
pub fn f_004879bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4879bc); }
 let mut pc: u32 = 0x4879bc;
 loop { match pc {
 0x4879bc => {
  { let v: u32 = 0x5240fcu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4879c8); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004879c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4879c9); }
 let mut pc: u32 = 0x4879c9;
 loop { match pc {
 0x4879c9 => {
  { let v: u32 = 0x5240f8u32; cpu.ecx = v; }
  cpu.push(0x4879d3); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004879d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4879d9); }
 let mut pc: u32 = 0x4879d9;
 loop { match pc {
 0x4879d9 => {
  { let v: u32 = 0x4b9298u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004879f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4879f1); }
 let mut pc: u32 = 0x4879f1;
 loop { match pc {
 0x4879f1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487a6au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b188cu32; cpu.w32(0x524258u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524258u32; cpu.push(v); }
  cpu.push(0x487a2b); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487a33; continue;
 }
 0x487a33 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487a3c); f_00487a4d(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487a48); f_00487a5a(cpu);
  pc = 0x487a65; continue;
 }
 0x487a65 => {
  pc = 0x487a74; continue;
 }
 0x487a74 => {
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
pub fn f_00487a4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487a4d); }
 let mut pc: u32 = 0x487a4d;
 loop { match pc {
 0x487a4d => {
  { let v: u32 = 0x52425cu32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487a59); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487a5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487a5a); }
 let mut pc: u32 = 0x487a5a;
 loop { match pc {
 0x487a5a => {
  { let v: u32 = 0x524258u32; cpu.ecx = v; }
  cpu.push(0x487a64); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487a6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487a6a); }
 let mut pc: u32 = 0x487a6a;
 loop { match pc {
 0x487a6a => {
  { let v: u32 = 0x4b92c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487a82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487a82); }
 let mut pc: u32 = 0x487a82;
 loop { match pc {
 0x487a82 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487afbu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4b18acu32; cpu.w32(0x524210u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x524210u32; cpu.push(v); }
  cpu.push(0x487abc); f_004af3fb(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487ac4; continue;
 }
 0x487ac4 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487acd); f_00487ade(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x487ad9); f_00487aeb(cpu);
  pc = 0x487af6; continue;
 }
 0x487af6 => {
  pc = 0x487b05; continue;
 }
 0x487b05 => {
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
pub fn f_00487ade(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487ade); }
 let mut pc: u32 = 0x487ade;
 loop { match pc {
 0x487ade => {
  { let v: u32 = 0x524214u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487aea); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487aeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487aeb); }
 let mut pc: u32 = 0x487aeb;
 loop { match pc {
 0x487aeb => {
  { let v: u32 = 0x524210u32; cpu.ecx = v; }
  cpu.push(0x487af5); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487afb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487afb); }
 let mut pc: u32 = 0x487afb;
 loop { match pc {
 0x487afb => {
  { let v: u32 = 0x4b92f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487b13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487b13); }
 let mut pc: u32 = 0x487b13;
 loop { match pc {
 0x487b13 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487b45; continue; }
  pc = 0x487b34; continue;
 }
 0x487b34 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x487b40); cpu.call(t);
  pc = 0x487b45; continue;
 }
 0x487b45 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4f68f0u32, v); }
  pc = 0x487b54; continue;
 }
 0x487b54 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487be7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487be7); }
 let mut pc: u32 = 0x487be7;
 loop { match pc {
 0x487be7 => {
  { let v: u32 = 0x5240b4u32; cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x487bf3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487bf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487bf4); }
 let mut pc: u32 = 0x487bf4;
 loop { match pc {
 0x487bf4 => {
  { let v: u32 = 0x5240a8u32; cpu.ecx = v; }
  cpu.push(0x487bfe); f_00487ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487c20); }
 let mut pc: u32 = 0x487c20;
 loop { match pc {
 0x487c20 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x487c3a; continue;
 }
 0x487c3a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487c40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487c40); }
 let mut pc: u32 = 0x487c40;
 loop { match pc {
 0x487c40 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x487c5a; continue;
 }
 0x487c5a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x487c70; continue; }
  pc = 0x487c64; continue;
 }
 0x487c64 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x487c6d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487c70; continue;
 }
 0x487c70 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x487c78; continue;
 }
 0x487c78 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487c80); }
 let mut pc: u32 = 0x487c80;
 loop { match pc {
 0x487c80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x487c9a; continue;
 }
 0x487c9a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487ca0); }
 let mut pc: u32 = 0x487ca0;
 loop { match pc {
 0x487ca0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x487cba; continue;
 }
 0x487cba => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x487cd0; continue; }
  pc = 0x487cc4; continue;
 }
 0x487cc4 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x487ccd); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487cd0; continue;
 }
 0x487cd0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x487cd8; continue;
 }
 0x487cd8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487ce0); }
 let mut pc: u32 = 0x487ce0;
 loop { match pc {
 0x487ce0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b8cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x487cfa; continue;
 }
 0x487cfa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487d00); }
 let mut pc: u32 = 0x487d00;
 loop { match pc {
 0x487d00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b8cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x487d1a; continue;
 }
 0x487d1a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x487d30; continue; }
  pc = 0x487d24; continue;
 }
 0x487d24 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x487d2d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x487d30; continue;
 }
 0x487d30 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x487d38; continue;
 }
 0x487d38 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487d40); }
 let mut pc: u32 = 0x487d40;
 loop { match pc {
 0x487d40 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487e07u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x18u32; cpu.push(v); }
  cpu.push(0x487d68); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487d8f; continue; }
  pc = 0x487d7f; continue;
 }
 0x487d7f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x487d87); f_0048453d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487d96; continue;
 }
 0x487d8f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487d96; continue;
 }
 0x487d96 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487df2; continue; }
  pc = 0x487dad; continue;
 }
 0x487dad => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x487dbc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x487df2; continue; }
  pc = 0x487dc4; continue;
 }
 0x487dc4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487deb; continue; }
  pc = 0x487dda; continue;
 }
 0x487dda => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x487de6); cpu.call(t);
  pc = 0x487deb; continue;
 }
 0x487deb => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487df2; continue;
 }
 0x487df2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x487e11; continue;
 }
 0x487e11 => {
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
pub fn f_00487dfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487dfa); }
 let mut pc: u32 = 0x487dfa;
 loop { match pc {
 0x487dfa => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x487e03); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487e07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487e07); }
 let mut pc: u32 = 0x487e07;
 loop { match pc {
 0x487e07 => {
  { let v: u32 = 0x4b9358u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487e30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487e30); }
 let mut pc: u32 = 0x487e30;
 loop { match pc {
 0x487e30 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x487ef7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6cu32; cpu.push(v); }
  cpu.push(0x487e58); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487e7f; continue; }
  pc = 0x487e6f; continue;
 }
 0x487e6f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x487e77); f_00484033(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487e86; continue;
 }
 0x487e7f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487e86; continue;
 }
 0x487e86 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487ee2; continue; }
  pc = 0x487e9d; continue;
 }
 0x487e9d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x487eac); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x487ee2; continue; }
  pc = 0x487eb4; continue;
 }
 0x487eb4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487edb; continue; }
  pc = 0x487eca; continue;
 }
 0x487eca => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x487ed6); cpu.call(t);
  pc = 0x487edb; continue;
 }
 0x487edb => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487ee2; continue;
 }
 0x487ee2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x487f01; continue;
 }
 0x487f01 => {
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
pub fn f_00487eea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487eea); }
 let mut pc: u32 = 0x487eea;
 loop { match pc {
 0x487eea => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x487ef3); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487ef7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487ef7); }
 let mut pc: u32 = 0x487ef7;
 loop { match pc {
 0x487ef7 => {
  { let v: u32 = 0x4b9380u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00487f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x487f20); }
 let mut pc: u32 = 0x487f20;
 loop { match pc {
 0x487f20 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488037u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x68u32; cpu.push(v); }
  cpu.push(0x487f48); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487f71; continue; }
  pc = 0x487f5f; continue;
 }
 0x487f5f => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x487f69); f_004a1c92(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487f78; continue;
 }
 0x487f71 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x487f78; continue;
 }
 0x487f78 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488004; continue; }
  pc = 0x487f8f; continue;
 }
 0x487f8f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x487fb6); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488004; continue; }
  pc = 0x487fbe; continue;
 }
 0x487fbe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x487ffd; continue; }
  pc = 0x487fd4; continue;
 }
 0x487fd4 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x487ff8); cpu.call(t);
  pc = 0x487ffd; continue;
 }
 0x487ffd => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488004; continue;
 }
 0x488004 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x488015; continue; }
  pc = 0x48800e; continue;
 }
 0x48800e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x488025; continue;
 }
 0x488015 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x488025; continue;
 }
 0x488025 => {
  pc = 0x488041; continue;
 }
 0x488041 => {
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
pub fn f_0048802a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48802a); }
 let mut pc: u32 = 0x48802a;
 loop { match pc {
 0x48802a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488033); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488037(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488037); }
 let mut pc: u32 = 0x488037;
 loop { match pc {
 0x488037 => {
  { let v: u32 = 0x4b93a8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488060); }
 let mut pc: u32 = 0x488060;
 loop { match pc {
 0x488060 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488177u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x40u32; cpu.push(v); }
  cpu.push(0x488088); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4880b1; continue; }
  pc = 0x48809f; continue;
 }
 0x48809f => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4880a9); f_004a6fd8(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4880b8; continue;
 }
 0x4880b1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4880b8; continue;
 }
 0x4880b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488144; continue; }
  pc = 0x4880cf; continue;
 }
 0x4880cf => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x4880f6); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488144; continue; }
  pc = 0x4880fe; continue;
 }
 0x4880fe => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48813d; continue; }
  pc = 0x488114; continue;
 }
 0x488114 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x488138); cpu.call(t);
  pc = 0x48813d; continue;
 }
 0x48813d => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488144; continue;
 }
 0x488144 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x488155; continue; }
  pc = 0x48814e; continue;
 }
 0x48814e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x488165; continue;
 }
 0x488155 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x488165; continue;
 }
 0x488165 => {
  pc = 0x488181; continue;
 }
 0x488181 => {
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
pub fn f_0048816a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48816a); }
 let mut pc: u32 = 0x48816a;
 loop { match pc {
 0x48816a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488173); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488177(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488177); }
 let mut pc: u32 = 0x488177;
 loop { match pc {
 0x488177 => {
  { let v: u32 = 0x4b93d0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004881a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4881a0); }
 let mut pc: u32 = 0x4881a0;
 loop { match pc {
 0x4881a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4882b7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x4cu32; cpu.push(v); }
  cpu.push(0x4881c8); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4881f1; continue; }
  pc = 0x4881df; continue;
 }
 0x4881df => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4881e9); f_004a76bf(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4881f8; continue;
 }
 0x4881f1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4881f8; continue;
 }
 0x4881f8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488284; continue; }
  pc = 0x48820f; continue;
 }
 0x48820f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x488236); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488284; continue; }
  pc = 0x48823e; continue;
 }
 0x48823e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48827d; continue; }
  pc = 0x488254; continue;
 }
 0x488254 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x488278); cpu.call(t);
  pc = 0x48827d; continue;
 }
 0x48827d => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488284; continue;
 }
 0x488284 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x488295; continue; }
  pc = 0x48828e; continue;
 }
 0x48828e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4882a5; continue;
 }
 0x488295 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x4882a5; continue;
 }
 0x4882a5 => {
  pc = 0x4882c1; continue;
 }
 0x4882c1 => {
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
pub fn f_004882aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4882aa); }
 let mut pc: u32 = 0x4882aa;
 loop { match pc {
 0x4882aa => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4882b3); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004882b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4882b7); }
 let mut pc: u32 = 0x4882b7;
 loop { match pc {
 0x4882b7 => {
  { let v: u32 = 0x4b93f8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004882e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4882e0); }
 let mut pc: u32 = 0x4882e0;
 loop { match pc {
 0x4882e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4883fau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x84u32; cpu.push(v); }
  cpu.push(0x48830b); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488334; continue; }
  pc = 0x488322; continue;
 }
 0x488322 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48832c); f_004a7e0a(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48833b; continue;
 }
 0x488334 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48833b; continue;
 }
 0x48833b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4883c7; continue; }
  pc = 0x488352; continue;
 }
 0x488352 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x488379); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4883c7; continue; }
  pc = 0x488381; continue;
 }
 0x488381 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4883c0; continue; }
  pc = 0x488397; continue;
 }
 0x488397 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x4883bb); cpu.call(t);
  pc = 0x4883c0; continue;
 }
 0x4883c0 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4883c7; continue;
 }
 0x4883c7 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4883d8; continue; }
  pc = 0x4883d1; continue;
 }
 0x4883d1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4883e8; continue;
 }
 0x4883d8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x4883e8; continue;
 }
 0x4883e8 => {
  pc = 0x488404; continue;
 }
 0x488404 => {
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
pub fn f_004883ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4883ed); }
 let mut pc: u32 = 0x4883ed;
 loop { match pc {
 0x4883ed => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4883f6); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004883fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4883fa); }
 let mut pc: u32 = 0x4883fa;
 loop { match pc {
 0x4883fa => {
  { let v: u32 = 0x4b9420u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488420); }
 let mut pc: u32 = 0x488420;
 loop { match pc {
 0x488420 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4884e7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x488448); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48846f; continue; }
  pc = 0x48845f; continue;
 }
 0x48845f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x488467); f_004a86c7(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488476; continue;
 }
 0x48846f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488476; continue;
 }
 0x488476 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4884d2; continue; }
  pc = 0x48848d; continue;
 }
 0x48848d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x48849c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4884d2; continue; }
  pc = 0x4884a4; continue;
 }
 0x4884a4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4884cb; continue; }
  pc = 0x4884ba; continue;
 }
 0x4884ba => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4884c6); cpu.call(t);
  pc = 0x4884cb; continue;
 }
 0x4884cb => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4884d2; continue;
 }
 0x4884d2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4884f1; continue;
 }
 0x4884f1 => {
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
pub fn f_004884da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4884da); }
 let mut pc: u32 = 0x4884da;
 loop { match pc {
 0x4884da => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4884e3); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004884e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4884e7); }
 let mut pc: u32 = 0x4884e7;
 loop { match pc {
 0x4884e7 => {
  { let v: u32 = 0x4b9448u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488510); }
 let mut pc: u32 = 0x488510;
 loop { match pc {
 0x488510 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48862au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x84u32; cpu.push(v); }
  cpu.push(0x48853b); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488564; continue; }
  pc = 0x488552; continue;
 }
 0x488552 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48855c); f_004a8cb6(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48856b; continue;
 }
 0x488564 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48856b; continue;
 }
 0x48856b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4885f7; continue; }
  pc = 0x488582; continue;
 }
 0x488582 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x4885a9); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4885f7; continue; }
  pc = 0x4885b1; continue;
 }
 0x4885b1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4885f0; continue; }
  pc = 0x4885c7; continue;
 }
 0x4885c7 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x4885eb); cpu.call(t);
  pc = 0x4885f0; continue;
 }
 0x4885f0 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4885f7; continue;
 }
 0x4885f7 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x488608; continue; }
  pc = 0x488601; continue;
 }
 0x488601 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x488618; continue;
 }
 0x488608 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x488618; continue;
 }
 0x488618 => {
  pc = 0x488634; continue;
 }
 0x488634 => {
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
pub fn f_0048861d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48861d); }
 let mut pc: u32 = 0x48861d;
 loop { match pc {
 0x48861d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488626); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048862a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48862a); }
 let mut pc: u32 = 0x48862a;
 loop { match pc {
 0x48862a => {
  { let v: u32 = 0x4b9470u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488650); }
 let mut pc: u32 = 0x488650;
 loop { match pc {
 0x488650 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488717u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x488678); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48869f; continue; }
  pc = 0x48868f; continue;
 }
 0x48868f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x488697); f_004a9595(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4886a6; continue;
 }
 0x48869f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4886a6; continue;
 }
 0x4886a6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488702; continue; }
  pc = 0x4886bd; continue;
 }
 0x4886bd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x4886cc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488702; continue; }
  pc = 0x4886d4; continue;
 }
 0x4886d4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4886fb; continue; }
  pc = 0x4886ea; continue;
 }
 0x4886ea => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4886f6); cpu.call(t);
  pc = 0x4886fb; continue;
 }
 0x4886fb => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488702; continue;
 }
 0x488702 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x488721; continue;
 }
 0x488721 => {
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
pub fn f_0048870a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48870a); }
 let mut pc: u32 = 0x48870a;
 loop { match pc {
 0x48870a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488713); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488717(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488717); }
 let mut pc: u32 = 0x488717;
 loop { match pc {
 0x488717 => {
  { let v: u32 = 0x4b9498u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488740); }
 let mut pc: u32 = 0x488740;
 loop { match pc {
 0x488740 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48885au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x84u32; cpu.push(v); }
  cpu.push(0x48876b); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488794; continue; }
  pc = 0x488782; continue;
 }
 0x488782 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48878c); f_004a9bb9(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48879b; continue;
 }
 0x488794 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48879b; continue;
 }
 0x48879b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488827; continue; }
  pc = 0x4887b2; continue;
 }
 0x4887b2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x4887d9); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488827; continue; }
  pc = 0x4887e1; continue;
 }
 0x4887e1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488820; continue; }
  pc = 0x4887f7; continue;
 }
 0x4887f7 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x48881b); cpu.call(t);
  pc = 0x488820; continue;
 }
 0x488820 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488827; continue;
 }
 0x488827 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x488838; continue; }
  pc = 0x488831; continue;
 }
 0x488831 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x488848; continue;
 }
 0x488838 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x488848; continue;
 }
 0x488848 => {
  pc = 0x488864; continue;
 }
 0x488864 => {
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
pub fn f_0048884d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48884d); }
 let mut pc: u32 = 0x48884d;
 loop { match pc {
 0x48884d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488856); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048885a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48885a); }
 let mut pc: u32 = 0x48885a;
 loop { match pc {
 0x48885a => {
  { let v: u32 = 0x4b94c0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488880); }
 let mut pc: u32 = 0x488880;
 loop { match pc {
 0x488880 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488947u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x18u32; cpu.push(v); }
  cpu.push(0x4888a8); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4888cf; continue; }
  pc = 0x4888bf; continue;
 }
 0x4888bf => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4888c7); f_004aa4a2(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4888d6; continue;
 }
 0x4888cf => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4888d6; continue;
 }
 0x4888d6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488932; continue; }
  pc = 0x4888ed; continue;
 }
 0x4888ed => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x4888fc); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488932; continue; }
  pc = 0x488904; continue;
 }
 0x488904 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48892b; continue; }
  pc = 0x48891a; continue;
 }
 0x48891a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x488926); cpu.call(t);
  pc = 0x48892b; continue;
 }
 0x48892b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488932; continue;
 }
 0x488932 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x488951; continue;
 }
 0x488951 => {
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
pub fn f_0048893a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48893a); }
 let mut pc: u32 = 0x48893a;
 loop { match pc {
 0x48893a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488943); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488947(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488947); }
 let mut pc: u32 = 0x488947;
 loop { match pc {
 0x488947 => {
  { let v: u32 = 0x4b94e8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488970); }
 let mut pc: u32 = 0x488970;
 loop { match pc {
 0x488970 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488a56u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48899f; continue;
 }
 0x48899f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x4889b2); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b7cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f7330u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x4889cc); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x4889d6); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x4889e9); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4889fa); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488a03); f_00488a2c(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488a0c); f_004af36d(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488a18); f_00488a36(cpu);
  pc = 0x488a1d; continue;
 }
 0x488a1d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x488a60; continue;
 }
 0x488a60 => {
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
pub fn f_00488a2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488a2c); }
 let mut pc: u32 = 0x488a2c;
 loop { match pc {
 0x488a2c => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488a35); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488a36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488a36); }
 let mut pc: u32 = 0x488a36;
 loop { match pc {
 0x488a36 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488a3f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488a40); }
 let mut pc: u32 = 0x488a40;
 loop { match pc {
 0x488a40 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488a4c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488a4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488a4d); }
 let mut pc: u32 = 0x488a4d;
 loop { match pc {
 0x488a4d => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x488a55); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488a56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488a56); }
 let mut pc: u32 = 0x488a56;
 loop { match pc {
 0x488a56 => {
  { let v: u32 = 0x4b9510u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488a70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488a70); }
 let mut pc: u32 = 0x488a70;
 loop { match pc {
 0x488a70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488b8au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x84u32; cpu.push(v); }
  cpu.push(0x488a9b); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488ac4; continue; }
  pc = 0x488ab2; continue;
 }
 0x488ab2 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x488abc); f_004aab1e(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488acb; continue;
 }
 0x488ac4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488acb; continue;
 }
 0x488acb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488b57; continue; }
  pc = 0x488ae2; continue;
 }
 0x488ae2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x488b09); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488b57; continue; }
  pc = 0x488b11; continue;
 }
 0x488b11 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488b50; continue; }
  pc = 0x488b27; continue;
 }
 0x488b27 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x488b4b); cpu.call(t);
  pc = 0x488b50; continue;
 }
 0x488b50 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488b57; continue;
 }
 0x488b57 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x488b68; continue; }
  pc = 0x488b61; continue;
 }
 0x488b61 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x488b78; continue;
 }
 0x488b68 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x488b78; continue;
 }
 0x488b78 => {
  pc = 0x488b94; continue;
 }
 0x488b94 => {
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
pub fn f_00488b7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488b7d); }
 let mut pc: u32 = 0x488b7d;
 loop { match pc {
 0x488b7d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488b86); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488b8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488b8a); }
 let mut pc: u32 = 0x488b8a;
 loop { match pc {
 0x488b8a => {
  { let v: u32 = 0x4b9550u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488bb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488bb0); }
 let mut pc: u32 = 0x488bb0;
 loop { match pc {
 0x488bb0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488c96u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x488bdf; continue;
 }
 0x488bdf => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x488bf2); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b6cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f7358u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x488c0c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x488c16); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x488c29); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x488c3a); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488c43); f_00488c6c(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488c4c); f_004af36d(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488c58); f_00488c76(cpu);
  pc = 0x488c5d; continue;
 }
 0x488c5d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x488ca0; continue;
 }
 0x488ca0 => {
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
pub fn f_00488c6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488c6c); }
 let mut pc: u32 = 0x488c6c;
 loop { match pc {
 0x488c6c => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488c75); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488c76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488c76); }
 let mut pc: u32 = 0x488c76;
 loop { match pc {
 0x488c76 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488c7f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488c80); }
 let mut pc: u32 = 0x488c80;
 loop { match pc {
 0x488c80 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488c8c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488c8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488c8d); }
 let mut pc: u32 = 0x488c8d;
 loop { match pc {
 0x488c8d => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x488c95); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488c96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488c96); }
 let mut pc: u32 = 0x488c96;
 loop { match pc {
 0x488c96 => {
  { let v: u32 = 0x4b9578u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488cb0); }
 let mut pc: u32 = 0x488cb0;
 loop { match pc {
 0x488cb0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488d77u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x488cd8); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488cff; continue; }
  pc = 0x488cef; continue;
 }
 0x488cef => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x488cf7); f_004ab40d(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488d06; continue;
 }
 0x488cff => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488d06; continue;
 }
 0x488d06 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488d62; continue; }
  pc = 0x488d1d; continue;
 }
 0x488d1d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x488d2c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488d62; continue; }
  pc = 0x488d34; continue;
 }
 0x488d34 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488d5b; continue; }
  pc = 0x488d4a; continue;
 }
 0x488d4a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x488d56); cpu.call(t);
  pc = 0x488d5b; continue;
 }
 0x488d5b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488d62; continue;
 }
 0x488d62 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x488d81; continue;
 }
 0x488d81 => {
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
pub fn f_00488d6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488d6a); }
 let mut pc: u32 = 0x488d6a;
 loop { match pc {
 0x488d6a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488d73); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488d77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488d77); }
 let mut pc: u32 = 0x488d77;
 loop { match pc {
 0x488d77 => {
  { let v: u32 = 0x4b95b8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488da0); }
 let mut pc: u32 = 0x488da0;
 loop { match pc {
 0x488da0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488e86u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x488dcf; continue;
 }
 0x488dcf => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x488de2); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b5cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f7378u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x488dfc); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x488e06); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x488e19); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x488e2a); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488e33); f_00488e5c(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488e3c); f_004af36d(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x488e48); f_00488e66(cpu);
  pc = 0x488e4d; continue;
 }
 0x488e4d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x488e90; continue;
 }
 0x488e90 => {
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
pub fn f_00488e5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488e5c); }
 let mut pc: u32 = 0x488e5c;
 loop { match pc {
 0x488e5c => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488e65); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488e66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488e66); }
 let mut pc: u32 = 0x488e66;
 loop { match pc {
 0x488e66 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488e6f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488e70); }
 let mut pc: u32 = 0x488e70;
 loop { match pc {
 0x488e70 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x488e7c); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488e7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488e7d); }
 let mut pc: u32 = 0x488e7d;
 loop { match pc {
 0x488e7d => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x488e85); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488e86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488e86); }
 let mut pc: u32 = 0x488e86;
 loop { match pc {
 0x488e86 => {
  { let v: u32 = 0x4b95e0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488ea0); }
 let mut pc: u32 = 0x488ea0;
 loop { match pc {
 0x488ea0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x488fbau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x84u32; cpu.push(v); }
  cpu.push(0x488ecb); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488ef4; continue; }
  pc = 0x488ee2; continue;
 }
 0x488ee2 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x488eec); f_004abaee(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488efb; continue;
 }
 0x488ef4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488efb; continue;
 }
 0x488efb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488f87; continue; }
  pc = 0x488f12; continue;
 }
 0x488f12 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x488f39); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x488f87; continue; }
  pc = 0x488f41; continue;
 }
 0x488f41 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x488f80; continue; }
  pc = 0x488f57; continue;
 }
 0x488f57 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x488f7b); cpu.call(t);
  pc = 0x488f80; continue;
 }
 0x488f80 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x488f87; continue;
 }
 0x488f87 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x488f98; continue; }
  pc = 0x488f91; continue;
 }
 0x488f91 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x488fa8; continue;
 }
 0x488f98 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x488fa8; continue;
 }
 0x488fa8 => {
  pc = 0x488fc4; continue;
 }
 0x488fc4 => {
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
pub fn f_00488fad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488fad); }
 let mut pc: u32 = 0x488fad;
 loop { match pc {
 0x488fad => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x488fb6); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488fba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488fba); }
 let mut pc: u32 = 0x488fba;
 loop { match pc {
 0x488fba => {
  { let v: u32 = 0x4b9620u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00488fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x488fe0); }
 let mut pc: u32 = 0x488fe0;
 loop { match pc {
 0x488fe0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4890c6u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48900f; continue;
 }
 0x48900f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x489022); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b4cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f73a0u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48903c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x489046); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x489059); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x48906a); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x489073); f_0048909c(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48907c); f_004af36d(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x489088); f_004890a6(cpu);
  pc = 0x48908d; continue;
 }
 0x48908d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4890d0; continue;
 }
 0x4890d0 => {
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
pub fn f_0048909c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48909c); }
 let mut pc: u32 = 0x48909c;
 loop { match pc {
 0x48909c => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4890a5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004890a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4890a6); }
 let mut pc: u32 = 0x4890a6;
 loop { match pc {
 0x4890a6 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4890af); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004890b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4890b0); }
 let mut pc: u32 = 0x4890b0;
 loop { match pc {
 0x4890b0 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4890bc); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004890bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4890bd); }
 let mut pc: u32 = 0x4890bd;
 loop { match pc {
 0x4890bd => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4890c5); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004890c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4890c6); }
 let mut pc: u32 = 0x4890c6;
 loop { match pc {
 0x4890c6 => {
  { let v: u32 = 0x4b9648u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004890e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4890e0); }
 let mut pc: u32 = 0x4890e0;
 loop { match pc {
 0x4890e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4891a7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x2cu32; cpu.push(v); }
  cpu.push(0x489108); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48912f; continue; }
  pc = 0x48911f; continue;
 }
 0x48911f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x489127); f_004ac449(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489136; continue;
 }
 0x48912f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489136; continue;
 }
 0x489136 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x489192; continue; }
  pc = 0x48914d; continue;
 }
 0x48914d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x48915c); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x489192; continue; }
  pc = 0x489164; continue;
 }
 0x489164 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48918b; continue; }
  pc = 0x48917a; continue;
 }
 0x48917a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x489186); cpu.call(t);
  pc = 0x48918b; continue;
 }
 0x48918b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489192; continue;
 }
 0x489192 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4891b1; continue;
 }
 0x4891b1 => {
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
pub fn f_0048919a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48919a); }
 let mut pc: u32 = 0x48919a;
 loop { match pc {
 0x48919a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4891a3); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004891a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4891a7); }
 let mut pc: u32 = 0x4891a7;
 loop { match pc {
 0x4891a7 => {
  { let v: u32 = 0x4b9688u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004891d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4891d0); }
 let mut pc: u32 = 0x4891d0;
 loop { match pc {
 0x4891d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4892b6u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4891ff; continue;
 }
 0x4891ff => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x489212); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b3cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f73c0u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48922c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x489236); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x489249); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x48925a); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x489263); f_0048928c(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48926c); f_004af36d(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x489278); f_00489296(cpu);
  pc = 0x48927d; continue;
 }
 0x48927d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x4892c0; continue;
 }
 0x4892c0 => {
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
pub fn f_0048928c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48928c); }
 let mut pc: u32 = 0x48928c;
 loop { match pc {
 0x48928c => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x489295); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489296(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489296); }
 let mut pc: u32 = 0x489296;
 loop { match pc {
 0x489296 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48929f); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004892a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4892a0); }
 let mut pc: u32 = 0x4892a0;
 loop { match pc {
 0x4892a0 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4892ac); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004892ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4892ad); }
 let mut pc: u32 = 0x4892ad;
 loop { match pc {
 0x4892ad => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4892b5); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004892b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4892b6); }
 let mut pc: u32 = 0x4892b6;
 loop { match pc {
 0x4892b6 => {
  { let v: u32 = 0x4b96b0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004892d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4892d0); }
 let mut pc: u32 = 0x4892d0;
 loop { match pc {
 0x4892d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4893eau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x84u32; cpu.push(v); }
  cpu.push(0x4892fb); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x489324; continue; }
  pc = 0x489312; continue;
 }
 0x489312 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48931c); f_004acbb3(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48932b; continue;
 }
 0x489324 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48932b; continue;
 }
 0x48932b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4893b7; continue; }
  pc = 0x489342; continue;
 }
 0x489342 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.push(0x489369); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4893b7; continue; }
  pc = 0x489371; continue;
 }
 0x489371 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4893b0; continue; }
  pc = 0x489387; continue;
 }
 0x489387 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x4893ab); cpu.call(t);
  pc = 0x4893b0; continue;
 }
 0x4893b0 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4893b7; continue;
 }
 0x4893b7 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x4893c8; continue; }
  pc = 0x4893c1; continue;
 }
 0x4893c1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x4893d8; continue;
 }
 0x4893c8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x4893d8; continue;
 }
 0x4893d8 => {
  pc = 0x4893f4; continue;
 }
 0x4893f4 => {
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
pub fn f_004893dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4893dd); }
 let mut pc: u32 = 0x4893dd;
 loop { match pc {
 0x4893dd => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4893e6); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004893ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4893ea); }
 let mut pc: u32 = 0x4893ea;
 loop { match pc {
 0x4893ea => {
  { let v: u32 = 0x4b96f0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489410); }
 let mut pc: u32 = 0x489410;
 loop { match pc {
 0x489410 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4894f6u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b191cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48943f; continue;
 }
 0x48943f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x489452); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b2cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f73f0u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48946c); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x489476); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x489489); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x48949a); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4894a3); f_004894cc(cpu);
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4894ac); f_004af36d(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4894b8); f_004894d6(cpu);
  pc = 0x4894bd; continue;
 }
 0x4894bd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x489500; continue;
 }
 0x489500 => {
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
pub fn f_004894cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4894cc); }
 let mut pc: u32 = 0x4894cc;
 loop { match pc {
 0x4894cc => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4894d5); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004894d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4894d6); }
 let mut pc: u32 = 0x4894d6;
 loop { match pc {
 0x4894d6 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4894df); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004894e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4894e0); }
 let mut pc: u32 = 0x4894e0;
 loop { match pc {
 0x4894e0 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x4894ec); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004894ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4894ed); }
 let mut pc: u32 = 0x4894ed;
 loop { match pc {
 0x4894ed => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x4894f5); f_00487c20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004894f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4894f6); }
 let mut pc: u32 = 0x4894f6;
 loop { match pc {
 0x4894f6 => {
  { let v: u32 = 0x4b9718u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489510); }
 let mut pc: u32 = 0x489510;
 loop { match pc {
 0x489510 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x489527; continue;
 }
 0x489527 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489530); }
 let mut pc: u32 = 0x489530;
 loop { match pc {
 0x489530 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x4895f7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x38u32; cpu.push(v); }
  cpu.push(0x489558); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48957f; continue; }
  pc = 0x48956f; continue;
 }
 0x48956f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x489577); f_004ad675(cpu);
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489586; continue;
 }
 0x48957f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489586; continue;
 }
 0x489586 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4895e2; continue; }
  pc = 0x48959d; continue;
 }
 0x48959d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.push(0x4895ac); cpu.call(t);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4895e2; continue; }
  pc = 0x4895b4; continue;
 }
 0x4895b4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x4895db; continue; }
  pc = 0x4895ca; continue;
 }
 0x4895ca => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.push(0x4895d6); cpu.call(t);
  pc = 0x4895db; continue;
 }
 0x4895db => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4895e2; continue;
 }
 0x4895e2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x489601; continue;
 }
 0x489601 => {
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
pub fn f_004895ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4895ea); }
 let mut pc: u32 = 0x4895ea;
 loop { match pc {
 0x4895ea => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4895f3); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004895f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4895f7); }
 let mut pc: u32 = 0x4895f7;
 loop { match pc {
 0x4895f7 => {
  { let v: u32 = 0x4b9758u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489620); }
 let mut pc: u32 = 0x489620;
 loop { match pc {
 0x489620 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x489724u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48964f; continue;
 }
 0x48964f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x489662); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1b0cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f7410u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x489685); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x48968f); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x4896a2); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x4896b3); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4896bc); f_004896fa(cpu);
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x4896dd; continue; }
  pc = 0x4896c9; continue;
 }
 0x4896c9 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x4896dd); cpu.call(t);
  pc = 0x4896dd; continue;
 }
 0x4896dd => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4896e6); f_00489704(cpu);
  pc = 0x4896eb; continue;
 }
 0x4896eb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48972e; continue;
 }
 0x48972e => {
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
pub fn f_004896fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4896fa); }
 let mut pc: u32 = 0x4896fa;
 loop { match pc {
 0x4896fa => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x489703); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489704(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489704); }
 let mut pc: u32 = 0x489704;
 loop { match pc {
 0x489704 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48970d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048970e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48970e); }
 let mut pc: u32 = 0x48970e;
 loop { match pc {
 0x48970e => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48971a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048971b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48971b); }
 let mut pc: u32 = 0x48971b;
 loop { match pc {
 0x48971b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x489723); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489724(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489724); }
 let mut pc: u32 = 0x489724;
 loop { match pc {
 0x489724 => {
  { let v: u32 = 0x4b9780u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489740); }
 let mut pc: u32 = 0x489740;
 loop { match pc {
 0x489740 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x489af7u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x64u32; cpu.push(v); }
  cpu.push(0x48976f); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x489aa6; continue; }
  pc = 0x489786; continue;
 }
 0x489786 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1bb8u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1ba8u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1ba0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x4897b9; continue;
 }
 0x4897b9 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4897ea; continue;
 }
 0x4897ea => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48980d; continue;
 }
 0x48980d => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489838; continue;
 }
 0x489838 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4898f8; continue;
 }
 0x4898f8 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1cb4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1c9cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1c6cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48992e; continue;
 }
 0x48992e => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48996c; continue;
 }
 0x48996c => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x489980); f_004996be(cpu);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x489990); cpu.call(t);
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1cf4u32; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x4899a3; continue;
 }
 0x4899a3 => {
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4899c2; continue;
 }
 0x4899c2 => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x4899e9; continue;
 }
 0x4899e9 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1c14u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1bfcu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1bccu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x489a45; continue;
 }
 0x489a45 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489aa1; continue;
 }
 0x489aa1 => {
  pc = 0x489aad; continue;
 }
 0x489aa6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489aad; continue;
 }
 0x489aad => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x489ad0; continue; }
  pc = 0x489ac4; continue;
 }
 0x489ac4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489ae2; continue;
 }
 0x489ad0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489ae2; continue;
 }
 0x489ae2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x489b01; continue;
 }
 0x489b01 => {
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
pub fn f_00489822(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489822); }
 let mut pc: u32 = 0x489822;
 loop { match pc {
 0x489822 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x489837; continue; }
  pc = 0x48982c; continue;
 }
 0x48982c => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x489837); f_00489b10(cpu);
  pc = 0x489837; continue;
 }
 0x489837 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004898d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4898d7); }
 let mut pc: u32 = 0x4898d7;
 loop { match pc {
 0x4898d7 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x4898ec; continue; }
  pc = 0x4898e1; continue;
 }
 0x4898e1 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x14u32); cpu.ecx = v; }
  cpu.push(0x4898ec); f_00489b10(cpu);
  pc = 0x4898ec; continue;
 }
 0x4898ec => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489940); }
 let mut pc: u32 = 0x489940;
 loop { match pc {
 0x489940 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x489955; continue; }
  pc = 0x48994a; continue;
 }
 0x48994a => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  cpu.push(0x489955); f_00490b40(cpu);
  pc = 0x489955; continue;
 }
 0x489955 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489956(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489956); }
 let mut pc: u32 = 0x489956;
 loop { match pc {
 0x489956 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48996b; continue; }
  pc = 0x489960; continue;
 }
 0x489960 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x48996b); f_00489b10(cpu);
  pc = 0x48996b; continue;
 }
 0x48996b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004899b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4899b5); }
 let mut pc: u32 = 0x4899b5;
 loop { match pc {
 0x4899b5 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x4899c1); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489a54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489a54); }
 let mut pc: u32 = 0x489a54;
 loop { match pc {
 0x489a54 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  cpu.push(0x489a5f); f_00489b70(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489a60); }
 let mut pc: u32 = 0x489a60;
 loop { match pc {
 0x489a60 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x489a6b); f_00490ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489a6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489a6c); }
 let mut pc: u32 = 0x489a6c;
 loop { match pc {
 0x489a6c => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x489a74); f_004913d0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489a75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489a75); }
 let mut pc: u32 = 0x489a75;
 loop { match pc {
 0x489a75 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x489a8a; continue; }
  pc = 0x489a7f; continue;
 }
 0x489a7f => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x5cu32); cpu.ecx = v; }
  cpu.push(0x489a8a); f_00490b40(cpu);
  pc = 0x489a8a; continue;
 }
 0x489a8a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489a8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489a8b); }
 let mut pc: u32 = 0x489a8b;
 loop { match pc {
 0x489a8b => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x489aa0; continue; }
  pc = 0x489a95; continue;
 }
 0x489a95 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x489aa0); f_00489b10(cpu);
  pc = 0x489aa0; continue;
 }
 0x489aa0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489aea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489aea); }
 let mut pc: u32 = 0x489aea;
 loop { match pc {
 0x489aea => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x489af3); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489af7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489af7); }
 let mut pc: u32 = 0x489af7;
 loop { match pc {
 0x489af7 => {
  { let v: u32 = 0x4b97c0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489b10); }
 let mut pc: u32 = 0x489b10;
 loop { match pc {
 0x489b10 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x489b2a; continue;
 }
 0x489b2a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489b30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489b30); }
 let mut pc: u32 = 0x489b30;
 loop { match pc {
 0x489b30 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x489b4a; continue;
 }
 0x489b4a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x489b60; continue; }
  pc = 0x489b54; continue;
 }
 0x489b54 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x489b5d); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x489b60; continue;
 }
 0x489b60 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x489b68; continue;
 }
 0x489b68 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489b70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489b70); }
 let mut pc: u32 = 0x489b70;
 loop { match pc {
 0x489b70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x489bcau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1cf4u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525750u32); cpu.push(0x489baa); cpu.call(t);
  pc = 0x489baf; continue;
 }
 0x489baf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x489bbb); f_00489bc0(cpu);
  pc = 0x489bd4; continue;
 }
 0x489bd4 => {
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
pub fn f_00489bc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489bc0); }
 let mut pc: u32 = 0x489bc0;
 loop { match pc {
 0x489bc0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x489bc9); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489bca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489bca); }
 let mut pc: u32 = 0x489bca;
 loop { match pc {
 0x489bca => {
  { let v: u32 = 0x4b9838u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489bf0); }
 let mut pc: u32 = 0x489bf0;
 loop { match pc {
 0x489bf0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x489c7du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.push(0x489c18); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x489c54; continue; }
  pc = 0x489c2f; continue;
 }
 0x489c2f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x525704u32); cpu.push(0x489c38); cpu.call(t);
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x489c49; continue;
 }
 0x489c49 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489c5b; continue;
 }
 0x489c54 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489c5b; continue;
 }
 0x489c5b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x489c87; continue;
 }
 0x489c87 => {
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
pub fn f_00489c70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489c70); }
 let mut pc: u32 = 0x489c70;
 loop { match pc {
 0x489c70 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x489c79); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489c7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489c7d); }
 let mut pc: u32 = 0x489c7d;
 loop { match pc {
 0x489c7d => {
  { let v: u32 = 0x4b9860u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489ca0); }
 let mut pc: u32 = 0x489ca0;
 loop { match pc {
 0x489ca0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x489da4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x489ccf; continue;
 }
 0x489ccf => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x489ce2); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1aecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f74a0u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x489d05); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x489d0f); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x489d22); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x489d33); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x489d3c); f_00489d7a(cpu);
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x489d5d; continue; }
  pc = 0x489d49; continue;
 }
 0x489d49 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x489d5d); cpu.call(t);
  pc = 0x489d5d; continue;
 }
 0x489d5d => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x489d66); f_00489d84(cpu);
  pc = 0x489d6b; continue;
 }
 0x489d6b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x489dae; continue;
 }
 0x489dae => {
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
pub fn f_00489d7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489d7a); }
 let mut pc: u32 = 0x489d7a;
 loop { match pc {
 0x489d7a => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x489d83); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489d84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489d84); }
 let mut pc: u32 = 0x489d84;
 loop { match pc {
 0x489d84 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x489d8d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489d8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489d8e); }
 let mut pc: u32 = 0x489d8e;
 loop { match pc {
 0x489d8e => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x489d9a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489d9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489d9b); }
 let mut pc: u32 = 0x489d9b;
 loop { match pc {
 0x489d9b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x489da3); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489da4); }
 let mut pc: u32 = 0x489da4;
 loop { match pc {
 0x489da4 => {
  { let v: u32 = 0x4b9888u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489dc0); }
 let mut pc: u32 = 0x489dc0;
 loop { match pc {
 0x489dc0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48a25au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x98u32; cpu.push(v); }
  cpu.push(0x489df2); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48a209; continue; }
  pc = 0x489e09; continue;
 }
 0x489e09 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d80u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d70u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d68u32; cpu.w32(0x94u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  pc = 0x489e42; continue;
 }
 0x489e42 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x489e79; continue;
 }
 0x489e79 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x90u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x94u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x489ea5; continue;
 }
 0x489ea5 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x90u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489ed5; continue;
 }
 0x489ed5 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x489f9f; continue;
 }
 0x489f9f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x489fa9); f_00490b70(cpu);
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  cpu.push(0x489fbd); f_004996be(cpu);
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x52574cu32); cpu.push(0x489fcd); cpu.call(t);
  { let v: u8 = 0x9u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1cf4u32; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x489fe0; continue;
 }
 0x489fe0 => {
  { let v: u8 = 0x8u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x34u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x489fff; continue;
 }
 0x489fff => {
  { let v: u8 = 0xau8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x48a026; continue;
 }
 0x48a026 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1c14u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1bfcu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1bccu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48a082; continue;
 }
 0x48a082 => {
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a0de; continue;
 }
 0x48a0de => {
  { let v: u32 = 0xbu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1edcu32; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a0f4; continue;
 }
 0x48a0f4 => {
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1e7cu32; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a107; continue;
 }
 0x48a107 => {
  { let v: u8 = 0xbu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a125; continue;
 }
 0x48a125 => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a138; continue;
 }
 0x48a138 => {
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1e3cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1ddcu32; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1dc4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d94u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48a1a2; continue;
 }
 0x48a1a2 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a204; continue;
 }
 0x48a204 => {
  pc = 0x48a210; continue;
 }
 0x48a209 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a210; continue;
 }
 0x48a210 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48a233; continue; }
  pc = 0x48a227; continue;
 }
 0x48a227 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a245; continue;
 }
 0x48a233 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a245; continue;
 }
 0x48a245 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48a264; continue;
 }
 0x48a264 => {
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
pub fn f_00489ebc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489ebc); }
 let mut pc: u32 = 0x489ebc;
 loop { match pc {
 0x489ebc => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x489ed4; continue; }
  pc = 0x489ec6; continue;
 }
 0x489ec6 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x98u32); cpu.ecx = v; }
  cpu.push(0x489ed4); f_00489b10(cpu);
  pc = 0x489ed4; continue;
 }
 0x489ed4 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489f7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489f7e); }
 let mut pc: u32 = 0x489f7e;
 loop { match pc {
 0x489f7e => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x489f93; continue; }
  pc = 0x489f88; continue;
 }
 0x489f88 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x64u32); cpu.ecx = v; }
  cpu.push(0x489f93); f_00489b10(cpu);
  pc = 0x489f93; continue;
 }
 0x489f93 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00489ff2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x489ff2); }
 let mut pc: u32 = 0x489ff2;
 loop { match pc {
 0x489ff2 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  let t = cpu.r32(0x525754u32); cpu.push(0x489ffe); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a091(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a091); }
 let mut pc: u32 = 0x48a091;
 loop { match pc {
 0x48a091 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x34u32); cpu.ecx = v; }
  cpu.push(0x48a09c); f_00489b70(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a09d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a09d); }
 let mut pc: u32 = 0x48a09d;
 loop { match pc {
 0x48a09d => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  cpu.push(0x48a0a8); f_00490ce0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a0a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a0a9); }
 let mut pc: u32 = 0x48a0a9;
 loop { match pc {
 0x48a0a9 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48a0b1); f_004913d0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a0b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a0b2); }
 let mut pc: u32 = 0x48a0b2;
 loop { match pc {
 0x48a0b2 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48a0c7; continue; }
  pc = 0x48a0bc; continue;
 }
 0x48a0bc => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x5cu32); cpu.ecx = v; }
  cpu.push(0x48a0c7); f_00490b40(cpu);
  pc = 0x48a0c7; continue;
 }
 0x48a0c7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a0c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a0c8); }
 let mut pc: u32 = 0x48a0c8;
 loop { match pc {
 0x48a0c8 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48a0dd; continue; }
  pc = 0x48a0d2; continue;
 }
 0x48a0d2 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x48a0dd); f_00489b10(cpu);
  pc = 0x48a0dd; continue;
 }
 0x48a0dd => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a119(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a119); }
 let mut pc: u32 = 0x48a119;
 loop { match pc {
 0x48a119 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  cpu.push(0x48a124); f_0048a280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a1b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a1b1); }
 let mut pc: u32 = 0x48a1b1;
 loop { match pc {
 0x48a1b1 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x54u32); cpu.ecx = v; }
  cpu.push(0x48a1bc); f_00405cd0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a1bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a1bd); }
 let mut pc: u32 = 0x48a1bd;
 loop { match pc {
 0x48a1bd => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  cpu.push(0x48a1c8); f_00491110(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a1c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a1c9); }
 let mut pc: u32 = 0x48a1c9;
 loop { match pc {
 0x48a1c9 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48a1d1); f_00491040(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a1d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a1d2); }
 let mut pc: u32 = 0x48a1d2;
 loop { match pc {
 0x48a1d2 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48a1ea; continue; }
  pc = 0x48a1dc; continue;
 }
 0x48a1dc => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x90u32); cpu.ecx = v; }
  cpu.push(0x48a1ea); f_00490b40(cpu);
  pc = 0x48a1ea; continue;
 }
 0x48a1ea => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a1eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a1eb); }
 let mut pc: u32 = 0x48a1eb;
 loop { match pc {
 0x48a1eb => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48a203; continue; }
  pc = 0x48a1f5; continue;
 }
 0x48a1f5 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x88u32); cpu.ecx = v; }
  cpu.push(0x48a203); f_00489b10(cpu);
  pc = 0x48a203; continue;
 }
 0x48a203 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a24d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a24d); }
 let mut pc: u32 = 0x48a24d;
 loop { match pc {
 0x48a24d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48a256); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a25a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a25a); }
 let mut pc: u32 = 0x48a25a;
 loop { match pc {
 0x48a25a => {
  { let v: u32 = 0x4b98c8u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a280); }
 let mut pc: u32 = 0x48a280;
 loop { match pc {
 0x48a280 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1edcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a29a; continue;
 }
 0x48a29a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a2a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a2a0); }
 let mut pc: u32 = 0x48a2a0;
 loop { match pc {
 0x48a2a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1edcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a2ba; continue;
 }
 0x48a2ba => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48a2d0; continue; }
  pc = 0x48a2c4; continue;
 }
 0x48a2c4 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48a2cd); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x48a2d0; continue;
 }
 0x48a2d0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48a2d8; continue;
 }
 0x48a2d8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a2e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a2e0); }
 let mut pc: u32 = 0x48a2e0;
 loop { match pc {
 0x48a2e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48a3e4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a30f; continue;
 }
 0x48a30f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x48a322); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1accu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f74c8u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48a345); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x48a34f); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x48a362); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x48a373); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48a37c); f_0048a3ba(cpu);
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x48a39d; continue; }
  pc = 0x48a389; continue;
 }
 0x48a389 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x48a39d); cpu.call(t);
  pc = 0x48a39d; continue;
 }
 0x48a39d => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48a3a6); f_0048a3c4(cpu);
  pc = 0x48a3ab; continue;
 }
 0x48a3ab => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48a3ee; continue;
 }
 0x48a3ee => {
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
pub fn f_0048a3ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a3ba); }
 let mut pc: u32 = 0x48a3ba;
 loop { match pc {
 0x48a3ba => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48a3c3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a3c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a3c4); }
 let mut pc: u32 = 0x48a3c4;
 loop { match pc {
 0x48a3c4 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48a3cd); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a3ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a3ce); }
 let mut pc: u32 = 0x48a3ce;
 loop { match pc {
 0x48a3ce => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48a3da); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a3db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a3db); }
 let mut pc: u32 = 0x48a3db;
 loop { match pc {
 0x48a3db => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48a3e3); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a3e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a3e4); }
 let mut pc: u32 = 0x48a3e4;
 loop { match pc {
 0x48a3e4 => {
  { let v: u32 = 0x4b9960u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a400); }
 let mut pc: u32 = 0x48a400;
 loop { match pc {
 0x48a400 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48a9a6u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x168u32; cpu.push(v); }
  cpu.push(0x48a432); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48a955; continue; }
  pc = 0x48a449; continue;
 }
 0x48a449 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1f28u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1f20u32; cpu.w32(0x154u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1f10u32; cpu.w32(0x15cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1f00u32; cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x148u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a492; continue;
 }
 0x48a492 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a4c9; continue;
 }
 0x48a4c9 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x150u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x154u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x154u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48a4f5; continue;
 }
 0x48a4f5 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x150u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a525; continue;
 }
 0x48a525 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a605; continue;
 }
 0x48a605 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x158u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0x15cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x15cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0x15cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48a64b; continue;
 }
 0x48a64b => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x158u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a694; continue;
 }
 0x48a694 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a727; continue;
 }
 0x48a727 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b208cu32; cpu.w32(0x160u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2074u32; cpu.w32(0x164u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2044u32; cpu.w32(0x164u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b202cu32; cpu.w32(0x164u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48a787; continue;
 }
 0x48a787 => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x160u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a7e9; continue;
 }
 0x48a7e9 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  cpu.push(0x48a801); f_00491440(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1edcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a816; continue;
 }
 0x48a816 => {
  { let v: u8 = 0xdu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1eecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48a828; continue;
 }
 0x48a828 => {
  { let v: u8 = 0xcu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a840; continue;
 }
 0x48a840 => {
  { let v: u8 = 0xeu8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1fecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1fd4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1fa4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1f8cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1f44u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48a8c8; continue;
 }
 0x48a8c8 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a950; continue;
 }
 0x48a950 => {
  pc = 0x48a95c; continue;
 }
 0x48a955 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a95c; continue;
 }
 0x48a95c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48a97f; continue; }
  pc = 0x48a973; continue;
 }
 0x48a973 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a991; continue;
 }
 0x48a97f => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48a991; continue;
 }
 0x48a991 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48a9b0; continue;
 }
 0x48a9b0 => {
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
pub fn f_0048a50c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a50c); }
 let mut pc: u32 = 0x48a50c;
 loop { match pc {
 0x48a50c => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48a524; continue; }
  pc = 0x48a516; continue;
 }
 0x48a516 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x158u32); cpu.ecx = v; }
  cpu.push(0x48a524); f_00489b10(cpu);
  pc = 0x48a524; continue;
 }
 0x48a524 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a5e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a5e1); }
 let mut pc: u32 = 0x48a5e1;
 loop { match pc {
 0x48a5e1 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x48a5f9; continue; }
  pc = 0x48a5eb; continue;
 }
 0x48a5eb => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x16cu32); cpu.ecx = v; }
  cpu.push(0x48a5f9); f_00489b10(cpu);
  pc = 0x48a5f9; continue;
 }
 0x48a5f9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a662(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a662); }
 let mut pc: u32 = 0x48a662;
 loop { match pc {
 0x48a662 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48a67a; continue; }
  pc = 0x48a66c; continue;
 }
 0x48a66c => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x164u32); cpu.ecx = v; }
  cpu.push(0x48a67a); f_00490b40(cpu);
  pc = 0x48a67a; continue;
 }
 0x48a67a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a67b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a67b); }
 let mut pc: u32 = 0x48a67b;
 loop { match pc {
 0x48a67b => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x48a693; continue; }
  pc = 0x48a685; continue;
 }
 0x48a685 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x160u32); cpu.ecx = v; }
  cpu.push(0x48a693); f_00489b10(cpu);
  pc = 0x48a693; continue;
 }
 0x48a693 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a79e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a79e); }
 let mut pc: u32 = 0x48a79e;
 loop { match pc {
 0x48a79e => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48a7b6; continue; }
  pc = 0x48a7a8; continue;
 }
 0x48a7a8 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x174u32); cpu.ecx = v; }
  cpu.push(0x48a7b6); f_004913d0(cpu);
  pc = 0x48a7b6; continue;
 }
 0x48a7b6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a7b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a7b7); }
 let mut pc: u32 = 0x48a7b7;
 loop { match pc {
 0x48a7b7 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48a7cf; continue; }
  pc = 0x48a7c1; continue;
 }
 0x48a7c1 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x16cu32); cpu.ecx = v; }
  cpu.push(0x48a7cf); f_00490b40(cpu);
  pc = 0x48a7cf; continue;
 }
 0x48a7cf => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a7d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a7d0); }
 let mut pc: u32 = 0x48a7d0;
 loop { match pc {
 0x48a7d0 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48a7e8; continue; }
  pc = 0x48a7da; continue;
 }
 0x48a7da => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x168u32); cpu.ecx = v; }
  cpu.push(0x48a7e8); f_00489b10(cpu);
  pc = 0x48a7e8; continue;
 }
 0x48a7e8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a837(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a837); }
 let mut pc: u32 = 0x48a837;
 loop { match pc {
 0x48a837 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48a83f); f_0048a280(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a8d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a8d7); }
 let mut pc: u32 = 0x48a8d7;
 loop { match pc {
 0x48a8d7 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48a8df); f_00491e50(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a8e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a8e0); }
 let mut pc: u32 = 0x48a8e0;
 loop { match pc {
 0x48a8e0 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  cpu.push(0x48a8eb); f_00491da0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a8ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a8ec); }
 let mut pc: u32 = 0x48a8ec;
 loop { match pc {
 0x48a8ec => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48a904; continue; }
  pc = 0x48a8f6; continue;
 }
 0x48a8f6 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x160u32); cpu.ecx = v; }
  cpu.push(0x48a904); f_00491400(cpu);
  pc = 0x48a904; continue;
 }
 0x48a904 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a905(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a905); }
 let mut pc: u32 = 0x48a905;
 loop { match pc {
 0x48a905 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48a91d; continue; }
  pc = 0x48a90f; continue;
 }
 0x48a90f => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x158u32); cpu.ecx = v; }
  cpu.push(0x48a91d); f_004913d0(cpu);
  pc = 0x48a91d; continue;
 }
 0x48a91d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a91e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a91e); }
 let mut pc: u32 = 0x48a91e;
 loop { match pc {
 0x48a91e => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48a936; continue; }
  pc = 0x48a928; continue;
 }
 0x48a928 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x150u32); cpu.ecx = v; }
  cpu.push(0x48a936); f_00490b40(cpu);
  pc = 0x48a936; continue;
 }
 0x48a936 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a937(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a937); }
 let mut pc: u32 = 0x48a937;
 loop { match pc {
 0x48a937 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48a94f; continue; }
  pc = 0x48a941; continue;
 }
 0x48a941 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x148u32); cpu.ecx = v; }
  cpu.push(0x48a94f); f_00489b10(cpu);
  pc = 0x48a94f; continue;
 }
 0x48a94f => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a999(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a999); }
 let mut pc: u32 = 0x48a999;
 loop { match pc {
 0x48a999 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48a9a2); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a9a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a9a6); }
 let mut pc: u32 = 0x48a9a6;
 loop { match pc {
 0x48a9a6 => {
  { let v: u32 = 0x4b99a0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048a9c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48a9c0); }
 let mut pc: u32 = 0x48a9c0;
 loop { match pc {
 0x48a9c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48aa0b; continue; }
  pc = 0x48a9e2; continue;
 }
 0x48a9e2 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.ecx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x48aa06); cpu.call(t);
  pc = 0x48aa0b; continue;
 }
 0x48aa0b => {
  pc = 0x48aa10; continue;
 }
 0x48aa10 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048aa20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48aa20); }
 let mut pc: u32 = 0x48aa20;
 loop { match pc {
 0x48aa20 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48ab24u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48aa4f; continue;
 }
 0x48aa4f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x48aa62); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1aacu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f7550u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48aa85); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x48aa8f); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x48aaa2); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x48aab3); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48aabc); f_0048aafa(cpu);
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x48aadd; continue; }
  pc = 0x48aac9; continue;
 }
 0x48aac9 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x48aadd); cpu.call(t);
  pc = 0x48aadd; continue;
 }
 0x48aadd => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48aae6); f_0048ab04(cpu);
  pc = 0x48aaeb; continue;
 }
 0x48aaeb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48ab2e; continue;
 }
 0x48ab2e => {
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
pub fn f_0048aafa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48aafa); }
 let mut pc: u32 = 0x48aafa;
 loop { match pc {
 0x48aafa => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48ab03); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ab04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ab04); }
 let mut pc: u32 = 0x48ab04;
 loop { match pc {
 0x48ab04 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48ab0d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ab0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ab0e); }
 let mut pc: u32 = 0x48ab0e;
 loop { match pc {
 0x48ab0e => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48ab1a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ab1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ab1b); }
 let mut pc: u32 = 0x48ab1b;
 loop { match pc {
 0x48ab1b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48ab23); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ab24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ab24); }
 let mut pc: u32 = 0x48ab24;
 loop { match pc {
 0x48ab24 => {
  { let v: u32 = 0x4b9a38u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ab40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ab40); }
 let mut pc: u32 = 0x48ab40;
 loop { match pc {
 0x48ab40 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48b140u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0xc8u32; cpu.push(v); }
  cpu.push(0x48ab72); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48b0ef; continue; }
  pc = 0x48ab89; continue;
 }
 0x48ab89 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2188u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2170u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2168u32; cpu.w32(0xb0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2158u32; cpu.w32(0xb8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2148u32; cpu.w32(0xc4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0xa4u32.wrapping_add(cpu.eax), v); }
  pc = 0x48abdc; continue;
 }
 0x48abdc => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48ac13; continue;
 }
 0x48ac13 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0xacu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xb0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0xb0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48ac3f; continue;
 }
 0x48ac3f => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xacu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48ac6f; continue;
 }
 0x48ac6f => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48ad4f; continue;
 }
 0x48ad4f => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0xb4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xb8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0xb8u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xb8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0xb8u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48ad95; continue;
 }
 0x48ad95 => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xb4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48adde; continue;
 }
 0x48adde => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48aeef; continue;
 }
 0x48aeef => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0xc0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xc4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0xc4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xc4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0xc4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xc4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0xc4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48af4f; continue;
 }
 0x48af4f => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48afb1; continue;
 }
 0x48afb1 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48afc6); f_0049e50b(cpu);
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2244u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2224u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b220cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b21dcu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b21c4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b21a4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48b06e; continue;
 }
 0x48b06e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b0ea; continue;
 }
 0x48b0ea => {
  pc = 0x48b0f6; continue;
 }
 0x48b0ef => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b0f6; continue;
 }
 0x48b0f6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48b119; continue; }
  pc = 0x48b10d; continue;
 }
 0x48b10d => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b12b; continue;
 }
 0x48b119 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b12b; continue;
 }
 0x48b12b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48b14a; continue;
 }
 0x48b14a => {
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
pub fn f_0048ac56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ac56); }
 let mut pc: u32 = 0x48ac56;
 loop { match pc {
 0x48ac56 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48ac6e; continue; }
  pc = 0x48ac60; continue;
 }
 0x48ac60 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb4u32); cpu.ecx = v; }
  cpu.push(0x48ac6e); f_00489b10(cpu);
  pc = 0x48ac6e; continue;
 }
 0x48ac6e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048ad2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ad2b); }
 let mut pc: u32 = 0x48ad2b;
 loop { match pc {
 0x48ad2b => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x48ad43; continue; }
  pc = 0x48ad35; continue;
 }
 0x48ad35 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xc8u32); cpu.ecx = v; }
  cpu.push(0x48ad43); f_00489b10(cpu);
  pc = 0x48ad43; continue;
 }
 0x48ad43 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048adac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48adac); }
 let mut pc: u32 = 0x48adac;
 loop { match pc {
 0x48adac => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48adc4; continue; }
  pc = 0x48adb6; continue;
 }
 0x48adb6 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xc0u32); cpu.ecx = v; }
  cpu.push(0x48adc4); f_00490b40(cpu);
  pc = 0x48adc4; continue;
 }
 0x48adc4 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048adc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48adc5); }
 let mut pc: u32 = 0x48adc5;
 loop { match pc {
 0x48adc5 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x48addd; continue; }
  pc = 0x48adcf; continue;
 }
 0x48adcf => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xbcu32); cpu.ecx = v; }
  cpu.push(0x48addd); f_00489b10(cpu);
  pc = 0x48addd; continue;
 }
 0x48addd => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048aeaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48aeaa); }
 let mut pc: u32 = 0x48aeaa;
 loop { match pc {
 0x48aeaa => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48aec2; continue; }
  pc = 0x48aeb4; continue;
 }
 0x48aeb4 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xd4u32); cpu.ecx = v; }
  cpu.push(0x48aec2); f_00489b10(cpu);
  pc = 0x48aec2; continue;
 }
 0x48aec2 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048af66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48af66); }
 let mut pc: u32 = 0x48af66;
 loop { match pc {
 0x48af66 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48af7e; continue; }
  pc = 0x48af70; continue;
 }
 0x48af70 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xd4u32); cpu.ecx = v; }
  cpu.push(0x48af7e); f_004913d0(cpu);
  pc = 0x48af7e; continue;
 }
 0x48af7e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048af7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48af7f); }
 let mut pc: u32 = 0x48af7f;
 loop { match pc {
 0x48af7f => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48af97; continue; }
  pc = 0x48af89; continue;
 }
 0x48af89 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xccu32); cpu.ecx = v; }
  cpu.push(0x48af97); f_00490b40(cpu);
  pc = 0x48af97; continue;
 }
 0x48af97 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048af98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48af98); }
 let mut pc: u32 = 0x48af98;
 loop { match pc {
 0x48af98 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48afb0; continue; }
  pc = 0x48afa2; continue;
 }
 0x48afa2 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xc8u32); cpu.ecx = v; }
  cpu.push(0x48afb0); f_00489b10(cpu);
  pc = 0x48afb0; continue;
 }
 0x48afb0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b07d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b07d); }
 let mut pc: u32 = 0x48b07d;
 loop { match pc {
 0x48b07d => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48b085); f_00491f60(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b086(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b086); }
 let mut pc: u32 = 0x48b086;
 loop { match pc {
 0x48b086 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48b09e; continue; }
  pc = 0x48b090; continue;
 }
 0x48b090 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xc0u32); cpu.ecx = v; }
  cpu.push(0x48b09e); f_00491400(cpu);
  pc = 0x48b09e; continue;
 }
 0x48b09e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b09f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b09f); }
 let mut pc: u32 = 0x48b09f;
 loop { match pc {
 0x48b09f => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48b0b7; continue; }
  pc = 0x48b0a9; continue;
 }
 0x48b0a9 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb4u32); cpu.ecx = v; }
  cpu.push(0x48b0b7); f_004913d0(cpu);
  pc = 0x48b0b7; continue;
 }
 0x48b0b7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b0b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b0b8); }
 let mut pc: u32 = 0x48b0b8;
 loop { match pc {
 0x48b0b8 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48b0d0; continue; }
  pc = 0x48b0c2; continue;
 }
 0x48b0c2 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xacu32); cpu.ecx = v; }
  cpu.push(0x48b0d0); f_00490b40(cpu);
  pc = 0x48b0d0; continue;
 }
 0x48b0d0 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b0d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b0d1); }
 let mut pc: u32 = 0x48b0d1;
 loop { match pc {
 0x48b0d1 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48b0e9; continue; }
  pc = 0x48b0db; continue;
 }
 0x48b0db => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xa4u32); cpu.ecx = v; }
  cpu.push(0x48b0e9); f_00489b10(cpu);
  pc = 0x48b0e9; continue;
 }
 0x48b0e9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b133(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b133); }
 let mut pc: u32 = 0x48b133;
 loop { match pc {
 0x48b133 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48b13c); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b140); }
 let mut pc: u32 = 0x48b140;
 loop { match pc {
 0x48b140 => {
  { let v: u32 = 0x4b9a78u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b160); }
 let mut pc: u32 = 0x48b160;
 loop { match pc {
 0x48b160 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48b264u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48b18f; continue;
 }
 0x48b18f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x48b1a2); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1a8cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f75c8u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48b1c5); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x48b1cf); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x48b1e2); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x48b1f3); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48b1fc); f_0048b23a(cpu);
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x48b21d; continue; }
  pc = 0x48b209; continue;
 }
 0x48b209 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x48b21d); cpu.call(t);
  pc = 0x48b21d; continue;
 }
 0x48b21d => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48b226); f_0048b244(cpu);
  pc = 0x48b22b; continue;
 }
 0x48b22b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48b26e; continue;
 }
 0x48b26e => {
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
pub fn f_0048b23a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b23a); }
 let mut pc: u32 = 0x48b23a;
 loop { match pc {
 0x48b23a => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48b243); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b244); }
 let mut pc: u32 = 0x48b244;
 loop { match pc {
 0x48b244 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48b24d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b24e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b24e); }
 let mut pc: u32 = 0x48b24e;
 loop { match pc {
 0x48b24e => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48b25a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b25b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b25b); }
 let mut pc: u32 = 0x48b25b;
 loop { match pc {
 0x48b25b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48b263); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b264(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b264); }
 let mut pc: u32 = 0x48b264;
 loop { match pc {
 0x48b264 => {
  { let v: u32 = 0x4b9b08u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b280); }
 let mut pc: u32 = 0x48b280;
 loop { match pc {
 0x48b280 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48b7eau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x98u32; cpu.push(v); }
  cpu.push(0x48b2b2); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48b799; continue; }
  pc = 0x48b2c9; continue;
 }
 0x48b2c9 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2350u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2338u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2330u32; cpu.w32(0x80u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2320u32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2310u32; cpu.w32(0x94u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x74u32.wrapping_add(cpu.eax), v); }
  pc = 0x48b319; continue;
 }
 0x48b319 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b350; continue;
 }
 0x48b350 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x7cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x80u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48b379; continue;
 }
 0x48b379 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b3a7; continue;
 }
 0x48b3a7 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b487; continue;
 }
 0x48b487 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0x88u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0x88u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48b4cd; continue;
 }
 0x48b4cd => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x84u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b516; continue;
 }
 0x48b516 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b5a9; continue;
 }
 0x48b5a9 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b22f4u32; cpu.w32(0x90u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22dcu32; cpu.w32(0x94u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b22acu32; cpu.w32(0x94u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2294u32; cpu.w32(0x94u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48b609; continue;
 }
 0x48b609 => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x90u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b66b; continue;
 }
 0x48b66b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48b680); f_00492250(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b23ecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b23d4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b23a4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b238cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b236cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48b71e; continue;
 }
 0x48b71e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b794; continue;
 }
 0x48b794 => {
  pc = 0x48b7a0; continue;
 }
 0x48b799 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b7a0; continue;
 }
 0x48b7a0 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48b7c3; continue; }
  pc = 0x48b7b7; continue;
 }
 0x48b7b7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b7d5; continue;
 }
 0x48b7c3 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b7d5; continue;
 }
 0x48b7d5 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48b7f4; continue;
 }
 0x48b7f4 => {
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
pub fn f_0048b38e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b38e); }
 let mut pc: u32 = 0x48b38e;
 loop { match pc {
 0x48b38e => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48b3a6; continue; }
  pc = 0x48b398; continue;
 }
 0x48b398 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x84u32); cpu.ecx = v; }
  cpu.push(0x48b3a6); f_00489b10(cpu);
  pc = 0x48b3a6; continue;
 }
 0x48b3a6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b463(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b463); }
 let mut pc: u32 = 0x48b463;
 loop { match pc {
 0x48b463 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x48b47b; continue; }
  pc = 0x48b46d; continue;
 }
 0x48b46d => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x98u32); cpu.ecx = v; }
  cpu.push(0x48b47b); f_00489b10(cpu);
  pc = 0x48b47b; continue;
 }
 0x48b47b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b4e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b4e4); }
 let mut pc: u32 = 0x48b4e4;
 loop { match pc {
 0x48b4e4 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48b4fc; continue; }
  pc = 0x48b4ee; continue;
 }
 0x48b4ee => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x90u32); cpu.ecx = v; }
  cpu.push(0x48b4fc); f_00490b40(cpu);
  pc = 0x48b4fc; continue;
 }
 0x48b4fc => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b4fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b4fd); }
 let mut pc: u32 = 0x48b4fd;
 loop { match pc {
 0x48b4fd => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x48b515; continue; }
  pc = 0x48b507; continue;
 }
 0x48b507 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8cu32); cpu.ecx = v; }
  cpu.push(0x48b515); f_00489b10(cpu);
  pc = 0x48b515; continue;
 }
 0x48b515 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b620); }
 let mut pc: u32 = 0x48b620;
 loop { match pc {
 0x48b620 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48b638; continue; }
  pc = 0x48b62a; continue;
 }
 0x48b62a => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xa4u32); cpu.ecx = v; }
  cpu.push(0x48b638); f_004913d0(cpu);
  pc = 0x48b638; continue;
 }
 0x48b638 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b639(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b639); }
 let mut pc: u32 = 0x48b639;
 loop { match pc {
 0x48b639 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48b651; continue; }
  pc = 0x48b643; continue;
 }
 0x48b643 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x9cu32); cpu.ecx = v; }
  cpu.push(0x48b651); f_00490b40(cpu);
  pc = 0x48b651; continue;
 }
 0x48b651 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b652(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b652); }
 let mut pc: u32 = 0x48b652;
 loop { match pc {
 0x48b652 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48b66a; continue; }
  pc = 0x48b65c; continue;
 }
 0x48b65c => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x98u32); cpu.ecx = v; }
  cpu.push(0x48b66a); f_00489b10(cpu);
  pc = 0x48b66a; continue;
 }
 0x48b66a => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b72d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b72d); }
 let mut pc: u32 = 0x48b72d;
 loop { match pc {
 0x48b72d => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48b735); f_00492d20(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b736(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b736); }
 let mut pc: u32 = 0x48b736;
 loop { match pc {
 0x48b736 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48b74e; continue; }
  pc = 0x48b740; continue;
 }
 0x48b740 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x90u32); cpu.ecx = v; }
  cpu.push(0x48b74e); f_00491400(cpu);
  pc = 0x48b74e; continue;
 }
 0x48b74e => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b74f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b74f); }
 let mut pc: u32 = 0x48b74f;
 loop { match pc {
 0x48b74f => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48b767; continue; }
  pc = 0x48b759; continue;
 }
 0x48b759 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x84u32); cpu.ecx = v; }
  cpu.push(0x48b767); f_004913d0(cpu);
  pc = 0x48b767; continue;
 }
 0x48b767 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b768(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b768); }
 let mut pc: u32 = 0x48b768;
 loop { match pc {
 0x48b768 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48b77d; continue; }
  pc = 0x48b772; continue;
 }
 0x48b772 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x7cu32); cpu.ecx = v; }
  cpu.push(0x48b77d); f_00490b40(cpu);
  pc = 0x48b77d; continue;
 }
 0x48b77d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b77e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b77e); }
 let mut pc: u32 = 0x48b77e;
 loop { match pc {
 0x48b77e => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48b793; continue; }
  pc = 0x48b788; continue;
 }
 0x48b788 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x74u32); cpu.ecx = v; }
  cpu.push(0x48b793); f_00489b10(cpu);
  pc = 0x48b793; continue;
 }
 0x48b793 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b7dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b7dd); }
 let mut pc: u32 = 0x48b7dd;
 loop { match pc {
 0x48b7dd => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48b7e6); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b7ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b7ea); }
 let mut pc: u32 = 0x48b7ea;
 loop { match pc {
 0x48b7ea => {
  { let v: u32 = 0x4b9b48u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b810); }
 let mut pc: u32 = 0x48b810;
 loop { match pc {
 0x48b810 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48b914u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48b83f; continue;
 }
 0x48b83f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x48b852); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1a6cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f7600u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48b875); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x48b87f); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x48b892); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x48b8a3); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48b8ac); f_0048b8ea(cpu);
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x48b8cd; continue; }
  pc = 0x48b8b9; continue;
 }
 0x48b8b9 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x48b8cd); cpu.call(t);
  pc = 0x48b8cd; continue;
 }
 0x48b8cd => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48b8d6); f_0048b8f4(cpu);
  pc = 0x48b8db; continue;
 }
 0x48b8db => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48b91e; continue;
 }
 0x48b91e => {
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
pub fn f_0048b8ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b8ea); }
 let mut pc: u32 = 0x48b8ea;
 loop { match pc {
 0x48b8ea => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48b8f3); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b8f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b8f4); }
 let mut pc: u32 = 0x48b8f4;
 loop { match pc {
 0x48b8f4 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48b8fd); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b8fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b8fe); }
 let mut pc: u32 = 0x48b8fe;
 loop { match pc {
 0x48b8fe => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48b90a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b90b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b90b); }
 let mut pc: u32 = 0x48b90b;
 loop { match pc {
 0x48b90b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48b913); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b914(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b914); }
 let mut pc: u32 = 0x48b914;
 loop { match pc {
 0x48b914 => {
  { let v: u32 = 0x4b9bd0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048b930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48b930); }
 let mut pc: u32 = 0x48b930;
 loop { match pc {
 0x48b930 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48be64u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x68u32; cpu.push(v); }
  cpu.push(0x48b95f); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48be13; continue; }
  pc = 0x48b976; continue;
 }
 0x48b976 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b24f0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b24e0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b24d8u32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b24c8u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  pc = 0x48b9b3; continue;
 }
 0x48b9b3 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48b9e4; continue;
 }
 0x48b9e4 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x5cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48ba07; continue;
 }
 0x48ba07 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48ba32; continue;
 }
 0x48ba32 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48baf2; continue;
 }
 0x48baf2 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0x64u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0x64u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48bb29; continue;
 }
 0x48bb29 => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48bb6a; continue;
 }
 0x48bb6a => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48bc62; continue;
 }
 0x48bc62 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48bc6c); f_00493510(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2494u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b247cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b244cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2434u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48bce3; continue;
 }
 0x48bce3 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48bd3d; continue;
 }
 0x48bd3d => {
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2564u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b254cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b251cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2504u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48bdb4; continue;
 }
 0x48bdb4 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48be0e; continue;
 }
 0x48be0e => {
  pc = 0x48be1a; continue;
 }
 0x48be13 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48be1a; continue;
 }
 0x48be1a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48be3d; continue; }
  pc = 0x48be31; continue;
 }
 0x48be31 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48be4f; continue;
 }
 0x48be3d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48be4f; continue;
 }
 0x48be4f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48be6e; continue;
 }
 0x48be6e => {
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
pub fn f_0048ba1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48ba1c); }
 let mut pc: u32 = 0x48ba1c;
 loop { match pc {
 0x48ba1c => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48ba31; continue; }
  pc = 0x48ba26; continue;
 }
 0x48ba26 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x60u32); cpu.ecx = v; }
  cpu.push(0x48ba31); f_00489b10(cpu);
  pc = 0x48ba31; continue;
 }
 0x48ba31 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bad1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bad1); }
 let mut pc: u32 = 0x48bad1;
 loop { match pc {
 0x48bad1 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48bae6; continue; }
  pc = 0x48badb; continue;
 }
 0x48badb => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x74u32); cpu.ecx = v; }
  cpu.push(0x48bae6); f_00489b10(cpu);
  pc = 0x48bae6; continue;
 }
 0x48bae6 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bb3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bb3e); }
 let mut pc: u32 = 0x48bb3e;
 loop { match pc {
 0x48bb3e => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x48bb53; continue; }
  pc = 0x48bb48; continue;
 }
 0x48bb48 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x6cu32); cpu.ecx = v; }
  cpu.push(0x48bb53); f_00490b40(cpu);
  pc = 0x48bb53; continue;
 }
 0x48bb53 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bb54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bb54); }
 let mut pc: u32 = 0x48bb54;
 loop { match pc {
 0x48bb54 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48bb69; continue; }
  pc = 0x48bb5e; continue;
 }
 0x48bb5e => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x68u32); cpu.ecx = v; }
  cpu.push(0x48bb69); f_00489b10(cpu);
  pc = 0x48bb69; continue;
 }
 0x48bb69 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bc23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bc23); }
 let mut pc: u32 = 0x48bc23;
 loop { match pc {
 0x48bc23 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48bc38; continue; }
  pc = 0x48bc2d; continue;
 }
 0x48bc2d => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x58u32); cpu.ecx = v; }
  cpu.push(0x48bc38); f_00489b10(cpu);
  pc = 0x48bc38; continue;
 }
 0x48bc38 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bcf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bcf2); }
 let mut pc: u32 = 0x48bcf2;
 loop { match pc {
 0x48bcf2 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48bcfa); f_00493bf0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bcfb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bcfb); }
 let mut pc: u32 = 0x48bcfb;
 loop { match pc {
 0x48bcfb => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48bd10; continue; }
  pc = 0x48bd05; continue;
 }
 0x48bd05 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x58u32); cpu.ecx = v; }
  cpu.push(0x48bd10); f_004913d0(cpu);
  pc = 0x48bd10; continue;
 }
 0x48bd10 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bd11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bd11); }
 let mut pc: u32 = 0x48bd11;
 loop { match pc {
 0x48bd11 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48bd26; continue; }
  pc = 0x48bd1b; continue;
 }
 0x48bd1b => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  cpu.push(0x48bd26); f_00490b40(cpu);
  pc = 0x48bd26; continue;
 }
 0x48bd26 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bd27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bd27); }
 let mut pc: u32 = 0x48bd27;
 loop { match pc {
 0x48bd27 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x48bd3c; continue; }
  pc = 0x48bd31; continue;
 }
 0x48bd31 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x48u32); cpu.ecx = v; }
  cpu.push(0x48bd3c); f_00489b10(cpu);
  pc = 0x48bd3c; continue;
 }
 0x48bd3c => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bdc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bdc3); }
 let mut pc: u32 = 0x48bdc3;
 loop { match pc {
 0x48bdc3 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48bdcb); f_00492de0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bdcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bdcc); }
 let mut pc: u32 = 0x48bdcc;
 loop { match pc {
 0x48bdcc => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48bde1; continue; }
  pc = 0x48bdd6; continue;
 }
 0x48bdd6 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x60u32); cpu.ecx = v; }
  cpu.push(0x48bde1); f_004913d0(cpu);
  pc = 0x48bde1; continue;
 }
 0x48bde1 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bde2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bde2); }
 let mut pc: u32 = 0x48bde2;
 loop { match pc {
 0x48bde2 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48bdf7; continue; }
  pc = 0x48bdec; continue;
 }
 0x48bdec => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x58u32); cpu.ecx = v; }
  cpu.push(0x48bdf7); f_00490b40(cpu);
  pc = 0x48bdf7; continue;
 }
 0x48bdf7 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bdf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bdf8); }
 let mut pc: u32 = 0x48bdf8;
 loop { match pc {
 0x48bdf8 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x48be0d; continue; }
  pc = 0x48be02; continue;
 }
 0x48be02 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x50u32); cpu.ecx = v; }
  cpu.push(0x48be0d); f_00489b10(cpu);
  pc = 0x48be0d; continue;
 }
 0x48be0d => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048be57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48be57); }
 let mut pc: u32 = 0x48be57;
 loop { match pc {
 0x48be57 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48be60); f_004ae7a0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048be64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48be64); }
 let mut pc: u32 = 0x48be64;
 loop { match pc {
 0x48be64 => {
  { let v: u32 = 0x4b9c10u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048be80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48be80); }
 let mut pc: u32 = 0x48be80;
 loop { match pc {
 0x48be80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48bf84u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b196cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48beaf; continue;
 }
 0x48beaf => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256ecu32); cpu.push(0x48bec2); cpu.call(t);
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1a4cu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4f7680u32; cpu.ecx = v; }
  let t = cpu.r32(0x525638u32); cpu.push(0x48bee5); cpu.call(t);
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525714u32); cpu.push(0x48beef); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x525788u32); cpu.push(0x48bf02); cpu.call(t);
  { let v: u8 = 0x3u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x5256e8u32); cpu.push(0x48bf13); cpu.call(t);
  { let v: u8 = 0x2u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48bf1c); f_0048bf5a(cpu);
  { cpu.sub32(cpu.r32(0x4f68f0u32), 0x0u32); }
  if cpu.zf { pc = 0x48bf3d; continue; }
  pc = 0x48bf29; continue;
 }
 0x48bf29 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4f68f0u32); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.push(0x48bf3d); cpu.call(t);
  pc = 0x48bf3d; continue;
 }
 0x48bf3d => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48bf46); f_0048bf64(cpu);
  pc = 0x48bf4b; continue;
 }
 0x48bf4b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48bf8e; continue;
 }
 0x48bf8e => {
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
pub fn f_0048bf5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bf5a); }
 let mut pc: u32 = 0x48bf5a;
 loop { match pc {
 0x48bf5a => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48bf63); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bf64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bf64); }
 let mut pc: u32 = 0x48bf64;
 loop { match pc {
 0x48bf64 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48bf6d); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bf6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bf6e); }
 let mut pc: u32 = 0x48bf6e;
 loop { match pc {
 0x48bf6e => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  let t = cpu.r32(0x52571cu32); cpu.push(0x48bf7a); cpu.call(t);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bf7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bf7b); }
 let mut pc: u32 = 0x48bf7b;
 loop { match pc {
 0x48bf7b => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48bf83); f_00487c80(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bf84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bf84); }
 let mut pc: u32 = 0x48bf84;
 loop { match pc {
 0x48bf84 => {
  { let v: u32 = 0x4b9ca0u32; cpu.eax = v; }
  pc = 0x4ae830; continue;
 }
 0x4ae830 => {
  pc = cpu.r32(0x5256ccu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048bfa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48bfa0); }
 let mut pc: u32 = 0x48bfa0;
 loop { match pc {
 0x48bfa0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x48c4d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x30u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x68u32; cpu.push(v); }
  cpu.push(0x48bfcf); f_004ae83c(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x48c483; continue; }
  pc = 0x48bfe6; continue;
 }
 0x48bfe6 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b25c0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b25b0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b25a8u32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2598u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  pc = 0x48c023; continue;
 }
 0x48c023 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x1u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c054; continue;
 }
 0x48c054 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x5cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c077; continue;
 }
 0x48c077 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c0a2; continue;
 }
 0x48c0a2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c162; continue;
 }
 0x48c162 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0x64u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0x64u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c199; continue;
 }
 0x48c199 => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c1da; continue;
 }
 0x48c1da => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c2d2; continue;
 }
 0x48c2d2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48c2dc); f_00493510(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2494u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b247cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b244cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2434u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c353; continue;
 }
 0x48c353 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c3ad; continue;
 }
 0x48c3ad => {
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2634u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b261cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b25ecu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b25d4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c424; continue;
 }
 0x48c424 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c47e; continue;
 }
 0x48c47e => {
  pc = 0x48c48a; continue;
 }
 0x48c483 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c48a; continue;
 }
 0x48c48a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48c4ad; continue; }
  pc = 0x48c4a1; continue;
 }
 0x48c4a1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c4bf; continue;
 }
 0x48c4ad => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c4bf; continue;
 }
 0x48c4bf => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48c4de; continue;
 }
 0x48c4de => {
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
pub fn f_0048c00f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c00f); }
 let mut pc: u32 = 0x48c00f;
 loop { match pc {
 0x48c00f => {
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x458b004bu32); cpu.eax = v; }
  cpu.unimplemented(0x48c016, "Enter");
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xe9004bu32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xc601e04du32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0xc601e04du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d00u32; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d54u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  pc = 0x48c049; continue;
 }
 0x48c049 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x8u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x5cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c077; continue;
 }
 0x48c077 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c0a2; continue;
 }
 0x48c0a2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c162; continue;
 }
 0x48c162 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0x64u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0x64u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c199; continue;
 }
 0x48c199 => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c1da; continue;
 }
 0x48c1da => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c2d2; continue;
 }
 0x48c2d2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48c2dc); f_00493510(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2494u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b247cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b244cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2434u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c353; continue;
 }
 0x48c353 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c3ad; continue;
 }
 0x48c3ad => {
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2634u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b261cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b25ecu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b25d4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c424; continue;
 }
 0x48c424 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c47e; continue;
 }
 0x48c47e => {
  pc = 0x48c48a; continue;
 }
 0x48c48a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48c4ad; continue; }
  pc = 0x48c4a1; continue;
 }
 0x48c4a1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c4bf; continue;
 }
 0x48c4ad => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c4bf; continue;
 }
 0x48c4bf => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48c4de; continue;
 }
 0x48c4de => {
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
pub fn f_0048c04b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c04b); }
 let mut pc: u32 = 0x48c04b;
 loop { match pc {
 0x48c04b => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x48c055; continue; }
  pc = 0x48c04d; continue;
 }
 0x48c04d => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b1d24u32; cpu.w32(0x58u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x5cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c077; continue;
 }
 0x48c055 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.unimplemented(0x48c056, "Enter");
  { let v: u8 = (cpu.eax as u8) & 0x1du8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x408bc845u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x408bc845u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b1d0cu32; cpu.w32(0x5cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c077; continue;
 }
 0x48c077 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c0a2; continue;
 }
 0x48c0a2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x2u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c162; continue;
 }
 0x48c162 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2134u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b211cu32; cpu.w32(0x64u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b20ecu32; cpu.w32(0x64u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c199; continue;
 }
 0x48c199 => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c1da; continue;
 }
 0x48c1da => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)) | 0x4u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c2d2; continue;
 }
 0x48c2d2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48c2dc); f_00493510(cpu);
  { let v: u32 = 0xcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2494u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b247cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b244cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b2434u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c353; continue;
 }
 0x48c353 => {
  { let v: u8 = 0x7u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c3ad; continue;
 }
 0x48c3ad => {
  { let v: u32 = 0xdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x4b2634u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b261cu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b25ecu32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x4b25d4u32; cpu.w32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x48c424; continue;
 }
 0x48c424 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c47e; continue;
 }
 0x48c47e => {
  pc = 0x48c48a; continue;
 }
 0x48c48a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x48c4ad; continue; }
  pc = 0x48c4a1; continue;
 }
 0x48c4a1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c4bf; continue;
 }
 0x48c4ad => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x48c4bf; continue;
 }
 0x48c4bf => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x48c4de; continue;
 }
 0x48c4de => {
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
pub fn f_0048c08c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c08c); }
 let mut pc: u32 = 0x48c08c;
 loop { match pc {
 0x48c08c => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x48c0a1; continue; }
  pc = 0x48c096; continue;
 }
 0x48c096 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x60u32); cpu.ecx = v; }
  cpu.push(0x48c0a1); f_00489b10(cpu);
  pc = 0x48c0a1; continue;
 }
 0x48c0a1 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c141(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c141); }
 let mut pc: u32 = 0x48c141;
 loop { match pc {
 0x48c141 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x48c156; continue; }
  pc = 0x48c14b; continue;
 }
 0x48c14b => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x74u32); cpu.ecx = v; }
  cpu.push(0x48c156); f_00489b10(cpu);
  pc = 0x48c156; continue;
 }
 0x48c156 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c1ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c1ae); }
 let mut pc: u32 = 0x48c1ae;
 loop { match pc {
 0x48c1ae => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x48c1c3; continue; }
  pc = 0x48c1b8; continue;
 }
 0x48c1b8 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x6cu32); cpu.ecx = v; }
  cpu.push(0x48c1c3); f_00490b40(cpu);
  pc = 0x48c1c3; continue;
 }
 0x48c1c3 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c1c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c1c4); }
 let mut pc: u32 = 0x48c1c4;
 loop { match pc {
 0x48c1c4 => {
  { cpu.and8(cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x48c1d9; continue; }
  pc = 0x48c1ce; continue;
 }
 0x48c1ce => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x68u32); cpu.ecx = v; }
  cpu.push(0x48c1d9); f_00489b10(cpu);
  pc = 0x48c1d9; continue;
 }
 0x48c1d9 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c293(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c293); }
 let mut pc: u32 = 0x48c293;
 loop { match pc {
 0x48c293 => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x48c2a8; continue; }
  pc = 0x48c29d; continue;
 }
 0x48c29d => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x58u32); cpu.ecx = v; }
  cpu.push(0x48c2a8); f_00489b10(cpu);
  pc = 0x48c2a8; continue;
 }
 0x48c2a8 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c362(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c362); }
 let mut pc: u32 = 0x48c362;
 loop { match pc {
 0x48c362 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x48c36a); f_00493bf0(cpu);
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0048c36b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x48c36b); }
 let mut pc: u32 = 0x48c36b;
 loop { match pc {
 0x48c36b => {
  { cpu.and8(cpu.r8(0xffffffe1u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x48c380; continue; }
  pc = 0x48c375; continue;
 }
 0x48c375 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x58u32); cpu.ecx = v; }
  cpu.push(0x48c380); f_004913d0(cpu);
  pc = 0x48c380; continue;
 }
 0x48c380 => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
