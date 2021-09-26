#include <efi.h>

#define ERRCHK(e) Status = e; if(EFI_ERROR(Status))return Status