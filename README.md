Steps to create
===============

To be written: Actual README that explains how to build

1. cargo new --bin
2. no_std, no_main
3. macro_use + crate embedded
4. run board! macro
5. linker script (layout.ld)
6. .cargo/config + xargo build
7. panic = "abort"

Now we have a binary (in ELF format).

8. arm-none-eabi-objcopy -Obinary target/thumbv7em-none-eabi/release/hello-embed-rs output.bin
9. Upload
