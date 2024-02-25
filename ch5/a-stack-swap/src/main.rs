use std::arch::asm;
use std::mem;
use std::ops::Neg;

const STACK_SIZE: isize = 1024;
#[derive(Debug, Default)]
#[repr(align(16))]
struct ThreadContext {
    rsp: u64,
}
fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0u8; STACK_SIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(STACK_SIZE);
        let stack_bottom_aligned = (stack_bottom as usize & !15) as *mut u8;
        std::ptr::write(stack_bottom_aligned.offset((mem::size_of::<u64>() as isize).neg()) as *mut u64, hello as u64);
        ctx.rsp = stack_bottom_aligned.offset((mem::size_of::<u64>() as isize).neg()) as u64;
        gt_switch(&ctx);
    }
}
fn hello() -> ! {
    println!("I love waking up on a new stack!");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0}]",
        "ret",
        in(reg) new,
    )
}
