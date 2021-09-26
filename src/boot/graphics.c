#include <efi.h>
#include <efilib.h>
#include "print.h"
#include "types.h"
#include "boot.h"



frame_buffer_t* init_gop()
{
    frame_buffer_t framebuffer;
    EFI_GUID gop_guid = EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID; 
    EFI_GRAPHICS_OUTPUT_PROTOCOL* gop;
    
    EFI_STATUS status = uefi_call_wrapper(BS->LocateProtocol, 3, &gop_guid, NULL, (void**)&gop);
    if(EFI_ERROR(status)) {
        efi_println(L"Unable to locate GOP");
        return;
    }

    framebuffer.base_addr = (void*)gop->Mode->FrameBufferBase;
    framebuffer.buf_size = gop->Mode->FrameBufferSize;
    framebuffer.width = gop->Mode->Info->HorizontalResolution;
    framebuffer.height = gop->Mode->Info->VerticalResolution;
    framebuffer.pixels_per_scanline = gop->Mode->Info->PixelsPerScanLine;

    return &framebuffer;
}