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
            // println!("Got inst: {:X}", instruction);
            // println!("Address: {:X}", &self.reg.pc);
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

    
}
