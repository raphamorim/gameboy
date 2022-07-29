// use crate::gb::HEIGHT;
// use crate::gb::WIDTH;

const VRAM_SIZE: usize = 8 << 10; // 8K
const HEIGHT: usize = 144;
const WIDTH: usize = 160;

pub struct Gpu {
	pub image_data: Box<[u8; WIDTH * HEIGHT * 4]>,

    // CGB supports only 2 banks of vram
    vrambanks: Box<[[u8; VRAM_SIZE]; 2]>,
    // Selected vram bank
    vrambank: u8,

	clock: u32,
}

pub type Color = [u8; 4];

// The palette for the monochrome GB. The possible values are:
//
// 0 - white
// 1 - light gray
// 2 - dark gray
// 3 - black
const PALETTE: [Color; 4] = [
    [255, 255, 255, 255],
    [192, 192, 192, 255],
    [ 96,  96,  96, 255],
    [  0,   0,   0, 255],
];

impl Gpu {
	pub fn new() -> Gpu {
        Gpu {
            vrambanks: Box::new([[0; VRAM_SIZE];  2]),
            vrambank: 0,
            image_data: Box::new([255; HEIGHT * WIDTH * 4]),
            clock: 0,
        }
    }

    pub fn step(&mut self, clocks: u32, if_: &mut u8) {
        self.clock += clocks;

        // // If clock >= 456, then we've completed an entire line. This line might
        // // have been part of a vblank or part of a scanline.
        // if self.clock >= 456 {
        //     self.clock -= 456;
        //     self.ly = (self.ly + 1) % 154; // 144 lines tall, 10 for a vblank

        //     if self.ly >= 144 && self.mode != Mode::VBlank {
        //         self.switch(Mode::VBlank, if_);
        //     }

        //     if self.ly == self.lyc && self.lycly {
        //         *if_ |= Interrupt::LCDStat as u8;
        //     }
        // }

        // // Hop between modes if we're not in vblank
        // if self.ly < 144 {
        //     if self.clock <= 80 { // RDOAM takes 80 cycles
        //         if self.mode != Mode::RdOam { self.switch(Mode::RdOam, if_); }
        //     } else if self.clock <= 252 { // RDVRAM takes 172 cycles
        //         if self.mode != Mode::RdVram { self.switch(Mode::RdVram, if_); }
        //     } else { // HBLANK takes rest of time before line rendered
        //         if self.mode != Mode::HBlank { self.switch(Mode::HBlank, if_); }
        //     }
        // }
    }

    pub fn set_blank(&mut self) {
        for slot in self.image_data.iter_mut() {
            *slot = 0xff; // white
        }
    }

	// pub fn rb(&self, a: u16) -> u8 {
 //        match a {
 //            0x8000 ..= 0x9FFF => self.vram[(self.vrambank * 0x2000) | (a as usize & 0x1FFF)],
 //            0xFE00 ..= 0xFE9F => self.voam[a as usize - 0xFE00],
 //            _ => panic!("GPU does not handle read {:04X}", a),
 //        }
 //    }
}
