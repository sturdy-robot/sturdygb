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

fn get_registers_from_gb_type(gb_type: &GbTypes) -> [u8; 8] {
    match gb_type {
        GbTypes::Dmg => {
            [0x01, 0xB0, 0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D]
        }
        GbTypes::Mgb => {
            [0x01, 0xB0, 0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D]
        }
        GbTypes::Cgb => {
            [0x11, 0x80, 0x00, 0x00, 0xFF, 0x56, 0x00, 0x0D]
        }
        GbTypes::Sgb => {
            [0x01, 0x00, 0x00, 0x14, 0x00, 0x00, 0xC0, 0x60]
        }
    }
}

// Values from the Cycle-Accurate Game Boy documentation
// Pan Docs is not that detailed
fn get_div_values(gb_type: &GbTypes, gb_mode: &GbMode) -> u16 {
    if (gb_mode == &GbMode::DmgMode) || (gb_mode == &GbMode::NonCgbMode) {
        match gb_type {
            GbTypes::Dmg | GbTypes::Mgb => 0xABCC,
            GbTypes::Sgb => 0x0000,
            GbTypes::Cgb => 0x267C,
        }
    } else {
        match gb_type {
            GbTypes::Cgb => 0x1EA0,
            _ => panic!("Trying to initialize a GBC game on a non-GBC compatible device."),
        }
    }
}

impl Gb {
    pub fn new(mbc: Box<dyn Mbc>, gb_mode: GbMode, gb_type: GbTypes) -> Self {
        let registers: [u8; 8] = get_registers_from_gb_type(&gb_type);
        let div: u16 = get_div_values(&gb_type, &gb_mode);
        let mut wram: Vec<u8> = if gb_mode == GbMode::CgbMode { vec![0; 0x8000] } else { vec![0; 0x2000] };
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

    pub fn run(&mut self) {
        self.cpu.pc = 0x100;
        while !self.cpu.is_halted {
            self.handle_interrupt();
            self.cpu.current_instruction = self.read_byte(self.cpu.pc);
            // let instr_disasm = self.disassemble();
            println!(
                "[{:04X}]: {:04X} \t\tAF: {:04X} BC: {:04X} DE: {:04X} HL: {:04X} SP: {:04X}",
                self.cpu.pc,
                self.cpu.current_instruction,
                self.cpu.af(),
                self.cpu.bc(),
                self.cpu.de(),
                self.cpu.hl(),
                self.cpu.sp
            );
            self.decode();
            match self.serial.get_serial_message() {
                Some(message) => println!("{}", message),
                None => (),
            };
            self.run_timer();
        }
    }
}
