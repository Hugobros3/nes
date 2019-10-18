use crate::cartdrige::Cartdrige;
use crate::ines_loader::INesHeaderInfo;
use std::io::{Read, BufReader};

pub fn create_mapper2_cartdrige<T: Read>(header: INesHeaderInfo, reader: &mut BufReader<T>) -> Box<dyn Cartdrige> {
    let mut prg_rom = Vec::<[u8; 16384]>::new();
    for i in 0..header.prg_pages {
        let mut page = [0; 16384];
        reader.read_exact(&mut page);
        prg_rom.push(page);
    }
    let mut chr_rom = Vec::<[u8; 8192]>::new();
    for i in 0..header.chr_pages {
        let mut page = [0; 8192];
        reader.read_exact(&mut page);
        chr_rom.push(page);
    }

    if chr_rom.is_empty() {
        let mut page = [0; 8192];
        chr_rom.push(page);
    }

    return Box::new(Mapper2Cartdrige {
        header,
        prg_banks: prg_rom,
        chr_banks: chr_rom,
        selected_prg_bank: 0,
    });
}

struct Mapper2Cartdrige {
    header: INesHeaderInfo,
    prg_banks: Vec<[u8; 16384]>,
    chr_banks: Vec<[u8; 8192]>,
    selected_prg_bank: u8,
}

impl Cartdrige for Mapper2Cartdrige {
    fn get_info(&self) -> &INesHeaderInfo {
        return &self.header;
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool {
        if address >= 0x8000u16 && address <= 0xFFFFu16 {
            let bank = if address > 0xBFFF { (self.prg_banks.len() - 1) as u8 } else { self.selected_prg_bank };
            *data = self.prg_banks[bank as usize][(address & 0x3FFF) as usize];
            return true;
        }
        return false;
    }

    fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        if address >= 0x8000u16 && address <= 0xFFFFu16 {
            self.selected_prg_bank = data % self.prg_banks.len() as u8;
        }
        return false;
    }

    fn ppu_read(&mut self, address: u16, data: &mut u8) -> bool {
        if address >= 0x0000u16 && address <= 0x1FFFu16 {
            let bank = address >> 12;
            // left or right bank ? we don't care actually!
            *data = self.chr_banks[0][address as usize];
            return true;
        }
        return false;
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> bool {
        if address >= 0x0000u16 && address <= 0x1FFFu16 {
            self.chr_banks[0][address as usize] = data;
            return true;
        }
        return false;
    }
}