use crate::cpu::addressing_modes::*;
use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::CpuStateFlags;

pub struct Instruction {
    pub name: &'static str,
    pub implementation: InstructionImplementation,
    pub addressing: &'static AddressingMode,
    pub(crate) cycles: i8,
}

type InstructionImplementation = fn(&mut Cpu, &Bus, &Instruction) -> i8;

pub const INSTRUCTIONS: [Instruction; 256] = [
    Instruction { name: "BRK", implementation: BRK, addressing: IMM, cycles: 7 },
    Instruction { name: "ORA", implementation: ORA, addressing: IZX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 3 },
    Instruction { name: "ORA", implementation: ORA, addressing: ZP0, cycles: 3 },
    Instruction { name: "ASL", implementation: ASL, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "PHP", implementation: PHP, addressing: IMP, cycles: 3 },
    Instruction { name: "ORA", implementation: ORA, addressing: IMM, cycles: 2 },
    Instruction { name: "ASL", implementation: ASL, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ORA", implementation: ORA, addressing: ABS, cycles: 4 },
    Instruction { name: "ASL", implementation: ASL, addressing: ABS, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BPL", implementation: BPL, addressing: REL, cycles: 2 },
    Instruction { name: "ORA", implementation: ORA, addressing: IZY, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ORA", implementation: ORA, addressing: ZPX, cycles: 4 },
    Instruction { name: "ASL", implementation: ASL, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "CLC", implementation: CLC, addressing: IMP, cycles: 2 },
    Instruction { name: "ORA", implementation: ORA, addressing: ABY, cycles: 4 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ORA", implementation: ORA, addressing: ABX, cycles: 4 },
    Instruction { name: "ASL", implementation: ASL, addressing: ABX, cycles: 7 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "JSR", implementation: JSR, addressing: ABS, cycles: 6 },
    Instruction { name: "AND", implementation: AND, addressing: IZX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "BIT", implementation: BIT, addressing: ZP0, cycles: 3 },
    Instruction { name: "AND", implementation: AND, addressing: ZP0, cycles: 3 },
    Instruction { name: "ROL", implementation: ROL, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "PLP", implementation: PLP, addressing: IMP, cycles: 4 },
    Instruction { name: "AND", implementation: AND, addressing: IMM, cycles: 2 },
    Instruction { name: "ROL", implementation: ROL, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "BIT", implementation: BIT, addressing: ABS, cycles: 4 },
    Instruction { name: "AND", implementation: AND, addressing: ABS, cycles: 4 },
    Instruction { name: "ROL", implementation: ROL, addressing: ABS, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BMI", implementation: BMI, addressing: REL, cycles: 2 },
    Instruction { name: "AND", implementation: AND, addressing: IZY, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "AND", implementation: AND, addressing: ZPX, cycles: 4 },
    Instruction { name: "ROL", implementation: ROL, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "SEC", implementation: SEC, addressing: IMP, cycles: 2 },
    Instruction { name: "AND", implementation: AND, addressing: ABY, cycles: 4 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "AND", implementation: AND, addressing: ABX, cycles: 4 },
    Instruction { name: "ROL", implementation: ROL, addressing: ABX, cycles: 7 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "RTI", implementation: RTI, addressing: IMP, cycles: 6 },
    Instruction { name: "EOR", implementation: EOR, addressing: IZX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 3 },
    Instruction { name: "EOR", implementation: EOR, addressing: ZP0, cycles: 3 },
    Instruction { name: "LSR", implementation: LSR, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "PHA", implementation: PHA, addressing: IMP, cycles: 3 },
    Instruction { name: "EOR", implementation: EOR, addressing: IMM, cycles: 2 },
    Instruction { name: "LSR", implementation: LSR, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "JMP", implementation: JMP, addressing: ABS, cycles: 3 },
    Instruction { name: "EOR", implementation: EOR, addressing: ABS, cycles: 4 },
    Instruction { name: "LSR", implementation: LSR, addressing: ABS, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BVC", implementation: BVC, addressing: REL, cycles: 2 },
    Instruction { name: "EOR", implementation: EOR, addressing: IZY, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "EOR", implementation: EOR, addressing: ZPX, cycles: 4 },
    Instruction { name: "LSR", implementation: LSR, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "CLI", implementation: CLI, addressing: IMP, cycles: 2 },
    Instruction { name: "EOR", implementation: EOR, addressing: ABY, cycles: 4 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "EOR", implementation: EOR, addressing: ABX, cycles: 4 },
    Instruction { name: "LSR", implementation: LSR, addressing: ABX, cycles: 7 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "RTS", implementation: RTS, addressing: IMP, cycles: 6 },
    Instruction { name: "ADC", implementation: ADC, addressing: IZX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 3 },
    Instruction { name: "ADC", implementation: ADC, addressing: ZP0, cycles: 3 },
    Instruction { name: "ROR", implementation: ROR, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "PLA", implementation: PLA, addressing: IMP, cycles: 4 },
    Instruction { name: "ADC", implementation: ADC, addressing: IMM, cycles: 2 },
    Instruction { name: "ROR", implementation: ROR, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "JMP", implementation: JMP, addressing: IND, cycles: 5 },
    Instruction { name: "ADC", implementation: ADC, addressing: ABS, cycles: 4 },
    Instruction { name: "ROR", implementation: ROR, addressing: ABS, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BVS", implementation: BVS, addressing: REL, cycles: 2 },
    Instruction { name: "ADC", implementation: ADC, addressing: IZY, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ADC", implementation: ADC, addressing: ZPX, cycles: 4 },
    Instruction { name: "ROR", implementation: ROR, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "SEI", implementation: SEI, addressing: IMP, cycles: 2 },
    Instruction { name: "ADC", implementation: ADC, addressing: ABY, cycles: 4 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ADC", implementation: ADC, addressing: ABX, cycles: 4 },
    Instruction { name: "ROR", implementation: ROR, addressing: ABX, cycles: 7 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "STA", implementation: STA, addressing: IZX, cycles: 6 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "STY", implementation: STY, addressing: ZP0, cycles: 3 },
    Instruction { name: "STA", implementation: STA, addressing: ZP0, cycles: 3 },
    Instruction { name: "STX", implementation: STX, addressing: ZP0, cycles: 3 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 3 },
    Instruction { name: "DEY", implementation: DEY, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "TXA", implementation: TXA, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "STY", implementation: STY, addressing: ABS, cycles: 4 },
    Instruction { name: "STA", implementation: STA, addressing: ABS, cycles: 4 },
    Instruction { name: "STX", implementation: STX, addressing: ABS, cycles: 4 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "BCC", implementation: BCC, addressing: REL, cycles: 2 },
    Instruction { name: "STA", implementation: STA, addressing: IZY, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "STY", implementation: STY, addressing: ZPX, cycles: 4 },
    Instruction { name: "STA", implementation: STA, addressing: ZPX, cycles: 4 },
    Instruction { name: "STX", implementation: STX, addressing: ZPY, cycles: 4 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "TYA", implementation: TYA, addressing: IMP, cycles: 2 },
    Instruction { name: "STA", implementation: STA, addressing: ABY, cycles: 5 },
    Instruction { name: "TXS", implementation: TXS, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 5 },
    Instruction { name: "STA", implementation: STA, addressing: ABX, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "LDY", implementation: LDY, addressing: IMM, cycles: 2 },
    Instruction { name: "LDA", implementation: LDA, addressing: IZX, cycles: 6 },
    Instruction { name: "LDX", implementation: LDX, addressing: IMM, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "LDY", implementation: LDY, addressing: ZP0, cycles: 3 },
    Instruction { name: "LDA", implementation: LDA, addressing: ZP0, cycles: 3 },
    Instruction { name: "LDX", implementation: LDX, addressing: ZP0, cycles: 3 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 3 },
    Instruction { name: "TAY", implementation: TAY, addressing: IMP, cycles: 2 },
    Instruction { name: "LDA", implementation: LDA, addressing: IMM, cycles: 2 },
    Instruction { name: "TAX", implementation: TAX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "LDY", implementation: LDY, addressing: ABS, cycles: 4 },
    Instruction { name: "LDA", implementation: LDA, addressing: ABS, cycles: 4 },
    Instruction { name: "LDX", implementation: LDX, addressing: ABS, cycles: 4 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "BCS", implementation: BCS, addressing: REL, cycles: 2 },
    Instruction { name: "LDA", implementation: LDA, addressing: IZY, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "LDY", implementation: LDY, addressing: ZPX, cycles: 4 },
    Instruction { name: "LDA", implementation: LDA, addressing: ZPX, cycles: 4 },
    Instruction { name: "LDX", implementation: LDX, addressing: ZPY, cycles: 4 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "CLV", implementation: CLV, addressing: IMP, cycles: 2 },
    Instruction { name: "LDA", implementation: LDA, addressing: ABY, cycles: 4 },
    Instruction { name: "TSX", implementation: TSX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "LDY", implementation: LDY, addressing: ABX, cycles: 4 },
    Instruction { name: "LDA", implementation: LDA, addressing: ABX, cycles: 4 },
    Instruction { name: "LDX", implementation: LDX, addressing: ABY, cycles: 4 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "CPY", implementation: CPY, addressing: IMM, cycles: 2 },
    Instruction { name: "CMP", implementation: CMP, addressing: IZX, cycles: 6 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "CPY", implementation: CPY, addressing: ZP0, cycles: 3 },
    Instruction { name: "CMP", implementation: CMP, addressing: ZP0, cycles: 3 },
    Instruction { name: "DEC", implementation: DEC, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "INY", implementation: INY, addressing: IMP, cycles: 2 },
    Instruction { name: "CMP", implementation: CMP, addressing: IMM, cycles: 2 },
    Instruction { name: "DEX", implementation: DEX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "CPY", implementation: CPY, addressing: ABS, cycles: 4 },
    Instruction { name: "CMP", implementation: CMP, addressing: ABS, cycles: 4 },
    Instruction { name: "DEC", implementation: DEC, addressing: ABS, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BNE", implementation: BNE, addressing: REL, cycles: 2 },
    Instruction { name: "CMP", implementation: CMP, addressing: IZY, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "CMP", implementation: CMP, addressing: ZPX, cycles: 4 },
    Instruction { name: "DEC", implementation: DEC, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "CLD", implementation: CLD, addressing: IMP, cycles: 2 },
    Instruction { name: "CMP", implementation: CMP, addressing: ABY, cycles: 4 },
    Instruction { name: "NOP", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "CMP", implementation: CMP, addressing: ABX, cycles: 4 },
    Instruction { name: "DEC", implementation: DEC, addressing: ABX, cycles: 7 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "CPX", implementation: CPX, addressing: IMM, cycles: 2 },
    Instruction { name: "SBC", implementation: SBC, addressing: IZX, cycles: 6 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "CPX", implementation: CPX, addressing: ZP0, cycles: 3 },
    Instruction { name: "SBC", implementation: SBC, addressing: ZP0, cycles: 3 },
    Instruction { name: "INC", implementation: INC, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "INX", implementation: INX, addressing: IMP, cycles: 2 },
    Instruction { name: "SBC", implementation: SBC, addressing: IMM, cycles: 2 },
    Instruction { name: "NOP", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: SBC, addressing: IMP, cycles: 2 },
    Instruction { name: "CPX", implementation: CPX, addressing: ABS, cycles: 4 },
    Instruction { name: "SBC", implementation: SBC, addressing: ABS, cycles: 4 },
    Instruction { name: "INC", implementation: INC, addressing: ABS, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BEQ", implementation: BEQ, addressing: REL, cycles: 2 },
    Instruction { name: "SBC", implementation: SBC, addressing: IZY, cycles: 5 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "SBC", implementation: SBC, addressing: ZPX, cycles: 4 },
    Instruction { name: "INC", implementation: INC, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "SED", implementation: SED, addressing: IMP, cycles: 2 },
    Instruction { name: "SBC", implementation: SBC, addressing: ABY, cycles: 4 },
    Instruction { name: "NOP", implementation: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", implementation: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "SBC", implementation: SBC, addressing: ABX, cycles: 4 },
    Instruction { name: "INC", implementation: INC, addressing: ABX, cycles: 7 },
    Instruction { name: "???", implementation: XXX, addressing: IMP, cycles: 7 },
];

fn XXX(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    panic!("Illegal instruction")
}

fn JMP(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.pc = instruction.addressing.address_rel(cpu, bus);
    return 0;
}

// Branch on carry set
fn BCS(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) {
        cpu.rem_cycles += 1;

        let offset = instruction.addressing.offset_rel(cpu, bus);

        let address_abs = cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (cpu.pc & 0xFF00u16) {
            cpu.rem_cycles += 1;
        }

        cpu.pc = address_abs;
    }
    return 0;
}

// Branch on carry clear
fn BCC(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) {
        cpu.rem_cycles += 1;

        let offset = instruction.addressing.offset_rel(cpu, bus);

        let address_abs = cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (cpu.pc & 0xFF00u16) {
            cpu.rem_cycles += 1;
        }

        cpu.pc = address_abs;
    }
    return 0;
}

// Branch if equal
fn BEQ(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::Z) {
        cpu.rem_cycles += 1;

        let offset = instruction.addressing.offset_rel(cpu, bus);

        let address_abs = cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (cpu.pc & 0xFF00u16) {
            cpu.rem_cycles += 1;
        }

        cpu.pc = address_abs;
    }
    return 0;
}

// Branch if not equal
fn BNE(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::Z) {
        cpu.rem_cycles += 1;

        let offset = instruction.addressing.offset_rel(cpu, bus);

        let address_abs = cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (cpu.pc & 0xFF00u16) {
            cpu.rem_cycles += 1;
        }

        cpu.pc = address_abs;
    }
    return 0;
}

// Branch if negative (N set)
fn BMI(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::N) {
        cpu.rem_cycles += 1;

        let offset = instruction.addressing.offset_rel(cpu, bus);

        let address_abs = cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (cpu.pc & 0xFF00u16) {
            cpu.rem_cycles += 1;
        }

        cpu.pc = address_abs;
    }
    return 0;
}

// Branch if positive (N not set)
fn BPL(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::N) {
        cpu.rem_cycles += 1;

        let offset = instruction.addressing.offset_rel(cpu, bus);

        let address_abs = cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (cpu.pc & 0xFF00u16) {
            cpu.rem_cycles += 1;
        }

        cpu.pc = address_abs;
    }
    return 0;
}

// Branch if overflow clear
fn BVC(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::V) {
        cpu.rem_cycles += 1;

        let offset = instruction.addressing.offset_rel(cpu, bus);

        let address_abs = cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (cpu.pc & 0xFF00u16) {
            cpu.rem_cycles += 1;
        }

        cpu.pc = address_abs;
    }
    return 0;
}

// Branch if overflow set
fn BVS(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::V) {
        cpu.rem_cycles += 1;

        let offset = instruction.addressing.offset_rel(cpu, bus);

        let address_abs = cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (cpu.pc & 0xFF00u16) {
            cpu.rem_cycles += 1;
        }

        cpu.pc = address_abs;
    }
    return 0;
}

// Clear carry bit
fn CLC(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, false);
    return 0;
}

// Clear decimal flag (but we don't use it ???)
fn CLD(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::D, false);
    return 0;
}

// Disable interrupts
fn CLI(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::I, false);
    return 0;
}

// Clear overflow
fn CLV(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::V, false);
    return 0;
}

