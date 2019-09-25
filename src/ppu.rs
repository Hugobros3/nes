use crate::bus::Bus;
use crate::main;
use crate::ines_loader::MirroringMode;

pub mod debug;

pub struct Ppu {
    nametables: [[u8; 1024]; 2],
    palette: [u8; 32],
}

impl Ppu {
    pub fn new() -> Self {
        return Ppu {
            nametables: [[0u8; 1024]; 2],
            palette: [0; 32],
        };
    }

    pub fn read_ppu_register(&mut self, address: u16, read_only: bool) -> u8 {
        match address {
            0x0000 => { // Control
            }
            0x0001 => { // Mask
            }
            0x0002 => { // Status
            }
            0x0003 => { // OAM address
            }
            0x0004 => { // OAM data
            }
            0x0005 => { // Scroll
            }
            0x0006 => { // PPU Address
            }
            0x0007 => { // PPU data
            }
            _ => panic!("Unreachable")
        }

        return 0;
    }

    pub fn write_ppu_register(&mut self, address: u16, data: u8) {
        match address {
            0x0000 => { // Control
            }
            0x0001 => { // Mask
            }
            0x0002 => { // Status
            }
            0x0003 => { // OAM address
            }
            0x0004 => { // OAM data
            }
            0x0005 => { // Scroll
            }
            0x0006 => { // PPU Address
            }
            0x0007 => { // PPU data
            }
            _ => panic!("Unreachable")
        }
    }

    pub fn ppu_read(&self, bus: &Bus, address: u16, read_only: bool) -> u8 {
        let address = address & 0x3FFFu16;
        let mut data = 0u8;

        let mut cart_brw = bus.cartdrige.borrow_mut();
        //let cart_ref = cart_brw.as_mut();
        //if let Option::Some(cartdrige) = cart_ref {
        //}

        if cart_brw.is_some() && cart_brw.as_mut().unwrap().ppu_read(address, &mut data) {}
        else if address >= 0x2000u16 && address <= 0x3EFF {
            let address = address & 0x0FFF;
            let quadrant = address >> 10;

            let mirroring = if cart_brw.is_some() { cart_brw.as_mut().unwrap().get_info().mirroring_mode } else { MirroringMode::Horizontal };

            let tlb_bank = match mirroring {
                MirroringMode::Horizontal => { quadrant % 2 }
                MirroringMode::Vertical => { quadrant / 2 }
                MirroringMode::FourScreen => { quadrant }
            };

            data = self.nametables[tlb_bank as usize][(address & 0x03FF) as usize];
        } else if address >= 0x3F00u16 && address <= 0x3FFF {
            let address = address & 0x1F;
            //TODO map palette blacks ?
            data = self.palette[address as usize];
        }

        return data;
    }

    pub fn ppu_write(&mut self, bus: &Bus, address: u16, data: u8) {
        let address = address & 0x3FFFu16;

        let mut cart_brw = bus.cartdrige.borrow_mut();
        let cart_ref = cart_brw.as_mut();
        if cart_ref.is_some() && cart_ref.unwrap().ppu_write(address, data) {}
        else if address >= 0x2000u16 && address <= 0x3EFF {
            let address = address & 0x0FFF;
            let quadrant = address >> 10;

            let mirroring = if cart_brw.is_some() { cart_brw.as_mut().unwrap().get_info().mirroring_mode } else { MirroringMode::Horizontal };

            let tlb_bank = match mirroring {
                MirroringMode::Horizontal => { quadrant % 2 }
                MirroringMode::Vertical => { quadrant / 2 }
                MirroringMode::FourScreen => { quadrant }
            };

            self.nametables[tlb_bank as usize][(address & 0x03FF) as usize] = data;
        } else if address >= 0x3F00u16 && address <= 0x3FFF {
            let address = address & 0x1F;
            //TODO map palette blacks ?
            self.palette[address as usize] = data;
        }
    }
}