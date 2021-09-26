#include "efi.h"
#include "macros.h"
#include "kernel.h"
#include "types.h"

EFI_SYSTEM_TABLE* ST;

EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable)
{
	// Assign Global System Table from received pointer
	ST = SystemTable;
	UNUSED(ImageHandle);


	// Empty console out so we have only our output

	init_gop();
 
	kmain();
 
	return 0;
}
