pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            register_x: 0,
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
                    let param= program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);

                    
                }
                0xAA => {
                    self.tax();
                },
                0x00 => {
                    break;
                }
                0xE8 => {
                    self.inx();
                }
                _ => todo!(),
            }
        }
    }

    fn lda(&mut self, param: u8) {
        
        self.register_a = param;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn update_zero_and_negative_flag(&mut self, register: u8) {
        if register == 0{
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if self.register_x & 0b1000_0000 != 0{
            self.status |= 0b1000_0000;

        }else {
            self.status &= 0b0111_1111;
        }
    }

    fn inx(&mut self) {
        if self.register_x == 0xff{
          self.register_x = 0x00;   
        }
        else {
            self.register_x += 1;
        }
        
        self.update_zero_and_negative_flag(self.register_x);
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
    #[test]
   fn test_0xaa_tax_move_a_to_x() {
       let mut cpu = CPU::new();
       cpu.register_a = 10;
       cpu.interpret(vec![0xaa, 0x00]);
 
       assert_eq!(cpu.register_x, 10)
   }
   #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
