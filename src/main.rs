pub mod emulator;
pub mod mmu;


fn main() {
    let mut emu = emulator::Interpreter::new();

    // Some tests
    emu.execute_instruction(0b01000101010100000000000011101111u32);
    emu.execute_instruction(0b00000000000001000000010100010011u32);
    emu.execute_instruction(0b00000000000001111000010001100011u32);
    emu.execute_instruction(0b11111111000000010000000100010011u32);

}
