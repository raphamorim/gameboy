use blip_buf::BlipBuf;

const WAVE_PATTERN: [[i32; 8]; 4] = [
    [-1, -1, -1, -1, 1, -1, -1, -1],
    [-1, -1, -1, -1, 1, 1, -1, -1],
    [-1, -1, 1, 1, 1, 1, -1, -1],
    [1, 1, 1, 1, -1, -1, 1, 1],
];
// Game Boy CPU runs at 4.194304 MHz
const CLOCKS_PER_SECOND: u32 = 4194304;
const CLOCKS_PER_FRAME: u32 = CLOCKS_PER_SECOND / 512;
const OUTPUT_SAMPLE_COUNT: usize = 2000;
const OUTPUT_SAMPLE_COUNT_U32: u32 = 2000;
const SAMPLE_RATE: u32 = 44100;

pub trait AudioPlayer: Send {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]);
    fn samples_rate(&self) -> u32;
    fn underflowed(&self) -> bool;
}

struct VolumeEnvelope {
    period: u8,
    goes_up: bool,
    delay: u8,
    initial_volume: u8,
    volume: u8,
}

impl VolumeEnvelope {
    fn new() -> VolumeEnvelope {
        VolumeEnvelope {
            period: 0,
            goes_up: false,
            delay: 0,
            initial_volume: 0,
            volume: 0,
        }
    }

    fn rb(&self, a: u16) -> u8 {
        match a {
            0xFF12 | 0xFF17 | 0xFF21 => {
                ((self.initial_volume & 0xF) << 4)
                    | if self.goes_up { 0x08 } else { 0 }
                    | (self.period & 0x7)
            }
            _ => unimplemented!(),
        }
    }

    fn wb(&mut self, a: u16, v: u8) {
        match a {
            0xFF12 | 0xFF17 | 0xFF21 => {
                self.period = v & 0x7;
                self.goes_up = v & 0x8 == 0x8;
                self.initial_volume = v >> 4;
                self.volume = self.initial_volume;
            }
            0xFF14 | 0xFF19 | 0xFF23 if v & 0x80 == 0x80 => {
                self.delay = self.period;
                self.volume = self.initial_volume;
            }
            _ => (),
        }
    }

    fn step(&mut self) {
        if self.delay > 1 {
            self.delay -= 1;
        } else if self.delay == 1 {
            self.delay = self.period;
            if self.goes_up && self.volume < 15 {
                self.volume += 1;
            } else if !self.goes_up && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }
}

struct LengthCounter {
    enabled: bool,
    value: u16,
    max: u16,
}

impl LengthCounter {
    fn new(max: u16) -> Self {
        LengthCounter {
            enabled: false,
            value: 0,
            max,
        }
    }

    fn is_active(&self) -> bool {
        self.value > 0
    }

    fn rb(&self, a: u16) -> u8 {
        match a {
            0xFF11 | 0xFF16 | 0xFF20 => (self.max - self.value) as u8,
            0xFF1B => (256 - self.value) as u8,
            _ => unimplemented!(),
        }
    }

    fn wb(&mut self, a: u16, v: u8) {
        match a {
            0xFF11 | 0xFF16 | 0xFF20 => self.value = self.max - (v as u16),
            0xFF1B => self.value = 256 - (v as u16),
            0xFF14 | 0xFF19 | 0xFF1E | 0xFF23 => {
                self.enabled = v & 0x40 == 0x40;
                if v & 0x80 == 0x80 && self.value == 0 {
                    self.value = self.max;
                }
            }
            _ => (),
        }
    }

    fn step(&mut self) {
        if self.enabled && self.value > 0 {
            self.value -= 1;
        }
    }
}

struct SquareChannel {
    enabled: bool,
    dac_enabled: bool,
    volume: VolumeEnvelope,
    length: LengthCounter,
    duty: u8,
    phase: u8,
    freq: u16,
    period: u32,
    delay: u32,
    sweep_period: u8,
    sweep_shift: u8,
    sweep_decrease: bool,
    sweep_delay: u8,
    sweep_freq: u16,
    sweep_enabled: bool,
    blip: BlipBuf,
    last_amp: i32,
}

impl SquareChannel {
    fn new(blip: BlipBuf) -> SquareChannel {
        SquareChannel {
            enabled: false,
            dac_enabled: false,
            volume: VolumeEnvelope::new(),
            length: LengthCounter::new(64),
            duty: 1,
            phase: 1,
            freq: 0,
            period: 2048,
            delay: 0,
            sweep_period: 0,
            sweep_shift: 0,
            sweep_decrease: false,
            sweep_delay: 0,
            sweep_freq: 0,
            sweep_enabled: false,
            blip,
            last_amp: 0,
        }
    }

