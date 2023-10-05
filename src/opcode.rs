use std::arch::x86_64::_CMP_EQ_OQ;

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
        //
        OpCode::new(Instruction::CLD, 0xD8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::CLI, 0x58, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::CLV, 0xB8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::CMP, 0xC9, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::CMP, 0xC5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::CMP, 0xD5, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::CMP, 0xCD, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::CMP, 0xDD, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::CMP, 0xD9, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::CMP, 0xC1, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::CMP, 0xD1, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::CPX, 0xE0, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::CPX, 0xE4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::CPX, 0xEC, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::CPY, 0xC0, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::CPY, 0xC4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::CPY, 0xCC, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::DEC, 0xC6, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::DEC, 0xD6, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::DEC, 0xCE, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::DEC, 0xDE, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::DEX, 0xCA, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::DEY, 0x88, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::EOR, 0x49, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::EOR, 0x45, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::EOR, 0x55, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::EOR, 0x4D, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::EOR, 0x5D, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::EOR, 0x59, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::EOR, 0x41, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::EOR, 0x51, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::INC, 0xE6, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::INC, 0xF6, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::INC, 0xEE, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::INC, 0xFE, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::INX, 0xE8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::INY, 0xC8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::JMP, 0x4C, 3, 3, AddressingMode::Absolute),
        OpCode::new(Instruction::JMP, 0x6C, 3, 5, AddressingMode::Indirect),
        //
        OpCode::new(Instruction::JSR, 0x20, 3, 6, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::LDA, 0xA9, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::LDA, 0xA5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::LDA, 0xB5, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::LDA, 0xAD, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::LDA, 0xBD, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::LDA, 0xB9, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::LDA, 0xA1, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::LDA, 0xB1, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::LDX, 0xA2, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::LDX, 0xA6, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::LDX, 0xB6, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(Instruction::LDX, 0xAE, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::LDX, 0xBE, 3, 4, AddressingMode::AbsoluteY),
        //
        OpCode::new(Instruction::LDY, 0xA0, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::LDY, 0xA4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::LDY, 0xB4, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::LDY, 0xAC, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::LDY, 0xBC, 3, 4, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::LSR, 0x4A, 1, 2, AddressingMode::Accumulator),
        OpCode::new(Instruction::LSR, 0x46, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::LSR, 0x56, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::LSR, 0x4E, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::LSR, 0x5E, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::NOP, 0xEA, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::ORA, 0x09, 2, 2, AddressingMode::Immediate),  
        OpCode::new(Instruction::ORA, 0x05, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ORA, 0x15, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ORA, 0x0D, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::ORA, 0x1D, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::ORA, 0x19, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::ORA, 0x01, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::ORA, 0x11, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::PHA, 0x48, 1, 3, AddressingMode::Implied),
        //
        OpCode::new(Instruction::PHP, 0x08, 1, 3, AddressingMode::Implied),
        //
        OpCode::new(Instruction::PLA, 0x68, 1, 4, AddressingMode::Implied),
        //
        OpCode::new(Instruction::PLP, 0x28, 1, 4, AddressingMode::Implied),
        //
        OpCode::new(Instruction::ROL, 0x2A, 1, 2, AddressingMode::Accumulator),
        OpCode::new(Instruction::ROL, 0x26, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ROL, 0x36, 2, 6, AddressingMode::ZeroPageX),   
        OpCode::new(Instruction::ROL, 0x2E, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::ROL, 0x3E, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::ROR, 0x6A, 1, 2, AddressingMode::Accumulator),
        OpCode::new(Instruction::ROR, 0x66, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ROR, 0x76, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ROR, 0x6E, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::ROR, 0x7E, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::RTI, 0x40, 1, 6, AddressingMode::Implied),
        //
        OpCode::new(Instruction::RTS, 0x60, 1, 6, AddressingMode::Implied),
        //
        OpCode::new(Instruction::SBC, 0xE9, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::SBC, 0xE5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::SBC, 0xF5, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::SBC, 0xED, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::SBC, 0xFD, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::SBC, 0xF9, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::SBC, 0xE1, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::SBC, 0xF1, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::SEC, 0x38, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::SED, 0xF8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::SEI, 0x78, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::STA, 0x85, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::STA, 0x95, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::STA, 0x8D, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::STA, 0x9D, 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::STA, 0x99, 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::STA, 0x81, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::STA, 0x91, 2, 6, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::STX, 0x86, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::STX, 0x96, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(Instruction::STX, 0x8E, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::STY, 0x84, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::STY, 0x94, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::STY, 0x8C, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::TAX, 0xAA, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TAY, 0xA8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TSX, 0xBA, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TXA, 0x8A, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TXS, 0x9A, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TYA, 0x98, 1, 2, AddressingMode::Implied),
        
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
