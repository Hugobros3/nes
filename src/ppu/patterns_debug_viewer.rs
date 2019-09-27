use minifb::{WindowOptions, Window};
use crate::bus::Bus;
use crate::ppu::palette::get_colour_from_palette_ram;
use std::borrow::Borrow;
use crate::ppu::window_common::*;

pub struct PatternsDebugWindow {
    pub window: Window,
    buffer: Vec<u32>,
}

impl PatternsDebugWindow {
    pub fn new() -> Self {
        let width = 256 as usize;
        let height = 128 as usize;

        let mut buffer: Vec<u32> = vec![0; (width * height) as usize];
        let options = WindowOptions {
            resize: false,
            ..WindowOptions::default()
        };
        let mut window = Window::new("Pattern tables", width as usize, height as usize, options).unwrap_or_else(|e| { panic!("{}", e); });
        return PatternsDebugWindow {
            window,
            buffer
        };
    }

    pub fn update(&mut self, bus: &Bus) {
        let width = 256 as usize;
        let height = 128 as usize;
        /*
            DCBA98 76543210
            ---------------
            0HRRRR CCCCPTTT
            |||||| |||||+++- T: Fine Y offset, the row number within a tile
            |||||| ||||+---- P: Bit plane (0: "lower"; 1: "upper")
            |||||| ++++----- C: Tile column
            ||++++---------- R: Tile row
            |+-------------- H: Half of sprite table (0: "left"; 1: "right")
            +--------------- 0: Pattern table is at $0000-$1FFF
        */

        //while window.is_open() {
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
                let palette_rgb = get_colour_from_palette_ram(bus.ppu.borrow().borrow(), bus, 0, indexed_color);
                self.buffer[(y as usize * width + x as usize)] = pack(palette_rgb.0, palette_rgb.1, palette_rgb.2);
                //TODO use real palettes
                /*let color = match (indexed_color) {
                    0 => Color(0.0, 0.0, 0.0),
                    1 => Color(1.0, 0.0, 0.0),
                    2 => Color(0.0, 1.0, 0.0),
                    _ => Color(0.0, 0.0, 1.0),
                };
                self.buffer[(y as usize * width + x as usize)] = rgb(&color);*/
            }
        }
        self.window.update_with_buffer(self.buffer.as_slice()).unwrap();
        //}
    }
}