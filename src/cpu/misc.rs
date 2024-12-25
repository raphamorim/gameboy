use crate::cpu::core::Cpu;
use crate::cpu::registers::CpuFlag::{C, H, N, Z};

pub fn bit(cpu: &mut Cpu, a: u8, b: u8) -> u32 {
    let r = a & (1 << (b as u32)) == 0;
    cpu.registers.flag(N, false);
    cpu.registers.flag(H, true);
    cpu.registers.flag(Z, r);
    2
}
pub fn bit_m(cpu: &mut Cpu, i: u8) -> u32 {
    let addr = ((cpu.registers.h as u16) << 8) | cpu.registers.l as u16;
    let value = cpu.memory.rb(addr);
    bit(cpu, value, i);
    3
}
pub fn rla(cpu: &mut Cpu) {
    cpu.registers.a = alu_rl(cpu, cpu.registers.a);
    cpu.registers.flag(Z, false);
}
pub fn rlca(cpu: &mut Cpu) {
    cpu.registers.a = alu_rlc(cpu, cpu.registers.a);
    cpu.registers.flag(Z, false);
}
pub fn rra(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let c = a & 0x01 == 0x01;
    let r = (a >> 1) | (if cpu.registers.getflag(C) { 0x80 } else { 0 });
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    // cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, c);
    cpu.registers.a = r;
    cpu.registers.flag(Z, false);
}
pub fn rrca(cpu: &mut Cpu) {
    cpu.registers.a = alu_rrc(cpu, cpu.registers.a);
    cpu.registers.flag(Z, false);
}

fn alu_srflagupdate(cpu: &mut Cpu, r: u8, c: bool) {
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, c);
}

fn alu_rlc(cpu: &mut Cpu, a: u8) -> u8 {
    let c = a & 0x80 == 0x80;
    let r = (a << 1) | (if c { 1 } else { 0 });
    alu_srflagupdate(cpu, r, c);
    r
}

fn alu_rl(cpu: &mut Cpu, a: u8) -> u8 {
    let c = a & 0x80 == 0x80;
    let r = (a << 1) | (if cpu.registers.getflag(C) { 1 } else { 0 });
    alu_srflagupdate(cpu, r, c);
    r
}

fn alu_rrc(cpu: &mut Cpu, a: u8) -> u8 {
    let c = a & 0x01 == 0x01;
    let r = (a >> 1) | (if c { 0x80 } else { 0 });
    alu_srflagupdate(cpu, r, c);
    r
}

fn alu_rr(cpu: &mut Cpu, a: u8) -> u8 {
    let c = a & 0x01 == 0x01;
    let r = (a >> 1) | (if cpu.registers.getflag(C) { 0x80 } else { 0 });
    alu_srflagupdate(cpu, r, c);
    r
}

