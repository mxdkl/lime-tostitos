pub mod emulator;
pub mod mmu;


fn main() {
    let emu = emulator::Interpreter::new();
    println!("{:?}", emu);
}
