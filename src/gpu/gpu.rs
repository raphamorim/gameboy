use crate::cpu::cpu::Interrupt;
use crate::mmu::mmu::Mmu;

const VRAM_SIZE: usize = 8 << 10; // 8K
const OAM_SIZE: usize = 0xa0; // 0xffe00 - 0xffe9f is OAM
const CGB_BP_SIZE: usize = 64; // 64 bytes of extra memory
const NUM_TILES: usize = 384; // number of in-memory tiles

pub const HEIGHT: usize = 144;
pub const WIDTH: usize = 160;

// The palette for the monochrome GB. The possible values are:
//
// 0 - white
// 1 - light gray
// 2 - dark gray
// 3 - black
const PALETTE: [Color; 4] = [
    [255, 255, 255, 255],
    [192, 192, 192, 255],
    [96, 96, 96, 255],
    [0, 0, 0, 255],
];

pub type Color = [u8; 4];

#[derive(Debug)]
pub struct Gpu {
    pub oam: [u8; OAM_SIZE],

    pub image_data: Box<[u8; WIDTH * HEIGHT * 4]>,

    pub is_cgb: bool,
    pub is_sgb: bool,

    mode: Mode,

    // CGB supports only 2 banks of vram
    vrambanks: Box<[[u8; VRAM_SIZE]; 2]>,
    // Selected vram bank
    vrambank: u8,

    clock: u32,

    // Registers used by the GPU

    // 0xff40 - LCD control (LCDC) - in order from most to least significant bit
    pub lcdon: bool,    // LCD monitor turned on or off?
    winmap: bool,       // Window Tile Map Display (0=9800-9BFF, 1=9C00-9FFF)
    winon: bool,        // Window Display Enable   (0=Off, 1=On)
    pub tiledata: bool, // BG & Window Tile Data   (0=8800-97FF, 1=8000-8FFF)
    bgmap: bool,        // BG Tile Map Display     (0=9800-9BFF, 1=9C00-9FFF)
    objsize: bool,      // OBJ (Sprite) Size       (0=8x8, 1=8x16)
    objon: bool,        // OBJ (Sprite) Display    (0=Off, 1=On)
    bgon: bool,         // BG Display              (0=Off, 1=On)

    // 0xff41 - STAT - LCDC Status - starts with bit 6
    lycly: bool,    // LYC=LY Coincidence Interrupt (1=Enable)
    mode2int: bool, // Mode 2 OAM Interrupt         (1=Enable)
    mode1int: bool, // Mode 1 V-Blank Interrupt     (1=Enable)
    mode0int: bool, // Mode 0 H-Blank Interrupt     (1=Enable)

    // 0xff42 - SCY - Scroll Y
    scy: u8,
    // 0xff43 - SCX - Scroll X
    scx: u8,
    // 0xff44 - LY - LCDC Y-Coordinate
    ly: u8,
    // 0xff45 - LYC - LY Compare
    lyc: u8,

    // 0xff47 - BGP - BG Palette Data
    bgp: u8,
    // 0xff48 - OBP0 - Object Palette 0 Data
    obp0: u8,
    // 0xff49 - OBP1 - Object Palette 1Data
    obp1: u8,
    // 0xff4a - WY - Window Y Position
    wy: u8,
    // 0xff4b - WX - Window X Position minus 7
    wx: u8,

    // CGB VRAM DMA transfer, more info at:
    // http://nocash.emubase.de/pandocs.htm#lcdvramdmatransferscgbonly
    hdma_src: u16,
    hdma_dst: u16,
    hdma5: u8,

    // Compiled palettes. These are updated when writing to BGP/OBP0/OBP1. Meant
    // for non CGB use only. Each palette is an array of 4 color schemes. Each
    // color scheme is one in PALETTE.
    pal: Box<Palette>,

    // Compiled tiles
    tiles: Box<Tiles>,

    // When in CGB mode, the BGP and OBP memory is stored internally and is only
    // accessible through some I/O registers. Each section of memory is 64 bytes
    // and defines 8 palettes of 4 colors each
    cgb: Box<CgbData>,

