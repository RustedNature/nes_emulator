use crate::cpu::*;

#[test]
fn asl_accumulator() {
    let mut cpu = CPU::default();
    cpu.load_and_run(vec![0xa9, 0b0000_1111, 0x0a, 0x00]);
    assert_eq!(cpu.accumulator, 0b0001_1110);
    assert_ne!(cpu.status, NEGATIVE_FLAG);
    assert_ne!(cpu.status, ZERO_FLAG);
}
