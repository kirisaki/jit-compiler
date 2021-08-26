#![feature(asm)]

use std::{ffi::c_void, mem, ptr::null_mut};

const PAGE_SIZE: usize = 4096;

fn main() {
    let program: Box<[u8]> = Box::new([0xc3; PAGE_SIZE]);
    unsafe {
        let p: *mut c_void = null_mut();
        let mut q: *mut u8;
        asm!(
            "mov rax, 9",
            "mov rdi, {0}",
            "mov rsi, {1}",
            "mov rdx, 0x7",
            "mov r10, 0x1022",
            "mov r8, 0",
            "mov r9, 0",
            "syscall",
            in(reg) p,
            const PAGE_SIZE,
            out("rax") q,
        );
        q.copy_from_nonoverlapping(program.as_ptr(), PAGE_SIZE);
        let f: fn() = mem::transmute(q);
        f();
    }
}
