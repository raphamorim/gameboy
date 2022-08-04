use crate::cpu::cpu::Cpu;
use crate::mmu::mmu::Mmu;

pub fn addr_b(c: &mut Cpu) {
    c._r.a += c._r.b;
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_c(c: &mut Cpu) {
    c._r.a += c._r.c;
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_d(c: &mut Cpu) {
    c._r.a += c._r.d;
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_e(c: &mut Cpu) {
    c._r.a += c._r.e;
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_h(c: &mut Cpu) {
    c._r.a += c._r.h;
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_l(c: &mut Cpu) {
    c._r.a += c._r.l;
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addr_a(c: &mut Cpu) {
    c._r.a += c._r.a;
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn addhl(c: &mut Cpu, m: &mut Mmu) {
    c._r.a += m.rr8b((c._r.h << 8) + c._r.l);
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
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
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn addhlbc(c: &mut Cpu) {
    let mut hl = ((c._r.h << 8) + c._r.l) as u16;
    hl += ((c._r.b << 8) + c._r.c) as u16;
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
pub fn addhlde(c: &mut Cpu) {
    let mut hl = ((c._r.h << 8) + c._r.l) as u16;
    hl += ((c._r.d << 8) + c._r.e) as u16;
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
pub fn addhlhl(c: &mut Cpu) {
    let mut hl = ((c._r.h << 8) + c._r.l) as u16;
    hl += ((c._r.h << 8) + c._r.l) as u16;
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
    let mut hl = ((c._r.h << 8) + c._r.l) as u16;
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
pub fn addspn(c: &mut Cpu, m: &mut Mmu) {
    let i = m.r8b(c._r.pc);
    if i > 127 {
        // i=-{ ((~i+1)&255)
    }
    c._r.pc += 1;
    c._r.sp += i as u16;
    c._r.m = 4;
    c._r.t = 16;
}

pub fn adcr_b(c: &mut Cpu) {
    c._r.a += c._r.b;
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
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
    if c._r.a > 255 {
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
    if c._r.a > 255 {
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
    if c._r.a > 255 {
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
    if c._r.a > 255 {
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
    if c._r.a > 255 {
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
    if c._r.a > 255 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn adchl(c: &mut Cpu, m: &mut Mmu) {
    c._r.a += m.rr8b((c._r.h << 8) + c._r.l);
    if c._r.f >= 0x10 {
        c._r.a += 1
    } else {
        c._r.a += 0
    }
    c.fz(c._r.a, 0);
    if c._r.a > 255 {
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
    if c._r.a > 255 {
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
    c._r.a -= c._r.c;
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
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
    c._r.a -= c._r.a;
    c.fz(c._r.a, 1);
    if c._r.a < 0 {
        c._r.f |= 0x10;
    }
    c._r.a &= 255;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn subhl(c: &mut Cpu, m: &mut Mmu) {
    c._r.a -= m.rr8b((c._r.h << 8) + c._r.l);
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
    c._r.a -= m.rr8b((c._r.h << 8) + c._r.l);
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
    i -= m.rr8b((c._r.h << 8) + c._r.l);
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
    c._r.m = 2;
    c._r.t = 8;
}
pub fn cpn(c: &mut Cpu, m: &mut Mmu) {
    let mut i = c._r.a;
    i -= m.r8b(c._r.pc);
    c._r.pc += 1;
    c.fz(i, 1);
    if i < 0 {
        c._r.f |= 0x10;
    }
    i &= 255;
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
    c._r.a &= m.rr8b((c._r.h << 8) + c._r.l);
    c._r.a &= 255;
    c.fz(c._r.a, 0);
    c._r.m = 2;
    c._r.t = 8;
}
pub fn andn(c: &mut Cpu, m: &mut Mmu) {
    c._r.a &= m.r8b(c._r.pc);
    c._r.pc += 1;
    c._r.a &= 255;
    c.fz(c._r.a, 0);
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
    c._r.a |= m.rr8b((c._r.h << 8) + c._r.l);
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
    c._r.a ^= m.rr8b((c._r.h << 8) + c._r.l);
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
    let mut i = m.rr8b((c._r.h << 8) + c._r.l) + 1;
    i &= 255;
    m.ww8b((c._r.h << 8) + c._r.l, i);
    c.fz(i, 0);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn decr_b(c: &mut Cpu) {
    c._r.b -= 1;
    c._r.b &= 255;
    c.fz(c._r.b, 0);
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
    c._r.d -= 1;
    c._r.d &= 255;
    c.fz(c._r.d, 0);
    c._r.m = 1;
    c._r.t = 4;
}
pub fn decr_e(c: &mut Cpu) {
    c._r.e -= 1;
    c._r.e &= 255;
    c.fz(c._r.e, 0);
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
    let mut i = m.rr8b((c._r.h << 8) + c._r.l) - 1;
    i &= 255;
    m.ww8b((c._r.h << 8) + c._r.l, i);
    c.fz(i, 0);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn incbc(c: &mut Cpu) {
    c._r.c = (c._r.c + 1) & 255;
    if c._r.c == 0 {
        c._r.b = (c._r.b + 1) & 255;
    }
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
    c._r.l = (c._r.l + 1) & 255;
    if c._r.l == 0 {
        c._r.h = (c._r.h + 1) & 255;
    }
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
    c._r.l = (c._r.l - 1) & 255;
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
