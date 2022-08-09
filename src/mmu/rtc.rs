#[derive(Debug)]
pub struct Rtc {
    pub current: u8,
    pub regs: [u8; 8],

    s: u8,
    m: u8,
    h: u8,
    d: u16,
    t: u8,
    carry: u8,
    stop: u8,
    readylatch: bool,
}

impl Rtc {
    pub fn new() -> Rtc {
        Rtc {
            s: 0,
            m: 0,
            h: 0,
            d: 0,
            t: 0,
            carry: 0,
            current: 0,
            regs: [0; 8],
            stop: 0,
            readylatch: false,
        }
    }

    pub fn latch(&mut self, value: u8) {
        if self.readylatch {
            if value == 1 {
                self.regs[0] = self.s;
                self.regs[1] = self.m;
                self.regs[2] = self.h;
                self.regs[3] = self.d as u8;
                self.regs[4] = ((self.d >> 8) as u8) | (self.stop << 6) | (self.carry << 7);
                self.regs[5] = 0xff;
                self.regs[6] = 0xff;
                self.regs[7] = 0xff;
            }
            self.readylatch = false;
        } else {
            self.readylatch = if value == 0 { true } else { false };
        }
    }

    pub fn wb(&mut self, _addr: u16, value: u8) {
        match self.current & 0x7 {
            0 => {
                self.s = value % 60;
                self.regs[0] = self.s;
            }
            1 => {
                self.m = value % 60;
                self.regs[1] = self.m;
            }
            2 => {
                self.h = value % 24;
                self.regs[2] = self.h;
            }
            3 => {
                self.regs[3] = value;
                self.d = (self.d & 0x100) | (value as u16);
            }
            4 => {
                self.regs[4] = value;
                self.d = (self.d & 0xff) | (((value as u16) & 1) << 8);
                self.stop = (value >> 6) & 1;
                self.carry = (value >> 7) & 1;
            }
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn step(&mut self) {
        if self.stop != 0 {
            return;
        }

        self.t += 1;
        if self.t >= 60 {
            self.s += 1;
            if self.s >= 60 {
                self.m += 1;
                if self.m >= 60 {
                    self.d += 1;
                    if self.d >= 365 {
                        self.d = 0;
                        self.carry = 1;
                    }
                    self.m = 0;
                }
                self.s = 0;
            }
            self.t = 0;
        }
    }
}
