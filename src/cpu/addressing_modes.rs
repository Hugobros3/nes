use crate::cpu::Cpu;
use crate::bus::Bus;

pub struct AddressingMode {
    pub name: &'static str,
    pub implementation: AddressingModeImplementation,
}

/// Executes the operations necessary to fetch data and returns a struct describing
/// the remainder work left to the instruction
pub type AddressingModeImplementation = fn(&mut Cpu, &Bus) -> AddressingResult;

pub const ADDRESSING_MODES: [AddressingMode; 12] = [
    AddressingMode { name: "IMP", implementation: am_implied },
    AddressingMode { name: "IMM", implementation: am_immediate },
    AddressingMode { name: "ZP0", implementation: am_zero_page },
    AddressingMode { name: "ZPX", implementation: am_zero_page_x_offset },
    AddressingMode { name: "ZPY", implementation: am_zero_page_y_offset },
    AddressingMode { name: "REL", implementation: am_relative },
    
    AddressingMode { name: "ABS", implementation: am_absolute },
    AddressingMode { name: "ABX", implementation: am_absolute_x },
    AddressingMode { name: "ABY", implementation: am_absolute_y },
    AddressingMode { name: "IND", implementation: am_indirect },
    AddressingMode { name: "IZX", implementation: am_indexed_indirect },
    AddressingMode { name: "IZY", implementation: am_indirect_indexed },
];

// Pretty names to reference over in instructions.rs
pub const IMP: &'static AddressingMode = &ADDRESSING_MODES[0];
pub const IMM: &'static AddressingMode = &ADDRESSING_MODES[1];
pub const ZP0: &'static AddressingMode = &ADDRESSING_MODES[2];
pub const ZPX: &'static AddressingMode = &ADDRESSING_MODES[3];
pub const ZPY: &'static AddressingMode = &ADDRESSING_MODES[4];
pub const REL: &'static AddressingMode = &ADDRESSING_MODES[5];
pub const ABS: &'static AddressingMode = &ADDRESSING_MODES[6];
pub const ABX: &'static AddressingMode = &ADDRESSING_MODES[7];
pub const ABY: &'static AddressingMode = &ADDRESSING_MODES[8];
pub const IND: &'static AddressingMode = &ADDRESSING_MODES[9];
pub const IZX: &'static AddressingMode = &ADDRESSING_MODES[10];
pub const IZY: &'static AddressingMode = &ADDRESSING_MODES[11];

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

fn am_implied(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
    return AddressingResult::Implicit {data: cpu.a }
}

fn am_immediate(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
    let address = cpu.pc;
    cpu.pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

fn am_zero_page(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
    let mut address = bus.read(cpu.pc, false) as u16;
    address &= 0x00FFu16;
    cpu.pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

fn am_zero_page_x_offset(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
    let mut address = bus.read(cpu.pc, false) as u16;
    address += cpu.x as u16;
    address &= 0x00FFu16;
    cpu.pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

fn am_zero_page_y_offset(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
    let mut address = bus.read(cpu.pc, false) as u16;
    address += cpu.y as u16;
    address &= 0x00FFu16;
    cpu.pc+=1;
    return AddressingResult::ReadFrom { address, cycles:0 }
}

fn am_absolute(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
    let low = bus.read(cpu.pc, false);
    cpu.pc+=1;
    let hi = bus.read(cpu.pc, false);
    cpu.pc+=1;
    let address = ((hi as u16) << 8) | (low as u16);
    return AddressingResult::ReadFrom { address, cycles:0 }
}

fn am_absolute_x(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
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

fn am_absolute_y(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
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

/// Fetches a pointer then fetches the address from there
fn am_indirect(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
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

/// Fetches a pointer, adds x to it, fetches the address from the result
fn am_indexed_indirect(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
    let mut ptr = bus.read(cpu.pc, false) as u16;
    cpu.pc+=1;

    ptr += cpu.x as u16;

    let address_lo = bus.read(ptr & 0x00FF, false);
    let address_hi = bus.read((ptr + 1) & 0x00FF, false);

    let address = ((address_hi as u16) << 8) | (address_lo as u16);

    return AddressingResult::ReadFrom { address, cycles: 0 }
}

/// Fetches a pointer, fetches the address from it, *then* adds y to it.
fn am_indirect_indexed(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
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

/// Fetches relative to the program counter (USED BY BRANCHING INSTRUCTIONS ONLY)
fn am_relative(cpu: &mut Cpu, bus: &Bus) -> AddressingResult {
    let mut address_rel = bus.read(cpu.pc, false) as u16;
    cpu.pc += 1;

    // Extend sign
    if address_rel & 0x80u16 != 0u16 {
        address_rel |= 0xFF00u16;
    }

    return AddressingResult::ProgramCounterRelative {address_rel: address_rel}
}