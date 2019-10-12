use crate::bus::Bus;
use crate::main;
use crate::ines_loader::MirroringMode;
use crate::ppu::palette::get_colour_from_palette_ram;
use std::rc::Rc;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod main_window;
pub mod patterns_debug_viewer;
pub mod nametables_debug_viewer;
mod palette;
mod window_common;

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
    coarse_y: 5:9,
    nametable_x: 10:10,
    nametable_y: 11:11,
    fine_y: 12:14,
    unused: 15:15,
});

pub struct Ppu where {
    nametables: [[u8; 1024]; 2],
    palette: [u8; 32],

    pub frame_complete: bool,

    status: Status,
    mask: Mask,
    control: Control,

    vram_addr: Loopy,
    tram_addr: Loopy,

    fine_x: u8,

    address_latch: u8,
    ppu_data_buffer: u8,

    scanline: i16,
    cycle: i16,

    bg_next_tile_id: u8,
    bg_next_tile_attrib: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,
    bg_shifter_pattern_lo: u16,
    bg_shifter_pattern_hi: u16,
    bg_shifter_attrib_lo: u16,
    bg_shifter_attrib_hi: u16,

    pub send_nmi: bool,

    output: Rc<dyn PpuOutput>,
}

impl Ppu {
    pub fn new(output: Rc<dyn PpuOutput>) -> Self {
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

            send_nmi: false,

            output,
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
                if read_only {
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
                if !read_only {
                    data = self.ppu_data_buffer;
                    self.ppu_data_buffer = self.ppu_read(bus, self.vram_addr.val, read_only);
                    if self.vram_addr.val >= 0x3F00 {
                        data = self.ppu_data_buffer;
                    }
                    self.vram_addr.val = self.vram_addr.val + (if self.control.increment_mode() == 1 { 32 } else { 1 });
                }
            }
            _ => panic!("Unreachable")
        }

