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

mod cpu;
mod bus;
mod ppu;
mod cartdrige;
mod ines_loader;
mod mappers;
mod tools;

fn main() {
    let main_window = Rc::new(RefCell::new(MainWindow::new()));
    let mut nes = Bus::new(Rc::clone(&main_window) as Rc<dyn PpuOutput>);

    let cartdrige = load_rom_file_as_cartdrige("roms/nestest.nes");
    nes.load_cartdrige(cartdrige);
    nes.reset();

    nes.cpu.borrow_mut().pc = 0xC000;
    for i in 0 .. 750000 {
        nes.clock();
    }

    /*let mut pattern_debug_window = PatternsDebugWindow::new();
    while pattern_debug_window.window.is_open() && main_window.borrow().window.is_open() {
        let instr_prev = nes.master_clock_counter;
        while !nes.ppu.borrow().frame_complete {
            nes.clock();
        }
        println!("frame {}", nes.master_clock_counter - instr_prev);
        nes.ppu.borrow_mut().frame_complete = false;
        pattern_debug_window.update(&nes);

        main_window.borrow_mut().refresh();
    }*/

    dump_memory_contents(&nes, "mem.bin");
    dump_visual_memory_contents(&nes, "ppu_mem.bin");

    /*let code = "A2 0A 8E 00 00 A2 03 8E 01 00 AC 00 00 A9 00 18 6D 01 00 88 D0 FA 8D 02 00 EA EA EA";
    let split = code.split(" ");
    let mapped = split.map(|x| -> u8 {
        u8::from_str_radix(x, 16).unwrap()
    });

    let base_address = 0x8000u16;
    for (i, x) in mapped.enumerate() {
        nes.cpu_write(base_address + i as u16, x);
    }

    nes.cpu_write(0xFFFC, 0x00u8);
    nes.cpu_write(0xFFFD, 0x00u8);
    */
}