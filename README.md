Steps to create
===============

1. cargo new --bin
2. no_std
3. macro_use + crate embedded
4. run board! macro
5. UNDECIDED: panic_fmt? (now in lib)
6. FIXME (ask phil): copy over .cargo/config
7. linker script (layout.ld)
8. xargo build

Now we have a binary (in ELF format).

9. arm-none-eabi-objcopy -Obinary target/thumbv7em-none-eabi/release/hello-embed-rs output.bin
10. Upload
