#include <efi.h>
#include <efilib.h>

extern void kmain(void);

EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable)
{
    InitializeLib(ImageHandle, SystemTable);
    Print(L"Welcome to Walnut OS!\n\r");


  
    kmain();

    return EFI_FAILURE;
}
