# We will use macro to store register for loop
.altmacro
# The number of bytes corresponding to the register width
.set    REG_SIZE, 8
# Context size
.set    CONTEXT_SIZE, 34

#  Macro: store register into stack
.macro SAVE reg, offset
    sd \reg, \offset*8(sp)
.endm

.macro SAVE_N n 
    SAVE x\n, \n 
.endm 

.macro LOAD_N n 
    LOAD x\n, \n 
.endm 

    .section .text 
    .globl __interrupt

# Into interrupt
# Store Context && goto interrupt::handler::handle_interrupt()(Rust interrupt handler function)

__interrupt:
    # Open up the space required for Context on the stack
    addi sp, sp, -34*8; #   sp =sp1 -34*8;m
    
    # Store general register apart from x0(is 0)
    SAVE x1, 1
    # Write the original sp (sp aka x2) into position 2
    addi x1, sp, 34*8
    SAVE x1, 2
    # Save x3 to x31
    .set n, 3
    .rept 29
        SAVE_N %n 
        .set n, n+1
    .endr

    # Take out CSR and store
    csrr s1, sstatus
    csrr s2, sepc
    SAVE s1, 32
    SAVE s2, 33

    # Call handle_interrupt and pass params
    # context: &mut Context
    mv a0, sp
    # scause: Scause
    csrr a1, scause
    # stval: usize
    csrr a2, stval
    jal handle_interrupt

    .globl __restore

# Leave interrupt
# Restore all registers from Context and jump to the position of sepc in Context     
__restore:
    # restore CSR
    LOAD s1, 32
    LOAD s2, 33
    csrw sstatus, s1
    csrw sepc, s2

    # restore general register
    LOAD x1, 1
    # Restore x3 to x31
    .set n, 3
    .rept 29
        LOAD_N %n 
        .set n, n+1
    .endr 

    # Restore sp (aka x2) for use marco correctly
    LOAD x2, 2
    sret