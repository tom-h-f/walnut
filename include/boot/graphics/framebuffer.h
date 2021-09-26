
#include "types.h"

typedef struct {
    void* base_addr;
    size_t buf_size;
    u64 width;
    u64 height;
    u64 pixels_per_scanline;
} frame_buffer_t;