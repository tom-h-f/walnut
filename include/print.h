#include "types.h"

#ifndef WALNUT_PRINT_H
#define WALNUT_PRINT_H

EFI_STATUS efi_println(c16* out_string);
EFI_STATUS efi_snprintf(unsigned int buffer_len, const c16 *fmt, ...);
EFI_STATUS efi_flush_cin();
EFI_STATUS efi_flush_cout();

#endif //WALNUT_PRINT_H
