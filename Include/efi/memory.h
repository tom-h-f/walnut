#include "types.h"


typedef void* EFI_PHYSICAL_ADDRESS;
typedef void* EFI_VIRTUAL_ADDRESS;


typedef struct {
    u32 Type;
    EFI_PHYSICAL_ADDRESS PhysicalStart;
    EFI_VIRTUAL_ADDRESS VirtualStart;
    u64 NumberOfPages;
    u64 Attribute;
} EFI_MEMORY_DESCRIPTOR;

typedef
EFI_STATUS
(*EFI_GET_MEMORY_MAP) (
    u64 *MemoryMapSize,
    EFI_MEMORY_DESCRIPTOR *MemoryMap,
    u64 *MapKey,
    u64 *DescriptorSize,
    u32 *DescriptorVersion
);
