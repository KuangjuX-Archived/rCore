use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
#[derive(Debug)]

// Go have Context too !!!
// Store general register and spec, scause, stval, sstatus
// scause and stval is treated as temporary variables
pub struct Context{
    pub x: [usize; 32], // 32 general register
    pub sstatus: Sstatus,
    pub spec: usize
}