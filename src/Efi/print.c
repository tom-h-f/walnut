#include "types.h"


// Print routines for 16-bit characters

EFI_STATUS efi_print(c16* out_string)
{
    return ST->ConOut->OutputString(ST->ConOut, out_string);
}



// Print routines for 8-bit characters
// TODO


// Console In Routines

EFI_STATUS efi_flush_cin() 
{
	// Empty the console input buffer to flush out any keystrokes entered before this point.
	return ST->ConIn->Reset(ST->ConIn, false);
}

EFI_STATUS efi_flush_cout() 
{
	// Empty the console input buffer to flush out any keystrokes entered before this point.
	return ST->ConIn->Reset(ST->ConOut, false);
}
