#include "efi/types.h"
 
#define EFIERR(a) (a | ~(((EFI_STATUS)-1) >> 1))
#define EFI_ERROR(a) (a & ~(((EFI_STATUS)-1) >> 1))

// Checks if a given error is not EFI_SUCCESS, if it is, return the error.
 
#define EFI_NOT_READY EFIERR(6)
 