        return data;
    }

    pub fn write_ppu_register(&mut self, bus: &Bus, address: u16, data: u8) {
        match address {
            0x0000 => { // Control
                self.control.val = data;
                self.tram_addr.set_nametable_x(self.control.nametable_x() as u16);
                self.tram_addr.set_nametable_y(self.control.nametable_y() as u16);
            }
            0x0001 => { // Mask
                self.mask.val = data;
            }
            0x0002 => { // Status
            }
            0x0003 => { // OAM address
            }
            0x0004 => { // OAM data
            }
            0x0005 => { // Scroll
                if self.address_latch == 0 {
                    self.fine_x = data & 0x07;
                    self.tram_addr.set_coarse_x(((data >> 3) as u16));
                    self.address_latch = 1;
                } else {
                    self.tram_addr.set_fine_y((data & 0x07) as u16);
                    self.tram_addr.set_coarse_y(((data >> 3) as u16));
                    self.address_latch = 0;
                }
            }
            0x0006 => { // PPU Address
                if self.address_latch == 0 {
                    self.tram_addr.val = (((data & 0x3F) as u16) << 8) | (self.tram_addr.val & 0x00FF);
                    self.address_latch = 1;
                } else {
                    self.tram_addr.val = (self.tram_addr.val & 0xFF00) | (data as u16);
                    self.vram_addr.val = self.tram_addr.val;
                    self.address_latch = 0;
                }
            }
            0x0007 => { // PPU data
                self.ppu_write(bus, self.vram_addr.val, data);
                self.vram_addr.val += (if self.control.increment_mode() == 1 { 32 } else { 1 });
            }
            _ => panic!("Unreachable")
        }
    }

    pub fn ppu_read(&self, bus: &Bus, address: u16, read_only: bool) -> u8 {
        let address = address & 0x3FFFu16;
        let mut data = 0u8;

        let mut cart_brw = bus.cartdrige.borrow_mut();

        if cart_brw.is_some() && cart_brw.as_mut().unwrap().ppu_read(address, &mut data) {} else if address >= 0x2000u16 && address <= 0x3EFF {
            let address = address & 0x0FFF;
            let quadrant = address >> 10;

            let mirroring = if cart_brw.is_some() { cart_brw.as_mut().unwrap().get_info().mirroring_mode } else { MirroringMode::Horizontal };

            //TODO it doesn't work like that in actuality, emulate relevant cartdrige port lines ( CIRAM/CE CIRAM A10 )
            let tlb_bank = match mirroring {
                MirroringMode::Horizontal => { quadrant / 2 }
                MirroringMode::Vertical => { quadrant % 2 }
                MirroringMode::FourScreen => { quadrant }
            };

            data = self.nametables[tlb_bank as usize][(address & 0x03FF) as usize];
        } else if address >= 0x3F00u16 && address <= 0x3FFF {
            let mut address = address & 0x1F;
            if address == 0x0010 { address = 0x0000; }
            if address == 0x0014 { address = 0x0004; }
            if address == 0x0018 { address = 0x0008; }
            if address == 0x001C { address = 0x000C; }
            data = self.palette[address as usize];
        }

        return data;
    }

    pub fn ppu_write(&mut self, bus: &Bus, address: u16, data: u8) {
        let address = address & 0x3FFFu16;
        //println!("write ppu add={} d={} vb={}", address, data, self.status.vertical_blank());

        let mut cart_brw = bus.cartdrige.borrow_mut();
        let cart_ref = cart_brw.as_mut();
        if cart_ref.is_some() && cart_ref.unwrap().ppu_write(address, data) {} else if address >= 0x2000u16 && address <= 0x3EFF {
            let address = address & 0x0FFF;
            let quadrant = address >> 10;

            let mirroring = if cart_brw.is_some() { cart_brw.as_mut().unwrap().get_info().mirroring_mode } else { MirroringMode::Horizontal };

            let tlb_bank = match mirroring {
                MirroringMode::Horizontal => { quadrant / 2 }
                MirroringMode::Vertical => { quadrant % 2 }
                MirroringMode::FourScreen => { quadrant }
            };

            self.nametables[tlb_bank as usize][(address & 0x03FF) as usize] = data;
        } else if address >= 0x3F00u16 && address <= 0x3FFF {
            let mut address = address & 0x1F;
            if address == 0x0010 { address = 0x0000; }
            if address == 0x0014 { address = 0x0004; }
            if address == 0x0018 { address = 0x0008; }
            if address == 0x001C { address = 0x000C; }
            //println!("write to palette {}, {}", address, data);
            self.palette[address as usize] = data;
        }
    }

    fn increment_scroll_x(&mut self) {
        if (self.mask.render_background() | self.mask.render_sprites()) != 0 {
            if self.vram_addr.coarse_x() == 31 {
                self.vram_addr.set_coarse_x(0);
                self.vram_addr.set_nametable_x(1 - self.vram_addr.nametable_x());
            } else {
                self.vram_addr.set_coarse_x(self.vram_addr.coarse_x() + 1);
            }
        }
    }

    fn increment_scroll_y(&mut self) {
        if (self.mask.render_background() | self.mask.render_sprites()) != 0 {
            if self.vram_addr.fine_y() < 7 {
                self.vram_addr.set_fine_y(self.vram_addr.fine_y() + 1);
            } else {
                self.vram_addr.set_fine_y(0);

                match self.vram_addr.coarse_y() {
                    29 => {
                        self.vram_addr.set_coarse_y(0);
                        let pre = self.vram_addr.nametable_y();
                        self.vram_addr.set_nametable_y(1 - self.vram_addr.nametable_y());
                        //println!("pre {} post {}", pre, self.vram_addr.nametable_y());
                    }
                    31 => {
                        self.vram_addr.set_coarse_y(0);
                    }
                    _ => {
                        self.vram_addr.set_coarse_y(self.vram_addr.coarse_y() + 1);
                    }
                }
            }
        }
    }

    fn transfer_address_x(&mut self) {
        if (self.mask.render_background() | self.mask.render_sprites()) != 0 {
            self.vram_addr.set_nametable_x(self.tram_addr.nametable_x());
            self.vram_addr.set_coarse_x(self.tram_addr.coarse_x());
        }
    }

    fn transfer_address_y(&mut self) {
        if (self.mask.render_background() | self.mask.render_sprites()) != 0 {
            self.vram_addr.set_fine_y(self.tram_addr.fine_y());
            self.vram_addr.set_nametable_y(self.tram_addr.nametable_y());
            self.vram_addr.set_coarse_y(self.tram_addr.coarse_y());
        }
    }

    fn load_background_shifters(&mut self) {
        self.bg_shifter_pattern_lo = (self.bg_shifter_pattern_lo & 0xFF00) | (self.bg_next_tile_lsb as u16);
        self.bg_shifter_pattern_hi = (self.bg_shifter_pattern_hi & 0xFF00) | (self.bg_next_tile_msb as u16);

        fn expand(n: u8) -> u16 {
            if n == 0 { 0 } else { 0x00FF }
        }

        self.bg_shifter_attrib_lo = (self.bg_shifter_attrib_lo & 0xFF00) | expand(self.bg_next_tile_attrib & 0b01);
        self.bg_shifter_attrib_hi = (self.bg_shifter_attrib_hi & 0xFF00) | expand(self.bg_next_tile_attrib & 0b10);
    }

    fn update_shifters(&mut self) {
        if self.mask.render_background() != 0 {
            self.bg_shifter_pattern_lo <<= 1;
            self.bg_shifter_pattern_hi <<= 1;

            self.bg_shifter_attrib_lo <<= 1;
            self.bg_shifter_attrib_hi <<= 1;
        }
    }

    pub fn clock(&mut self, bus: &Bus) {
        if self.scanline >= -1 && self.scanline < 240 {
            if self.scanline == 0 && self.cycle == 0 {
                // Odd frame cycle skip
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 1 {
                self.status.set_vertical_blank(0);
            }

            // TODO this is NTSC timings but I want PAL
            if (self.cycle >= 2 && self.cycle < 258) || (self.cycle >= 321 && self.cycle < 338) {
                self.update_shifters();

                match (self.cycle - 1) % 8 {
                    0 => {
                        self.load_background_shifters();
                        self.bg_next_tile_id = self.ppu_read(bus, 0x2000 | (self.vram_addr.val & 0x0FFF), false);
                    }
                    2 => {
                        let mut attrib = self.ppu_read(bus, 0x23C0 | (self.vram_addr.nametable_y() << 11) | (self.vram_addr.nametable_x() << 10) | ((self.vram_addr.coarse_y() >> 2) << 3) | (self.vram_addr.coarse_x() >> 2), false);
                        if (self.vram_addr.coarse_y() & 0x02) != 0 {
                            attrib >>= 4;
                        }
                        if (self.vram_addr.coarse_x() & 0x02) != 0 {
                            attrib >>= 2;
                        }
                        self.bg_next_tile_attrib = attrib & 0x03;
                    }
                    4 => {
                        self.bg_next_tile_lsb = self.ppu_read(bus, ((self.control.pattern_background() as u16) << 12) as u16 + ((self.bg_next_tile_id as u16) << 4) + self.vram_addr.fine_y() + 0, false);
                    }
                    6 => {
                        self.bg_next_tile_msb = self.ppu_read(bus, ((self.control.pattern_background() as u16) << 12) as u16 + ((self.bg_next_tile_id as u16) << 4) + self.vram_addr.fine_y() + 8, false);
                    }
                    7 => {
                        self.increment_scroll_x();
                    }
                    _ => {}
                }
            }

            if self.cycle == 256 {
                self.increment_scroll_y();
            }

            if self.cycle == 257 {
                self.load_background_shifters();
                self.transfer_address_x();
            }

            if self.cycle == 338 || self.cycle == 340 {
                // Garbage reads
                self.bg_next_tile_id = self.ppu_read(bus, 0x2000 | (self.vram_addr.val & 0x0FFF), false);
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305 {
                self.transfer_address_y();
            }
        }

        if self.scanline == 240 {
            // nothing lol
        }

        if self.scanline >= 241 && self.scanline < 261 {
            if self.scanline == 241 && self.cycle == 1 {
                self.status.set_vertical_blank(1);
                if self.control.enable_nmi() != 0 {
                    self.send_nmi = true;
                }
            }
        }

        let mut bg_pixel = 0u8;
        let mut bg_palette = 0u8;

        if self.mask.render_background() != 0 {
            let bit_mux = 0x8000 >> self.fine_x;

            let p0_pixel = ((self.bg_shifter_pattern_lo & bit_mux) > 0) as u8;
            let p1_pixel = ((self.bg_shifter_pattern_hi & bit_mux) > 0) as u8;
            bg_pixel = (p1_pixel << 1) | p0_pixel;

            let bg_pal0 = ((self.bg_shifter_attrib_lo & bit_mux) > 0) as u8;
            let bg_pal1 = ((self.bg_shifter_attrib_hi & bit_mux) > 0) as u8;
            bg_palette = (bg_pal1 << 1) | bg_pal0;
        }

        //TODO actual output :)
        let color = get_colour_from_palette_ram(self, bus, bg_palette, bg_pixel);
        self.output.set_pixel((self.cycle - 1) as i32, self.scanline as i32, color);

        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
    }

    pub fn reset(&mut self, bus: &Bus) {
        self.fine_x = 0;
        self.address_latch = 0;
        self.ppu_data_buffer = 0;
        self.scanline = 0;
        self.cycle = 0;

        self.bg_next_tile_id = 0;
        self.bg_next_tile_attrib = 0;
        self.bg_next_tile_lsb = 0;
        self.bg_next_tile_msb = 0;
        self.bg_shifter_pattern_lo = 0;
        self.bg_shifter_pattern_hi = 0;
        self.bg_shifter_attrib_lo = 0;
        self.bg_shifter_attrib_hi = 0;

        self.status.val = 0;
        self.mask.val = 0;
        self.control.val = 0;
        self.vram_addr.val = 0;
        self.tram_addr.val = 0;
    }
}

pub trait PpuOutput {
    fn set_pixel(&self, x: i32, y: i32, rgb: (u8, u8, u8));
}