    // Data related to SGB operation
    pub sgb: Box<SgbData>,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Mode {
    HBlank = 0x00, // mode 0
    VBlank = 0x01, // mode 1
    RdOam = 0x02,  // mode 2
    RdVram = 0x03, // mode 3
}

#[derive(Debug)]
struct Palette {
    bg: [Color; 4],
    obp0: [Color; 4],
    obp1: [Color; 4],
}

#[derive(Debug)]
struct Tiles {
    data: [[[u8; 8]; 8]; NUM_TILES * 2],
    need_update: bool,
    to_update: [bool; NUM_TILES * 2],
}

#[derive(Debug)]
struct CgbData {
    // Raw memory
    bgp: [u8; CGB_BP_SIZE],
    obp: [u8; CGB_BP_SIZE],
    // Index registers into memory
    bgpi: u8,
    obpi: u8,
    // Compiled palettes
    cbgp: [[Color; 4]; 8],
    cobp: [[Color; 4]; 8],
}

#[derive(Debug)]
pub struct SgbData {
    // This is a 20x18 array which maps palettes to locations on the screen.
    // Each element defines an 8x8 block on the GB screen which should be mapped
    // through these palettes instead of using the normal grayscale.
    pub atf: [u8; 20 * 18],

    // Actual compiled palettes where each palette is an array of 4 colors where
    // each color has 4 components
    pub pal: [[Color; 4]; 4],
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            vrambanks: Box::new([[0; VRAM_SIZE]; 2]),
            vrambank: 0,
            oam: [0; OAM_SIZE],
            is_cgb: false,
            is_sgb: false,
            image_data: Box::new([0; HEIGHT * WIDTH * 4]),

            mode: Mode::RdOam,
            wx: 0,
            wy: 0,
            obp1: 0,
            obp0: 0,
            bgp: 0,
            lyc: 0,
            ly: 0,
            scx: 0,
            scy: 0,
            mode0int: false,
            mode1int: false,
            mode2int: false,
            lycly: false,
            bgon: false,
            objon: false,
            objsize: false,
            bgmap: false,
            tiledata: false,
            winon: false,
            winmap: false,
            lcdon: false,

            clock: 0,
            hdma_src: 0,
            hdma_dst: 0,
            hdma5: 0,

            pal: Box::new(Palette {
                bg: [[0; 4]; 4],
                obp0: [[0; 4]; 4],
                obp1: [[0; 4]; 4],
            }),

            tiles: Box::new(Tiles {
                need_update: false,
                to_update: [false; NUM_TILES * 2],
                data: [[[0; 8]; 8]; NUM_TILES * 2],
            }),

            cgb: Box::new(CgbData {
                bgp: [255; CGB_BP_SIZE],
                obp: [0; CGB_BP_SIZE],
                bgpi: 0,
                obpi: 0,
                cbgp: [[[255, 255, 255, 255]; 4]; 8],
                cobp: [[[0, 0, 0, 255]; 4]; 8],
            }),

