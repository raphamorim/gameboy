use crate::cpu::cpu::Cpu;
use crate::mmu::mmu::Mmu;

const ZFLAG: i32 = 0b10000000;

pub fn pushbc(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 1;
    m.w8b(c._r.sp, c._r.b);
    c._r.sp -= 1;
    m.w8b(c._r.sp, c._r.c);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn pushde(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 1;
    m.w8b(c._r.sp, c._r.d);
    c._r.sp -= 1;
    m.w8b(c._r.sp, c._r.e);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn pushhl(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 1;
    m.w8b(c._r.sp, c._r.h);
    c._r.sp -= 1;
    m.w8b(c._r.sp, c._r.l);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn pushaf(c: &mut Cpu, m: &mut Mmu) {
    if c._r.sp > 0 {
        c._r.sp -= 1;
    }
    m.w8b(c._r.sp, c._r.a);
    if c._r.sp > 0 {
        c._r.sp -= 1;
    }
    m.w8b(c._r.sp, c._r.f);
    c._r.m = 3;
    c._r.t = 12;
}

pub fn popbc(c: &mut Cpu, m: &mut Mmu) {
    c._r.c = m.r8b(c._r.sp);
    c._r.sp += 1;
    c._r.b = m.r8b(c._r.sp);
    c._r.sp += 1;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn popde(c: &mut Cpu, m: &mut Mmu) {
    c._r.e = m.r8b(c._r.sp);
    c._r.sp += 1;
    c._r.d = m.r8b(c._r.sp);
    c._r.sp += 1;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn pophl(c: &mut Cpu, m: &mut Mmu) {
    c._r.l = m.r8b(c._r.sp);
    c._r.sp += 1;
    c._r.h = m.r8b(c._r.sp);
    c._r.sp += 1;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn popaf(c: &mut Cpu, m: &mut Mmu) {
    c._r.f = m.r8b(c._r.sp);
    c._r.sp += 1;
    c._r.a = m.r8b(c._r.sp);
    c._r.sp += 1;
    c._r.m = 3;
    c._r.t = 12;
}

pub fn jpnn(c: &mut Cpu, m: &mut Mmu) {
    // c._r.pc = m.r16b(c._r.pc);
    c._r.pc = c.get_word(m);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn jphl(c: &mut Cpu) {
    c._r.pc = c._r.h as u16;
    c._r.m = 1;
    c._r.t = 4;
}
pub fn jpnznn(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 3;
    c._r.t = 12;
    let mask = ZFLAG as u8;
    let is = c._r.f & mask > 0;
    // if (c._r.f & 0x80) == 0x00 {
    if is {
        c._r.pc = m.r16b(c._r.pc);
        c._r.m += 1;
        c._r.t += 4;
    } else {
        c._r.pc += 2;
    }
}
pub fn jpznn(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 3;
    c._r.t = 12;
    if (c._r.f & 0x80) == 0x80 {
        c._r.pc = m.r16b(c._r.pc);
        c._r.m += 1;
        c._r.t += 4;
    } else {
        c._r.pc += 2;
    }
}
pub fn jpncnn(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 3;
    c._r.t = 12;
    if (c._r.f & 0x10) == 0x00 {
        c._r.pc = m.r16b(c._r.pc);
        c._r.m += 1;
        c._r.t += 4;
    } else {
        c._r.pc += 2;
    }
}
pub fn jpcnn(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 3;
    c._r.t = 12;
    if (c._r.f & 0x10) == 0x10 {
        c._r.pc = m.r16b(c._r.pc);
        c._r.m += 1;
        c._r.t += 4;
    } else {
        c._r.pc += 2;
    }
}

pub fn jrn(c: &mut Cpu, m: &mut Mmu) {
    let mut i = m.r8b(c._r.pc);
    if i > 127 {
        // i=-((!i+1)&255);
        i = 1;
    }
    c._r.pc += 1;
    c._r.m = 2;
    c._r.t = 8;
    c._r.pc += i as u16;
    c._r.m += 1;
    c._r.t += 4;
}
pub fn jrnzn(c: &mut Cpu, m: &mut Mmu) {
    let mut i = m.r8b(c._r.pc);
    if i > 127 {
        // i=-((~i+1)&255);
        i = 1;
    }
    c._r.pc += 1;
    c._r.m = 2;
    c._r.t = 8;
    if (c._r.f & 0x80) == 0x00 {
        c._r.pc += i as u16;
        c._r.m += 1;
        c._r.t += 4;
    }
}
pub fn jrzn(c: &mut Cpu, m: &mut Mmu) {
    let mut i = m.r8b(c._r.pc);
    if i > 127 {
        // i=-((~i+1)&255);
        i = 1;
    }
    c._r.pc += 1;
    c._r.m = 2;
    c._r.t = 8;
    if (c._r.f & 0x80) == 0x80 {
        c._r.pc += i as u16;
        c._r.m += 1;
        c._r.t += 4;
    }
}

// 0x38 => { if self.reg.getflag(C) { self.cpu_jr(); 3 } else { self.reg.pc += 1; 2  } },
pub fn jrncn(c: &mut Cpu, m: &mut Mmu) {
    // let c = m.r8b(c._r.c);
    if c._r.c > 0 {
        let n = c.get_byte(m) as i8;
        c._r.pc = ((c._r.pc as u32 as i32) + (n as i32)) as u16;
        c._r.m += 1;
        c._r.t += 4;
    } else {
        c._r.pc += 1;
        c._r.m = 2;
        c._r.t = 8;
    }

    // let mut i = m.r8b(c._r.pc);
    // if i > 127 {
    //     // i=-((~i+1)&255);
    //     i = (i - 1) & 255;
    // }

    // if (c._r.f & 0x10) == 0x00 {
    // c._r.pc += i as u16;
    // c._r.m += 1;
    // c._r.t += 4;
    // }
}
pub fn jrcn(c: &mut Cpu, m: &mut Mmu) {
    let mut i = m.r8b(c._r.pc);
    if i > 127 {
        // i=-((~i+1)&255);
        i = 1;
    }
    c._r.pc += 1;
    c._r.m = 2;
    c._r.t = 8;
    if (c._r.f & 0x10) == 0x10 {
        c._r.pc += i as u16;
        c._r.m += 1;
        c._r.t += 4;
    }
}

pub fn djnzn(c: &mut Cpu, m: &mut Mmu) {
    let mut i = m.r8b(c._r.pc);
    if i > 127 {
        // i=-((~i+1)&255)
        i = 1;
    };
    c._r.pc += 1;
    c._r.m = 2;
    c._r.t = 8;
    c._r.b -= 1;
    if c._r.b > 0 {
        c._r.pc += i as u16;
        c._r.m += 1;
        c._r.t += 4;
    }
}

pub fn callnn(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc + 2);
    c._r.pc = m.r16b(c._r.pc);
    c._r.m = 5;
    c._r.t = 20;
}
pub fn callnznn(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 3;
    c._r.t = 12;
    if (c._r.f & 0x80) == 0x00 {
        c._r.sp -= 2;
        m.w16b(c._r.sp, c._r.pc + 2);
        c._r.pc = m.r16b(c._r.pc);
        c._r.m += 2;
        c._r.t += 8;
    } else {
        c._r.pc += 2;
    }
}
pub fn callznn(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 3;
    c._r.t = 12;
    if (c._r.f & 0x80) == 0x80 {
        c._r.sp -= 2;
        m.w16b(c._r.sp, c._r.pc + 2);
        c._r.pc = m.r16b(c._r.pc);
        c._r.m += 2;
        c._r.t += 8;
    } else {
        c._r.pc += 2;
    }
}
pub fn callncnn(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 3;
    c._r.t = 12;
    if (c._r.f & 0x10) == 0x00 {
        c._r.sp -= 2;
        m.w16b(c._r.sp, c._r.pc + 2);
        c._r.pc = m.r16b(c._r.pc);
        c._r.m += 2;
        c._r.t += 8;
    } else {
        c._r.pc += 2;
    }
}
pub fn callcnn(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 3;
    c._r.t = 12;
    if (c._r.f & 0x10) == 0x10 {
        c._r.sp -= 2;
        m.w16b(c._r.sp, c._r.pc + 2);
        c._r.pc = m.r16b(c._r.pc);
        c._r.m += 2;
        c._r.t += 8;
    } else {
        c._r.pc += 2;
    }
}
pub fn ret(c: &mut Cpu, m: &mut Mmu) {
    c._r.pc = m.r16b(c._r.sp) as u16;
    c._r.sp += 2;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn reti(c: &mut Cpu, m: &mut Mmu) {
    c._r.ime = 1;
    c._r.pc = m.r16b(c._r.sp);
    c._r.sp += 2;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn retnz(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 1;
    c._r.t = 4;
    if (c._r.f & 0x80) == 0x00 {
        c._r.pc = m.r16b(c._r.sp);
        c._r.sp += 2;
        c._r.m += 2;
        c._r.t += 8;
    }
}
pub fn retz(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 1;
    c._r.t = 4;
    if (c._r.f & 0x80) == 0x80 {
        c._r.pc = m.r16b(c._r.sp);
        c._r.sp += 2;
        c._r.m += 2;
        c._r.t += 8;
    }
}
pub fn retnc(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 1;
    c._r.t = 4;
    if (c._r.f & 0x10) == 0x00 {
        c._r.pc = m.r16b(c._r.sp);
        c._r.sp += 2;
        c._r.m += 2;
        c._r.t += 8;
    }
}
pub fn retc(c: &mut Cpu, m: &mut Mmu) {
    c._r.m = 1;
    c._r.t = 4;
    if (c._r.f & 0x10) == 0x10 {
        c._r.pc = m.r16b(c._r.sp);
        c._r.sp += 2;
        c._r.m += 2;
        c._r.t += 8;
    }
}

pub fn rst00(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x00;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst08(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x08;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst10(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x10;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst18(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x18;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst20(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x20;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst28(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x28;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst30(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x30;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst38(c: &mut Cpu, m: &mut Mmu) {
    // c._r.sp -= 2;
    // m.w16b(c._r.sp, c._r.pc);
    // c._r.pc = 0x38;
    c._r.a = c._r.a | (1 << 7);
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst40(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x40;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst48(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x48;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst50(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x50;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst58(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x58;
    c._r.m = 3;
    c._r.t = 12;
}
pub fn rst60(c: &mut Cpu, m: &mut Mmu) {
    c._r.sp -= 2;
    m.w16b(c._r.sp, c._r.pc);
    c._r.pc = 0x60;
    c._r.m = 3;
    c._r.t = 12;
}
