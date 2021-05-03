pub mod vga;

#[macro_export]
macro_rules! print {
    ($($arg: tt)*) => {
        $crate::output::vga::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt: expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt: expr, $($arg: tt)*) => ($crate::print!(
        concat!($fmt, "\n"), $($arg)*));
}
