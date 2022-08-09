use crate::cpu::cpu::Cpu;
use crate::cpu::registers::CpuFlag::{C, H, N, Z};

pub fn pushbc(c: &mut Cpu) {
    let value = ((c.registers.b as u16) << 8) | (c.registers.c as u16);
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, value);

    // c.registers.sp -= 1;
    // c.memory.wb(c.registers.sp, c.registers.b);
    // c.registers.sp -= 1;
    // c.memory.wb(c.registers.sp, c.registers.c);
}
pub fn pushde(c: &mut Cpu) {
    let value = ((c.registers.d as u16) << 8) | (c.registers.e as u16);
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, value);

    // c.registers.sp -= 1;
    // c.memory.wb(c.registers.sp, c.registers.d);
    // c.registers.sp -= 1;
    // c.memory.wb(c.registers.sp, c.registers.e);
}
pub fn pushhl(c: &mut Cpu) {
    c.registers.sp -= 1;
    c.memory.wb(c.registers.sp, c.registers.h);
    c.registers.sp -= 1;
    c.memory.wb(c.registers.sp, c.registers.l);
}
pub fn pushaf(c: &mut Cpu) {
    if c.registers.sp > 0 {
        c.registers.sp -= 1;
    }
    c.memory.wb(c.registers.sp, c.registers.a);
    if c.registers.sp > 0 {
        c.registers.sp -= 1;
    }
    c.memory.wb(c.registers.sp, c.registers.f);
}

pub fn popbc(c: &mut Cpu) {
    let val = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    c.registers.b = (val >> 8) as u8;
    c.registers.c = (val & 0x00FF) as u8;

    // c.registers.c = c.memory.rb(c.registers.sp);
    // c.registers.sp += 1;
    // c.registers.b = c.memory.rb(c.registers.sp);
    // c.registers.sp += 1;
}
pub fn popde(c: &mut Cpu) {
    // c.registers.e = c.memory.rb(c.registers.sp);
    // c.registers.sp += 1;
    // c.registers.d = c.memory.rb(c.registers.sp);
    // c.registers.sp += 1;
    let val = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    c.registers.d = (val >> 8) as u8;
    c.registers.e = (val & 0x00FF) as u8;
}
pub fn pophl(c: &mut Cpu) {
    c.registers.l = c.memory.rb(c.registers.sp);
    c.registers.sp += 1;
    c.registers.h = c.memory.rb(c.registers.sp);
    c.registers.sp += 1;
}
pub fn popaf(c: &mut Cpu) {
    c.registers.f = c.memory.rb(c.registers.sp);
    c.registers.sp += 1;
    c.registers.a = c.memory.rb(c.registers.sp);
    c.registers.sp += 1;
}
pub fn jpnn(c: &mut Cpu) {
    // c.registers.pc = c.memory.rw(c.registers.pc);
    c.registers.pc = c.get_word();
}
pub fn jphl(c: &mut Cpu) {
    c.registers.pc = c.registers.h as u16;
}
pub fn jpnznn(c: &mut Cpu) {
    if !c.registers.getflag(Z) {
        c.registers.pc = c.get_word();
    } else {
        c.registers.pc += 2;
    }
}
pub fn jpznn(c: &mut Cpu) {
    if (c.registers.f & 0x80) == 0x80 {
        c.registers.pc = c.memory.rw(c.registers.pc);
    } else {
        c.registers.pc += 2;
    }
}
pub fn jpncnn(c: &mut Cpu) {
    if (c.registers.f & 0x10) == 0x00 {
        c.registers.pc = c.memory.rw(c.registers.pc);
    } else {
        c.registers.pc += 2;
    }
}
pub fn jpcnn(c: &mut Cpu) {
    if (c.registers.f & 0x10) == 0x10 {
        c.registers.pc = c.memory.rw(c.registers.pc);
    } else {
        c.registers.pc += 2;
    }
}

pub fn jrn(c: &mut Cpu) {
    let n = c.get_byte() as i8;
    c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;

    // c.registers.pc += i as u16;
}
pub fn jrnzn(c: &mut Cpu) {
    if !c.registers.getflag(Z) {
        let n = c.get_byte() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;
    } else {
        c.registers.pc += 1;
    }
}
pub fn jrzn(c: &mut Cpu) {
    if c.registers.getflag(Z) {
        let n = c.get_byte() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;
    } else {
        c.registers.pc += 1;
    }
}

