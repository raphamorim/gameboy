use crate::cpu::registers::Registers;
use crate::cpu::{data, ld, misc, stack};
use crate::mmu::MemoryManagementUnit;

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
    pub ime: bool,
    pub setdi: u32,
    pub setei: u32,
    pub halt: u32,
    pub stop: u32,
    pub memory: MemoryManagementUnit<'a>,

    // To debug
    _executed_operations: Vec<u8>,
}

impl Cpu<'_> {
    pub fn new(data: Vec<u8>, file: Option<std::path::PathBuf>) -> Self {
        let memory = MemoryManagementUnit::new_cgb(data, file).unwrap();
        let registers = Registers::new(memory.gbmode);

        Cpu {
            registers,
            memory,
            ime: false,
            setdi: 0,
            setei: 0,
            halt: 0,
            stop: 0,
            _executed_operations: Vec::new(),
        }
    }
    fn mut_find_or_insert<T: PartialEq>(vec: &mut Vec<T>, val: T) -> &mut T {
        if let Some(i) = vec.iter().position(|each| *each == val) {
            &mut vec[i]
        } else {
            vec.push(val);
            vec.last_mut().unwrap()
        }
    }
    pub fn debug(&mut self, op: u8) {
        Cpu::mut_find_or_insert(&mut self._executed_operations, op);
        // println!("{} {:#01x} {}", op, op, format!("{:?}", self.registers));
        // println!("{} {}", op, format!("{:?}", self.registers));
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

    fn handleinterrupt(&mut self) -> u32 {
        if !self.ime && self.halt == 0 {
            return 0;
        }

        let triggered = self.memory.inte & self.memory.intf;
        if triggered == 0 {
            return 0;
        }

        self.halt = 0;
        if !self.ime {
            return 0;
        }
        self.ime = false;

        let n = triggered.trailing_zeros();
        if n >= 5 {
            panic!("Invalid interrupt triggered");
        }
        self.memory.intf &= !(1 << n);
        let pc = self.registers.pc;
        stack::pushstack(self, pc);
        self.registers.pc = 0x0040 | ((n as u16) << 3);

        4
    }

    pub fn exec(&mut self) -> u32 {
        self.updateime();
        match self.handleinterrupt() {
            0 => {}
            n => return n,
        };

        if self.halt == 1 {
            // Emulate an noop instruction
            1
        } else {
            self.operation()
        }
    }

    pub fn do_cycle(&mut self) -> u32 {
        let ticks = self.exec() * 4;
        self.memory.do_cycle(ticks)

        // match self.delay {
        //     0 => {}
        //     1 => {
        //         self.delay = 0;
        //         self.ime = 1;
        //     }
        //     2 => {
        //         self.delay = 1;
        //     }
        //     _ => {}
        // }

        // let mut ticks = if self.halt == 0 && self.stop == 0 {
        //     self.operation()
        // } else {
        //     self.memory.switch_speed();
        //     self.stop = 0;
        //     1
        // };

        // // See http://bgb.bircd.org/pandocs.htm#interrupts
        // if self.ime != 0 || self.halt != 0 {
        //     let ints = self.memory.intf & self.memory.inte;

        //     if ints != 0 {
        //         let i = ints.trailing_zeros();
        //         if self.ime != 0 {
        //             self.memory.intf &= !(1 << (i as u32));
        //         }
        //         self.ime = 0;
        //         self.halt = 0;
        //         self.stop = 0;
        //         match i {
        //             0 => {
        //                 stack::rst(self, 0x40);
        //             }
        //             1 => {
        //                 stack::rst(self, 0x48);
        //             }
        //             2 => {
        //                 stack::rst(self, 0x50);
        //             }
        //             3 => {
        //                 stack::rst(self, 0x58);
        //             }
        //             4 => {
        //                 stack::rst(self, 0x60);
        //             }
        //             _ => {}
        //         }
        //         ticks += 1;
        //     }
        // }

        // self.ticks += ticks;
        // return ticks;
    }

    fn operation(&mut self) -> u32 {
        let op = self.get_byte();
        // self.debug(op);
        // println!("{:?}", self.registers);
        match op {
            0x00 => 1,
            0x01 => {
                ld::bcnn(self);
                3
            }
            0x02 => {
                ld::bcm_a(self);
                2
            }
            0x03 => {
                data::incbc(self);
                2
            }
            0x04 => {
                data::incr_b(self);
                1
            }
            0x05 => {
                data::decr_b(self);
                1
            }
            0x06 => {
                ld::rr_b(self);
                2
            }
            0x07 => {
                misc::rlca(self);
                1
            }
            0x08 => {
                ld::mmsp(self);
                5
            }
            0x09 => {
                data::addhlbc(self);
                2
            }
            0x0A => {
                ld::abcm(self);
                2
            }
            0x0B => {
                data::decbc(self);
                2
            }
            0x0C => {
                data::incr_c(self);
                1
            }
            0x0D => {
                data::decr_c(self);
                1
            }
            0x0E => {
                ld::rr_c(self);
                2
            }
            0x0F => {
                misc::rrca(self);
                1
            }
            0x10 => {
                // stack::djnzn(self);
                self.memory.switch_speed();
                1
            }
            0x11 => {
                ld::denn(self);
                3
            }
            0x12 => {
                ld::dem_a(self);
                2
            }
            0x13 => {
                data::incde(self);
                2
            }
            0x14 => {
                data::incr_d(self);
                1
            }
            0x15 => {
                data::decr_d(self);
                1
            }
            0x16 => {
                ld::rr_d(self);
                2
            }
            0x17 => {
                misc::rla(self);
                1
            }
            0x18 => {
                stack::jrn(self);
                3
            }
            0x19 => {
                data::addhlde(self);
                2
            }
            0x1A => {
                ld::adem(self);
                2
            }
            0x1B => {
                data::decde(self);
                2
            }
            0x1C => {
                data::incr_e(self);
                1
            }
            0x1D => {
                data::decr_e(self);
                1
            }
            0x1E => {
                ld::rr_e(self);
                2
            }
            0x1F => {
                misc::rra(self);
                1
            }
            0x20 => stack::jrnzn(self),
            0x21 => {
                ld::hlnn(self);
                3
            }
            0x22 => {
                ld::hlia(self);
                2
            }
            0x23 => {
                data::inchl(self);
                2
            }
            0x24 => {
                data::incr_h(self);
                1
            }
            0x25 => {
                data::decr_h(self);
                1
            }
            0x26 => {
                self.registers.h = self.get_byte();
                2
            }
            0x27 => {
                ld::rr_h(self);
                1
            }
            0x28 => stack::jrzn(self),
            0x29 => {
                data::addhlhl(self);
                2
            }
            0x2A => {
                ld::ahli(self);
                2
            }
            0x2B => {
                data::dechl(self);
                2
            }
            0x2C => {
                data::incr_l(self);
                1
            }
            0x2D => {
                data::decr_l(self);
                1
            }
            0x2E => {
                ld::rr_l(self);
                2
            }
            0x2F => {
                data::cpl(self);
                1
            }
            0x30 => stack::jrncn(self),
            0x31 => {
                ld::spnn(self);
                3
            }
            0x32 => {
                ld::hld_a(self);
                2
            }
            0x33 => {
                data::incsp(self);
                2
            }
            0x34 => {
                data::inchlm(self);
                3
            }
            0x35 => {
                data::dechlm(self);
                3
            }
            0x36 => {
                ld::hlmn(self);
                3
            }
            0x37 => {
                data::scf(self);
                1
            }
            0x38 => stack::jrcn(self),
            0x39 => {
                data::addhlsp(self);
                2
            }
            0x3A => {
                ld::ahld(self);
                2
            }
            0x3B => {
                data::decsp(self);
                2
            }
            0x3C => {
                data::incr_a(self);
                1
            }
            0x3D => {
                data::decr_a(self);
                1
            }
            0x3E => {
                ld::rr_a(self);
                2
            }
            0x3F => {
                data::ccf(self);
                1
            }
            0x40 => {
                // ld::rr_bb(self);
                1
            }
            0x41 => {
                ld::rr_bc(self);
                1
            }
            0x42 => {
                ld::rr_bd(self);
                1
            }
            0x43 => {
                ld::rr_be(self);
                1
            }
            0x44 => {
                ld::rr_bh(self);
                1
            }
            0x45 => {
                ld::rr_bl(self);
                1
            }
            0x46 => {
                ld::r_hlm_b(self);
                2
            }
            0x47 => {
                ld::rr_ba(self);
                1
            }
            0x48 => {
                ld::rr_cb(self);
                1
            }
            0x49 => {
                // ld::rr_cc(self);
                1
            }
            0x4A => {
                ld::rr_cd(self);
                1
            }
            0x4B => {
                ld::rr_ce(self);
                1
            }
            0x4C => {
                ld::rr_ch(self);
                1
            }
            0x4D => {
                ld::rr_cl(self);
                1
            }
            0x4E => {
                ld::r_hlm_c(self);
                2
            }
            0x4F => {
                ld::rr_ca(self);
                1
            }
            0x50 => {
                ld::rr_db(self);
                1
            }
            0x51 => {
                ld::rr_dc(self);
                1
            }
            0x52 => 1,
            0x53 => {
                ld::rr_de(self);
                1
            }
            0x54 => {
                ld::rr_dh(self);
                1
            }
            0x55 => {
                ld::rr_dl(self);
                1
            }
            0x56 => {
                ld::r_hlm_d(self);
                2
            }
            0x57 => {
                ld::rr_da(self);
                1
            }
            0x58 => {
                ld::rr_eb(self);
                1
            }
            0x59 => {
                ld::rr_ec(self);
                1
            }
            0x5A => {
                ld::rr_ed(self);
                1
            }
            0x5B => 1,
            0x5C => {
                ld::rr_eh(self);
                1
            }
            0x5D => {
                ld::rr_el(self);
                1
            }
            0x5E => {
                ld::r_hlm_e(self);
                2
            }
            0x5F => {
                ld::rr_ea(self);
                1
            }
            0x60 => {
                ld::rr_hb(self);
                1
            }
            0x61 => {
                ld::rr_hc(self);
                1
            }
            0x62 => {
                ld::rr_hd(self);
                1
            }
            0x63 => {
                ld::rr_he(self);
                1
            }
            0x64 => 1,
            0x65 => {
                ld::rr_hl(self);
                1
            }
            0x66 => {
                ld::r_hlm_h(self);
                2
            }
            0x67 => {
                ld::rr_ha(self);
                1
            }
            0x68 => {
                ld::rr_lb(self);
                1
            }
            0x69 => {
                ld::rr_lc(self);
                1
            }
            0x6A => {
                ld::rr_ld(self);
                1
            }
            0x6B => {
                ld::rr_le(self);
                1
            }
            0x6C => {
                ld::rr_lh(self);
                1
            }
            0x6D => 1,
            0x6E => {
                ld::r_hlm_l(self);
                2
            }
            0x6F => {
                ld::rr_la(self);
                1
            }
            0x70 => {
                ld::hlmr_b(self);
                2
            }
            0x71 => {
                ld::hlmr_c(self);
                2
            }
            0x72 => {
                ld::hlmr_d(self);
                2
            }
            0x73 => {
                ld::hlmr_e(self);
                2
            }
            0x74 => {
                ld::hlmr_h(self);
                2
            }
            0x75 => {
                ld::hlmr_l(self);
                2
            }
            0x76 => {
                self.halt = 1;
                1
            }
            0x77 => {
                ld::hlmr_a(self);
                2
            }
            0x78 => {
                ld::rr_ab(self);
                1
            }
            0x79 => {
                ld::rr_ac(self);
                1
            }
            0x7A => {
                ld::rr_ad(self);
                1
            }
            0x7B => {
                ld::rr_ae(self);
                1
            }
            0x7C => {
                ld::rr_ah(self);
                1
            }
            0x7D => {
                ld::rr_al(self);
                1
            }
            0x7E => {
                ld::r_hlm_a(self);
                2
            }
            0x7F => {
                // ld::rr_aa(self);
                1
            }
            0x80 => {
                data::addr_b(self);
                1
            }
            0x81 => {
                data::addr_c(self);
                1
            }
            0x82 => {
                data::addr_d(self);
                1
            }
            0x83 => {
                data::addr_e(self);
                1
            }
            0x84 => {
                data::addr_h(self);
                1
            }
            0x85 => {
                data::addr_l(self);
                1
            }
            0x86 => {
                data::addhl(self);
                2
            }
            0x87 => {
                data::addr_a(self);
                1
            }
            0x88 => {
                data::adcr_b(self);
                1
            }
            0x89 => {
                data::adcr_c(self);
                1
            }
            0x8A => {
                data::adcr_d(self);
                1
            }
            0x8B => {
                data::adcr_e(self);
                1
            }
            0x8C => {
                data::adcr_h(self);
                1
            }
            0x8D => {
                data::adcr_l(self);
                1
            }
            0x8E => {
                data::adchl(self);
                2
            }
            0x8F => {
                data::adcr_a(self);
                1
            }
            0x90 => {
                data::subr_b(self);
                1
            }
            0x91 => {
                data::subr_c(self);
                1
            }
            0x92 => {
                data::subr_d(self);
                1
            }
            0x93 => {
                data::subr_e(self);
                1
            }
            0x94 => {
                data::subr_h(self);
                1
            }
            0x95 => {
                data::subr_l(self);
                1
            }
            0x96 => {
                data::subhl(self);
                2
            }
            0x97 => {
                data::subr_a(self);
                1
            }
            0x98 => {
                data::sbcr_b(self);
                1
            }
            0x99 => {
                data::sbcr_c(self);
                1
            }
            0x9A => {
                data::sbcr_d(self);
                1
            }
            0x9B => {
                data::sbcr_e(self);
                1
            }
            0x9C => {
                data::sbcr_h(self);
                1
            }
            0x9D => {
                data::sbcr_l(self);
                1
            }
            0x9E => {
                data::sbchl(self);
                2
            }
            0x9F => {
                data::sbcr_a(self);
                1
            }
            0xA0 => {
                data::andr_b(self);
                1
            }
            0xA1 => {
                data::andr_c(self);
                1
            }
            0xA2 => {
                data::andr_d(self);
                1
            }
            0xA3 => {
                data::andr_e(self);
                1
            }
            0xA4 => {
                data::andr_h(self);
                1
            }
            0xA5 => {
                data::andr_l(self);
                1
            }
            0xA6 => {
                data::andhl(self);
                2
            }
            0xA7 => {
                data::andr_a(self);
                1
            }
            0xA8 => {
                data::xorr_b(self);
                1
            }
            0xA9 => {
                data::xorr_c(self);
                1
            }
            0xAA => {
                data::xorr_d(self);
                1
            }
            0xAB => {
                data::xorr_e(self);
                1
            }
            0xAC => {
                data::xorr_h(self);
                1
            }
            0xAD => {
                data::xorr_l(self);
                1
            }
            0xAE => {
                data::xorhl(self);
                2
            }
            0xAF => {
                data::xorr_a(self);
                1
            }
            0xB0 => {
                data::orr_b(self);
                1
            }
            0xB1 => {
                data::orr_c(self);
                1
            }
            0xB2 => {
                data::orr_d(self);
                1
            }
            0xB3 => {
                data::orr_e(self);
                1
            }
            0xB4 => {
                data::orr_h(self);
                1
            }
            0xB5 => {
                data::orr_l(self);
                1
            }
            0xB6 => {
                data::orhl(self);
                2
            }
            0xB7 => {
                data::orr_a(self);
                1
            }
            0xB8 => {
                data::cpr_b(self);
                1
            }
            0xB9 => {
                data::cpr_c(self);
                1
            }
            0xBA => {
                data::cpr_d(self);
                1
            }
            0xBB => {
                data::cpr_e(self);
                1
            }
            0xBC => {
                data::cpr_h(self);
                1
            }
            0xBD => {
                data::cpr_l(self);
                1
            }
            0xBE => {
                data::cphl(self);
                2
            }
            0xBF => {
                data::cpr_a(self);
                1
            }
            0xC0 => stack::retnz(self),
            0xC1 => {
                stack::popbc(self);
                3
            }
            0xC2 => stack::jpnznn(self),
            0xC3 => {
                stack::jpnn(self);
                4
            }
            0xC4 => stack::callnznn(self),
            0xC5 => {
                stack::pushbc(self);
                4
            }
            0xC6 => {
                data::addn(self);
                2
            }
            0xC7 => {
                stack::rst(self, 0x00);
                4
            }
            0xC8 => stack::retz(self),
            0xC9 => {
                stack::ret(self);
                4
            }
            0xCA => stack::jpznn(self),
            0xCB => misc::cbmap(self),
            0xCC => stack::callznn(self),
            0xCD => {
                stack::callnn(self);
                6
            }
            0xCE => {
                data::adcn(self);
                2
            }
            0xCF => {
                stack::rst(self, 0x08);
                4
            }
            0xD0 => stack::retnc(self),
            0xD1 => {
                stack::popde(self);
                3
            }
            0xD2 => stack::jpncnn(self),
            0xD4 => stack::callncnn(self),
            0xD5 => {
                stack::pushde(self);
                4
            }
            0xD6 => {
                data::subn(self);
                2
            }
            0xD7 => {
                stack::rst(self, 0x10);
                4
            }
            0xD8 => stack::retc(self),
            0xD9 => {
                stack::reti(self);
                self.setei = 1;
                4
            }
            0xDA => stack::jpcnn(self),
            0xDC => stack::callcnn(self),
            0xDE => {
                data::sbcn(self);
                2
            }
            0xDF => {
                stack::rst(self, 0x18);
                4
            }
            0xE0 => {
                ld::ion_a(self);
                3
            }
            0xE1 => {
                stack::pophl(self);
                3
            }
            0xE2 => {
                ld::ioca(self);
                2
            }
            0xE5 => {
                stack::pushhl(self);
                4
            }
            0xE6 => {
                data::andn(self);
                2
            }
            0xE7 => {
                stack::rst(self, 0x20);
                4
            }
            0xE8 => {
                data::addspn(self);
                4
            }
            0xE9 => {
                stack::jphl(self);
                1
            }
            0xEA => {
                ld::mm_a(self);
                4
            }
            0xEE => {
                data::orn(self);
                2
            }
            0xEF => {
                stack::rst(self, 0x28);
                4
            }
            0xF0 => {
                ld::aion(self);
                3
            }
            0xF1 => {
                stack::popaf(self);
                3
            }
            0xF2 => {
                ld::aioc(self);
                2
            }
            0xF3 => {
                self.setdi = 2;
                1
            }
            0xF5 => {
                stack::pushaf(self);
                4
            }
            0xF6 => {
                let v = self.get_byte();
                data::alu_or(self, v);
                2
            }
            0xF7 => {
                stack::rst(self, 0x30);
                4
            }
            0xF8 => {
                let r = data::alu_add16imm(self);
                data::set_hl(self, r);
                3
            }
            0xF9 => {
                ld::hlspn(self);
                2
            }
            0xFA => {
                ld::amm(self);
                4
            }
            0xFB => {
                self.setei = 2;
                1
            }
            0xFE => {
                data::cpn(self);
                2
            }
            0xFF => {
                stack::rst(self, 0x38);
                4
            }
            _ => {
                println!(
                    "Instruction at {:#01x} | {:#06x} | {} not implemented, stopping.",
                    op, op, op
                );
                self.stop = 1;
                panic!("{:#06x} not implemented", op);
            }
        }
    }
}