    fn calculate_period(&mut self) {
        if self.freq > 2047 {
            self.period = 0;
        } else {
            self.period = (2048 - self.freq as u32) * 4;
        }
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        
        if !self.enabled || !self.length.is_active() || self.period == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            let pattern = WAVE_PATTERN[self.duty as usize];
            let vol = self.volume.volume as i32;

            while time < end_time {
                let amp = vol * pattern[self.phase as usize];
                if amp != self.last_amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }
                time += self.period;
                self.phase = (self.phase + 1) % 8;
            }

            self.delay = time - end_time;
        }
    }

    fn trigger(&mut self) {
        self.enabled = true;
        self.calculate_period();
        if self.sweep_period != 0 || self.sweep_shift != 0 {
            self.sweep_enabled = true;
            self.sweep_freq = self.freq;
            self.sweep_delay = if self.sweep_period == 0 {
                8
            } else {
                self.sweep_period
            };
            if self.sweep_shift != 0 {
                let _ = self.calc_sweep_freq();
            }
        }
    }

    fn calc_sweep_freq(&mut self) -> u16 {
        let offset = self.sweep_freq >> self.sweep_shift;
        if self.sweep_decrease {
            self.sweep_freq - offset
        } else {
            let new_freq = self.sweep_freq + offset;
            if new_freq > 2047 {
                self.enabled = false;
            }
            new_freq
        }
    }

    fn sweep_step(&mut self) {
        if !self.sweep_enabled {
            return;
        }
        if self.sweep_delay > 1 {
            self.sweep_delay -= 1;
        } else {
            self.sweep_delay = if self.sweep_period == 0 {
                8
            } else {
                self.sweep_period
            };
            if self.sweep_period != 0 {
                let new_freq = self.calc_sweep_freq();
                if new_freq < 2048 && self.sweep_shift != 0 {
                    self.sweep_freq = new_freq;
                    self.freq = new_freq;
                    self.calculate_period();
                    let _ = self.calc_sweep_freq();
                }
            }
        }
    }
}

struct WaveChannel {
    enabled: bool,
    dac_enabled: bool,
    length: LengthCounter,
    freq: u16,
    period: u32,
    delay: u32,
    volume_shift: u8,
    waveram: [u8; 16],
    wave_pos: u8,
    blip: BlipBuf,
    last_amp: i32,
}

impl WaveChannel {
    fn new(blip: BlipBuf) -> WaveChannel {
        WaveChannel {
            enabled: false,
            dac_enabled: false,
            length: LengthCounter::new(256),
            freq: 0,
            period: 2048,
            delay: 0,
            volume_shift: 0,
            waveram: [
                0x84, 0x40, 0x43, 0xAA, 0x2D, 0x78, 0x92, 0x3C,
                0x60, 0x59, 0x59, 0xB0, 0x34, 0xB8, 0x2E, 0xDA
            ], // Default wave pattern
            wave_pos: 0,
            blip,
            last_amp: 0,
        }
    }

    fn calculate_period(&mut self) {
        if self.freq > 2047 {
            self.period = 0;
        } else {
            self.period = (2048 - self.freq as u32) * 2;
        }
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || !self.length.is_active() || !self.dac_enabled || self.period == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            
            // Volume shift for wave channel (output at 4x amplitude)
            let volshift = match self.volume_shift {
                0 => 4 + 2,  // Mute
                1 => 0,      // 100%
                2 => 1,      // 50%
                3 => 2,      // 25%
                _ => 4 + 2,
            };

            while time < end_time {
                let byte = self.waveram[self.wave_pos as usize / 2];
                let sample = if self.wave_pos & 1 == 0 {
                    byte >> 4
                } else {
                    byte & 0xF
                };
                
                // Shift left by 2 for 4x amplitude
                let amp = ((sample << 2) >> volshift) as i32;
                
                if amp != self.last_amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }
                
                time += self.period;
                self.wave_pos = (self.wave_pos + 1) % 32;
            }
            
