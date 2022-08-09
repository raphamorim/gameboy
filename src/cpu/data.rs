use crate::cpu::cpu::Cpu;
use crate::cpu::registers::CpuFlag::{C, H, N, Z};

pub fn addr_b(c: &mut Cpu) {
    c.registers.a += c.registers.b;
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn addr_c(c: &mut Cpu) {
    c.registers.a += c.registers.c;
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn addr_d(c: &mut Cpu) {
    c.registers.a += c.registers.d;
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn addr_e(c: &mut Cpu) {
    c.registers.a += c.registers.e;
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn addr_h(c: &mut Cpu) {
    c.registers.a += c.registers.h;
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn addr_l(c: &mut Cpu) {
    c.registers.a += c.registers.l;
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn addr_a(c: &mut Cpu) {
    c.registers.a += c.registers.a;
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn addhl(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.registers.a += c.memory.rb(addr);
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn addn(c: &mut Cpu) {
    let b = c.get_byte();
    let a = c.registers.a;
    let r = a.wrapping_add(b);
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, (a & 0xF) + (b & 0xF) > 0xF);
    c.registers.flag(N, false);
    c.registers.flag(C, (a as u16) + (b as u16) > 0xFF);
    c.registers.a = r;
}
pub fn addhlbc(c: &mut Cpu) {
    let b = ((c.registers.b as u16) << 8) | (c.registers.c as u16);
    let a = ((c.registers.h as u16) << 8) | (c.registers.l as u16);
    let r = a.wrapping_add(b);
    c.registers.flag(H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
    c.registers.flag(N, false);
    c.registers.flag(C, a > 0xFFFF - b);
    c.registers.h = (r >> 8) as u8;
    c.registers.l = (r & 0x00FF) as u8;

    // let mut hl = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    // hl += ((c.registers.b as u16) << 8) + c.registers.c as u16;
    // if hl > 65535 {
    //     c.registers.f |= 0x10;
    // } else {
    //     c.registers.f &= 0xEF;
    // }
    // c.registers.h = ((hl >> 8) & 255) as u8;
    // c.registers.l = (hl & 255) as u8;
}
pub fn addhlde(c: &mut Cpu) {
    // let mut hl: u16 = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    // hl += ((c.registers.d as u16) << 8) + c.registers.e as u16;

    let de = ((c.registers.d as u16) << 8) | (c.registers.e as u16);
    let a = ((c.registers.h as u16) << 8) | (c.registers.l as u16);
    let r = a.wrapping_add(de);
    c.registers.flag(H, (a & 0x07FF) + (de & 0x07FF) > 0x07FF);
    c.registers.flag(N, false);
    c.registers.flag(C, a > 0xFFFF - de);
    c.registers.h = (r >> 8) as u8;
    c.registers.l = (r & 0x00FF) as u8;
}
pub fn addhlhl(c: &mut Cpu) {
    let mut hl: u16 = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    hl += ((c.registers.h as u16) << 8) + c.registers.l as u16;

    if hl > 65535 {
        c.registers.f |= 0x10;
    } else {
        c.registers.f &= 0xEF;
        c.registers.h = ((hl >> 8) & 255) as u8;
        c.registers.l = (hl & 255) as u8;
    }
}
pub fn addhlsp(c: &mut Cpu) {
    let mut hl = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    hl += c.registers.sp;
    if hl > 65535 {
        c.registers.f |= 0x10;
    } else {
        c.registers.f &= 0xEF;
    }
    c.registers.h = ((hl >> 8) & 255) as u8;
    c.registers.l = (hl & 255) as u8;
}
// pub fn addspn(c: &mut Cpu) {
//     let i = c.memory.rb(c.registers.pc);
//     if i > 127 {
//         // i=-{ ((~i+1)&255)

//     }
//     c.registers.pc += 1;
//     c.registers.sp += i as u16;
//
// }
// https://github.com/alexcrichton/jba/blob/rust/src/cpu/z80/imp.rs#L81
pub fn addspn(c: &mut Cpu) {
    let b = c.memory.rb(c.registers.pc) as i8 as i16 as u16;
    let res = c.registers.sp + b;
    let tmp = b ^ res ^ c.registers.sp;
    c.registers.f =
        if tmp & 0x100 != 0 { 0x10 } else { 0 } | if tmp & 0x010 != 0 { 0x20 } else { 0 };
    c.registers.sp = res;
}
pub fn adcr_b(c: &mut Cpu) {
    c.registers.a += c.registers.b;
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn adcr_c(c: &mut Cpu) {
    c.registers.a += c.registers.c;
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn adcr_d(c: &mut Cpu) {
    c.registers.a += c.registers.d;
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn adcr_e(c: &mut Cpu) {
    c.registers.a += c.registers.e;
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn adcr_h(c: &mut Cpu) {
    c.registers.a += c.registers.h;
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn adcr_l(c: &mut Cpu) {
    c.registers.a += c.registers.l;
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn adcr_a(c: &mut Cpu) {
    c.registers.a += c.registers.a;
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn adchl(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.registers.a += c.memory.rb(addr);
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn adcn(c: &mut Cpu) {
    c.registers.a += c.memory.rb(c.registers.pc);
    c.registers.pc += 1;
    if c.registers.f >= 0x10 {
        c.registers.a += 1
    } else {
        c.registers.a += 0
    }
    c.fz(c.registers.a, 0);
    if c.registers.a > u8::MAX {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn subr_b(c: &mut Cpu) {
    c.registers.a -= c.registers.b;
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn subr_c(c: &mut Cpu) {
    let b = c.registers.c;
    let a = c.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    c.registers.flag(N, true);
    c.registers.flag(C, (a as u16) < (b as u16));
    c.registers.a = r;
}
pub fn subr_d(c: &mut Cpu) {
    c.registers.a -= c.registers.d;
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn subr_e(c: &mut Cpu) {
    c.registers.a -= c.registers.e;
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn subr_h(c: &mut Cpu) {
    c.registers.a -= c.registers.h;
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn subr_l(c: &mut Cpu) {
    c.registers.a -= c.registers.l;
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn subr_a(c: &mut Cpu) {
    // let c = if usec && self.reg.getflag(C) { 1 } else { 0 };
    let b = c.registers.a;
    let a = c.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    c.registers.flag(N, true);
    c.registers.flag(C, (a as u16) < (b as u16));
    c.registers.a = r;
}
pub fn subhl(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.registers.a -= c.memory.rb(addr);
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn subn(c: &mut Cpu) {
    let b = c.get_byte();
    let a = c.registers.a;
    let r = a.wrapping_sub(b);
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    c.registers.flag(N, true);
    c.registers.flag(C, (a as u16) < (b as u16));
    c.registers.a = r;
}
pub fn sbcr_b(c: &mut Cpu) {
    c.registers.a -= c.registers.b;
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn sbcr_c(c: &mut Cpu) {
    c.registers.a -= c.registers.c;
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn sbcr_d(c: &mut Cpu) {
    c.registers.a -= c.registers.d;
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn sbcr_e(c: &mut Cpu) {
    c.registers.a -= c.registers.e;
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn sbcr_h(c: &mut Cpu) {
    c.registers.a -= c.registers.h;
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn sbcr_l(c: &mut Cpu) {
    c.registers.a -= c.registers.l;
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn sbcr_a(c: &mut Cpu) {
    c.registers.a -= c.registers.a;
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn sbchl(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.registers.a -= c.memory.rb(addr);
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}
pub fn sbcn(c: &mut Cpu) {
    c.registers.a -= c.memory.rb(c.registers.pc);
    c.registers.pc += 1;
    if c.registers.f >= 0x10 {
        c.registers.a -= 1
    } else {
        c.registers.a -= 0;
    }
    c.fz(c.registers.a, 1);
    if c.registers.a < 0 {
        c.registers.f |= 0x10;
    }
    c.registers.a &= 255;
}

pub fn cpr_b(c: &mut Cpu) {
    let mut i = c.registers.a;
    i -= c.registers.b;
    c.fz(i, 1);
    if i < 0 {
        c.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_c(c: &mut Cpu) {
    let mut i = c.registers.a;
    i -= c.registers.c;
    c.fz(i, 1);
    if i < 0 {
        c.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_d(c: &mut Cpu) {
    let mut i = c.registers.a;
    i -= c.registers.d;
    c.fz(i, 1);
    if i < 0 {
        c.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_e(c: &mut Cpu) {
    let mut i = c.registers.a;
    i -= c.registers.e;
    c.fz(i, 1);
    if i < 0 {
        c.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_h(c: &mut Cpu) {
    let mut i = c.registers.a;
    i -= c.registers.h;
    c.fz(i, 1);
    if i < 0 {
        c.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_l(c: &mut Cpu) {
    let mut i = c.registers.a;
    i -= c.registers.l;
    c.fz(i, 1);
    if i < 0 {
        c.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_a(c: &mut Cpu) {
    let mut i = c.registers.a;
    i -= c.registers.a;
    c.fz(i, 1);
    if i < 0 {
        c.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cphl(c: &mut Cpu) {
    let mut i = c.registers.a;
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    i -= c.memory.rb(addr);
    c.fz(i, 1);
    if i < 0 {
        c.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpn(c: &mut Cpu) {
    // let v = self.fetchbyte(); self.alu_cp(v)

    let b = c.get_byte();
    let a = c.registers.a;

    // self.alu_sub(b, false);
    // self.reg.a = r;

    let r = c.registers.a.wrapping_sub(b);
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    c.registers.flag(N, true);
    c.registers.flag(C, (a as u16) < (b as u16));

    // c.registers.a = r;
    c.registers.a = a;

    // let mut i = c.registers.a as u16;
    // i = c.memory.rw(c.registers.pc) - i;
    // c.registers.pc += 1;
    // c.fz(i as u8, 1);
    // if i < 255 {
    //     c.registers.f |= 0x10;
    // }
    // i &= 255;
}
pub fn andr_b(c: &mut Cpu) {
    c.registers.a &= c.registers.b;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn andr_c(c: &mut Cpu) {
    c.registers.a &= c.registers.c;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn andr_d(c: &mut Cpu) {
    c.registers.a &= c.registers.d;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn andr_e(c: &mut Cpu) {
    c.registers.a &= c.registers.e;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn andr_h(c: &mut Cpu) {
    c.registers.a &= c.registers.h;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn andr_l(c: &mut Cpu) {
    c.registers.a &= c.registers.l;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn andr_a(c: &mut Cpu) {
    c.registers.a &= c.registers.a;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn andhl(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.registers.a &= c.memory.rb(addr);
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn andn(c: &mut Cpu) {
    let v = c.get_byte();
    let r = c.registers.a & v;
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, true);
    c.registers.flag(C, false);
    c.registers.flag(N, false);
    c.registers.a = r;

    // c.registers.a &= c.memory.rb(c.registers.pc);
    // c.registers.pc += 1;
    // c.registers.a &= 255;
    // c.fz(c.registers.a, 0);
}
pub fn orr_b(c: &mut Cpu) {
    c.registers.a |= c.registers.b;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn orr_c(c: &mut Cpu) {
    c.registers.a |= c.registers.c;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn orr_d(c: &mut Cpu) {
    c.registers.a |= c.registers.d;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn orr_e(c: &mut Cpu) {
    c.registers.a |= c.registers.e;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn orr_h(c: &mut Cpu) {
    c.registers.a |= c.registers.h;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn orr_l(c: &mut Cpu) {
    c.registers.a |= c.registers.l;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn orr_a(c: &mut Cpu) {
    c.registers.a |= c.registers.a;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn orhl(c: &mut Cpu) {
    let a = (c.registers.h as u16) << 8;
    let addr: u16 = a + (c.registers.l as u16);
    c.registers.a |= c.memory.rb(addr);
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn orn(c: &mut Cpu) {
    c.registers.a |= c.memory.rb(c.registers.pc);
    c.registers.pc += 1;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorr_b(c: &mut Cpu) {
    c.registers.a ^= c.registers.b;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorr_c(c: &mut Cpu) {
    c.registers.a ^= c.registers.c;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorr_d(c: &mut Cpu) {
    c.registers.a ^= c.registers.d;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorr_e(c: &mut Cpu) {
    c.registers.a ^= c.registers.e;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorr_h(c: &mut Cpu) {
    c.registers.a ^= c.registers.h;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorr_l(c: &mut Cpu) {
    c.registers.a ^= c.registers.l;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorr_a(c: &mut Cpu) {
    c.registers.a ^= c.registers.a;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorhl(c: &mut Cpu) {
    let addr = (c.registers.h as u16) << 8 + (c.registers.l as u16);
    c.registers.a ^= c.memory.rb(addr);
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn xorn(c: &mut Cpu) {
    c.registers.a ^= c.memory.rb(c.registers.pc);
    c.registers.pc += 1;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn incr_b(c: &mut Cpu) {
    c.registers.b += 1;
    c.registers.b &= 255;
    c.fz(c.registers.b, 0);
}
pub fn incr_c(c: &mut Cpu) {
    c.registers.c += 1;
    c.registers.c &= 255;
    c.fz(c.registers.c, 0);
}
pub fn incr_d(c: &mut Cpu) {
    c.registers.d += 1;
    c.registers.d &= 255;
    c.fz(c.registers.d, 0);
}
pub fn incr_e(c: &mut Cpu) {
    c.registers.e += 1;
    c.registers.e &= 255;
    c.fz(c.registers.e, 0);
}
pub fn incr_h(c: &mut Cpu) {
    c.registers.h += 1;
    c.registers.h &= 255;
    c.fz(c.registers.h, 0);
}
pub fn incr_l(c: &mut Cpu) {
    c.registers.l += 1;
    c.registers.l &= 255;
    c.fz(c.registers.l, 0);
}
pub fn incr_a(c: &mut Cpu) {
    c.registers.a += 1;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn inchlm(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + (c.registers.l as u16);
    let mut i = c.memory.rb(addr) + 1;
    i &= 255;
    let waddr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.memory.wb(waddr, i);
    c.fz(i, 0);
}
pub fn decr_b(c: &mut Cpu) {
    let a = c.registers.b;
    let r = a.wrapping_sub(1);
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, (a & 0x0F) == 0);
    c.registers.flag(N, true);
    c.registers.b = r;
}
pub fn decr_c(c: &mut Cpu) {
    c.registers.c -= 1;
    c.registers.c &= 255;
    c.fz(c.registers.c, 0);
}
pub fn decr_d(c: &mut Cpu) {
    let d = c.registers.d;
    let r = d.wrapping_sub(1);
    // c.registers.d &= 255;
    // c.fz(c.registers.d, 0);
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, (d & 0x0F) == 0);
    c.registers.flag(N, true);
    c.registers.d = r;
}
pub fn decr_e(c: &mut Cpu) {
    let r = c.registers.e.wrapping_sub(1);
    c.registers.flag(Z, r == 0);
    c.registers.flag(H, (c.registers.e & 0x0F) == 0);
    c.registers.flag(N, true);
    c.registers.e = r;

    // c.registers.e &= 255;
    // c.fz(c.registers.e, 0);
}
pub fn decr_h(c: &mut Cpu) {
    c.registers.h -= 1;
    c.registers.h &= 255;
    c.fz(c.registers.h, 0);
}
pub fn decr_l(c: &mut Cpu) {
    c.registers.l -= 1;
    c.registers.l &= 255;
    c.fz(c.registers.l, 0);
}
pub fn decr_a(c: &mut Cpu) {
    c.registers.a -= 1;
    c.registers.a &= 255;
    c.fz(c.registers.a, 0);
}
pub fn dechlm(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    let mut i = c.memory.rb(addr) - 1;
    i &= 255;
    c.memory.wb(addr, i);
    c.fz(i, 0);
}
pub fn incbc(c: &mut Cpu) {
    // c.registers.c = (c.registers.c + 1) & 255;
    // if c.registers.c == 0 {
    // c.registers.b = (c.registers.b + 1) & 255;
    let cr_val = (c.registers.c as u16).wrapping_add(1);
    let br_val = ((c.registers.b as u16) << 8).wrapping_add(1);
    c.registers.b = (br_val >> 8) as u8;
    c.registers.c = (cr_val & 0x00FF) as u8;
}
pub fn incde(c: &mut Cpu) {
    c.registers.e = (c.registers.e + 1) & 255;
    if c.registers.e == 0 {
        c.registers.d = (c.registers.d + 1) & 255;
    }
}
pub fn inchl(c: &mut Cpu) {
    let val = ((c.registers.h as u16) << 8) | (c.registers.l as u16).wrapping_add(1);
    c.registers.h = (val >> 8) as u8;
    c.registers.l = (val & 0x00FF) as u8;

    // c.registers.l = (c.registers.l + 1) & 255;
    // if c.registers.l == 0 {
    //     c.registers.h = (c.registers.h + 1) & 255;
    // }
}
pub fn incsp(c: &mut Cpu) {
    c.registers.sp = (c.registers.sp + 1) & 65535;
}
pub fn decbc(c: &mut Cpu) {
    c.registers.c = (c.registers.c - 1) & 255;
    if c.registers.c == 255 {
        c.registers.b = (c.registers.b - 1) & 255;
    }
}
pub fn decde(c: &mut Cpu) {
    c.registers.e = (c.registers.e - 1) & 255;
    if c.registers.e == 255 {
        c.registers.d = (c.registers.d - 1) & 255;
    }
}
pub fn dechl(c: &mut Cpu) {
    // c.registers.l = (c.registers.l - 1) & 255;
    if c.registers.l == 255 {
        c.registers.h = (c.registers.h - 1) & 255;
    }
}
pub fn decsp(c: &mut Cpu) {
    c.registers.sp = (c.registers.sp - 1) & 65535;
}

pub fn cpl(c: &mut Cpu) {
    c.registers.a = !c.registers.a;
    c.registers.flag(H, true);
    c.registers.flag(N, true);
}
pub fn scf(c: &mut Cpu) {
    c.registers.flag(C, true);
    c.registers.flag(H, false);
    c.registers.flag(N, false);
}
pub fn ccf(c: &mut Cpu) {
    let v = !c.registers.getflag(C);
    c.registers.flag(C, v);
    c.registers.flag(H, false);
    c.registers.flag(N, false);
}
