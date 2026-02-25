// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::memory::Memory;

const DUTY_CYCLES: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
    [1, 0, 0, 0, 0, 0, 0, 1], // 25%
    [1, 0, 0, 0, 0, 1, 1, 1], // 50%
    [0, 1, 1, 1, 1, 1, 1, 0], // 75%
];

#[derive(Default)]
struct VolumeEnvelope {
    period: u8,
    direction: i8,
    initial_volume: u8,
    volume: u8,
    timer: u8,
    enabled: bool,
}

impl VolumeEnvelope {
    fn step(&mut self) {
        if self.period == 0 {
            return;
        }
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.timer = if self.period == 0 { 8 } else { self.period };
            let new_volume = self.volume as i8 + self.direction;
            if new_volume >= 0 && new_volume <= 15 {
                self.volume = new_volume as u8;
            } else {
                self.enabled = false;
            }
        }
    }

    fn write(&mut self, val: u8) {
        self.initial_volume = val >> 4;
        self.direction = if (val & 0x08) != 0 { 1 } else { -1 };
        self.period = val & 0x07;
        self.volume = self.initial_volume;
    }

    fn read(&self) -> u8 {
        (self.initial_volume << 4) | (if self.direction == 1 { 0x08 } else { 0 }) | self.period
    }

    fn trigger(&mut self) {
        self.timer = if self.period == 0 { 8 } else { self.period };
        self.volume = self.initial_volume;
        self.enabled = true;
    }
}

#[derive(Default)]
struct Sweep {
    enabled: bool,
    period: u8,
    direction: i8,
    shift: u8,
    timer: u8,
    frequency: u16,
    has_calculated: bool,
}

impl Sweep {
    fn write(&mut self, val: u8) {
        self.period = (val >> 4) & 0x07;
        self.direction = if (val & 0x08) != 0 { -1 } else { 1 };
        self.shift = val & 0x07;
    }

    fn read(&self) -> u8 {
        (self.period << 4) | (if self.direction == -1 { 0x08 } else { 0 }) | self.shift | 0x80
    }
    
    fn trigger(&mut self, freq: u16) -> bool {
        self.frequency = freq;
        self.timer = if self.period == 0 { 8 } else { self.period };
        self.enabled = self.period > 0 || self.shift > 0;
        self.has_calculated = false;
        
        let mut overflow = false;
        if self.shift > 0 {
            overflow = self.calculate_new_freq() > 2047;
        }
        overflow
    }

    fn calculate_new_freq(&mut self) -> u16 {
        self.has_calculated = true;
        let offset = self.frequency >> self.shift;
        let new_freq = if self.direction == 1 {
            self.frequency.wrapping_add(offset)
        } else {
            self.frequency.wrapping_sub(offset)
        };
        if new_freq > 2047 {
            self.enabled = false;
        }
        new_freq
    }

    fn step(&mut self) -> (Option<u16>, bool) {
        if !self.enabled {
            return (None, false);
        }
        let mut new_freq_ret = None;
        let mut overflow = false;
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.timer = if self.period == 0 { 8 } else { self.period };
            if self.period > 0 {
                let new_freq = self.calculate_new_freq();
                if new_freq > 2047 {
                    overflow = true;
                } else if self.shift > 0 {
                    self.frequency = new_freq;
                    new_freq_ret = Some(self.frequency);
                    if self.calculate_new_freq() > 2047 {
                        overflow = true;
                    }
                }
            }
        }
        (new_freq_ret, overflow)
    }
}

#[derive(Default)]
struct SquareChannel {
    enabled: bool,
    dac_enabled: bool,
    sweep: Option<Sweep>,
    envelope: VolumeEnvelope,
    length_timer: u16,
    length_enabled: bool,
    frequency: u16,
    freq_timer: u16,
    duty: usize,
    duty_pos: usize,
}

impl SquareChannel {
    fn new(has_sweep: bool) -> Self {
        Self {
            sweep: if has_sweep { Some(Sweep::default()) } else { None },
            ..Default::default()
        }
    }

    fn step_length(&mut self) {
        if self.length_enabled && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.freq_timer = (2048 - self.frequency) * 4;
        self.envelope.trigger();
        if let Some(sweep) = &mut self.sweep {
            if sweep.trigger(self.frequency) {
                self.enabled = false;
            }
        }
    }

    fn tick(&mut self) {
        if self.freq_timer > 0 {
            self.freq_timer -= 1;
        }
        if self.freq_timer == 0 {
            self.freq_timer = (2048 - self.frequency) * 4;
            self.duty_pos = (self.duty_pos + 1) % 8;
        }
    }

    fn sample(&self) -> u8 {
        if !self.enabled || !self.dac_enabled {
            return 0;
        }
        if DUTY_CYCLES[self.duty][self.duty_pos] == 1 {
            self.envelope.volume
        } else {
            0
        }
    }
}

