pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = param;

                    if self.register_a == 0 {
                        self.status |= 0b0000_0010;
                    } else {
                        self.status &= 0b1111_1101;
                    }
                }
                0x00 => {
                    break;
                }
                _ => todo!(),
            }
        }
    }
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_state() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
        
    }#[test]
    fn test_0xa9_lad_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9,0x00,0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}
