use crate::cpu::cpu::Cpu;
use crate::cpu::registers::CpuFlag::{C, H, N, Z};
use crate::mmu::mmu::Mmu;

pub fn addr_b(c: &mut Cpu) {
    c._r.a += c._r.b;
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_c(c: &mut Cpu) {
    c._r.a += c._r.c;
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_d(c: &mut Cpu) {
    c._r.a += c._r.d;
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_e(c: &mut Cpu) {
    c._r.a += c._r.e;
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_h(c: &mut Cpu) {
    c._r.a += c._r.h;
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_l(c: &mut Cpu) {
    c._r.a += c._r.l;
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_a(c: &mut Cpu) {
    c._r.a += c._r.a;
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addhl(c: &mut Cpu, m: &mut Mmu) {
    let addr = ((c._r.h as u16) << 8) + c._r.l as u16;
    c._r.a += m.r8b(addr);
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn addn(c: &mut Cpu, m: &mut Mmu) {
    c._r.a += m.r8b(c._r.pc);
    c._r.pc += 1;
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn addhlbc(c: &mut Cpu) {
    let b = ((c._r.b as u16) << 8) | (c._r.c as u16);
    let a = ((c._r.h as u16) << 8) | (c._r.l as u16);
    let r = a.wrapping_add(b);
    c._r.flag(H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
    c._r.flag(N, false);
    c._r.flag(C, a > 0xFFFF - b);
    c._r.h = (r >> 8) as u8;
    c._r.l = (r & 0x00FF) as u8;

    // let mut hl = ((c._r.h as u16) << 8) + c._r.l as u16;
    // hl += ((c._r.b as u16) << 8) + c._r.c as u16;
    // if hl > 65535 {
    //     c._r.f |= 0x10;
    // } else {
    //     c._r.f &= 0xEF;
    // }
    // c._r.h = ((hl >> 8) & 255) as u8;
    // c._r.l = (hl & 255) as u8;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn addhlde(c: &mut Cpu) {
    // let mut hl: u16 = ((c._r.h as u16) << 8) + c._r.l as u16;
    // hl += ((c._r.d as u16) << 8) + c._r.e as u16;

    let de = ((c._r.d as u16) << 8) | (c._r.e as u16);
    let a = ((c._r.h as u16) << 8) | (c._r.l as u16);
    let r = a.wrapping_add(de);
    c._r.flag(H, (a & 0x07FF) + (de & 0x07FF) > 0x07FF);
    c._r.flag(N, false);
    c._r.flag(C, a > 0xFFFF - de);
    c._r.h = (r >> 8) as u8;
    c._r.l = (r & 0x00FF) as u8;

    c._r.m = 3;
    c._r.t = 12;
}
pub fn addhlhl(c: &mut Cpu) {
    let mut hl: u16 = ((c._r.h as u16) << 8) + c._r.l as u16;
    hl += ((c._r.h as u16) << 8) + c._r.l as u16;

    if hl > 65535 {
        c._r.f |= 0x10;
    } else {
        c._r.f &= 0xEF;
        c._r.h = ((hl >> 8) & 255) as u8;
        c._r.l = (hl & 255) as u8;
        c._r.m = 3;
        c._r.t = 12;
    }
}
pub fn addhlsp(c: &mut Cpu) {
    let mut hl = ((c._r.h as u16) << 8) + c._r.l as u16;
    hl += c._r.sp;
    if hl > 65535 {
        c._r.f |= 0x10;
    } else {
        c._r.f &= 0xEF;
    }
    c._r.h = ((hl >> 8) & 255) as u8;
    c._r.l = (hl & 255) as u8;
    c._r.m = 3;
    c._r.t = 12;
}
// pub fn addspn(c: &mut Cpu, m: &mut Mmu) {
//     let i = m.r8b(c._r.pc);
//     if i > 127 {
//         // i=-{ ((~i+1)&255)

//     }
//     c._r.pc += 1;
//     c._r.sp += i as u16;
//     c._r.m = 4;
//     c._r.t = 16;
// }
// https://github.com/alexcrichton/jba/blob/rust/src/cpu/z80/imp.rs#L81
pub fn addspn(c: &mut Cpu, m: &mut Mmu) {
    let b = m.r8b(c._r.pc) as i8 as i16 as u16;
    let res = c._r.sp + b;
    let tmp = b ^ res ^ c._r.sp;
    c._r.f = if tmp & 0x100 != 0 { 0x10 } else { 0 } | if tmp & 0x010 != 0 { 0x20 } else { 0 };
    c._r.sp = res;
}
pub fn adcr_b(c: &mut Cpu) {
    c._r.a += c._r.b;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn adcr_c(c: &mut Cpu) {
    c._r.a += c._r.c;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn adcr_d(c: &mut Cpu) {
    c._r.a += c._r.d;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn adcr_e(c: &mut Cpu) {
    c._r.a += c._r.e;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn adcr_h(c: &mut Cpu) {
    c._r.a += c._r.h;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn adcr_l(c: &mut Cpu) {
    c._r.a += c._r.l;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn adcr_a(c: &mut Cpu) {
    c._r.a += c._r.a;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn adchl(c: &mut Cpu, m: &mut Mmu) {
    let addr = ((c._r.h as u16) << 8) + c._r.l as u16;
    c._r.a += m.r8b(addr);
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn adcn(c: &mut Cpu, m: &mut Mmu) {
    c._r.a += m.r8b(c._r.pc);
    c._r.pc += 1;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > u8::MAX {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn subr_b(c: &mut Cpu) {
    c._r.a -= c._r.b;
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn subr_c(c: &mut Cpu) {
    let b = c._r.c;
    let a = c._r.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    c._r.flag(Z, r == 0);
    c._r.flag(H, (a & 0x0F) < (b & 0x0F));
    c._r.flag(N, true);
    c._r.flag(C, (a as u16) < (b as u16));
    c._r.a = r;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn subr_d(c: &mut Cpu) {
    c._r.a -= c._r.d;
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn subr_e(c: &mut Cpu) {
    c._r.a -= c._r.e;
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn subr_h(c: &mut Cpu) {
    c._r.a -= c._r.h;
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn subr_l(c: &mut Cpu) {
    c._r.a -= c._r.l;
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn subr_a(c: &mut Cpu) {
    // let c = if usec && self.reg.getflag(C) { 1 } else { 0 };
    let b = c._r.a;
    let a = c._r.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    c._r.flag(Z, r == 0);
    c._r.flag(H, (a & 0x0F) < (b & 0x0F));
    c._r.flag(N, true);
    c._r.flag(C, (a as u16) < (b as u16));
    c._r.a = r;

    c._r.m = 1;
    c._r.t = 4;
}
pub fn subhl(c: &mut Cpu, m: &mut Mmu) {
    let addr = ((c._r.h as u16) << 8) + c._r.l as u16;
    c._r.a -= m.r8b(addr);
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn subn(c: &mut Cpu, m: &mut Mmu) {
    c._r.a -= m.r8b(c._r.pc);
    c._r.pc += 1;
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}

pub fn sbcr_b(c: &mut Cpu) {
    c._r.a -= c._r.b;
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn sbcr_c(c: &mut Cpu) {
    c._r.a -= c._r.c;
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn sbcr_d(c: &mut Cpu) {
    c._r.a -= c._r.d;
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn sbcr_e(c: &mut Cpu) {
    c._r.a -= c._r.e;
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn sbcr_h(c: &mut Cpu) {
    c._r.a -= c._r.h;
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn sbcr_l(c: &mut Cpu) {
    c._r.a -= c._r.l;
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn sbcr_a(c: &mut Cpu) {
    c._r.a -= c._r.a;
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn sbchl(c: &mut Cpu, m: &mut Mmu) {
    let addr = ((c._r.h as u16) << 8) + c._r.l as u16;
    c._r.a -= m.r8b(addr);
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn sbcn(c: &mut Cpu, m: &mut Mmu) {
    c._r.a -= m.r8b(c._r.pc);
    c._r.pc += 1;
    if c._r.f >= 0x10 {
        c._r.a -= 1
    } else {
        c._r.a -= 0;
    }
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}

pub fn cpr_b(c: &mut Cpu) {
    let mut i = c._r.a;
    i -= c._r.b;
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn cpr_c(c: &mut Cpu) {
    let mut i = c._r.a;
    i -= c._r.c;
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn cpr_d(c: &mut Cpu) {
    let mut i = c._r.a;
    i -= c._r.d;
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn cpr_e(c: &mut Cpu) {
    let mut i = c._r.a;
    i -= c._r.e;
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn cpr_h(c: &mut Cpu) {
    let mut i = c._r.a;
    i -= c._r.h;
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn cpr_l(c: &mut Cpu) {
    let mut i = c._r.a;
    i -= c._r.l;
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn cpr_a(c: &mut Cpu) {
    let mut i = c._r.a;
    i -= c._r.a;
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn cphl(c: &mut Cpu, m: &mut Mmu) {
    let mut i = c._r.a;
    let addr = ((c._r.h as u16) << 8) + c._r.l as u16;
    i -= m.r8b(addr);
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn cpn(c: &mut Cpu, m: &mut Mmu) {
    // let v = self.fetchbyte(); self.alu_cp(v)

    let b = c.get_byte(m);
    let a = c._r.a;

    // self.alu_sub(b, false);
    // self.reg.a = r;

    let r = c._r.a.wrapping_sub(b);
    c._r.flag(Z, r == 0);
    c._r.flag(H, (a & 0x0F) < (b & 0x0F));
    c._r.flag(N, true);
    c._r.flag(C, (a as u16) < (b as u16));

    // c._r.a = r;
    c._r.a = a;

    // let mut i = c._r.a as u16;
    // i = m.r16b(c._r.pc) - i;
    // c._r.pc += 1;
    // c.fz(i as u8, 1);
    // if i < 255 {
    //     c._r.f |= 0x10;
    // }
    // i &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn andr_b(c: &mut Cpu) {
    c._r.a &= c._r.b;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn andr_c(c: &mut Cpu) {
    c._r.a &= c._r.c;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn andr_d(c: &mut Cpu) {
    c._r.a &= c._r.d;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn andr_e(c: &mut Cpu) {
    c._r.a &= c._r.e;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn andr_h(c: &mut Cpu) {
    c._r.a &= c._r.h;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn andr_l(c: &mut Cpu) {
    c._r.a &= c._r.l;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn andr_a(c: &mut Cpu) {
    c._r.a &= c._r.a;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn andhl(c: &mut Cpu, m: &mut Mmu) {
    let addr = ((c._r.h as u16) << 8) + c._r.l as u16;
    c._r.a &= m.r8b(addr);
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 2;
    c._r.t = 8;
}
pub fn andn(c: &mut Cpu, m: &mut Mmu) {
    let v = c.get_byte(m);
    let r = c._r.a & v;
    c._r.flag(Z, r == 0);
    c._r.flag(H, true);
    c._r.flag(C, false);
    c._r.flag(N, false);
    c._r.a = r;

    // c._r.a &= m.r8b(c._r.pc);
    // c._r.pc += 1;
    // c._r.a &= 255;
    // c.fz(c._r.a, 0);
    c._r.m = 2;
    c._r.t = 8;
}
pub fn orr_b(c: &mut Cpu) {
    c._r.a |= c._r.b;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn orr_c(c: &mut Cpu) {
    c._r.a |= c._r.c;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn orr_d(c: &mut Cpu) {
    c._r.a |= c._r.d;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn orr_e(c: &mut Cpu) {
    c._r.a |= c._r.e;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn orr_h(c: &mut Cpu) {
    c._r.a |= c._r.h;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn orr_l(c: &mut Cpu) {
    c._r.a |= c._r.l;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn orr_a(c: &mut Cpu) {
    c._r.a |= c._r.a;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn orhl(c: &mut Cpu, m: &mut Mmu) {
    let a = (c._r.h as u16) << 8;
    let addr: u16 = a + (c._r.l as u16);
    c._r.a |= m.r8b(addr);
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 2;
    c._r.t = 8;
}
pub fn orn(c: &mut Cpu, m: &mut Mmu) {
    c._r.a |= m.r8b(c._r.pc);
    c._r.pc += 1;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 2;
    c._r.t = 8;
}
pub fn xorr_b(c: &mut Cpu) {
    c._r.a ^= c._r.b;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn xorr_c(c: &mut Cpu) {
    c._r.a ^= c._r.c;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn xorr_d(c: &mut Cpu) {
    c._r.a ^= c._r.d;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn xorr_e(c: &mut Cpu) {
    c._r.a ^= c._r.e;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn xorr_h(c: &mut Cpu) {
    c._r.a ^= c._r.h;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn xorr_l(c: &mut Cpu) {
    c._r.a ^= c._r.l;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn xorr_a(c: &mut Cpu) {
    c._r.a ^= c._r.a;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn xorhl(c: &mut Cpu, m: &mut Mmu) {
    let addr = (c._r.h as u16) << 8 + (c._r.l as u16);
    c._r.a ^= m.r8b(addr);
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 2;
    c._r.t = 8;
}
pub fn xorn(c: &mut Cpu, m: &mut Mmu) {
    c._r.a ^= m.r8b(c._r.pc);
    c._r.pc += 1;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 2;
    c._r.t = 8;
}
pub fn incr_b(c: &mut Cpu) {
    c._r.b += 1;
    c._r.b &= 255;
    c.fz(c._r.b, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn incr_c(c: &mut Cpu) {
    c._r.c += 1;
    c._r.c &= 255;
    c.fz(c._r.c, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn incr_d(c: &mut Cpu) {
    c._r.d += 1;
    c._r.d &= 255;
    c.fz(c._r.d, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn incr_e(c: &mut Cpu) {
    c._r.e += 1;
    c._r.e &= 255;
    c.fz(c._r.e, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn incr_h(c: &mut Cpu) {
    c._r.h += 1;
    c._r.h &= 255;
    c.fz(c._r.h, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn incr_l(c: &mut Cpu) {
    c._r.l += 1;
    c._r.l &= 255;
    c.fz(c._r.l, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn incr_a(c: &mut Cpu) {
    c._r.a += 1;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn inchlm(c: &mut Cpu, m: &mut Mmu) {
    let addr = ((c._r.h as u16) << 8) + (c._r.l as u16);
    let mut i = m.r8b(addr) + 1;
    i &= 255;
    let waddr = ((c._r.h as u16) << 8) + c._r.l as u16;
    m.w8b(waddr, i);
    c.fz(i, 0);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn decr_b(c: &mut Cpu) {
    // if c._r.b > 0 {
    //     c._r.b -= 1;
    // }
    // c._r.b &= 255;
    // c.fz(c._r.b, 0);

    let a = c._r.b;
    let r = a.wrapping_sub(1);
    c._r.flag(Z, r == 0);
    c._r.flag(H, (a & 0x0F) == 0);
    c._r.flag(N, true);
    c._r.b = r;

    c._r.m = 1;
    c._r.t = 4;
}
pub fn decr_c(c: &mut Cpu) {
    c._r.c -= 1;
    c._r.c &= 255;
    c.fz(c._r.c, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decr_d(c: &mut Cpu) {
    let d = c._r.d;
    let r = d.wrapping_sub(1);
    // c._r.d &= 255;
    // c.fz(c._r.d, 0);
    c._r.flag(Z, r == 0);
    c._r.flag(H, (d & 0x0F) == 0);
    c._r.flag(N, true);
    c._r.d = r;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decr_e(c: &mut Cpu) {
    let r = c._r.e.wrapping_sub(1);
    c._r.flag(Z, r == 0);
    c._r.flag(H, (c._r.e & 0x0F) == 0);
    c._r.flag(N, true);
    c._r.e = r;

    // c._r.e &= 255;
    // c.fz(c._r.e, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decr_h(c: &mut Cpu) {
    c._r.h -= 1;
    c._r.h &= 255;
    c.fz(c._r.h, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decr_l(c: &mut Cpu) {
    c._r.l -= 1;
    c._r.l &= 255;
    c.fz(c._r.l, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decr_a(c: &mut Cpu) {
    c._r.a -= 1;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn dechlm(c: &mut Cpu, m: &mut Mmu) {
    let addr = ((c._r.h as u16) << 8) + c._r.l as u16;
    let mut i = m.r8b(addr) - 1;
    i &= 255;
    m.w8b(addr, i);
    c.fz(i, 0);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn incbc(c: &mut Cpu) {
    // c._r.c = (c._r.c + 1) & 255;
    // if c._r.c == 0 {
    // c._r.b = (c._r.b + 1) & 255;
    let cr_val = (c._r.c as u16).wrapping_add(1);
    let br_val = ((c._r.b as u16) << 8).wrapping_add(1);
    c._r.b = (br_val >> 8) as u8;
    c._r.c = (cr_val & 0x00FF) as u8;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn incde(c: &mut Cpu) {
    c._r.e = (c._r.e + 1) & 255;
    if c._r.e == 0 {
        c._r.d = (c._r.d + 1) & 255;
    }
    c._r.m = 1;
    c._r.t = 4;
}
pub fn inchl(c: &mut Cpu) {
    let val = ((c._r.h as u16) << 8) | (c._r.l as u16).wrapping_add(1);
    c._r.h = (val >> 8) as u8;
    c._r.l = (val & 0x00FF) as u8;

    // c._r.l = (c._r.l + 1) & 255;
    // if c._r.l == 0 {
    //     c._r.h = (c._r.h + 1) & 255;
    // }
    c._r.m = 1;
    c._r.t = 4;
}
pub fn incsp(c: &mut Cpu) {
    c._r.sp = (c._r.sp + 1) & 65535;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decbc(c: &mut Cpu) {
    c._r.c = (c._r.c - 1) & 255;
    if c._r.c == 255 {
        c._r.b = (c._r.b - 1) & 255;
    }
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decde(c: &mut Cpu) {
    c._r.e = (c._r.e - 1) & 255;
    if c._r.e == 255 {
        c._r.d = (c._r.d - 1) & 255;
    }
    c._r.m = 1;
    c._r.t = 4;
}
pub fn dechl(c: &mut Cpu) {
    // c._r.l = (c._r.l - 1) & 255;
    if c._r.l == 255 {
        c._r.h = (c._r.h - 1) & 255;
    }
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decsp(c: &mut Cpu) {
    c._r.sp = (c._r.sp - 1) & 65535;
    c._r.m = 1;
    c._r.t = 4;
}

pub fn cpl(c: &mut Cpu) {
    c._r.a = !c._r.a;
    c._r.flag(H, true);
    c._r.flag(N, true);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn scf(c: &mut Cpu) {
    c._r.flag(C, true);
    c._r.flag(H, false);
    c._r.flag(N, false);
}
pub fn ccf(c: &mut Cpu) {
    let v = !c._r.getflag(C);
    c._r.flag(C, v);
    c._r.flag(H, false);
    c._r.flag(N, false);
}
