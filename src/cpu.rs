use crate::bus::Bus;
use crate::cpu::instructions::*;
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
    stack_pointer: u8,
    pub(crate) pc: u16,

    rem_cycles: i8,
}

bf!(CpuStateFlags[u8] {
    C: 0:0,
    Z: 1:1,
    I: 2:2,
    D: 3:3,
    B: 4:4,
    U: 5:5,
    V: 6:6,
    N: 7:7
});

impl Cpu {
    pub(crate) fn new() -> Self {
        return Cpu {
            flags: CpuStateFlags::new(0),
            a: 0x00u8,
            x: 0x00u8,
            y: 0x00u8,
            stack_pointer: 0x00u8,
            pc: 0x0000u16,

            rem_cycles: 0,
        };
    }

    pub fn clock(&mut self, bus: &Bus) {
        if self.rem_cycles == 0 {
            // Fetch instruction
            let fetching_from = self.pc;
            let opcode = bus.cpu_read(self.pc, false);
            self.flags.set_U(1);
            self.pc += 1;
            let instruction = &INSTRUCTIONS[opcode as usize];

            // Bump clock cycles
            self.rem_cycles = instruction.cycles;

            // Fetch memory according to addressing mode
            let addressing_mode_implementation = instruction.addressing.implementation;
            let addressing_result = addressing_mode_implementation(self, bus);

            let hex_pc = hex::encode(fetching_from.to_be_bytes());

            // Execute actual instruction
            let instruction_implementation = instruction.implementation;
            instruction_implementation(self, bus, instruction, &addressing_result);

            self.flags.set_U(1);
        }

        self.rem_cycles -= 1;
    }

    pub fn reset(&mut self, bus: &Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.stack_pointer = 0xFD;
        self.flags = CpuStateFlags::new(0);
        self.flags.set_U(1);// = CpuStateFlags::U;

        let reset_vector = 0xFFFCu16;
        let lo = bus.cpu_read(reset_vector, false) as u16;
        let hi = bus.cpu_read(reset_vector + 1, false) as u16;

        self.pc = (hi << 8) | lo;
        self.rem_cycles = 8;
    }

    pub fn irq(&mut self, bus: &Bus) {
        if self.flags.I() == 0 {
            bus.cpu_write(0x0100 + self.stack_pointer as u16, (self.pc >> 8) as u8);
            self.stack_pointer = self.stack_pointer.wrapping_sub(1);
            bus.cpu_write(0x0100 + self.stack_pointer as u16, (self.pc & 0x00FFu16) as u8);
            self.stack_pointer = self.stack_pointer.wrapping_sub(1);

            self.flags.set_B(0);
            self.flags.set_U(1);
            self.flags.set_I(1);
            bus.cpu_write(0x0100 + self.stack_pointer as u16, self.flags.val);
            self.stack_pointer = self.stack_pointer.wrapping_sub(1);

            let interrupt_vector = 0xFFFEu16;
            let lo = bus.cpu_read(interrupt_vector, false) as u16;
            let hi = bus.cpu_read(interrupt_vector + 1, false) as u16;

            self.pc = (hi << 8) | lo;

            self.rem_cycles = 7;
        }
    }

    pub fn nmi(&mut self, bus: &Bus) {
        //let cpu: &mut Cpu = &mut bus.cpu.borrow_mut();
        bus.cpu_write(0x0100 + self.stack_pointer as u16, (self.pc >> 8) as u8);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        bus.cpu_write(0x0100 + self.stack_pointer as u16, (self.pc & 0x00FFu16) as u8);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        self.flags.set_B(0);
        self.flags.set_U(1);
        self.flags.set_I(1);
        bus.cpu_write(0x0100 + self.stack_pointer as u16, self.flags.val);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        let interrupt_vector = 0xFFFAu16;
        let lo = bus.cpu_read(interrupt_vector, false) as u16;
        let hi = bus.cpu_read(interrupt_vector + 1, false) as u16;

        self.pc = (hi << 8) | lo;

        self.rem_cycles = 8;
    }
}