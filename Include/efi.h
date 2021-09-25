#include "types.h"

#pragma once
#include "efi/print.h"
#include "efi/types.h"
#include "efi/error.h"
//#include "efi/table.h"

 
#include <stdint.h>
#include <stdbool.h>
#include <wchar.h>
 
#ifndef EFI_H
#define EFI_H

 
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
 
#endif



// The EFI System Table, defined upon entry to `efi_main`
extern EFI_SYSTEM_TABLE* ST;

