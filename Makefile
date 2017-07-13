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
CFLAGS  := $(DEBUG) -fpic -ffreestanding -std=gnu99 -O2 -Wall -Wextra -Isrc -include stddef.h -include stdint.h



SRC_DIR := src
# All source files architecture or machine agnostic
SRCS    := $(shell find $(SRC_DIR) -path "*/arch" -prune -o -path "*/mach" -prune -o \( -name "*.c" -o -name "*.s" \) -print)
# Add architecture specific files
SRCS    := $(SRCS) $(shell find $(SRC_DIR) -ipath "*/arch/$(ARCH)/*" \( -name "*.c" -o -name "*.s" \) -print)
# Add machine specific files
SRCS    := $(SRCS) $(shell find $(SRC_DIR) -ipath "*/mach/$(MACH)/*" \( -name "*.c" -o -name "*.s" \) -print)


OBJS    := $(SRCS:%.c=build/%.o)
OBJS    := $(OBJS:%.s=build/%.o)

build/%.o: %.c
	@mkdir -p ${@D}
	$(CC) $(CFLAGS) -c $< -o $@

build/%.o: %.s
	@mkdir -p ${@D}
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	@rm -rf build/

.PHONY: clean

# Raspberry Pi
ifeq ($(ARCH), ARM)
ifeq ($(MACH), RPI2)
LFLAGS := -T rpi2.ld -ffreestanding -O2 -nostdlib
CFLAGS += -mcpu=arm1176jzf-s

all: kernel.elf

kernel.elf: $(OBJS)
	$(CC) $(LFLAGS) $^ -o $@

kernel.bin: kernel.elf
	$(OBJCOPY) $< -O binary $@
endif
endif
