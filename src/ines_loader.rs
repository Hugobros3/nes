use crate::cartdrige::Cartdrige;
use std::fs::File;
use std::io::{Read, BufReader};

use crate::mappers::{create_mmc0_cartdrige};

const INES_MAGIC_BYTES: &'static str = "NES\u{001a}";

pub fn load_rom_file_as_cartdrige(filename: &str) -> Box<dyn Cartdrige> {
    let f = File::open(filename).expect("File not found");
    let mut reader = BufReader::new(f);

    let header = read_header(&mut reader);

    match header.mapper_type {
        0 => {
            return create_mmc0_cartdrige(&mut reader, header);
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

#[derive(Copy, Clone, Debug)]
pub enum MirroringMode {
    Horizontal,
    Vertical,
    FourScreen,
}

pub struct INesHeaderInfo {
    pub mapper_type: u8,
    pub mirroring_mode: MirroringMode,
    pub batter_backed_prg_ram: bool,
    pub trainer_present: bool,
    pub prg_pages: u8,
    pub chr_pages: u8,
    //prg_ram_size: u8,
}

fn read_byte(reader: &mut BufReader<File>) -> u8 {
    let mut buf = [0u8];
    reader.read_exact(&mut buf);
    return buf[0];
}