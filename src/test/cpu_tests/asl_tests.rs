use crate::cpu::*;

#[test]
fn asl_accumulator() {
    let mut cpu = CPU::default();
    cpu.load_and_run(vec![0xa9, 0b0000_1111, 0x0a, 0x00]);
    assert_eq!(cpu.accumulator, 0b0001_1110);
    assert_ne!(cpu.status, NEGATIVE_FLAG);
    assert_ne!(cpu.status, ZERO_FLAG);
    assert_ne!(cpu.status, CARRY_FLAG);
}
#[test]
fn asl_set_carry_flag() {
    let mut cpu = CPU::default();
    cpu.load_and_run(vec![0xa9, 0b1000_1111, 0x0a, 0x00]);
    assert_eq!(cpu.accumulator, 0b0001_1110);
    assert_eq!(cpu.status, CARRY_FLAG);
}
#[test]
fn asl_set_zero_flag() {
    let mut cpu = CPU::default();
    cpu.load_and_run(vec![0xa9, 0b0000_0000, 0x0a, 0x00]);
    assert_eq!(cpu.accumulator, 0b0000_0000);
    assert_eq!(cpu.status, ZERO_FLAG);
}
#[test]
fn asl_set_negative_flag() {
    let mut cpu = CPU::default();
    cpu.load_and_run(vec![0xa9, 0b0100_1111, 0x0a, 0x00]);
    assert_eq!(cpu.accumulator, 0b1001_1110);
    assert_eq!(cpu.status, NEGATIVE_FLAG);
}
