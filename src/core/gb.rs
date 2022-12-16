use crate::core::registers::Registers;
use crate::core::cpu::Cpu;
use crate::core::mmu::Mmu;
use crate::core::mbc::Mbc;

pub enum GbType {
    Dmg0,
    Dmg,
    Mgb,
    Sgb,
    Sgb2,
    CgbDmg,
    AgbDmg,
    Cgb,
    Agb
}

/// GameBoy emulator implementation
pub struct GB {
    pub cpu: Cpu,
    gb_type: GbType
}

fn get_registers_from_gb_type(gb_type: &GbType) -> Registers {
    match gb_type {
        GbType::Dmg0 => Registers::new(0x01, 0x00, 0xFF, 0x13, 0x00, 0xC1, 0x84, 0x03),
        GbType::Dmg => Registers::new(0x01, 0x00, 0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D), // F VALUE INCORRECT
        GbType::Mgb => Registers::new(0xFF, 0x00, 0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D), // F VALUE INCORRECT
        GbType::Sgb => Registers::new(0x01, 0x00, 0x00, 0x14, 0x00, 0x00, 0xC0, 0x60),
        GbType::Sgb2 => Registers::new(0xFF, 0x00, 0x00, 0x14, 0x00, 0x00, 0xC0, 0x60),
        // TODO: FROM HERE ON THE VALUES ARE NOT RIGHT! FIX THESE
        GbType::CgbDmg => Registers::new(0x11, 0x00, 0x01, 0x00, 0x00, 0x08, 0x99, 0x1A),
        GbType::AgbDmg => Registers::new(0x11, 0x00, 0x01, 0x00, 0x00, 0x08, 0x99, 0x1A),
        GbType::Cgb => Registers::new(0x11, 0x00, 0x00, 0x00, 0xFF, 0x56, 0x00, 0x0D),
        GbType::Agb => Registers::new(0x11, 0x00, 0x01, 0x00, 0xFF, 0x56, 0x00, 0x0D),
    }
}

impl GB {
    pub fn new(mbc: Box<dyn Mbc>, gb_type: GbType) -> Self {
        let registers: Registers = get_registers_from_gb_type(&gb_type);
        let mmu = Mmu::new(mbc);
        
        Self {
            cpu: Cpu::new(registers, mmu),
            gb_type,
        }
    }

    pub fn run(&mut self) {
        self.cpu.execute();
    }
}
