//#![feature(const_fn)]
#[macro_use]
extern crate bitflags;

use crate::bus::Bus;

mod cpu;
mod bus;

fn main() {
    let nes = Bus::new();

    let mut a = 9;
    t(&mut a);

    print!("{}", a)
}

fn t(r: &mut i32) {
    *r *= 2;
    z(r);
    *r /= 2;
}


fn z(r: &mut i32) {
    *r += 1;
}
