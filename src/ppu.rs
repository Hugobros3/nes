use crate::bus::Bus;
use crate::main;
use crate::ines_loader::MirroringMode;

pub mod debug;

bf!(Status[u8] {
    unused: 0:4,
    sprite_overflow: 5:5,
    sprite_zero_hit: 6:6,
    vertical_blank: 7:7
});

bf!(Mask[u8] {
    grayscale: 0:0,
    render_background_left: 1:1,
    render_sprites_left: 2:2,
    render_background: 3:3,
    render_sprites: 4:4,
    enhance_red: 5:5,
    enhance_green: 6:6,
    enhance_blue: 7:7
});

bf!(Control[u8] {
    nametable_x: 0:0,
    nametable_y: 1:1,
    increment_mode: 2:2,
    pattern_sprite: 3:3,
    pattern_background: 4:4,
    sprite_size: 5:5,
    slave_mode: 6:6,
    enable_nmi: 7:7,
});

bf!(Loopy[u16] {
    coarse_x: 0:4,
    coarse_y: 5:10,
    nametablex: 11:11,
    nametable_y: 12:12,
    fine_y: 13:15,
    unused: 16:16,
});

pub struct Ppu {
    nametables: [[u8; 1024]; 2],
    palette: [u8; 32],

    frame_complete: bool,

    status: Status,
    mask: Mask,
    control: Control,

    vram_addr: Loopy,
    tram_addr: Loopy,

    fine_x: u8,

    address_latch: u8,
    ppu_data_buffer: u8,

    scanline: u16,
    cycle: u16,

    bg_next_tile_id: u8,
    bg_next_tile_attrib: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,
    bg_shifter_pattern_lo: u16,
    bg_shifter_pattern_hi: u16,
    bg_shifter_attrib_lo: u16,
    bg_shifter_attrib_hi: u16,
}

impl Ppu {
    pub fn new() -> Self {
        return Ppu {
            nametables: [[0u8; 1024]; 2],
            palette: [0; 32],

            frame_complete: false,

            status: Status::new(0),
            mask: Mask::new(0),
            control: Control::new(0),

            vram_addr: Loopy::new(0),
            tram_addr: Loopy::new(0),
            fine_x: 0,
            address_latch: 0,
            ppu_data_buffer: 0,
            scanline: 0,
            cycle: 0,
            bg_next_tile_id: 0,
            bg_next_tile_attrib: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            bg_shifter_pattern_lo: 0,
            bg_shifter_pattern_hi: 0,
            bg_shifter_attrib_lo: 0,
            bg_shifter_attrib_hi: 0,
        };
    }

    pub fn read_ppu_register(&mut self, bus: &Bus, address: u16, read_only: bool) -> u8 {
        let mut data = 0u8;
        match address {
            0x0000 => { // Control
                if read_only {
                    data = self.control.val;
                }
            }
            0x0001 => { // Mask
                if read_only {
                    data = self.mask.val;
                }
            }
            0x0002 => { // Status
                if(read_only) {
                    data = self.status.val;
                } else {
                    data = (self.status.val & 0xE0) | (self.ppu_data_buffer & 0x1F);
                    self.status.set_vertical_blank(0);
                    self.address_latch = 0;
                }
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
                if(!read_only) {
                    data = self.ppu_data_buffer;
                    self.ppu_data_buffer = self.ppu_read(bus, address, read_only);
                    if(self.vram_addr.val >= 0x3F00) {
                        data = self.ppu_data_buffer;
                    }
                    self.vram_addr.val += (if self.control.increment_mode() == 1 { 32 } else { 1 });
                }
            }
            _ => panic!("Unreachable")
        }

        return data;
    }

    pub fn write_ppu_register(&mut self, bus: &Bus, address: u16, data: u8) {
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

        if cart_brw.is_some() && cart_brw.as_mut().unwrap().ppu_read(address, &mut data) {} else if address >= 0x2000u16 && address <= 0x3EFF {
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
        if cart_ref.is_some() && cart_ref.unwrap().ppu_write(address, data) {} else if address >= 0x2000u16 && address <= 0x3EFF {
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