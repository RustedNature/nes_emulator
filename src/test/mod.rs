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
fn test_relative_addressing_mode() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xd0]);
    //panic!("Not implemented yet")
}
