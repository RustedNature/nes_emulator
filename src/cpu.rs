use super::opcode::*;

const ZERO_RESULT: u8 = 0b0000_0000;
const NEGATIVE_RESULT: u8 = 0b1000_0000;
const STACK_START: u16 = 0x0100;
const STACK_END: u16 = 0x01ff;
const STACK_POINTER_START: u8 = 0xff;

pub const CARRY_FLAG: u8 = 0b0000_0001;
pub const ZERO_FLAG: u8 = 0b0000_0010;
pub const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
pub const DECIMAL_FLAG: u8 = 0b0000_1000; //not used in NES
pub const B_FLAG: u8 = 0b0001_0000;
pub const ALWAYS_1_FLAG: u8 = 0b0010_0000; //always 1
pub const OVERFLOW_FLAG: u8 = 0b0100_0000;
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
    pub stack_pointer: u8,
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
    pub fn memory_read_2_byte(&self, address: u16) -> u16 {
        let low = self.memory_read_byte(address) as u16;
        let hi = self.memory_read_byte(address + 1) as u16;
        (hi << 8) | low
    }
    pub fn memory_read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.stack_pointer = STACK_POINTER_START;
        self.program_counter = self.memory_read_2_byte(0xfffc);
    }
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..0x8000 + program.len()].copy_from_slice(&program[..]);
        self.memory_write_2_byte(0xfffc, 0x8000);
    }
    pub(crate) fn memory_write_2_byte(&mut self, address: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.memory_write_byte(address, low);
        self.memory_write_byte(address + 1, hi);
    }
    pub(crate) fn memory_write_byte(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
    fn push_byte_to_stack(&mut self, data: u8){
        self.memory_write_byte(STACK_START + self.stack_pointer as u16, data);
        self.stack_pointer -= 1;
    }
    fn pop_byte_from_stack(&mut self) -> u8{
        self.stack_pointer += 1;
        self.memory_read_byte(STACK_START + self.stack_pointer as u16)
    }
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub(crate) fn get_address_for(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.memory_read_byte(self.program_counter) as u16,

            AddressingMode::Absolute => self.memory_read_2_byte(self.program_counter),

            AddressingMode::ZeroPageX => {
                let pos = self.memory_read_byte(self.program_counter);

                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos = self.memory_read_byte(self.program_counter);

                pos.wrapping_add(self.register_y) as u16
            }

            AddressingMode::AbsoluteX => {
                let base = self.memory_read_2_byte(self.program_counter);

                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let base = self.memory_read_2_byte(self.program_counter);

                base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::IndirectX => {
                let base = self.memory_read_byte(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.memory_read_byte(ptr as u16);
                let hi = self.memory_read_byte(ptr.wrapping_add(1) as u16);
                ((hi as u16) << 8) | (lo as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.memory_read_byte(self.program_counter);

                let lo = self.memory_read_byte(base as u16);
                let hi = self.memory_read_byte(base.wrapping_add(1) as u16);
                let deref_base = ((hi as u16) << 8) | (lo as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Accumulator => {
                todo!()
            }
            AddressingMode::Relative => {
                let offset = self.memory_read_byte(self.program_counter);
                if self.if_relative_offset_negative(offset) {
                    self.program_counter - (offset & 0b0111_1111) as u16
                } else {
                    self.program_counter + (offset & 0b0111_1111) as u16
                }
            }
            AddressingMode::Implied => {
                todo!()
            }
            AddressingMode::Indirect => self.memory_read_2_byte(self.program_counter), //TODO: TESTING
            _ => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
    pub fn run(&mut self) {
        let mut opcode: OpCode;
        loop {
            match get_opcode_with_code(self.memory_read_byte(self.program_counter)) {
                Some(vale) => {
                    opcode = vale;
                }
                None => panic!(
                    "opcode with code {:X} not found",
                    self.memory_read_byte(self.program_counter)
                ),
            }
            self.program_counter += 1;

            match opcode.get_code() {
                //ADC
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    self.adc(opcode.get_addressing_mode());
                }
                //AND
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    self.and(opcode.get_addressing_mode());
                }
                //ASL
                0x0a | 0x06 | 0x16 | 0x0e | 0x1e => {
                    self.asl(opcode.get_addressing_mode());
                }
                //BCC
                0x90 => {
                    self.bcc(opcode.get_addressing_mode());
                }
                //BCS
                0xb0 => {
                    self.bcs(opcode.get_addressing_mode());
                }
                //BEQ
                0xf0 => {
                    self.beq(opcode.get_addressing_mode());
                }
                //BIT
                0x24 | 0x2c => self.bit(opcode.get_addressing_mode()),
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
                    self.inx();
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
            self.program_counter += (opcode.get_bytes() - 1) as u16;
        }
    }

    pub fn adc(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_for(addressing_mode);
        let memory_value = self.memory_read_byte(address);

        self.accumulator += memory_value;
        self.update_zero_and_negative_flag(self.accumulator);
    } //TODO: CARRY BIT

    fn and(&mut self, addressing_mode: &AddressingMode) {
        self.accumulator &= self.memory_read_byte(self.get_address_for(addressing_mode));
        self.update_zero_and_negative_flag(self.accumulator);
    }
    fn asl(&mut self, addressing_mode: &AddressingMode) {
        let out_shifted_bit: u8;
        match addressing_mode {
            AddressingMode::Accumulator => {
                out_shifted_bit = self.accumulator >> 7;
                self.accumulator <<= 1;
                self.update_zero_and_negative_flag(self.accumulator);
            }
            _ => {
                let address = self.get_address_for(addressing_mode);
                let mut memory_content = self.memory_read_byte(address);
                out_shifted_bit = memory_content >> 7;
                memory_content <<= 1;
                self.memory_write_byte(address, memory_content);
                self.update_zero_and_negative_flag(memory_content);
            }
        }
        self.set_carry_flag_to(out_shifted_bit);
    }
    //TODO: TEST FROM HERE
    fn bcc(&mut self, addressing_mode: &AddressingMode) {
        if !self.is_carry_flag_set() {
            self.program_counter = self.get_address_for(addressing_mode);
        }
    }
    fn bcs(&mut self, addressing_mode: &AddressingMode) {
        if self.is_carry_flag_set() {
            self.program_counter = self.get_address_for(addressing_mode);
        }
    }
    fn beq(&mut self, addressing_mode: &AddressingMode) {
        if self.is_zero_flag_set() {
            self.program_counter = self.get_address_for(addressing_mode);
        }
    }
    fn bit(&mut self, addressing_mode: &AddressingMode) {
        let memory_value = self.memory_read_byte(self.get_address_for(addressing_mode));
        let and_result = self.accumulator & memory_value;
        self.update_zero_flag(and_result);
        let overflow_flag_from_memory = (memory_value << 1) >> 7;
        let negative_flag_from_memory = memory_value >> 7;
        self.set_negative_flag_to(negative_flag_from_memory);
        self.set_overflow_flag_to(overflow_flag_from_memory);
    }
    fn bmi(&mut self, addressing_mode: &AddressingMode) {
        if self.is_negative_flag_set() {
            let new_program_counter_address = self.get_address_for(addressing_mode);
            self.program_counter = new_program_counter_address;
        }
    }
    fn bne(&mut self, addressing_mode: &AddressingMode) {
        if !self.is_zero_flag_set() {
            let new_program_counter_address = self.get_address_for(addressing_mode);
            self.program_counter = new_program_counter_address;
        }
    }
    fn bpl(&mut self, addressing_mode: &AddressingMode) {
        if !self.is_negative_flag_set() {
            let new_program_counter_address = self.get_address_for(addressing_mode);
            self.program_counter = new_program_counter_address;
        }
    }
    fn brk(&mut self, addressing_mode: &AddressingMode) {
        todo!();
    }

    pub(crate) fn lda(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_for(addressing_mode);
        let value = self.memory_read_byte(address);

        self.accumulator = value;
        self.update_zero_and_negative_flag(self.accumulator);
    }

    pub(crate) fn sta(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_for(addressing_mode);
        self.memory_write_byte(address, self.accumulator);
    }

    pub(crate) fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.update_zero_and_negative_flag(self.register_x);
    }

    pub(crate) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flag(self.register_x);
    }

    pub(crate) fn update_zero_and_negative_flag(&mut self, result_of_last_operation: u8) {
        self.update_zero_flag(result_of_last_operation);

        self.update_negative_flag(result_of_last_operation);
    }

    fn update_negative_flag(&mut self, byte_to_check: u8) {
        if (byte_to_check & NEGATIVE_RESULT) != 0 {
            self.set_negative_flag();
        } else {
            self.reset_negative_flag();
        }
    }

    fn update_zero_flag(&mut self, byte_to_check: u8) {
        if byte_to_check == ZERO_RESULT {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }
    }

    fn set_negative_flag(&mut self) {
        self.status |= NEGATIVE_FLAG;
    }
    fn reset_negative_flag(&mut self) {
        self.status &= !NEGATIVE_FLAG;
    }

    fn set_zero_flag(&mut self) {
        self.status |= ZERO_FLAG;
    }
    fn reset_zero_flag(&mut self) {
        self.status &= !ZERO_FLAG;
    }

    fn set_carry_flag_to(&mut self, out_shifted_bit: u8) {
        if out_shifted_bit == 0x1 {
            self.status |= CARRY_FLAG;
        } else {
            self.status &= !CARRY_FLAG;
        }
    }

    fn set_overflow_flag_to(&mut self, isolated_overflow_bit: u8) {
        if isolated_overflow_bit == 0x1 {
            self.status |= OVERFLOW_FLAG;
        } else {
            self.status &= !OVERFLOW_FLAG;
        }
    }
    fn set_negative_flag_to(&mut self, isolated_negative_bit: u8) {
        if isolated_negative_bit == 0x1 {
            self.status |= NEGATIVE_FLAG;
        } else {
            self.status &= !NEGATIVE_FLAG;
        }
    }
    fn is_carry_flag_set(&self) -> bool {
        let carry_bit = self.status << 7;
        carry_bit == 1
    }
    fn is_zero_flag_set(&self) -> bool {
        let zero_bit = (self.status >> 1) << 7;
        zero_bit == 1
    }
    fn is_negative_flag_set(&self) -> bool {
        let negative_bit = self.status >> 7;
        negative_bit == 1
    }
    fn if_relative_offset_negative(&self, offset: u8) -> bool {
        offset >> 7 == 1
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}
