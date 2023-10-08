use crate::cpu::*;

#[test]
fn test_bcc_positve_offset() {
    let mut cpu = CPU::default();
    cpu.load_and_run(vec![0x90, 0x05]);
    assert_eq!(cpu.program_counter, 0x8008)
}
#[test]
fn test_bcc_negative_offset() {
    let mut cpu = CPU::default();
    cpu.load_and_run(vec![0x90, 0x05]);
    assert_eq!(cpu.program_counter, 0x8008)
}
