use super::opcode::*;

const ZERO_RESULT: u8 = 0b0000_0000;
const NEGATIVE_RESULT: u8 = 0b1000_0000;
pub const ZERO_FLAG: u8 = 0b0000_0010;
pub const NEGATIVE_FLAG: u8 = 0b1000_0000;

#[derive(Debug, Clone)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteY,
    AbsoluteX,
    IndirectX,
    IndirectY,
    Accumulator,
    Relative,
    Implied,
    Indirect,
    NoneAddressing,
}

pub struct CPU {
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub memory: [u8; 0xffff],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            accumulator: 0x00,
            register_x: 0x00,
            register_y: 0x00,
            status: 0b0000_0000,
            program_counter: 0x0000,
            stack_pointer: 0x0000,
            memory: [0; 0xffff],
        }
    }
    pub fn mem_read_u16(&self, address: u16) -> u16 {
        let low = self.mem_read(address) as u16;
        let hi = self.mem_read(address + 1) as u16;
        (hi << 8) | low
    }
    pub fn mem_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xfffc);
    }
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..0x8000 + program.len()].copy_from_slice(&program[..]);
        self.mem_write_u16(0xfffc, 0x8000);
    }
    pub(crate) fn mem_write_u16(&mut self, address: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.mem_write(address, low);
        self.mem_write(address + 1, hi);
    }
    pub(crate) fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub(crate) fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                ((hi as u16) << 8) | (lo as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = ((hi as u16) << 8) | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::Accumulator => { todo!() }
            AddressingMode::Relative => {
                let offset = self.mem_read(self.program_counter) as i16;
                let addr = self.mem_read(((self.program_counter as i16) + offset) as u16);
                addr as u16 //TODO: TESTING
            }
            AddressingMode::Implied => { todo!() }
            AddressingMode::Indirect => self.mem_read_u16(self.program_counter), //TODO: TESTING
            _ => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
    pub fn run(&mut self) {
        let mut opcode: OpCode;
        loop {
            match get_opcode_with_code(self.mem_read(self.program_counter)) {
                Some(vale) => {
                    opcode = vale;
                }
                None =>
                    panic!("opcode with code {:X} not found", self.mem_read(self.program_counter)),
            }
            self.program_counter += 1;

            match opcode.get_code() {
                //ADC
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    self.adc(opcode.get_addressing_mode());
                    self.program_counter += (opcode.get_bytes() - 1) as u16;
                }
                //AND
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    self.and(opcode.get_addressing_mode());
                    self.program_counter += (opcode.get_bytes() - 1) as u16;
                }
                //ASL
                0x0a | 0x06 | 0x16 | 0x0e | 0x1e => {
                    todo!();
                }
                //BCC
                0x90 => {
                    todo!();
                }
                //BCS
                0xb0 => {
                    todo!();
                }
                //BEQ
                0xf0 => {
                    todo!();
                }
                //BIT
                0x24 | 0x2c => {
                    todo!();
                }
                //BMI
                0x30 => {
                    todo!();
                }
                //BNE
                0xd0 => {
                    todo!();
                }
                //BPL
                0x10 => {
                    todo!();
                }
                //BRK
                0x00 => {
                    break;
                }
                //BVC
                0x50 => {
                    todo!();
                }
                //BVS
                0x70 => {
                    todo!();
                }
                //CLC
                0x18 => {
                    todo!();
                }
                //CLD
                0xd8 => {
                    todo!();
                }
                //CLI
                0x58 => {
                    todo!();
                }
                //CLV
                0xb8 => {
                    todo!();
                }
                //CMP
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    todo!();
                }
                //CPX
                0xe0 | 0xe4 | 0xec => {
                    todo!();
                }
                //CPY
                0xc0 | 0xc4 | 0xcc => {
                    todo!();
                }
                //DEC
                0xc6 | 0xd6 | 0xce | 0xde => {
                    todo!();
                }
                //DEX
                0xca => {
                    todo!();
                }
                //DEY
                0x88 => {
                    todo!();
                }
                //EOR
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                    todo!();
                }
                //INC
                0xe6 | 0xf6 | 0xee | 0xfe => {
                    todo!();
                }
                //INX
                0xe8 => {
                    self.inx(opcode.get_addressing_mode());
                }
                //INY
                0xc8 => {
                    todo!();
                }
                //JMP
                0x4c | 0x6c => {
                    todo!();
                }
                //JSR
                0x20 => {
                    todo!();
                }
                //LDA
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    self.lda(opcode.get_addressing_mode());
                    self.program_counter += (opcode.get_bytes() - 1) as u16;
                }
                //LDX
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => {
                    todo!();
                }
                //LDY
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => {
                    todo!();
                }
                //LSR
                0x4a | 0x46 | 0x56 | 0x4e | 0x5e => {
                    todo!();
                }
                //NOP
                0xea => {
                    todo!();
                }
                //ORA
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                    todo!();
                }
                //PHA
                0x48 => {
                    todo!();
                }
                //PHP
                0x08 => {
                    todo!();
                }
                //PLA
                0x68 => {
                    todo!();
                }
                //PLP
                0x28 => {
                    todo!();
                }
                //ROL
                0x2a | 0x26 | 0x36 | 0x2e | 0x3e => {
                    todo!();
                }
                //ROR
                0x6a | 0x66 | 0x76 | 0x6e | 0x7e => {
                    todo!();
                }
                //RTI
                0x40 => {
                    todo!();
                }
                //RTS
                0x60 => {
                    todo!();
                }
                //SBC
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    todo!();
                }
                //SEC
                0x38 => {
                    todo!();
                }
                //SED
                0xf8 => {
                    todo!();
                }
                //SEI
                0x78 => {
                    todo!();
                }
                //STA
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.sta(opcode.get_addressing_mode());
                    self.program_counter += (opcode.get_bytes() - 1) as u16;
                }
                //STX
                0x86 | 0x96 | 0x8e => {
                    todo!();
                }
                //STY
                0x84 | 0x94 | 0x8c => {
                    todo!();
                }
                //TAX
                0xaa => {
                    self.tax();
                    self.program_counter += (opcode.get_bytes() - 1) as u16;
                }
                //TAY
                0xa8 => {
                    todo!();
                }
                //TSX
                0xba => {
                    todo!();
                }
                //TXA
                0x8a => {
                    todo!();
                }
                //TXS
                0x9a => {
                    todo!();
                }
                //TYA
                0x98 => {
                    todo!();
                }
                _ => {
                    panic!("opcode {:X} not found", opcode.get_code());
                }
            }
        }
    }

    pub fn adc(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_operand_address(addressing_mode);
        let memory_value = self.mem_read(address);

        self.accumulator += memory_value;
        self.update_zero_and_negative_flag(self.accumulator);
    }

    pub(crate) fn lda(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_operand_address(addressing_mode);
        let value = self.mem_read(address);

        self.accumulator = value;
        self.update_zero_and_negative_flag(self.accumulator);
    }
    pub(crate) fn sta(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_operand_address(addressing_mode);
        self.mem_write(address, self.accumulator);
    }
    pub(crate) fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.update_zero_and_negative_flag(self.register_x);
    }
    pub(crate) fn inx(&mut self, addressing_mode: &AddressingMode) {
        if self.register_x == 0xff {
            self.register_x = 0x00;
        } else {
            self.register_x += 1;
        }

        self.update_zero_and_negative_flag(self.register_x);
    }

    pub(crate) fn update_zero_and_negative_flag(&mut self, result_of_last_operation: u8) {
        if result_of_last_operation == ZERO_RESULT {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        if (result_of_last_operation & NEGATIVE_RESULT) != 0 {
            self.set_negative_flag();
        } else {
            self.reset_negative_flag();
        }
    }

    fn set_negative_flag(&mut self) {
        self.status |= 0b1000_0000;
    }
    fn reset_negative_flag(&mut self) {
        self.status &= 0b0111_1111;
    }

    fn set_zero_flag(&mut self) {
        self.status |= 0b0000_0010;
    }
    fn reset_zero_flag(&mut self) {
        self.status &= 0b1111_1101;
    }

    fn and(&mut self, addressing_mode: &AddressingMode) {
        self.accumulator &= self.mem_read(self.get_operand_address(addressing_mode));
        self.update_zero_and_negative_flag(self.accumulator);
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}