// Set carry flag
fn SEC(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, true);
    return 0;
}

// Set decimal flag
fn SED(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::D, true);
    return 0;
}

// Set interrupt flag
fn SEI(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::I, true);
    return 0;
}

// Add with carry
fn ADC(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus) as u16;
    let carry_in = if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) { 1u16 } else { 0u16 };

    let temp = (cpu.a as u16) + (fetched) + carry_in;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, temp > 255);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);

    let v = (!((cpu.a as u16) ^ (fetched)) & ((cpu.a as u16) ^ (temp)) & 0x0080u16) != 0;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::V, v);

    cpu.a = (temp & 0x00ffu16) as u8;

    return 1;
}

// Substract with borrow in
fn SBC(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus) as u16 ^ 0x00FFu16;
    let borrow_in = if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) { 1u16 } else { 0u16 };

    let temp = (cpu.a as u16) + (fetched) + borrow_in;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, temp > 255);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);

    let v = (!((cpu.a as u16) ^ (fetched)) & ((cpu.a as u16) ^ (temp)) & 0x0080u16) != 0;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::V, v);

    cpu.a = (temp & 0x00ffu16) as u8;

    return 1;
}

// Push A to stack
fn PHA(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    bus.write(0x0100u16 + cpu.sp as u16, cpu.a);
    cpu.sp -= 1;
    return 0;
}

