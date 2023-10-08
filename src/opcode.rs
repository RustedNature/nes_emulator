pub(crate) use super::cpu::*;

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<OpCode> = vec![
        OpCode::new(Instruction::ADC, 0x69, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::ADC, 0x65, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ADC, 0x75, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ADC, 0x6d, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::ADC, 0x7d, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::ADC, 0x79, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::ADC, 0x61, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::ADC, 0x61, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::AND, 0x29, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::AND, 0x25, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::AND, 0x35, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::AND, 0x2d, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::AND, 0x3d, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::AND, 0x39, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::AND, 0x21, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::AND, 0x31, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::ASL, 0x0a, 1, 2, AddressingMode::Accumulator),
        OpCode::new(Instruction::ASL, 0x06, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ASL, 0x16, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ASL, 0x0e, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::ASL, 0x1e, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::BCC, 0x90, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BCS, 0xb0, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BEQ, 0xf0, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BIT, 0x24, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::BIT, 0x2c, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::BMI, 0x30, 2, 2, AddressingMode::Relative),
        //
        OpCode::new(Instruction::BNE, 0xd0, 2, 2, AddressingMode::Relative),
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
        OpCode::new(Instruction::CLD, 0xd8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::CLI, 0x58, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::CLV, 0xb8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::CMP, 0xc9, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::CMP, 0xc5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::CMP, 0xd5, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::CMP, 0xcd, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::CMP, 0xdd, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::CMP, 0xd9, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::CMP, 0xc1, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::CMP, 0xd1, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::CPX, 0xe0, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::CPX, 0xe4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::CPX, 0xec, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::CPY, 0xc0, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::CPY, 0xc4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::CPY, 0xcc, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::DEC, 0xc6, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::DEC, 0xd6, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::DEC, 0xce, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::DEC, 0xde, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::DEX, 0xca, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::DEY, 0x88, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::EOR, 0x49, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::EOR, 0x45, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::EOR, 0x55, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::EOR, 0x4d, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::EOR, 0x5d, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::EOR, 0x59, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::EOR, 0x41, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::EOR, 0x51, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::INC, 0xe6, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::INC, 0xf6, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::INC, 0xee, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::INC, 0xfe, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::INX, 0xe8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::INY, 0xc8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::JMP, 0x4c, 3, 3, AddressingMode::Absolute),
        OpCode::new(Instruction::JMP, 0x6c, 3, 5, AddressingMode::Indirect),
        //
        OpCode::new(Instruction::JSR, 0x20, 3, 6, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::LDA, 0xa9, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::LDA, 0xa5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::LDA, 0xb5, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::LDA, 0xad, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::LDA, 0xbd, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::LDA, 0xb9, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::LDA, 0xa1, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::LDA, 0xb1, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::LDX, 0xa2, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::LDX, 0xa6, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::LDX, 0xb6, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(Instruction::LDX, 0xae, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::LDX, 0xbe, 3, 4, AddressingMode::AbsoluteY),
        //
        OpCode::new(Instruction::LDY, 0xa0, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::LDY, 0xa4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::LDY, 0xb4, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::LDY, 0xac, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::LDY, 0xbc, 3, 4, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::LSR, 0x4a, 1, 2, AddressingMode::Accumulator),
        OpCode::new(Instruction::LSR, 0x46, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::LSR, 0x56, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::LSR, 0x4e, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::LSR, 0x5e, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::NOP, 0xea, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::ORA, 0x09, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::ORA, 0x05, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ORA, 0x15, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ORA, 0x0d, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::ORA, 0x1d, 3, 4, AddressingMode::AbsoluteX),
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
        OpCode::new(Instruction::ROL, 0x2a, 1, 2, AddressingMode::Accumulator),
        OpCode::new(Instruction::ROL, 0x26, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ROL, 0x36, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ROL, 0x2e, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::ROL, 0x3e, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::ROR, 0x6a, 1, 2, AddressingMode::Accumulator),
        OpCode::new(Instruction::ROR, 0x66, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(Instruction::ROR, 0x76, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::ROR, 0x6e, 3, 6, AddressingMode::Absolute),
        OpCode::new(Instruction::ROR, 0x7e, 3, 7, AddressingMode::AbsoluteX),
        //
        OpCode::new(Instruction::RTI, 0x40, 1, 6, AddressingMode::Implied),
        //
        OpCode::new(Instruction::RTS, 0x60, 1, 6, AddressingMode::Implied),
        //
        OpCode::new(Instruction::SBC, 0xe9, 2, 2, AddressingMode::Immediate),
        OpCode::new(Instruction::SBC, 0xe5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::SBC, 0xf5, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::SBC, 0xed, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::SBC, 0xfd, 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::SBC, 0xf9, 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::SBC, 0xe1, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::SBC, 0xf1, 2, 5, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::SEC, 0x38, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::SED, 0xf8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::SEI, 0x78, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::STA, 0x85, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::STA, 0x95, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::STA, 0x8d, 3, 4, AddressingMode::Absolute),
        OpCode::new(Instruction::STA, 0x9d, 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(Instruction::STA, 0x99, 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(Instruction::STA, 0x81, 2, 6, AddressingMode::IndirectX),
        OpCode::new(Instruction::STA, 0x91, 2, 6, AddressingMode::IndirectY),
        //
        OpCode::new(Instruction::STX, 0x86, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::STX, 0x96, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(Instruction::STX, 0x8e, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::STY, 0x84, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(Instruction::STY, 0x94, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(Instruction::STY, 0x8c, 3, 4, AddressingMode::Absolute),
        //
        OpCode::new(Instruction::TAX, 0xaa, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TAY, 0xa8, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TSX, 0xba, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TXA, 0x8a, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TXS, 0x9a, 1, 2, AddressingMode::Implied),
        //
        OpCode::new(Instruction::TYA, 0x98, 1, 2, AddressingMode::Implied)
    ];
}
#[derive(Clone)]
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
#[derive(Clone)]
pub struct OpCode {
    instruction: Instruction,
    code: u8,
    bytes: u8,
    cycles: u8,
    addressing_mode: AddressingMode,
}
impl OpCode {
    pub fn new(
        instruction: Instruction,
        code: u8,
        bytes: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        OpCode {
            instruction,
            code,
            bytes,
            cycles,
            addressing_mode,
        }
    }
    pub fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }
    pub fn get_code(&self) -> u8 {
        self.code
    }
    pub fn get_bytes(&self) -> u8 {
        self.bytes
    }
    pub fn get_cycles(&self) -> u8 {
        self.cycles
    }
    pub fn get_addressing_mode(&self) -> &AddressingMode {
        &self.addressing_mode
    }
}

pub fn get_opcode_with_code(code: u8) -> Option<OpCode> {
    for (index, opcode) in CPU_OP_CODES.iter().enumerate() {
        if opcode.code == code {
            return Some(CPU_OP_CODES[index].clone());
        }
    }
    None
}
