ARCH := x86_64
OBJFILES := $(shell find -type f -name "*.o")


CC := gcc

CFLAGS :=-ffreestanding -fpic -fno-stack-protector -fshort-wchar -mno-red-zone -mgeneral-regs-only -mabi=ms -Wall -Wextra -Wpedantic -O3 

LDFLAGS :=-nostdlib -shared -Wl,-T,x86_64.lds -Wl,-Bsymbolic -Wl,-znocombreloc 


.PHONY: all clean


all: run 

build:
	$(CC) $(CFLAGS) -c -o main.o Source/main.c
	$(CC) $(LDFLAGS) -o kernel_x64.elf main.o -lgcc
	
to_efi_image: build
	objcopy -I elf64-x86-64 -O efi-app-x86_64 kernel_x64.elf BOOTX64.EFI

run: disk_image
	qemu-system-x86_64 -machine q35 -m 256 -smp 2 -net none \
    -global driver=cfi.pflash01,property=secure,value=on \
    -drive if=pflash,format=raw,unit=0,file=/usr/share/ovmf/OVMF.fd,readonly=on \
    -drive if=ide,format=raw,file=fat.img


clean:
	$(RM) $(shell find -type f -name "*.o")

disk_image: to_efi_image
	dd if=/dev/zero of=fat.img bs=1k count=1440
	mformat -i fat.img -f 1440 ::
	mmd -i fat.img ::/EFI
	mmd -i fat.img ::/EFI/BOOT
	mcopy -i fat.img BOOTX64.EFI ::/EFI/BOOT