// Pop A from stack
fn PLA(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.sp += 1;
    cpu.a = bus.read(0x0100u16 + cpu.sp as u16, false);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);

    return 0;
}

// Push status register to stack
fn PHP(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    bus.write(0x0100u16 + cpu.sp as u16, cpu.flags.bits | CpuStateFlags::B.bits | CpuStateFlags::U.bits);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::B, false);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::U, false);
    cpu.sp -= 1;
    return 0;
}

// Pop status register from stack
fn PLP(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.sp += 1;
    cpu.flags.bits = bus.read(0x0100u16 + cpu.sp as u16, false);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::U, true);

    return 0;
}

// Break (manual interrupt)
fn BRK(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.pc += 1;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::I, true);
    // Push PC
    bus.write(0x0100 + cpu.sp as u16, (cpu.pc >> 8) as u8);
    cpu.sp -= 1;
    bus.write(0x0100 + cpu.sp as u16, (cpu.pc & 0x00FFu16) as u8);
    cpu.sp -= 1;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::B, true);
    // Push SP
    bus.write(0x0100 + cpu.sp as u16, cpu.flags.bits);
    cpu.sp -= 1;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::B, false);

    // Read PC from interrupt vector
    let interrupt_vector = 0xFFFEu16;
    let lo = bus.read(interrupt_vector, false) as u16;
    let hi = bus.read(interrupt_vector + 1, false) as u16;

    cpu.pc = (hi << 8) | lo;
    return 0;
}

