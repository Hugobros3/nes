pub trait Cartdrige {
    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool;
    fn cpu_write(&mut self, address: u16, data: u8) -> bool;

    fn ppu_read(&mut self, address: u16, data: &mut u8) -> bool;
    fn ppu_write(&mut self, address: u16, data: u8) -> bool;
}