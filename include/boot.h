#pragma once
#include "types.h"
#include "print.h"
#include "boot/graphics/framebuffer.h"
frame_buffer_t* init_gop();

extern void exit_qemu(void);