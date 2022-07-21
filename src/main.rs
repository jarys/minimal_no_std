#![no_std]
#![no_main]
#![feature(lang_items)]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
unsafe fn _start() {
    core::arch::asm!("mov rdi, rsp", "call main", options(noreturn))
}

// reminder: `!` is the `never` type - this indicates
// that `exit` never returns.
pub unsafe fn exit(code: i32) -> ! {
    let syscall_number: u64 = 60;
    core::arch::asm!(
        "syscall",
        in("rax") syscall_number,
        in("rdi") code,
        options(noreturn)
    );
}

#[no_mangle]
pub unsafe fn main(stack_top: *const u8) {
    let argc = *(stack_top as *const u64);
    exit(argc as _);
}