#[derive(Default)]
struct WaveChannel {
    enabled: bool,
    dac_enabled: bool,
    length_timer: u16,
    length_enabled: bool,
    frequency: u16,
    freq_timer: u16,
    volume_shift: u8,
    wave_ram: [u8; 16],
    wave_pos: usize,
    sample_buf: u8,
}

impl WaveChannel {
    fn step_length(&mut self) {
        if self.length_enabled && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }
        if self.length_timer == 0 {
            self.length_timer = 256;
        }
        self.freq_timer = (2048 - self.frequency) * 2;
        self.wave_pos = 0;
    }

    fn tick(&mut self) {
        if self.freq_timer > 0 {
            self.freq_timer -= 1;
        }
        if self.freq_timer == 0 {
            self.freq_timer = (2048 - self.frequency) * 2;
            self.wave_pos = (self.wave_pos + 1) % 32;
            let byte = self.wave_ram[self.wave_pos / 2];
            self.sample_buf = if self.wave_pos % 2 == 0 {
                byte >> 4
            } else {
                byte & 0x0F
            };
        }
    }

    fn sample(&self) -> u8 {
        if !self.enabled || !self.dac_enabled || self.volume_shift == 0 {
            return 0;
        }
        self.sample_buf >> (self.volume_shift - 1)
    }
}

#[derive(Default)]
struct NoiseChannel {
    enabled: bool,
    dac_enabled: bool,
    envelope: VolumeEnvelope,
    length_timer: u16,
    length_enabled: bool,
    freq_timer: u16,
    lfsr: u16,
    shift: u8,
    width_mode: bool,
    divisor_code: u8,
}

impl NoiseChannel {
    fn new() -> Self {
        Self {
            lfsr: 0x7FFF,
            ..Default::default()
        }
    }

    fn step_length(&mut self) {
        if self.length_enabled && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.lfsr = 0x7FFF;
        self.freq_timer = self.divisor() << self.shift;
        self.envelope.trigger();
    }

    fn divisor(&self) -> u16 {
        match self.divisor_code {
            0 => 8,
            n => (n as u16) * 16,
        }
    }

    fn tick(&mut self) {
        if self.freq_timer > 0 {
            self.freq_timer -= 1;
        }
        if self.freq_timer == 0 {
            self.freq_timer = self.divisor() << self.shift;
            
            let xor_bit = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
            self.lfsr = (self.lfsr >> 1) | (xor_bit << 14);
            if self.width_mode {
                self.lfsr = (self.lfsr & !(1 << 6)) | (xor_bit << 6);
            }
        }
    }

    fn sample(&self) -> u8 {
        if !self.enabled || !self.dac_enabled {
            return 0;
        }
        if (self.lfsr & 1) == 0 {
            self.envelope.volume
        } else {
            0
        }
    }
}

pub struct Sound {
    enabled: bool,
    vin_left: bool,
    vin_right: bool,
    vol_left: u8,
    vol_right: u8,
    pan_left: [bool; 4],
    pan_right: [bool; 4],

    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,

    frame_seq_timer: u16,
    frame_seq_step: u8,

    sample_accumulator: u32,
    sample_rate: u32,
    audio_buffer: std::cell::RefCell<Vec<f32>>,
    
    cap_left: std::cell::Cell<f32>,
    cap_right: std::cell::Cell<f32>,
}

impl Sound {
    pub fn new() -> Self {
        Self {
            enabled: false,
            vin_left: false,
            vin_right: false,
            vol_left: 7,
            vol_right: 7,
            pan_left: [true; 4],
            pan_right: [true; 4],
            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
            ch3: WaveChannel::default(),
            ch4: NoiseChannel::new(),
            frame_seq_timer: 8192,
            frame_seq_step: 0,
            sample_accumulator: 0,
            sample_rate: 44100,
            audio_buffer: std::cell::RefCell::new(Vec::with_capacity(4096)),
            cap_left: std::cell::Cell::new(0.0),
            cap_right: std::cell::Cell::new(0.0),
        }
    }

    pub fn set_sample_rate(&mut self, rate: u32) {
        self.sample_rate = rate;
    }

    pub fn tick(&mut self, cycles: u32) {
        if !self.enabled {
            return;
        }

        for _ in 0..cycles {
            // Tick channels
            self.ch1.tick();
            self.ch2.tick();
            self.ch3.tick();
            self.ch4.tick();

            // Frame Sequencer
            self.frame_seq_timer -= 1;
            if self.frame_seq_timer == 0 {
                self.frame_seq_timer = 8192;
                self.step_frame_sequencer();
            }

            // Downsample
            self.sample_accumulator += self.sample_rate;
            if self.sample_accumulator >= 4194304 {
                self.sample_accumulator -= 4194304;
                self.generate_sample();
            }
        }
    }