// Return from interrupt
fn RTI(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.sp += 1;
    cpu.flags.bits = bus.read(0x0100u16 + cpu.sp as u16, false);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::B, false);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::U, false);

    cpu.sp += 1;
    let pc_lo = bus.read(0x0100u16 + cpu.sp as u16, false) as u16;
    cpu.sp += 1;
    let pc_hi = bus.read(0x0100u16 + cpu.sp as u16, false) as u16;

    cpu.pc = (pc_hi << 8) | pc_lo;
    return 0;
}

fn JSR(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.sp = cpu.sp.wrapping_sub(1);

    // Push PC
    bus.write(0x0100 + cpu.sp as u16, (cpu.pc >> 8) as u8);
    cpu.sp -= 1;
    bus.write(0x0100 + cpu.sp as u16, (cpu.pc & 0x00FFu16) as u8);
    cpu.sp -= 1;

    cpu.pc = instruction.addressing.address_rel(cpu, bus);
    return 0;
}

// Return from subroutine
fn RTS(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.sp += 1;
    let pc_lo = bus.read(0x0100u16 + cpu.sp as u16, false) as u16;
    cpu.sp += 1;
    let pc_hi = bus.read(0x0100u16 + cpu.sp as u16, false) as u16;

    cpu.pc = (pc_hi << 8) | pc_lo;
    cpu.pc += 1;
    return 0;
}