            self.delay = time - end_time;
        }
    }

    fn trigger(&mut self) {
        self.enabled = true;
        self.wave_pos = 0;
        self.calculate_period();
        self.delay = 6;  // Wave channel has initial delay
    }
}

struct NoiseChannel {
    enabled: bool,
    dac_enabled: bool,
    volume: VolumeEnvelope,
    length: LengthCounter,
    shift: u8,
    width_mode: bool,
    divisor: u8,
    lfsr: u16,
    period: u32,
    delay: u32,
    blip: BlipBuf,
    last_amp: i32,
}

impl NoiseChannel {
    fn new(blip: BlipBuf) -> NoiseChannel {
        NoiseChannel {
            enabled: false,
            dac_enabled: false,
            volume: VolumeEnvelope::new(),
            length: LengthCounter::new(64),
            shift: 0,
            width_mode: false,
            divisor: 0,
            lfsr: 0x7FFF,
            period: 8,
            delay: 0,
            blip,
            last_amp: 0,
        }
    }

    fn calculate_period(&mut self) {
        let divisor = if self.divisor == 0 { 8 } else { self.divisor as u32 * 16 };
        self.period = divisor << self.shift;
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || !self.length.is_active() {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            
            while time < end_time {
                // Update LFSR
                let bit = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
                self.lfsr = (self.lfsr >> 1) | (bit << 14);
                if self.width_mode {
                    self.lfsr = (self.lfsr & !0x40) | (bit << 6);
                }
                
                // Calculate amplitude based on LFSR bit 0
                let amp = if self.lfsr & 1 == 0 {
                    self.volume.volume as i32
                } else {
                    -(self.volume.volume as i32)
                };
                
                if amp != self.last_amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }
                
                time += self.period;
            }
            
            self.delay = time - end_time;
        }
    }

    fn trigger(&mut self) {
        self.enabled = true;
        self.lfsr = 0x7FFF;
        self.calculate_period();
        self.delay = 0;
    }
}

pub struct Sound {
    on: bool,
    channel1: SquareChannel,
    channel2: SquareChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    left_volume: u8,
    right_volume: u8,
    channel_to_left: u8,
    channel_to_right: u8,
    frame_seq_counter: u8,
    player: Option<Box<dyn AudioPlayer>>,
    time: u32,
    prev_time: u32,
    next_time: u32,
    output_period: u32,
}

impl Sound {
    pub fn new() -> Sound {
        let mut ch1_blip = BlipBuf::new(OUTPUT_SAMPLE_COUNT_U32 * 2);
        let mut ch2_blip = BlipBuf::new(OUTPUT_SAMPLE_COUNT_U32 * 2);
        let mut ch3_blip = BlipBuf::new(OUTPUT_SAMPLE_COUNT_U32 * 2);
        let mut ch4_blip = BlipBuf::new(OUTPUT_SAMPLE_COUNT_U32 * 2);
        ch1_blip.set_rates(CLOCKS_PER_SECOND as f64, SAMPLE_RATE as f64);
        ch2_blip.set_rates(CLOCKS_PER_SECOND as f64, SAMPLE_RATE as f64);
        ch3_blip.set_rates(CLOCKS_PER_SECOND as f64, SAMPLE_RATE as f64);
        ch4_blip.set_rates(CLOCKS_PER_SECOND as f64, SAMPLE_RATE as f64);

        let output_period = ((OUTPUT_SAMPLE_COUNT_U32 as u64 * CLOCKS_PER_SECOND as u64) / SAMPLE_RATE as u64) as u32;
        
        Sound {
            on: false,
            channel1: SquareChannel::new(ch1_blip),
            channel2: SquareChannel::new(ch2_blip),
            channel3: WaveChannel::new(ch3_blip),
            channel4: NoiseChannel::new(ch4_blip),
            left_volume: 7,  // Max volume
            right_volume: 7, // Max volume
            channel_to_left: 0xFF,  // All channels to left
            channel_to_right: 0xFF, // All channels to right
            frame_seq_counter: 0,
            player: None,
            time: 0,
            prev_time: 0,
            next_time: CLOCKS_PER_FRAME,
            output_period,
        }
    }

