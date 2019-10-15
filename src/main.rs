#[macro_use]
extern crate bitutils;

use crate::bus::Bus;

use crate::ines_loader::load_rom_file_as_cartdrige;
use crate::ppu::patterns_debug_viewer::PatternsDebugWindow;
use crate::tools::{dump_memory_contents, dump_visual_memory_contents};
use crate::ppu::main_window::MainWindow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ppu::PpuOutput;
use crate::apu::streaming_audio::{launch_sound, garbage_test};
use std::env;
use crate::input::InputProvider;
use crate::ppu::nametables_debug_viewer::NametableDebugWindow;
use std::time::{Instant, Duration};
use std::ops::Sub;

mod bus;
mod cpu;
mod ppu;
mod apu;
mod input;
mod cartdrige;

mod mappers;
mod ines_loader;

mod tools;

fn main() {
    let mut audio_tx = launch_sound();

    let mut args = env::args();

    let main_window = Rc::new(RefCell::new(MainWindow::new()));
    let mut nes = Bus::new(
        Rc::clone(&main_window) as Rc<dyn InputProvider>,
        Rc::clone(&main_window) as Rc<dyn PpuOutput>,
        audio_tx
    );

    let cartridge = load_rom_file_as_cartdrige("roms/smb.nes");
    nes.load_cartdrige(cartridge);
    nes.reset();

    let nestest_mode = args.find(|i| { i == "--nestest"}).is_some();
    let unlimited_speed = args.find(|i| { i == "-u"}).is_some();
    let fps = 60;

    let ideal_frame_duration = Duration::from_micros(1_000_000 / fps);

    if !nestest_mode {
        let mut pattern_debug_window = PatternsDebugWindow::new();
        let mut nametable_debug_window = NametableDebugWindow::new();

        while main_window.borrow().window.is_open() {
            let frame_start_time = Instant::now();

            let instr_prev = nes.master_clock_counter;
            while !nes.ppu.borrow().frame_complete {
                nes.clock();
            }
            nes.apu.borrow_mut().frame_done();
            //println!("{}", nes.master_clock_counter - instr_prev);

            nes.ppu.borrow_mut().frame_complete = false;
            pattern_debug_window.update(&nes);
            nametable_debug_window.update(&nes);
            main_window.borrow_mut().refresh();

            let frame_done_time = Instant::now();
            let frame_computing_duration = Instant::duration_since(&frame_done_time, frame_start_time);

            if frame_computing_duration < ideal_frame_duration {
                let sleep_duration = ideal_frame_duration.sub(frame_computing_duration);
                spin_sleep::sleep(sleep_duration);
            }
        }
    } else {
        // Jump to nestest routine
        nes.cpu.borrow_mut().pc = 0xC000;
        for i in 0..750000 {
            nes.clock();
        }
    }

    dump_memory_contents(&nes, "mem.bin");
    dump_visual_memory_contents(&nes, "ppu_mem.bin");
}
