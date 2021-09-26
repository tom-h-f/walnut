ARCH := x86_64

OS_NAME = walnut

BUILD_DIR = $(abspath Build)
SOURCE_DIR = $(abspath Source)
INCLUDE_DIR = $(abspath Include)

SRCFILES := $(shell find $(SOURCE_DIR) -type f -name "*.c")
HDRFILES = $(shell find $(SOURCE_DIR) -name '*.h') $(shell find $(LIBC_DIR) -name '*.h')
OBJFILES = $(patsubst $(SOURCE_DIR)%.c, $(BUILD_DIR)%.o, $(SRCFILES))

BUILD_DIR := Build


CC := gcc
CXX := g++
EMU = qemu-system-x86_64
DBG = gdb
AS = nasm
LD = ld

EMUFLAGS := -machine q35 -m 256 -smp 4 -net none \
    -global driver=cfi.pflash01,property=secure,value=on \
    -drive if=pflash,format=raw,unit=0,file=/usr/share/ovmf/OVMF.fd,readonly=on \
    -drive if=ide,format=raw,file=$(BUILD_DIR)/fat.img \
	-nographic

EMU_DBG_FLAGS = -s -d guest_errors,cpu_reset,int -no-reboot -no-shutdown

DBG_FLAGS = -ex "target remote localhost:1234" \
			-ex "symbol-file $(BUILD_DIR)/kernel_x64.elf" \
			-ex "set disassemble-next-line on" \
			-ex "set step-mode on"


CSTD := c17

CFLAGS := -ffreestanding -fpic -fno-stack-protector -fshort-wchar -mno-red-zone -mgeneral-regs-only -mabi=ms -Wall -Wextra -Wpedantic -O3 
CXXFLAGS := -ffreestanding -fpic -fno-stack-protector -fshort-wchar -mno-red-zone -mgeneral-regs-only -mabi=ms -Wall -Wextra -Wpedantic -O3  

LDFLAGS := -nostdlib -shared -T x86_64.lds -Bsymbolic -znocombreloc 


.PHONY: all clean

all: run 

build: $(OBJFILES)
	@$(LD) $(LDFLAGS) -o $(BUILD_DIR)/kernel_x64.elf $(OBJFILES)
	
to_efi_image: build
	@objcopy -I elf64-x86-64 -O efi-app-x86_64 $(BUILD_DIR)/kernel_x64.elf $(BUILD_DIR)/BOOTX64.EFI

# Enable pressing CTRL+C to close QEMU
run: disk_image
	@$(EMU) $(EMUFLAGS) 


debug: disk_image
	$(EMU) $(EMUFLAGS) $(EMU_DBG_FLAGS) &
	$(DBG) $(DBG_FLAGS)


disk_image: to_efi_image
	@dd if=/dev/zero of=$(BUILD_DIR)/fat.img bs=1k count=1440
	@mformat -i $(BUILD_DIR)/fat.img -f 1440 ::
	@mmd -i $(BUILD_DIR)/fat.img ::/EFI
	@mmd -i $(BUILD_DIR)/fat.img ::/EFI/BOOT
	@mcopy -i $(BUILD_DIR)/fat.img $(BUILD_DIR)/BOOTX64.EFI ::/EFI/BOOT

clean:
	@find $(SOURCE_DIR) -name "*.o" -type f -delete
	@find $(BUILD_DIR) -name "*.o" -type f -delete
	@find $(BUILD_DIR) -name "*.so" -type f -delete
	@find $(BUILD_DIR) -name "*.efi" -type f -delete
	@find $(BUILD_DIR) -name "*.efi.debug" -type f -delete
	@find $(BUILD_DIR) -name "*.elf" -type f -delete
	@$(RM) -rf $(BUILD_DIR)/*

$(BUILD_DIR)/%.o: $(SOURCE_DIR)/%.c $(HDRFILES)
	@mkdir -p $(dir $@)
	@$(CC) -std=$(CSTD) $(CFLAGS) -I$(INCLUDE_DIR)  -c $< -o $@
