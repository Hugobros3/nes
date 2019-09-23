use crate::cpu::addressing_modes::*;
use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::CpuStateFlags;

pub const INSTRUCTIONS: [Instruction; 256] = [
    Instruction { name: "BRK", work: BRK, addressing: IMM, cycles: 7 },
    Instruction { name: "ORA", work: ORA, addressing: IZX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 3 },
    Instruction { name: "ORA", work: ORA, addressing: ZP0, cycles: 3 },
    Instruction { name: "ASL", work: ASL, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "PHP", work: PHP, addressing: IMP, cycles: 3 },
    Instruction { name: "ORA", work: ORA, addressing: IMM, cycles: 2 },
    Instruction { name: "ASL", work: ASL, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ORA", work: ORA, addressing: ABS, cycles: 4 },
    Instruction { name: "ASL", work: ASL, addressing: ABS, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BPL", work: BPL, addressing: REL, cycles: 2 },
    Instruction { name: "ORA", work: ORA, addressing: IZY, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ORA", work: ORA, addressing: ZPX, cycles: 4 },
    Instruction { name: "ASL", work: ASL, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "CLC", work: CLC, addressing: IMP, cycles: 2 },
    Instruction { name: "ORA", work: ORA, addressing: ABY, cycles: 4 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ORA", work: ORA, addressing: ABX, cycles: 4 },
    Instruction { name: "ASL", work: ASL, addressing: ABX, cycles: 7 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "JSR", work: JSR, addressing: ABS, cycles: 6 },
    Instruction { name: "AND", work: AND, addressing: IZX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "BIT", work: BIT, addressing: ZP0, cycles: 3 },
    Instruction { name: "AND", work: AND, addressing: ZP0, cycles: 3 },
    Instruction { name: "ROL", work: ROL, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "PLP", work: PLP, addressing: IMP, cycles: 4 },
    Instruction { name: "AND", work: AND, addressing: IMM, cycles: 2 },
    Instruction { name: "ROL", work: ROL, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "BIT", work: BIT, addressing: ABS, cycles: 4 },
    Instruction { name: "AND", work: AND, addressing: ABS, cycles: 4 },
    Instruction { name: "ROL", work: ROL, addressing: ABS, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BMI", work: BMI, addressing: REL, cycles: 2 },
    Instruction { name: "AND", work: AND, addressing: IZY, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "AND", work: AND, addressing: ZPX, cycles: 4 },
    Instruction { name: "ROL", work: ROL, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "SEC", work: SEC, addressing: IMP, cycles: 2 },
    Instruction { name: "AND", work: AND, addressing: ABY, cycles: 4 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "AND", work: AND, addressing: ABX, cycles: 4 },
    Instruction { name: "ROL", work: ROL, addressing: ABX, cycles: 7 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "RTI", work: RTI, addressing: IMP, cycles: 6 },
    Instruction { name: "EOR", work: EOR, addressing: IZX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 3 },
    Instruction { name: "EOR", work: EOR, addressing: ZP0, cycles: 3 },
    Instruction { name: "LSR", work: LSR, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "PHA", work: PHA, addressing: IMP, cycles: 3 },
    Instruction { name: "EOR", work: EOR, addressing: IMM, cycles: 2 },
    Instruction { name: "LSR", work: LSR, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "JMP", work: JMP, addressing: ABS, cycles: 3 },
    Instruction { name: "EOR", work: EOR, addressing: ABS, cycles: 4 },
    Instruction { name: "LSR", work: LSR, addressing: ABS, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BVC", work: BVC, addressing: REL, cycles: 2 },
    Instruction { name: "EOR", work: EOR, addressing: IZY, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "EOR", work: EOR, addressing: ZPX, cycles: 4 },
    Instruction { name: "LSR", work: LSR, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "CLI", work: CLI, addressing: IMP, cycles: 2 },
    Instruction { name: "EOR", work: EOR, addressing: ABY, cycles: 4 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "EOR", work: EOR, addressing: ABX, cycles: 4 },
    Instruction { name: "LSR", work: LSR, addressing: ABX, cycles: 7 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "RTS", work: RTS, addressing: IMP, cycles: 6 },
    Instruction { name: "ADC", work: ADC, addressing: IZX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 3 },
    Instruction { name: "ADC", work: ADC, addressing: ZP0, cycles: 3 },
    Instruction { name: "ROR", work: ROR, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "PLA", work: PLA, addressing: IMP, cycles: 4 },
    Instruction { name: "ADC", work: ADC, addressing: IMM, cycles: 2 },
    Instruction { name: "ROR", work: ROR, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "JMP", work: JMP, addressing: IND, cycles: 5 },
    Instruction { name: "ADC", work: ADC, addressing: ABS, cycles: 4 },
    Instruction { name: "ROR", work: ROR, addressing: ABS, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BVS", work: BVS, addressing: REL, cycles: 2 },
    Instruction { name: "ADC", work: ADC, addressing: IZY, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ADC", work: ADC, addressing: ZPX, cycles: 4 },
    Instruction { name: "ROR", work: ROR, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "SEI", work: SEI, addressing: IMP, cycles: 2 },
    Instruction { name: "ADC", work: ADC, addressing: ABY, cycles: 4 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "ADC", work: ADC, addressing: ABX, cycles: 4 },
    Instruction { name: "ROR", work: ROR, addressing: ABX, cycles: 7 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "STA", work: STA, addressing: IZX, cycles: 6 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "STY", work: STY, addressing: ZP0, cycles: 3 },
    Instruction { name: "STA", work: STA, addressing: ZP0, cycles: 3 },
    Instruction { name: "STX", work: STX, addressing: ZP0, cycles: 3 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 3 },
    Instruction { name: "DEY", work: DEY, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "TXA", work: TXA, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "STY", work: STY, addressing: ABS, cycles: 4 },
    Instruction { name: "STA", work: STA, addressing: ABS, cycles: 4 },
    Instruction { name: "STX", work: STX, addressing: ABS, cycles: 4 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "BCC", work: BCC, addressing: REL, cycles: 2 },
    Instruction { name: "STA", work: STA, addressing: IZY, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "STY", work: STY, addressing: ZPX, cycles: 4 },
    Instruction { name: "STA", work: STA, addressing: ZPX, cycles: 4 },
    Instruction { name: "STX", work: STX, addressing: ZPY, cycles: 4 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "TYA", work: TYA, addressing: IMP, cycles: 2 },
    Instruction { name: "STA", work: STA, addressing: ABY, cycles: 5 },
    Instruction { name: "TXS", work: TXS, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 5 },
    Instruction { name: "STA", work: STA, addressing: ABX, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "LDY", work: LDY, addressing: IMM, cycles: 2 },
    Instruction { name: "LDA", work: LDA, addressing: IZX, cycles: 6 },
    Instruction { name: "LDX", work: LDX, addressing: IMM, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "LDY", work: LDY, addressing: ZP0, cycles: 3 },
    Instruction { name: "LDA", work: LDA, addressing: ZP0, cycles: 3 },
    Instruction { name: "LDX", work: LDX, addressing: ZP0, cycles: 3 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 3 },
    Instruction { name: "TAY", work: TAY, addressing: IMP, cycles: 2 },
    Instruction { name: "LDA", work: LDA, addressing: IMM, cycles: 2 },
    Instruction { name: "TAX", work: TAX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "LDY", work: LDY, addressing: ABS, cycles: 4 },
    Instruction { name: "LDA", work: LDA, addressing: ABS, cycles: 4 },
    Instruction { name: "LDX", work: LDX, addressing: ABS, cycles: 4 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "BCS", work: BCS, addressing: REL, cycles: 2 },
    Instruction { name: "LDA", work: LDA, addressing: IZY, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "LDY", work: LDY, addressing: ZPX, cycles: 4 },
    Instruction { name: "LDA", work: LDA, addressing: ZPX, cycles: 4 },
    Instruction { name: "LDX", work: LDX, addressing: ZPY, cycles: 4 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "CLV", work: CLV, addressing: IMP, cycles: 2 },
    Instruction { name: "LDA", work: LDA, addressing: ABY, cycles: 4 },
    Instruction { name: "TSX", work: TSX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "LDY", work: LDY, addressing: ABX, cycles: 4 },
    Instruction { name: "LDA", work: LDA, addressing: ABX, cycles: 4 },
    Instruction { name: "LDX", work: LDX, addressing: ABY, cycles: 4 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 4 },
    Instruction { name: "CPY", work: CPY, addressing: IMM, cycles: 2 },
    Instruction { name: "CMP", work: CMP, addressing: IZX, cycles: 6 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "CPY", work: CPY, addressing: ZP0, cycles: 3 },
    Instruction { name: "CMP", work: CMP, addressing: ZP0, cycles: 3 },
    Instruction { name: "DEC", work: DEC, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "INY", work: INY, addressing: IMP, cycles: 2 },
    Instruction { name: "CMP", work: CMP, addressing: IMM, cycles: 2 },
    Instruction { name: "DEX", work: DEX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "CPY", work: CPY, addressing: ABS, cycles: 4 },
    Instruction { name: "CMP", work: CMP, addressing: ABS, cycles: 4 },
    Instruction { name: "DEC", work: DEC, addressing: ABS, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BNE", work: BNE, addressing: REL, cycles: 2 },
    Instruction { name: "CMP", work: CMP, addressing: IZY, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "CMP", work: CMP, addressing: ZPX, cycles: 4 },
    Instruction { name: "DEC", work: DEC, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "CLD", work: CLD, addressing: IMP, cycles: 2 },
    Instruction { name: "CMP", work: CMP, addressing: ABY, cycles: 4 },
    Instruction { name: "NOP", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "CMP", work: CMP, addressing: ABX, cycles: 4 },
    Instruction { name: "DEC", work: DEC, addressing: ABX, cycles: 7 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "CPX", work: CPX, addressing: IMM, cycles: 2 },
    Instruction { name: "SBC", work: SBC, addressing: IZX, cycles: 6 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "CPX", work: CPX, addressing: ZP0, cycles: 3 },
    Instruction { name: "SBC", work: SBC, addressing: ZP0, cycles: 3 },
    Instruction { name: "INC", work: INC, addressing: ZP0, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 5 },
    Instruction { name: "INX", work: INX, addressing: IMP, cycles: 2 },
    Instruction { name: "SBC", work: SBC, addressing: IMM, cycles: 2 },
    Instruction { name: "NOP", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: SBC, addressing: IMP, cycles: 2 },
    Instruction { name: "CPX", work: CPX, addressing: ABS, cycles: 4 },
    Instruction { name: "SBC", work: SBC, addressing: ABS, cycles: 4 },
    Instruction { name: "INC", work: INC, addressing: ABS, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "BEQ", work: BEQ, addressing: REL, cycles: 2 },
    Instruction { name: "SBC", work: SBC, addressing: IZY, cycles: 5 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 8 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "SBC", work: SBC, addressing: ZPX, cycles: 4 },
    Instruction { name: "INC", work: INC, addressing: ZPX, cycles: 6 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 6 },
    Instruction { name: "SED", work: SED, addressing: IMP, cycles: 2 },
    Instruction { name: "SBC", work: SBC, addressing: ABY, cycles: 4 },
    Instruction { name: "NOP", work: NOP, addressing: IMP, cycles: 2 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
    Instruction { name: "???", work: NOP, addressing: IMP, cycles: 4 },
    Instruction { name: "SBC", work: SBC, addressing: ABX, cycles: 4 },
    Instruction { name: "INC", work: INC, addressing: ABX, cycles: 7 },
    Instruction { name: "???", work: XXX, addressing: IMP, cycles: 7 },
];

pub struct Instruction {
    pub name: &'static str,
    pub work: InstructionWork,
    pub addressing: AddressingMode,
    pub(crate) cycles: i8,
}

type InstructionWork = fn(&mut Cpu, &Bus, AddressingMode) -> i8;

fn XXX(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    panic!("Illegal instruction")
}

fn JMP(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.pc = Cpu::address_rel(cpu, bus, addressing_mode);
    return 0;
}

// Branch on carry set
fn BCS(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) {
        cpu.rem_cycles += 1;

        let offset = match addressing_mode(cpu, bus) {
            AddressingResult::ProgramCounterRelative { address_rel } => address_rel,
            _ => panic!("No")
        };

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
fn BCC(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) {
        cpu.rem_cycles += 1;

        let offset = match addressing_mode(cpu, bus) {
            AddressingResult::ProgramCounterRelative { address_rel } => address_rel,
            _ => panic!("No")
        };

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
fn BEQ(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::Z) {
        cpu.rem_cycles += 1;

        let offset = match addressing_mode(cpu, bus) {
            AddressingResult::ProgramCounterRelative { address_rel } => address_rel,
            _ => panic!("No")
        };

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
fn BNE(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::Z) {
        cpu.rem_cycles += 1;

        let offset = match addressing_mode(cpu, bus) {
            AddressingResult::ProgramCounterRelative { address_rel } => address_rel,
            _ => panic!("No")
        };

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
fn BMI(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::N) {
        cpu.rem_cycles += 1;

        let offset = match addressing_mode(cpu, bus) {
            AddressingResult::ProgramCounterRelative { address_rel } => address_rel,
            _ => panic!("No")
        };

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
fn BPL(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::N) {
        cpu.rem_cycles += 1;

        let offset = match addressing_mode(cpu, bus) {
            AddressingResult::ProgramCounterRelative { address_rel } => address_rel,
            _ => panic!("No")
        };

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
fn BVC(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    if !CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::V) {
        cpu.rem_cycles += 1;

        let offset = match addressing_mode(cpu, bus) {
            AddressingResult::ProgramCounterRelative { address_rel } => address_rel,
            _ => panic!("No")
        };

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
fn BVS(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::V) {
        cpu.rem_cycles += 1;

        let offset = match addressing_mode(cpu, bus) {
            AddressingResult::ProgramCounterRelative { address_rel } => address_rel,
            _ => panic!("No")
        };

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
fn CLC(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, false);
    return 0;
}

// Clear decimal flag (but we don't use it ???)
fn CLD(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::D, false);
    return 0;
}

// Disable interrupts
fn CLI(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::I, false);
    return 0;
}

// Clear overflow
fn CLV(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::V, false);
    return 0;
}

// Set carry flag
fn SEC(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, true);
    return 0;
}

// Set decimal flag
fn SED(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::D, true);
    return 0;
}

// Set interrupt flag
fn SEI(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::I, true);
    return 0;
}

// Add with carry
fn ADC(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode) as u16;
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
fn SBC(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode) as u16 ^ 0x00FFu16;
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
fn PHA(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(0x0100u16 + cpu.sp as u16, cpu.a);
    cpu.sp -= 1;
    return 0;
}

// Pop A from stack
fn PLA(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.sp += 1;
    cpu.a = bus.read(0x0100u16 + cpu.sp as u16, false);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);

    return 0;
}

// Push status register to stack
fn PHP(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(0x0100u16 + cpu.sp as u16, cpu.flags.bits | CpuStateFlags::B.bits | CpuStateFlags::U.bits);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::B, false);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::U, false);
    cpu.sp -= 1;
    return 0;
}

// Pop status register from stack
fn PLP(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.sp += 1;
    cpu.flags.bits = bus.read(0x0100u16 + cpu.sp as u16, false);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::U, true);

    return 0;
}

// Break (manual interrupt)
fn BRK(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
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
fn RTI(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
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

fn JSR(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.sp = cpu.sp.wrapping_sub(1);

    // Push PC
    bus.write(0x0100 + cpu.sp as u16, (cpu.pc >> 8) as u8);
    cpu.sp -= 1;
    bus.write(0x0100 + cpu.sp as u16, (cpu.pc & 0x00FFu16) as u8);
    cpu.sp -= 1;

    cpu.pc = Cpu::address_rel(cpu, bus, addressing_mode);
    return 0;
}

// Return from subroutine
fn RTS(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.sp += 1;
    let pc_lo = bus.read(0x0100u16 + cpu.sp as u16, false) as u16;
    cpu.sp += 1;
    let pc_hi = bus.read(0x0100u16 + cpu.sp as u16, false) as u16;

    cpu.pc = (pc_hi << 8) | pc_lo;
    cpu.pc += 1;
    return 0;
}

// Stores A
fn STA(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(Cpu::address(cpu, bus, addressing_mode), cpu.a);
    return 0;
}

// Stores X
fn STX(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(Cpu::address(cpu, bus, addressing_mode), cpu.x);
    return 0;
}

// Stores Y
fn STY(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(Cpu::address(cpu, bus, addressing_mode), cpu.y);
    return 0;
}

// Xfer A to X
fn TAX(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.x = cpu.a;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.x == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.x & 0x80u8 != 0);
    return 0;
}

// Xfer A to Y
fn TAY(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.y = cpu.a;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.y == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.y & 0x80u8 != 0);
    return 0;
}

// Xfer SP TO X
fn TSX(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.x = cpu.sp;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.x == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.x & 0x80u8 != 0);
    return 0;
}

// Xfer X to A
fn TXA(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.a = cpu.x;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 0;
}

// Xfer X to SP
fn TXS(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.sp = cpu.x;
    return 0;
}

// Xfer Y to A
fn TYA(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.a = cpu.y;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 0;
}

fn LDA(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    cpu.a = fetched;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 0;
}

fn LDX(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    cpu.x = fetched;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.x == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.x & 0x80u8 != 0);
    return 0;
}

fn LDY(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    cpu.y = fetched;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.y == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.y & 0x80u8 != 0);
    return 0;
}

// Load State Register
fn LSR(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, (fetched & 0x01) == 0x01);
    let temp = fetched >> 1;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (fetched & 0x00FF) == 0x0000);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (fetched & 0x0080) != 0x0000);

    if let AddressingResult::Implicit { data } = addressing_mode(cpu, bus) {
        cpu.a = (temp & 0x0FF) as u8;
    } else {
        bus.write(Cpu::address(cpu, bus, addressing_mode), (temp & 0x00FF) as u8);
    }

    return 0;
}

// No op
fn NOP(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    return 1;
}

// Bitwise And
fn AND(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    cpu.a = cpu.a & fetched;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 1;
}

// Bitwise Or
fn ORA(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    cpu.a = cpu.a | fetched;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 1;
}

// Bitwise Xor
fn EOR(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    cpu.a = cpu.a ^ fetched;
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, cpu.a == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, cpu.a & 0x80u8 != 0);
    return 1;
}

