## Snapshot fuzzer powered by a riscv emulator to allow cross-core scaling



#### Build the riscv toolchain:

`./configure --prefix=$HOME/.local/share/riscv \
            --with-arch=rv64i \
            --with-abi=lp64 \
            --disable-multilib`

#### Compile riscv 64i

`riscv64-unknown-elf -gcc -march=rv64i -mabi=lp64 -static <file>`
