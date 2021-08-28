#![feature(asm)]

use std::{ffi::c_void, mem, ptr::{null_mut}};

const PAGE_SIZE: usize = 4096;

fn main() {
    let program: Box<[u8]> = Box::new([
        0x48, 0x83, 0xc0, 0x1f,
        0x48, 0x89, 0xc6,
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00,
        0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00,
        0x48, 0xC7, 0xC2, 0x0E, 0x00, 0x00, 0x00,
        0x0F, 0x05,
        0xC3,
        0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0a]);
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
        let f: fn(*mut u8) -> i64 = mem::transmute(q);
        println!("{:?}", f(q));
    }
}
