#include <efi.h>
#include <efilib.h>
#include "libc/string.h"
#include "boot.h"
#include "macros.h"

EFI_SYSTEM_TABLE* ST;
EFI_BOOT_SERVICES* BS;

 
EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    EFI_STATUS Status;
    EFI_INPUT_KEY Key;
 
    // Store the system table for future use in other functions
    ST = SystemTable;
    BS = ST->BootServices;
 
    ERRCHK(efi_println(L"Welcome to Walnut.\nBooting..."));

 
    /* Now wait for a keystroke before continuing, otherwise your
       message will flash off the screen before you see it.
 
       First, we need to empty the console input buffer to flush
       out any keystrokes entered before this point */
    ERRCHK(efi_flush_cin());
    


    frame_buffer_t *fb = init_gop();
 
    exit_qemu();
    return EFI_SUCCESS;
}
