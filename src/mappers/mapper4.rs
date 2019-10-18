use crate::cartdrige::Cartdrige;
use crate::ines_loader::{INesHeaderInfo, MirroringMode};
use std::io::{Read, BufReader, BufRead};

pub fn create_mapper4_cartdrige<T: Read>(header: INesHeaderInfo, reader: &mut BufReader<T>) -> Box<dyn Cartdrige> {
    let mut prg_data = vec![0; header.prg_pages as usize * 16384];
    reader.read_exact(&mut prg_data);

    let mut chr_data = vec![0; header.chr_pages as usize * 8192];
    reader.read_exact(&mut chr_data);

    let prg_banks_count = ((prg_data.len() / 8192) - 0) as u8;
    let prg_last_bank = ((prg_data.len() / 8192) - 1) as u8;
    let prg_second_to_last_bank = ((prg_data.len() / 8192) - 2) as u8;

    //println!("{}, {}, {}", prg_banks_count, prg_last_bank, prg_second_to_last_bank);

    return Box::new(Mapper4Cartdrige {
        header,

        register_to_update: 0,
        prg_bank_mode: false,
        chr_a12_inversion: false,

        registers: [0; 8],

        a12_low_clocks: 0,
        irq_couter: 0,
        irq_reload_value: 0,
        irq_reload_flag: false,
        irq_enable: false,

        prg_banks_count,
        prg_second_to_last_bank,
        prg_last_bank,

        mirroring_mode: MirroringMode::Horizontal,

        prg_data,
        chr_data,
    });
}

struct Mapper4Cartdrige {
    header: INesHeaderInfo,
    prg_data: Vec<u8>,

    prg_banks_count: u8,
    prg_second_to_last_bank: u8,
    prg_last_bank: u8,

    chr_data: Vec<u8>,

    register_to_update: u8,
    prg_bank_mode: bool,
    chr_a12_inversion: bool,

    registers: [u8; 8],

    a12_low_clocks: u8,
    irq_couter: u8,
    irq_reload_value: u8,
    irq_reload_flag: bool,
    irq_enable: bool,

    mirroring_mode: MirroringMode,
}

impl Cartdrige for Mapper4Cartdrige {
    fn get_info(&self) -> &INesHeaderInfo {
        return &self.header;
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool {
        if address >= 0x8000u16 && address <= 0xFFFFu16 {
            let quad = (address >> 13) & 0x3;
            let bank = match quad {
                /* 8000-9FFF */ 0 => { if !self.prg_bank_mode { self.registers[6] & 0x3F } else { self.prg_second_to_last_bank } }
                /* A000-BFFF */ 1 => { self.registers[7] & 0x3F }
                /* C000-DFFF */ 2 => { if !self.prg_bank_mode { self.prg_second_to_last_bank } else { self.registers[6] & 0x3F } }
                /* E000-FFFF */ 3 => { self.prg_last_bank }
                _ => { panic!("impossible case") }
            };
            let address = ((bank as u32) * 8192) | (address & 0x1FFF) as u32;
            *data = self.prg_data[address as usize];
            return true;
        }
        return false;
    }

    fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        if address >= 0x8000u16 && address <= 0xFFFFu16 {
            let quad = (address >> 13) & 0x3;
            let even = address % 2 == 0;
            match quad {
                0 => {
                    if even {
                        // bank select
                        self.register_to_update = data & 0b00000111;
                        self.prg_bank_mode = ((data >> 6) & 0x01) == 1;
                        self.chr_a12_inversion = ((data >> 7) & 0x01) == 1;
                    } else {
                        // bank update
                        self.registers[self.register_to_update as usize] = data;
                    }
                }
                1 => {
                    if even {
                        // mirroring
                        self.mirroring_mode = if (data & 0x01) == 0 { MirroringMode::Vertical } else { MirroringMode::Horizontal };
                    } else {
                        // prg ram protect
                        // todo
                    }
                }
                2 => {
                    if even {
                        // irq latch
                        self.irq_reload_value = data;
                    } else {
                        // irq reload
                        self.irq_reload_flag = true;
                    }
                }
                3 => {
                    if even {
                        // irq disable
                        self.irq_enable = false;
                    } else {
                        // irq enable
                        self.irq_enable = true;
                    }
                }
                _ => { panic!("impossible case") }
            }
        }
        return false;
    }

    fn ppu_read(&mut self, address: u16, data: &mut u8) -> (bool, bool) {
        let mut irq = false;
        let a12 = (address >> 12) & 0x01;
        if a12 == 1 {
            if self.a12_low_clocks == 2 * 3 {
                if self.irq_couter == 0 || self.irq_reload_flag {
                    self.irq_couter = self.irq_reload_value;
                    self.irq_reload_flag = false;
                } else {
                    self.irq_couter -= 1;

                    if self.irq_couter == 0 {
                        if self.irq_enable {
                            irq = true;
                        }
                    }
                }
            }
            self.a12_low_clocks = 0;
        } else {
            if self.a12_low_clocks < 255 {
                self.a12_low_clocks += 1;
            }
        }

        if address >= 0x0000u16 && address <= 0x1FFFu16 {
            let address = if self.chr_a12_inversion { address ^ 0x1000 } else { address };
            let address: u16 =
                if address >= 0x0000 && address <= 0x07FF {
                    ((self.registers[0] & 0xFE) as u16 * 1024) | address & 0x7FF
                } else if address >= 0x0800 && address <= 0x0FFF {
                    ((self.registers[1] & 0xFE) as u16 * 1024) | address & 0x7FF
                } else if address >= 0x1000 && address <= 0x13FF {
                    (self.registers[2] as u16 * 1024) | address & 0x3FF
                } else if address >= 0x1400 && address <= 0x17FF {
                    (self.registers[3] as u16 * 1024) | address & 0x3FF
                } else if address >= 0x1800 && address <= 0x1BFF {
                    (self.registers[4] as u16 * 1024) | address & 0x3FF
                } else if address >= 0x1C00 && address <= 0x1FFF {
                    (self.registers[5] as u16 * 1024) | address & 0x3FF
                } else { panic!("out of range") };
            *data = self.chr_data[address as usize];

            return (true, irq);
        }
        return (false, irq);
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> bool {
        if address >= 0x0000u16 && address <= 0x1FFFu16 {
            return true;
        }
        return false;
    }
}