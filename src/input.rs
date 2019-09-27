use crate::ppu::main_window::MainWindow;
use minifb::Key;
use std::rc::Rc;
use rand::distributions::uniform::SampleBorrow;
use std::cell::RefCell;

pub struct Input {
    reading_button: u8,
    provider: Rc<dyn InputProvider>,
}

pub trait InputProvider {
    fn get_button_state(&self, button: u8, controller: u8) -> bool ;
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