#include "efi.h"



int PrintMemoryMap() 
{
    u64 MemoryMapSize;
    EFI_MEMORY_DESCRIPTOR MemoryMap;
    u64 MapKey;
    u64 DescriptorSize;
    u32 DescriptorVer;

    (*ST->BootServices->GetMemoryMap)(&MemoryMapSize, &MemoryMap,
                                    MapKey, DescriptorSize,
                                    DescriptorVer);
    u32 total_entries = MemoryMapSize / DescriptorSize;
    
    // Iterate through each entry and print them all
    // TODO
}
