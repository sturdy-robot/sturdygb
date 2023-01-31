use crate::core::cpu::Cpu;
use crate::core::joypad::Joypad;
use crate::core::mbc::{GbMode, Mbc};
use crate::core::ppu::Ppu;
use crate::core::serial::Serial;
use crate::core::sound::Sound;
use crate::core::timer::Timer;
use rand::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum GbTypes {
    Dmg,
    Mgb,
    Cgb,
    Sgb,
}

pub enum SpeedMode {
    Normal,
    Double,
}

fn get_gb_type(t: u8) -> GbTypes {
    match t {
        _ => GbTypes::Dmg,
    }
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
    pub undoc_registers: [u8; 4],
}

impl Gb {
    pub fn new(mbc: Box<dyn Mbc>, gb_mode: GbMode, gb_type: GbTypes) -> Self {
        let registers: [u8; 8];
        match gb_type {
            GbTypes::Dmg => {
                registers = [0x01, 0x00, 0xFF, 0x13, 0x00, 0xC1, 0x84, 0x03];
            }
            GbTypes::Mgb => {
                registers = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
            }
            GbTypes::Cgb => {
                registers = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
            }
            GbTypes::Sgb => {
                registers = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
            }
        };
        let mut wram: Vec<u8>;
        if gb_mode == GbMode::CgbMode {
            wram = vec![0; 0x8000];
        } else {
            wram = vec![0; 0x2000];
        }

        let mut hram = vec![0; 0x7F];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut wram);
        rng.fill_bytes(&mut hram);

        Self {
            cpu: Cpu::new(registers),
            ppu: Ppu::new(),
            serial: Serial::new(),
            joypad: Joypad::new(),
            sound: Sound::new(),
            timer: Timer::new(),
            mbc,
            gb_speed: 0,
            gb_type,
            gb_mode,
            wram,
            hram,
            ram_bank: 0,
            ie_flag: 0,
            if_flag: 0,
            boot_rom_enabled: 0,
            prepare_speed_switch: false,
            speed_mode: SpeedMode::Normal,
            undoc_registers: [0; 4],
        }
    }

    pub fn run(&mut self) {
        self.cpu.pc = 0x100;
        while !self.cpu.is_halted {
            self.cpu.current_instruction = self.read_byte(self.cpu.pc);
            let instr_disasm = self.disassemble();
            println!("[{:04X}]: {} \tAF: {:04X} BC: {:04X} DE: {:04X} HL: {:04X} SP: {:04X}", self.cpu.pc, instr_disasm, self.cpu.af(), self.cpu.bc(), self.cpu.de(), self.cpu.hl(), self.cpu.sp);
            self.decode();
        }
    }
}
