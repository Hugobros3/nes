#[macro_use]
extern crate bitutils;

use crate::bus::Bus;

use std::str::*;
use std::slice::*;
use crate::cpu::Cpu;
use crate::ines_loader::load_rom_file_as_cartdrige;
use crate::ppu::patterns_debug_viewer::PatternsDebugWindow;
use crate::tools::{dump_memory_contents, dump_visual_memory_contents};
use crate::ppu::main_window::MainWindow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ppu::PpuOutput;
use std::env;
use crate::input::InputProvider;

mod bus;
mod cpu;
mod ppu;
mod input;
mod cartdrige;

mod ines_loader;
mod mappers;

mod tools;

fn main() {
    let main_window = Rc::new(RefCell::new(MainWindow::new()));
    let mut nes = Bus::new(
        Rc::clone(&main_window) as Rc<dyn InputProvider>,
        Rc::clone(&main_window) as Rc<dyn PpuOutput>,
    );

    let cartdrige = load_rom_file_as_cartdrige("roms/nestest.nes");
    nes.load_cartdrige(cartdrige);
    nes.reset();

    let mut args = env::args();
    println!("{:?}", args.next());
    if args.next().is_none() {
        let mut pattern_debug_window = PatternsDebugWindow::new();
        while pattern_debug_window.window.is_open() && main_window.borrow().window.is_open() {
            let instr_prev = nes.master_clock_counter;
            while !nes.ppu.borrow().frame_complete {
                nes.clock();
            }
            println!("frame {}", nes.master_clock_counter - instr_prev);
            nes.ppu.borrow_mut().frame_complete = false;
            pattern_debug_window.update(&nes);

            main_window.borrow_mut().refresh();
        }
    } else {
        nes.cpu.borrow_mut().pc = 0xC000;
        for i in 0..750000 {
            nes.clock();
        }
    }

    dump_memory_contents(&nes, "mem.bin");
    dump_visual_memory_contents(&nes, "ppu_mem.bin");
}