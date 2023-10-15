pub mod cpu;
pub mod opcode;
pub mod test;

#[macro_use]
extern crate lazy_static;
fn main() {
    let cpu = cpu::CPU::default();

    //cpu.load_and_run(program.load());
}