fn alu_swap(cpu: &mut Cpu, a: u8) -> u8 {
    cpu.registers.flag(Z, a == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    a.rotate_left(4)
    // (a >> 4) | (a << 4)
}

fn alu_sla(cpu: &mut Cpu, a: u8) -> u8 {
    let c = a & 0x80 == 0x80;
    let r = a << 1;
    alu_srflagupdate(cpu, r, c);
    r
}

fn alu_sra(cpu: &mut Cpu, a: u8) -> u8 {
    let c = a & 0x01 == 0x01;
    let r = (a >> 1) | (a & 0x80);
    alu_srflagupdate(cpu, r, c);
    r
}

fn alu_srl(cpu: &mut Cpu, a: u8) -> u8 {
    let c = a & 0x01 == 0x01;
    let r = a >> 1;
    alu_srflagupdate(cpu, r, c);
    r
}

pub fn cbmap(cpu: &mut Cpu) -> u32 {
    let op = cpu.get_byte();
    match op {
        0 => {
            cpu.registers.b = alu_rlc(cpu, cpu.registers.b);
            2
        }
        1 => {
            cpu.registers.c = alu_rlc(cpu, cpu.registers.c);
            2
        }
        2 => {
            cpu.registers.d = alu_rlc(cpu, cpu.registers.d);
            2
        }
        3 => {
            cpu.registers.e = alu_rlc(cpu, cpu.registers.e);
            2
        }
        4 => {
            cpu.registers.h = alu_rlc(cpu, cpu.registers.h);
            2
        }
        5 => {
            cpu.registers.l = alu_rlc(cpu, cpu.registers.l);
            2
        }
        6 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a);
            let v2 = alu_rlc(cpu, v);
            cpu.memory.wb(a, v2);
            4
        }
        7 => {
            cpu.registers.a = alu_rlc(cpu, cpu.registers.a);
            2
        }
        8 => {
            cpu.registers.b = alu_rrc(cpu, cpu.registers.b);
            2
        }
        9 => {
            cpu.registers.c = alu_rrc(cpu, cpu.registers.c);
            2
        }
        10 => {
            cpu.registers.d = alu_rrc(cpu, cpu.registers.d);
            2
        }
        11 => {
            cpu.registers.e = alu_rrc(cpu, cpu.registers.e);
            2
        }
        12 => {
            cpu.registers.h = alu_rrc(cpu, cpu.registers.h);
            2
        }
        13 => {
            cpu.registers.l = alu_rrc(cpu, cpu.registers.l);
            2
        }
        14 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a);
            let v2 = alu_rrc(cpu, v);
            cpu.memory.wb(a, v2);
            4
        }
        15 => {
            cpu.registers.a = alu_rrc(cpu, cpu.registers.a);
            2
        }
        16 => {
            cpu.registers.b = alu_rl(cpu, cpu.registers.b);
            2
        }
        17 => {
            cpu.registers.c = alu_rl(cpu, cpu.registers.c);
            2
        }
        18 => {
            cpu.registers.d = alu_rl(cpu, cpu.registers.d);
            2
        }
        19 => {
            cpu.registers.e = alu_rl(cpu, cpu.registers.e);
            2
        }
        20 => {
            cpu.registers.h = alu_rl(cpu, cpu.registers.h);
            2
        }
        21 => {
            cpu.registers.l = alu_rl(cpu, cpu.registers.l);
            2
        }
        0x16 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a);
            let v2 = alu_rl(cpu, v);
            cpu.memory.wb(a, v2);
            4
        }
        0x17 => {
            cpu.registers.a = alu_rl(cpu, cpu.registers.a);
            2
        }
        0x18 => {
            cpu.registers.b = alu_rr(cpu, cpu.registers.b);
            2
        }
        0x19 => {
            cpu.registers.c = alu_rr(cpu, cpu.registers.c);
            2
        }
        0x1A => {
            cpu.registers.d = alu_rr(cpu, cpu.registers.d);
            2
        }
        0x1B => {
            cpu.registers.e = alu_rr(cpu, cpu.registers.e);
            2
        }
        0x1C => {
            cpu.registers.h = alu_rr(cpu, cpu.registers.h);
            2
        }
        0x1D => {
            cpu.registers.l = alu_rr(cpu, cpu.registers.l);
            2
        }
        0x1E => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a);
            let v2 = alu_rr(cpu, v);
            cpu.memory.wb(a, v2);
            4
        }
        0x1F => {
            cpu.registers.a = alu_rr(cpu, cpu.registers.a);
            2
        }
        0x20 => {
            cpu.registers.b = alu_sla(cpu, cpu.registers.b);
            2
        }
        0x21 => {
            cpu.registers.c = alu_sla(cpu, cpu.registers.c);
            2
        }
        0x22 => {
            cpu.registers.d = alu_sla(cpu, cpu.registers.d);
            2
        }
        0x23 => {
            cpu.registers.e = alu_sla(cpu, cpu.registers.e);
            2
        }
        0x24 => {
            cpu.registers.h = alu_sla(cpu, cpu.registers.h);
            2
        }
        0x25 => {
            cpu.registers.l = alu_sla(cpu, cpu.registers.l);
            2
        }
        0x26 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a);
            let v2 = alu_sla(cpu, v);
            cpu.memory.wb(a, v2);
            4
        }
        0x27 => {
            cpu.registers.a = alu_sla(cpu, cpu.registers.a);
            2
        }
        0x28 => {
            cpu.registers.b = alu_sra(cpu, cpu.registers.b);
            2
        }
        0x29 => {
            cpu.registers.c = alu_sra(cpu, cpu.registers.c);
            2
        }
        0x2A => {
            cpu.registers.d = alu_sra(cpu, cpu.registers.d);
            2
        }
        0x2B => {
            cpu.registers.e = alu_sra(cpu, cpu.registers.e);
            2
        }
        0x2C => {
            cpu.registers.h = alu_sra(cpu, cpu.registers.h);
            2
        }
        0x2D => {
            cpu.registers.l = alu_sra(cpu, cpu.registers.l);
            2
        }
        0x2E => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a);
            let v2 = alu_sra(cpu, v);
            cpu.memory.wb(a, v2);
            4
        }
        0x2F => {
            cpu.registers.a = alu_sra(cpu, cpu.registers.a);
            2
        }
        0x30 => {
            cpu.registers.b = alu_swap(cpu, cpu.registers.b);
            2
        }
        0x31 => {
            cpu.registers.c = alu_swap(cpu, cpu.registers.c);
            2
        }
        0x32 => {
            cpu.registers.d = alu_swap(cpu, cpu.registers.d);
            2
        }
        0x33 => {
            cpu.registers.e = alu_swap(cpu, cpu.registers.e);
            2
        }
        0x34 => {
            cpu.registers.h = alu_swap(cpu, cpu.registers.h);
            2
        }
        0x35 => {
            cpu.registers.l = alu_swap(cpu, cpu.registers.l);
            2
        }
        0x36 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a);
            let v2 = alu_swap(cpu, v);
            cpu.memory.wb(a, v2);
            4
        }
        0x37 => {
            cpu.registers.a = alu_swap(cpu, cpu.registers.a);
            2
        }
        0x38 => {
            cpu.registers.b = alu_srl(cpu, cpu.registers.b);
            2
        }
        0x39 => {
            cpu.registers.c = alu_srl(cpu, cpu.registers.c);
            2
        }
        0x3A => {
            cpu.registers.d = alu_srl(cpu, cpu.registers.d);
            2
        }
        0x3B => {
            cpu.registers.e = alu_srl(cpu, cpu.registers.e);
            2
        }
        0x3E => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a);
            let v2 = alu_srl(cpu, v);
            cpu.memory.wb(a, v2);
            4
        }
        60 => {
            let a = cpu.registers.h;
            let c = a & 0x01 == 0x01;
            let r = a >> 1;
            cpu.registers.flag(H, false);
            cpu.registers.flag(N, false);
            cpu.registers.flag(Z, r == 0);
            cpu.registers.flag(C, c);
            cpu.registers.h = r;

            2
        }
        61 => {
            let a = cpu.registers.l;
            let c = a & 0x01 == 0x01;
            let r = a >> 1;
            cpu.registers.flag(H, false);
            cpu.registers.flag(N, false);
            cpu.registers.flag(Z, r == 0);
            cpu.registers.flag(C, c);
            cpu.registers.l = r;
            2
        }
        63 => {
            let a = cpu.registers.a;
            let c = a & 0x01 == 0x01;
            let r = a >> 1;

            cpu.registers.flag(H, false);
            cpu.registers.flag(N, false);
            cpu.registers.flag(Z, r == 0);
            cpu.registers.flag(C, c);

            cpu.registers.a = r;
            2
        }
        64 => bit(cpu, cpu.registers.b, 0),
        65 => bit(cpu, cpu.registers.c, 0),
        66 => bit(cpu, cpu.registers.d, 0),
        67 => bit(cpu, cpu.registers.e, 0),
        68 => bit(cpu, cpu.registers.h, 0),
        69 => bit(cpu, cpu.registers.l, 0),
        70 => bit_m(cpu, 0),
        71 => bit(cpu, cpu.registers.a, 0),
        72 => bit(cpu, cpu.registers.b, 1),
        73 => bit(cpu, cpu.registers.c, 1),
        74 => bit(cpu, cpu.registers.d, 1),
        75 => bit(cpu, cpu.registers.e, 1),
        76 => bit(cpu, cpu.registers.h, 1),
        77 => bit(cpu, cpu.registers.l, 1),
        78 => bit_m(cpu, 1),
        79 => bit(cpu, cpu.registers.a, 1),
        80 => bit(cpu, cpu.registers.b, 2),
        81 => bit(cpu, cpu.registers.c, 2),
        82 => bit(cpu, cpu.registers.d, 2),
        83 => bit(cpu, cpu.registers.e, 2),
        84 => bit(cpu, cpu.registers.h, 2),
        85 => bit(cpu, cpu.registers.l, 2),
        86 => bit_m(cpu, 2),
        87 => bit(cpu, cpu.registers.a, 2),
        88 => bit(cpu, cpu.registers.b, 3),
        89 => bit(cpu, cpu.registers.c, 3),
        90 => bit(cpu, cpu.registers.d, 3),
        91 => bit(cpu, cpu.registers.e, 3),
        92 => bit(cpu, cpu.registers.h, 3),
        93 => bit(cpu, cpu.registers.l, 3),
        94 => bit_m(cpu, 3),
        95 => bit(cpu, cpu.registers.a, 3),
        96 => bit(cpu, cpu.registers.b, 4),
        97 => bit(cpu, cpu.registers.c, 4),
        98 => bit(cpu, cpu.registers.d, 4),
        99 => bit(cpu, cpu.registers.e, 4),
        100 => bit(cpu, cpu.registers.h, 4),
        101 => bit(cpu, cpu.registers.l, 4),
        102 => bit_m(cpu, 4),
        103 => bit(cpu, cpu.registers.a, 4),
        104 => bit(cpu, cpu.registers.b, 5),
        105 => bit(cpu, cpu.registers.c, 5),
        106 => bit(cpu, cpu.registers.d, 5),
        107 => bit(cpu, cpu.registers.e, 5),
        108 => bit(cpu, cpu.registers.h, 5),
        109 => bit(cpu, cpu.registers.l, 5),
        110 => bit_m(cpu, 5),
        111 => bit(cpu, cpu.registers.a, 5),
        112 => bit(cpu, cpu.registers.b, 6),
        113 => bit(cpu, cpu.registers.c, 6),
        114 => bit(cpu, cpu.registers.d, 6),
        115 => bit(cpu, cpu.registers.e, 6),
        116 => bit(cpu, cpu.registers.h, 6),
        117 => bit(cpu, cpu.registers.l, 6),
        118 => bit_m(cpu, 6),
        119 => bit(cpu, cpu.registers.a, 6),
        120 => bit(cpu, cpu.registers.b, 7),
        121 => bit(cpu, cpu.registers.c, 7),
        122 => bit(cpu, cpu.registers.d, 7),
        123 => bit(cpu, cpu.registers.e, 7),
        124 => bit(cpu, cpu.registers.h, 7),
        125 => bit(cpu, cpu.registers.l, 7),
        126 => bit_m(cpu, 7),
        127 => bit(cpu, cpu.registers.a, 7),
        0x80 => {
            cpu.registers.b &= !(1 << 0);
            2
        }
        0x81 => {
            cpu.registers.c &= !(1 << 0);
            2
        }
        0x82 => {
            cpu.registers.d &= !(1 << 0);
            2
        }
        0x83 => {
            cpu.registers.e &= !(1 << 0);
            2
        }
        0x84 => {
            cpu.registers.h &= !(1 << 0);
            2
        }
        0x85 => {
            cpu.registers.l &= !(1 << 0);
            2
        }
        0x86 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) & !(1 << 0);
            cpu.memory.wb(a, v);
            4
        }
        0x87 => {
            cpu.registers.a &= !(1 << 0);
            2
        }
        0x88 => {
            cpu.registers.b &= !(1 << 1);
            2
        }
        0x89 => {
            cpu.registers.c &= !(1 << 1);
            2
        }
        0x8A => {
            cpu.registers.d &= !(1 << 1);
            2
        }
        0x8B => {
            cpu.registers.e &= !(1 << 1);
            2
        }
        0x8C => {
            cpu.registers.h &= !(1 << 1);
            2
        }
        0x8D => {
            cpu.registers.l &= !(1 << 1);
            2
        }
        0x8E => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) & !(1 << 1);
            cpu.memory.wb(a, v);
            4
        }
        0x8F => {
            cpu.registers.a &= !(1 << 1);
            2
        }
        0x90 => {
            cpu.registers.b &= !(1 << 2);
            2
        }
        0x91 => {
            cpu.registers.c &= !(1 << 2);
            2
        }
        0x92 => {
            cpu.registers.d &= !(1 << 2);
            2
        }
        0x93 => {
            cpu.registers.e &= !(1 << 2);
            2
        }
        0x94 => {
            cpu.registers.h &= !(1 << 2);
            2
        }
        0x95 => {
            cpu.registers.l &= !(1 << 2);
            2
        }
        0x96 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) & !(1 << 2);
            cpu.memory.wb(a, v);
            4
        }
        0x97 => {
            cpu.registers.a &= !(1 << 2);
            2
        }
        0x98 => {
            cpu.registers.b &= !(1 << 3);
            2
        }
        0x99 => {
            cpu.registers.c &= !(1 << 3);
            2
        }
        0x9A => {
            cpu.registers.d &= !(1 << 3);
            2
        }
        0x9B => {
            cpu.registers.e &= !(1 << 3);
            2
        }
        0x9C => {
            cpu.registers.h &= !(1 << 3);
            2
        }
        0x9D => {
            cpu.registers.l &= !(1 << 3);
            2
        }
        0x9E => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) & !(1 << 3);
            cpu.memory.wb(a, v);
            4
        }
        0x9F => {
            cpu.registers.a &= !(1 << 3);
            2
        }
        0xA0 => {
            cpu.registers.b &= !(1 << 4);
            2
        }
        0xA1 => {
            cpu.registers.c &= !(1 << 4);
            2
        }
        0xA2 => {
            cpu.registers.d &= !(1 << 4);
            2
        }
        0xA3 => {
            cpu.registers.e &= !(1 << 4);
            2
        }
        0xA4 => {
            cpu.registers.h &= !(1 << 4);
            2
        }
        0xA5 => {
            cpu.registers.l &= !(1 << 4);
            2
        }
        0xA6 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) & !(1 << 4);
            cpu.memory.wb(a, v);
            4
        }
        0xA7 => {
            cpu.registers.a &= !(1 << 4);
            2
        }
        0xA8 => {
            cpu.registers.b &= !(1 << 5);
            2
        }
        0xA9 => {
            cpu.registers.c &= !(1 << 5);
            2
        }
        0xAA => {
            cpu.registers.d &= !(1 << 5);
            2
        }
        0xAB => {
            cpu.registers.e &= !(1 << 5);
            2
        }
        0xAC => {
            cpu.registers.h &= !(1 << 5);
            2
        }
        0xAD => {
            cpu.registers.l &= !(1 << 5);
            2
        }
        0xAE => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) & !(1 << 5);
            cpu.memory.wb(a, v);
            4
        }
        0xAF => {
            cpu.registers.a &= !(1 << 5);
            2
        }
        0xB0 => {
            cpu.registers.b &= !(1 << 6);
            2
        }
        0xB1 => {
            cpu.registers.c &= !(1 << 6);
            2
        }
        0xB2 => {
            cpu.registers.d &= !(1 << 6);
            2
        }
        0xB3 => {
            cpu.registers.e &= !(1 << 6);
            2
        }
        0xB4 => {
            cpu.registers.h &= !(1 << 6);
            2
        }
        0xB5 => {
            cpu.registers.l &= !(1 << 6);
            2
        }
        0xB6 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) & !(1 << 6);
            cpu.memory.wb(a, v);
            4
        }
        0xB7 => {
            cpu.registers.a &= !(1 << 6);
            2
        }
        0xB8 => {
            cpu.registers.b &= !(1 << 7);
            2
        }
        0xB9 => {
            cpu.registers.c &= !(1 << 7);
            2
        }
        0xBA => {
            cpu.registers.d &= !(1 << 7);
            2
        }
        0xBB => {
            cpu.registers.e &= !(1 << 7);
            2
        }
        0xBC => {
            cpu.registers.h &= !(1 << 7);
            2
        }
        0xBD => {
            cpu.registers.l &= !(1 << 7);
            2
        }
        0xBE => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) & !(1 << 7);
            cpu.memory.wb(a, v);
            4
        }
        0xBF => {
            cpu.registers.a &= !(1 << 7);
            2
        }
        0xC0 => {
            cpu.registers.b |= 1 << 0;
            2
        }
        0xC1 => {
            cpu.registers.c |= 1 << 0;
            2
        }
        0xC2 => {
            cpu.registers.d |= 1 << 0;
            2
        }
        0xC3 => {
            cpu.registers.e |= 1 << 0;
            2
        }
        0xC4 => {
            cpu.registers.h |= 1 << 0;
            2
        }
        0xC5 => {
            cpu.registers.l |= 1 << 0;
            2
        }
        0xC6 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) | (1 << 0);
            cpu.memory.wb(a, v);
            4
        }
        0xC7 => {
            cpu.registers.a |= 1 << 0;
            2
        }
        0xC8 => {
            cpu.registers.b |= 1 << 1;
            2
        }
        0xC9 => {
            cpu.registers.c |= 1 << 1;
            2
        }
        0xCA => {
            cpu.registers.d |= 1 << 1;
            2
        }
        0xCB => {
            cpu.registers.e |= 1 << 1;
            2
        }
        0xCC => {
            cpu.registers.h |= 1 << 1;
            2
        }
        0xCD => {
            cpu.registers.l |= 1 << 1;
            2
        }
        0xCE => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) | (1 << 1);
            cpu.memory.wb(a, v);
            4
        }
        0xCF => {
            cpu.registers.a |= 1 << 1;
            2
        }
        0xD0 => {
            cpu.registers.b |= 1 << 2;
            2
        }
        0xD1 => {
            cpu.registers.c |= 1 << 2;
            2
        }
        0xD2 => {
            cpu.registers.d |= 1 << 2;
            2
        }
        0xD3 => {
            cpu.registers.e |= 1 << 2;
            2
        }
        0xD4 => {
            cpu.registers.h |= 1 << 2;
            2
        }
        0xD5 => {
            cpu.registers.l |= 1 << 2;
            2
        }
        0xD6 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) | (1 << 2);
            cpu.memory.wb(a, v);
            4
        }
        0xD7 => {
            cpu.registers.a |= 1 << 2;
            2
        }
        0xD8 => {
            cpu.registers.b |= 1 << 3;
            2
        }
        0xD9 => {
            cpu.registers.c |= 1 << 3;
            2
        }
        0xDA => {
            cpu.registers.d |= 1 << 3;
            2
        }
        0xDB => {
            cpu.registers.e |= 1 << 3;
            2
        }
        0xDC => {
            cpu.registers.h |= 1 << 3;
            2
        }
        0xDD => {
            cpu.registers.l |= 1 << 3;
            2
        }
        0xDE => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) | (1 << 3);
            cpu.memory.wb(a, v);
            4
        }
        0xDF => {
            cpu.registers.a |= 1 << 3;
            2
        }
        0xE0 => {
            cpu.registers.b |= 1 << 4;
            2
        }
        0xE1 => {
            cpu.registers.c |= 1 << 4;
            2
        }
        0xE2 => {
            cpu.registers.d |= 1 << 4;
            2
        }
        0xE3 => {
            cpu.registers.e |= 1 << 4;
            2
        }
        0xE4 => {
            cpu.registers.h |= 1 << 4;
            2
        }
        0xE5 => {
            cpu.registers.l |= 1 << 4;
            2
        }
        0xE6 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) | (1 << 4);
            cpu.memory.wb(a, v);
            4
        }
        0xE7 => {
            cpu.registers.a |= 1 << 4;
            2
        }
        0xE8 => {
            cpu.registers.b |= 1 << 5;
            2
        }
        0xE9 => {
            cpu.registers.c |= 1 << 5;
            2
        }
        0xEA => {
            cpu.registers.d |= 1 << 5;
            2
        }
        0xEB => {
            cpu.registers.e |= 1 << 5;
            2
        }
        0xEC => {
            cpu.registers.h |= 1 << 5;
            2
        }
        0xED => {
            cpu.registers.l |= 1 << 5;
            2
        }
        0xEE => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) | (1 << 5);
            cpu.memory.wb(a, v);
            4
        }
        0xEF => {
            cpu.registers.a |= 1 << 5;
            2
        }
        0xF0 => {
            cpu.registers.b |= 1 << 6;
            2
        }
        0xF1 => {
            cpu.registers.c |= 1 << 6;
            2
        }
        0xF2 => {
            cpu.registers.d |= 1 << 6;
            2
        }
        0xF3 => {
            cpu.registers.e |= 1 << 6;
            2
        }
        0xF4 => {
            cpu.registers.h |= 1 << 6;
            2
        }
        0xF5 => {
            cpu.registers.l |= 1 << 6;
            2
        }
        0xF6 => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) | (1 << 6);
            cpu.memory.wb(a, v);
            4
        }
        0xF7 => {
            cpu.registers.a |= 1 << 6;
            2
        }
        0xF8 => {
            cpu.registers.b |= 1 << 7;
            2
        }
        0xF9 => {
            cpu.registers.c |= 1 << 7;
            2
        }
        0xFA => {
            cpu.registers.d |= 1 << 7;
            2
        }
        0xFB => {
            cpu.registers.e |= 1 << 7;
            2
        }
        0xFC => {
            cpu.registers.h |= 1 << 7;
            2
        }
        0xFD => {
            cpu.registers.l |= 1 << 7;
            2
        }
        0xFE => {
            let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
            let v = cpu.memory.rb(a) | (1 << 7);
            cpu.memory.wb(a, v);
            4
        }
        0xFF => {
            cpu.registers.a |= 1 << 7;
            2
        }
    }
}