// Stores A
fn STA(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    bus.write(instruction.addressing.address(cpu, bus), cpu.a);
    return 0;
}

// Stores X
fn STX(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    bus.write(instruction.addressing.address(cpu, bus), cpu.x);
    return 0;
}

// Stores Y
fn STY(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    bus.write(instruction.addressing.address(cpu, bus), cpu.y);
    return 0;
}

// Xfer A to X
fn TAX(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.x = cpu.a;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.x == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.x & 0x80u8 != 0);
    return 0;
}

// Xfer A to Y
fn TAY(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.y = cpu.a;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.y == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.y & 0x80u8 != 0);
    return 0;
}

// Xfer SP TO X
fn TSX(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.x = cpu.sp;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.x == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.x & 0x80u8 != 0);
    return 0;
}

// Xfer X to A
fn TXA(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.a = cpu.x;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 0;
}

// Xfer X to SP
fn TXS(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.sp = cpu.x;
    return 0;
}

// Xfer Y to A
fn TYA(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.a = cpu.y;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 0;
}

fn LDA(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    cpu.a = fetched;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 0;
}

fn LDX(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    cpu.x = fetched;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.x == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.x & 0x80u8 != 0);
    return 0;
}

fn LDY(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    cpu.y = fetched;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.y == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.y & 0x80u8 != 0);
    return 0;
}

