> ### Gameboy emulator written in Rust to WebAssembly
>
> Disclaimer: This project doesn't endorse or promote any type of piracy activities. The act of build or install a emulator isn't an illegal activity. As many others emulators written, this *project is a study case*.
>
> `...` 

<img src="resources/internals-cpu-section.gif" alt="GameBoy Internals" width="600px" />

* (c) Jeff Frohwein Source: http://www.devrs.com/gb/hardware.php#hardgb

<img src="resources/dmg-main-board-schematic-circuit.png" alt="GameBoy Internals" width="600px" />

* A scan of the main logic board for the DMG 01: https://chipmusic.org/forums/topic/13608/dmg-main-board-schematic-circuit-arduinoboy/

## Emulators and a bit of Game Boy history...

An emulator is often simulating physical hardware and electronics in software, and in the case of the Game Boy most of the work involves dealing with 8-bit buses around (a variant of) the Z80 CPU. The Game Boy CPU is a hybrid between the Intel 8080 and the Zilog Z80. In computing, an emulator is hardware or software that enables one computer system (called the host) to behave like another computer system (called the guest). An emulator typically enables the host system to run software or use peripheral devices designed for the guest system. Emulation refers to the ability of a computer program in an electronic device to emulate (or imitate) another program or device.

Back to Z80...

<img src="resources/zilog-Z80.jpg" alt="Zilog Z80" width="220px" />

The Z80 is an 8-bit microprocessor introduced by Zilog as the startup company's first product. The Z80 was conceived by Federico Faggin in late 1974 and developed by him and his 11 employees starting in early 1975. The first working samples were delivered in March 1976, and it was officially introduced on the market in July 1976. With the revenue from the Z80, the company built its own chip factories and grew to over a thousand employees over the following two years.

That means manipulating a lot of individual bytes, especially while navigating through huge banks of ROM and RAM. The Game Boy is a pretty simple architecture — getting button input requires reading specific memory addresses, writing pixels to the screen involves pushing bytes to specific places in VRAM.

The Game Boy has four operation buttons labeled _"A"_, _"B"_, _"SELECT"_, and _"START"_, and a _directional pad (d-pad)_. There is a volume control dial on the right side of the device and a similar dial on the left side to adjust the contrast. At the top of the Game Boy, a sliding on-off switch and the slot for the Game Boy cartridges are located. The on-off switch includes a physical lockout to prevent users from either inserting or removing a cartridge while the unit is switched on. Nintendo recommends users leave a cartridge in the slot to prevent dust and dirt from entering the system.

The Game Boy contains optional input or output connectors. On the left side of the system is an external 3.5 mm × 1.35 mm DC power supply jack that allows users to use an external rechargeable battery pack or AC adapter (sold separately) instead of four AA batteries. The Game Boy requires 6 V DC of at least 150 mA. A 3.5 mm stereo headphone jack is located on the bottom side of the unit which allows users to listen to the audio with the bundled headphones or external speakers.

The right side of the device offers a port that allows a user to connect to another Game Boy system via a link cable, provided both users are playing games that support connecting to each other (most often, only copies of the same game, although for example, the Pokémon games can connect between different generations). The port can also be used to connect a Game Boy Printer. The link cable was originally designed for players to play head-to-head two-player games such as in Tetris. However, game developer Satoshi Tajiri later used the link cable technology as a method of communication and networking in the popular Pokémon video game series.

Anyway, enough about the Game Boy history.

The Z80 was designed to be binary compatible with the already existing Intel 8080. This means that the instruction set found in the 8080 was also implemented by the Z80 (in essence, the 8080 can be seen as a subset of the Z80). The Game Boy’s custom hybrid chip official name is *Sharp LR35902*.

## LR35902 ~ High level architecture

##### CPU (`src/cpu.rs`)

A central processing unit (CPU), also called a central processor, main processor or just processor, is the electronic circuitry that executes instructions comprising a computer program. The CPU performs basic arithmetic, logic, controlling, and input/output (I/O) operations specified by the instructions in the program. This contrasts with external components such as main memory and I/O circuitry, and specialized processors such as graphics processing units (GPUs). 

The Game Boy CPU is composed of 8 different "registers". Registers are responsible for holding on to little pieces of data that the CPU can manipulate when it executes various instructions. The Game Boy's CPU is an 8-Bit CPU, meaning that each of its registers can hold 8 bits (_1 byte_) of data. The CPU has 8 different registers labled as `a`, `b`, `c`, `d`, `e`, `f`, `h`, `l`. 

Example below:

```rust
// Note the usage of the type u8 for our registers. u8 are 8-bit unsigned integers.

#[derive(Copy, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8
}
```

##### GPU (`src/gpu.rs`)

##### RAM

##### ROM


The screen resolution of the original Game Boy [is 160×144 pixels]()

<img src="resources/LR35902.jpg" alt="LR35902" width="600px" />

# Running

Install `wasm-pack`:

```zsh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Run this command:

```zsh
make start
```

# TODO

- [x] `./target/release/LR35902` flags
    - [x] `--browser` runs in the browser with WASM (default)
    - [x] `--desktop` runs in a desktop window
- [ ] `--desktop`
    - [ ] Create desktop window
- [ ] `--browser`
    - [ ] Create server using port `8888`
- [ ] Audio

# Resources & References

- https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
- http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
- http://emubook.emulation64.com/
- https://github.com/jawline/Mimic
- https://www.youtube.com/watch?v=LqcEg3IVziQ
- https://en.wikipedia.org/wiki/Zilog_Z80
- https://en.wikipedia.org/wiki/Game_Boy
- https://github.com/mvdnes/rboy
- https://github.com/alexcrichton/jba/tree/rust
- https://medium.com/@andrewimm/writing-a-game-boy-emulator-in-wasm-part-1-1ba023eb2c7c
- https://github.com/yodalee/ruGameboy
- https://www.youtube.com/watch?v=LqcEg3IVziQ
- https://realboyemulator.wordpress.com/2013/01/01/the-nintendo-game-boy-1/
- https://gbdev.gg8.se/wiki/articles/DMG_Schematics
- https://chipmusic.org/forums/topic/13608/dmg-main-board-schematic-circuit-arduinoboy/
- https://github.com/torch2424/wasmboy/
- https://rylev.github.io/DMG-01/public/book/introduction.html