use minifb::{WindowOptions, Window};
use crate::bus::Bus;

pub fn show_patterns_debug_window(bus: &Bus) {
    let width = 256 as usize;
    let height = 128 as usize;

    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];
    let options = WindowOptions {
        resize: false,
        ..WindowOptions::default()
    };
    let mut window = Window::new("", width as usize, height as usize, options).unwrap_or_else(|e| { panic!("{}", e); });

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

    while window.is_open() {
        for x in 0..width {
            for y in 0..height {
                let bank: u16 = if x >= 128 { 1 } else { 0 };
                let in_bank_x: u16 = x as u16 % 128;
                let fine_y: u16 = y as u16 & 7;
                let tile_col: u16 = in_bank_x / 8;
                let tile_row: u16 = y as u16 / 8;

                let lsb = bus.ppu_read(bank << 12 | tile_row << 8 | tile_col << 4 | 0 << 3 | fine_y, false);
                let msb = bus.ppu_read(bank << 12 | tile_row << 8 | tile_col << 4 | 1 << 3 | fine_y, false);

                let fine_x = 7 - (x as u16 & 7);

                let indexed_color: u8 = ((msb >> fine_x) & 0x1) << 1 | (lsb >> fine_x) & 0x01;
                let color = match (indexed_color) {
                    0 => Color(0.0, 0.0, 0.0),
                    1 => Color(1.0, 0.0, 0.0),
                    2 => Color(0.0, 1.0, 0.0),
                    _ => Color(0.0, 0.0, 1.0),
                };

                //buffer[y * width + x] = rgb(&Color(x as f32 / 256f32, 0f32, 0f32));
                buffer[(y as usize * width + x as usize)] = rgb(&color);
            }
        }
        window.update_with_buffer(buffer.as_slice()).unwrap();
    }
}

pub struct Color(pub f32, pub f32, pub f32);

fn rgb(color: &Color) -> u32 {
    let r = clamp((color.0 * 256.0) as u32, 0, 255);
    let g = clamp((color.1 * 256.0) as u32, 0, 255);
    let b = clamp((color.2 * 256.0) as u32, 0, 255);
    r << 16 | g << 8 | b << 0
}

fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min { min } else if v > max { max } else { v }
}