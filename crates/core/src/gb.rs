// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use rand::prelude::*;

use crate::cartridge::{GbMode, Mbc};
use crate::cpu::Cpu;
use crate::joypad::{Joypad, JoypadButton};
use crate::ppu::{Ppu, PpuMode};
use crate::serial::Serial;
use crate::sound::Sound;
use crate::timer::Timer;

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum GbTypes {
    Dmg,
    Mgb,
    Cgb,
    Sgb,
}

#[allow(dead_code)]
pub enum SpeedMode {
    Normal,
    Double,
}

pub struct Gb {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub serial: Serial,
    pub joypad: Joypad,
    pub sound: Sound,
    pub timer: Timer,
    pub mbc: Box<dyn Mbc>,
    pub gb_speed: u8,
    pub gb_type: GbTypes,
    pub gb_mode: GbMode,
    pub wram: Vec<u8>,
    pub hram: Vec<u8>,
    pub ram_bank: usize,
    pub ie_flag: u8,
    pub if_flag: u8,
    pub boot_rom_enabled: u8,
    pub prepare_speed_switch: bool,
    pub speed_mode: SpeedMode,
}

fn get_register_values(gb_mode: &GbMode, gb_type: &GbTypes) -> [u8; 8] {
    if gb_mode == &GbMode::DmgMode || gb_mode == &GbMode::NonCgbMode {
        match gb_type {
            GbTypes::Dmg => [0x01, 0xB0, 0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D],
            GbTypes::Mgb => [0x01, 0xB0, 0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D],
            GbTypes::Cgb => [0x11, 0xB0, 0x43, 0x00, 0x00, 0x08, 0x99, 0x1A],
            GbTypes::Sgb => [0x01, 0x00, 0x00, 0x14, 0x00, 0x00, 0xC0, 0x60],
        }
    } else {
        [0x11, 0x80, 0x00, 0x00, 0xFF, 0x56, 0x00, 0x0D]
    }
}

// Values from the Cycle-Accurate Game Boy documentation
// Pan Docs is not that detailed
fn get_div_values(gb_type: &GbTypes, gb_mode: &GbMode) -> u8 {
    let div_value = if gb_mode == &GbMode::CgbMode {
        0x1EA0
    } else {
        match gb_type {
            GbTypes::Dmg | GbTypes::Mgb => 0xABCC,
            GbTypes::Sgb => 0x0000,
            GbTypes::Cgb => 0x267C,
        }
    };
    (div_value >> 8) as u8
}

impl Gb {
    pub fn new(mbc: Box<dyn Mbc>, gb_mode: GbMode, gb_type: GbTypes) -> Self {
        let registers: [u8; 8] = get_register_values(&gb_mode, &gb_type);
        let div: u8 = get_div_values(&gb_type, &gb_mode);
        let mut wram: Vec<u8> = if gb_mode == GbMode::CgbMode {
            vec![0; 0x8000]
        } else {
            vec![0; 0x2000]
        };
        let mut hram = vec![0; 0x7F];
        let mut rng = thread_rng();
        rng.fill_bytes(&mut wram);
        rng.fill_bytes(&mut hram);

        Self {
            cpu: Cpu::new(registers),
            ppu: Ppu::new(&gb_mode),
            serial: Serial::new(),
            joypad: Joypad::new(),
            sound: Sound::new(),
            timer: Timer::new(div),
            mbc,
            gb_speed: 0,
            gb_type,
            gb_mode,
            wram,
            hram,
            ram_bank: 1,
            ie_flag: 0,
            if_flag: 0,
            boot_rom_enabled: 0,
            prepare_speed_switch: false,
            speed_mode: SpeedMode::Normal,
        }
    }

    pub fn headless_run(&mut self) {
        while !self.cpu.is_stopped {
            self.run();
        }
    }

    pub fn run(&mut self) {
        //self.debug_message();
        self.handle_interrupt();
        self.cpu_tick();
        self.components_tick();
        //self.print_serial_message();
    }

    pub fn run_one_frame(&mut self) {
        while !self.ppu.frame_ready {
            self.run();
        }
    }

    pub fn get_screen_data(&mut self) -> &[[u8; 160]; 144] {
        self.ppu.get_screen()
    }

    pub fn components_tick(&mut self) {
        self.dma_tick(self.cpu.pending_cycles as u32 * 4);
        self.ppu_tick(self.cpu.pending_cycles as u32 * 4);
        self.timer_tick(self.cpu.pending_cycles as u32 * 4);
        self.cpu.pending_cycles = 0;
    }

    fn cpu_tick(&mut self) {
        if !self.cpu.is_halted {
            self.cpu.current_instruction = self.read_byte(self.cpu.pc);
            
            if self.cpu.halt_bug {
                // For HALT bug, execute the instruction but don't increment PC
                self.decode();
                self.cpu.halt_bug = false;
                // PC increment will be skipped since halt_bug is true
            } else {
                self.decode();
            }
            
            self.cpu.pending_cycles += self.cpu.instruction_cycles;
        } else {
            // When halted, we still need to check for interrupts
            // HALT always consumes 4 T-cycles (1 M-cycle)
            self.cpu.pending_cycles = 4;
            
            // Check if we should exit HALT
            if (self.ie_flag & self.if_flag) != 0 {
                if self.cpu.interrupt_master {
                    // Normal interrupt handling will resume execution
                    self.cpu.is_halted = false;
                } else {
                    // Exit HALT but don't handle interrupt since IME=0
                    self.cpu.is_halted = false;
                }
            }
        }
    }

    fn print_serial_message(&mut self) {
        if let Some(message) = self.serial.get_serial_message() {
            println!("{}", message)
        };
    }

    fn debug_message(&self) {
        println!(
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            self.cpu.a(),
            self.cpu.f(),
            self.cpu.b(),
            self.cpu.c(),
            self.cpu.d(),
            self.cpu.e(),
            self.cpu.h(),
            self.cpu.l(),
            self.cpu.sp,
            self.cpu.pc,
            self.read_byte(self.cpu.pc),
            self.read_byte(self.cpu.pc.wrapping_add(1)),
            self.read_byte(self.cpu.pc.wrapping_add(2)),
            self.read_byte(self.cpu.pc.wrapping_add(3)),
        );
    }

    pub fn press_button(&mut self, button: JoypadButton) {
        self.joypad.press(button);
    }

    pub fn release_button(&mut self, button: JoypadButton) {
        self.joypad.release(button);
    }
}
