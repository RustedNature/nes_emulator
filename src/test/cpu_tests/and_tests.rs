use crate::cpu::*;

#[test]
fn test_immediate_and() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x29, 0b1111_1111, 0x00]);
    assert_eq!(cpu.accumulator, 0x00);
    assert_eq!(cpu.status, ZERO_FLAG);
}
#[test]
fn test_immediate_lda_immediate_and() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0b1001_1111, 0x29, 0b1111_1111, 0x00]);
    assert_eq!(cpu.accumulator, 0b1001_1111);
    assert_eq!(cpu.status, NEGATIVE_FLAG);
}
#[test]
fn test_zeropage_lda_zeropage_and_immediate() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x55, 0b1001_1111);
    cpu.load_and_run(vec![0xa5, 0x55, 0x29, 0b0000_1010, 0x00]);
    assert_eq!(cpu.accumulator, 0b0000_1010);
}
#[test]
fn test_zeropage_lda_zeropage_and_zeropage() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x55, 0b1001_1111);
    cpu.mem_write(0xff, 0b1001_1011);
    cpu.load_and_run(vec![0xa5, 0x55, 0x25, 0xff, 0x00]);
    assert_eq!(cpu.accumulator, 0b1001_1011);
}

