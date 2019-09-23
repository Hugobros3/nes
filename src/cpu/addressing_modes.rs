use crate::cpu::R6502;
use crate::bus::Bus;

pub type AddressingMode = fn(&Bus) -> AddressingResult;

/*enum AddressingMdode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY
}*/

pub enum AddressingResult {
    Implicit {
        data: u8
    },
    ReadFrom {
        address: u16,
        cycles: i8,
    },
    Relative {
        address_rel: u16
    }
}

pub fn IMP(bus: &Bus) -> AddressingResult {
    return AddressingResult::Implicit {data: bus.cpu.borrow().a }
}

pub fn IMM(bus: &Bus) -> AddressingResult {
    let address = bus.cpu.borrow().pc;
    bus.cpu.borrow_mut().pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ZP0(bus: &Bus) -> AddressingResult {
    let mut address = bus.read(bus.cpu.borrow().pc, false) as u16;
    address &= 0x00FFu16;
    bus.cpu.borrow_mut().pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ZPX(bus: &Bus) -> AddressingResult {
    let mut address = bus.read(bus.cpu.borrow().pc, false) as u16;
    address += bus.cpu.borrow().x as u16;
    address &= 0x00FFu16;
    bus.cpu.borrow_mut().pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ZPY(bus: &Bus) -> AddressingResult {
    let mut address = bus.read(bus.cpu.borrow().pc, false) as u16;
    address += bus.cpu.borrow().y as u16;
    address &= 0x00FFu16;
    bus.cpu.borrow_mut().pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ABS(bus: &Bus) -> AddressingResult {
    let low = bus.read(bus.cpu.borrow().pc, false);
    bus.cpu.borrow_mut().pc+=1;
    let hi = bus.read(bus.cpu.borrow().pc, false);
    bus.cpu.borrow_mut().pc+=1;
    let address = ((hi as u16) << 8) | (low as u16);
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ABX(bus: &Bus) -> AddressingResult {
    let low = bus.read(bus.cpu.borrow().pc, false);
    bus.cpu.borrow_mut().pc+=1;
    let hi = bus.read(bus.cpu.borrow().pc, false);
    bus.cpu.borrow_mut().pc+=1;

    let address = ((hi as u16) << 8) | (low as u16);
    let offseted_address = address + bus.cpu.borrow().x as u16;

    let og_page = address >> 8;
    let offseted_page = offseted_address >> 8;

    let additional_cycles = if(og_page != offseted_page) { 1 } else { 0 };

    return AddressingResult::ReadFrom { address, cycles: additional_cycles }
}

pub fn ABY(bus: &Bus) -> AddressingResult {
    let low = bus.read(bus.cpu.borrow().pc, false);
    bus.cpu.borrow_mut().pc+=1;
    let hi = bus.read(bus.cpu.borrow().pc, false);
    bus.cpu.borrow_mut().pc+=1;

    let address = ((hi as u16) << 8) | (low as u16);
    let offseted_address = address + bus.cpu.borrow().y as u16;

    let og_page = address >> 8;
    let offseted_page = offseted_address >> 8;

    let additional_cycles = if(og_page != offseted_page) { 1 } else { 0 };

    return AddressingResult::ReadFrom { address, cycles: additional_cycles }
}

pub fn IND(bus: &Bus) -> AddressingResult {
    let ptr_low = bus.read(bus.cpu.borrow().pc, false);
    bus.cpu.borrow_mut().pc+=1;
    let ptr_hi = bus.read(bus.cpu.borrow().pc, false);
    bus.cpu.borrow_mut().pc+=1;

    let ptr = ((ptr_hi as u16) << 8) | (ptr_low as u16);

    let address_lo = bus.read(ptr, false);

    let address_hi = if(ptr_low == 0xFFu8) {
         bus.read((ptr & 0xFF00), false)
    } else {
        bus.read(ptr + 1, false)
    };

    let address = ((address_hi as u16) << 8) | (address_lo as u16);

    return AddressingResult::ReadFrom { address, cycles: 0 }
}

pub fn IZX(bus: &Bus) -> AddressingResult {
    let mut ptr = bus.read(bus.cpu.borrow().pc, false) as u16;
    bus.cpu.borrow_mut().pc+=1;

    ptr += bus.cpu.borrow().x as u16;

    let address_lo = bus.read(ptr & 0x00FF, false);
    let address_hi = bus.read((ptr + 1) & 0x00FF, false);

    let address = ((address_hi as u16) << 8) | (address_lo as u16);

    return AddressingResult::ReadFrom { address, cycles: 0 }
}

pub fn IZY(bus: &Bus) -> AddressingResult {
    let mut ptr = bus.read(bus.cpu.borrow().pc, false) as u16;
    bus.cpu.borrow_mut().pc+=1;

    let address_lo = bus.read(ptr & 0x00FF, false);
    let address_hi = bus.read((ptr + 1) & 0x00FF, false);

    let address = ((address_hi as u16) << 8) | (address_lo as u16);
    let offseted_address = address + bus.cpu.borrow().y as u16;

    let og_page = address >> 8;
    let offseted_page = offseted_address >> 8;

    let additional_cycles = if(og_page != offseted_page) { 1 } else { 0 };

    return AddressingResult::ReadFrom { address, cycles: additional_cycles }
}

pub fn REL(bus: &Bus) -> AddressingResult {
    let mut address_rel = bus.read(bus.cpu.borrow().pc, false) as u16;
    bus.cpu.borrow_mut().pc += 1;

    // Extend sign
    if address_rel & 0x80u16 != 0u16 {
        address_rel |= 0xFF00u16;
    }

    return AddressingResult::Relative {address_rel: address_rel}
}