// No op
fn NOP(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    return 1;
}

// Bitwise And
fn AND(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    cpu.a = cpu.a & fetched;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 1;
}

// Bitwise Or
fn ORA(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    cpu.a = cpu.a | fetched;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 1;
}

// Bitwise Xor
fn EOR(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    cpu.a = cpu.a ^ fetched;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 1;
}

// Shift left
fn ROL(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus) as u16;
    let carry_in = if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) { 1u16 } else { 0u16 };
    let temp = fetched << 1 | carry_in;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, (temp & 0xFF00u16) != 0);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);

    if instruction.addressing == IMM {
        cpu.a = (temp & 0x0FF) as u8;
    } else {
        bus.write(instruction.addressing.address(cpu, bus), (temp & 0x00FF) as u8);
    }

    return 0;
}

// Shift right (beamng is better tbh)
fn ROR(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus) as u16;
    let carry_in = if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) { 1u16 } else { 0u16 };
    let temp = carry_in << 7 | fetched >> 1;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, (temp & 0xFF00u16) != 0);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);

    if instruction.addressing == IMM {
        cpu.a = (temp & 0x0FF) as u8;
    } else {
        bus.write(instruction.addressing.address(cpu, bus), (temp & 0x00FF) as u8);
    }

    return 0;
}

// Logical Shift left
fn ASL(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus) as u16;
    let temp = fetched << 1;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, (temp & 0xFF00u16) != 0);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);

    if instruction.addressing == IMM {
        cpu.a = (temp & 0x0FF) as u8;
    } else {
        bus.write(instruction.addressing.address(cpu, bus), (temp & 0x00FF) as u8);
    }

    return 0;
}

// Logical Shift Right
fn LSR(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, (fetched & 0x01) == 0x01);
    let temp = fetched >> 1;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (fetched & 0x00FF) == 0x0000);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (fetched & 0x0080) != 0x0000);

    if instruction.addressing == IMM {
        cpu.a = (temp & 0x0FF) as u8;
    } else {
        bus.write(instruction.addressing.address(cpu, bus), (temp & 0x00FF) as u8);
    }

    return 0;
}

// Bit testing (does the mask match anything ?)
fn BIT(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    let temp = cpu.a & fetched;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & (1 << 7)) != 0);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::V, (fetched & (1 << 6)) != 0);
    return 0;
}

// Compare A with ...
fn CMP(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    let temp = (cpu.a as u16).wrapping_sub(fetched as u16);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, cpu.a as u16 >= fetched as u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);
    return 1;
}

// Compare X with ...
fn CPX(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    let temp = (cpu.x as u16).wrapping_sub(fetched as u16);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, cpu.x as u16 >= fetched as u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);
    return 1;
}

// Compare Y with ...
fn CPY(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    let temp = (cpu.y as u16).wrapping_sub(fetched as u16);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, cpu.y as u16 >= fetched as u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);
    return 1;
}

// Decrement memory location
fn DEC(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    let temp = fetched.wrapping_sub(1u8);
    bus.write(instruction.addressing.address(cpu, bus), temp);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.x & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.x & 0x80u8) != 0);
    return 0;
}

// Decrement X
fn DEX(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.x = cpu.x.wrapping_sub(1);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.x & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.x & 0x80u8) != 0);
    return 0;
}

// Decrement Y
fn DEY(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.y = cpu.y.wrapping_sub(1);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.y & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.y & 0x80u8) != 0);
    return 0;
}

// Increment memory location
fn INC(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    let fetched = instruction.addressing.fetch(cpu, bus);
    let temp = fetched.wrapping_add(1u8);
    bus.write(instruction.addressing.address(cpu, bus), temp);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.x & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.x & 0x80u8) != 0);
    return 0;
}

// Increment X
fn INX(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.x = cpu.x.wrapping_add(1);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.x & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.x & 0x80u8) != 0);
    return 0;
}

// Increment Y
fn INY(cpu: &mut Cpu, bus: &Bus, instruction: &Instruction) -> i8 {
    cpu.y = cpu.y.wrapping_add(1);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.y & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.y & 0x80u8) != 0);
    return 0;
}