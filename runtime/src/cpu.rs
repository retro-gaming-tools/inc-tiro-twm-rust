//! The state generated code runs on: registers, flags, x87 and guest memory.
//! Nothing here interprets instructions: generated code calls these helpers
//! directly, and they compile down to the native operations.

use std::collections::HashMap;

pub type GuestFn = fn(&mut Cpu);

pub const IMPORT_BASE: u32 = 0xf000_0000;

pub struct Cpu {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub cf: bool,
    pub zf: bool,
    pub sf: bool,
    pub of: bool,
    pub df: bool,
    /// The low byte of the last result, for the parity flag.
    pub parity: u8,
    pub fs_base: u32,
    pub st: [f64; 8],
    pub top: usize,
    pub c0: bool,
    pub c1: bool,
    pub c2: bool,
    pub c3: bool,
    pub fpu_control: u16,
    pub memory: *mut u8,
    pub functions: HashMap<u32, GuestFn>,
    pub imports: Vec<Host>,
    pub host: Box<crate::host::State>,
    /// Prints every call to the runtime, when RECOMP_TRACE is set.
    pub trace: bool,
    /// Limits the trace to names containing this, when RECOMP_TRACE isn't just 1.
    pub trace_filter: Option<String>,
    /// Exports to log when called through a pointer, when RECOMP_WATCH names a DLL.
    pub watched: HashMap<u32, String>,
    pub watch_counts: HashMap<u32, u32>,
    /// Counts calls of each function between the script's `profile:` and `hits:` steps.
    pub profiling: bool,
    pub hits: HashMap<u32, u64>,
    /// The game's error logger, from a script's `errors:ADDRESS` first step.
    pub error_function: Option<u32>,
}

/// How many calls of each watched export to log.
const WATCH_LIMIT: u32 = 3;

/// One imported function. Functions the runtime lacks stop with their name.
#[derive(Clone)]
pub struct Host {
    pub name: String,
    pub function: Option<GuestFn>,
}

