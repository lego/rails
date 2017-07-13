# Rails, a realtime operating system

Inspired from CS 452 at the University of Waterloo.

This kernel will be primarily developed on the Raspberry Pi 2, but I will also attempt to have basic support for other devices that I can get my hands on, including the CS 452's TS7200.

## Testing
You will need an ARM compiler to test this. You can get the latest version from [ARM](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm).

In `Makefile` you'll need to specify the GCC root to point to it.

In addition you can emulate the RPI2 hardware and test the kernel locally using `qemu-system-arm`, using `make kernel.elf && ./test`.
