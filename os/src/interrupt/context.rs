use riscv::register::{sstatus:Sstatus, scause::Scause}

#[repr(C)]
#[derive(debug)]

pub struct Context{
    pub x: [usize: 32], // 32 register
    pub sstatus: Sstatus,
    pub spec: usize
}