// Shift left
fn ROL(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode) as u16;
    let carry_in = if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) { 1u16 } else { 0u16 };
    let temp = fetched << 1 | carry_in;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, (temp & 0xFF00u16) != 0);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);

    if let AddressingResult::Implicit { data } = addressing_mode(cpu,bus) {
        cpu.a = (temp & 0x0FF) as u8;
    } else {
        bus.write(Cpu::address(cpu, bus, addressing_mode), (temp & 0x00FF) as u8);
    }

    return 0;
}

// Shift right (beamng is better tbh)
fn ROR(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode) as u16;
    let carry_in = if CpuStateFlags::contains(&mut cpu.flags, CpuStateFlags::C) { 1u16 } else { 0u16 };
    let temp = carry_in << 7 | fetched >> 1;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, (temp & 0xFF00u16) != 0);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);

    if let AddressingResult::Implicit { data } = addressing_mode(cpu,bus) {
        cpu.a = (temp & 0x0FF) as u8;
    } else {
        bus.write(Cpu::address(cpu, bus, addressing_mode), (temp & 0x00FF) as u8);
    }

    return 0;
}

// Shift left
fn ASL(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode) as u16;
    let temp = fetched << 1;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, (temp & 0xFF00u16) != 0);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);

    if let AddressingResult::Implicit { data } = addressing_mode(cpu, bus) {
        cpu.a = (temp & 0x0FF) as u8;
    } else {
        bus.write(Cpu::address(cpu, bus, addressing_mode), (temp & 0x00FF) as u8);
    }

    return 0;
}