    pub fn set_player(&mut self, player: Box<dyn AudioPlayer>) {
        self.player = Some(player);
        let sample_rate = self.player.as_ref().unwrap().samples_rate();
        self.channel1
            .blip
            .set_rates(CLOCKS_PER_SECOND as f64, sample_rate as f64);
        self.channel2
            .blip
            .set_rates(CLOCKS_PER_SECOND as f64, sample_rate as f64);
        self.channel3
            .blip
            .set_rates(CLOCKS_PER_SECOND as f64, sample_rate as f64);
        self.channel4
            .blip
            .set_rates(CLOCKS_PER_SECOND as f64, sample_rate as f64);
        
        // Recalculate output period based on actual sample rate
        self.output_period = ((OUTPUT_SAMPLE_COUNT_U32 as u64 * CLOCKS_PER_SECOND as u64) / sample_rate as u64) as u32;
    }

    pub fn do_cycle(&mut self, cycles: u32) {
        if !self.on {
            return;
        }

        if self.player.is_none() {
            return;
        }

        self.time += cycles;

        if self.time >= self.output_period {
            self.do_output();
        }
    }

    fn do_output(&mut self) {
        self.run();
        
        // End frame for all channels
        self.channel1.blip.end_frame(self.time);
        self.channel2.blip.end_frame(self.time);
        self.channel3.blip.end_frame(self.time);
        self.channel4.blip.end_frame(self.time);
        
        // Adjust next_time before resetting time
        self.next_time -= self.time;
        self.time = 0;
        self.prev_time = 0;
        
        self.play_samples();
    }

    fn run(&mut self) {
        // Run frame sequencer
        while self.next_time <= self.time {
            // Run channels up to frame boundary
            self.channel1.run(self.prev_time, self.next_time);
            self.channel2.run(self.prev_time, self.next_time);
            self.channel3.run(self.prev_time, self.next_time);
            self.channel4.run(self.prev_time, self.next_time);

            if self.frame_seq_counter % 2 == 0 {
                self.channel1.length.step();
                self.channel2.length.step();
                self.channel3.length.step();
                self.channel4.length.step();
            }
            if self.frame_seq_counter % 4 == 2 {
                self.channel1.sweep_step();
            }
            if self.frame_seq_counter == 7 {
                self.channel1.volume.step();
                self.channel2.volume.step();
                self.channel4.volume.step();
            }

            self.frame_seq_counter = (self.frame_seq_counter + 1) % 8;
            self.prev_time = self.next_time;
            self.next_time += CLOCKS_PER_FRAME;
        }
        
        // Run remaining time
        if self.prev_time != self.time {
            self.channel1.run(self.prev_time, self.time);
            self.channel2.run(self.prev_time, self.time);
            self.channel3.run(self.prev_time, self.time);
            self.channel4.run(self.prev_time, self.time);
            self.prev_time = self.time;
        }
    }

