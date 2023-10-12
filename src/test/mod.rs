use super::*;
#[cfg(test)]
mod cpu_tests;

#[test]
fn test_5_ops_working_together() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_relative_subtract_addressing_mode() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0x90,0b1001_1010, 0x00]);
    print!("{}",cpu.program_counter);
    assert_eq!(cpu.program_counter, (0x8003 - 0b0001_1010));
    
}
#[test]
fn test_relative_add_addressing_mode() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0x90,0b0001_1010, 0x00]);
    print!("{}",cpu.program_counter);
    assert_eq!(cpu.program_counter, (0x8003 + 0b0001_1010));
    
}