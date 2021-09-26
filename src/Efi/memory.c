#include "efi.h"
#include "types.h"



int PrintMemoryMap() 
{
    UINTN MemoryMapSize;
    EFI_MEMORY_DESCRIPTOR MemoryMap;
    UINTN MapKey;
    UINTN DescriptorSize;
    u32 DescriptorVer;

    (*ST->BootServices->GetMemoryMap)(&MemoryMapSize, &MemoryMap,
                                    &MapKey, &DescriptorSize,
                                    &DescriptorVer);
    UINTN total_entries = MemoryMapSize / DescriptorSize;
    
    // Iterate through each entry and print them all
    // TODO
    return total_entries;
}
