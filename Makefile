ARCH := x86_64

OS_NAME = walnut

BUILD_DIR = $(abspath build)
SOURCE_DIR = $(abspath src)
INCLUDE_DIR = $(abspath include)
GNU_EFI_DIR = $(INCLUDE_DIR)/gnu-efi
GNU_EFI_INCLUDE_DIR = $(GNU_EFI_DIR)/inc

SRCFILES := $(shell find $(SOURCE_DIR) -type f -name "*.c")
HDRFILES = $(shell find $(SOURCE_DIR) -name '*.h') $(shell find $(LIBC_DIR) -name '*.h')
OBJFILES = $(patsubst $(SOURCE_DIR)%.c, $(BUILD_DIR)%.o, $(SRCFILES)) 



CC := clang
EMU = qemu-system-x86_64
DBG = gdb
AS = nasm
LD = clang

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
CXXSTD := c++20


GNUEFIPATH := include/gnu-efi

CFLAGS := -target x86_64-unknown-windows \
		-ffreestanding \
		-fshort-wchar \
		-mno-red-zone \
		-I$(GNUEFIPATH)/inc -I$(GNUEFIPATH)/inc/x86_64 -I$(GNUEFIPATH)/inc/protocol \
		-I$(INCLUDE_DIR)

LDFLAGS := -target x86_64-unknown-windows \
	-nostdlib \
	-Wl,-entry:efi_main \
	-Wl,-subsystem:efi_application \
	-fuse-ld=lld-link

.PHONY: all clean

all: run

build: $(OBJFILES)
	@$(LD) $(LDFLAGS) -o $(BUILD_DIR)/BOOTX64.EFI $(OBJFILES)

diskimg:
	dd if=/dev/zero of=$(BUILD_DIR)/fat.img bs=1k count=1440
	mformat -i $(BUILD_DIR)/fat.img -f 1440 ::
	mmd -i $(BUILD_DIR)/fat.img ::/EFI
	mmd -i $(BUILD_DIR)/fat.img ::/EFI/BOOT
	mcopy -i $(BUILD_DIR)/fat.img $(BUILD_DIR)/BOOTX64.EFI ::/EFI/BOOT

run: build diskimg
	qemu-system-x86_64 $(EMUFLAGS)

debug: build diskimg
	$(EMU) $(EMUFLAGS) $(EMU_DBG_FLAGS) &
	$(DBG) $(DBG_FLAGS)

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
	@$(CC) -std=$(CSTD) $(CFLAGS)  -c $< -o $@