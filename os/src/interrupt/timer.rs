use crate::sbi::set_timer;
use riscv::register::{time, sie, sstatus};

// The interval of clock interruption, the unit is CPU refers to
static INTERVAL: usize = 100000;

// Trigger clock interrupt count
pub static mut TICKS: usize = 0;

// init clock interupt
// Turn on the clock interrupt enable and schedule the first clock interrupt

pub fn init() {
    unsafe {
        // Start STIE to enable clock interrupt
        sie::set_stimer();
        // Start SIE (not sie register), Allow kernel mode to be interrupted by interrupt
        // sstatus::set_sie();
    }
    // Set the next clock interrupt
    set_next_timeout();
}

fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}


/// Called every time the clock is interrupted
///
/// Set the next clock interrupt and count at the same time +1
pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}