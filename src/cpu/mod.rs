use crate::bus::Bus;

pub struct R6502 {
    flags: CpuStateFlags,
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,

    rem_cycles: i8,
}

bitflags! {
    pub struct CpuStateFlags: u8 {
        const C = 0b00000001; // Carry
        const Z = 0b00000010; // Zero
        const I = 0b00000100; // Disable interrupts
        const D = 0b00001000; // BCD (unused)
        const B = 0b00010000; // Break
        const U = 0b00100000; // Unused
        const V = 0b01000000; // Overflow
        const N = 0b10000000; // Negative

        const Init = 0x00_u8 | 0b00100000; // Initial state at reset
    }
}

mod addressing_modes;
mod instructions;

use crate::cpu::instructions::*;
use crate::cpu::addressing_modes::{AddressingResult, AddressingMode};

impl R6502 {
    pub(crate) fn new() -> Self {
        return R6502 {
            flags: CpuStateFlags::Init,
            a: 0x00u8,
            x: 0x00u8,
            y: 0x00u8,
            sp: 0x00u8,
            pc: 0x0000u16,

            rem_cycles: 0,
        };
    }

    fn clock(&mut self, bus: &Bus) {
        if (self.rem_cycles == 0) {
            let opcode = bus.read(self.pc, false);
            self.pc += 1;

            let instruction = &INSTRUCTIONS[opcode as usize];
            self.rem_cycles = instruction.cycles;

            //TODO cycles
        }

        self.rem_cycles -= 1;
    }

    fn reset(bus: &mut Bus) {
        bus.cpu.a = 0;
        bus.cpu.x = 0;
        bus.cpu.y = 0;

        bus.cpu.sp = 0xFD;
        bus.cpu.flags = CpuStateFlags::Init;

        let reset_vector = 0xFFFCu16;
        let lo = bus.read(reset_vector, false) as u16;
        let hi = bus.read(reset_vector + 1, false) as u16;

        bus.cpu.pc = (hi << 8) | lo;

        bus.cpu.rem_cycles = 8;
    }

    fn irq(bus: &mut Bus) {
        if !CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::I) {
            bus.write(0x0100 + bus.cpu.sp as u16, (bus.cpu.pc >> 8) as u8);
            bus.cpu.sp -= 1;
            bus.write(0x0100 + bus.cpu.sp as u16, (bus.cpu.pc & 0x00FFu16) as u8);
            bus.cpu.sp -= 1;

            CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::B, false);
            CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::U, true);
            CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::I, true);
            bus.write(0x0100 + bus.cpu.sp as u16, bus.cpu.flags.bits);
            bus.cpu.sp -= 1;

            let interrupt_vector = 0xFFFEu16;
            let lo = bus.read(interrupt_vector, false) as u16;
            let hi = bus.read(interrupt_vector + 1, false) as u16;

            bus.cpu.pc = (hi << 8) | lo;

            bus.cpu.rem_cycles = 7;
        }
    }

    fn nmi(bus: &mut Bus) {
        bus.write(0x0100 + bus.cpu.sp as u16, (bus.cpu.pc >> 8) as u8);
        bus.cpu.sp -= 1;
        bus.write(0x0100 + bus.cpu.sp as u16, (bus.cpu.pc & 0x00FFu16) as u8);
        bus.cpu.sp -= 1;

        CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::B, false);
        CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::U, true);
        CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::I, true);
        bus.write(0x0100 + bus.cpu.sp as u16, bus.cpu.flags.bits);
        bus.cpu.sp -= 1;

        let interrupt_vector = 0xFFFAu16;
        let lo = bus.read(interrupt_vector, false) as u16;
        let hi = bus.read(interrupt_vector + 1, false) as u16;

        bus.cpu.pc = (hi << 8) | lo;

        bus.cpu.rem_cycles = 8;
    }

    pub fn fetch(bus: &mut Bus, addressing_mode: AddressingMode) -> u8 {
        //let addressing_mode = instruction.addressing;
        let what_to_fetch = addressing_mode(bus);
        match what_to_fetch {
            AddressingResult::Implicit { data } => {
                return data;
            }
            _ => {
                panic!("lol")
            }
        }
    }

    pub fn address(bus: &mut Bus, addressing_modes: AddressingMode) -> u16 {
        let where_to_fetch = addressing_mode(bus);
        match where_to_fetch {
            AddressingResult::Implicit { data } => {
                panic!("i dunno lol")
            },
            AddressingResult::ReadFrom { address, cycles } => {
                return address;
            },
            AddressingResult::Relative { address_rel } => {
                let t = bus.cpu.pc.wrapping_add(address_rel);
                return t;
            }
        }
    }
}

impl CpuStateFlags {
    /*fn set(self, bits: u8, value: bool) -> u8 {
        let stripped = bits & !self.bits;
        if value {
            return stripped | self.bits;
        } else {
            return stripped;
        }
    }

    fn get(self, bits: u8) -> bool {
        return bits & self.bits != 0;
    }*/
}