            sgb: Box::new(SgbData {
                atf: [0; 20 * 18],
                pal: [[[0, 0, 0, 255]; 4]; 4],
            }),
        }
    }

    pub fn vram(&self) -> &[u8; VRAM_SIZE] {
        &self.vrambanks[self.vrambank as usize]
    }
    pub fn vram_mut(&mut self) -> &mut [u8; VRAM_SIZE] {
        &mut self.vrambanks[self.vrambank as usize]
    }

    fn switch(&mut self, mode: Mode, if_: &mut u8) {
        self.mode = mode;
        match mode {
            Mode::HBlank => {
                self.render_line();
                if self.mode0int {
                    *if_ |= Interrupt::LCDStat as u8;
                }
            }
            Mode::VBlank => {
                // TODO: a frame is ready, it should be put on screen at this
                // point
                *if_ |= Interrupt::Vblank as u8;
                if self.mode1int {
                    *if_ |= Interrupt::LCDStat as u8;
                }
            }
            Mode::RdOam => {
                if self.mode2int {
                    *if_ |= Interrupt::LCDStat as u8;
                }
            }
            Mode::RdVram => {}
        }
    }

    // Step the GPU a number of clock cycles forward. The GPU's screen is
    // synchronized with the CPU clock because in a real GB, the two are
    // matched up on the same clock.
    //
    // This function mostly doesn't do anything except for incrementing its own
    // internal counter of clock cycles that have passed. It's a state machine
    // between a few different states. In one state, however, the rendering of a
    // screen occurs, but that doesn't always happen when calling this function.
    pub fn step(&mut self, clocks: u32, if_: &mut u8) {
        // Timings located here:
        //      http://nocash.emubase.de/pandocs.htm#lcdstatusregister
        self.clock += clocks;

        // If clock >= 456, then we've completed an entire line. This line might
        // have been part of a vblank or part of a scanline.
        if self.clock >= 456 {
            self.clock -= 456;
            self.ly = (self.ly + 1) % 154; // 144 lines tall, 10 for a vblank

            if self.ly >= 144 && self.mode != Mode::VBlank {
                self.switch(Mode::VBlank, if_);
            }

            if self.ly == self.lyc && self.lycly {
                *if_ |= Interrupt::LCDStat as u8;
            }
        }

        // Hop between modes if we're not in vblank
        if self.ly < 144 {
            if self.clock <= 80 {
                // RDOAM takes 80 cycles
                if self.mode != Mode::RdOam {
                    self.switch(Mode::RdOam, if_);
                }
            } else if self.clock <= 252 {
                // RDVRAM takes 172 cycles
                if self.mode != Mode::RdVram {
                    self.switch(Mode::RdVram, if_);
                }
            } else {
                // HBLANK takes rest of time before line rendered
                if self.mode != Mode::HBlank {
                    self.switch(Mode::HBlank, if_);
                }
            }
        }
    }

    fn render_line(&mut self) {
        if !self.lcdon {
            return;
        }

        let mut scanline = [0u8; WIDTH];

        if self.tiles.need_update {
            self.update_tileset();
            self.tiles.need_update = false;
        }

        if self.bgon {
            self.render_background(&mut scanline);
        }
        if self.winon {
            self.render_window(&mut scanline);
        }
        if self.objon {
            self.render_sprites(&mut scanline);
        }
    }

    fn update_tileset(&mut self) {
        let tiles = &mut *self.tiles;
        let iter = tiles.to_update.iter_mut();
        for (i, slot) in iter.enumerate().filter(|&(_, &mut i)| i) {
            *slot = false;

            // Each tile is 16 bytes long. Each pair of bytes represents a line
            // of pixels (making 8 lines). The first byte is the LSB of the
            // color number and the second byte is the MSB of the color.
            //
            // For example, for:
            //      byte 0 : 01011011
            //      byte 1 : 01101010
            //
            // The colors are [0, 2, 2, 1, 3, 0, 3, 1]
            for j in 0..8 {
                let addr = ((i % NUM_TILES) * 16) + j * 2;
                // All tiles are located 0x8000-0x97ff => 0x0000-0x17ff in VRAM
                // meaning that the index is simply an index into raw VRAM
                let (mut lsb, mut msb) = if i < NUM_TILES {
                    (self.vrambanks[0][addr], self.vrambanks[0][addr + 1])
                } else {
                    (self.vrambanks[1][addr], self.vrambanks[1][addr + 1])
                };

                // LSB is the right-most pixel.
                for k in (0..8).rev() {
                    tiles.data[i][j][k] = ((msb & 1) << 1) | (lsb & 1);
                    lsb >>= 1;
                    msb >>= 1;
                }
            }
        }
    }

    pub fn add_tilei(&self, base: usize, tilei: u8) -> usize {
        // tiledata = 0 => tilei is a signed byte, so fix it here
        if self.tiledata {
            base + tilei as usize
        } else {
            (base as isize + (tilei as i8 as isize)) as usize
        }
    }

    pub fn bgbase(&self) -> usize {
        // vram is from 0x8000-0x9fff
        // self.bgmap: 0=9800-9bff, 1=9c00-9fff
        //
        // Each map is a 32x32 (1024) array of bytes. Each byte is an index into
        // the tile map. Each tile is an 8x8 block of pixels.
        if self.bgmap {
            0x1c00
        } else {
            0x1800
        }
    }

    fn render_background(&mut self, scanline: &mut [u8; WIDTH]) {
        let mapbase = self.bgbase();
        let line = self.ly as usize + self.scy as usize;

        // Now offset from the base to the right location. We divide by 8
        // because each tile is 8 pixels high. We then multiply by 32
        // because each row is 32 bytes long. We can't just multiply by 4
        // because we need the truncation to happen beforehand
        let mapbase = mapbase + ((line % 256) >> 3) * 32;

        println!("{:?} {:?}", self.ly, self.scy);

        // X and Y location inside the tile itself to paint
        let y = (self.ly + self.scy) % 8;
        let mut x = self.scx % 8;

        // Offset into the canvas to draw. line * width * 4 colors
        let mut coff = (self.ly as usize) * WIDTH * 4;

        // this.tiledata is a flag to determine which tile data table to use
        // 0=8800-97FF, 1=8000-8FFF. For some odd reason, if tiledata = 0, then
        // (&tiles[0]) == 0x9000, where if tiledata = 1, (&tiles[0]) = 0x8000.
        // This implies that the indices are treated as signed numbers.
        let mut i = 0;
        let tilebase = if !self.tiledata { 256 } else { 0 };

        loop {
            // Backgrounds wrap around, so calculate the offset into the bgmap
            // each loop to check for wrapping
            let mapoff = ((i as usize + self.scx as usize) % 256) >> 3;
            let tilei = self.vrambanks[0][mapbase + mapoff];

            // tiledata = 0 => tilei is a signed byte, so fix it here
            let tilebase = self.add_tilei(tilebase, tilei);

            let row;
            let bgpri;
            let hflip;
            let bgp;
            if self.is_cgb {
                // See http://nocash.emubase.de/pandocs.htm#vrambackgroundmaps
                // for what the attribute byte all maps to
                //
                // Summary of attributes bits:
                //  Bit 0-2  Background Palette number  (BGP0-7)
                //  Bit 3    Tile VRAM Bank number      (0=Bank 0, 1=Bank 1)
                //  Bit 4    Not used
                //  Bit 5    Horizontal Flip       (0=Normal, 1=Mirror)
                //  Bit 6    Vertical Flip         (0=Normal, 1=Mirror)
                //  Bit 7    BG-to-OAM Priority    (0=OAM, 1=BG)

                let attrs = self.vrambanks[1][mapbase + mapoff as usize] as usize;

                let tile = self.tiles.data[tilebase + ((attrs >> 3) & 1) * NUM_TILES];
                bgpri = attrs & 0x80 != 0;
                hflip = attrs & 0x20 != 0;
                row = tile[if attrs & 0x40 != 0 { 7 - y } else { y } as usize];
                bgp = self.cgb.cbgp[attrs & 0x7];
            } else {
                row = self.tiles.data[tilebase as usize][y as usize];
                bgpri = false;
                hflip = false;
                bgp = self.pal.bg;
            }

            while x < 8 && i < WIDTH as u8 {
                let colori = row[if hflip { 7 - x } else { x } as usize];
                let color;
                if self.is_sgb && !self.is_cgb {
                    let sgbaddr = (i >> 3) as usize + (self.ly as usize >> 3) * 20;
                    let mapped = self.sgb.atf[sgbaddr] as usize;
                    match bgp[colori as usize][0] {
                        0 => {
                            color = self.sgb.pal[mapped][3];
                        }
                        96 => {
                            color = self.sgb.pal[mapped][2];
                        }
                        192 => {
                            color = self.sgb.pal[mapped][1];
                        }
                        255 => {
                            color = self.sgb.pal[mapped][0];
                        }

                        // not actually reachable
                        _ => {
                            color = [0, 0, 0, 0];
                        }
                    }
                } else {
                    color = bgp[colori as usize];
                }
                // To indicate bg priority, list a color >= 4
                scanline[i as usize] = if bgpri { 4 } else { colori };

                self.image_data[coff] = color[0];
                self.image_data[coff + 1] = color[1];
                self.image_data[coff + 2] = color[2];
                self.image_data[coff + 3] = color[3];

                x += 1;
                i += 1;
                coff += 4;
            }

            x = 0;
            if i >= WIDTH as u8 {
                break;
            }
        }
    }

    fn render_window(&mut self, scanline: &mut [u8; WIDTH]) {
        // If our current line is less than the windows initial offset, then
        // there's no window to draw
        if self.ly < self.wy {
            return;
        }

        // The window's x position is actually at (wx - 7), so if the wx
        // register is greater than WIDTH + 7, then there's nothing to do
        if self.wx >= WIDTH as u8 + 7 {
            return;
        }

        let mapbase = if self.winmap { 0x1c00 } else { 0x1800 };
        let mapbase = mapbase + ((self.ly as usize - self.wy as usize) >> 3) * 32;

        // X and Y location inside the tile itself to paint
        //
        // The Y location is offset by the window's offset (as with the above
        // mapbase calculation), and the X location takes into account that the
        // actual offset is wx - 7 (and is careful to avoid overflow).
        let y = (self.ly - self.wy) % 8;
        let (mut x, mut i) = if self.wx < 7 {
            (7 - self.wx, 0)
        } else {
            ((self.wx - 7) % 8, self.wx - 7)
        };

        // Offset into the canvas to draw. (line * width + xoff) * 4 colors
        let mut coff = (self.ly as usize * WIDTH + i as usize) * 4;

        // this.tiledata is a flag to determine which tile data table to use
        // 0=8800-97FF, 1=8000-8FFF. For some odd reason, if tiledata = 0, then
        // (&tiles[0]) == 0x9000, where if tiledata = 1, (&tiles[0]) = 0x8000.
        // This implies that the indices are treated as signed numbers.
        let tilebase = if !self.tiledata { 256 } else { 0 };

        let mut mapoff = 0;
        loop {
            let tilei = self.vrambanks[0][mapbase + mapoff as usize];
            mapoff += 1;

            // tiledata = 0 => tilei is a signed byte, so fix it here
            let tilebase = self.add_tilei(tilebase, tilei);

            let row;
            let bgpri;
            let hflip;
            let bgp;
            if self.is_cgb {
                let attrs = self.vrambanks[1][mapbase + mapoff as usize - 1] as usize;

                let tile = self.tiles.data[tilebase + ((attrs >> 3) & 1) * NUM_TILES];
                bgpri = attrs & 0x80 != 0;
                hflip = attrs & 0x20 != 0;
                row = tile[if attrs & 0x40 != 0 { 7 - y } else { y } as usize];
                bgp = self.cgb.cbgp[attrs & 0x7];
            } else {
                row = self.tiles.data[tilebase as usize][y as usize];
                bgpri = false;
                hflip = false;
                bgp = self.pal.bg;
            }

            while x < 8 && i < WIDTH as u8 {
                let colori = row[if hflip { 7 - x } else { x } as usize];
                let color;
                if self.is_sgb && !self.is_cgb {
                    let sgbaddr = (i >> 3) + (self.ly >> 3) * 20;
                    let mapped = self.sgb.atf[sgbaddr as usize] as usize;
                    match bgp[colori as usize][0] {
                        0 => {
                            color = self.sgb.pal[mapped][3];
                        }
                        96 => {
                            color = self.sgb.pal[mapped][2];
                        }
                        192 => {
                            color = self.sgb.pal[mapped][1];
                        }
                        255 => {
                            color = self.sgb.pal[mapped][0];
                        }

                        // not actually reachable
                        _ => {
                            color = [0, 0, 0, 0];
                        }
                    }
                } else {
                    color = bgp[colori as usize];
                }
                // To indicate bg priority, list a color >= 4
                scanline[i as usize] = if bgpri { 4 } else { colori };

                self.image_data[coff] = color[0];
                self.image_data[coff + 1] = color[1];
                self.image_data[coff + 2] = color[2];
                self.image_data[coff + 3] = color[3];

                x += 1;
                i += 1;
                coff += 4;
            }

            x = 0;
            if i >= 160 {
                break;
            }
        }
    }

    fn render_sprites(&mut self, scanline: &mut [u8; WIDTH]) {
        // More information about sprites is located at:
        // http://nocash.emubase.de/pandocs.htm#vramspriteattributetableoam

        let line = self.ly as i32;
        let ysize = if self.objsize { 16 } else { 8 };

        // All sprits are located in OAM
        // There are 40 sprites in total, each is 4 bytes wide
        for sprite in self.oam.chunks(4) {
            let mut yoff = (sprite[0] as i32) - 16;
            let xoff = (sprite[1] as i32) - 8;
            let mut tile = sprite[2] as usize;
            let flags = sprite[3];

            // First make sure that this sprite even lands on the current line
            // being rendered. The y value in the sprite is the top left corner,
            // so if that is below the scanline or the bottom of the sprite
            // (which is 8 pixels high) lands below the scanline, this sprite
            // doesn't need to be rendered right now
            if yoff > line || yoff + ysize <= line || xoff <= -8 || xoff >= WIDTH as i32 {
                continue;
            }

            // 8x16 tiles always use adjacent tile indices. If we're in 8x16
            // mode and this sprite needs the second tile, add 1 to the tile
            // index and change yoff so it looks like we're rendering that tile
            if ysize == 16 {
                tile &= 0xfe; // ignore the lowest bit
                if line - yoff >= 8 {
                    tile |= 1;
                    yoff += 8;
                }
            }

            // 160px/line, 4 entries/px
            let mut coff = (WIDTH as i32 * line + xoff) * 4;

            // All sprite tile palettes are at 0x8000-0x8fff => start of vram.
            // If we're in CGB mode, then we get our palette from the spite
            // flags. We also need to take into account the tile being in a
            // different bank. Otherwise, we just use the tile index as a raw
            // index.
            let pal;
            let tiled;
            if self.is_cgb {
                tiled = self.tiles.data[((flags as usize >> 3) & 1 * NUM_TILES) + tile as usize];
                pal = self.cgb.cobp[(flags & 0x3) as usize];
            } else {
                // bit4 is the palette number. 0 = obp0, 1 = obp1
                pal = if flags & 0x10 != 0 {
                    self.pal.obp1
                } else {
                    self.pal.obp0
                };
                tiled = self.tiles.data[tile as usize];
            }

            // bit6 is the vertical flip bit
            let row = if flags & 0x40 != 0 {
                tiled[(7 - (line - yoff)) as usize]
            } else {
                tiled[(line - yoff) as usize]
            };

            for x in 0..8 {
                coff += 4;

                // If these pixels are off screen, don't bother drawing
                // anything. Also, if the background tile at this pixel has
                // priority, don't render this sprite at all.
                if xoff + x < 0 || xoff + x >= WIDTH as i32 || scanline[(x + xoff) as usize] > 3 {
                    continue;
                }
                // bit5 is the horizontal flip flag
                let colori = row[if flags & 0x20 != 0 { 7 - x } else { x } as usize];

                // A color index of 0 for sprites means transparent
                if colori == 0 {
                    continue;
                }

                // bit7 0=OBJ Above BG, 1=OBJ Behind BG color 1-3. So if this
                // sprite has this flag set and the data at this location
                // already contains data (nonzero), then don't render this
                // sprite
                if flags & 0x80 != 0 && scanline[(xoff + x) as usize] != 0 {
                    continue;
                }

                let color;
                if self.is_sgb && !self.is_cgb {
                    let sgbaddr = ((xoff as usize + x as usize) >> 3) + (line as usize >> 3) * 20;
                    let mapped = self.sgb.atf[sgbaddr as usize];
                    match pal[colori as usize][0] {
                        0 => {
                            color = self.sgb.pal[mapped as usize][3];
                        }
                        96 => {
                            color = self.sgb.pal[mapped as usize][2];
                        }
                        192 => {
                            color = self.sgb.pal[mapped as usize][1];
                        }
                        255 => {
                            color = self.sgb.pal[mapped as usize][0];
                        }

                        // not actually reachable
                        _ => {
                            color = [0, 0, 0, 0];
                        }
                    }
                } else {
                    color = pal[colori as usize];
                }

                self.image_data[(coff - 4) as usize] = color[0];
                self.image_data[(coff - 3) as usize] = color[1];
                self.image_data[(coff - 2) as usize] = color[2];
                self.image_data[(coff - 1) as usize] = color[3];
            }
        }
    }

    pub fn rb(&self, addr: u16) -> u8 {
        match addr & 0xff {
            0x40 => {
                ((self.lcdon as u8) << 7)
                    | ((self.winmap as u8) << 6)
                    | ((self.winon as u8) << 5)
                    | ((self.tiledata as u8) << 4)
                    | ((self.bgmap as u8) << 3)
                    | ((self.objsize as u8) << 2)
                    | ((self.objon as u8) << 1)
                    | ((self.bgon as u8) << 0)
            }

            0x41 => {
                // 4
                ((self.lycly as u8) << 6)
                    | ((self.mode2int as u8) << 5)
                    | ((self.mode1int as u8) << 4)
                    | ((self.mode0int as u8) << 3)
                    | ((if self.lycly as u8 == self.ly { 1 } else { 0 } as u8) << 2)
                    | ((self.mode as u8) << 0)
            }

            0x42 => self.scy,
            0x43 => self.scx,
            0x44 => self.ly,
            0x45 => self.lyc,
            // 0x46 is DMA transfer, can't read
            0x47 => self.bgp,
            0x48 => self.obp0,
            0x49 => self.obp1,
            0x4a => self.wy,
            0x4b => self.wx,
            0x4f => self.vrambank,

            // http://nocash.emubase.de/pandocs.htm#lcdvramdmatransferscgbonly
            0x51 => (self.hdma_src >> 8) as u8,
            0x52 => self.hdma_src as u8,
            0x53 => (self.hdma_dst >> 8) as u8,
            0x54 => self.hdma_dst as u8,
            0x55 => self.hdma5,

            // http://nocash.emubase.de/pandocs.htm#lcdcolorpalettescgbonly
            0x68 => self.cgb.bgpi,
            0x69 => self.cgb.bgp[(self.cgb.bgpi & 0x3f) as usize],
            0x6a => self.cgb.obpi,
            0x6b => self.cgb.obp[(self.cgb.obpi & 0x3f) as usize],

            _ => 0xff,
        }
    }

    pub fn wb(&mut self, addr: u16, val: u8) {
        match addr & 0xff {
            0x40 => {
                let before = self.lcdon;
                self.lcdon = (val >> 7) & 1 != 0;
                self.winmap = (val >> 6) & 1 != 0;
                self.winon = (val >> 5) & 1 != 0;
                self.tiledata = (val >> 4) & 1 != 0;
                self.bgmap = (val >> 3) & 1 != 0;
                self.objsize = (val >> 2) & 1 != 0;
                self.objon = (val >> 1) & 1 != 0;
                self.bgon = (val >> 0) & 1 != 0;
                if !before && self.lcdon {
                    self.clock = 4; // ??? why 4?!
                    self.ly = 0;
                }
            }

            0x41 => {
                self.lycly = (val >> 6) & 1 != 0;
                self.mode2int = (val >> 5) & 1 != 0;
                self.mode1int = (val >> 4) & 1 != 0;
                self.mode0int = (val >> 3) & 1 != 0;
                // The other bits of this register are mode and lycly, but thse
                // are read-only and won't be modified
            }

            0x42 => {
                self.scy = val;
            }
            0x43 => {
                self.scx = val;
            }
            // 0x44 self.ly is read-only
            0x45 => {
                self.lyc = val;
            }
            // 0x46 handled in mem
            0x47 => {
                self.bgp = val;
                update_pal(&mut self.pal.bg, val);
            }
            0x48 => {
                self.obp0 = val;
                update_pal(&mut self.pal.obp0, val);
            }
            0x49 => {
                self.obp1 = val;
                update_pal(&mut self.pal.obp1, val);
            }
            0x4a => {
                self.wy = val;
            }
            0x4b => {
                self.wx = val;
            }
            0x4f => {
                // if self.is_cgb {
                self.vrambank = val & 1;
                // }
            }

            // http://nocash.emubase.de/pandocs.htm#lcdvramdmatransferscgbonly
            0x51 => {
                self.hdma_src = (self.hdma_src & 0x00ff) | ((val as u16) << 8);
            }
            0x52 => {
                self.hdma_src = (self.hdma_src & 0xff00) | (val as u16);
            }
            0x53 => {
                self.hdma_dst = (self.hdma_dst & 0x00ff) | ((val as u16) << 8);
            }
            0x54 => {
                self.hdma_dst = (self.hdma_dst & 0xff00) | (val as u16);
            }
            // 0x55 handled in mem

            // http://nocash.emubase.de/pandocs.htm#lcdcolorpalettescgbonly
            //
            // The two indices/palette memories work the same way. The index's
            // lower 6 bits are the actual index, and bit 7 indicates that the
            // index should be automatically incremented whenever this memory is
            // written to. When dealing with the index, make sure to mask out
            // bit 6.
            0x68 => {
                self.cgb.bgpi = val & 0xbf;
            }
            0x6a => {
                self.cgb.obpi = val & 0xbf;
            }
            0x69 => {
                let cgb = &mut *self.cgb;
                cgb.bgp[(cgb.bgpi & 0x3f) as usize] = val;
                update_cgb_pal(&mut cgb.cbgp, &cgb.bgp, cgb.bgpi);
                if cgb.bgpi & 0x80 != 0 {
                    cgb.bgpi = (cgb.bgpi + 1) & 0xbf;
                }
            }
            0x6b => {
                let cgb = &mut *self.cgb;
                cgb.obp[(cgb.obpi & 0x3f) as usize] = val;
                update_cgb_pal(&mut cgb.cobp, &cgb.obp, cgb.obpi);
                if cgb.obpi & 0x80 != 0 {
                    cgb.obpi = (cgb.obpi + 1) & 0xbf;
                }
            }

            _ => {}
        }
    }

    // Register that a tile needs to be updated
    pub fn update_tile(&mut self, addr: u16) {
        let tilei = (addr & 0x1fff) / 16; // each tile is 16 bytes, divide by 16
        let tilei = tilei + (self.vrambank as u16) * (NUM_TILES as u16);
        self.tiles.need_update = true;
        self.tiles.to_update[tilei as usize] = true;
    }

    // Trigger a DMA transfer into OAM. This happens whenever something is
    // written to 0xff46. See
    // http://nocash.emubase.de/pandocs.htm#lcdoamdmatransfers for the
    // specifics, but the gist is that the value written to this memory is the
    // upper byte of the addresses which should be copied over into OAM.
    pub fn oam_dma_transfer(mem: &mut Mmu, val: u8) {
        // DMA transfer moves data in regular ram to OAM. It's triggered when
        // writing to a specific address in memory. Here's what happens:
        //
        //      Source:      XX00-XX9F   ;XX in range from 00-F1h
        //      Destination: FE00-FE9F
        let orval = (val as u16) << 8;
        if orval > 0xf100 {
            return;
        }

        for i in 0..OAM_SIZE as u16 {
            mem.gpu.oam[i as usize] = mem.rb(orval | i);
        }
    }

    // When in CGB mode, this triggers a DMA transfer to VRAM. For more info,
    // see http://nocash.emubase.de/pandocs.htm#lcdvramdmatransferscgbonly
    pub fn hdma_dma_transfer(mem: &mut Mmu, _val: u8) {
        let src = mem.gpu.hdma_src & 0xfff0;
        let dst = mem.gpu.hdma_dst & 0x1ff0;

        if (src > 0x7ff0 && src < 0xa000) || src > 0xdff0 || dst < 0x8000 || dst > 0x9ff0 {
            return;
        }
    }
}

