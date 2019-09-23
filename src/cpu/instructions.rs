use crate::cpu::addressing_modes::*;
use crate::bus::Bus;
use crate::cpu::R6502;
use crate::cpu::CpuStateFlags;

//pub const INSTRUCTIONS: &'static [Instruction] = &[
pub const INSTRUCTIONS: [Instruction;256] = [
    //i("BRK", IMM, break_implem, 7)
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

// won't bother with this tbh
/*const fn i(name: &'static str, addressing_mode: AddressingMode, actual_work: InstructionWork, cycles: i8) -> Instruction {
    return Instruction {
        name,
        fn_pointer: actual_work,
        cycles: cycles,
    }
}*/

pub struct Instruction {
    pub name: &'static str,
    pub work: InstructionWork,
    pub addressing: AddressingMode,
    pub(crate) cycles: i8,
}

type InstructionWork = fn(&mut Bus, AddressingMode) -> i8;

fn XXX(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    panic!("Illegal instruction")
}

fn AND(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = R6502::fetch(bus, addressing_mode);
    bus.cpu.a = bus.cpu.a & fetched;
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.a == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.a & 0x80u8 != 0);
    return 1;
}

// Branch on carry set
fn BCS(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    if CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::C) {
        bus.cpu.rem_cycles += 1;

        let offset = match addressing_mode(bus) {
            AddressingResult::Relative { address_rel } => address_rel,
            _ => panic!("No")
        };

        let address_abs = bus.cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (bus.cpu.pc & 0xFF00u16) {
            bus.cpu.rem_cycles += 1;
        }

        bus.cpu.pc = address_abs;
    }
    return 0;
}

// Branch on carry clear
fn BCC(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    if !CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::C) {
        bus.cpu.rem_cycles += 1;

        let offset = match addressing_mode(bus) {
            AddressingResult::Relative { address_rel } => address_rel,
            _ => panic!("No")
        };

        let address_abs = bus.cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (bus.cpu.pc & 0xFF00u16) {
            bus.cpu.rem_cycles += 1;
        }

        bus.cpu.pc = address_abs;
    }
    return 0;
}

// Branch if equal
fn BEQ(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    if CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::Z) {
        bus.cpu.rem_cycles += 1;

        let offset = match addressing_mode(bus) {
            AddressingResult::Relative { address_rel } => address_rel,
            _ => panic!("No")
        };

        let address_abs = bus.cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (bus.cpu.pc & 0xFF00u16) {
            bus.cpu.rem_cycles += 1;
        }

        bus.cpu.pc = address_abs;
    }
    return 0;
}

// Branch if not equal
fn BNE(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    if !CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::Z) {
        bus.cpu.rem_cycles += 1;

        let offset = match addressing_mode(bus) {
            AddressingResult::Relative { address_rel } => address_rel,
            _ => panic!("No")
        };

        let address_abs = bus.cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (bus.cpu.pc & 0xFF00u16) {
            bus.cpu.rem_cycles += 1;
        }

        bus.cpu.pc = address_abs;
    }
    return 0;
}

// Branch if negative (N set)
fn BMI(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    if CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::N) {
        bus.cpu.rem_cycles += 1;

        let offset = match addressing_mode(bus) {
            AddressingResult::Relative { address_rel } => address_rel,
            _ => panic!("No")
        };

        let address_abs = bus.cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (bus.cpu.pc & 0xFF00u16) {
            bus.cpu.rem_cycles += 1;
        }

        bus.cpu.pc = address_abs;
    }
    return 0;
}

// Branch if positive (N not set)
fn BPL(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    if !CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::N) {
        bus.cpu.rem_cycles += 1;

        let offset = match addressing_mode(bus) {
            AddressingResult::Relative { address_rel } => address_rel,
            _ => panic!("No")
        };

        let address_abs = bus.cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (bus.cpu.pc & 0xFF00u16) {
            bus.cpu.rem_cycles += 1;
        }

        bus.cpu.pc = address_abs;
    }
    return 0;
}

// Branch if overflow clear
fn BVC(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    if !CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::V) {
        bus.cpu.rem_cycles += 1;

        let offset = match addressing_mode(bus) {
            AddressingResult::Relative { address_rel } => address_rel,
            _ => panic!("No")
        };

        let address_abs = bus.cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (bus.cpu.pc & 0xFF00u16) {
            bus.cpu.rem_cycles += 1;
        }

        bus.cpu.pc = address_abs;
    }
    return 0;
}

// Branch if overflow set
fn BVS(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    if CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::V) {
        bus.cpu.rem_cycles += 1;

        let offset = match addressing_mode(bus) {
            AddressingResult::Relative { address_rel } => address_rel,
            _ => panic!("No")
        };

        let address_abs = bus.cpu.pc.wrapping_add(offset);

        // Page cross penalty
        if (address_abs & 0xFF00u16) != (bus.cpu.pc & 0xFF00u16) {
            bus.cpu.rem_cycles += 1;
        }

        bus.cpu.pc = address_abs;
    }
    return 0;
}

// Clear carry bit
fn CLC(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::C, false);
    return 0;
}

// Clear decimal flag (but we don't use it ???)
fn CLD(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::D, false);
    return 0;
}

