#pragma once

typedef enum
{
    false = ( 1 == 0 ),
    true = ( ! false )
} bool;

// Unsigned Integers

typedef unsigned long long u64;
typedef unsigned int  u32;
typedef unsigned int u16;
typedef unsigned char u8;

// Characters

typedef unsigned short c16;
typedef unsigned short c16;
typedef unsigned char c8;

typedef c16* str16;
typedef c8* str8;

// Floats
typedef float f32;
typedef double f64;