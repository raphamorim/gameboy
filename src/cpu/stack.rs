use crate::cpu::cpu::Cpu;
use crate::cpu::registers::CpuFlag::{C, Z};

pub fn pushbc(c: &mut Cpu) {
    let value = ((c.registers.b as u16) << 8) | (c.registers.c as u16);
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, value);
}
pub fn pushde(c: &mut Cpu) {
    let value = ((c.registers.d as u16) << 8) | (c.registers.e as u16);
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, value);
}
pub fn pushhl(c: &mut Cpu) {
    let value = ((c.registers.h as u16) << 8) | (c.registers.l as u16);
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, value);
}
pub fn pushaf(c: &mut Cpu) {
    let value = ((c.registers.a as u16) << 8) | ((c.registers.f & 0xF0) as u16);
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, value);
}
pub fn popbc(c: &mut Cpu) {
    let val = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    c.registers.b = (val >> 8) as u8;
    c.registers.c = (val & 0x00FF) as u8
}
pub fn popde(c: &mut Cpu) {
    let val = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    c.registers.d = (val >> 8) as u8;
    c.registers.e = (val & 0x00FF) as u8;
}
pub fn pophl(c: &mut Cpu) {
    let value = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    c.registers.h = (value >> 8) as u8;
    c.registers.l = (value & 0x00FF) as u8;
}
pub fn popaf(c: &mut Cpu) {
    let res = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    let v = res & 0xFFF0;
    c.registers.a = (v >> 8) as u8;
    c.registers.f = (v & 0x00F0) as u8;
}
pub fn jpnn(c: &mut Cpu) {
    c.registers.pc = c.get_word();
}
pub fn jphl(c: &mut Cpu) {
    c.registers.pc = c.registers.h as u16;
}
pub fn jpnznn(c: &mut Cpu) -> u32 {
    if !c.registers.getflag(Z) {
        c.registers.pc = c.get_word();
        4
    } else {
        c.registers.pc += 2;
        3
    }
}
pub fn jpznn(c: &mut Cpu) -> u32 {
    if c.registers.getflag(Z) {
        c.registers.pc = c.memory.rw(c.registers.pc);
        4
    } else {
        c.registers.pc += 2;
        3
    }
}
pub fn jpncnn(c: &mut Cpu) -> u32 {
    if !c.registers.getflag(C) {
        c.registers.pc = c.memory.rw(c.registers.pc);
        4
    } else {
        c.registers.pc += 2;
        3
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
}
pub fn jrnzn(c: &mut Cpu) -> u32 {
    if !c.registers.getflag(Z) {
        let n = c.get_byte() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;
        3
    } else {
        c.registers.pc += 1;
        2
    }
}
pub fn jrzn(c: &mut Cpu) -> u32 {
    if c.registers.getflag(Z) {
        let n = c.get_byte() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;
        3
    } else {
        c.registers.pc += 1;
        2
    }
}

pub fn jrncn(c: &mut Cpu) -> u32 {
    if !c.registers.getflag(C) {
        let n = c.get_byte() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;
        3
    } else {
        c.registers.pc += 1;
        2
    }
}
pub fn jrcn(c: &mut Cpu) -> u32 {
    if c.registers.getflag(C) {
        let n = c.get_byte() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (n as i32)) as u16;
        3
    } else {
        c.registers.pc += 1;
        2
    }
}

// Switch speed
// pub fn djnzn(c: &mut Cpu) {
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
// }

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
pub fn callznn(c: &mut Cpu) -> u32 {
    if c.registers.getflag(Z) {
        c.registers.sp -= 2;
        c.memory.ww(c.registers.sp, c.registers.pc + 2);
        c.registers.pc = c.memory.rw(c.registers.pc);
        6
    } else {
        c.registers.pc += 2;
        3
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
pub fn callcnn(c: &mut Cpu) -> u32 {
    if c.registers.getflag(C) {
        c.registers.sp -= 2;
        c.memory.ww(c.registers.sp, c.registers.pc + 2);
        c.registers.pc = c.get_word();
        6
    } else {
        c.registers.pc += 2;
        3
    }
}
pub fn ret(c: &mut Cpu) {
    let val = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    c.registers.pc = val;
}
pub fn reti(c: &mut Cpu) {
    let val = c.memory.rw(c.registers.sp);
    c.registers.sp += 2;
    c.registers.pc = val;
    c.ime = 1;
}
pub fn retnz(c: &mut Cpu) {
    if (c.registers.f & 0x80) == 0x00 {
        c.registers.pc = c.memory.rw(c.registers.sp);
        c.registers.sp += 2;
    }
}
pub fn retz(c: &mut Cpu) -> u32 {
    if c.registers.getflag(Z) {
        c.registers.pc = c.memory.rw(c.registers.sp);
        c.registers.sp += 2;
        5
    } else {
        2
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
pub fn rst(c: &mut Cpu, val: u16) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = val;
}
