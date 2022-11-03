use crate::core::cartridge::Cartridge;
use crate::core::mmu::Mmu;
use crate::core::opcodes::Opcode;
use crate::core::registers::Registers;

pub struct Cpu {
    pub reg: Registers,
    pub mmu: Mmu,
    pub is_halted: bool,
    pub cycles: u8,
    pub is_cgb: bool,
}

enum Interrupt {
    Vblank,
    Lcdc,
    Serial,
    Timer,
    Hitolo,
}


impl Cpu {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            reg: Registers::new(&is_cgb),
            mmu: Mmu::new(cartridge),
            is_halted: false,
            cycles: 0,
            is_cgb,
        }
    }

    pub fn execute(&mut self) {
        while !self.is_halted && self.reg.pc < 0x8000 {
            // TODO: WRITE INTERRUPT CHECKING
            if self.check_interrupt() {
                self.reg.sp = self.mmu.push_stack(self.reg.sp, self.reg.pc);
                if self.reg.ime {
                    self.reg.ime = false;
                    let interrupt = match self.mmu.ieflag & 0x1F{
                        0x10 => Interrupt::Hitolo,
                        0x08 | 0x18 => Interrupt::Serial,
                        0x04 | 0x0C  => Interrupt::Timer,
                        0x02 | 0x06 | 0x0E | 0x1E => Interrupt::Lcdc,
                        0x01 | 0x03 | 0x07 | 0x0F | 0x1F => Interrupt::Vblank,
                        _ => unreachable!(),
                    };
                    self.handle_interrupt(interrupt);
                }
            }
            let instruction: u8;
            (instruction, self.reg.pc) = self.mmu.fetch_instruction(&mut self.reg.pc);
            println!("Got inst: {:X}", instruction);
            let mut opcode = Opcode::new(instruction, &mut self.reg, &mut self.mmu);
            opcode.decode();
            if opcode.is_halted {
                self.is_halted = true;
                break;
            }
        }
    }

    fn check_interrupt(&mut self) -> bool {
        let ifflag = self.mmu.read_byte(0xFF0F);
        let ieflag = self.mmu.read_byte(0xFFFF);
        ifflag & ieflag != 0
    }

    fn handle_interrupt(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::Vblank => {

            }
            Interrupt::Serial => {

            }
            Interrupt::Hitolo => {

            }
            Interrupt::Lcdc => {

            }
            Interrupt::Timer => {

            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;
    use crate::core::cartridge::Cartridge;
    use crate::core::opcodes::Opcode;

    fn set_up() -> Cpu {
        let cartridge = Cartridge::new("roms/gb-test-roms/cpu_instrs/cpu_instrs.gb");
        let is_cgb = cartridge.is_cgb_only();
        Cpu::new(cartridge, is_cgb)
    }

    #[test]
    fn test_add_instructions() {
        let mut cpu = set_up();
        let mut opcode = Opcode::new(0x80, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x81, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x14);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x82, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x14);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x83, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0xEC);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x84, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0xED);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x85, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x3A);
        assert_eq!(cpu.reg.f, 0x30);
        opcode = Opcode::new(0x86, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x3A);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x87, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x74);
        assert_eq!(cpu.reg.f, 0x20);
    }

    #[test]
    fn test_adc_instructions() {
        let mut cpu = set_up();
        let mut opcode = Opcode::new(0x88, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x89, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x14);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x8A, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x14);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x8B, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0xEC);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x8C, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0xED);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x8D, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x3A);
        assert_eq!(cpu.reg.f, 0x30);
        opcode = Opcode::new(0x8E, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x3A);
        assert_eq!(cpu.reg.f, 0x00);
        opcode = Opcode::new(0x8F, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x74);
        assert_eq!(cpu.reg.f, 0x20);
    }

}