    fn play_samples(&mut self) {
        
        // Check if any channel has samples available
        let sample_count = self
            .channel1
            .blip
            .samples_avail()
            .min(self.channel2.blip.samples_avail())
            .min(self.channel3.blip.samples_avail())
            .min(self.channel4.blip.samples_avail()) as usize;

        if sample_count < OUTPUT_SAMPLE_COUNT {
            return;
        }

        let mut left_buf = vec![0f32; OUTPUT_SAMPLE_COUNT];
        let mut right_buf = vec![0f32; OUTPUT_SAMPLE_COUNT];

        let mut ch1_buf = vec![0i16; OUTPUT_SAMPLE_COUNT];
        let mut ch2_buf = vec![0i16; OUTPUT_SAMPLE_COUNT];
        let mut ch3_buf = vec![0i16; OUTPUT_SAMPLE_COUNT];
        let mut ch4_buf = vec![0i16; OUTPUT_SAMPLE_COUNT];

        self.channel1.blip.read_samples(&mut ch1_buf[..], false);
        self.channel2.blip.read_samples(&mut ch2_buf[..], false);
        self.channel3.blip.read_samples(&mut ch3_buf[..], false);
        self.channel4.blip.read_samples(&mut ch4_buf[..], false);

        let left_vol = (self.left_volume as f32 / 7.0) * (1.0 / 15.0) * 0.25;
        let right_vol = (self.right_volume as f32 / 7.0) * (1.0 / 15.0) * 0.25;

        for i in 0..OUTPUT_SAMPLE_COUNT {
            let mut left = 0.0f32;
            let mut right = 0.0f32;

            if self.channel_to_left & 0x01 != 0 {
                left += ch1_buf[i] as f32 * left_vol;
            }
            if self.channel_to_right & 0x01 != 0 {
                right += ch1_buf[i] as f32 * right_vol;
            }
            if self.channel_to_left & 0x02 != 0 {
                left += ch2_buf[i] as f32 * left_vol;
            }
            if self.channel_to_right & 0x02 != 0 {
                right += ch2_buf[i] as f32 * right_vol;
            }
            if self.channel_to_left & 0x04 != 0 {
                left += (ch3_buf[i] as f32 / 4.0) * left_vol;  // Wave channel is 4x amplitude
            }
            if self.channel_to_right & 0x04 != 0 {
                right += (ch3_buf[i] as f32 / 4.0) * right_vol;  // Wave channel is 4x amplitude
            }
            if self.channel_to_left & 0x08 != 0 {
                left += ch4_buf[i] as f32 * left_vol;
            }
            if self.channel_to_right & 0x08 != 0 {
                right += ch4_buf[i] as f32 * right_vol;
            }

            left_buf[i] = left;
            right_buf[i] = right;
        }

        if let Some(ref mut player) = self.player {
            player.play(&left_buf, &right_buf);
        }
    }

    pub fn rb(&self, a: u16) -> u8 {
        match a {
            0xFF10 => {
                0x80 | (self.channel1.sweep_period << 4)
                    | (if self.channel1.sweep_decrease {
                        0x08
                    } else {
                        0
                    })
                    | self.channel1.sweep_shift
            }
            0xFF11 => (self.channel1.duty << 6) | self.channel1.length.rb(a),
            0xFF12 => self.channel1.volume.rb(a),
            0xFF13 => 0xFF,
            0xFF14 => {
                0xBF | if self.channel1.length.enabled {
                    0x40
                } else {
                    0
                }
            }
            0xFF16 => (self.channel2.duty << 6) | self.channel2.length.rb(a),
            0xFF17 => self.channel2.volume.rb(a),
            0xFF18 => 0xFF,
            0xFF19 => {
                0xBF | if self.channel2.length.enabled {
                    0x40
                } else {
                    0
                }
            }
            // Wave channel
            0xFF1A => (if self.channel3.dac_enabled { 0x80 } else { 0 }) | 0x7F,
            0xFF1B => self.channel3.length.rb(a),
            0xFF1C => 0x9F | (self.channel3.volume_shift << 5),
            0xFF1D => 0xFF,
            0xFF1E => 0xBF | if self.channel3.length.enabled { 0x40 } else { 0 },
            // Noise channel
            0xFF20 => self.channel4.length.rb(a),
            0xFF21 => self.channel4.volume.rb(a),
            0xFF22 => {
                (self.channel4.shift << 4)
                    | if self.channel4.width_mode { 0x08 } else { 0 }
                    | (self.channel4.divisor & 0x07)
            }
            0xFF23 => 0xBF | if self.channel4.length.enabled { 0x40 } else { 0 },
            0xFF24 => (self.left_volume << 4) | self.right_volume,
            0xFF25 => self.channel_to_left << 4 | self.channel_to_right,
            0xFF26 => {
                (if self.on { 0x80 } else { 0 })
                    | 0x70
                    | if self.channel1.enabled && self.channel1.length.is_active() {
                        0x01
                    } else {
                        0
                    }
                    | if self.channel2.enabled && self.channel2.length.is_active() {
                        0x02
                    } else {
                        0
                    }
                    | if self.channel3.enabled && self.channel3.length.is_active() {
                        0x04
                    } else {
                        0
                    }
                    | if self.channel4.enabled && self.channel4.length.is_active() {
                        0x08
                    } else {
                        0
                    }
            }
            // Wave RAM
            0xFF30..=0xFF3F => {
                if !self.channel3.enabled {
                    self.channel3.waveram[a as usize - 0xFF30]
                } else {
                    // Return current sample when playing
                    self.channel3.waveram[self.channel3.wave_pos as usize / 2]
                }
            }
            _ => 0xFF,
        }
    }

