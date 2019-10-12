use crate::cpu::Cpu;
use std::cell::RefCell;
use crate::ppu::{Ppu, PpuOutput};
use crate::cartdrige::Cartdrige;
use std::rc::Rc;
use crate::input::{Controllers, InputProvider};

pub struct Bus {
    pub cpu: RefCell<Cpu>,
    pub cpu_ram: RefCell<[u8; 2048]>,
    pub ppu: RefCell<Ppu>,
    pub cartdrige: RefCell<Option<Box<dyn Cartdrige>>>,
    pub controllers: RefCell<Controllers>,

    pub master_clock_counter: u64,
}

impl Bus where {
    pub fn new(input_provider: Rc<dyn InputProvider>, graphical_output: Rc<dyn PpuOutput>) -> Self {
        let mut bus = Bus {
            cpu: RefCell::new(Cpu::new()),
            cpu_ram: RefCell::new([0; 2048]),
            ppu: RefCell::new(Ppu::new(graphical_output)),
            cartdrige: RefCell::new(Option::None),
            controllers: RefCell::new(Controllers::new(input_provider)),

            master_clock_counter: 0,
        };

        bus.reset();

        return bus;
    }

    pub fn cpu_read(&self, address: u16, read_only: bool) -> u8 {
        let mut data = 0_u8;
        {
            let mut cart_brw = self.cartdrige.borrow_mut();
            let cart_ref = cart_brw.as_mut();
            if cart_ref.is_some() && cart_ref.unwrap().cpu_read(address, &mut data) {
                return data;
            }
        }

        if address >= 0x0000u16 && address < 0x1FFFu16 {
            data = self.cpu_ram.borrow()[(address & 0x07FF) as usize]
        } else if address >= 0x2000u16 && address < 0x3FFFu16 {
            data = self.ppu.borrow_mut().read_ppu_register(self, address & 0x0007, read_only);
        } else if address >= 0x4016 && address <= 0x4017 {
            self.controllers.borrow_mut().read(address, &mut data);
        }

        return data;
    }

    pub fn cpu_write(&self, address: u16, data: u8) {
        {
            let mut cart_brw = self.cartdrige.borrow_mut();
            let cart_ref = cart_brw.as_mut();
            if cart_ref.is_some() && cart_ref.unwrap().cpu_write(address, data) {
                return;
            }
        }
        if address >= 0x0000u16 && address < 0x1FFFu16 {
            self.cpu_ram.borrow_mut()[(address & 0x07FF) as usize] = data;
            //println!("write ok {}, {}", address, data);
        } else if address >= 0x2000u16 && address < 0x3FFFu16 {
            self.ppu.borrow_mut().write_ppu_register(self, address & 0x0007, data);
        } else if address >= 0x4016 && address <= 0x4017 {
            self.controllers.borrow_mut().write(address, data);
        }
    }

    pub fn load_cartdrige(&mut self, cart: Box<dyn Cartdrige>) {
        *self.cartdrige.borrow_mut() = Option::Some(cart);
    }

    pub fn clock(&mut self) {
        self.ppu.borrow_mut().clock(self);
        if self.master_clock_counter % 3 == 0 {
            self.cpu.borrow_mut().clock(self);
        }

        let do_ppu_nmi =
            {
                let mut ppu = self.ppu.borrow_mut();//.borrow_mut();
                if ppu.send_nmi {
                    ppu.send_nmi = false;
                    true
                } else {
                    false
                }
            };
        if do_ppu_nmi {
            //println!("nmi!");
            self.cpu.borrow_mut().nmi(self);
        }

        self.master_clock_counter += 1;
    }

    pub fn reset(&mut self) {
        self.cpu.borrow_mut().reset(self);
        self.ppu.borrow_mut().reset(self);
        self.master_clock_counter = 0;
    }
}