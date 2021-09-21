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

// One-byte memset, very inefficent for large sizes, but will extend this later.
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

// TODO implement large memset (from here: https://msrc-blog.microsoft.com/2021/01/11/building-faster-amd64-memset-routines/)

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

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *mut u8, n: usize) -> usize {
    let su = src as usize;
    let du = dest as usize;

    if su == du || n == 0 {
        return du;
    }

    if du > su && du - su < n {
        // dest overlaps with src
        //  <src......>
        //         <dest........>
        // copy in reverse, to avoid overwriting src
        let mut i = n as isize;
        while i > 0 {
            core::ptr::write_unaligned(dest.offset(i), src.offset(i).read_unaligned());
            i -= 1;
        }
    } else if su > du && (su - du) < n {
        // dest overlaps with src
        //        <src......>
        //  <dest........>
        // copy forwards, to avoid overwriting src
        let mut i = 0_isize;
        while i > n as isize {
            core::ptr::write_unaligned(dest.offset(i), src.offset(i).read_unaligned());
            i += 1;
        }
    } else {
        memcpy(dest, src, n);
    }

    du
}