    fn step_frame_sequencer(&mut self) {
        // Step 0, 2, 4, 6: Length
        if self.frame_seq_step % 2 == 0 {
            self.ch1.step_length();
            self.ch2.step_length();
            self.ch3.step_length();
            self.ch4.step_length();
        }

        // Step 2, 6: Sweep
        if self.frame_seq_step == 2 || self.frame_seq_step == 6 {
            if let Some(sweep) = &mut self.ch1.sweep {
                let (new_freq_opt, overflow) = sweep.step();
                if let Some(new_freq) = new_freq_opt {
                    self.ch1.frequency = new_freq;
                }
                if overflow {
                    self.ch1.enabled = false;
                }
            }
        }

        // Step 7: Envelope
        if self.frame_seq_step == 7 {
            self.ch1.envelope.step();
            self.ch2.envelope.step();
            self.ch4.envelope.step();
        }

        self.frame_seq_step = (self.frame_seq_step + 1) % 8;
    }

    fn generate_sample(&self) {
        let mut left = 0.0;
        let mut right = 0.0;

        let samples = [
            (self.ch1.sample() as f32 / 15.0) * 2.0 - 1.0,
            (self.ch2.sample() as f32 / 15.0) * 2.0 - 1.0,
            (self.ch3.sample() as f32 / 15.0) * 2.0 - 1.0,
            (self.ch4.sample() as f32 / 15.0) * 2.0 - 1.0,
        ];

        for i in 0..4 {
            if self.pan_left[i] { left += samples[i]; }
            if self.pan_right[i] { right += samples[i]; }
        }

        // Divide by 4 to prevent clipping (max 4 channels)
        left /= 4.0;
        right /= 4.0;

        // Apply master volume
        left *= (self.vol_left as f32 + 1.0) / 8.0;
        right *= (self.vol_right as f32 + 1.0) / 8.0;

        // Apply high-pass filter (DC Blocker)
        let r = 0.996;
        let out_left = left - self.cap_left.get();
        let out_right = right - self.cap_right.get();
        self.cap_left.set(left - out_left * r);
        self.cap_right.set(right - out_right * r);

        let mut buf = self.audio_buffer.borrow_mut();
        buf.push(out_left);
        buf.push(out_right);
    }

    pub fn get_audio_buffer(&self) -> Vec<f32> {
        let mut buf = self.audio_buffer.borrow_mut();
        let res = buf.clone();
        buf.clear();
        res
    }
}

impl Memory for Sound {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF10 => self.ch1.sweep.as_ref().unwrap().read(),
            0xFF11 => (self.ch1.duty as u8) << 6 | 0x3F,
            0xFF12 => self.ch1.envelope.read(),
            0xFF13 => 0xFF,
            0xFF14 => (if self.ch1.length_enabled { 0x40 } else { 0 }) | 0xBF,
            
            0xFF16 => (self.ch2.duty as u8) << 6 | 0x3F,
            0xFF17 => self.ch2.envelope.read(),
            0xFF18 => 0xFF,
            0xFF19 => (if self.ch2.length_enabled { 0x40 } else { 0 }) | 0xBF,
            
            0xFF1A => (if self.ch3.dac_enabled { 0x80 } else { 0 }) | 0x7F,
            0xFF1B => 0xFF,
            0xFF1C => (self.ch3.volume_shift << 5) | 0x9F,
            0xFF1D => 0xFF,
            0xFF1E => (if self.ch3.length_enabled { 0x40 } else { 0 }) | 0xBF,
            
            0xFF20 => 0xFF,
            0xFF21 => self.ch4.envelope.read(),
            0xFF22 => (self.ch4.shift << 4) | (if self.ch4.width_mode { 0x08 } else { 0 }) | self.ch4.divisor_code,
            0xFF23 => (if self.ch4.length_enabled { 0x40 } else { 0 }) | 0xBF,
            
            0xFF24 => (if self.vin_left { 0x80 } else { 0 }) | (self.vol_left << 4) | (if self.vin_right { 0x08 } else { 0 }) | self.vol_right,
            0xFF25 => {
                let mut val = 0;
                for i in 0..4 {
                    if self.pan_right[i] { val |= 1 << i; }
                    if self.pan_left[i] { val |= 1 << (i + 4); }
                }
                val
            },
            0xFF26 => {
                let mut val = 0;
                if self.enabled { val |= 0x80; }
                if self.ch4.enabled { val |= 0x08; }
                if self.ch3.enabled { val |= 0x04; }
                if self.ch2.enabled { val |= 0x02; }
                if self.ch1.enabled { val |= 0x01; }
                val | 0x70
            },
            
