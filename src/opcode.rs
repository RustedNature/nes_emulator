pub(crate) use super::cpu::*;

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<OpCode> = vec![
        OpCode::new(Instruction::ADC, 0x69, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::ADC, 0x65, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ADC, 0x75, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ADC, 0x6D, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::ADC, 0x7D, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::ADC, 0x79, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::ADC, 0x61, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::ADC, 0x61, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::AND, 0x29, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::AND, 0x25, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::AND, 0x35, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::AND, 0x2D, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::AND, 0x3D, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::AND, 0x39, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::AND, 0x21, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::AND, 0x31, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::ASL, 0x0A, 1, 2, AddressingMode::Accumulator),
        OpCode::new(Instruction::ASL, 0x06, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ASL, 0x16, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ASL, 0x0E, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::ASL, 0x1E, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::BCC, 0x90, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BCS, 0xB0, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BEQ, 0xF0, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BIT, 0x24, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::BIT, 0x2C, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::BMI, 0x30, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BNE, 0xD0, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BPL, 0x10, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BRK, 0x00, 1, 7, AddressingMode::Implied),
        //
        OpCode::new(Instruction::BVC, 0x50, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BVS, 0x70, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::CLC, 0x18, 1, 2, AddressingMode::Implied),
    ];
}

pub enum Instruction {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}
pub struct OpCode {
    instruction: Instruction,
    opcode: u8,
    bytes: u8,
    cycles: u8,
    address_mode: AddressingMode,
}
impl OpCode {
    pub fn new(
        instruction: Instruction,
        opcode: u8,
        bytes: u8,
        cycles: u8,
        address_mode: AddressingMode,
    ) -> Self {
        OpCode {
            instruction,
            opcode,
            bytes,
            cycles,
            address_mode,
        }
    }
}
