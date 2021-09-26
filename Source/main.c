#include "efi.h"
#include "macros.h"


EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable)
{
	// Assign Global System Table from received pointer
	ST = SystemTable;
	UNUSED(ImageHandle);

 
	// Print welcome message
	EFI_STATUS Status = efi_wprint(L"Welcome to Walnut");
	if(EFI_ERROR(Status))
		return Status;
 
	// Empty the console input buffer to flush out any keystrokes entered before this point.
	Status = efi_flush_cin();
	if(EFI_ERROR(Status))
		return Status;
 
	EFI_INPUT_KEY Key;
	// Wait for keypress. 
	while((Status = SystemTable->ConIn->ReadKeyStroke(SystemTable->ConIn, &Key)) == EFI_NOT_READY) ;
 
	return Status;
}
