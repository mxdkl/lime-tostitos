pub mod emulator;
pub mod mmu;


fn main() {
    let emu = emulator::Interpreter::new();
    emu.instruction_decode(0b11111111000000010000000100010011u32);
}
