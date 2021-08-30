#![feature(asm)]

use std::{ffi::c_void, mem, ptr::{null_mut}};

const PAGE_SIZE: usize = 1024 * 1024;

fn main() {
    let program: Box<[u8]> = compile(",+.".into());
    let f: fn(*mut u8) -> *mut u8;
    let res: *mut u8;
    let p: *mut u8;
    let m: *mut u8;
    unsafe {
        p = allocate(program.len()) as *mut u8;
        m = allocate(PAGE_SIZE) as *mut u8;
        println!("{:?} {:?}", p, m);
        p.copy_from(program.as_ptr(), program.len());
        f = mem::transmute(p);
    }
    res = f(m);
    println!("\n{:?}", res);
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

fn compile(src: String) -> Box<[u8]> {
    let mut program: Vec<u8> = vec!(
        // mov r15, rdi ; the current pointer at r15
        0x49, 0x89, 0xff,
        // mov rax, 0x0
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00,
    );
    for c in src.as_str().chars() {
        program.append(&mut match c {
            // put a char
            '.' => vec!(
                // mov rax, 0x01
                0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00,
                // mov rdi, 0x01
                0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00,
                // mov rsi, r15
                0x4c, 0x89, 0xfe,
                // mov rdx, 0x01
                0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00,
                // syscall
                0x0f, 0x05,
            ),
            // read a char
            ',' => vec!(
                // mov rax, 0x00
                0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00,
                // mov rdi, 0x00
                0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00,
                // mov rdx, 0x01
                0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00,
                // mov rsi, r15
                0x4c, 0x89, 0xfe,
                // syscall
                0x0f, 0x05,
            ),
            // increment the value pointed by r15
            '+' => vec!(
                // mov ax, WORD PTR [r15]
                0x66, 0x41, 0x8b, 0x07,
                // inc ax
                0x66, 0xff, 0xc0,
                // mov WORD PTR [r15], ax
                0x66, 0x41, 0x89, 0x07,
            ),
            // decrement the value pointed by r15
            '-' => vec!(
                // mov ax, WORD PTR [r15]
                0x66, 0x41, 0x8b, 0x07,
                // dec ax
                0x66, 0xff, 0xc8,
                // mov WORD PTR [r15], ax
                0x66, 0x41, 0x89, 0x07,
            ),
            // increment the pointer
            '>' => vec!(
                // inc r15
                0x49, 0xff, 0xc7,
            ),
            // decrement the pointer
            '<' => vec!(
                // dec r15
                0x49, 0xff, 0xcf,
            ),
            _ => vec!(),
        })
    }
    // program.append(&mut vec!(0x4c, 0x89, 0xf8));
    program.append(&mut vec!(0xc3)); // ret
    program.into_boxed_slice()
}