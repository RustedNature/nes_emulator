use crate::cpu::*;
#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xaa, 0x00]);

    assert_eq!(cpu.register_x, 0)
}
