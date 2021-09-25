#include "efi/types.h"
#include "efi/print.h"
 
typedef struct {
	u64 Signature;
	u32 Revision;
	u32 HeaderSize;
	u32 CRC32;
	u32 Reserved;
} EFI_TABLE_HEADER;
 
typedef struct {
	EFI_TABLE_HEADER                Hdr;
	EFI_PVOID                       FirmwareVendor;
	u32                        FirmwareRevision;
	EFI_PVOID                       ConsoleInHandle;
	EFI_SIMPLE_TEXT_INPUT_PROTOCOL  *ConIn;
	EFI_PVOID                       ConsoleOutHandle;
	EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *ConOut;
	EFI_PVOID                       StandardErrorHandle;
	EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *StdErr;
	void                            *RuntimeServices;
	void                            *BootServices;
	UINTN                           NumberOfTableEntries;
	void                            *ConfigurationTable;
} EFI_SYSTEM_TABLE;
 
