use crate::cpu::R6502;

pub struct Bus {
    pub cpu: R6502,
    pub ram: [u8;65536]
}

impl Bus {
    pub fn write(&mut self, address: u16, data: u8) {
        if(address >= 0x0000u16 && address < 0xFFFFu16) {
            self.ram[address as usize] = data
        }
    }

    pub fn read(&self, address: u16, read_only: bool) -> u8 {
        if(address >= 0x0000u16 && address < 0xFFFFu16) {
            return self.ram[address as usize]
        } else {
            return 0x00u8;
        }
    }

    pub fn new() -> Self {
        return Bus {
            cpu: R6502::new(),
            ram: [0;65536]
        }
    }
}