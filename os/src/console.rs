use crate::sbi::*
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout{
    // print string

    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = [0u8; 4];
        for c in s.chars() {
            for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
                console_putchar(*code_point as usize);
            }
        }

        Ok(())
    }
}



// Print format data by [`core::format_args!`]

// [`print!`] and [`println!`] marcos will expand into this function

pub fn print(args: fmt:Arguments) {
    Stdout.write_fmt(args).unwarp();
}


// Implement Marco `print!` like std lib
// Use [`core::fmt::Write`] which is trait implement [`console::Stdout`]
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}



// Implement Marco `println!` like std lib
// Use [`core::fmt::Write`] which is trait implement [`console::Stdout`]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $(arg)+)?));
    }
}