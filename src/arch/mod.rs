//! This module exports the same API across architectures
//! depending on the target architecture.

mod x86_64;
pub use x86_64::*;
