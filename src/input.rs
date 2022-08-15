use crate::cpu::cpu::Interrupt;

#[derive(Debug)]
pub struct Input {
    buttons: u8,
    directions: u8,
    pub joypad_sel: u8,
    col: Selected,
}

pub enum Button {
    A,
    B,
    Start,
    Select,
    Left,
    Up,
    Down,
    Right,
}

// Enum for which column of inputs is selected. See
// http://nocash.emubase.de/pandocs.htm#joypadinput for codes and what each
// column is.
#[derive(Debug)]
pub enum Selected {
    Button = 0x20,
    Direction = 0x10,
    MltReq = 0x00,
}

impl Input {
    pub fn new() -> Input {
        Input {
            buttons: 0xf,
            directions: 0xf,
            col: Selected::Direction,
            joypad_sel: 0,
        }
    }

    pub fn rb(&self, _addr: u16) -> u8 {
        match self.col {
            Selected::Button => self.buttons,
            Selected::Direction => self.directions,
            Selected::MltReq => 0xf - self.joypad_sel,
        }
    }

    pub fn wb(&mut self, _addr: u16, value: u8) {
        // The selected column is also negatively asserted, so invert the value
        // written in to get a positively asserted selection
        match !value & 0x30 {
            0x20 => self.col = Selected::Button,
            0x10 => self.col = Selected::Direction,
            0x00 => self.col = Selected::MltReq,
            _ => {}
        }
    }

    // This is a mapping of key codes to the mask which will be AND'ed into the
    // correct value. These values are asserted low, so the relevant bit is
    // cleared. Here's what each bit position is:
    //
    // Bit 3 - P13 Input Down or Start (0=Pressed) 0111 = 0x7
    // Bit 2 - P12 Input Up or Select (0=Pressed) 1011 = 0xb
    // Bit 1 - P11 Input Left or Button B (0=Pressed) 1101 = 0xd
    // Bit 0 - P10 Input Right or Button A (0=Pressed) 1110 = 0xe

    pub fn keydown(&mut self, key: Button, if_: &mut u8) {
        *if_ |= Interrupt::Joypad as u8;
        match key {
            Button::A => {
                self.buttons &= 0xe;
            }
            Button::B => {
                self.buttons &= 0xd;
            }
            Button::Start => {
                self.buttons &= 0x7;
            }
            Button::Select => {
                self.buttons &= 0xb;
            }
            Button::Left => {
                self.directions &= 0xd;
            }
            Button::Up => {
                self.directions &= 0xb;
            }
            Button::Down => {
                self.directions &= 0x7;
            }
            Button::Right => {
                self.directions &= 0xe;
            }
        }
    }

    pub fn keyup(&mut self, key: Button) {
        match key {
            Button::A => {
                self.buttons |= !0xe;
            }
            Button::B => {
                self.buttons |= !0xd;
            }
            Button::Start => {
                self.buttons |= !0x7;
            }
            Button::Select => {
                self.buttons |= !0xb;
            }
            Button::Left => {
                self.directions |= !0xd;
            }
            Button::Up => {
                self.directions |= !0xb;
            }
            Button::Down => {
                self.directions |= !0x7;
            }
            Button::Right => {
                self.directions |= !0xe;
            }
        }
    }
}
