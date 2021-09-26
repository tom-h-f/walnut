#pragma once

#include "types.h"

#include "efi/print.h"
#include "efi/types.h"
#include "efi/error.h"
#include "efi/table.h"
#include "efi/memory.h"
#include "efi/boot.h"

 

// The EFI System Table, defined upon entry to `efi_main`
extern EFI_SYSTEM_TABLE* ST;

