mod mmu;


fn main() {
    let mem = mmu::Memory::new(100);
    println!("{:?}", mem);
}
