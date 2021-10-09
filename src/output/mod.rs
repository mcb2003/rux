pub mod serial;
pub mod vga;

/// Print to the VGA text buffer and over the first serial port.
#[macro_export]
macro_rules! print {
    ($($arg: tt)*) => {
        $crate::output::vga::_print(format_args!($($arg)*));
        $crate::output::serial::_print(format_args!($($arg)*));
    };
}

/// Print to the VGA text buffer and over the first serial port, appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt: expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt: expr, $($arg: tt)*) => ($crate::print!(
        concat!($fmt, "\n"), $($arg)*));
}
