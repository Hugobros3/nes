use crate::cpu::R6502;
use crate::bus::Bus;

pub type AddressingMode = fn(&mut R6502, &Bus) -> AddressingResult;

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
    ProgramCounterRelative {
        address_rel: u16
    }
}

pub fn IMP(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    return AddressingResult::Implicit {data: cpu.a }
}

pub fn IMM(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let address = cpu.pc;
    cpu.pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ZP0(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let mut address = bus.read(cpu.pc, false) as u16;
    address &= 0x00FFu16;
    cpu.pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ZPX(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let mut address = bus.read(cpu.pc, false) as u16;
    address += cpu.x as u16;
    address &= 0x00FFu16;
    cpu.pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ZPY(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let mut address = bus.read(cpu.pc, false) as u16;
    address += cpu.y as u16;
    address &= 0x00FFu16;
    cpu.pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ABS(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let low = bus.read(cpu.pc, false);
    cpu.pc+=1;
    let hi = bus.read(cpu.pc, false);
    cpu.pc+=1;
    let address = ((hi as u16) << 8) | (low as u16);
    return AddressingResult::ReadFrom { address, cycles:0 }
}

pub fn ABX(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let low = bus.read(cpu.pc, false);
    cpu.pc+=1;
    let hi = bus.read(cpu.pc, false);
    cpu.pc+=1;

    let address = ((hi as u16) << 8) | (low as u16);
    let offseted_address = address + cpu.x as u16;

    let og_page = address >> 8;
    let offseted_page = offseted_address >> 8;

    let additional_cycles = if(og_page != offseted_page) { 1 } else { 0 };

    return AddressingResult::ReadFrom { address, cycles: additional_cycles }
}

pub fn ABY(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let low = bus.read(cpu.pc, false);
    cpu.pc+=1;
    let hi = bus.read(cpu.pc, false);
    cpu.pc+=1;

    let address = ((hi as u16) << 8) | (low as u16);
    let offseted_address = address + cpu.y as u16;

    let og_page = address >> 8;
    let offseted_page = offseted_address >> 8;

    let additional_cycles = if(og_page != offseted_page) { 1 } else { 0 };

    return AddressingResult::ReadFrom { address, cycles: additional_cycles }
}

pub fn IND(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let ptr_low = bus.read(cpu.pc, false);
    cpu.pc+=1;
    let ptr_hi = bus.read(cpu.pc, false);
    cpu.pc+=1;

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

pub fn IZX(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let mut ptr = bus.read(cpu.pc, false) as u16;
    cpu.pc+=1;

    ptr += cpu.x as u16;

    let address_lo = bus.read(ptr & 0x00FF, false);
    let address_hi = bus.read((ptr + 1) & 0x00FF, false);

    let address = ((address_hi as u16) << 8) | (address_lo as u16);

    return AddressingResult::ReadFrom { address, cycles: 0 }
}

pub fn IZY(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let mut ptr = bus.read(cpu.pc, false) as u16;
    cpu.pc+=1;

    let address_lo = bus.read(ptr & 0x00FF, false);
    let address_hi = bus.read((ptr + 1) & 0x00FF, false);

    let address = ((address_hi as u16) << 8) | (address_lo as u16);
    let offseted_address = address + cpu.y as u16;

    let og_page = address >> 8;
    let offseted_page = offseted_address >> 8;

    let additional_cycles = if(og_page != offseted_page) { 1 } else { 0 };

    return AddressingResult::ReadFrom { address, cycles: additional_cycles }
}

pub fn REL(cpu: &mut R6502, bus: &Bus) -> AddressingResult {
    let mut address_rel = bus.read(cpu.pc, false) as u16;
    cpu.pc += 1;

    // Extend sign
    if address_rel & 0x80u16 != 0u16 {
        address_rel |= 0xFF00u16;
    }

    return AddressingResult::ProgramCounterRelative {address_rel: address_rel}
}