            0xFF30..=0xFF3F => {
                if self.ch3.enabled {
                    0xFF
                } else {
                    self.ch3.wave_ram[(address - 0xFF30) as usize]
                }
            },
            
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if !self.enabled && address != 0xFF26 && !(0xFF30..=0xFF3F).contains(&address) {
            return; // Only allow writes to NR52 or Wave RAM when APU is off
        }

        match address {
            0xFF10 => {
                if let Some(sweep) = &mut self.ch1.sweep {
                    sweep.write(value);
                }
            },
            0xFF11 => {
                self.ch1.duty = (value >> 6) as usize;
                self.ch1.length_timer = 64 - (value & 0x3F) as u16;
            },
            0xFF12 => {
                self.ch1.envelope.write(value);
                self.ch1.dac_enabled = (value & 0xF8) != 0;
                if !self.ch1.dac_enabled { self.ch1.enabled = false; }
            },
            0xFF13 => {
                self.ch1.frequency = (self.ch1.frequency & 0xFF00) | value as u16;
            },
            0xFF14 => {
                self.ch1.frequency = (self.ch1.frequency & 0x00FF) | (((value & 0x07) as u16) << 8);
                self.ch1.length_enabled = (value & 0x40) != 0;
                if value & 0x80 != 0 {
                    self.ch1.trigger();
                }
            },
            
            0xFF16 => {
                self.ch2.duty = (value >> 6) as usize;
                self.ch2.length_timer = 64 - (value & 0x3F) as u16;
            },
            0xFF17 => {
                self.ch2.envelope.write(value);
                self.ch2.dac_enabled = (value & 0xF8) != 0;
                if !self.ch2.dac_enabled { self.ch2.enabled = false; }
            },
            0xFF18 => {
                self.ch2.frequency = (self.ch2.frequency & 0xFF00) | value as u16;
            },
            0xFF19 => {
                self.ch2.frequency = (self.ch2.frequency & 0x00FF) | (((value & 0x07) as u16) << 8);
                self.ch2.length_enabled = (value & 0x40) != 0;
                if value & 0x80 != 0 {
                    self.ch2.trigger();
                }
            },
            
            0xFF1A => {
                self.ch3.dac_enabled = (value & 0x80) != 0;
                if !self.ch3.dac_enabled { self.ch3.enabled = false; }
            },
            0xFF1B => {
                self.ch3.length_timer = 256 - value as u16;
            },
            0xFF1C => {
                self.ch3.volume_shift = match (value >> 5) & 0x03 {
                    0 => 0, // Mute
                    1 => 1, // 100%
                    2 => 2, // 50%
                    3 => 3, // 25%
                    _ => 0,
                };
            },
            0xFF1D => {
                self.ch3.frequency = (self.ch3.frequency & 0xFF00) | value as u16;
            },
            0xFF1E => {
                self.ch3.frequency = (self.ch3.frequency & 0x00FF) | (((value & 0x07) as u16) << 8);
                self.ch3.length_enabled = (value & 0x40) != 0;
                if value & 0x80 != 0 {
                    self.ch3.trigger();
                }
            },
            
            0xFF20 => {
                self.ch4.length_timer = 64 - (value & 0x3F) as u16;
            },
            0xFF21 => {
                self.ch4.envelope.write(value);
                self.ch4.dac_enabled = (value & 0xF8) != 0;
                if !self.ch4.dac_enabled { self.ch4.enabled = false; }
            },
            0xFF22 => {
                self.ch4.shift = value >> 4;
                self.ch4.width_mode = (value & 0x08) != 0;
                self.ch4.divisor_code = value & 0x07;
            },
            0xFF23 => {
                self.ch4.length_enabled = (value & 0x40) != 0;
                if value & 0x80 != 0 {
                    self.ch4.trigger();
                }
            },
            
            0xFF24 => {
                self.vin_left = (value & 0x80) != 0;
                self.vol_left = (value >> 4) & 0x07;
                self.vin_right = (value & 0x08) != 0;
                self.vol_right = value & 0x07;
            },
            0xFF25 => {
                for i in 0..4 {
                    self.pan_right[i] = (value & (1 << i)) != 0;
                    self.pan_left[i] = (value & (1 << (i + 4))) != 0;
                }
            },
            0xFF26 => {
                let was_enabled = self.enabled;
                self.enabled = (value & 0x80) != 0;
                if !self.enabled {
                    // Reset all registers when turned off
                    for addr in 0xFF10..=0xFF25 {
                        self.write_byte(addr, 0x00);
                    }
                    self.ch1.enabled = false;
                    self.ch2.enabled = false;
                    self.ch3.enabled = false;
                    self.ch4.enabled = false;
                } else if !was_enabled {
                    self.frame_seq_step = 0;
                }
            },
            
            0xFF30..=0xFF3F => {
                if !self.ch3.enabled {
                    self.ch3.wave_ram[(address - 0xFF30) as usize] = value;
                }
            },
            
            _ => {},
        }
    }
}
