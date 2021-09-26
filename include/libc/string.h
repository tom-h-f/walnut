#pragma once

#include "types.h"

c16* strcat(c16* dest, const c16* src);

static unsigned int itoa(int value, unsigned int radix, unsigned int uppercase, unsigned int unsig, c16 *buffer, unsigned int zero_pad);
int vsnprintf(c16 *buffer, unsigned int buffer_len, const c16 *fmt, __builtin_va_list va);
int snprintf(c16* buffer, unsigned int buffer_len, const c16 *fmt, ...);
