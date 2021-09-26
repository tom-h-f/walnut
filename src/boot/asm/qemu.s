section .text

global exit_qemu

exit_qemu:
    mov dx, 0xf4 ; QEMU IO Port
    mov ax, 0x10 ; Exit Code
    out dx, ax