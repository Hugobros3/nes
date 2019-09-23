use crate::bus::Bus;
use crate::cpu::instructions::*;
use crate::cpu::addressing_modes::{AddressingResult, AddressingModeImplementation, AddressingMode};
use std::borrow::BorrowMut;

mod addressing_modes;
mod instructions;

/// NES 6502 CPU
/// No BCD support
#[derive(Debug)]
pub struct Cpu {
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
    }
}

impl Cpu {
    pub(crate) fn new() -> Self {
        return Cpu {
            flags: CpuStateFlags::empty(),
            a: 0x00u8,
            x: 0x00u8,
            y: 0x00u8,
            sp: 0x00u8,
            pc: 0x0000u16,

            rem_cycles: 0,
        };
    }

    pub fn clock(bus: &Bus) {
        let cpu: &mut Cpu = &mut bus.cpu.borrow_mut();
        if (cpu.rem_cycles == 0) {
            let opcode = bus.read(cpu.pc, false);
            cpu.pc += 1;

            let instruction = &INSTRUCTIONS[opcode as usize];
            cpu.rem_cycles = instruction.cycles;

            let instruction_implementation = instruction.implementation;

            println!("Executing {} (code={})", instruction.name, opcode);
            instruction_implementation(cpu, bus, instruction);

            println!("{:?}", cpu)
        }

        cpu.rem_cycles -= 1;
    }

    pub fn reset(bus: &Bus) {
        let cpu: &mut Cpu = &mut bus.cpu.borrow_mut();
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;

        cpu.sp = 0xFD;
        cpu.flags = CpuStateFlags::U;

        let reset_vector = 0xFFFCu16;
        let lo = bus.read(reset_vector, false) as u16;
        let hi = bus.read(reset_vector + 1, false) as u16;

        cpu.pc = (hi << 8) | lo;
        cpu.rem_cycles = 8;
    }

    fn irq(bus: &Bus) {
        let cpu: &mut Cpu = &mut bus.cpu.borrow_mut();

        if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::I) {
            bus.write(0x0100 + cpu.sp as u16, (cpu.pc >> 8) as u8);
            cpu.sp -= 1;
            bus.write(0x0100 + cpu.sp as u16, (cpu.pc & 0x00FFu16) as u8);
            cpu.sp -= 1;

            CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::B, false);
            CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::U, true);
            CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::I, true);
            bus.write(0x0100 + cpu.sp as u16, cpu.flags.bits);
            cpu.sp -= 1;

            let interrupt_vector = 0xFFFEu16;
            let lo = bus.read(interrupt_vector, false) as u16;
            let hi = bus.read(interrupt_vector + 1, false) as u16;

            cpu.pc = (hi << 8) | lo;

            cpu.rem_cycles = 7;
        }
    }

    fn nmi(bus: &mut Bus) {
        let cpu: &mut Cpu = &mut bus.cpu.borrow_mut();
        bus.write(0x0100 + cpu.sp as u16, (cpu.pc >> 8) as u8);
        cpu.sp -= 1;
        bus.write(0x0100 + cpu.sp as u16, (cpu.pc & 0x00FFu16) as u8);
        cpu.sp -= 1;

        CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::B, false);
        CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::U, true);
        CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::I, true);
        bus.write(0x0100 + cpu.sp as u16, cpu.flags.bits);
        cpu.sp -= 1;

        let interrupt_vector = 0xFFFAu16;
        let lo = bus.read(interrupt_vector, false) as u16;
        let hi = bus.read(interrupt_vector + 1, false) as u16;

        cpu.pc = (hi << 8) | lo;

        cpu.rem_cycles = 8;
    }
}