use crate::cartdrige::Cartdrige;
use crate::ines_loader::INesHeaderInfo;
use std::io::{Read, BufReader};

pub fn create_mmc0_cartdrige<T: Read>(reader: &mut BufReader<T>, header: INesHeaderInfo) -> Box<dyn Cartdrige> {
    let mut prg_rom = Vec::<[u8;16384]>::new();
    for i in 0..header.prg_pages {
        let mut page = [0; 16384];
        reader.read_exact(&mut page);
        prg_rom.push(page);
    }
    let mut chr_rom = Vec::<[u8;8192]>::new();
    for i in 0..header.chr_pages {
        let mut page = [0; 8192];
        reader.read_exact(&mut page);
        chr_rom.push(page);
    }

    return Box::new(Mapper0Cartdrige {
        header,
        prg_rom,
        chr_rom,
    });
}

struct Mapper0Cartdrige {
    header: INesHeaderInfo,
    prg_rom: Vec<[u8;16384]>,
    chr_rom: Vec<[u8;8192]>,
}

impl Cartdrige for Mapper0Cartdrige {
    fn get_info(&self) -> &INesHeaderInfo {
        return &self.header;
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool {
        if address >= 0x8000u16 && address <= 0xFFFFu16 {
            let bank = ((address & 0x7FFF) >> 14) % (self.header.prg_pages as u16);
            *data = self.prg_rom[bank as usize][(address & 0x3FFF) as usize];
            return true;
        }
        return false;
    }

    fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        if address >= 0x8000u16 && address <= 0xFFFFu16 {
            let bank = ((address & 0x7FFF) >> 14) % (self.header.prg_pages as u16);
            //*data = self.prg_rom[bank as usize][(address & 0x3FFF) as usize];
            return true;
        }
        return false;
    }

    fn ppu_read(&mut self, address: u16, data: &mut u8) -> bool {
        if address >= 0x0000u16 && address <= 0x1FFFu16 {
            let bank = address >> 12;
            // left or right bank ? we don't care actually!
            *data = self.chr_rom[0][address as usize];
            return true;
        }
        return false;
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> bool {
        if address >= 0x0000u16 && address <= 0x1FFFu16 {
            return true;
        }
        return false;
    }
}