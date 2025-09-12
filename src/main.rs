pub mod emulator;
pub mod mmu;


fn main() {
    let mut emu = emulator::Interpreter::new();
    emu.execute_instruction(0b11111111000000010000000100010011u32);
}
