use crate::cpu::Cpu;
use std::cell::RefCell;
use crate::ppu::Ppu;
use crate::cartdrige::Cartdrige;

pub struct Bus {
    pub cpu: RefCell<Cpu>,
    pub cpu_ram: RefCell<[u8; 2048]>,
    pub ppu: RefCell<Ppu>,
    pub cartdrige: Option<Box<dyn Cartdrige>>,
}

impl Bus {
    pub fn new() -> Self {
        return Bus {
            cpu: RefCell::new(Cpu::new()),
            cpu_ram: RefCell::new([0; 2048]),
            ppu: RefCell::new(Ppu::new()),
            cartdrige: Option::None,
        };
    }

    pub fn cpu_write(&self, address: u16, data: u8) {
        if self.cartdrige.is_some() && self.cartdrige.unwrap().cpu_write(address, data) {}
        else if address >= 0x0000u16 && address < 0x1FFFu16 {
            self.cpu_ram.borrow_mut()[(address & 0x07FF) as usize] = data
        } else if address >= 0x2000u16 && address < 0x3FFFu16 {
            self.ppu.borrow_mut().cpu_write(address & 0x0007, data);
        }
    }

    pub fn cpu_read(&self, address: u16, read_only: bool) -> u8 {
        let mut data = 0_u8;

        if self.cartdrige.is_some() && self.cartdrige.unwrap().cpu_read(address, &mut data) {}
        if address >= 0x0000u16 && address < 0x1FFFu16 {
            data = self.cpu_ram.borrow()[(address & 0x07FF) as usize]
        } else if address >= 0x2000u16 && address < 0x3FFFu16 {
            data = self.ppu.borrow_mut().cpu_read(address & 0x0007, read_only);
        }

        return data;
    }
}