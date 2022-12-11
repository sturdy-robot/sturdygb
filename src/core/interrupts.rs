use crate::core::cpu::Cpu;

pub enum Interrupt {
    Vblank = 1,
    Lcdc = 2,
    Serial = 4,
    Timer = 8,
    Hitolo = 16,
}

impl Cpu {
    pub fn update_interrupts(&mut self) {
        if self.check_interrupt() {
            if self.is_halted {
                self.is_halted = false;
            }
            self.reg.sp = self.mmu.push_stack(self.reg.sp, self.reg.pc);
            let interrupt = self.get_interrupt();
            self.handle_interrupt(interrupt);
        }
    }

    fn check_interrupt(&mut self) -> bool {
        if self.reg.ime {
            let ifflag = self.mmu.io.ifflag;
            let ieflag = self.mmu.ieflag;
            ifflag & ieflag != 0
        } else {
            false
        }
    }

    fn get_interrupt(&mut self) -> Interrupt {
        match self.mmu.ieflag & 0x1F {
            0x10 => Interrupt::Hitolo,
            0x08 | 0x18 => Interrupt::Serial,
            0x04 | 0x0C  => Interrupt::Timer,
            0x02 | 0x06 | 0x0E | 0x1E => Interrupt::Lcdc,
            0x01 | 0x03 | 0x07 | 0x0F | 0x1F => Interrupt::Vblank,
            _ => unreachable!(),
        }
    }

    fn handle_interrupt(&mut self, interrupt: Interrupt) {
        self.cycles += 8;
        match interrupt {
            Interrupt::Vblank => {
                self.reg.pc = 0x40;
            }
            Interrupt::Serial => {
                self.reg.pc = 0x58;
            }
            Interrupt::Hitolo => {
                self.reg.pc = 0x60;
            }
            Interrupt::Lcdc => {
                self.reg.pc = 0x48;
            }
            Interrupt::Timer => {
                self.reg.pc = 0x50;
            }
        };

        self.reg.ime = false;
    }
}