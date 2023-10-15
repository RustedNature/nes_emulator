use crate::cpu::*;
#[test]
fn test_0xa9_lda_immediate_load_state() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.accumulator, 0x05);
    assert!((cpu.status & 0b0000_0010) == 0b00);
    assert!((cpu.status & 0b1000_0000) == 0);
}
#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!((cpu.status & 0b0000_0010) == 0b10);
}

#[test]
fn test_0xa5_lda_zero_page() {
    let mut cpu = CPU::new();
    cpu.memory_write_byte(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.accumulator, 0x55);
}
