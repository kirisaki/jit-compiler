#![feature(asm)]

use std::{ffi::c_void, mem, ptr::{null_mut}};

const PAGE_SIZE: usize = 1024 * 1024;

fn main() {
    let program: Box<[u8]> = compile("".into(), PAGE_SIZE);
    let memory: Box<[u8]> = Box::new([
        0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0a]);
    unsafe {
        let p = allocate(PAGE_SIZE) as *mut u8;
        let m = allocate(14) as *mut u8;
        println!("{:?} {:?}", p, m);
        p.copy_from_nonoverlapping(program.as_ptr(), program.len());
        m.copy_from_nonoverlapping(memory.as_ptr(), 14);
        let f: fn(*mut u8) -> *mut u8 = mem::transmute(p);
        println!("{:?}", f(m));
    }
}

unsafe fn allocate(len: usize) -> *mut c_void {
    let p: *mut c_void = null_mut();
    let mut q: *mut c_void;
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
        in(reg) len,
        out("rax") q,
    );
    q
}

fn compile(src: String, len: usize) -> Box<[u8]> {
    let mut program: Vec<u8> = Vec::with_capacity(len); 
    let mut p: Vec<u8> = vec!(
        0x48, 0x8b, 0x74, 0x24, 0x30,
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00,
        0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00,
        0x48, 0xC7, 0xC2, 0x0E, 0x00, 0x00, 0x00,
        0x0F, 0x05,
        0xc3,
    );
    program.append(&mut p);
    program.into_boxed_slice()
}