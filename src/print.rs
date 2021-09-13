use core::fmt::{Result, Write};

pub struct ScreenWriter;

impl Write for ScreenWriter {
    fn write_str(&mut self, string: &str) -> Result {
        crate::efi::output_string(string);
        Ok(())
    }
}

pub fn _print(args: core::fmt::Arguments) {
    <ScreenWriter as core::fmt::Write>::write_fmt(&mut ScreenWriter, args).unwrap();
}

/// The standard Rust `print!()` macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