    pub fn wb(&mut self, a: u16, v: u8) {
        if !self.on && a != 0xFF26 {
            return;
        }

        // Debug: log sound register writes
        if a == 0xFF26 && v & 0x80 == 0x80 {
            println!("Sound enabled");
        }

        match a {
            0xFF10 => {
                self.channel1.sweep_period = (v >> 4) & 0x7;
                self.channel1.sweep_decrease = v & 0x08 == 0x08;
                self.channel1.sweep_shift = v & 0x7;
            }
            0xFF11 => {
                self.channel1.duty = v >> 6;
                self.channel1.length.wb(a, v);
            }
            0xFF12 => {
                self.channel1.volume.wb(a, v);
                self.channel1.dac_enabled = v & 0xF8 != 0;
                self.channel1.enabled = self.channel1.enabled && self.channel1.dac_enabled;
            }
            0xFF13 => {
                self.channel1.freq = (self.channel1.freq & 0x700) | (v as u16);
                self.channel1.calculate_period();
            }
            0xFF14 => {
                self.channel1.freq =
                    (self.channel1.freq & 0xFF) | (((v & 0x7) as u16) << 8);
                self.channel1.calculate_period();
                self.channel1.length.wb(a, v);
                self.channel1.volume.wb(a, v);
                if v & 0x80 == 0x80 {
                    self.channel1.trigger();
                }
            }
            0xFF16 => {
                self.channel2.duty = v >> 6;
                self.channel2.length.wb(a, v);
            }
            0xFF17 => {
                self.channel2.volume.wb(a, v);
                self.channel2.dac_enabled = v & 0xF8 != 0;
                self.channel2.enabled = self.channel2.enabled && self.channel2.dac_enabled;
            }
            0xFF18 => {
                self.channel2.freq = (self.channel2.freq & 0x700) | (v as u16);
                self.channel2.calculate_period();
            }
            0xFF19 => {
                self.channel2.freq =
                    (self.channel2.freq & 0xFF) | (((v & 0x7) as u16) << 8);
                self.channel2.calculate_period();
                self.channel2.length.wb(a, v);
                self.channel2.volume.wb(a, v);
                if v & 0x80 == 0x80 {
                    self.channel2.trigger();
                }
            }
            // Wave channel
            0xFF1A => self.channel3.dac_enabled = v & 0x80 == 0x80,
            0xFF1B => self.channel3.length.wb(a, v),
            0xFF1C => self.channel3.volume_shift = (v >> 5) & 0x3,
            0xFF1D => {
                self.channel3.freq = (self.channel3.freq & 0x700) | (v as u16);
                self.channel3.calculate_period();
            }
            0xFF1E => {
                self.channel3.freq = (self.channel3.freq & 0xFF) | (((v & 0x7) as u16) << 8);
                self.channel3.calculate_period();
                self.channel3.length.wb(a, v);
                if v & 0x80 == 0x80 {
                    self.channel3.trigger();
                }
            }
            // Noise channel
            0xFF20 => self.channel4.length.wb(a, v),
            0xFF21 => {
                self.channel4.volume.wb(a, v);
                self.channel4.dac_enabled = v & 0xF8 != 0;
                self.channel4.enabled = self.channel4.enabled && self.channel4.dac_enabled;
            }
            0xFF22 => {
                self.channel4.shift = v >> 4;
                self.channel4.width_mode = v & 0x08 == 0x08;
                self.channel4.divisor = v & 0x07;
                self.channel4.calculate_period();
            }
            0xFF23 => {
                self.channel4.length.wb(a, v);
                self.channel4.volume.wb(a, v);
                if v & 0x80 == 0x80 {
                    self.channel4.trigger();
                }
            }
            // Wave RAM
            0xFF30..=0xFF3F => {
                if !self.channel3.enabled {
                    self.channel3.waveram[a as usize - 0xFF30] = v;
                }
            }
            0xFF24 => {
                self.left_volume = (v >> 4) & 0x7;
                self.right_volume = v & 0x7;
            }
            0xFF25 => {
                self.channel_to_left = v >> 4;
                self.channel_to_right = v & 0xF;
            }
            0xFF26 => {
                let was_on = self.on;
                self.on = v & 0x80 == 0x80;
                if was_on && !self.on {
                    for i in 0xFF10..=0xFF25 {
                        self.wb(i, 0);
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod cpal_audio {
    use super::AudioPlayer;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{FromSample, Sample};
    use std::sync::{Arc, Mutex};

    pub struct CpalPlayer {
        buffer: Arc<Mutex<Vec<(f32, f32)>>>,
        sample_rate: u32,
    }

    impl CpalPlayer {
        pub fn new() -> Option<(CpalPlayer, cpal::Stream)> {
            let device = match cpal::default_host().default_output_device() {
                Some(e) => e,
                None => {
                    eprintln!("No default output device found");
                    return None;
                }
            };

            let wanted_samplerate = cpal::SampleRate(44100);
            let supported_configs = match device.supported_output_configs() {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("Failed to get supported configs: {}", err);
                    return None;
                }
            };
            let mut supported_config = None;
            for f in supported_configs {
                if f.sample_format() == cpal::SampleFormat::F32 {
                    if f.min_sample_rate() <= wanted_samplerate
                        && wanted_samplerate <= f.max_sample_rate()
                    {
                        supported_config = Some(f.with_sample_rate(wanted_samplerate));
                    } else {
                        supported_config = Some(f.with_max_sample_rate());
                    }
                    break;
                }
            }
            if supported_config.is_none() {
                eprintln!("No supported audio configuration found");
                return None;
            }

            let selected_config = supported_config.unwrap();
            let sample_format = selected_config.sample_format();
            let channels = selected_config.channels();
            let config: cpal::StreamConfig = selected_config.into();

            let err_fn =
                |err| eprintln!("An error occurred on the output audio stream: {}", err);

            let shared_buffer = Arc::new(Mutex::new(Vec::new()));
            let stream_buffer = shared_buffer.clone();

            let player = CpalPlayer {
                buffer: shared_buffer,
                sample_rate: config.sample_rate.0,
            };

            let stream = match sample_format {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        cpal_thread(data, &stream_buffer, channels as usize)
                    },
                    err_fn,
                    None,
                ),
                _ => return None,
            };

            match stream {
                Ok(s) => {
                    if let Err(e) = s.play() {
                        eprintln!("Failed to play audio stream: {}", e);
                        return None;
                    }
                    Some((player, s))
                }
                Err(e) => {
                    eprintln!("Failed to build audio stream: {}", e);
                    None
                }
            }
        }
    }

    fn cpal_thread<T: Sample + FromSample<f32>>(
        output: &mut [T],
        buffer: &Arc<Mutex<Vec<(f32, f32)>>>,
        channels: usize,
    ) {
        let mut buffer = match buffer.lock() {
            Ok(b) => b,
            Err(_) => return,
        };

        let mut idx = 0;
        for frame in output.chunks_mut(channels) {
            if idx < buffer.len() {
                let (left, right) = buffer[idx];
                // Fill all channels with stereo data
                for (i, sample) in frame.iter_mut().enumerate() {
                    *sample = T::from_sample(if i % 2 == 0 { left } else { right });
                }
                idx += 1;
            } else {
                for sample in frame.iter_mut() {
                    *sample = T::from_sample(0.0);
                }
            }
        }

        buffer.drain(0..idx);
    }

    impl AudioPlayer for CpalPlayer {
        fn play(&mut self, left_channel: &[f32], right_channel: &[f32]) {
            let mut buffer = match self.buffer.lock() {
                Ok(b) => b,
                Err(_) => return,
            };

            for i in 0..left_channel.len() {
                buffer.push((left_channel[i], right_channel[i]));
            }

            const MAX_BUFFER_SIZE: usize = 44100;
            if buffer.len() > MAX_BUFFER_SIZE {
                let to_remove = buffer.len() - MAX_BUFFER_SIZE;
                buffer.drain(0..to_remove);
            }
        }

        fn samples_rate(&self) -> u32 {
            self.sample_rate
        }

        fn underflowed(&self) -> bool {
            match self.buffer.lock() {
                Ok(b) => b.is_empty(),
                Err(_) => true,
            }
        }
    }
}
