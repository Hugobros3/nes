use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use crate::bus::Bus;

pub fn dump_memory_contents(nes: &Bus, filename: &str) {
    let mut f = OpenOptions::new().write(true).create(true).append(false).open(filename).expect("failed to open file");
    let mut writer = BufWriter::new(f);

    let mut at = 0u32;
    let to = 0x10000u32;

    while at < to {
        let bytes_per_line = 8;
        let mut line = String::new();
        line.push('$');
        let line_hex = hex::encode((at as u16).to_be_bytes());
        line.push_str(line_hex.as_str());

        let bytes_per_group = 4;
        let mut group = 0;
        while group < bytes_per_line / bytes_per_group {
            line.push(' ');
            for i in 0 .. bytes_per_group {
                let memory_contents = nes.cpu_read((at as u16) + ((group * bytes_per_group) as u16) + (i as u16), true);
                line.push_str(hex::encode([memory_contents]).as_str());
            }
            group += 1;
        }

        line.push('\n');
        writer.write(line.as_bytes());

        at += bytes_per_line;
    }
}

pub fn dump_visual_memory_contents(nes: &Bus, filename: &str) {
    let mut f = OpenOptions::new().write(true).create(true).append(false).open(filename).expect("failed to open file");
    let mut writer = BufWriter::new(f);

    let mut at = 0u32;
    let to = 0x4000u32;

    let ppu = nes.ppu.borrow_mut();

    while at < to {
        let bytes_per_line = 8;
        let mut line = String::new();
        line.push('$');
        let line_hex = hex::encode((at as u16).to_be_bytes());
        line.push_str(line_hex.as_str());

        let bytes_per_group = 4;
        let mut group = 0;
        while group < bytes_per_line / bytes_per_group {
            line.push(' ');
            for i in 0 .. bytes_per_group {
                let memory_contents = ppu.ppu_read(nes, (at as u16) + ((group * bytes_per_group) as u16) + (i as u16), true);
                line.push_str(hex::encode([memory_contents]).as_str());
            }
            group += 1;
        }

        line.push('\n');
        writer.write(line.as_bytes());

        at += bytes_per_line;
    }
}