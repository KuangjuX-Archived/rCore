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
mod process;

extern crate alloc;

use process::*;

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

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("kernel remapped");


    extern "C" {
        fn __restore(context: usize);
    }
    // 获取第一个线程的 Context，具体原理后面讲解
    let context = PROCESSOR.lock().prepare_next_thread();
    // 启动第一个线程
    unsafe { __restore(context as usize) };
    unreachable!();


    // panic!("end of rust_main");
}


/// 内核线程需要调用这个函数来退出
fn kernel_thread_exit() {
    // 当前线程标记为结束
    PROCESSOR.lock().current_thread().as_ref().inner().dead = true;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}
