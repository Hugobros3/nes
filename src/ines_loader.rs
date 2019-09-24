use crate::cartdrige::Cartdrige;
use std::fs::File;
use std::io::{Read, BufReader};

const INES_MAGIC_BYTES: &'static str = "NES\u{001a}";

pub fn load_rom_file_as_cartdrige(filename: &str) -> Box<dyn Cartdrige> {
    let f = File::open(filename).expect("File not found");
    let mut reader = BufReader::new(f);

    let header = read_header(&mut reader);

    match header.mapper_type {
        0 => {
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
                prg_rom,
                prg_banks: header.prg_pages,
                chr_rom,
                chr_banks: header.chr_pages,
            });
        }
        _ => {
            panic!("Unsupported mapper type: {}", header.mapper_type)
        }
    }

    panic!("todo")
}

// Reads the header and spits out an internal struct that's easy to deal with
fn read_header(reader: &mut BufReader<File>) -> INesHeaderInfo {
    let mut magic_bytes = [0u8;4];
    reader.read_exact(&mut magic_bytes);

    if magic_bytes != INES_MAGIC_BYTES.as_bytes() {
        panic!("header doesn't match")
    }

    let prg_pages = read_byte(reader);
    let chr_pages = read_byte(reader);

    let flags6 = read_byte(reader);
    let flags7 = read_byte(reader);

    let mapper_low = flags6 >> 4;
    let mapper_hi = flags7 >> 4;
    let mapper_type = (mapper_hi << 4) | mapper_low;

    let mirroring_hv = (flags6 & 0x01) == 0x01;
    let mirroring4s = ((flags6 >> 3) & 0x01) == 0x01;
    let mirroring_mode = if(mirroring4s) { MirroringMode::FourScreen } else {
        if(mirroring_hv) {
            MirroringMode::Vertical
        } else {
            MirroringMode::Horizontal
        }
    };

    let trainer_present = ((flags6 >> 2) & 0x01) == 0x01;
    let batter_backed_prg_ram = ((flags6 >> 1) & 0x01) == 0x01;

    let flags8 = read_byte(reader);
    let flags9 = read_byte(reader);
    let flags10 = read_byte(reader);

    let padding11 = read_byte(reader);
    let padding12 = read_byte(reader);
    let padding13 = read_byte(reader);
    let padding14 = read_byte(reader);
    let padding15 = read_byte(reader);

    return INesHeaderInfo {
        mapper_type,
        mirroring_mode,
        batter_backed_prg_ram,
        trainer_present,
        prg_pages,
        chr_pages,
    };
}

enum MirroringMode {
    Horizontal,
    Vertical,
    FourScreen,
}

struct INesHeaderInfo {
    mapper_type: u8,
    mirroring_mode: MirroringMode,
    batter_backed_prg_ram: bool,
    trainer_present: bool,
    prg_pages: u8,
    chr_pages: u8,
    //prg_ram_size: u8,
}

fn read_byte(reader: &mut BufReader<File>) -> u8 {
    let mut buf = [0u8];
    reader.read_exact(&mut buf);
    return buf[0];
}

struct Mapper0Cartdrige {
    prg_rom: Vec<[u8;16384]>,
    prg_banks: u8,
    chr_rom: Vec<[u8;8192]>,
    chr_banks: u8,
}

impl Cartdrige for Mapper0Cartdrige {
    fn cpu_read(&self, address: u16, data: &mut u8) -> bool {
        if address >= 0x8000u16 && address <= 0xFFFFu16 {
            let bank = (address & 0x7FFF) >> 14;
            *data = self.prg_rom[bank as usize][(address & 0x3FFF) as usize];
            return true;
        }
        return false;
    }

    fn cpu_write(&self, address: u16, data: u8) -> bool {
        if address >= 0x8000u16 && address <= 0xFFFFu16 {
            let bank = (address & 0x7FFF) >> 14;
            //*data = self.prg_rom[bank as usize][(address & 0x3FFF) as usize];
            return true;
        }
        return false;
    }

    fn ppu_read(&self, address: u16, data: &mut u8) -> bool {
        unimplemented!()
    }

    fn ppu_write(&self, address: u16, data: u8) -> bool {
        unimplemented!()
    }
}