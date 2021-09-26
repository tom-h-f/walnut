#include "efi.h"

#define EFI_BOOT_SERVICES_SIGNATURE 0x56524553544f4f42


typedef struct {
    EFI_TABLE_HEADER Hdr;
    void*            RaiseTPL;
    void*            RestoreTPL;
    void*            AllocatePages;
    void*            FreePages;
    EFI_GET_MEMORY_MAP            *GetMemoryMap;
} EFI_BOOT_SERVICES;