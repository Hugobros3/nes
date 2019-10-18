use minifb::{Window, WindowOptions};
use crate::bus::Bus;
use crate::ppu::palette::get_colour_from_palette_ram;
use crate::ppu::window_common::pack;

pub struct NametableDebugWindow {
    pub window: Window,
    buffer: Vec<u32>,
}

impl NametableDebugWindow {
    pub fn new() -> Self {
        let width = 256 * 2 as usize;
        let height = 240 * 2 as usize;

        let mut buffer: Vec<u32> = vec![0; (width * height) as usize];
        let options = WindowOptions {
            resize: false,
            ..WindowOptions::default()
        };
        let mut window = Window::new("Name tables", width as usize, height as usize, options).unwrap_or_else(|e| { panic!("{}", e); });
        return Self {
            window,
            buffer,
        };
    }

    pub fn update(&mut self, bus: &Bus) {
        let mut ppu = bus.ppu.borrow_mut();
        for nametable_x in 0..=1 {
            for nametable_y in 0..=1 {
                for coarse_x in 0..32 {
                    for coarse_y in 0..30 {
                        let pt_addr = coarse_x | (coarse_y << 5) | nametable_x << 10 | nametable_y << 11;
                        let tile_id = ppu.ppu_read(bus, 0x2000 | pt_addr as u16, true);

                        let mut attrib = ppu.ppu_read(bus, 0x23C0 | (nametable_y << 11) | (nametable_x << 10) | ((coarse_y >> 2) << 3) | (coarse_x >> 2), true);
                        if (coarse_y & 0x02) != 0 {
                            attrib >>= 4;
                        }
                        if (coarse_x & 0x02) != 0 {
                            attrib >>= 2;
                        }

                        for fine_y in 0..8 {
                            let address_low = ((ppu.control.pattern_background() as u16) << 12) as u16 + ((tile_id as u16) << 4) + fine_y + 0;
                            let address_hi =  ((ppu.control.pattern_background() as u16) << 12) as u16 + ((tile_id as u16) << 4) + fine_y + 8;
                            let lsb = ppu.ppu_read(bus, address_low, false);
                            let msb = ppu.ppu_read(bus, address_hi, false);

                            for fine_x in 0..8 {
                                let x = nametable_x * 256 + coarse_x * 8 + 7 - fine_x;
                                let y = nametable_y * 240 + coarse_y * 8 + fine_y;

                                let indexed_color: u8 = ((msb >> fine_x) & 0x1) << 1 | (lsb >> fine_x) & 0x01;
                                let palette_rgb = get_colour_from_palette_ram(&mut ppu, bus, attrib & 0b11, indexed_color);
                                self.buffer[((y) as usize * (2 * 256) + (x) as usize)] = pack(palette_rgb.0, palette_rgb.1, palette_rgb.2);
                            }
                        }
                    }
                }
                /*let width = 256 as usize;
                let height = 240 as usize;

                for x in 0..width {
                    for y in 0..height {
                        let bank: u16 = if x >= 128 { 1 } else { 0 };
                        let in_bank_x: u16 = x as u16 % 128;
                        let fine_y: u16 = y as u16 & 7;
                        let tile_col: u16 = in_bank_x / 8;
                        let tile_row: u16 = y as u16 / 8;

                        let lsb = bus.ppu.borrow_mut().ppu_read(bus, bank << 12 | tile_row << 8 | tile_col << 4 | 0 << 3 | fine_y, false);
                        let msb = bus.ppu.borrow_mut().ppu_read(bus, bank << 12 | tile_row << 8 | tile_col << 4 | 1 << 3 | fine_y, false);

                        let fine_x = 7 - (x as u8 & 7);

                        let indexed_color: u8 = ((msb >> fine_x) & 0x1) << 1 | (lsb >> fine_x) & 0x01;
                        let palette_rgb = get_colour_from_palette_ram(&mut bus.ppu.borrow(), bus, 0, indexed_color);
                        self.buffer[((y + nametable_y * 240) as usize * (2 * width) + (x + nametable_x * 256) as usize)] = pack(palette_rgb.0, palette_rgb.1, palette_rgb.2);
                    }
                }*/
            }
        }
        self.window.update_with_buffer(self.buffer.as_slice()).unwrap();
    }
}