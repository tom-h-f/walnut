//! This module exports the same API across architectures
//! depending on the target architecture.

pub mod x86_64;
pub use self::x86_64::*;
