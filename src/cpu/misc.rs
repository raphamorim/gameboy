use crate::cpu::cpu::Cpu;
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
    let mut ci = 0;
    let mut co = 0;
    if (cpu.registers.f & 0x10) > 0 {
        ci = 1;
    }
    if (cpu.registers.a & 0x80) > 0 {
        co = 0x10;
    }
    cpu.registers.a = (cpu.registers.a << 1) + ci;
    cpu.registers.f = (cpu.registers.f & 0xEF) + co;
}
pub fn rlca(cpu: &mut Cpu) {
    let mut ci = 0;
    let mut co = 0;
    if (cpu.registers.a & 0x80) > 0 {
        ci = 1;
        co = 0x10;
    }

    cpu.registers.a = (cpu.registers.a << 1) + ci;
    cpu.registers.f = (cpu.registers.f & 0xEF) + co;
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
    let mut ci = 0;
    let mut co = 0;
    if (cpu.registers.a & 1) > 0 {
        ci = 0x80;
        co = 0x10;
    }

    cpu.registers.a = (cpu.registers.a >> 1) + ci;
    cpu.registers.f = (cpu.registers.f & 0xEF) + co;
}
pub fn r_b(cpu: &mut Cpu) {
    let r = cpu.registers.b;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.b = (r >> 4) | (r << 4);
}
pub fn r_c(cpu: &mut Cpu) {
    let r = cpu.registers.c;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.c = (r >> 4) | (r << 4);
}
pub fn r_d(cpu: &mut Cpu) {
    let r = cpu.registers.d;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.d = (r >> 4) | (r << 4);
}
pub fn r_e(cpu: &mut Cpu) {
    let r = cpu.registers.e;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.e = (r >> 4) | (r << 4);
}
pub fn r_h(cpu: &mut Cpu) {
    let r = cpu.registers.h;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.h = (r >> 4) | (r << 4);
}
pub fn r_l(cpu: &mut Cpu) {
    let r = cpu.registers.l;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.l = (r >> 4) | (r << 4);
}
pub fn r_a(cpu: &mut Cpu) {
    let r = cpu.registers.a;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = (r >> 4) | (r << 4);
}
pub fn cbmap(cpu: &mut Cpu) -> u32 {
    let op = cpu.get_byte();
    // println!("cbmap {:?}", op);
    match op {
        // CB00
        // 0 => cpu.RLCr_b,
        // 1 => cpu.RLCr_c,
        // 2 => cpu.RLCr_d,
        // 3 => cpu.RLCr_e,
        // 4 => cpu.RLCr_h,
        // 5 => cpu.RLCr_l,
        // 6 => cpu.RLCHL,
        // 7 => cpu.RLCr_a,
        // 8 => cpu.RRCr_b,
        // 9 => cpu.RRCr_c,
        // 10 => cpu.RRCr_d,
        // 11 => cpu.RRCr_e,
        // 12 => cpu.RRCr_h,
        // 13 => cpu.RRCr_l,
        // 14 => cpu.RRCHL,
        // 15 => cpu.RRCr_a,
        // 16 => cpu.RLr_b,
        // 17 => cpu.RLr_c,
        18 => {
            let a = cpu.registers.d;
            let c = a & 0x80 == 0x80;
            let r = (a << 1) | (if c { 1 } else { 0 });

            cpu.registers.flag(H, false);
            cpu.registers.flag(N, false);
            cpu.registers.flag(Z, r == 0);
            cpu.registers.flag(C, c);

            cpu.registers.d = r;
            2
        }
        // 19 => cpu.RLr_e,
        // 20 => cpu.RLr_h,
        // 21 => cpu.RLr_l,
        // 22 => cpu.RLHL,
        // 23 => cpu.RLr_a,
        // 24 => cpu.RRr_b,
        // 25 => cpu.RRr_c,
        // 26 => cpu.RRr_d,
        // 27 => cpu.RRr_e,
        28 => {
            let a = cpu.registers.h;
            let c = a & 0x01 == 0x01;
            let r = (a >> 1) | (if cpu.registers.getflag(C) { 0x80 } else { 0 });

            cpu.registers.flag(H, false);
            cpu.registers.flag(N, false);
            cpu.registers.flag(Z, r == 0);
            cpu.registers.flag(C, c);

            cpu.registers.h = r;
            2
        }
        29 => {
            let a = cpu.registers.l;
            let c = a & 0x01 == 0x01;
            let r = (a >> 1) | (if cpu.registers.getflag(C) { 0x80 } else { 0 });
            cpu.registers.flag(H, false);
            cpu.registers.flag(N, false);
            cpu.registers.flag(Z, r == 0);
            cpu.registers.flag(C, c);

            cpu.registers.l = r;
            2
        }
        // 30 => cpu.RRHL,
        // 31 => cpu.RRr_a,
        // 32 => cpu.SLAr_b,
        // 33 => cpu.SLAr_c,
        // 34 => cpu.SLAr_d,
        35 => {
            let a = cpu.registers.e;
            let c = a & 0x80 == 0x80;
            let r = a << 1;

            cpu.registers.flag(H, false);
            cpu.registers.flag(N, false);
            cpu.registers.flag(Z, r == 0);
            cpu.registers.flag(C, c);

            cpu.registers.e = r;
            2
        }
        // 36 => cpu.SLAr_h,
        // 37 => cpu.SLAr_l,
        39 => {
            let a = cpu.registers.a;
            let c = a & 0x80 == 0x80;
            let r = a << 1;
            cpu.registers.flag(H, false);
            cpu.registers.flag(N, false);
            cpu.registers.flag(Z, r == 0);
            cpu.registers.flag(C, c);
            cpu.registers.a = r;
            2
        }
        // 40 => cpu.SRAr_b,
        // 41 => cpu.SRAr_c,
        // 42 => cpu.SRAr_d,
        // 43 => cpu.SRAr_e,
        // 44 => cpu.SRAr_h,
        // 45 => cpu.SRAr_l,
        // 47 => { cpu.SRAr_a; 2 },
        48 => {
            r_b(cpu);
            2
        }
        49 => {
            r_c(cpu);
            2
        }
        50 => {
            r_d(cpu);
            2
        }
        51 => {
            r_e(cpu);
            2
        }
        52 => {
            r_h(cpu);
            2
        }
        53 => {
            r_l(cpu);
            2
        }
        54 => {
            // let a = self.reg.hl(); let v = self.memory.rb(a); let v2 = self.alu_swap(v); self.memory.wb(a, v2); 4
            4
        }
        55 => {
            r_a(cpu);
            2
        }
        // 56 => { cpu.SRLr_b; 2 },
        // 57 => { cpu.SRLr_c; 2 },
        // 58 => { cpu.SRLr_d; 2 },
        // 59 => { cpu.SRLr_e; 2 },
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
        _ => {
            println!(
                "cbmap -> Instruction at {:#01x} | {} not implemented",
                op, op
            );
            0
        }
    }
}
