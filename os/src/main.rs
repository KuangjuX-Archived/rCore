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

//! # Some unstable functions need to be declared at the crate level before they can be used
//!
//!-`#![feature(alloc_error_handler)]`
//! We use a global dynamic memory allocator to 
//! implement the heap memory allocation in the original standard library.
//! The language requires us to implement an error callback at the same time, 
//! here we directly panic
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;

extern crate alloc;

global_asm!(include_str!("entry.asm"));


// override the _start function in crt0
#[no_mangle]
pub extern "C" fn rust_main() {
    interrupt::init();
    memory::init();



    println!("Hello, rCore-Tutorial!");

    // unsafe { llvm_asm!("ebreak") };

    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }

    println!("heap test passed");

    panic!("end of rust_main");
}
