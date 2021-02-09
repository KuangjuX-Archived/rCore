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



    println!("Hello, KuangjuX!");

    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }


    panic!("end of rust_main");
}
