use crate::cpu::core::Cpu;
use crate::cpu::registers::CpuFlag::{C, Z};

pub fn pushstack(cpu: &mut Cpu, value: u16) {
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(2);
    cpu.memory.ww(cpu.registers.sp, value);
}

fn popstack(cpu: &mut Cpu) -> u16 {
    let res = cpu.memory.rw(cpu.registers.sp);
    cpu.registers.sp += 2;
    res
}

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
    let value = ((c.registers.h as u16) << 8) | (c.registers.l as u16);
    c.registers.pc = value;
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
pub fn jpcnn(cpu: &mut Cpu) -> u32 {
    if cpu.registers.getflag(C) {
        cpu.registers.pc = cpu.get_word();
        4
    } else {
        cpu.registers.pc += 2;
        3
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

pub fn callnn(c: &mut Cpu) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc + 2);
    c.registers.pc = c.get_word();
}
pub fn callnznn(c: &mut Cpu) -> u32 {
    if !c.registers.getflag(Z) {
        c.registers.sp -= 2;
        c.memory.ww(c.registers.sp, c.registers.pc + 2);
        c.registers.pc = c.get_word();
        6
    } else {
        c.registers.pc += 2;
        3
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
pub fn callncnn(c: &mut Cpu) -> u32 {
    if !c.registers.getflag(C) {
        pushstack(c, c.registers.pc + 2);
        c.registers.pc = c.get_word();
        6
    } else {
        c.registers.pc += 2;
        3
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
}
pub fn retnz(c: &mut Cpu) -> u32 {
    if !c.registers.getflag(Z) {
        c.registers.pc = popstack(c);
        5
    } else {
        2
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
pub fn retnc(c: &mut Cpu) -> u32 {
    if !c.registers.getflag(C) {
        let res = c.memory.rw(c.registers.sp);
        c.registers.sp += 2;
        c.registers.pc = res;
        5
    } else {
        2
    }
}
pub fn retc(cpu: &mut Cpu) -> u32 {
    if cpu.registers.getflag(C) {
        cpu.registers.pc = popstack(cpu);
        5
    } else {
        2
    }
}
pub fn rst(c: &mut Cpu, val: u16) {
    c.registers.sp -= 2;
    c.memory.ww(c.registers.sp, c.registers.pc);
    c.registers.pc = val;
}
