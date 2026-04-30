// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use rand::prelude::*;

use crate::cartridge::{GbMode, Mbc};
use crate::cpu::{opcode_size, Cpu};
use crate::joypad::{Joypad, JoypadButton};
use crate::ppu::Ppu;
use crate::serial::Serial;
use crate::sound::Sound;
use crate::timer::Timer;

pub use crate::ppu::PpuDebugSnapshot;
pub use crate::sound::{
    ApuDebugSnapshot, NoiseChannelDebugSnapshot, SquareChannelDebugSnapshot,
    WaveChannelDebugSnapshot,
};

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum GbTypes {
    Dmg,
    Mgb,
    Cgb,
    Sgb,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ModelSelection {
    #[default]
    Auto,
    Dmg,
    Cgb,
}

impl ModelSelection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Dmg => "DMG",
            Self::Cgb => "CGB",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpeedMode {
    Normal,
    Double,
}

#[derive(Clone)]
pub struct DebugSnapshot {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
    pub opcode: u8,
    pub disassembly: String,
    pub pc_bytes: [u8; 4],
    pub stack_bytes: [u8; 8],
    pub interrupt_master: bool,
    pub is_halted: bool,
    pub is_stopped: bool,
    pub pending_cycles: usize,
    pub ticks: u32,
    pub ly: u8,
    pub stat: u8,
    pub if_flag: u8,
    pub ie_flag: u8,
}

#[derive(Clone)]
pub struct MemoryWriteEvent {
    pub address: u16,
    pub value: u8,
}

#[derive(Clone)]
pub struct DisassemblyLine {
    pub address: u16,
    pub bytes: Vec<u8>,
    pub text: String,
}

#[derive(Clone)]
pub struct OamSprite {
    pub index: usize,
    pub y: u8,
    pub x: u8,
    pub tile_number: u8,
    pub attributes: u8,
}

pub enum ScreenData<'a> {
    Dmg(&'a [[u8; 160]; 144]),
    Cgb(&'a [[[u8; 3]; 160]; 144]),
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
    pub rp: u8,
    pub wram: Vec<u8>,
    pub hram: Vec<u8>,
    pub ram_bank: usize,
    pub svbk: u8,
    pub ie_flag: u8,
    pub if_flag: u8,
    pub boot_rom_enabled: u8,
    pub prepare_speed_switch: bool,
    pub speed_mode: SpeedMode,
    pub serial_stdout_enabled: bool,
    pub(crate) debug_write_log: Vec<MemoryWriteEvent>,
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
        [0x11, 0x80, 0x00, 0x00, 0x00, 0x08, 0x00, 0x7C]
    }
}

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
        let cgb_hardware = gb_type == GbTypes::Cgb;
        let mut wram: Vec<u8> = if cgb_hardware {
            vec![0; 0x8000]
        } else {
            vec![0; 0x2000]
        };
        let mut hram = vec![0; 0x7F];
        let mut rng = rand::rng();
        rng.fill_bytes(&mut wram);
        rng.fill_bytes(&mut hram);

        Self {
            cpu: Cpu::new(registers),
            ppu: Ppu::new(cgb_hardware, gb_mode == GbMode::CgbMode),
            serial: Serial::new(),
            joypad: Joypad::new(),
            sound: Sound::new(),
            timer: Timer::new(div),
            mbc,
            gb_speed: 0,
            gb_type,
            gb_mode,
            rp: if cgb_hardware { 0x3E } else { 0xFF },
            wram,
            hram,
            ram_bank: 1,
            svbk: 0,
            ie_flag: 0,
            if_flag: 0xE1,
            boot_rom_enabled: 0,
            prepare_speed_switch: false,
            speed_mode: SpeedMode::Normal,
            serial_stdout_enabled: true,
            debug_write_log: Vec::with_capacity(16),
        }
    }

    pub fn headless_run(&mut self) {
        while !self.cpu.is_stopped {
            self.run();
        }
    }

    pub fn run(&mut self) {
        //self.debug_message();
        self.debug_write_log.clear();
        self.handle_interrupt();
        self.cpu_tick();
        self.components_tick();
        self.print_serial_message();
    }

    pub fn run_one_frame(&mut self) {
        while !self.ppu.frame_ready {
            self.run();
        }
    }

    pub fn step_instruction(&mut self) {
        self.run();
    }

    pub fn current_pc(&self) -> u16 {
        self.cpu.pc
    }

    pub fn current_opcode_size(&self) -> u16 {
        opcode_size(self.read_byte(self.cpu.pc)).max(1) as u16
    }

    pub fn frame_ready(&self) -> bool {
        self.ppu.frame_ready
    }

    pub fn get_screen_data(&mut self) -> ScreenData<'_> {
        if self.gb_mode == GbMode::CgbMode {
            ScreenData::Cgb(self.ppu.get_color_screen())
        } else {
            ScreenData::Dmg(self.ppu.get_screen())
        }
    }

    pub fn debug_snapshot(&mut self) -> DebugSnapshot {
        let opcode = self.read_byte(self.cpu.pc);
        self.cpu.current_instruction = opcode;

        let mut pc_bytes = [0; 4];
        for (offset, byte) in pc_bytes.iter_mut().enumerate() {
            *byte = self.read_byte(self.cpu.pc.wrapping_add(offset as u16));
        }

        let mut stack_bytes = [0; 8];
        for (offset, byte) in stack_bytes.iter_mut().enumerate() {
            *byte = self.read_byte(self.cpu.sp.wrapping_add(offset as u16));
        }

        DebugSnapshot {
            af: self.cpu.af(),
            bc: self.cpu.bc(),
            de: self.cpu.de(),
            hl: self.cpu.hl(),
            sp: self.cpu.sp,
            pc: self.cpu.pc,
            opcode,
            disassembly: self.disassemble(),
            pc_bytes,
            stack_bytes,
            interrupt_master: self.cpu.interrupt_master,
            is_halted: self.cpu.is_halted,
            is_stopped: self.cpu.is_stopped,
            pending_cycles: self.cpu.pending_cycles,
            ticks: self.cpu.ticks,
            ly: self.ppu.get_ly(),
            stat: self.ppu.stat,
            if_flag: self.if_flag,
            ie_flag: self.ie_flag,
        }
    }

    pub fn read_memory_range(&self, start: u16, len: usize) -> Vec<u8> {
        (0..len)
            .map(|offset| self.read_byte(start.wrapping_add(offset as u16)))
            .collect()
    }

    pub fn disassemble_range(&mut self, start: u16, count: usize) -> Vec<DisassemblyLine> {
        let original_pc = self.cpu.pc;
        let original_instruction = self.cpu.current_instruction;
        let mut address = start;
        let mut lines = Vec::with_capacity(count);

        for _ in 0..count {
            let opcode = self.read_byte(address);
            self.cpu.pc = address;
            self.cpu.current_instruction = opcode;

            let size = opcode_size(opcode).max(1) as usize;
            let bytes = (0..size)
                .map(|offset| self.read_byte(address.wrapping_add(offset as u16)))
                .collect();

            lines.push(DisassemblyLine {
                address,
                bytes,
                text: self.disassemble(),
            });

            address = address.wrapping_add(size as u16);
        }

        self.cpu.pc = original_pc;
        self.cpu.current_instruction = original_instruction;
        lines
    }

    pub fn ppu_debug_snapshot(&self) -> PpuDebugSnapshot {
        self.ppu.debug_snapshot()
    }

    pub fn apu_debug_snapshot(&self) -> ApuDebugSnapshot {
        self.sound.debug_snapshot()
    }

    pub fn vram_bank_count(&self) -> usize {
        self.ppu.vram.len() / 0x2000
    }

    pub fn vram_tile_data(&self, bank: usize) -> &[u8] {
        let bank_count = self.vram_bank_count().max(1);
        let bank = bank.min(bank_count - 1);
        let start = bank * 0x2000;
        let end = start + 0x1800;
        &self.ppu.vram[start..end]
    }

    pub fn vram_map_data(&self, bank: usize, map_index: usize) -> &[u8] {
        let bank_count = self.vram_bank_count().max(1);
        let bank = bank.min(bank_count - 1);
        let map_offset = if map_index & 1 == 0 { 0x1800 } else { 0x1C00 };
        let start = bank * 0x2000 + map_offset;
        let end = start + 0x400;
        &self.ppu.vram[start..end]
    }

    pub fn sprite_height(&self) -> u8 {
        if self.read_byte(0xFF40) & 0x04 != 0 {
            16
        } else {
            8
        }
    }

    pub fn oam_sprites(&self) -> Vec<OamSprite> {
        self.ppu
            .oam
            .chunks_exact(4)
            .enumerate()
            .map(|(index, bytes)| OamSprite {
                index,
                y: bytes[0],
                x: bytes[1],
                tile_number: bytes[2],
                attributes: bytes[3],
            })
            .collect()
    }

    pub fn last_memory_writes(&self) -> &[MemoryWriteEvent] {
        &self.debug_write_log
    }

    pub fn components_tick(&mut self) {
        let cpu_cycles = self.cpu.pending_cycles as u32 * 4;
        let ppu_cycles = match self.speed_mode {
            SpeedMode::Normal => cpu_cycles,
            SpeedMode::Double => cpu_cycles / 2,
        };

        self.dma_tick(cpu_cycles);
        self.ppu_tick(ppu_cycles);
        self.timer_tick(cpu_cycles);
        self.sound.tick(ppu_cycles);
        self.cpu.pending_cycles = 0;
    }

    pub fn get_audio_buffer(&self) -> Vec<f32> {
        self.sound.get_audio_buffer()
    }

    pub fn take_serial_output(&mut self) -> Option<String> {
        self.serial.get_serial_message()
    }

    pub fn set_sample_rate(&mut self, rate: u32) {
        self.sound.set_sample_rate(rate);
    }

    pub fn set_serial_stdout_enabled(&mut self, enabled: bool) {
        self.serial_stdout_enabled = enabled;
    }

    fn cpu_tick(&mut self) {
        if self.cpu.is_halted || self.cpu.is_stopped {
            self.cpu.pending_cycles += 1;
            return;
        }

        self.cpu.current_instruction = self.read_byte(self.cpu.pc);

        self.decode();
        self.cpu.pending_cycles += self.cpu.instruction_cycles;
    }

    fn print_serial_message(&mut self) {
        if self.serial_stdout_enabled {
            if let Some(message) = self.take_serial_output() {
                println!("{}", message)
            };
        }
    }

    fn debug_message(&self) {
        println!(
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X} LY:{:02X} STAT: {:02X}",
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
            self.ppu.get_ly(),
            self.ppu.stat,
        );
    }

    pub fn press_button(&mut self, button: JoypadButton) {
        self.joypad.press(button);
    }

    pub fn release_button(&mut self, button: JoypadButton) {
        self.joypad.release(button);
    }

    pub fn get_battery_ram(&self) -> Option<&[u8]> {
        self.mbc.get_battery_ram()
    }

    pub fn set_battery_ram(&mut self, data: &[u8]) {
        self.mbc.set_battery_ram(data);
    }
}
