use crate::cpu::Cpu;
use std::cell::RefCell;
use crate::ppu::{Ppu, PpuOutput};
use crate::cartdrige::Cartdrige;
use std::rc::Rc;
use crate::input::{Controllers, InputProvider};
use crate::apu::Apu;
use std::sync::mpsc::SyncSender;
use crate::apu::streaming_audio::FrameSoundBuffer;

pub struct Bus {
    pub cpu: RefCell<Cpu>,
    pub cpu_ram: RefCell<[u8; 2048]>,
    pub ppu: RefCell<Ppu>,
    pub apu: RefCell<Apu>,
    pub cartdrige: RefCell<Option<Box<dyn Cartdrige>>>,
    pub controllers: RefCell<Controllers>,

    pub master_clock_counter: u64,

    dma: RefCell<Dma>,
}

struct Dma {
    dma_page: u8,
    dma_addr: u8,
    dma_data: u8,
    is_doing_dma: bool,
    dma_dummy: bool,
}

impl Dma {
    fn new() -> Self {
        Self {
            dma_page: 0,
            dma_addr: 0,
            dma_data: 0,

            is_doing_dma: false,
            dma_dummy: true,
        }
    }
}

impl Bus where {
    pub fn new(input_provider: Rc<dyn InputProvider>, graphical_output: Rc<dyn PpuOutput>, audio_output: SyncSender<FrameSoundBuffer>) -> Self {
        let mut bus = Bus {
            cpu: RefCell::new(Cpu::new()),
            cpu_ram: RefCell::new([0; 2048]),
            ppu: RefCell::new(Ppu::new(graphical_output)),
            apu: RefCell::new(Apu::new(audio_output)),
            cartdrige: RefCell::new(Option::None),
            controllers: RefCell::new(Controllers::new(input_provider)),

            master_clock_counter: 0,

            dma: RefCell::new(Dma::new()),
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
        } else if address == 0x4015 {
            self.apu.borrow_mut().cpu_read(address, &mut data);
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
        } else if address == 0x4014 {
            let mut dma = self.dma.borrow_mut();
            dma.dma_page = data;
            dma.dma_addr = 0x00;
            dma.is_doing_dma = true;
        } else if address == 0x4016 {
            self.controllers.borrow_mut().write(address, data);
        } else if address >= 0x4000 && address <= 0x4017 {
            self.apu.borrow_mut().cpu_write(self, address, data);
        }
    }

    pub fn load_cartdrige(&mut self, cart: Box<dyn Cartdrige>) {
        *self.cartdrige.borrow_mut() = Option::Some(cart);
    }

    pub fn clock(&mut self) {
        self.ppu.borrow_mut().clock(self);
        if self.master_clock_counter % 3 == 0 {
            if self.dma.borrow().is_doing_dma {
                let mut dma = self.dma.borrow_mut();
                if dma.dma_dummy {
                    if self.master_clock_counter % 2 == 1 {
                        dma.dma_dummy = false;
                    }
                } else {
                    if self.master_clock_counter % 2 == 0 {
                        dma.dma_data = self.cpu_read((dma.dma_page as u16) << 8 | dma.dma_addr as u16, false);
                    } else {
                        self.ppu.borrow_mut().borrow_oam_raw()[dma.dma_addr as usize] = dma.dma_data;
                        dma.dma_addr = dma.dma_addr.wrapping_add(1);

                        if dma.dma_addr == 0 {
                            dma.dma_dummy = true;
                            dma.is_doing_dma = false;
                        }
                    }
                }
            } else {
                self.cpu.borrow_mut().clock(self);
            }

            self.apu.borrow_mut().clock_main(12);
            self.apu.borrow_mut().clock_cpu_clock();

            if self.apu.borrow().is_raising_interrupt() {
                self.cpu.borrow_mut().irq(self);
            }
        }

        let do_ppu_nmi = {
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