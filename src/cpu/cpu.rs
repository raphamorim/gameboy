use crate::cpu::registers::CpuFlag::*;
use crate::cpu::registers::Registers;
use crate::mmu::mmu::MMU;
use crate::mode::GbMode;

#[allow(dead_code)]
pub enum Interrupt {
    Vblank = 0x01,
    LCDStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

pub struct Cpu<'a> {
    pub registers: Registers,
    ime: bool,
    pub halt: u32,
    pub stop: u32,
    pub memory: MMU<'a>,
    delay: u32,
    ticks: u32,

    halted: bool,
    setdi: u32,
    setei: u32,

    // To debug
    _executed_operations: Vec<u8>,
}

impl Cpu<'_> {
    pub fn new(data: Vec<u8>, file: Option<std::path::PathBuf>) -> Self {
        let cpu_mmu = MMU::new_cgb(data, file).unwrap();
        let registers = Registers::new(cpu_mmu.gbmode);

        Cpu {
            registers,
            memory: cpu_mmu,
            halt: 0,
            setdi: 0,
            setei: 0,
            stop: 0,
            ticks: 0,
            delay: 0,
            halted: false,
            ime: true,
            _executed_operations: Vec::new(),
        }
    }

    pub fn do_cycle(&mut self) -> u32 {
        let ticks = self.exec() * 4;
        return self.memory.do_cycle(ticks);
    }

    fn updateime(&mut self) {
        self.setdi = match self.setdi {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }
            _ => 0,
        };
        self.setei = match self.setei {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }
            _ => 0,
        };
    }
    pub fn get_byte(&mut self) -> u8 {
        let pc = self.memory.rb(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        pc
    }
    pub fn get_word(&mut self) -> u16 {
        let w = self.memory.rw(self.registers.pc);
        self.registers.pc += 2;
        w
    }

    pub fn exec(&mut self) -> u32 {
        self.updateime();
        match self.handleinterrupt() {
            0 => {}
            n => return n,
        };

        if self.halted {
            // Emulate an noop instruction
            1
        } else {
            self.call()
        }
    }

    fn handleinterrupt(&mut self) -> u32 {
        if self.ime == false && self.halted == false {
            return 0;
        }

        let triggered = self.memory.inte & self.memory.intf;
        if triggered == 0 {
            return 0;
        }

        self.halted = false;
        if self.ime == false {
            return 0;
        }
        self.ime = false;

        let n = triggered.trailing_zeros();
        if n >= 5 {
            panic!("Invalid interrupt triggered");
        }
        self.memory.intf &= !(1 << n);
        let pc = self.registers.pc;
        self.pushstack(pc);
        self.registers.pc = 0x0040 | ((n as u16) << 3);

        return 4;
    }

    fn call(&mut self) -> u32 {
        let opcode = self.get_byte();
        match opcode {
            0x00 => 1,
            0x01 => {
                let v = self.get_word();
                self.registers.setbc(v);
                3
            }
            0x02 => {
                self.memory.wb(self.registers.bc(), self.registers.a);
                2
            }
            0x03 => {
                self.registers.setbc(self.registers.bc().wrapping_add(1));
                2
            }
            0x04 => {
                self.registers.b = self.alu_inc(self.registers.b);
                1
            }
            0x05 => {
                self.registers.b = self.alu_dec(self.registers.b);
                1
            }
            0x06 => {
                self.registers.b = self.get_byte();
                2
            }
            0x07 => {
                self.registers.a = self.alu_rlc(self.registers.a);
                self.registers.flag(Z, false);
                1
            }
            0x08 => {
                let a = self.get_word();
                self.memory.ww(a, self.registers.sp);
                5
            }
            0x09 => {
                self.alu_add16(self.registers.bc());
                2
            }
            0x0A => {
                self.registers.a = self.memory.rb(self.registers.bc());
                2
            }
            0x0B => {
                self.registers.setbc(self.registers.bc().wrapping_sub(1));
                2
            }
            0x0C => {
                self.registers.c = self.alu_inc(self.registers.c);
                1
            }
            0x0D => {
                self.registers.c = self.alu_dec(self.registers.c);
                1
            }
            0x0E => {
                self.registers.c = self.get_byte();
                2
            }
            0x0F => {
                self.registers.a = self.alu_rrc(self.registers.a);
                self.registers.flag(Z, false);
                1
            }
            0x10 => {
                self.memory.switch_speed();
                1
            } // STOP
            0x11 => {
                let v = self.get_word();
                self.registers.setde(v);
                3
            }
            0x12 => {
                self.memory.wb(self.registers.de(), self.registers.a);
                2
            }
            0x13 => {
                self.registers.setde(self.registers.de().wrapping_add(1));
                2
            }
            0x14 => {
                self.registers.d = self.alu_inc(self.registers.d);
                1
            }
            0x15 => {
                self.registers.d = self.alu_dec(self.registers.d);
                1
            }
            0x16 => {
                self.registers.d = self.get_byte();
                2
            }
            0x17 => {
                self.registers.a = self.alu_rl(self.registers.a);
                self.registers.flag(Z, false);
                1
            }
            0x18 => {
                self.cpu_jr();
                3
            }
            0x19 => {
                self.alu_add16(self.registers.de());
                2
            }
            0x1A => {
                self.registers.a = self.memory.rb(self.registers.de());
                2
            }
            0x1B => {
                self.registers.setde(self.registers.de().wrapping_sub(1));
                2
            }
            0x1C => {
                self.registers.e = self.alu_inc(self.registers.e);
                1
            }
            0x1D => {
                self.registers.e = self.alu_dec(self.registers.e);
                1
            }
            0x1E => {
                self.registers.e = self.get_byte();
                2
            }
            0x1F => {
                self.registers.a = self.alu_rr(self.registers.a);
                self.registers.flag(Z, false);
                1
            }
            0x20 => {
                if !self.registers.getflag(Z) {
                    self.cpu_jr();
                    3
                } else {
                    self.registers.pc += 1;
                    2
                }
            }
            0x21 => {
                let v = self.get_word();
                self.registers.sethl(v);
                3
            }
            0x22 => {
                self.memory.wb(self.registers.hli(), self.registers.a);
                2
            }
            0x23 => {
                let v = self.registers.hl().wrapping_add(1);
                self.registers.sethl(v);
                2
            }
            0x24 => {
                self.registers.h = self.alu_inc(self.registers.h);
                1
            }
            0x25 => {
                self.registers.h = self.alu_dec(self.registers.h);
                1
            }
            0x26 => {
                self.registers.h = self.get_byte();
                2
            }
            0x27 => {
                self.alu_daa();
                1
            }
            0x28 => {
                if self.registers.getflag(Z) {
                    self.cpu_jr();
                    3
                } else {
                    self.registers.pc += 1;
                    2
                }
            }
            0x29 => {
                let v = self.registers.hl();
                self.alu_add16(v);
                2
            }
            0x2A => {
                self.registers.a = self.memory.rb(self.registers.hli());
                2
            }
            0x2B => {
                let v = self.registers.hl().wrapping_sub(1);
                self.registers.sethl(v);
                2
            }
            0x2C => {
                self.registers.l = self.alu_inc(self.registers.l);
                1
            }
            0x2D => {
                self.registers.l = self.alu_dec(self.registers.l);
                1
            }
            0x2E => {
                self.registers.l = self.get_byte();
                2
            }
            0x2F => {
                self.registers.a = !self.registers.a;
                self.registers.flag(H, true);
                self.registers.flag(N, true);
                1
            }
            0x30 => {
                if !self.registers.getflag(C) {
                    self.cpu_jr();
                    3
                } else {
                    self.registers.pc += 1;
                    2
                }
            }
            0x31 => {
                self.registers.sp = self.get_word();
                3
            }
            0x32 => {
                self.memory.wb(self.registers.hld(), self.registers.a);
                2
            }
            0x33 => {
                self.registers.sp = self.registers.sp.wrapping_add(1);
                2
            }
            0x34 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_inc(v);
                self.memory.wb(a, v2);
                3
            }
            0x35 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_dec(v);
                self.memory.wb(a, v2);
                3
            }
            0x36 => {
                let v = self.get_byte();
                self.memory.wb(self.registers.hl(), v);
                3
            }
            0x37 => {
                self.registers.flag(C, true);
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                1
            }
            0x38 => {
                if self.registers.getflag(C) {
                    self.cpu_jr();
                    3
                } else {
                    self.registers.pc += 1;
                    2
                }
            }
            0x39 => {
                self.alu_add16(self.registers.sp);
                2
            }
            0x3A => {
                self.registers.a = self.memory.rb(self.registers.hld());
                2
            }
            0x3B => {
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                2
            }
            0x3C => {
                self.registers.a = self.alu_inc(self.registers.a);
                1
            }
            0x3D => {
                self.registers.a = self.alu_dec(self.registers.a);
                1
            }
            0x3E => {
                self.registers.a = self.get_byte();
                2
            }
            0x3F => {
                let v = !self.registers.getflag(C);
                self.registers.flag(C, v);
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                1
            }
            0x40 => 1,
            0x41 => {
                self.registers.b = self.registers.c;
                1
            }
            0x42 => {
                self.registers.b = self.registers.d;
                1
            }
            0x43 => {
                self.registers.b = self.registers.e;
                1
            }
            0x44 => {
                self.registers.b = self.registers.h;
                1
            }
            0x45 => {
                self.registers.b = self.registers.l;
                1
            }
            0x46 => {
                self.registers.b = self.memory.rb(self.registers.hl());
                2
            }
            0x47 => {
                self.registers.b = self.registers.a;
                1
            }
            0x48 => {
                self.registers.c = self.registers.b;
                1
            }
            0x49 => 1,
            0x4A => {
                self.registers.c = self.registers.d;
                1
            }
            0x4B => {
                self.registers.c = self.registers.e;
                1
            }
            0x4C => {
                self.registers.c = self.registers.h;
                1
            }
            0x4D => {
                self.registers.c = self.registers.l;
                1
            }
            0x4E => {
                self.registers.c = self.memory.rb(self.registers.hl());
                2
            }
            0x4F => {
                self.registers.c = self.registers.a;
                1
            }
            0x50 => {
                self.registers.d = self.registers.b;
                1
            }
            0x51 => {
                self.registers.d = self.registers.c;
                1
            }
            0x52 => 1,
            0x53 => {
                self.registers.d = self.registers.e;
                1
            }
            0x54 => {
                self.registers.d = self.registers.h;
                1
            }
            0x55 => {
                self.registers.d = self.registers.l;
                1
            }
            0x56 => {
                self.registers.d = self.memory.rb(self.registers.hl());
                2
            }
            0x57 => {
                self.registers.d = self.registers.a;
                1
            }
            0x58 => {
                self.registers.e = self.registers.b;
                1
            }
            0x59 => {
                self.registers.e = self.registers.c;
                1
            }
            0x5A => {
                self.registers.e = self.registers.d;
                1
            }
            0x5B => 1,
            0x5C => {
                self.registers.e = self.registers.h;
                1
            }
            0x5D => {
                self.registers.e = self.registers.l;
                1
            }
            0x5E => {
                self.registers.e = self.memory.rb(self.registers.hl());
                2
            }
            0x5F => {
                self.registers.e = self.registers.a;
                1
            }
            0x60 => {
                self.registers.h = self.registers.b;
                1
            }
            0x61 => {
                self.registers.h = self.registers.c;
                1
            }
            0x62 => {
                self.registers.h = self.registers.d;
                1
            }
            0x63 => {
                self.registers.h = self.registers.e;
                1
            }
            0x64 => 1,
            0x65 => {
                self.registers.h = self.registers.l;
                1
            }
            0x66 => {
                self.registers.h = self.memory.rb(self.registers.hl());
                2
            }
            0x67 => {
                self.registers.h = self.registers.a;
                1
            }
            0x68 => {
                self.registers.l = self.registers.b;
                1
            }
            0x69 => {
                self.registers.l = self.registers.c;
                1
            }
            0x6A => {
                self.registers.l = self.registers.d;
                1
            }
            0x6B => {
                self.registers.l = self.registers.e;
                1
            }
            0x6C => {
                self.registers.l = self.registers.h;
                1
            }
            0x6D => 1,
            0x6E => {
                self.registers.l = self.memory.rb(self.registers.hl());
                2
            }
            0x6F => {
                self.registers.l = self.registers.a;
                1
            }
            0x70 => {
                self.memory.wb(self.registers.hl(), self.registers.b);
                2
            }
            0x71 => {
                self.memory.wb(self.registers.hl(), self.registers.c);
                2
            }
            0x72 => {
                self.memory.wb(self.registers.hl(), self.registers.d);
                2
            }
            0x73 => {
                self.memory.wb(self.registers.hl(), self.registers.e);
                2
            }
            0x74 => {
                self.memory.wb(self.registers.hl(), self.registers.h);
                2
            }
            0x75 => {
                self.memory.wb(self.registers.hl(), self.registers.l);
                2
            }
            0x76 => {
                self.halted = true;
                1
            }
            0x77 => {
                self.memory.wb(self.registers.hl(), self.registers.a);
                2
            }
            0x78 => {
                self.registers.a = self.registers.b;
                1
            }
            0x79 => {
                self.registers.a = self.registers.c;
                1
            }
            0x7A => {
                self.registers.a = self.registers.d;
                1
            }
            0x7B => {
                self.registers.a = self.registers.e;
                1
            }
            0x7C => {
                self.registers.a = self.registers.h;
                1
            }
            0x7D => {
                self.registers.a = self.registers.l;
                1
            }
            0x7E => {
                self.registers.a = self.memory.rb(self.registers.hl());
                2
            }
            0x7F => 1,
            0x80 => {
                self.alu_add(self.registers.b, false);
                1
            }
            0x81 => {
                self.alu_add(self.registers.c, false);
                1
            }
            0x82 => {
                self.alu_add(self.registers.d, false);
                1
            }
            0x83 => {
                self.alu_add(self.registers.e, false);
                1
            }
            0x84 => {
                self.alu_add(self.registers.h, false);
                1
            }
            0x85 => {
                self.alu_add(self.registers.l, false);
                1
            }
            0x86 => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_add(v, false);
                2
            }
            0x87 => {
                self.alu_add(self.registers.a, false);
                1
            }
            0x88 => {
                self.alu_add(self.registers.b, true);
                1
            }
            0x89 => {
                self.alu_add(self.registers.c, true);
                1
            }
            0x8A => {
                self.alu_add(self.registers.d, true);
                1
            }
            0x8B => {
                self.alu_add(self.registers.e, true);
                1
            }
            0x8C => {
                self.alu_add(self.registers.h, true);
                1
            }
            0x8D => {
                self.alu_add(self.registers.l, true);
                1
            }
            0x8E => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_add(v, true);
                2
            }
            0x8F => {
                self.alu_add(self.registers.a, true);
                1
            }
            0x90 => {
                self.alu_sub(self.registers.b, false);
                1
            }
            0x91 => {
                self.alu_sub(self.registers.c, false);
                1
            }
            0x92 => {
                self.alu_sub(self.registers.d, false);
                1
            }
            0x93 => {
                self.alu_sub(self.registers.e, false);
                1
            }
            0x94 => {
                self.alu_sub(self.registers.h, false);
                1
            }
            0x95 => {
                self.alu_sub(self.registers.l, false);
                1
            }
            0x96 => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_sub(v, false);
                2
            }
            0x97 => {
                self.alu_sub(self.registers.a, false);
                1
            }
            0x98 => {
                self.alu_sub(self.registers.b, true);
                1
            }
            0x99 => {
                self.alu_sub(self.registers.c, true);
                1
            }
            0x9A => {
                self.alu_sub(self.registers.d, true);
                1
            }
            0x9B => {
                self.alu_sub(self.registers.e, true);
                1
            }
            0x9C => {
                self.alu_sub(self.registers.h, true);
                1
            }
            0x9D => {
                self.alu_sub(self.registers.l, true);
                1
            }
            0x9E => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_sub(v, true);
                2
            }
            0x9F => {
                self.alu_sub(self.registers.a, true);
                1
            }
            0xA0 => {
                self.alu_and(self.registers.b);
                1
            }
            0xA1 => {
                self.alu_and(self.registers.c);
                1
            }
            0xA2 => {
                self.alu_and(self.registers.d);
                1
            }
            0xA3 => {
                self.alu_and(self.registers.e);
                1
            }
            0xA4 => {
                self.alu_and(self.registers.h);
                1
            }
            0xA5 => {
                self.alu_and(self.registers.l);
                1
            }
            0xA6 => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_and(v);
                2
            }
            0xA7 => {
                self.alu_and(self.registers.a);
                1
            }
            0xA8 => {
                self.alu_xor(self.registers.b);
                1
            }
            0xA9 => {
                self.alu_xor(self.registers.c);
                1
            }
            0xAA => {
                self.alu_xor(self.registers.d);
                1
            }
            0xAB => {
                self.alu_xor(self.registers.e);
                1
            }
            0xAC => {
                self.alu_xor(self.registers.h);
                1
            }
            0xAD => {
                self.alu_xor(self.registers.l);
                1
            }
            0xAE => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_xor(v);
                2
            }
            0xAF => {
                self.alu_xor(self.registers.a);
                1
            }
            0xB0 => {
                self.alu_or(self.registers.b);
                1
            }
            0xB1 => {
                self.alu_or(self.registers.c);
                1
            }
            0xB2 => {
                self.alu_or(self.registers.d);
                1
            }
            0xB3 => {
                self.alu_or(self.registers.e);
                1
            }
            0xB4 => {
                self.alu_or(self.registers.h);
                1
            }
            0xB5 => {
                self.alu_or(self.registers.l);
                1
            }
            0xB6 => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_or(v);
                2
            }
            0xB7 => {
                self.alu_or(self.registers.a);
                1
            }
            0xB8 => {
                self.alu_cp(self.registers.b);
                1
            }
            0xB9 => {
                self.alu_cp(self.registers.c);
                1
            }
            0xBA => {
                self.alu_cp(self.registers.d);
                1
            }
            0xBB => {
                self.alu_cp(self.registers.e);
                1
            }
            0xBC => {
                self.alu_cp(self.registers.h);
                1
            }
            0xBD => {
                self.alu_cp(self.registers.l);
                1
            }
            0xBE => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_cp(v);
                2
            }
            0xBF => {
                self.alu_cp(self.registers.a);
                1
            }
            0xC0 => {
                if !self.registers.getflag(Z) {
                    self.registers.pc = self.popstack();
                    5
                } else {
                    2
                }
            }
            0xC1 => {
                let v = self.popstack();
                self.registers.setbc(v);
                3
            }
            0xC2 => {
                if !self.registers.getflag(Z) {
                    self.registers.pc = self.get_word();
                    4
                } else {
                    self.registers.pc += 2;
                    3
                }
            }
            0xC3 => {
                self.registers.pc = self.get_word();
                4
            }
            0xC4 => {
                if !self.registers.getflag(Z) {
                    self.pushstack(self.registers.pc + 2);
                    self.registers.pc = self.get_word();
                    6
                } else {
                    self.registers.pc += 2;
                    3
                }
            }
            0xC5 => {
                self.pushstack(self.registers.bc());
                4
            }
            0xC6 => {
                let v = self.get_byte();
                self.alu_add(v, false);
                2
            }
            0xC7 => {
                self.pushstack(self.registers.pc);
                self.registers.pc = 0x00;
                4
            }
            0xC8 => {
                if self.registers.getflag(Z) {
                    self.registers.pc = self.popstack();
                    5
                } else {
                    2
                }
            }
            0xC9 => {
                self.registers.pc = self.popstack();
                4
            }
            0xCA => {
                if self.registers.getflag(Z) {
                    self.registers.pc = self.get_word();
                    4
                } else {
                    self.registers.pc += 2;
                    3
                }
            }
            0xCB => self.call_cb(),
            0xCC => {
                if self.registers.getflag(Z) {
                    self.pushstack(self.registers.pc + 2);
                    self.registers.pc = self.get_word();
                    6
                } else {
                    self.registers.pc += 2;
                    3
                }
            }
            0xCD => {
                self.pushstack(self.registers.pc + 2);
                self.registers.pc = self.get_word();
                6
            }
            0xCE => {
                let v = self.get_byte();
                self.alu_add(v, true);
                2
            }
            0xCF => {
                self.pushstack(self.registers.pc);
                self.registers.pc = 0x08;
                4
            }
            0xD0 => {
                if !self.registers.getflag(C) {
                    self.registers.pc = self.popstack();
                    5
                } else {
                    2
                }
            }
            0xD1 => {
                let v = self.popstack();
                self.registers.setde(v);
                3
            }
            0xD2 => {
                if !self.registers.getflag(C) {
                    self.registers.pc = self.get_word();
                    4
                } else {
                    self.registers.pc += 2;
                    3
                }
            }
            0xD4 => {
                if !self.registers.getflag(C) {
                    self.pushstack(self.registers.pc + 2);
                    self.registers.pc = self.get_word();
                    6
                } else {
                    self.registers.pc += 2;
                    3
                }
            }
            0xD5 => {
                self.pushstack(self.registers.de());
                4
            }
            0xD6 => {
                let v = self.get_byte();
                self.alu_sub(v, false);
                2
            }
            0xD7 => {
                self.pushstack(self.registers.pc);
                self.registers.pc = 0x10;
                4
            }
            0xD8 => {
                if self.registers.getflag(C) {
                    self.registers.pc = self.popstack();
                    5
                } else {
                    2
                }
            }
            0xD9 => {
                self.registers.pc = self.popstack();
                self.setei = 1;
                4
            }
            0xDA => {
                if self.registers.getflag(C) {
                    self.registers.pc = self.get_word();
                    4
                } else {
                    self.registers.pc += 2;
                    3
                }
            }
            0xDC => {
                if self.registers.getflag(C) {
                    self.pushstack(self.registers.pc + 2);
                    self.registers.pc = self.get_word();
                    6
                } else {
                    self.registers.pc += 2;
                    3
                }
            }
            0xDE => {
                let v = self.get_byte();
                self.alu_sub(v, true);
                2
            }
            0xDF => {
                self.pushstack(self.registers.pc);
                self.registers.pc = 0x18;
                4
            }
            0xE0 => {
                let a = 0xFF00 | self.get_byte() as u16;
                self.memory.wb(a, self.registers.a);
                3
            }
            0xE1 => {
                let v = self.popstack();
                self.registers.sethl(v);
                3
            }
            0xE2 => {
                self.memory
                    .wb(0xFF00 | self.registers.c as u16, self.registers.a);
                2
            }
            0xE5 => {
                self.pushstack(self.registers.hl());
                4
            }
            0xE6 => {
                let v = self.get_byte();
                self.alu_and(v);
                2
            }
            0xE7 => {
                self.pushstack(self.registers.pc);
                self.registers.pc = 0x20;
                4
            }
            0xE8 => {
                self.registers.sp = self.alu_add16imm(self.registers.sp);
                4
            }
            0xE9 => {
                self.registers.pc = self.registers.hl();
                1
            }
            0xEA => {
                let a = self.get_word();
                self.memory.wb(a, self.registers.a);
                4
            }
            0xEE => {
                let v = self.get_byte();
                self.alu_xor(v);
                2
            }
            0xEF => {
                self.pushstack(self.registers.pc);
                self.registers.pc = 0x28;
                4
            }
            0xF0 => {
                let a = 0xFF00 | self.get_byte() as u16;
                self.registers.a = self.memory.rb(a);
                3
            }
            0xF1 => {
                let v = self.popstack() & 0xFFF0;
                self.registers.setaf(v);
                3
            }
            0xF2 => {
                self.registers.a = self.memory.rb(0xFF00 | self.registers.c as u16);
                2
            }
            0xF3 => {
                self.setdi = 2;
                1
            }
            0xF5 => {
                self.pushstack(self.registers.af());
                4
            }
            0xF6 => {
                let v = self.get_byte();
                self.alu_or(v);
                2
            }
            0xF7 => {
                self.pushstack(self.registers.pc);
                self.registers.pc = 0x30;
                4
            }
            0xF8 => {
                let r = self.alu_add16imm(self.registers.sp);
                self.registers.sethl(r);
                3
            }
            0xF9 => {
                self.registers.sp = self.registers.hl();
                2
            }
            0xFA => {
                let a = self.get_word();
                self.registers.a = self.memory.rb(a);
                4
            }
            0xFB => {
                self.setei = 2;
                1
            }
            0xFE => {
                let v = self.get_byte();
                self.alu_cp(v);
                2
            }
            0xFF => {
                self.pushstack(self.registers.pc);
                self.registers.pc = 0x38;
                4
            }
            other => panic!("Instruction {:2X} is not implemented", other),
        }
    }

    fn call_cb(&mut self) -> u32 {
        let opcode = self.get_byte();
        match opcode {
            0x00 => {
                self.registers.b = self.alu_rlc(self.registers.b);
                2
            }
            0x01 => {
                self.registers.c = self.alu_rlc(self.registers.c);
                2
            }
            0x02 => {
                self.registers.d = self.alu_rlc(self.registers.d);
                2
            }
            0x03 => {
                self.registers.e = self.alu_rlc(self.registers.e);
                2
            }
            0x04 => {
                self.registers.h = self.alu_rlc(self.registers.h);
                2
            }
            0x05 => {
                self.registers.l = self.alu_rlc(self.registers.l);
                2
            }
            0x06 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_rlc(v);
                self.memory.wb(a, v2);
                4
            }
            0x07 => {
                self.registers.a = self.alu_rlc(self.registers.a);
                2
            }
            0x08 => {
                self.registers.b = self.alu_rrc(self.registers.b);
                2
            }
            0x09 => {
                self.registers.c = self.alu_rrc(self.registers.c);
                2
            }
            0x0A => {
                self.registers.d = self.alu_rrc(self.registers.d);
                2
            }
            0x0B => {
                self.registers.e = self.alu_rrc(self.registers.e);
                2
            }
            0x0C => {
                self.registers.h = self.alu_rrc(self.registers.h);
                2
            }
            0x0D => {
                self.registers.l = self.alu_rrc(self.registers.l);
                2
            }
            0x0E => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_rrc(v);
                self.memory.wb(a, v2);
                4
            }
            0x0F => {
                self.registers.a = self.alu_rrc(self.registers.a);
                2
            }
            0x10 => {
                self.registers.b = self.alu_rl(self.registers.b);
                2
            }
            0x11 => {
                self.registers.c = self.alu_rl(self.registers.c);
                2
            }
            0x12 => {
                self.registers.d = self.alu_rl(self.registers.d);
                2
            }
            0x13 => {
                self.registers.e = self.alu_rl(self.registers.e);
                2
            }
            0x14 => {
                self.registers.h = self.alu_rl(self.registers.h);
                2
            }
            0x15 => {
                self.registers.l = self.alu_rl(self.registers.l);
                2
            }
            0x16 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_rl(v);
                self.memory.wb(a, v2);
                4
            }
            0x17 => {
                self.registers.a = self.alu_rl(self.registers.a);
                2
            }
            0x18 => {
                self.registers.b = self.alu_rr(self.registers.b);
                2
            }
            0x19 => {
                self.registers.c = self.alu_rr(self.registers.c);
                2
            }
            0x1A => {
                self.registers.d = self.alu_rr(self.registers.d);
                2
            }
            0x1B => {
                self.registers.e = self.alu_rr(self.registers.e);
                2
            }
            0x1C => {
                self.registers.h = self.alu_rr(self.registers.h);
                2
            }
            0x1D => {
                self.registers.l = self.alu_rr(self.registers.l);
                2
            }
            0x1E => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_rr(v);
                self.memory.wb(a, v2);
                4
            }
            0x1F => {
                self.registers.a = self.alu_rr(self.registers.a);
                2
            }
            0x20 => {
                self.registers.b = self.alu_sla(self.registers.b);
                2
            }
            0x21 => {
                self.registers.c = self.alu_sla(self.registers.c);
                2
            }
            0x22 => {
                self.registers.d = self.alu_sla(self.registers.d);
                2
            }
            0x23 => {
                self.registers.e = self.alu_sla(self.registers.e);
                2
            }
            0x24 => {
                self.registers.h = self.alu_sla(self.registers.h);
                2
            }
            0x25 => {
                self.registers.l = self.alu_sla(self.registers.l);
                2
            }
            0x26 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_sla(v);
                self.memory.wb(a, v2);
                4
            }
            0x27 => {
                self.registers.a = self.alu_sla(self.registers.a);
                2
            }
            0x28 => {
                self.registers.b = self.alu_sra(self.registers.b);
                2
            }
            0x29 => {
                self.registers.c = self.alu_sra(self.registers.c);
                2
            }
            0x2A => {
                self.registers.d = self.alu_sra(self.registers.d);
                2
            }
            0x2B => {
                self.registers.e = self.alu_sra(self.registers.e);
                2
            }
            0x2C => {
                self.registers.h = self.alu_sra(self.registers.h);
                2
            }
            0x2D => {
                self.registers.l = self.alu_sra(self.registers.l);
                2
            }
            0x2E => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_sra(v);
                self.memory.wb(a, v2);
                4
            }
            0x2F => {
                self.registers.a = self.alu_sra(self.registers.a);
                2
            }
            0x30 => {
                self.registers.b = self.alu_swap(self.registers.b);
                2
            }
            0x31 => {
                self.registers.c = self.alu_swap(self.registers.c);
                2
            }
            0x32 => {
                self.registers.d = self.alu_swap(self.registers.d);
                2
            }
            0x33 => {
                self.registers.e = self.alu_swap(self.registers.e);
                2
            }
            0x34 => {
                self.registers.h = self.alu_swap(self.registers.h);
                2
            }
            0x35 => {
                self.registers.l = self.alu_swap(self.registers.l);
                2
            }
            0x36 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_swap(v);
                self.memory.wb(a, v2);
                4
            }
            0x37 => {
                self.registers.a = self.alu_swap(self.registers.a);
                2
            }
            0x38 => {
                self.registers.b = self.alu_srl(self.registers.b);
                2
            }
            0x39 => {
                self.registers.c = self.alu_srl(self.registers.c);
                2
            }
            0x3A => {
                self.registers.d = self.alu_srl(self.registers.d);
                2
            }
            0x3B => {
                self.registers.e = self.alu_srl(self.registers.e);
                2
            }
            0x3C => {
                self.registers.h = self.alu_srl(self.registers.h);
                2
            }
            0x3D => {
                self.registers.l = self.alu_srl(self.registers.l);
                2
            }
            0x3E => {
                let a = self.registers.hl();
                let v = self.memory.rb(a);
                let v2 = self.alu_srl(v);
                self.memory.wb(a, v2);
                4
            }
            0x3F => {
                self.registers.a = self.alu_srl(self.registers.a);
                2
            }
            0x40 => {
                self.alu_bit(self.registers.b, 0);
                2
            }
            0x41 => {
                self.alu_bit(self.registers.c, 0);
                2
            }
            0x42 => {
                self.alu_bit(self.registers.d, 0);
                2
            }
            0x43 => {
                self.alu_bit(self.registers.e, 0);
                2
            }
            0x44 => {
                self.alu_bit(self.registers.h, 0);
                2
            }
            0x45 => {
                self.alu_bit(self.registers.l, 0);
                2
            }
            0x46 => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_bit(v, 0);
                3
            }
            0x47 => {
                self.alu_bit(self.registers.a, 0);
                2
            }
            0x48 => {
                self.alu_bit(self.registers.b, 1);
                2
            }
            0x49 => {
                self.alu_bit(self.registers.c, 1);
                2
            }
            0x4A => {
                self.alu_bit(self.registers.d, 1);
                2
            }
            0x4B => {
                self.alu_bit(self.registers.e, 1);
                2
            }
            0x4C => {
                self.alu_bit(self.registers.h, 1);
                2
            }
            0x4D => {
                self.alu_bit(self.registers.l, 1);
                2
            }
            0x4E => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_bit(v, 1);
                3
            }
            0x4F => {
                self.alu_bit(self.registers.a, 1);
                2
            }
            0x50 => {
                self.alu_bit(self.registers.b, 2);
                2
            }
            0x51 => {
                self.alu_bit(self.registers.c, 2);
                2
            }
            0x52 => {
                self.alu_bit(self.registers.d, 2);
                2
            }
            0x53 => {
                self.alu_bit(self.registers.e, 2);
                2
            }
            0x54 => {
                self.alu_bit(self.registers.h, 2);
                2
            }
            0x55 => {
                self.alu_bit(self.registers.l, 2);
                2
            }
            0x56 => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_bit(v, 2);
                3
            }
            0x57 => {
                self.alu_bit(self.registers.a, 2);
                2
            }
            0x58 => {
                self.alu_bit(self.registers.b, 3);
                2
            }
            0x59 => {
                self.alu_bit(self.registers.c, 3);
                2
            }
            0x5A => {
                self.alu_bit(self.registers.d, 3);
                2
            }
            0x5B => {
                self.alu_bit(self.registers.e, 3);
                2
            }
            0x5C => {
                self.alu_bit(self.registers.h, 3);
                2
            }
            0x5D => {
                self.alu_bit(self.registers.l, 3);
                2
            }
            0x5E => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_bit(v, 3);
                3
            }
            0x5F => {
                self.alu_bit(self.registers.a, 3);
                2
            }
            0x60 => {
                self.alu_bit(self.registers.b, 4);
                2
            }
            0x61 => {
                self.alu_bit(self.registers.c, 4);
                2
            }
            0x62 => {
                self.alu_bit(self.registers.d, 4);
                2
            }
            0x63 => {
                self.alu_bit(self.registers.e, 4);
                2
            }
            0x64 => {
                self.alu_bit(self.registers.h, 4);
                2
            }
            0x65 => {
                self.alu_bit(self.registers.l, 4);
                2
            }
            0x66 => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_bit(v, 4);
                3
            }
            0x67 => {
                self.alu_bit(self.registers.a, 4);
                2
            }
            0x68 => {
                self.alu_bit(self.registers.b, 5);
                2
            }
            0x69 => {
                self.alu_bit(self.registers.c, 5);
                2
            }
            0x6A => {
                self.alu_bit(self.registers.d, 5);
                2
            }
            0x6B => {
                self.alu_bit(self.registers.e, 5);
                2
            }
            0x6C => {
                self.alu_bit(self.registers.h, 5);
                2
            }
            0x6D => {
                self.alu_bit(self.registers.l, 5);
                2
            }
            0x6E => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_bit(v, 5);
                3
            }
            0x6F => {
                self.alu_bit(self.registers.a, 5);
                2
            }
            0x70 => {
                self.alu_bit(self.registers.b, 6);
                2
            }
            0x71 => {
                self.alu_bit(self.registers.c, 6);
                2
            }
            0x72 => {
                self.alu_bit(self.registers.d, 6);
                2
            }
            0x73 => {
                self.alu_bit(self.registers.e, 6);
                2
            }
            0x74 => {
                self.alu_bit(self.registers.h, 6);
                2
            }
            0x75 => {
                self.alu_bit(self.registers.l, 6);
                2
            }
            0x76 => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_bit(v, 6);
                3
            }
            0x77 => {
                self.alu_bit(self.registers.a, 6);
                2
            }
            0x78 => {
                self.alu_bit(self.registers.b, 7);
                2
            }
            0x79 => {
                self.alu_bit(self.registers.c, 7);
                2
            }
            0x7A => {
                self.alu_bit(self.registers.d, 7);
                2
            }
            0x7B => {
                self.alu_bit(self.registers.e, 7);
                2
            }
            0x7C => {
                self.alu_bit(self.registers.h, 7);
                2
            }
            0x7D => {
                self.alu_bit(self.registers.l, 7);
                2
            }
            0x7E => {
                let v = self.memory.rb(self.registers.hl());
                self.alu_bit(v, 7);
                3
            }
            0x7F => {
                self.alu_bit(self.registers.a, 7);
                2
            }
            0x80 => {
                self.registers.b = self.registers.b & !(1 << 0);
                2
            }
            0x81 => {
                self.registers.c = self.registers.c & !(1 << 0);
                2
            }
            0x82 => {
                self.registers.d = self.registers.d & !(1 << 0);
                2
            }
            0x83 => {
                self.registers.e = self.registers.e & !(1 << 0);
                2
            }
            0x84 => {
                self.registers.h = self.registers.h & !(1 << 0);
                2
            }
            0x85 => {
                self.registers.l = self.registers.l & !(1 << 0);
                2
            }
            0x86 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) & !(1 << 0);
                self.memory.wb(a, v);
                4
            }
            0x87 => {
                self.registers.a = self.registers.a & !(1 << 0);
                2
            }
            0x88 => {
                self.registers.b = self.registers.b & !(1 << 1);
                2
            }
            0x89 => {
                self.registers.c = self.registers.c & !(1 << 1);
                2
            }
            0x8A => {
                self.registers.d = self.registers.d & !(1 << 1);
                2
            }
            0x8B => {
                self.registers.e = self.registers.e & !(1 << 1);
                2
            }
            0x8C => {
                self.registers.h = self.registers.h & !(1 << 1);
                2
            }
            0x8D => {
                self.registers.l = self.registers.l & !(1 << 1);
                2
            }
            0x8E => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) & !(1 << 1);
                self.memory.wb(a, v);
                4
            }
            0x8F => {
                self.registers.a = self.registers.a & !(1 << 1);
                2
            }
            0x90 => {
                self.registers.b = self.registers.b & !(1 << 2);
                2
            }
            0x91 => {
                self.registers.c = self.registers.c & !(1 << 2);
                2
            }
            0x92 => {
                self.registers.d = self.registers.d & !(1 << 2);
                2
            }
            0x93 => {
                self.registers.e = self.registers.e & !(1 << 2);
                2
            }
            0x94 => {
                self.registers.h = self.registers.h & !(1 << 2);
                2
            }
            0x95 => {
                self.registers.l = self.registers.l & !(1 << 2);
                2
            }
            0x96 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) & !(1 << 2);
                self.memory.wb(a, v);
                4
            }
            0x97 => {
                self.registers.a = self.registers.a & !(1 << 2);
                2
            }
            0x98 => {
                self.registers.b = self.registers.b & !(1 << 3);
                2
            }
            0x99 => {
                self.registers.c = self.registers.c & !(1 << 3);
                2
            }
            0x9A => {
                self.registers.d = self.registers.d & !(1 << 3);
                2
            }
            0x9B => {
                self.registers.e = self.registers.e & !(1 << 3);
                2
            }
            0x9C => {
                self.registers.h = self.registers.h & !(1 << 3);
                2
            }
            0x9D => {
                self.registers.l = self.registers.l & !(1 << 3);
                2
            }
            0x9E => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) & !(1 << 3);
                self.memory.wb(a, v);
                4
            }
            0x9F => {
                self.registers.a = self.registers.a & !(1 << 3);
                2
            }
            0xA0 => {
                self.registers.b = self.registers.b & !(1 << 4);
                2
            }
            0xA1 => {
                self.registers.c = self.registers.c & !(1 << 4);
                2
            }
            0xA2 => {
                self.registers.d = self.registers.d & !(1 << 4);
                2
            }
            0xA3 => {
                self.registers.e = self.registers.e & !(1 << 4);
                2
            }
            0xA4 => {
                self.registers.h = self.registers.h & !(1 << 4);
                2
            }
            0xA5 => {
                self.registers.l = self.registers.l & !(1 << 4);
                2
            }
            0xA6 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) & !(1 << 4);
                self.memory.wb(a, v);
                4
            }
            0xA7 => {
                self.registers.a = self.registers.a & !(1 << 4);
                2
            }
            0xA8 => {
                self.registers.b = self.registers.b & !(1 << 5);
                2
            }
            0xA9 => {
                self.registers.c = self.registers.c & !(1 << 5);
                2
            }
            0xAA => {
                self.registers.d = self.registers.d & !(1 << 5);
                2
            }
            0xAB => {
                self.registers.e = self.registers.e & !(1 << 5);
                2
            }
            0xAC => {
                self.registers.h = self.registers.h & !(1 << 5);
                2
            }
            0xAD => {
                self.registers.l = self.registers.l & !(1 << 5);
                2
            }
            0xAE => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) & !(1 << 5);
                self.memory.wb(a, v);
                4
            }
            0xAF => {
                self.registers.a = self.registers.a & !(1 << 5);
                2
            }
            0xB0 => {
                self.registers.b = self.registers.b & !(1 << 6);
                2
            }
            0xB1 => {
                self.registers.c = self.registers.c & !(1 << 6);
                2
            }
            0xB2 => {
                self.registers.d = self.registers.d & !(1 << 6);
                2
            }
            0xB3 => {
                self.registers.e = self.registers.e & !(1 << 6);
                2
            }
            0xB4 => {
                self.registers.h = self.registers.h & !(1 << 6);
                2
            }
            0xB5 => {
                self.registers.l = self.registers.l & !(1 << 6);
                2
            }
            0xB6 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) & !(1 << 6);
                self.memory.wb(a, v);
                4
            }
            0xB7 => {
                self.registers.a = self.registers.a & !(1 << 6);
                2
            }
            0xB8 => {
                self.registers.b = self.registers.b & !(1 << 7);
                2
            }
            0xB9 => {
                self.registers.c = self.registers.c & !(1 << 7);
                2
            }
            0xBA => {
                self.registers.d = self.registers.d & !(1 << 7);
                2
            }
            0xBB => {
                self.registers.e = self.registers.e & !(1 << 7);
                2
            }
            0xBC => {
                self.registers.h = self.registers.h & !(1 << 7);
                2
            }
            0xBD => {
                self.registers.l = self.registers.l & !(1 << 7);
                2
            }
            0xBE => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) & !(1 << 7);
                self.memory.wb(a, v);
                4
            }
            0xBF => {
                self.registers.a = self.registers.a & !(1 << 7);
                2
            }
            0xC0 => {
                self.registers.b = self.registers.b | (1 << 0);
                2
            }
            0xC1 => {
                self.registers.c = self.registers.c | (1 << 0);
                2
            }
            0xC2 => {
                self.registers.d = self.registers.d | (1 << 0);
                2
            }
            0xC3 => {
                self.registers.e = self.registers.e | (1 << 0);
                2
            }
            0xC4 => {
                self.registers.h = self.registers.h | (1 << 0);
                2
            }
            0xC5 => {
                self.registers.l = self.registers.l | (1 << 0);
                2
            }
            0xC6 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) | (1 << 0);
                self.memory.wb(a, v);
                4
            }
            0xC7 => {
                self.registers.a = self.registers.a | (1 << 0);
                2
            }
            0xC8 => {
                self.registers.b = self.registers.b | (1 << 1);
                2
            }
            0xC9 => {
                self.registers.c = self.registers.c | (1 << 1);
                2
            }
            0xCA => {
                self.registers.d = self.registers.d | (1 << 1);
                2
            }
            0xCB => {
                self.registers.e = self.registers.e | (1 << 1);
                2
            }
            0xCC => {
                self.registers.h = self.registers.h | (1 << 1);
                2
            }
            0xCD => {
                self.registers.l = self.registers.l | (1 << 1);
                2
            }
            0xCE => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) | (1 << 1);
                self.memory.wb(a, v);
                4
            }
            0xCF => {
                self.registers.a = self.registers.a | (1 << 1);
                2
            }
            0xD0 => {
                self.registers.b = self.registers.b | (1 << 2);
                2
            }
            0xD1 => {
                self.registers.c = self.registers.c | (1 << 2);
                2
            }
            0xD2 => {
                self.registers.d = self.registers.d | (1 << 2);
                2
            }
            0xD3 => {
                self.registers.e = self.registers.e | (1 << 2);
                2
            }
            0xD4 => {
                self.registers.h = self.registers.h | (1 << 2);
                2
            }
            0xD5 => {
                self.registers.l = self.registers.l | (1 << 2);
                2
            }
            0xD6 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) | (1 << 2);
                self.memory.wb(a, v);
                4
            }
            0xD7 => {
                self.registers.a = self.registers.a | (1 << 2);
                2
            }
            0xD8 => {
                self.registers.b = self.registers.b | (1 << 3);
                2
            }
            0xD9 => {
                self.registers.c = self.registers.c | (1 << 3);
                2
            }
            0xDA => {
                self.registers.d = self.registers.d | (1 << 3);
                2
            }
            0xDB => {
                self.registers.e = self.registers.e | (1 << 3);
                2
            }
            0xDC => {
                self.registers.h = self.registers.h | (1 << 3);
                2
            }
            0xDD => {
                self.registers.l = self.registers.l | (1 << 3);
                2
            }
            0xDE => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) | (1 << 3);
                self.memory.wb(a, v);
                4
            }
            0xDF => {
                self.registers.a = self.registers.a | (1 << 3);
                2
            }
            0xE0 => {
                self.registers.b = self.registers.b | (1 << 4);
                2
            }
            0xE1 => {
                self.registers.c = self.registers.c | (1 << 4);
                2
            }
            0xE2 => {
                self.registers.d = self.registers.d | (1 << 4);
                2
            }
            0xE3 => {
                self.registers.e = self.registers.e | (1 << 4);
                2
            }
            0xE4 => {
                self.registers.h = self.registers.h | (1 << 4);
                2
            }
            0xE5 => {
                self.registers.l = self.registers.l | (1 << 4);
                2
            }
            0xE6 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) | (1 << 4);
                self.memory.wb(a, v);
                4
            }
            0xE7 => {
                self.registers.a = self.registers.a | (1 << 4);
                2
            }
            0xE8 => {
                self.registers.b = self.registers.b | (1 << 5);
                2
            }
            0xE9 => {
                self.registers.c = self.registers.c | (1 << 5);
                2
            }
            0xEA => {
                self.registers.d = self.registers.d | (1 << 5);
                2
            }
            0xEB => {
                self.registers.e = self.registers.e | (1 << 5);
                2
            }
            0xEC => {
                self.registers.h = self.registers.h | (1 << 5);
                2
            }
            0xED => {
                self.registers.l = self.registers.l | (1 << 5);
                2
            }
            0xEE => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) | (1 << 5);
                self.memory.wb(a, v);
                4
            }
            0xEF => {
                self.registers.a = self.registers.a | (1 << 5);
                2
            }
            0xF0 => {
                self.registers.b = self.registers.b | (1 << 6);
                2
            }
            0xF1 => {
                self.registers.c = self.registers.c | (1 << 6);
                2
            }
            0xF2 => {
                self.registers.d = self.registers.d | (1 << 6);
                2
            }
            0xF3 => {
                self.registers.e = self.registers.e | (1 << 6);
                2
            }
            0xF4 => {
                self.registers.h = self.registers.h | (1 << 6);
                2
            }
            0xF5 => {
                self.registers.l = self.registers.l | (1 << 6);
                2
            }
            0xF6 => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) | (1 << 6);
                self.memory.wb(a, v);
                4
            }
            0xF7 => {
                self.registers.a = self.registers.a | (1 << 6);
                2
            }
            0xF8 => {
                self.registers.b = self.registers.b | (1 << 7);
                2
            }
            0xF9 => {
                self.registers.c = self.registers.c | (1 << 7);
                2
            }
            0xFA => {
                self.registers.d = self.registers.d | (1 << 7);
                2
            }
            0xFB => {
                self.registers.e = self.registers.e | (1 << 7);
                2
            }
            0xFC => {
                self.registers.h = self.registers.h | (1 << 7);
                2
            }
            0xFD => {
                self.registers.l = self.registers.l | (1 << 7);
                2
            }
            0xFE => {
                let a = self.registers.hl();
                let v = self.memory.rb(a) | (1 << 7);
                self.memory.wb(a, v);
                4
            }
            0xFF => {
                self.registers.a = self.registers.a | (1 << 7);
                2
            }
        }
    }

    fn alu_add(&mut self, b: u8, usec: bool) {
        let c = if usec && self.registers.getflag(C) {
            1
        } else {
            0
        };
        let a = self.registers.a;
        let r = a.wrapping_add(b).wrapping_add(c);
        self.registers.flag(Z, r == 0);
        self.registers.flag(H, (a & 0xF) + (b & 0xF) + c > 0xF);
        self.registers.flag(N, false);
        self.registers
            .flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF);
        self.registers.a = r;
    }

    fn alu_sub(&mut self, b: u8, usec: bool) {
        let c = if usec && self.registers.getflag(C) {
            1
        } else {
            0
        };
        let a = self.registers.a;
        let r = a.wrapping_sub(b).wrapping_sub(c);
        self.registers.flag(Z, r == 0);
        self.registers.flag(H, (a & 0x0F) < (b & 0x0F) + c);
        self.registers.flag(N, true);
        self.registers.flag(C, (a as u16) < (b as u16) + (c as u16));
        self.registers.a = r;
    }

    fn alu_and(&mut self, b: u8) {
        let r = self.registers.a & b;
        self.registers.flag(Z, r == 0);
        self.registers.flag(H, true);
        self.registers.flag(C, false);
        self.registers.flag(N, false);
        self.registers.a = r;
    }

    fn alu_or(&mut self, b: u8) {
        let r = self.registers.a | b;
        self.registers.flag(Z, r == 0);
        self.registers.flag(C, false);
        self.registers.flag(H, false);
        self.registers.flag(N, false);
        self.registers.a = r;
    }

    fn alu_xor(&mut self, b: u8) {
        let r = self.registers.a ^ b;
        self.registers.flag(Z, r == 0);
        self.registers.flag(C, false);
        self.registers.flag(H, false);
        self.registers.flag(N, false);
        self.registers.a = r;
    }

    fn alu_cp(&mut self, b: u8) {
        let r = self.registers.a;
        self.alu_sub(b, false);
        self.registers.a = r;
    }

    fn alu_inc(&mut self, a: u8) -> u8 {
        let r = a.wrapping_add(1);
        self.registers.flag(Z, r == 0);
        self.registers.flag(H, (a & 0x0F) + 1 > 0x0F);
        self.registers.flag(N, false);
        return r;
    }

    fn alu_dec(&mut self, a: u8) -> u8 {
        let r = a.wrapping_sub(1);
        self.registers.flag(Z, r == 0);
        self.registers.flag(H, (a & 0x0F) == 0);
        self.registers.flag(N, true);
        return r;
    }

    fn alu_add16(&mut self, b: u16) {
        let a = self.registers.hl();
        let r = a.wrapping_add(b);
        self.registers.flag(H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
        self.registers.flag(N, false);
        self.registers.flag(C, a > 0xFFFF - b);
        self.registers.sethl(r);
    }

    fn alu_add16imm(&mut self, a: u16) -> u16 {
        let b = self.get_byte() as i8 as i16 as u16;
        self.registers.flag(N, false);
        self.registers.flag(Z, false);
        self.registers.flag(H, (a & 0x000F) + (b & 0x000F) > 0x000F);
        self.registers.flag(C, (a & 0x00FF) + (b & 0x00FF) > 0x00FF);
        return a.wrapping_add(b);
    }

    fn alu_swap(&mut self, a: u8) -> u8 {
        self.registers.flag(Z, a == 0);
        self.registers.flag(C, false);
        self.registers.flag(H, false);
        self.registers.flag(N, false);
        (a >> 4) | (a << 4)
    }

    fn alu_srflagupdate(&mut self, r: u8, c: bool) {
        self.registers.flag(H, false);
        self.registers.flag(N, false);
        self.registers.flag(Z, r == 0);
        self.registers.flag(C, c);
    }

    fn alu_rlc(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if c { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_rl(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if self.registers.getflag(C) { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_rrc(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if c { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_rr(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if self.registers.getflag(C) { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_sla(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = a << 1;
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_sra(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (a & 0x80);
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_srl(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = a >> 1;
        self.alu_srflagupdate(r, c);
        return r;
    }

    fn alu_bit(&mut self, a: u8, b: u8) {
        let r = a & (1 << (b as u32)) == 0;
        self.registers.flag(N, false);
        self.registers.flag(H, true);
        self.registers.flag(Z, r);
    }

    fn alu_daa(&mut self) {
        let mut a = self.registers.a;
        let mut adjust = if self.registers.getflag(C) {
            0x60
        } else {
            0x00
        };
        if self.registers.getflag(H) {
            adjust |= 0x06;
        };
        if !self.registers.getflag(N) {
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.registers.flag(C, adjust >= 0x60);
        self.registers.flag(H, false);
        self.registers.flag(Z, a == 0);
        self.registers.a = a;
    }

    fn cpu_jr(&mut self) {
        let n = self.get_byte() as i8;
        self.registers.pc = ((self.registers.pc as u32 as i32) + (n as i32)) as u16;
    }

    fn pushstack(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.memory.ww(self.registers.sp, value);
    }

    fn popstack(&mut self) -> u16 {
        let res = self.memory.rw(self.registers.sp);
        self.registers.sp += 2;
        res
    }
}
