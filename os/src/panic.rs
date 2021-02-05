// Implement function of panic and abort to replace stanard library

use core::panic::PanicInfo;
use crate::sbi::shutdown;

// Call this function when panic happend
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    shutdown();
}


// Finish programe run
// Call panic_handler
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()!")
}