macro_rules! arithmetic {
    ($add:ident, $sub:ident, $adc:ident, $sbb:ident, $and:ident, $or:ident, $xor:ident, $inc:ident, $dec:ident, $neg:ident,
     $shl:ident, $shr:ident, $sar:ident, $rol:ident, $ror:ident, $rcl:ident, $rcr:ident, $type:ty, $signed:ty, $bits:expr) => {
        #[inline(always)]
        pub fn $add(&mut self, a: $type, b: $type) -> $type {
            let r = a.wrapping_add(b);
            self.cf = r < a;
            self.of = ((a ^ r) & (b ^ r)) >> ($bits - 1) != 0;
            self.result(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $sub(&mut self, a: $type, b: $type) -> $type {
            let r = a.wrapping_sub(b);
            self.cf = a < b;
            self.of = ((a ^ b) & (a ^ r)) >> ($bits - 1) != 0;
            self.result(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $adc(&mut self, a: $type, b: $type) -> $type {
            let carry = self.cf as $type;
            let r = a.wrapping_add(b).wrapping_add(carry);
            self.cf = (a as u64) + (b as u64) + (carry as u64) > <$type>::MAX as u64;
            self.of = ((a ^ r) & (b ^ r)) >> ($bits - 1) != 0;
            self.result(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $sbb(&mut self, a: $type, b: $type) -> $type {
            let carry = self.cf as $type;
            let r = a.wrapping_sub(b).wrapping_sub(carry);
            self.cf = (a as u64) < (b as u64) + (carry as u64);
            self.of = ((a ^ b) & (a ^ r)) >> ($bits - 1) != 0;
            self.result(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $and(&mut self, a: $type, b: $type) -> $type {
            let r = a & b;
            self.logic(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $or(&mut self, a: $type, b: $type) -> $type {
            let r = a | b;
            self.logic(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $xor(&mut self, a: $type, b: $type) -> $type {
            let r = a ^ b;
            self.logic(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $inc(&mut self, a: $type) -> $type {
            let carry = self.cf;
            let r = self.$add(a, 1);
            self.cf = carry;
            r
        }
        #[inline(always)]
        pub fn $dec(&mut self, a: $type) -> $type {
            let carry = self.cf;
            let r = self.$sub(a, 1);
            self.cf = carry;
            r
        }
        #[inline(always)]
        pub fn $neg(&mut self, a: $type) -> $type {
            let r = self.$sub(0, a);
            self.cf = a != 0;
            r
        }
        #[inline(always)]
        pub fn $shl(&mut self, a: $type, count: u32) -> $type {
            let count = count & 31;
            if count == 0 {
                return a;
            }
            let wide = (a as u64) << count;
            let r = wide as $type;
            self.cf = (wide >> $bits) & 1 != 0;
            self.of = (r >> ($bits - 1) != 0) != self.cf;
            self.result(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $shr(&mut self, a: $type, count: u32) -> $type {
            let count = count & 31;
            if count == 0 {
                return a;
            }
            let r = ((a as u64) >> count) as $type;
            self.cf = ((a as u64) >> (count - 1)) & 1 != 0;
            self.of = a >> ($bits - 1) != 0;
            self.result(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $sar(&mut self, a: $type, count: u32) -> $type {
            let count = count & 31;
            if count == 0 {
                return a;
            }
            let wide = (a as $signed) as i64;
            let r = (wide >> count) as $type;
            self.cf = (wide >> (count - 1)) & 1 != 0;
            self.of = false;
            self.result(r as u32, r >> ($bits - 1) != 0, r == 0);
            r
        }
        #[inline(always)]
        pub fn $rol(&mut self, a: $type, count: u32) -> $type {
            let turn = count & 31;
            if turn == 0 {
                return a;
            }
            let r = a.rotate_left(turn % $bits);
            self.cf = r & 1 != 0;
            self.of = (r >> ($bits - 1) != 0) != self.cf;
            r
        }
        #[inline(always)]
        pub fn $ror(&mut self, a: $type, count: u32) -> $type {
            let turn = count & 31;
            if turn == 0 {
                return a;
            }
            let r = a.rotate_right(turn % $bits);
            self.cf = r >> ($bits - 1) != 0;
            self.of = (r >> ($bits - 1) != 0) != ((r >> ($bits - 2)) & 1 != 0);
            r
        }
        pub fn $rcl(&mut self, mut a: $type, count: u32) -> $type {
            for _ in 0..(count & 31) % ($bits + 1) {
                let top = a >> ($bits - 1) != 0;
                a = (a << 1) | self.cf as $type;
                self.cf = top;
            }
            a
        }
        pub fn $rcr(&mut self, mut a: $type, count: u32) -> $type {
            for _ in 0..(count & 31) % ($bits + 1) {
                let low = a & 1 != 0;
                a = (a >> 1) | ((self.cf as $type) << ($bits - 1));
                self.cf = low;
            }
            a
        }
    };
}

impl Cpu {
    arithmetic!(add32, sub32, adc32, sbb32, and32, or32, xor32, inc32, dec32, neg32, shl32, shr32, sar32, rol32, ror32, rcl32, rcr32, u32, i32, 32);
    arithmetic!(add16, sub16, adc16, sbb16, and16, or16, xor16, inc16, dec16, neg16, shl16, shr16, sar16, rol16, ror16, rcl16, rcr16, u16, i16, 16);
    arithmetic!(add8, sub8, adc8, sbb8, and8, or8, xor8, inc8, dec8, neg8, shl8, shr8, sar8, rol8, ror8, rcl8, rcr8, u8, i8, 8);

    #[inline(always)]
    fn result(&mut self, value: u32, sign: bool, zero: bool) {
        self.parity = value as u8;
        self.sf = sign;
        self.zf = zero;
    }

    #[inline(always)]
    fn logic(&mut self, value: u32, sign: bool, zero: bool) {
        self.cf = false;
        self.of = false;
        self.result(value, sign, zero);
    }

    #[inline(always)]
    pub fn pf(&self) -> bool {
        self.parity.count_ones() % 2 == 0
    }

    #[inline(always)]
    pub fn r8(&self, at: u32) -> u8 {
        unsafe { self.memory.add(at as usize).read() }
    }
    #[inline(always)]
    pub fn r16(&self, at: u32) -> u16 {
        unsafe { (self.memory.add(at as usize) as *const u16).read_unaligned() }
    }
    #[inline(always)]
    pub fn r32(&self, at: u32) -> u32 {
        unsafe { (self.memory.add(at as usize) as *const u32).read_unaligned() }
    }
    #[inline(always)]
    pub fn r64(&self, at: u32) -> u64 {
        unsafe { (self.memory.add(at as usize) as *const u64).read_unaligned() }
    }
    #[inline(always)]
    pub fn w8(&mut self, at: u32, value: u8) {
        unsafe { self.memory.add(at as usize).write(value) }
    }
    #[inline(always)]
    pub fn w16(&mut self, at: u32, value: u16) {
        unsafe { (self.memory.add(at as usize) as *mut u16).write_unaligned(value) }
    }
    #[inline(always)]
    pub fn w32(&mut self, at: u32, value: u32) {
        unsafe { (self.memory.add(at as usize) as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    pub fn w64(&mut self, at: u32, value: u64) {
        unsafe { (self.memory.add(at as usize) as *mut u64).write_unaligned(value) }
    }

    #[inline(always)]
    pub fn push(&mut self, value: u32) {
        self.esp = self.esp.wrapping_sub(4);
        self.w32(self.esp, value);
    }
    #[inline(always)]
    pub fn pop(&mut self) -> u32 {
        let value = self.r32(self.esp);
        self.esp = self.esp.wrapping_add(4);
        value
    }

    pub fn hit(&mut self, address: u32) {
        *self.hits.entry(address).or_default() += 1;
        if Some(address) == self.error_function {
            let strings: Vec<String> = (1..=5).map(|i| self.r32(self.esp + i * 4)).map(|value| {
                if (0x40_0000..0x7000_0000).contains(&value) { self.string(value) } else { format!("{value:#x}") }
            }).collect();
            println!("game error from {:#x}: {}", self.r32(self.esp), strings.join(" | "));
        }
    }

    /// Calls any guest address. The caller has pushed the return address.
    pub fn call(&mut self, address: u32) {
        if address >= IMPORT_BASE {
            let host = &self.imports[(address - IMPORT_BASE) as usize];
            if self.trace && self.trace_filter.as_ref().is_none_or(|filter| host.name.contains(filter.as_str())) {
                let arguments = [self.r32(self.esp + 4), self.r32(self.esp + 8), self.r32(self.esp + 12)];
                println!("{} {arguments:x?} from {:#x}", host.name, self.r32(self.esp));
            }
            match host.function {
                Some(function) => function(self),
                None => panic!("the runtime doesn't have {} yet", host.name),
            }
            return;
        }
        let Some(&function) = self.functions.get(&address) else {
            panic!("no recompiled code at {address:#010x}: discovery missed it");
        };
        if let Some(name) = self.watched.get(&address).cloned() {
            let seen = self.watch_counts.entry(address).or_default();
            *seen += 1;
            if *seen <= WATCH_LIMIT {
                let esp = self.esp;
                let raw: Vec<String> = (1..=7).map(|i| format!("{:x}", self.r32(esp + i * 4))).collect();
                let floats: Vec<String> = (1..=7).map(|i| format!("{:.3}", f32::from_bits(self.r32(esp + i * 4)))).collect();
                println!("{name}({}) floats ({})", raw.join(", "), floats.join(", "));
                let pointers: Vec<u32> = (1..=5).map(|i| self.r32(esp + i * 4)).filter(|&p| (0x40_0000..0x7000_0000).contains(&p)).collect();
                function(self);
                println!("  = eax {:x} st0 {:.4} top {}", self.eax, self.st(0), self.top);
                for pointer in pointers {
                    let words: Vec<String> = (0..9).map(|i| format!("{:x}", self.r32(pointer + i * 4))).collect();
                    println!("    [{pointer:x}]: {}", words.join(" "));
                }
                return;
            }
        }
        function(self);
    }

    pub fn unimplemented(&mut self, address: u32, name: &str) {
        panic!("{name} at {address:#010x} isn't translated yet");
    }

    pub fn mul32(&mut self, value: u32) {
        let wide = self.eax as u64 * value as u64;
        self.eax = wide as u32;
        self.edx = (wide >> 32) as u32;
        self.cf = self.edx != 0;
        self.of = self.cf;
    }
    pub fn mul16(&mut self, value: u16) {
        let wide = (self.eax as u16 as u32) * value as u32;
        self.set_ax(wide as u16);
        self.edx = (self.edx & 0xffff_0000) | (wide >> 16);
        self.cf = wide >> 16 != 0;
        self.of = self.cf;
    }
    pub fn mul8(&mut self, value: u8) {
        let wide = (self.eax as u8 as u16) * value as u16;
        self.set_ax(wide);
        self.cf = wide >> 8 != 0;
        self.of = self.cf;
    }
    pub fn imul32(&mut self, value: u32) {
        let wide = self.eax as i32 as i64 * value as i32 as i64;
        self.eax = wide as u32;
        self.edx = (wide >> 32) as u32;
        self.cf = wide != wide as i32 as i64;
        self.of = self.cf;
    }
    pub fn imul16(&mut self, value: u16) {
        let wide = self.eax as i16 as i32 * value as i16 as i32;
        self.set_ax(wide as u16);
        self.edx = (self.edx & 0xffff_0000) | ((wide >> 16) as u16 as u32);
        self.cf = wide != wide as i16 as i32;
        self.of = self.cf;
    }
    pub fn imul8(&mut self, value: u8) {
        let wide = self.eax as i8 as i16 * value as i8 as i16;
        self.set_ax(wide as u16);
        self.cf = wide != wide as i8 as i16;
        self.of = self.cf;
    }
    #[inline(always)]
    pub fn imul2_32(&mut self, a: u32, b: u32) -> u32 {
        let wide = a as i32 as i64 * b as i32 as i64;
        self.cf = wide != wide as i32 as i64;
        self.of = self.cf;
        wide as u32
    }
    pub fn imul2_16(&mut self, a: u16, b: u16) -> u16 {
        let wide = a as i16 as i32 * b as i16 as i32;
        self.cf = wide != wide as i16 as i32;
        self.of = self.cf;
        wide as u16
    }
    pub fn div32(&mut self, value: u32) {
        let wide = ((self.edx as u64) << 32) | self.eax as u64;
        self.eax = (wide / value as u64) as u32;
        self.edx = (wide % value as u64) as u32;
    }
    pub fn div16(&mut self, value: u16) {
        let wide = ((self.edx & 0xffff) << 16) | (self.eax & 0xffff);
        self.set_ax((wide / value as u32) as u16);
        self.edx = (self.edx & 0xffff_0000) | (wide % value as u32);
    }
    pub fn div8(&mut self, value: u8) {
        let wide = self.eax as u16;
        let quotient = wide / value as u16;
        let remainder = wide % value as u16;
        self.set_ax(quotient | (remainder << 8));
    }
    pub fn idiv32(&mut self, value: u32) {
        let wide = (((self.edx as u64) << 32) | self.eax as u64) as i64;
        let divisor = value as i32 as i64;
        self.eax = (wide / divisor) as u32;
        self.edx = (wide % divisor) as u32;
    }
    pub fn idiv16(&mut self, value: u16) {
        let wide = (((self.edx & 0xffff) << 16) | (self.eax & 0xffff)) as i32;
        let divisor = value as i16 as i32;
        self.set_ax((wide / divisor) as u16);
        self.edx = (self.edx & 0xffff_0000) | ((wide % divisor) as u16 as u32);
    }
    pub fn idiv8(&mut self, value: u8) {
        let wide = self.eax as u16 as i16;
        let divisor = value as i8 as i16;
        let quotient = (wide / divisor) as u8 as u16;
        let remainder = (wide % divisor) as u8 as u16;
        self.set_ax(quotient | (remainder << 8));
    }
    #[inline(always)]
    pub fn set_ax(&mut self, value: u16) {
        self.eax = (self.eax & 0xffff_0000) | value as u32;
    }

    pub fn shld32(&mut self, a: u32, b: u32, count: u32) -> u32 {
        let count = count & 31;
        if count == 0 {
            return a;
        }
        let r = (a << count) | (b >> (32 - count));
        self.cf = (a >> (32 - count)) & 1 != 0;
        self.result(r, r >> 31 != 0, r == 0);
        r
    }
    pub fn shrd32(&mut self, a: u32, b: u32, count: u32) -> u32 {
        let count = count & 31;
        if count == 0 {
            return a;
        }
        let r = (a >> count) | (b << (32 - count));
        self.cf = (a >> (count - 1)) & 1 != 0;
        self.result(r, r >> 31 != 0, r == 0);
        r
    }

    pub fn flags(&self) -> u32 {
        (self.cf as u32) | 2 | ((self.pf() as u32) << 2) | ((self.zf as u32) << 6) | ((self.sf as u32) << 7)
            | ((self.df as u32) << 10) | ((self.of as u32) << 11) | 0x200
    }
    pub fn set_flags(&mut self, value: u32) {
        self.cf = value & 1 != 0;
        self.parity = if value & 4 != 0 { 0 } else { 1 };
        self.zf = value & 0x40 != 0;
        self.sf = value & 0x80 != 0;
        self.df = value & 0x400 != 0;
        self.of = value & 0x800 != 0;
    }
    pub fn pushad(&mut self) {
        let esp = self.esp;
        for value in [self.eax, self.ecx, self.edx, self.ebx, esp, self.ebp, self.esi, self.edi] {
            self.push(value);
        }
    }
    pub fn popad(&mut self) {
        self.edi = self.pop();
        self.esi = self.pop();
        self.ebp = self.pop();
        self.pop();
        self.ebx = self.pop();
        self.edx = self.pop();
        self.ecx = self.pop();
        self.eax = self.pop();
    }

    fn step(&self, size: u32) -> u32 {
        if self.df { size.wrapping_neg() } else { size }
    }
    fn read_sized(&self, at: u32, size: u32) -> u32 {
        match size {
            1 => self.r8(at) as u32,
            2 => self.r16(at) as u32,
            _ => self.r32(at),
        }
    }
    fn write_sized(&mut self, at: u32, size: u32, value: u32) {
        match size {
            1 => self.w8(at, value as u8),
            2 => self.w16(at, value as u16),
            _ => self.w32(at, value),
        }
    }
    fn compare_sized(&mut self, a: u32, b: u32, size: u32) {
        match size {
            1 => { self.sub8(a as u8, b as u8); }
            2 => { self.sub16(a as u16, b as u16); }
            _ => { self.sub32(a, b); }
        }
    }
    pub fn movs(&mut self, size: u32, rep: bool) {
        let step = self.step(size);
        while !rep || self.ecx != 0 {
            let value = self.read_sized(self.esi, size);
            self.write_sized(self.edi, size, value);
            self.esi = self.esi.wrapping_add(step);
            self.edi = self.edi.wrapping_add(step);
            if !rep {
                return;
            }
            self.ecx -= 1;
        }
    }
    pub fn stos(&mut self, size: u32, rep: bool) {
        let step = self.step(size);
        while !rep || self.ecx != 0 {
            self.write_sized(self.edi, size, self.eax);
            self.edi = self.edi.wrapping_add(step);
            if !rep {
                return;
            }
            self.ecx -= 1;
        }
    }
    pub fn lods(&mut self, size: u32, rep: bool) {
        let step = self.step(size);
        while !rep || self.ecx != 0 {
            let value = self.read_sized(self.esi, size);
            match size {
                1 => self.eax = (self.eax & 0xffff_ff00) | value,
                2 => self.eax = (self.eax & 0xffff_0000) | value,
                _ => self.eax = value,
            }
            self.esi = self.esi.wrapping_add(step);
            if !rep {
                return;
            }
            self.ecx -= 1;
        }
    }
    /// `repeat` is 0 for none, 1 for repe, 2 for repne.
    pub fn scas(&mut self, size: u32, repeat: u32) {
        let step = self.step(size);
        while repeat == 0 || self.ecx != 0 {
            let value = self.read_sized(self.edi, size);
            self.compare_sized(self.eax, value, size);
            self.edi = self.edi.wrapping_add(step);
            if repeat == 0 {
                return;
            }
            self.ecx -= 1;
            if (repeat == 1) != self.zf {
                return;
            }
        }
    }
    pub fn cmps(&mut self, size: u32, repeat: u32) {
        let step = self.step(size);
        while repeat == 0 || self.ecx != 0 {
            let a = self.read_sized(self.esi, size);
            let b = self.read_sized(self.edi, size);
            self.compare_sized(a, b, size);
            self.esi = self.esi.wrapping_add(step);
            self.edi = self.edi.wrapping_add(step);
            if repeat == 0 {
                return;
            }
            self.ecx -= 1;
            if (repeat == 1) != self.zf {
                return;
            }
        }
    }

    // x87. Values are f64, not the 80-bit format, and this is checked where it matters.
    #[inline(always)]
    pub fn st(&self, index: usize) -> f64 {
        self.st[(self.top + index) & 7]
    }
    #[inline(always)]
    pub fn set_st(&mut self, index: usize, value: f64) {
        self.st[(self.top + index) & 7] = value;
    }
    #[inline(always)]
    pub fn fpush(&mut self, value: f64) {
        self.top = (self.top + 7) & 7;
        self.st[self.top] = value;
    }
    #[inline(always)]
    pub fn fpop(&mut self) -> f64 {
        let value = self.st[self.top];
        self.top = (self.top + 1) & 7;
        value
    }
    pub fn rf32(&self, at: u32) -> f64 {
        f32::from_bits(self.r32(at)) as f64
    }
    pub fn rf64(&self, at: u32) -> f64 {
        f64::from_bits(self.r64(at))
    }
    pub fn rf80(&self, at: u32) -> f64 {
        let mantissa = self.r64(at);
        let word = self.r16(at + 8);
        let negative = word & 0x8000 != 0;
        let exponent = (word & 0x7fff) as i32;
        let value = if exponent == 0 && mantissa == 0 {
            0.0
        } else if exponent == 0x7fff {
            if mantissa << 1 == 0 { f64::INFINITY } else { f64::NAN }
        } else {
            mantissa as f64 * 2f64.powi(exponent - 16383 - 63)
        };
        if negative { -value } else { value }
    }
    pub fn wf32(&mut self, at: u32, value: f64) {
        self.w32(at, (value as f32).to_bits());
    }
    pub fn wf64(&mut self, at: u32, value: f64) {
        self.w64(at, value.to_bits());
    }
    pub fn wf80(&mut self, at: u32, value: f64) {
        let bits = value.to_bits();
        let negative = (bits >> 63) as u16;
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        let fraction = bits & ((1 << 52) - 1);
        let (word, mantissa) = if exponent == 0 && fraction == 0 {
            (0, 0)
        } else if exponent == 0x7ff {
            (0x7fff, (1u64 << 63) | (fraction << 11))
        } else if exponent == 0 {
            let shift = fraction.leading_zeros();
            ((-1022 - 52 + 63 - shift as i32 + 16383) as u16, fraction << shift)
        } else {
            ((exponent - 1023 + 16383) as u16, (1u64 << 63) | (fraction << 11))
        };
        self.w64(at, mantissa);
        self.w16(at + 8, word | (negative << 15));
    }
    /// Rounds as the control word says, which integer stores depend on.
    pub fn round(&self, value: f64) -> f64 {
        match (self.fpu_control >> 10) & 3 {
            0 => {
                let rounded = value.round();
                if (value - value.trunc()).abs() == 0.5 && rounded % 2.0 != 0.0 { rounded - value.signum() } else { rounded }
            }
            1 => value.floor(),
            2 => value.ceil(),
            _ => value.trunc(),
        }
    }
    pub fn fi16(&self, value: f64) -> u16 {
        let rounded = self.round(value);
        if rounded >= -32768.0 && rounded <= 32767.0 { rounded as i16 as u16 } else { 0x8000 }
    }
    pub fn fi32(&self, value: f64) -> u32 {
        let rounded = self.round(value);
        if rounded >= -2147483648.0 && rounded <= 2147483647.0 { rounded as i32 as u32 } else { 0x8000_0000 }
    }
    pub fn fi64(&self, value: f64) -> u64 {
        let rounded = self.round(value);
        if rounded >= -9.3e18 && rounded <= 9.2e18 { rounded as i64 as u64 } else { 0x8000_0000_0000_0000 }
    }
    pub fn fcom(&mut self, a: f64, b: f64) {
        self.c1 = false;
        if a.is_nan() || b.is_nan() {
            (self.c3, self.c2, self.c0) = (true, true, true);
        } else {
            (self.c3, self.c2, self.c0) = (a == b, false, a < b);
        }
    }
    pub fn fcomi(&mut self, a: f64, b: f64) {
        let unordered = a.is_nan() || b.is_nan();
        self.zf = unordered || a == b;
        self.parity = if unordered { 0 } else { 1 };
        self.cf = unordered || a < b;
        self.of = false;
        self.sf = false;
    }
    pub fn fxam(&mut self) {
        let value = self.st(0);
        self.c1 = value.is_sign_negative();
        let (c3, c2, c0) = if value.is_nan() {
            (false, false, true)
        } else if value.is_infinite() {
            (false, true, true)
        } else if value == 0.0 {
            (true, false, false)
        } else if value.is_subnormal() {
            (true, true, false)
        } else {
            (false, true, false)
        };
        (self.c3, self.c2, self.c0) = (c3, c2, c0);
    }
    pub fn fstatus(&self) -> u16 {
        ((self.c0 as u16) << 8) | ((self.c1 as u16) << 9) | ((self.c2 as u16) << 10) | ((self.top as u16) << 11)
            | ((self.c3 as u16) << 14)
    }
    pub fn fprem(&mut self) {
        let (a, b) = (self.st(0), self.st(1));
        self.set_st(0, a % b);
        self.c2 = false;
    }
    pub fn fscale(&mut self) {
        let value = self.st(0) * 2f64.powf(self.st(1).trunc());
        self.set_st(0, value);
    }
    pub fn cpuid(&mut self) {
        // A plain Pentium, so nothing chooses MMX or SSE paths.
        (self.eax, self.ebx, self.edx, self.ecx) = if self.eax == 0 {
            (1, 0x756e_6547, 0x4965_6e69, 0x6c65_746e)
        } else {
            (0x543, 0, 0x1bf, 0)
        };
    }
}
