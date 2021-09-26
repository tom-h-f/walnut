#include "types.h"
#include "efi.h"

err_t init_gop(void)
{
    EFI_GRAPHICS_OUTPUT_PROTOCOL* gop;
    EFI_GUID gop_guid = EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
    EFI_STATUS status;

    efi_print(L"Locating GOP...\n\r");
    return SUCCESS;
    status = (*ST->BootServices->LocateProtocol)(&gop_guid, NULL, (void**)&gop);
    if(EFI_ERROR(status)) {
        efi_print(L"Unable to Locate GOP\n\r");
        return COULDNT_FIND;
    }
    efi_print(L"Located GOP\n\r");

    return SUCCESS;
}