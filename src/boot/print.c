#include "print.h"
#include "types.h"

#include "libc/string.h"

// Prints out 'out_string' along with a CRLF to EFI ConsoleOut
EFI_STATUS efi_println(c16* out_string)
{
    return ST->ConOut->OutputString(ST->ConOut, strcat(out_string, L"\n\r"));
}

EFI_STATUS efi_snprintf(unsigned int buffer_len, const c16 *fmt, ...)
{
    c16 buffer[buffer_len];
	EFI_STATUS ret;
	__builtin_va_list va;
    __builtin_va_start(va, fmt);
    snprintf((c16*)&buffer, buffer_len, fmt, va);
    ret = ST->ConOut->OutputString(ST->ConOut, buffer);
	__builtin_va_end(va);
    return ret;
}

EFI_STATUS efi_flush_cin()
{
    return ST->ConIn->Reset(ST->ConIn, FALSE);
}

EFI_STATUS efi_flush_cout()
{
    return ST->ConOut->Reset(ST->ConOut, FALSE);
}



