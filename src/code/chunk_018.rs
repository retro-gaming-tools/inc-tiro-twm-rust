#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_110034dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110034dc); }
 let mut pc: u32 = 0x110034dc;
 loop { match pc {
 0x110034dc => {
  { cpu.and8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x110034ea; continue; }
  pc = 0x110034e2; continue;
 }
 0x110034e2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 0x110034ea => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110034eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110034eb); }
 let mut pc: u32 = 0x110034eb;
 loop { match pc {
 0x110034eb => {
  { let v: u32 = 0x11007cb8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110034f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110034f5); }
 let mut pc: u32 = 0x110034f5;
 loop { match pc {
 0x110034f5 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110035dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110035dc); }
 let mut pc: u32 = 0x110035dc;
 loop { match pc {
 0x110035dc => {
  { let v: u32 = 0x11007d10u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110035e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110035e6); }
 let mut pc: u32 = 0x110035e6;
 loop { match pc {
 0x110035e6 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003673(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003673); }
 let mut pc: u32 = 0x11003673;
 loop { match pc {
 0x11003673 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100367e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100367e); }
 let mut pc: u32 = 0x1100367e;
 loop { match pc {
 0x1100367e => {
  { let v: u32 = 0x11007d40u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003688(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003688); }
 let mut pc: u32 = 0x11003688;
 loop { match pc {
 0x11003688 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003725(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003725); }
 let mut pc: u32 = 0x11003725;
 loop { match pc {
 0x11003725 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003730); }
 let mut pc: u32 = 0x11003730;
 loop { match pc {
 0x11003730 => {
  { let v: u32 = 0x11007d78u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100373a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100373a); }
 let mut pc: u32 = 0x1100373a;
 loop { match pc {
 0x1100373a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110037a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110037a7); }
 let mut pc: u32 = 0x110037a7;
 loop { match pc {
 0x110037a7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110037b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110037b2); }
 let mut pc: u32 = 0x110037b2;
 loop { match pc {
 0x110037b2 => {
  { let v: u32 = 0x11007db0u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110037bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110037bc); }
 let mut pc: u32 = 0x110037bc;
 loop { match pc {
 0x110037bc => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100384c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100384c); }
 let mut pc: u32 = 0x1100384c;
 loop { match pc {
 0x1100384c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003857(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003857); }
 let mut pc: u32 = 0x11003857;
 loop { match pc {
 0x11003857 => {
  { let v: u32 = 0x11007de8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003861(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003861); }
 let mut pc: u32 = 0x11003861;
 loop { match pc {
 0x11003861 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x4u32); cpu.ecx = v; }
  f_11001087(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110039d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110039d6); }
 let mut pc: u32 = 0x110039d6;
 loop { match pc {
 0x110039d6 => {
  { let v: u32 = 0x11007e20u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110039e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110039e0); }
 let mut pc: u32 = 0x110039e0;
 loop { match pc {
 0x110039e0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001113(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003ab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003ab0); }
 let mut pc: u32 = 0x11003ab0;
 loop { match pc {
 0x11003ab0 => {
  { let v: u32 = 0x11007e50u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003aba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003aba); }
 let mut pc: u32 = 0x11003aba;
 loop { match pc {
 0x11003aba => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100131b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003b23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003b23); }
 let mut pc: u32 = 0x11003b23;
 loop { match pc {
 0x11003b23 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001113(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003b2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003b2b); }
 let mut pc: u32 = 0x11003b2b;
 loop { match pc {
 0x11003b2b => {
  { let v: u32 = 0x11007e80u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003b35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003b35); }
 let mut pc: u32 = 0x11003b35;
 loop { match pc {
 0x11003b35 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100131b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003b85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003b85); }
 let mut pc: u32 = 0x11003b85;
 loop { match pc {
 0x11003b85 => {
  { let v: u32 = 0x11007eb8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003b8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003b8f); }
 let mut pc: u32 = 0x11003b8f;
 loop { match pc {
 0x11003b8f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100131b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003be5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003be5); }
 let mut pc: u32 = 0x11003be5;
 loop { match pc {
 0x11003be5 => {
  { let v: u32 = 0x11007ee8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003bef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003bef); }
 let mut pc: u32 = 0x11003bef;
 loop { match pc {
 0x11003bef => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100131b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003c68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003c68); }
 let mut pc: u32 = 0x11003c68;
 loop { match pc {
 0x11003c68 => {
  { let v: u32 = 0x11007f18u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003c72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003c72); }
 let mut pc: u32 = 0x11003c72;
 loop { match pc {
 0x11003c72 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100131b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003dd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003dd4); }
 let mut pc: u32 = 0x11003dd4;
 loop { match pc {
 0x11003dd4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  f_11001276(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003ddf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003ddf); }
 let mut pc: u32 = 0x11003ddf;
 loop { match pc {
 0x11001104 => {
  pc = 0x11003e25; continue;
 }
 0x11003ddf => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  pc = 0x11001104; continue;
 }
 0x11003e25 => {
  { let v: u32 = 0x11003e4eu32; cpu.eax = v; }
  cpu.push(0x11003e2f); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003e41); f_11003e58(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003dea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003dea); }
 let mut pc: u32 = 0x11003dea;
 loop { match pc {
 0x11003dea => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100132a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003df2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003df2); }
 let mut pc: u32 = 0x11003df2;
 loop { match pc {
 0x11003df2 => {
  { let v: u32 = 0x11007f48u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003dfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003dfc); }
 let mut pc: u32 = 0x11003dfc;
 loop { match pc {
 0x11001370 => {
  pc = 0x1100481d; continue;
 }
 0x11003dfc => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11001370; continue;
 }
 0x1100481d => {
  { let v: u32 = 0x11004858u32; cpu.eax = v; }
  cpu.push(0x11004827); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x110070b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100483f); f_11001172(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100484b); f_11004862(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003e4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003e4e); }
 let mut pc: u32 = 0x11003e4e;
 loop { match pc {
 0x11003e4e => {
  { let v: u32 = 0x11007f90u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003e58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003e58); }
 let mut pc: u32 = 0x11003e58;
 loop { match pc {
 0x11003e58 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001276(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003ece(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003ece); }
 let mut pc: u32 = 0x11003ece;
 loop { match pc {
 0x11003ece => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  f_11001276(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003ed9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003ed9); }
 let mut pc: u32 = 0x11003ed9;
 loop { match pc {
 0x11001104 => {
  pc = 0x11003e25; continue;
 }
 0x11003e25 => {
  { let v: u32 = 0x11003e4eu32; cpu.eax = v; }
  cpu.push(0x11003e2f); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003e41); f_11003e58(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x11003ed9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  pc = 0x11001104; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003ee4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003ee4); }
 let mut pc: u32 = 0x11003ee4;
 loop { match pc {
 0x11003ee4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100132a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003eec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003eec); }
 let mut pc: u32 = 0x11003eec;
 loop { match pc {
 0x11003eec => {
  { let v: u32 = 0x11007fc0u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003ef6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003ef6); }
 let mut pc: u32 = 0x11003ef6;
 loop { match pc {
 0x11001370 => {
  pc = 0x1100481d; continue;
 }
 0x11003ef6 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11001370; continue;
 }
 0x1100481d => {
  { let v: u32 = 0x11004858u32; cpu.eax = v; }
  cpu.push(0x11004827); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x110070b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100483f); f_11001172(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100484b); f_11004862(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003fc1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003fc1); }
 let mut pc: u32 = 0x11003fc1;
 loop { match pc {
 0x11003fc1 => {
  { let v: u32 = 0x11008008u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11003fcb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11003fcb); }
 let mut pc: u32 = 0x11003fcb;
 loop { match pc {
 0x11003fcb => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11003fd3); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004049(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004049); }
 let mut pc: u32 = 0x11004049;
 loop { match pc {
 0x11001104 => {
  pc = 0x11003e25; continue;
 }
 0x11003e25 => {
  { let v: u32 = 0x11003e4eu32; cpu.eax = v; }
  cpu.push(0x11003e2f); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11003e41); f_11003e58(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x11004049 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  pc = 0x11001104; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004054); }
 let mut pc: u32 = 0x11004054;
 loop { match pc {
 0x11004054 => {
  { let v: u32 = 0x11008038u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100405e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100405e); }
 let mut pc: u32 = 0x1100405e;
 loop { match pc {
 0x11001370 => {
  pc = 0x1100481d; continue;
 }
 0x1100405e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11001370; continue;
 }
 0x1100481d => {
  { let v: u32 = 0x11004858u32; cpu.eax = v; }
  cpu.push(0x11004827); f_11005e26(cpu);
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x110070b4u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100483f); f_11001172(cpu);
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1100484b); f_11004862(cpu);
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.fs_base), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110040fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110040fc); }
 let mut pc: u32 = 0x110040fc;
 loop { match pc {
 0x110040fc => {
  { let v: u32 = 0x11008070u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004106(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004106); }
 let mut pc: u32 = 0x11004106;
 loop { match pc {
 0x11004106 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1100410e); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110041af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110041af); }
 let mut pc: u32 = 0x110041af;
 loop { match pc {
 0x110041af => {
  { let v: u32 = 0x110080a0u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110041b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110041b9); }
 let mut pc: u32 = 0x110041b9;
 loop { match pc {
 0x110041b9 => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x110041c1); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004380); }
 let mut pc: u32 = 0x11004380;
 loop { match pc {
 0x11004380 => {
  { let v = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x11004388); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100438c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100438c); }
 let mut pc: u32 = 0x1100438c;
 loop { match pc {
 0x1100438c => {
  { let v: u32 = 0x110080d0u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004396(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004396); }
 let mut pc: u32 = 0x11004396;
 loop { match pc {
 0x11004396 => {
  { let v = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x1100439e); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004643(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004643); }
 let mut pc: u32 = 0x11004643;
 loop { match pc {
 0x11004643 => {
  { let v: u32 = 0x11008108u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100464d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100464d); }
 let mut pc: u32 = 0x1100464d;
 loop { match pc {
 0x1100464d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001221(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110046a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110046a3); }
 let mut pc: u32 = 0x110046a3;
 loop { match pc {
 0x110046a3 => {
  { let v: u32 = 0x11008138u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110046ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110046ad); }
 let mut pc: u32 = 0x110046ad;
 loop { match pc {
 0x110046ad => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001221(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004726(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004726); }
 let mut pc: u32 = 0x11004726;
 loop { match pc {
 0x11004726 => {
  { let v: u32 = 0x11008168u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004730); }
 let mut pc: u32 = 0x11004730;
 loop { match pc {
 0x11004730 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001221(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004858(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004858); }
 let mut pc: u32 = 0x11004858;
 loop { match pc {
 0x11004858 => {
  { let v: u32 = 0x11008198u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004862(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004862); }
 let mut pc: u32 = 0x11004862;
 loop { match pc {
 0x11004862 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100132a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110048d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110048d4); }
 let mut pc: u32 = 0x110048d4;
 loop { match pc {
 0x110048d4 => {
  { let v: u32 = 0x110081c8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110048de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110048de); }
 let mut pc: u32 = 0x110048de;
 loop { match pc {
 0x110048de => {
  { let v = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x110048e6); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004969(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004969); }
 let mut pc: u32 = 0x11004969;
 loop { match pc {
 0x11004969 => {
  { let v: u32 = 0x110081f8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004973(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004973); }
 let mut pc: u32 = 0x11004973;
 loop { match pc {
 0x11004973 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_1100132a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004b74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004b74); }
 let mut pc: u32 = 0x11004b74;
 loop { match pc {
 0x11004b74 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_11001221(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004b7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004b7c); }
 let mut pc: u32 = 0x11004b7c;
 loop { match pc {
 0x11004b7c => {
  { let v: u32 = 0x11008228u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004fd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004fd2); }
 let mut pc: u32 = 0x11004fd2;
 loop { match pc {
 0x11004fd2 => {
  { let v: u32 = 0x11008258u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11004fdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11004fdc); }
 let mut pc: u32 = 0x11004fdc;
 loop { match pc {
 0x11004fdc => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001221(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005028(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005028); }
 let mut pc: u32 = 0x11005028;
 loop { match pc {
 0x11005028 => {
  { let v: u32 = 0x11008288u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005032(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005032); }
 let mut pc: u32 = 0x11005032;
 loop { match pc {
 0x11005032 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001221(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005080); }
 let mut pc: u32 = 0x11005080;
 loop { match pc {
 0x11005080 => {
  { let v: u32 = 0x110082b8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100508a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100508a); }
 let mut pc: u32 = 0x1100508a;
 loop { match pc {
 0x1100508a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_11001221(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100535e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100535e); }
 let mut pc: u32 = 0x1100535e;
 loop { match pc {
 0x1100535e => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005366(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005366); }
 let mut pc: u32 = 0x11005366;
 loop { match pc {
 0x11005366 => {
  { let v: u32 = 0x110082e8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005370); }
 let mut pc: u32 = 0x11005370;
 loop { match pc {
 0x11005370 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010c8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005455(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005455); }
 let mut pc: u32 = 0x11005455;
 loop { match pc {
 0x11005455 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100545d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100545d); }
 let mut pc: u32 = 0x1100545d;
 loop { match pc {
 0x1100545d => {
  { let v: u32 = 0x11008320u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005467(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005467); }
 let mut pc: u32 = 0x11005467;
 loop { match pc {
 0x11005467 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010c8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110054d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110054d1); }
 let mut pc: u32 = 0x110054d1;
 loop { match pc {
 0x110054d1 => {
  { let v: u32 = 0x11008358u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110054db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110054db); }
 let mut pc: u32 = 0x110054db;
 loop { match pc {
 0x110054db => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010c8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110055dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110055dd); }
 let mut pc: u32 = 0x110055dd;
 loop { match pc {
 0x110055dd => {
  { let v: u32 = 0x11008388u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110055e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110055e7); }
 let mut pc: u32 = 0x110055e7;
 loop { match pc {
 0x110055e7 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  f_1100115e(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005666(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005666); }
 let mut pc: u32 = 0x11005666;
 loop { match pc {
 0x11005666 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100566e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100566e); }
 let mut pc: u32 = 0x1100566e;
 loop { match pc {
 0x1100566e => {
  { let v: u32 = 0x110083b8u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005678(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005678); }
 let mut pc: u32 = 0x11005678;
 loop { match pc {
 0x11005678 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010c8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005770); }
 let mut pc: u32 = 0x11005770;
 loop { match pc {
 0x11005770 => {
  { let v: u32 = 0x1100d910u32; cpu.ecx = v; }
  f_110010be(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005850); }
 let mut pc: u32 = 0x11005850;
 loop { match pc {
 0x11005850 => {
  { let v: u32 = 0x110083f0u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100585a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100585a); }
 let mut pc: u32 = 0x1100585a;
 loop { match pc {
 0x1100585a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110058b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110058b6); }
 let mut pc: u32 = 0x110058b6;
 loop { match pc {
 0x110058b6 => {
  { let v: u32 = 0x11008420u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110058c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110058c0); }
 let mut pc: u32 = 0x110058c0;
 loop { match pc {
 0x110058c0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100590a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100590a); }
 let mut pc: u32 = 0x1100590a;
 loop { match pc {
 0x1100590a => {
  { let v: u32 = 0x11008450u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005914(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005914); }
 let mut pc: u32 = 0x11005914;
 loop { match pc {
 0x11005914 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100596a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100596a); }
 let mut pc: u32 = 0x1100596a;
 loop { match pc {
 0x1100596a => {
  { let v: u32 = 0x11008480u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005974(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005974); }
 let mut pc: u32 = 0x11005974;
 loop { match pc {
 0x11005974 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110059ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110059ed); }
 let mut pc: u32 = 0x110059ed;
 loop { match pc {
 0x110059ed => {
  { let v: u32 = 0x110084b0u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110059f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110059f7); }
 let mut pc: u32 = 0x110059f7;
 loop { match pc {
 0x110059f7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005a71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005a71); }
 let mut pc: u32 = 0x11005a71;
 loop { match pc {
 0x11005a71 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010aa(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005a79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005a79); }
 let mut pc: u32 = 0x11005a79;
 loop { match pc {
 0x11005a79 => {
  { let v: u32 = 0x110084e0u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005a83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005a83); }
 let mut pc: u32 = 0x11005a83;
 loop { match pc {
 0x11005a83 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010c8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005ae2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005ae2); }
 let mut pc: u32 = 0x11005ae2;
 loop { match pc {
 0x11005ae2 => {
  { let v: u32 = 0x11008518u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005aec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005aec); }
 let mut pc: u32 = 0x11005aec;
 loop { match pc {
 0x11005aec => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010c8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005b4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005b4c); }
 let mut pc: u32 = 0x11005b4c;
 loop { match pc {
 0x11005b4c => {
  { let v: u32 = 0x11008548u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005b56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005b56); }
 let mut pc: u32 = 0x11005b56;
 loop { match pc {
 0x11005b56 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010c8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005bcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005bcf); }
 let mut pc: u32 = 0x11005bcf;
 loop { match pc {
 0x11005bcf => {
  { let v: u32 = 0x11008578u32; cpu.eax = v; }
  pc = 0x11005e20; continue;
 }
 0x11005e20 => {
  pc = cpu.r32(0x1100e21cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005bd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005bd9); }
 let mut pc: u32 = 0x11005bd9;
 loop { match pc {
 0x11005bd9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  f_110010c8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005c78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005c78); }
 let mut pc: u32 = 0x11005c78;
 loop { match pc {
 0x11005c78 => {
  pc = cpu.r32(0x1100e284u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005c90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005c90); }
 let mut pc: u32 = 0x11005c90;
 loop { match pc {
 0x11005c90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  cpu.push(0x11005c98); f_11006160(cpu);
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x11005ca8; continue; }
  pc = 0x11005c9f; continue;
 }
 0x11005c9f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11005ca5); f_11005cb6(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11005ca8; continue;
 }
 0x11005ca8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005cb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005cb6); }
 let mut pc: u32 = 0x11005cb6;
 loop { match pc {
 0x11005cb6 => {
  pc = cpu.r32(0x1100e274u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005cc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005cc0); }
 let mut pc: u32 = 0x11005cc0;
 loop { match pc {
 0x11005cc0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x110071a0u32; cpu.push(v); }
  { let v = 0x11006166u32; cpu.push(v); }
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
  if cpu.sf { pc = 0x11005d0b; continue; }
  pc = 0x11005cfa; continue;
 }
 0x11005cfa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x11005d06); cpu.call(t);
  { let v: u32 = cpu.dec32(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x11005cfa; continue; }
  pc = 0x11005d0b; continue;
 }
 0x11005d0b => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x11005d1e); f_11005d30(cpu);
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
pub fn f_11005d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005d30); }
 let mut pc: u32 = 0x11005d30;
 loop { match pc {
 0x11005d30 => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x11005d4b; continue; }
  pc = 0x11005d36; continue;
 }
 0x11005d36 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11005d4b); f_11005d50(cpu);
  pc = 0x11005d4b; continue;
 }
 0x11005d4b => {
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005d50); }
 let mut pc: u32 = 0x11005d50;
 loop { match pc {
 0x11005d50 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.fs_base)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = 0x110071b0u32; cpu.push(v); }
  { let v = 0x11006166u32; cpu.push(v); }
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
  if cpu.sf { pc = 0x11005dae; continue; }
  pc = 0x11005d86; continue;
 }
 0x11005d86 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x11005d8c; continue;
 }
 0x11005d8c => {
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  let t = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.push(0x11005d96); cpu.call(t);
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x11005d8c; continue; }
  pc = 0x11005d9c; continue;
 }
 0x11005d9c => {
  pc = 0x11005dae; continue;
 }
 0x11005dae => {
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
pub fn f_11005d9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005d9e); }
 let mut pc: u32 = 0x11005d9e;
 loop { match pc {
 0x11005d9e => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11005da7); f_11005dd0(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005dab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005dab); }
 let mut pc: u32 = 0x11005dab;
 loop { match pc {
 0x11005dab => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.esp = v; }
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
pub fn f_11005dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005dd0); }
 let mut pc: u32 = 0x11005dd0;
 loop { match pc {
 0x11005dd0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0xe06d7363u32); }
  if !cpu.zf { pc = 0x11005de3; continue; }
  pc = 0x11005dde; continue;
 }
 0x11005dde => {
  cpu.push(0x11005de3); f_1100616c(cpu);
  pc = 0x11005de3; continue;
 }
 0x11005de3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005de6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005de6); }
 let mut pc: u32 = 0x11005de6;
 loop { match pc {
 0x11005de6 => {
  pc = cpu.r32(0x1100e27cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005dec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005dec); }
 let mut pc: u32 = 0x11005dec;
 loop { match pc {
 0x11005dec => {
  pc = cpu.r32(0x1100e278u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005e26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005e26); }
 let mut pc: u32 = 0x11005e26;
 loop { match pc {
 0x11005e26 => {
  pc = cpu.r32(0x1100e20cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005e44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005e44); }
 let mut pc: u32 = 0x11005e44;
 loop { match pc {
 0x11005e44 => {
  pc = cpu.r32(0x1100e224u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005e60); }
 let mut pc: u32 = 0x11005e60;
 loop { match pc {
 0x11005e60 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1100d7a0u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x11005e8f; continue; }
  pc = 0x11005e6b; continue;
 }
 0x11005e6b => {
  let t = cpu.r32(0x1100e198u32); cpu.push(0x11005e71); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  { let v: u32 = cpu.r32(0x1100d7a0u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x11005e8e; continue; }
  pc = 0x11005e84; continue;
 }
 0x11005e84 => {
  { cpu.and32(cpu.eax, 0x80000000u32); }
  if cpu.zf { pc = 0x11005e8e; continue; }
  pc = 0x11005e8b; continue;
 }
 0x11005e8b => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x11005e8f; continue;
 }
 0x11005e8e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x11005e8f; continue;
 }
 0x11005e8f => {
  { let v: u32 = cpu.ecx; cpu.w32(0x1100d7a0u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x11005ebb; continue; }
  pc = 0x11005e99; continue;
 }
 0x11005e99 => {
  { let v: u32 = cpu.r32(0x1100d7a4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x11005eb5; continue; }
  pc = 0x11005ea2; continue;
 }
 0x11005ea2 => {
  { let v: u32 = cpu.r32(0x1100e19cu32); cpu.esi = v; }
  pc = 0x11005ea8; continue;
 }
 0x11005ea8 => {
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.esi; cpu.push(0x11005eac); cpu.call(t);
  { let v: u32 = cpu.r32(0x1100d7a4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x11005ea8; continue; }
  pc = 0x11005eb5; continue;
 }
 0x11005eb5 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1100d7a4u32, v); }
  pc = 0x11005ebb; continue;
 }
 0x11005ebb => {
  { cpu.sub32(cpu.r32(0x1100d948u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x11005ed4; continue; }
  pc = 0x11005ec4; continue;
 }
 0x11005ec4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1100e250u32); cpu.push(0x11005ecf); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x11005eeb; continue;
 }
 0x11005ed4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x1100d938u32; cpu.push(v); }
  { let v = 0x1100d948u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11005ee8); f_11006172(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x11005eeb; continue;
 }
 0x11005eeb => {
  { cpu.sub32(cpu.r32(0x1100d7a0u32), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x11005efa; continue; }
  pc = 0x11005ef4; continue;
 }
 0x11005ef4 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1100d7a4u32)); cpu.w32(0x1100d7a4u32, v); }
  pc = 0x11005efa; continue;
 }
 0x11005efa => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005f00); }
 let mut pc: u32 = 0x11005f00;
 loop { match pc {
 0x11005f00 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x11005f0a); f_11005e60(cpu);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005f14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005f14); }
 let mut pc: u32 = 0x11005f14;
 loop { match pc {
 0x11005f14 => {
  pc = cpu.r32(0x1100e280u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005f1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005f1a); }
 let mut pc: u32 = 0x11005f1a;
 loop { match pc {
 0x11005f1a => {
  pc = cpu.r32(0x1100e288u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005f20); }
 let mut pc: u32 = 0x11005f20;
 loop { match pc {
 0x11005f20 => {
  pc = cpu.r32(0x1100e23cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11005f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11005f30); }
 let mut pc: u32 = 0x11005f30;
 loop { match pc {
 0x11005f30 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x11005f48; continue; }
  pc = 0x11005f39; continue;
 }
 0x11005f39 => {
  { cpu.sub32(cpu.r32(0x1100d7a8u32), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x11005f6a; continue; }
  pc = 0x11005f42; continue;
 }
 0x11005f42 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1100d7a8u32)); cpu.w32(0x1100d7a8u32, v); }
  pc = 0x11005f48; continue;
 }
 0x11005f48 => {
  { cpu.sub32(cpu.r32(0x1100d7a0u32), 0x0u32); }
  if !cpu.zf { pc = 0x11005f76; continue; }
  pc = 0x11005f51; continue;
 }
 0x11005f51 => {
  let t = cpu.r32(0x1100e198u32); cpu.push(0x11005f57); cpu.call(t);
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if !cpu.zf { pc = 0x11005f70; continue; }
  pc = 0x11005f5b; continue;
 }
 0x11005f5b => {
  { cpu.and32(cpu.eax, 0x80000000u32); }
  if cpu.zf { pc = 0x11005f70; continue; }
  pc = 0x11005f62; continue;
 }
 0x11005f62 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1100d7a0u32)); cpu.w32(0x1100d7a0u32, v); }
  pc = 0x11005f76; continue;
 }
 0x11005f6a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x11005f70 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1100d7a0u32)); cpu.w32(0x1100d7a0u32, v); }
  pc = 0x11005f76; continue;
 }
 0x11005f76 => {
  { let v: u32 = cpu.r32(0x1100e260u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, 0x1u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1100d920u32, v); }
  if !cpu.zf { pc = 0x11006026; continue; }
  pc = 0x11005f8c; continue;
 }
 0x11005f8c => {
  { cpu.sub32(cpu.r32(0x1100d7a0u32), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x11005fab; continue; }
  pc = 0x11005f95; continue;
 }
 0x11005f95 => {
  { cpu.sub32(cpu.r32(0x1100d7a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x11006093; continue; }
  pc = 0x11005fa2; continue;
 }
 0x11005fa2 => {
  { cpu.sub32(cpu.r32(0x1100d7a0u32), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x11005fc8; continue; }
  pc = 0x11005fab; continue;
 }
 0x11005fab => {
  { let v = 0x80u32; cpu.push(v); }
  let t = cpu.r32(0x1100e25cu32); cpu.push(0x11005fb6); cpu.call(t);
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1100d948u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11005ff0; continue; }
  pc = 0x11005fc2; continue;
 }
 0x11005fc2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x11005fc8 => {
  { cpu.sub32(cpu.r32(0x1100d7a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x11005ff0; continue; }
  pc = 0x11005fd1; continue;
 }
 0x11005fd1 => {
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x2000u32; cpu.push(v); }
  let t = cpu.r32(0x1100e194u32); cpu.push(0x11005fe1); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.w32(0x1100d948u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11005ff0; continue; }
  pc = 0x11005fea; continue;
 }
 0x11005fea => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x11005ff0 => {
  { let v: u32 = cpu.r32(0x1100d948u32); cpu.eax = v; }
  { let v = 0x1100d208u32; cpu.push(v); }
  { let v = 0x1100d000u32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1100d948u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1100d938u32, v); }
  cpu.push(0x11006014); f_1100617e(cpu);
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1100d7a8u32)); cpu.w32(0x1100d7a8u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x11006026 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x11006093; continue; }
  pc = 0x1100602a; continue;
 }
 0x1100602a => {
  { cpu.sub32(cpu.r32(0x1100d7a0u32), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x1100603c; continue; }
  pc = 0x11006033; continue;
 }
 0x11006033 => {
  { cpu.sub32(cpu.r32(0x1100d7a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x11006093; continue; }
  pc = 0x1100603c; continue;
 }
 0x1100603c => {
  { cpu.sub32(cpu.r32(0x1100d948u32), 0x0u32); }
  if cpu.zf { pc = 0x11006093; continue; }
  pc = 0x11006045; continue;
 }
 0x11006045 => {
  { let v: u32 = cpu.r32(0x1100d938u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1100d948u32), cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x11006069; continue; }
  pc = 0x11006056; continue;
 }
 0x11006056 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1100605e; continue; }
  pc = 0x1100605c; continue;
 }
 0x1100605c => {
  let t = cpu.eax; cpu.push(0x1100605e); cpu.call(t);
  pc = 0x1100605e; continue;
 }
 0x1100605e => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1100d948u32), cpu.esi); }
  if (cpu.cf || cpu.zf) { pc = 0x11006056; continue; }
  pc = 0x11006069; continue;
 }
 0x11006069 => {
  { cpu.sub32(cpu.r32(0x1100d7a0u32), 0x0u32); }
  { let v: u32 = cpu.r32(0x1100d948u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  if (cpu.sf == cpu.of) { pc = 0x11006083; continue; }
  pc = 0x11006078; continue;
 }
 0x11006078 => {
  let t = cpu.r32(0x1100e254u32); cpu.push(0x1100607e); cpu.call(t);
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x11006089; continue;
 }
 0x11006083 => {
  let t = cpu.r32(0x1100e1a0u32); cpu.push(0x11006089); cpu.call(t);
  pc = 0x11006089; continue;
 }
 0x11006089 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1100d948u32, v); }
  pc = 0x11006093; continue;
 }
 0x11006093 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_110060a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x110060a0); }
 let mut pc: u32 = 0x110060a0;
 loop { match pc {
 0x110060a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  { let v = cpu.ebp; cpu.push(v); }
  if !cpu.zf { pc = 0x110060c3; continue; }
  pc = 0x110060b1; continue;
 }
 0x110060b1 => {
  { cpu.sub32(cpu.r32(0x1100d7a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x110060c3; continue; }
  pc = 0x110060ba; continue;
 }
 0x110060ba => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x110060c3 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x110060cd; continue; }
  pc = 0x110060c8; continue;
 }
 0x110060c8 => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if !cpu.zf { pc = 0x11006107; continue; }
  pc = 0x110060cd; continue;
 }
 0x110060cd => {
  { let v: u32 = cpu.r32(0x1100d92cu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x110060e6; continue; }
  pc = 0x110060d7; continue;
 }
 0x110060d7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.ecx; cpu.push(0x110060e4); cpu.call(t);
  pc = 0x110060ee; continue;
 }
 0x110060e6 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x110060ee; continue;
 }
 0x110060ee => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x110060fe; continue; }
  pc = 0x110060f2; continue;
 }
 0x110060f2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x110060fa); f_11005f30(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1100610f; continue; }
  pc = 0x110060fe; continue;
 }
 0x110060fe => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x11006107 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  pc = 0x1100610f; continue;
 }
 0x1100610f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x11006117); f_11006190(cpu);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x1100612b; continue; }
  pc = 0x1100611e; continue;
 }
 0x1100611e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x1100612b; continue; }
  pc = 0x11006122; continue;
 }
 0x11006122 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1100612b); f_11005f30(cpu);
  pc = 0x1100612b; continue;
 }
 0x1100612b => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x11006134; continue; }
  pc = 0x1100612f; continue;
 }
 0x1100612f => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if !cpu.zf { pc = 0x11006156; continue; }
  pc = 0x11006134; continue;
 }
 0x11006134 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1100613c); f_11005f30(cpu);
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x11006142; continue; }
  pc = 0x11006140; continue;
 }
 0x11006140 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x11006142; continue;
 }
 0x11006142 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x11006156; continue; }
  pc = 0x11006146; continue;
 }
 0x11006146 => {
  { let v: u32 = cpu.r32(0x1100d92cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x11006156; continue; }
  pc = 0x1100614f; continue;
 }
 0x1100614f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.eax; cpu.push(0x11006154); cpu.call(t);
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x11006156; continue;
 }
 0x11006156 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11006160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11006160); }
 let mut pc: u32 = 0x11006160;
 loop { match pc {
 0x11006160 => {
  pc = cpu.r32(0x1100e240u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11006166(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11006166); }
 let mut pc: u32 = 0x11006166;
 loop { match pc {
 0x11006166 => {
  pc = cpu.r32(0x1100e244u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100616c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100616c); }
 let mut pc: u32 = 0x1100616c;
 loop { match pc {
 0x1100616c => {
  pc = cpu.r32(0x1100e248u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11006172(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11006172); }
 let mut pc: u32 = 0x11006172;
 loop { match pc {
 0x11006172 => {
  pc = cpu.r32(0x1100e24cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_1100617e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1100617e); }
 let mut pc: u32 = 0x1100617e;
 loop { match pc {
 0x1100617e => {
  pc = cpu.r32(0x1100e258u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_11006190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11006190); }
 let mut pc: u32 = 0x11006190;
 loop { match pc {
 0x11006190 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x110061ab; continue; }
  pc = 0x11006197; continue;
 }
 0x11006197 => {
  { cpu.sub32(cpu.r32(0x1100d92cu32), 0x0u32); }
  if !cpu.zf { pc = 0x110061ab; continue; }
  pc = 0x110061a0; continue;
 }
 0x110061a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1100e190u32); cpu.push(0x110061ab); cpu.call(t);
  pc = 0x110061ab; continue;
 }
 0x110061ab => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