// Bit testing (does the mask match anything ?)
fn BIT(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    let temp = cpu.a & fetched;

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & (1 << 7)) != 0);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::V, (fetched & (1 << 6)) != 0);
    return 0;
}

// Compare A with ...
fn CMP(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    let temp = (cpu.a as u16).wrapping_sub(fetched as u16);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, cpu.a as u16 >= fetched as u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);
    return 1;
}

// Compare X with ...
fn CPX(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    let temp = (cpu.x as u16).wrapping_sub(fetched as u16);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, cpu.x as u16 >= fetched as u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);
    return 1;
}

// Compare Y with ...
fn CPY(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    let temp = (cpu.y as u16).wrapping_sub(fetched as u16);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::C, cpu.y as u16 >= fetched as u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (temp & 0x00FFu16) == 0x0000u16);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (temp & 0x0080u16) != 0);
    return 1;
}

// Decrement memory location
fn DEC(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    let temp = fetched.wrapping_sub(1u8);
    bus.write(Cpu::address(cpu, bus, addressing_mode), temp);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.x & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.x & 0x80u8) != 0);
    return 0;
}

// Decrement X
fn DEX(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.x = cpu.x.wrapping_sub(1);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.x & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.x & 0x80u8) != 0);
    return 0;
}

// Decrement Y
fn DEY(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.y = cpu.y.wrapping_sub(1);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.y & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.y & 0x80u8) != 0);
    return 0;
}

// Increment memory location
fn INC(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = Cpu::fetch(cpu, bus, addressing_mode);
    let temp = fetched.wrapping_add(1u8);
    bus.write(Cpu::address(cpu, bus, addressing_mode), temp);

    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.x & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.x & 0x80u8) != 0);
    return 0;
}

// Increment X
fn INX(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.x = cpu.x.wrapping_add(1);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.x & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.x & 0x80u8) != 0);
    return 0;
}

// Increment Y
fn INY(cpu: &mut Cpu, bus: &Bus, addressing_mode: AddressingMode) -> i8 {
    cpu.y = cpu.y.wrapping_add(1);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::Z, (cpu.y & 0xFFu8) == 0x00u8);
    CpuStateFlags::set(&mut cpu.flags, CpuStateFlags::N, (cpu.y & 0x80u8) != 0);
    return 0;
}