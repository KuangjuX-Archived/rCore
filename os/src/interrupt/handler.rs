use super::context::Context;
use riscv::register::stvec;

global_asm!(include_str!("./interrupt.asm"))

// init interrupt
// write tne entry of __interrupt into stvec and start interrupt function
pub fn init() {
    unsafe{
        extern "C" {
            // Entry of `interrupt_asm`
            fn __interrupt();
        }

        // Use Direct Mode, set entry of interrupt `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}


// Entry of interrupt handler

#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause. stval: usize) {
    panic!("Interrupted: {:?}", scause.cause());
}