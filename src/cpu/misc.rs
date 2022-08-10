use crate::cpu::cpu::Cpu;
use crate::cpu::registers::CpuFlag::{C, H, N, Z};
use crate::cpu::{bit, swap};

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
    let mut ci = 0;
    let mut co = 0;
    if (cpu.registers.f & 0x10) > 0 {
        ci = 0x80;
    }
    if (cpu.registers.a & 1) > 0 {
        co = 0x10;
    }

    cpu.registers.a = (cpu.registers.a >> 1) + ci;
    cpu.registers.f = (cpu.registers.f & 0xEF) + co;
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

pub fn cbmap(cpu: &mut Cpu) -> u32 {
    let op = cpu.get_byte();
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
            swap::r_b(cpu);
            2
        }
        49 => {
            swap::r_c(cpu);
            2
        }
        50 => {
            swap::r_d(cpu);
            2
        }
        51 => {
            swap::r_e(cpu);
            2
        }
        52 => {
            swap::r_h(cpu);
            2
        }
        53 => {
            swap::r_l(cpu);
            2
        }
        55 => {
            swap::r_a(cpu);
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
        64 => {
            bit::bit0b(cpu);
            2
        }
        65 => {
            bit::bit0c(cpu);
            2
        }
        66 => {
            bit::bit0d(cpu);
            2
        }
        67 => {
            bit::bit0e(cpu);
            2
        }
        68 => {
            bit::bit0h(cpu);
            2
        }
        69 => {
            bit::bit0l(cpu);
            2
        }
        70 => {
            bit::bit0m(cpu);
            2
        }
        71 => {
            bit::bit0a(cpu);
            2
        }
        72 => {
            bit::bit1b(cpu);
            2
        }
        73 => {
            bit::bit1c(cpu);
            2
        }
        74 => {
            bit::bit1d(cpu);
            2
        }
        75 => {
            bit::bit1e(cpu);
            2
        }
        76 => {
            bit::bit1h(cpu);
            2
        }
        77 => {
            bit::bit1l(cpu);
            2
        }
        78 => {
            bit::bit1m(cpu);
            2
        }
        79 => {
            bit::bit1a(cpu);
            2
        }
        80 => {
            bit::bit2b(cpu);
            2
        }
        81 => {
            bit::bit2c(cpu);
            2
        }
        82 => {
            bit::bit2d(cpu);
            2
        }
        83 => {
            bit::bit2e(cpu);
            2
        }
        84 => {
            bit::bit2h(cpu);
            2
        }
        85 => {
            bit::bit2l(cpu);
            2
        }
        86 => {
            bit::bit2m(cpu);
            2
        }
        87 => {
            bit::bit2a(cpu);
            2
        }
        88 => {
            bit::bit3b(cpu);
            2
        }
        89 => {
            bit::bit3c(cpu);
            2
        }
        90 => {
            bit::bit3d(cpu);
            2
        }
        91 => {
            bit::bit3e(cpu);
            2
        }
        92 => {
            bit::bit3h(cpu);
            2
        }
        93 => {
            bit::bit3l(cpu);
            2
        }
        94 => {
            bit::bit3m(cpu);
            2
        }
        95 => {
            bit::bit3a(cpu);
            2
        }
        96 => {
            bit::bit4b(cpu);
            2
        }
        97 => {
            bit::bit4c(cpu);
            2
        }
        98 => {
            bit::bit4d(cpu);
            2
        }
        99 => {
            bit::bit4e(cpu);
            2
        }
        100 => {
            bit::bit4h(cpu);
            2
        }
        101 => {
            bit::bit4l(cpu);
            2
        }
        102 => {
            bit::bit4m(cpu);
            2
        }
        103 => {
            bit::bit4a(cpu);
            2
        }
        104 => {
            bit::bit5b(cpu);
            2
        }
        105 => {
            bit::bit5c(cpu);
            2
        }
        106 => {
            bit::bit5d(cpu);
            2
        }
        107 => {
            bit::bit5e(cpu);
            2
        }
        108 => {
            bit::bit5h(cpu);
            2
        }
        109 => {
            bit::bit5l(cpu);
            2
        }
        110 => {
            bit::bit5m(cpu);
            2
        }
        111 => {
            bit::bit5a(cpu);
            2
        }
        112 => {
            bit::bit6b(cpu);
            2
        }
        113 => {
            bit::bit6c(cpu);
            2
        }
        114 => {
            bit::bit6d(cpu);
            2
        }
        115 => {
            bit::bit6e(cpu);
            2
        }
        116 => {
            bit::bit6h(cpu);
            2
        }
        117 => {
            bit::bit6l(cpu);
            2
        }
        118 => {
            bit::bit6m(cpu);
            2
        }
        119 => {
            bit::bit6a(cpu);
            2
        }
        120 => {
            bit::bit7b(cpu);
            2
        }
        121 => {
            bit::bit7c(cpu);
            2
        }
        122 => {
            bit::bit7d(cpu);
            2
        }
        123 => {
            bit::bit7e(cpu);
            2
        }
        124 => {
            bit::bit7h(cpu);
            2
        }
        125 => {
            bit::bit7l(cpu);
            2
        }
        126 => {
            bit::bit7m(cpu);
            2
        }
        127 => {
            bit::bit7a(cpu);
            2
        }
        _ => {
            println!(
                "cbmap -> Instruction at {:#01x} | {} not implemented, stopping.",
                op, op
            );
            cpu.stop = 1;
            0
        }
    }
}
