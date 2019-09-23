pub trait Cartdrige {
    fn cpu_read(&self, address: u16, data: &mut u8) -> bool;
    fn cpu_write(&self, address: u16, data: u8) -> bool;

    fn ppu_read(&self, address: u16, data: &mut u8) -> bool;
    fn ppu_write(&self, address: u16, data: u8) -> bool;
}