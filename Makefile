GCC_ROOT    := /Users/joey/.gcc-arm-6.3.1
GCC_TYPE    := arm-none-eabi
GCC_VERSION := 6.3.1

# architecture
ARCH    := ARM
# machine (more specific)
MACH    := RPI2
CC      := $(GCC_ROOT)/bin/$(GCC_TYPE)-gcc
AS      := $(GCC_ROOT)/bin/$(GCC_TYPE)-as
AR      := $(GCC_ROOT)/bin/$(GCC_TYPE)-ar
LD      := $(GCC_ROOT)/bin/$(GCC_TYPE)-ld
OBJCOPY := $(GCC_ROOT)/bin/$(GCC_TYPE)-objcopy

DEBUG   := # -g
CFLAGS  := $(DEBUG) -fpic -ffreestanding -nostdlib -std=gnu99 -O2 -Wall -Wextra -Isrc -include stddef.h -include stdint.h
LFLAGS  := -ffreestanding -O2 -nostdlib



SRC_DIR := src
# All source files architecture or machine agnostic
SRCS    := $(shell find $(SRC_DIR) -path "*/arch/*/" -prune -o -path "*/mach/*/" -prune -o \( -name "*.c" -o -name "*.s" \) -print)
# Add architecture specific files
SRCS    := $(SRCS) $(shell find $(SRC_DIR) -ipath "*/arch/$(ARCH)/*" \( -name "*.c" -o -name "*.s" \) -print)
# Add machine specific files
SRCS    := $(SRCS) $(shell find $(SRC_DIR) -ipath "*/mach/$(MACH)/*" \( -name "*.c" -o -name "*.s" \) -print)

OBJS    := $(SRCS:%.c=build/%.o)
OBJS    := $(OBJS:%.s=build/%.o)

RUST_SRCS    := $(shell find $(SRC_DIR) -name "*.rs" -print)


# Raspberry Pi
ifeq ($(ARCH), ARM)
ifeq ($(MACH), RPI2)
LFLAGS += -mcpu=arm1176jzf-s -mfpu=neon-vfpv4 -mfloat-abi=hard -T ld/rpi2.ld -L$(GCC_ROOT)/lib/gcc/$(GCC_TYPE)/$(GCC_VERSION) -lgcc
CFLAGS += -mcpu=arm1176jzf-s -mfpu=neon-vfpv4 -mfloat-abi=hard
MACH_CPU := arm1176jzf-s
RUST_TARGET := rpi2


kernel.elf: $(OBJS) target/$(RUST_TARGET)/debug/librails.rlib ld/rpi2.ld
	$(CC) $(LFLAGS) $^ -o $@

kernel.bin: kernel.elf
	$(OBJCOPY) $< -O binary $@

default: kernel.elf
all: kernel.bin
endif
endif

target/$(RUST_TARGET)/debug/librails.rlib: $(RUST_SRCS) Xargo.toml Cargo.toml .cargo/config
	CC_$(RUST_TARGET)="$(GCC_TYPE)-gcc" CFLAGS_$(RUST_TARGET)="-mcpu=$(MACH_CPU)" xargo build --target=$(RUST_TARGET) -j4 --verbose

target/$(RUST_TARGET)/release/librails.rlib: $(RUST_SRCS) Xargo.toml Cargo.toml .cargo/config
	CC_$(RUST_TARGET)="$(GCC_TYPE)-gcc" CFLAGS_$(RUST_TARGET)="-mcpu=$(MACH_CPU)" xargo build --release --target=$(RUST_TARGET) -j4

build/%.o: %.c
	@mkdir -p ${@D}
	$(CC) $(CFLAGS) -c $< -o $@

build/%.o: %.s
	@mkdir -p ${@D}
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	@rm -rf build/ target/
	@rm -f *.i *.s *.bin *.elf *.o

.PHONY: clean
