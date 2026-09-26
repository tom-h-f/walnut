//! This module fufills the requirements of memory functions required to build
//! Rust

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    asm!("rep movsb",
         inout("rcx") n => _,
         inout("rdi") dest => _,
         inout("rsi") src => _,);
    dest
}

/// Fill `n` bytes at `dest` with the low 8 bits of `byte`.
///
/// `rep stosb` is the whole implementation. The compiler calls this for
/// the sizes `alloc` clears. A wider store loop would not change the result.
#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, byte: u32, n: usize) -> *mut u8 {
    asm!(
    "rep stosb",
    inout("eax") byte => _,
    inout("rcx") n    => _,
    inout("rdi") dest => _,
    );
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        // calculate offsets from pointers w/ loop counter
        let a = *s1.add(i);
        let b = *s2.add(i);
        // if both values at ptr+offset are not equal,
        // return the difference
        if a != b {
            return (a as i32).wrapping_sub(b as i32);
        }
        // increment loop counter
        i = i.wrapping_add(1);
    }
    // return 0 to indicate all bytes compared were equal
    0
}

/// Copy `n` bytes from `src` to `dest`, including when the ranges overlap.
///
/// `rep movsb` walks upward. That is safe when `dest` is at or below `src`.
/// When `dest` sits inside `src`, an upward copy would read bytes it had
/// already overwritten, so those bytes are copied from the end.
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *mut u8, n: usize) -> usize {
    let su = src as usize;
    let du = dest as usize;

    if su == du || n == 0 {
        return du;
    }

    if du > su && du - su < n {
        let mut i = n;
        while i > 0 {
            i -= 1;
            core::ptr::write_unaligned(dest.add(i), src.add(i).read_unaligned());
        }
    } else {
        memcpy(dest, src, n);
    }

    du
}
