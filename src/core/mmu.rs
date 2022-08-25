pub struct MMU {
    pub current_rom_bank: u8,
    vram: Vec<u8>,
    eram: Vec<u8>,
    wram: Vec<u8>,
    oam: Vec<u8>,
    hram: Vec<u8>,
}

impl MMU {
    pub fn new() -> Self {
        Self {
            current_rom_bank: 0,
            vram: Vec::new(),
            eram: Vec::new(),
            wram: Vec::new(),
            oam: Vec::new(),
            hram: Vec::new(),
        }
    }

    pub fn read_byte(&self, address: u16) {
        let result: u8 = 0;

        if address < 0x4000 {

        }
    }
}