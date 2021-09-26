#pragma once


#include "types/std.h"
#include "efi.h"

extern EFI_SYSTEM_TABLE* ST;
extern EFI_BOOT_SERVICES* BS;



typedef i64 err_t;
#define SUCCESS 0;

#define FAILURE -1;
#define COULDNT_FIND -2;
