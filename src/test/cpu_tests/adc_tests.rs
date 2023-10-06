use crate::cpu::*;
#[test]
fn test_adc_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x69, 0x05, 0x69, 0x05, 0x00]);
    assert_eq!(cpu.accumulator, 0x0a);
}
