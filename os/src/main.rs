// Disable standard library
#![no_std]

// Do not use all Rust-level entries such as the'main' function as the program entry
#![no_main]

// Embed assembly
#![feature(llvm_asm)]

// Embed the entire assembly file
#![feature(global_asm)]

// Get message and print when panic!
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;

global_asm!(include_str!("entry.asm"));


// override the _start function in crt0
#[no_mangle]
pub extern "C" fn rust_main() {
    interrupt::init();

    println!("Hello, rCore-Tutorial!");

    // unsafe { llvm_asm!("ebreak") };

    // panic!("end of rust_main");
}
