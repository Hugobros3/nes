pub struct Ppu {
    nametables: [[u8;1024];2],
    palette: [u8; 32]
}

impl Ppu {
    pub fn new() -> Self {
        return Ppu {
            nametables: [[0u8; 1024];2],
            palette: [0; 32],
        };
    }

    pub fn cpu_read(&self, address: u16, read_only: bool) -> u8 {
        match(address) {
            0x0000 => { // Control

            },
            0x0001 => { // Mask

            },
            0x0002 => { // Status

            },
            0x0003 => { // OAM address

            },
            0x0004 => { // OAM data

            },
            0x0005 => { // Scroll

            },
            0x0006 => { // PPU Address

            },
            0x0007 => { // PPU data

            },
            _ => panic!("Unreachable")
        }

        return 0;
    }

    pub fn cpu_write(&self, address: u16, data: u8) {
        match(address) {
            0x0000 => { // Control

            },
            0x0001 => { // Mask

            },
            0x0002 => { // Status

            },
            0x0003 => { // OAM address

            },
            0x0004 => { // OAM data

            },
            0x0005 => { // Scroll

            },
            0x0006 => { // PPU Address

            },
            0x0007 => { // PPU data

            },
            _ => panic!("Unreachable")
        }
    }

    pub fn ppu_read(&self, address: u16, read_only: bool) -> u8 {
        //TODO cartdrige override
        let address = address & 0x3FFFu16;
        return 0;
    }
    pub fn ppu_write(&self, address: u16, data: u8) {
        //TODO cartdrige override
        let address = address & 0x3FFFu16;
    }
}