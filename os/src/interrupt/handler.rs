use super::context::Context;
use super::timer;
// use crate::fs::STDIN;
// use crate::kernel::syscall_handler;
use crate::memory::*;
use crate::process::PROCESSOR;
use crate::sbi::console_getchar;
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

        // 在 OpenSBI 中开启外部中断
        *PhysicalAddress(0x0c00_2080).deref_kernel() = 1u32 << 10;
        // 在 OpenSBI 中开启串口
        *PhysicalAddress(0x1000_0004).deref_kernel() = 0x0bu8;
        *PhysicalAddress(0x1000_0001).deref_kernel() = 0x01u8;
        // 其他一些外部中断相关魔数
        *PhysicalAddress(0x0C00_0028).deref_kernel() = 0x07u32;
        *PhysicalAddress(0x0C20_1000).deref_kernel() = 0u32;
    }
}


/// Entry of interrupt handler
/// `interrupt.asm` first save the register to Context, 
/// which is passed to this function as a parameter along with scause and stval
/// The specific interrupt type needs to be inferred according to scause, 
/// and then processed separately
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context{
    // panic!("Interrupted: {:?}", scause.cause());
    // context.sepc = 0;
    {
        let mut processor = PROCESSOR.lock();
        let current_thread = processor.current_thread();
        if current_thread.as_ref().unwrap().dead {
            println!("thread {} exit.", current_thread.id);
            processor.kill_current_thread();
            return processor.prepare_next_thread();
        }
    }

    match scause.cause() {
        // breakpoint interrupt(ebark)
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // clock interrupt
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // Visit a non-existent address
        Trap::Exception(Exception::LoadFault) => load_falut(context, scause, stval),
        // Outer Interrupt
        // Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
        // Other Situations: Stop current thread
        _ => fault("unimplemented interrupt type", scause, stval),
    };
}

/// Set ebreak breakpoint
/// 
/// Continue execution, where `sepc` is 
/// increased by 2 bytes to skip the current `ebreak` instruction
fn breakpoint(context: &mut Context) -> *mut Context{
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
    // context.sepc = 0;
    context;
}


/// Handler clock interrupt
/// 
/// Currently only counting in the [`timer`] module
fn supervisor_timer(context: &mut Context) -> *mut Context{
    timer::tick();
    PROCESSOR.lock().park_current_thread(context);
    PROCESSOR.lock().prepare_next_thread();
    context;
}

// Handler Load Fault
fn load_falut(context: &mut Context, scause: Scause, stval: usize) -> *mut Context{
    if (stval == 0){
        println!("Success!")
    }
    panic!(
        "Interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
    context;
}


/// 处理外部中断，只实现了键盘输入
// fn supervisor_external(context: &mut Context) -> *mut Context {
//     let mut c = console_getchar();
//     if c <= 255 {
//         if c == '\r' as usize {
//             c = '\n' as usize;
//         }
//         STDIN.push(c as u8);
//     }
//     context
// }


/// An unresolved exception occurred
fn fault(msg: &str, scause: Scause, stval: usize) -> *mut Context{
    println!(
        "{:#x?} terminated: {}",
        PROCESSOR.lock().current_thread(),
        msg
    );
    println!("cause: {:?}, stval: {:x}", scause.cause(), stval);

    PROCESSOR.lock().kill_current_thread();
    // 跳转到 PROCESSOR 调度的下一个线程
    PROCESSOR.lock().prepare_next_thread()
}