// 0x38 => { if self.reg.getflag(C) { self.cpu_jr(); 3 } else { self.reg.pc += 1; 2  } },
pub fn jrncn(c: &mut Cpu) {
    // let c = c.memory.rb(c.registers.c);
    if c.registers.c > 0 {
        let n = c.get_byte() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;
    } else {
        c.registers.pc += 1;
    }
}
pub fn jrcn(c: &mut Cpu) {
    if c.registers.getflag(C) {
        let n = c.get_byte() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;
    } else {
        c.registers.pc += 1;
    }
}

// Switch speed
pub fn djnzn(c: &mut Cpu) {
    // let mut i = c.memory.rb(c.registers.pc);
    // if i > 127 {
    //     // i=-((~i+1)&255)
    //     i = 1;
    // };
    // c.registers.pc += 1;
    // c.registers.b -= 1;
    // if c.registers.b > 0 {
    //     c.registers.pc += i as u16;

    // }
}

pub fn callnn(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc + 2);
    c.registers.pc = c.get_word();
}
pub fn callnznn(c: &mut Cpu) {
    if (c.registers.f & 0x80) == 0x00 {
        c.registers.sp -= 2;
        c.memory.ww(c.registers.sp, c.registers.pc + 2);
        c.registers.pc = c.memory.rw(c.registers.pc);
    } else {
        c.registers.pc += 2;
    }
}
pub fn callznn(c: &mut Cpu) {
    if (c.registers.f & 0x80) == 0x80 {
        c.registers.sp -= 2;
        c.memory.ww(c.registers.sp, c.registers.pc + 2);
        c.registers.pc = c.memory.rw(c.registers.pc);
    } else {
        c.registers.pc += 2;
    }
}
pub fn callncnn(c: &mut Cpu) {
    if (c.registers.f & 0x10) == 0x00 {
        c.registers.sp -= 2;
        c.memory.ww(c.registers.sp, c.registers.pc + 2);
        c.registers.pc = c.memory.rw(c.registers.pc);
    } else {
        c.registers.pc += 2;
    }
}
pub fn callcnn(c: &mut Cpu) {
    if (c.registers.f & 0x10) == 0x10 {
        c.registers.sp -= 2;
        c.memory.ww(c.registers.sp, c.registers.pc + 2);
        c.registers.pc = c.memory.rw(c.registers.pc);
    } else {
        c.registers.pc += 2;
    }
}
pub fn ret(c: &mut Cpu) {
    // c.registers.pc = c.memory.rw(c.registers.sp) as u16;
    // c.registers.sp += 2;

    let res = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    c.registers.pc = res;
}
pub fn reti(c: &mut Cpu) {
    c.registers.ime = 1;
    c.registers.pc = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
}
pub fn retnz(c: &mut Cpu) {
    if (c.registers.f & 0x80) == 0x00 {
        c.registers.pc = c.memory.rw(c.registers.sp);
        c.registers.sp += 2;
    }
}
pub fn retz(c: &mut Cpu) {
    if (c.registers.f & 0x80) == 0x80 {
        c.registers.pc = c.memory.rw(c.registers.sp);
        c.registers.sp += 2;
    }
}
pub fn retnc(c: &mut Cpu) {
    if (c.registers.f & 0x10) == 0x00 {
        c.registers.pc = c.memory.rw(c.registers.sp);
        c.registers.sp += 2;
    }
}
pub fn retc(c: &mut Cpu) {
    if (c.registers.f & 0x10) == 0x10 {
        c.registers.pc = c.memory.rw(c.registers.sp);
        c.registers.sp += 2;
    }
}

pub fn rst00(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x00;
}
pub fn rst08(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x08;
}
pub fn rst10(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x10;
}
pub fn rst18(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x18;
}
pub fn rst20(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x20;
}
pub fn rst28(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x28;
}
pub fn rst30(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x30;
}
pub fn rst38(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x38;
}
pub fn rst40(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x40;
}
pub fn rst48(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x48;
}
pub fn rst50(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x50;
}
pub fn rst58(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x58;
}
pub fn rst60(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = 0x60;
}
pub fn rst(c: &mut Cpu, i: u16) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = i;
}
