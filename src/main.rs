//#![feature(const_fn)]
#[macro_use]
extern crate bitflags;

use crate::bus::Bus;

mod cpu;
mod bus;

fn main() {
    let nes = Bus::new();
    println!("Hello, world!");
}
