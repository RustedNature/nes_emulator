use super::*;

#[cfg(test)]
#[test]
fn test_0xa9_lda_immediate_load_state() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!((cpu.status & 0b0000_0010) == 0b00);
    assert!((cpu.status & 0b1000_0000) == 0);
}
#[test]
fn test_0xa9_lad_zero_flag() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!((cpu.status & 0b0000_0010) == 0b10);
}

#[test]
fn test_lda_from_memory() {
    let mut cpu = cpu::CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}
#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xaa, 0x00]);

    assert_eq!(cpu.register_x, 0)
}
#[test]
fn test_5_ops_working_together() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(
        vec![
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0xe8,
            0x00
        ]
    );

    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_relative_addressing_mode() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xd0]);
    panic!("Not implemented yet")
}

#[test]
fn test_adc() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0x69, 0x05, 0x69, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x0a);
}