// Disable interrupts
fn CLI(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::I, false);
    return 0;
}

// Set carry flag
fn SEC(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::C, true);
    return 0;
}

// Set decimal flag
fn SED(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::D, true);
    return 0;
}

// Set interrupt flag
fn SEI(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::I, true);
    return 0;
}

// Add with carry
fn ADC(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = R6502::fetch(bus, addressing_mode) as u16;
    let carry_in = if CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::C) { 1u16 } else { 0u16 };

    let temp = (bus.cpu.a as u16) + (fetched) + carry_in;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::C, temp > 255);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.a == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.a & 0x80u8 != 0);

    let v = (!((bus.cpu.a as u16) ^ (fetched)) & ((bus.cpu.a as u16) ^ (temp)) & 0x0080u16) != 0;
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::V, v);

    bus.cpu.a = (temp & 0x00ffu16) as u8;

    return 1;
}

// Substract with borrow in
fn SBC(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = R6502::fetch(bus, addressing_mode) as u16 ^ 0x00FFu16;
    let borrow_in = if CpuStateFlags::contains(&mut bus.cpu.flags, CpuStateFlags::C) { 1u16 } else { 0u16 };

    let temp = (bus.cpu.a as u16) + (fetched) + borrow_in;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::C, temp > 255);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.a == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.a & 0x80u8 != 0);

    let v = (!((bus.cpu.a as u16) ^ (fetched)) & ((bus.cpu.a as u16) ^ (temp)) & 0x0080u16) != 0;
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::V, v);

    bus.cpu.a = (temp & 0x00ffu16) as u8;

    return 1;
}

// Push A to stack
fn PHA(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(0x0100u16 + bus.cpu.sp as u16, bus.cpu.a);
    bus.cpu.sp -= 1;
    return 0;
}

// Pop A from stack
fn PLA(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.sp += 1;
    bus.cpu.a = bus.read(0x0100u16 + bus.cpu.sp as u16, false);

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.a == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.a & 0x80u8 != 0);

    return 0;
}

// Return from interrupt
fn RTI(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.sp += 1;
    bus.cpu.flags.bits = bus.read(0x0100u16 + bus.cpu.sp as u16, false);

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::B, false);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::U, false);

    bus.cpu.sp += 1;
    let pc_lo = bus.read(0x0100u16 + bus.cpu.sp as u16, false) as u16;
    bus.cpu.sp += 1;
    let pc_hi = bus.read(0x0100u16 + bus.cpu.sp as u16, false) as u16;

    bus.cpu.pc = (pc_hi << 8) | pc_lo;
    return 0;
}

// Return from subroutine
fn RTS(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.sp += 1;
    let pc_lo = bus.read(0x0100u16 + bus.cpu.sp as u16, false) as u16;
    bus.cpu.sp += 1;
    let pc_hi = bus.read(0x0100u16 + bus.cpu.sp as u16, false) as u16;

    bus.cpu.pc = (pc_hi << 8) | pc_lo;
    bus.cpu.pc += 1;
    return 0;
}

// Stores A
fn STA(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(R6502::address(bus, addressing_mode), bus.cpu.a);
    return 0;
}

// Stores X
fn STX(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(R6502::address(bus, addressing_mode), bus.cpu.x);
    return 0;
}

// Stores Y
fn STY(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.write(R6502::address(bus, addressing_mode), bus.cpu.y);
    return 0;
}

// Xfer A to X
fn TAX(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.x = bus.cpu.a;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.x == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.x & 0x80u8 != 0);
    return 0;
}

// Xfer A to Y
fn TAY(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.y = bus.cpu.a;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.y == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.y & 0x80u8 != 0);
    return 0;
}

// Xfer SP TO X
fn TSX(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.x = bus.cpu.sp;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.x == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.x & 0x80u8 != 0);
    return 0;
}

// Xfer X to A
fn TXA(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.a = bus.cpu.x;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.a == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.a & 0x80u8 != 0);
    return 0;
}

// Xfer X to SP
fn TXS(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.sp = bus.cpu.x;
    return 0;
}

// Xfer Y to A
fn TYA(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    bus.cpu.a = bus.cpu.y;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.a == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.a & 0x80u8 != 0);
    return 0;
}

fn LDA(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = R6502::fetch(bus, addressing_mode);
    bus.cpu.a = fetched;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.a == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.a & 0x80u8 != 0);
    return 0;
}

fn LDX(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = R6502::fetch(bus, addressing_mode);
    bus.cpu.x = fetched;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.x == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.x & 0x80u8 != 0);
    return 0;
}

fn LDY(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = R6502::fetch(bus, addressing_mode);
    bus.cpu.y = fetched;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, bus.cpu.y == 0x00u8);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, bus.cpu.y & 0x80u8 != 0);
    return 0;
}

// Load State Register
fn LSR(bus: &mut Bus, addressing_mode: AddressingMode) -> i8 {
    let fetched = R6502::fetch(bus, addressing_mode);

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::C, fetched & 0x01);
    let temp = fetched >> 1;

    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::Z, (fetched & 0x00FF) == 0x0000);
    CpuStateFlags::set(&mut bus.cpu.flags, CpuStateFlags::N, (fetched & 0x0080) != 0x0000);
}
