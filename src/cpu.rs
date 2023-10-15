use super::opcode::*;

const ZERO_RESULT: u8 = 0b0000_0000;
const NEGATIVE_RESULT: u8 = 0b1000_0000;
const STACK_START: u16 = 0x0100;
const STACK_END: u16 = 0x01ff;
const STACK_POINTER_START: u8 = 0xff;
const INTEERRUPT_VECTOR_MEMEROY_ADDRESS: u16 = 0xfffe;

pub const CARRY_FLAG: u8 = 0b0000_0001;
pub const ZERO_FLAG: u8 = 0b0000_0010;
pub const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
pub const DECIMAL_FLAG: u8 = 0b0000_1000; //not used in NES
pub const BREAK_FLAG: u8 = 0b0001_0000;
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
    fn push_byte_to_stack(&mut self, data: u8) {
        self.memory_write_byte(STACK_START + self.stack_pointer as u16, data);
        self.stack_pointer -= 1;
    }
    fn push_2_byte_to_stack(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.push_byte_to_stack(hi);
        self.push_byte_to_stack(low);
    }
    fn pop_byte_from_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.memory_read_byte(STACK_START + self.stack_pointer as u16)
    }
    fn pop_2_byte_from_stack(&mut self) -> u16 {
        let low = self.pop_byte_from_stack();
        let hi = self.pop_byte_from_stack();
        ((hi as u16) << 8) | (low as u16)
    }
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    fn get_address_from(&self, mode: &AddressingMode) -> u16 {
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

            AddressingMode::Relative => {
                let offset = self.memory_read_byte(self.program_counter);
                if self.is_negativ_bit_present(offset) {
                    self.program_counter - (offset & 0b0111_1111) as u16
                } else {
                    self.program_counter + (offset & 0b0111_1111) as u16
                }
            }

            AddressingMode::Indirect => self.memory_read_2_byte(self.program_counter),
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
                0x30 => self.bmi(opcode.get_addressing_mode()),
                //BNE
                0xd0 => {
                    self.bne(opcode.get_addressing_mode());
                }
                //BPL
                0x10 => {
                    self.bpl(opcode.get_addressing_mode());
                }
                //BRK
                0x00 => {
                    //FIXME: self.brk(opcode.get_addressing_mode());
                    break;
                }
                //BVC
                0x50 => {
                    self.bvc(opcode.get_addressing_mode());
                }
                //BVS
                0x70 => {
                    self.bvs(opcode.get_addressing_mode());
                }
                //CLC
                0x18 => {
                    self.clc(opcode.get_addressing_mode());
                }
                //CLD
                0xd8 => {
                    self.cld(opcode.get_addressing_mode());
                }
                //CLI
                0x58 => {
                    self.cli(opcode.get_addressing_mode());
                }
                //CLV
                0xb8 => {
                    self.clv(opcode.get_addressing_mode());
                }
                //CMP
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    self.cmp(opcode.get_addressing_mode());
                }
                //CPX
                0xe0 | 0xe4 | 0xec => self.cpx(opcode.get_addressing_mode()),
                //CPY
                0xc0 | 0xc4 | 0xcc => self.cpy(opcode.get_addressing_mode()),
                //DEC
                0xc6 | 0xd6 | 0xce | 0xde => {
                    self.dec(opcode.get_addressing_mode());
                }
                //DEX
                0xca => {
                    self.dex(opcode.get_addressing_mode());
                }
                //DEY
                0x88 => {
                    self.dey(opcode.get_addressing_mode());
                }
                //EOR
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                    self.eor(opcode.get_addressing_mode());
                }
                //INC
                0xe6 | 0xf6 | 0xee | 0xfe => {
                    self.inc(opcode.get_addressing_mode());
                }
                //INX
                0xe8 => {
                    self.inx();
                }
                //INY
                0xc8 => {
                    self.iny();
                }
                //JMP
                0x4c | 0x6c => {
                    self.jmp(opcode.get_addressing_mode());
                }
                //JSR
                0x20 => {
                    self.jsr(opcode.get_addressing_mode());
                }
                //LDA
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    self.lda(opcode.get_addressing_mode());
                }
                //LDX
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => {
                    self.ldx(opcode.get_addressing_mode());
                }
                //LDY
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => {
                    self.ldy(opcode.get_addressing_mode());
                }
                //LSR
                0x4a | 0x46 | 0x56 | 0x4e | 0x5e => {
                    self.lsr(opcode.get_addressing_mode());
                }
                //NOP
                0xea => {
                    self.nop();
                }
                //ORA
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                    self.ora(opcode.get_addressing_mode());
                }
                //PHA
                0x48 => {
                    self.pha();
                }
                //PHP
                0x08 => {
                    self.php();
                }
                //PLA
                0x68 => {
                    self.pla();
                }
                //PLP
                0x28 => {
                    self.plp();
                }
                //ROL
                0x2a | 0x26 | 0x36 | 0x2e | 0x3e => {
                    self.rol(opcode.get_addressing_mode());
                }
                //ROR
                0x6a | 0x66 | 0x76 | 0x6e | 0x7e => {
                    self.ror(opcode.get_addressing_mode());
                }
                //RTI
                0x40 => {
                    self.rti();
                }
                //RTS
                0x60 => {
                    self.rts();
                }
                //SBC
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    self.sbc(opcode.get_addressing_mode());
                }
                //SEC
                0x38 => {
                    self.sec();
                }
                //SED
                0xf8 => {
                    self.sed();
                }
                //SEI
                0x78 => {
                    self.sei();
                }
                //STA
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.sta(opcode.get_addressing_mode());
                }
                //STX
                0x86 | 0x96 | 0x8e => {
                    self.stx(opcode.get_addressing_mode());
                }
                //STY
                0x84 | 0x94 | 0x8c => {
                    self.sty(opcode.get_addressing_mode());
                }
                //TAX
                0xaa => {
                    self.tax();
                }
                //TAY
                0xa8 => {
                    self.tay();
                }
                //TSX
                0xba => {
                    self.tsx();
                }
                //TXA
                0x8a => {
                    self.txa();
                }
                //TXS
                0x9a => {
                    self.txs();
                }
                //TYA
                0x98 => {
                    self.tya();
                }
                _ => {
                    panic!("opcode {:X} not found", opcode.get_code());
                }
            }
            self.program_counter += (opcode.get_bytes() - 1) as u16;
        }
    }

    pub fn adc(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);

        self.accumulator += memory_value;
        self.update_zero_and_negative_flag(self.accumulator);
    } //TODO: CARRY BIT

    fn and(&mut self, addressing_mode: &AddressingMode) {
        self.accumulator &= self.memory_read_byte(self.get_address_from(addressing_mode));
        self.update_zero_and_negative_flag(self.accumulator);
    }
    fn asl(&mut self, addressing_mode: &AddressingMode) {
        let out_shifted_bit: u8;
        match addressing_mode {
            AddressingMode::Accumulator => {
                out_shifted_bit = self.accumulator >> 7; //FIXME: READ THE ASL MANUAL
                self.accumulator <<= 1;
                self.update_zero_and_negative_flag(self.accumulator);
            }
            _ => {
                let address = self.get_address_from(addressing_mode);
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
            self.program_counter = self.get_address_from(addressing_mode);
        }
    }
    fn bcs(&mut self, addressing_mode: &AddressingMode) {
        if self.is_carry_flag_set() {
            self.program_counter = self.get_address_from(addressing_mode);
        }
    }
    fn beq(&mut self, addressing_mode: &AddressingMode) {
        if self.is_zero_flag_set() {
            self.program_counter = self.get_address_from(addressing_mode);
        }
    }
    fn bit(&mut self, addressing_mode: &AddressingMode) {
        let memory_value = self.memory_read_byte(self.get_address_from(addressing_mode));
        let and_result = self.accumulator & memory_value;
        self.set_zero_flag_to(and_result);
        let overflow_flag_from_memory = (memory_value << 1) >> 7;
        let negative_flag_from_memory = memory_value >> 7;
        self.set_negative_flag_to(negative_flag_from_memory);
        self.set_overflow_flag_to(overflow_flag_from_memory);
    }
    fn bmi(&mut self, addressing_mode: &AddressingMode) {
        if self.is_negative_flag_set() {
            let new_program_counter_address = self.get_address_from(addressing_mode);
            self.program_counter = new_program_counter_address;
        }
    }
    fn bne(&mut self, addressing_mode: &AddressingMode) {
        if !self.is_zero_flag_set() {
            let new_program_counter_address = self.get_address_from(addressing_mode);
            self.program_counter = new_program_counter_address;
        }
    }
    fn bpl(&mut self, addressing_mode: &AddressingMode) {
        if !self.is_negative_flag_set() {
            let new_program_counter_address = self.get_address_from(addressing_mode);
            self.program_counter = new_program_counter_address;
        }
    }
    fn brk(&mut self, _addressing_mode: &AddressingMode) {
        self.push_2_byte_to_stack(self.program_counter);
        self.push_byte_to_stack(self.status);
        //FIXMEself.program_counter = self.memory_read_2_byte(INTEERRUPT_VECTOR_MEMEROY_ADDRESS);
        self.set_brake_flag_to(0x1);
    }
    fn bvc(&mut self, addressing_mode: &AddressingMode) {
        if !self.is_overflow_flag_set() {
            let new_program_counter_address = self.get_address_from(addressing_mode);
            self.program_counter = new_program_counter_address;
        }
    }
    fn bvs(&mut self, addressing_mode: &AddressingMode) {
        if self.is_overflow_flag_set() {
            let new_program_counter_address = self.get_address_from(addressing_mode);
            self.program_counter = new_program_counter_address;
        }
    }
    fn clc(&mut self, _addressing_mode: &AddressingMode) {
        self.set_carry_flag_to(0x0);
    }
    fn cld(&mut self, _addressing_mode: &AddressingMode) {
        self.set_decimal_flag_to(0x0);
    }
    fn cli(&mut self, _addressing_mode: &AddressingMode) {
        self.set_interrupt_disable_flag_to(0x0);
    }
    fn clv(&mut self, _addressing_mode: &AddressingMode) {
        self.set_overflow_flag_to(0x0);
    }
    fn cmp(&mut self, addressing_mode: &AddressingMode) {
        let memory_value = self.memory_read_byte(self.get_address_from(addressing_mode));
        let result = self.accumulator - memory_value;
        if self.accumulator >= memory_value {
            self.set_carry_flag_to(0x1)
        } else if self.accumulator == memory_value {
            self.set_zero_flag_to(0x1);
        }
        if self.is_negativ_bit_present(result) {
            self.set_negative_flag_to(0x1)
        }
    }
    fn cpx(&mut self, addressing_mode: &AddressingMode) {
        let memory_value = self.memory_read_byte(self.get_address_from(addressing_mode));
        let result = self.register_x - memory_value;
        if self.register_x >= memory_value {
            self.set_carry_flag_to(0x1)
        } else if self.register_x == memory_value {
            self.set_zero_flag_to(0x1);
        }
        if self.is_negativ_bit_present(result) {
            self.set_negative_flag_to(0x1)
        }
    }
    fn cpy(&mut self, addressing_mode: &AddressingMode) {
        let memory_value = self.memory_read_byte(self.get_address_from(addressing_mode));
        let result = self.register_y - memory_value;
        if self.register_y >= memory_value {
            self.set_carry_flag_to(0x1)
        } else if self.register_y == memory_value {
            self.set_zero_flag_to(0x1);
        }
        if self.is_negativ_bit_present(result) {
            self.set_negative_flag_to(0x1)
        }
    }
    fn dec(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);
        self.memory_write_byte(address, memory_value.wrapping_sub(1));
        self.update_zero_and_negative_flag(memory_value.wrapping_sub(1));
    }
    fn dex(&mut self, _addressing_mode: &AddressingMode) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flag(self.register_x);
    }
    fn dey(&mut self, _addressing_mode: &AddressingMode) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn eor(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);
        self.accumulator ^= memory_value;
        self.update_zero_and_negative_flag(self.accumulator);
    }

    fn inc(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address).wrapping_add(1);
        self.memory_write_byte(address, memory_value);
        self.update_zero_and_negative_flag(memory_value);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flag(self.register_x);
    }
    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flag(self.register_y);
    }
    fn jmp(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let new_address = self.memory_read_2_byte(address);
        self.program_counter = new_address;
    }
    fn jsr(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let new_address = self.memory_read_2_byte(address);
        self.program_counter = new_address;
    }

    fn lda(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let value = self.memory_read_byte(address);

        self.accumulator = value;
        self.update_zero_and_negative_flag(self.accumulator);
    }
    fn ldx(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);
        self.register_x = memory_value;
        self.update_zero_and_negative_flag(self.register_x);
    }
    fn ldy(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);
        self.register_y = memory_value;
        self.update_zero_and_negative_flag(self.register_y);
    }
    fn lsr(&mut self, addressing_mode: &AddressingMode) {
        let out_shifted_bit: u8;
        match addressing_mode {
            AddressingMode::Accumulator => {
                out_shifted_bit = self.accumulator << 7;
                self.accumulator >>= 1;
                self.set_carry_flag_to(out_shifted_bit);
                self.update_zero_and_negative_flag(self.accumulator);
            }
            _ => {
                let address = self.get_address_from(addressing_mode);
                let mut memory_value = self.memory_read_byte(address);
                out_shifted_bit = memory_value << 7;
                memory_value >>= 1;
                self.memory_write_byte(address, memory_value);
                self.set_carry_flag_to(out_shifted_bit);
                self.update_zero_and_negative_flag(memory_value);
            }
        }
    }
    fn nop(&self) {
        let dont_do_anything: u8;
    }
    fn ora(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);
        self.accumulator |= memory_value;
        self.update_zero_and_negative_flag(self.accumulator);
    }
    fn pha(&mut self) {
        self.push_byte_to_stack(self.accumulator);
    }
    fn php(&mut self) {
        self.push_byte_to_stack(self.status);
    }
    fn pla(&mut self) {
        self.accumulator = self.pop_byte_from_stack();
        self.update_zero_and_negative_flag(self.accumulator);
    }
    fn plp(&mut self) {
        self.status = self.pop_byte_from_stack();
    }
    fn rol(&mut self, addressing_mode: &AddressingMode) {
        match addressing_mode {
            AddressingMode::Accumulator => {
                let outshifted_bit = self.accumulator >> 7;
                self.accumulator <<= 1;
                if self.is_carry_flag_set() {
                    self.accumulator |= CARRY_FLAG;
                }
                self.set_carry_flag_to(outshifted_bit);
                self.update_zero_and_negative_flag(self.accumulator);
            }
            _ => {
                let address = self.get_address_from(addressing_mode);
                let mut memory_value = self.memory_read_byte(address);
                let outshifted_bit = memory_value >> 7;
                memory_value <<= 1;
                if self.is_carry_flag_set() {
                    memory_value |= CARRY_FLAG;
                }
                self.memory_write_byte(address, memory_value);
                self.set_carry_flag_to(outshifted_bit);
                self.set_negative_flag_to(memory_value >> 7);
            }
        }
    }
    fn ror(&mut self, addressing_mode: &AddressingMode) {
        match addressing_mode {
            AddressingMode::Accumulator => {
                let outshifted_bit = self.accumulator << 7;
                self.accumulator >>= 1;
                if self.is_carry_flag_set() {
                    self.accumulator |= CARRY_FLAG;
                }
                self.set_carry_flag_to(outshifted_bit);
                self.update_zero_and_negative_flag(self.accumulator);
            }
            _ => {
                let address = self.get_address_from(addressing_mode);
                let mut memory_value = self.memory_read_byte(address);
                let outshifted_bit = memory_value << 7;
                memory_value >>= 1;
                if self.is_carry_flag_set() {
                    memory_value |= CARRY_FLAG;
                }
                self.memory_write_byte(address, memory_value);
                self.set_carry_flag_to(outshifted_bit);
                self.set_negative_flag_to(memory_value >> 7);
            }
        }
    }
    fn rti(&mut self) {
        self.status = self.pop_byte_from_stack();
        self.program_counter = self.pop_2_byte_from_stack();
    }
    fn rts(&mut self) {
        self.program_counter = self.pop_2_byte_from_stack() - 1;
    }
    fn sbc(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);
        if self.is_carry_flag_set() {
            self.accumulator = self
                .accumulator
                .wrapping_sub(memory_value.wrapping_sub(!CARRY_FLAG));
        } else {
            self.accumulator = self.accumulator.wrapping_sub(memory_value.wrapping_sub(!0));
        }

        //TODO: TAKE A LOOK LATER ON (I DONT FIGURED OUT HOW IT WORKS FOR NOW)
    }
    fn sec(&mut self) {
        self.set_carry_flag_to(1);
    }
    fn sed(&mut self) {
        self.set_decimal_flag_to(1);
    }
    fn sei(&mut self) {
        self.set_interrupt_disable_flag_to(1);
    }
    fn sta(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        self.memory_write_byte(address, self.accumulator);
    }
    fn stx(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);
        self.register_x = memory_value;
    }
    fn sty(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address_from(addressing_mode);
        let memory_value = self.memory_read_byte(address);
        self.register_y = memory_value;
    }
    fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.update_zero_and_negative_flag(self.register_x);
    }
    fn tay(&mut self) {
        self.register_y = self.accumulator;
        self.update_zero_and_negative_flag(self.register_y);
    }
    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flag(self.register_x);
    }
    fn txa(&mut self) {
        self.accumulator = self.register_x;
        self.update_zero_and_negative_flag(self.accumulator);
    }
    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }
    fn tya(&mut self) {
        self.accumulator = self.register_y;
        self.update_zero_and_negative_flag(self.accumulator);
    }

    fn update_zero_and_negative_flag(&mut self, byte_to_check: u8) {
        self.set_zero_flag_to((byte_to_check == ZERO_RESULT) as u8);

        self.set_negative_flag_to(self.is_negativ_bit_present(byte_to_check) as u8);
    }

    fn set_zero_flag_to(&mut self, zero_bit: u8) {
        if zero_bit == 0x1 {
            self.status |= ZERO_FLAG;
        } else {
            self.status &= !ZERO_FLAG;
        }
    }
    fn set_carry_flag_to(&mut self, carry_bit: u8) {
        if carry_bit == 0x1 {
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
    fn set_negative_flag_to(&mut self, negative_bit: u8) {
        if negative_bit == 0x1 {
            self.status |= NEGATIVE_FLAG;
        } else {
            self.status &= !NEGATIVE_FLAG;
        }
    }

    fn set_brake_flag_to(&mut self, break_bit: u8) {
        if break_bit == 1 {
            self.status |= BREAK_FLAG;
        } else {
            self.status &= !BREAK_FLAG;
        }
    }

    fn set_decimal_flag_to(&mut self, decimal_bit: u8) {
        if decimal_bit == 1 {
            self.status |= DECIMAL_FLAG;
        } else {
            self.status &= !DECIMAL_FLAG;
        }
    }

    fn is_overflow_flag_set(&self) -> bool {
        let overflow_bit = (self.status << 1) >> 7;
        overflow_bit == 1
    }

    fn set_interrupt_disable_flag_to(&mut self, interrupt_disable_bit: u8) {
        if interrupt_disable_bit == 1 {
            self.status |= INTERRUPT_DISABLE_FLAG;
        } else {
            self.status &= !INTERRUPT_DISABLE_FLAG;
        }
    }

    fn is_negativ_bit_present(&self, byte_to_check: u8) -> bool {
        byte_to_check >> 7 == 1
    }
    fn is_zero_flag_present(&self, byte_to_check: u8) -> bool {
        (byte_to_check >> 1) << 7 == 1
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
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}
