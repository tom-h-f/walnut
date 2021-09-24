ARCH := x86_64

OBJFILES := $(shell find -type f -name "*.o")

BUILD_DIR := Build


CC := gcc

CFLAGS := -ffreestanding -fpic -fno-stack-protector -fshort-wchar -mno-red-zone -mgeneral-regs-only -mabi=ms -Wall -Wextra -Wpedantic -O3 

LDFLAGS := -nostdlib -shared -Wl,-T,x86_64.lds -Wl,-Bsymbolic -Wl,-znocombreloc 

.PHONY: all clean


all: run 

build:
	$(CC) $(CFLAGS) -c -o $(BUILD_DIR)/main.o Source/main.c
	$(CC) $(LDFLAGS) -o $(BUILD_DIR)/kernel_x64.elf $(BUILD_DIR)/main.o -lgcc
	
to_efi_image: build
	objcopy -I elf64-x86-64 -O efi-app-x86_64 $(BUILD_DIR)/kernel_x64.elf $(BUILD_DIR)/BOOTX64.EFI

run: disk_image
	qemu-system-x86_64 -machine q35 -m 256 -smp 4 -net none \
    -global driver=cfi.pflash01,property=secure,value=on \
    -drive if=pflash,format=raw,unit=0,file=/usr/share/ovmf/OVMF.fd,readonly=on \
    -drive if=ide,format=raw,file=$(BUILD_DIR)/fat.img

clean:
	$(RM) $(shell find -type f -name "*.o")

disk_image: to_efi_image
	dd if=/dev/zero of=$(BUILD_DIR)/fat.img bs=1k count=1440
	mformat -i $(BUILD_DIR)/fat.img -f 1440 ::
	mmd -i $(BUILD_DIR)/fat.img ::/EFI
	mmd -i $(BUILD_DIR)/fat.img ::/EFI/BOOT
	mcopy -i $(BUILD_DIR)/fat.img $(BUILD_DIR)/BOOTX64.EFI ::/EFI/BOOT
