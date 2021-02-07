use super::context::Context;
use super::timer;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

global_asm!(include_str!("./interrupt.asm"));

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

        sie::set_sext();
    }
}


/// Entry of interrupt handler
/// `interrupt.asm` first save the register to Context, 
/// which is passed to this function as a parameter along with scause and stval
/// The specific interrupt type needs to be inferred according to scause, 
/// and then processed separately
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    // panic!("Interrupted: {:?}", scause.cause());
    match scause.cause() {
        // breakpoint interrupt(ebark)
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // clock interrupt
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // Other Situations: Stop current thread
        _ => fault(context, scause, stval),
    };
}

/// Set ebreak breakpoint
/// 
/// Continue execution, where `sepc` is 
/// increased by 2 bytes to skip the current `ebreak` instruction
fn breakpoint(context: &mut Context){
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}


/// Handler clock interrupt
/// 
/// Currently only counting in the [`timer`] module
fn supervisor_timer(_: &mut Context) {
    timer::tick();
}


/// An unresolved exception occurred
fn fault(context: &mut Context, scause: Scause, stval: usize){
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstavl: {:x}",
        scause.cause(),
        context,
        stval
    );
}