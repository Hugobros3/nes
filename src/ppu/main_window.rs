use minifb::{Window, WindowOptions};
use crate::ppu::patterns_debug_viewer::PatternsDebugWindow;
use crate::ppu::window_common::{Color, pack};
use crate::ppu::PpuOutput;
use std::cell::RefCell;

pub struct MainWindow {
    pub window: Window,
    buffer: Vec<u32>,
}

impl MainWindow {
    pub fn new() -> Self {
        let width = 256 as usize;
        let height = 240 as usize;

        let mut buffer: Vec<u32> = vec![0; (width * height) as usize];
        let options = WindowOptions {
            resize: false,
            ..WindowOptions::default()
        };
        let mut window = Window::new("Output", width as usize, height as usize, options).unwrap_or_else(|e| { panic!("{}", e); });
        return MainWindow {
            window,
            buffer
        };
    }

    pub fn refresh(&mut self) {
        self.window.update_with_buffer(self.buffer.as_slice()).unwrap();
    }

    fn set_pixel(&mut self, x: i32, y: i32, rgb: (u8, u8, u8)) {
        let width = 256 as usize;
        let height = 240 as usize;
        if x >= 0 && x < 256 {
            if y >= 0 && y < 240 {
                self.buffer[(y as usize * width + x as usize)] = pack(rgb.0, rgb.1, rgb.2);
            }
        }
    }
}

impl PpuOutput for RefCell<MainWindow> {
    fn set_pixel(&self, x: i32, y: i32, rgb: (u8, u8, u8)) {
        self.borrow_mut().set_pixel(x, y, rgb);
    }
}