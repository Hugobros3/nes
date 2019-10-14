use crate::ppu::main_window::MainWindow;
use minifb::Key;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

pub struct Controllers {
    reading_button: u8,
    provider: Rc<dyn InputProvider>,
}

impl Controllers {
    pub fn new(input_provider: Rc<dyn InputProvider>) -> Self {
        return Controllers {
            reading_button: 0,
            provider: input_provider,
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if address == 0x4016 {
            let strobe = (data & 0x01) == 01;
            if strobe {
                self.reading_button = 0;
            }
        }
    }

    pub fn read(&mut self, address: u16, data: &mut u8) {
        *data = *data & 0xF8;
        let provider: &dyn InputProvider = self.provider.borrow();
        *data = *data | if provider.get_button_state(self.reading_button, 0) { 0x01 } else { 0x00 };
        self.reading_button += 1;
        //println!("probing controller, reply={}", *data);
    }
}

pub trait InputProvider {
    fn get_button_state(&self, button: u8, controller: u8) -> bool;
}

impl InputProvider for RefCell<MainWindow> {
    fn get_button_state(&self, button: u8, controller: u8) -> bool {
        let main_window = self.borrow();
        let window = &main_window.window;
        return match button {
            0 => {
                // A
                window.is_key_down(Key::F)
            }
            1 => {
                // B
                window.is_key_down(Key::D)
            }
            2 => {
                // Select
                window.is_key_down(Key::S)
            }
            3 => {
                // Start
                window.is_key_down(Key::Enter)
            }
            4 => {
                // Up
                window.is_key_down(Key::Up)
            }
            5 => {
                // Down
                window.is_key_down(Key::Down)
            }
            6 => {
                // Left
                window.is_key_down(Key::Left)
            }
            7 => {
                // Right
                window.is_key_down(Key::Right)
            }
            _ => { false }
        };
    }
}