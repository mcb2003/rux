use core::fmt::{self, Write};

use spin::{Lazy, Mutex};
use uart_16550::SerialPort;

/// Base IO address of the first serial port.
const IO_BASE: u16 = 0x3F8;

/// An interface to the first serial port.
static SERIAL1: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
    let mut sp = unsafe { SerialPort::new(IO_BASE) };
    sp.init();
    Mutex::new(sp)
});

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    SERIAL1.lock().write_fmt(args).unwrap();
}
