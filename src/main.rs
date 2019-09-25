//#![feature(const_fn)]
#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate bitutils;

use crate::bus::Bus;

use std::str::*;
use std::slice::*;
use crate::cpu::Cpu;
use crate::ines_loader::load_rom_file_as_cartdrige;
use crate::ppu::debug::show_patterns_debug_window;

mod cpu;
mod bus;
mod ppu;
mod cartdrige;
mod ines_loader;
pub mod mappers;

fn main() {
    let mut nes = Bus::new();

    let cartdrige = load_rom_file_as_cartdrige("roms/smb.nes");
    nes.load_cartdrige(cartdrige);

    show_patterns_debug_window(&nes);

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

    Cpu::reset(&nes);

    for i in 0..320 {
        Cpu::clock(&nes);
    }
}