// Update the cached palettes for BG/OBP0/OBP1. This should be called whenever
// these registers are modified
fn update_pal(pal: &mut [Color; 4], val: u8) {
    // These registers are indices into the actual palette. See
    // http://nocash.emubase.de/pandocs.htm#lcdmonochromepalettes
    pal[0] = PALETTE[((val >> 0) & 0x3) as usize];
    pal[1] = PALETTE[((val >> 2) & 0x3) as usize];
    pal[2] = PALETTE[((val >> 4) & 0x3) as usize];
    pal[3] = PALETTE[((val >> 6) & 0x3) as usize];
}

// Update the cached CGB palette that was just written to
fn update_cgb_pal(pal: &mut [[Color; 4]; 8], mem: &[u8; CGB_BP_SIZE], addr: u8) {
    // See http://nocash.emubase.de/pandocs.htm#lcdcolorpalettescgbonly
    let addr = addr & 0x3f; // mask off the auto-increment bit
    let pali = addr / 8; // divide by 8 (size of one palette)
    let colori = (addr % 8) / 2; // 2 bytes per color, divide by 2

    let byte1 = mem[(addr & 0x3e) as usize];
    let byte2 = mem[((addr & 0x3e) + 1) as usize];

    let color = &mut pal[pali as usize][colori as usize];

    // Bits 0-7 in byte1, others in byte2
    //  Bit 0-4   Red Intensity   (00-1F)
    //  Bit 5-9   Green Intensity (00-1F)
    //  Bit 10-14 Blue Intensity  (00-1F)
    color[0] = (byte1 & 0x1f) << 3;
    color[1] = ((byte1 >> 5) | ((byte2 & 0x3) << 3)) << 3;
    color[2] = ((byte2 >> 2) & 0x1f) << 3;
    color[3] = 255;
}
