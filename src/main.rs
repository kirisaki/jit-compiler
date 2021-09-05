#![feature(asm)]

use std::{ffi::c_void, mem, ptr::{null_mut}};

const PAGE_SIZE: usize = 1024 * 1024;

#[derive(Clone, Debug)]
enum Op{
    Put,
    Get,
    IncVal,
    DecVal,
    IncPtr,
    DecPtr,
    Loop (Box<Vec<Op>>),
}

fn main() {
    let program: Box<[u8]> = compile(">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++
++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>
++++++++[<++++>-]<+.[-]++++++++++.".into());
    unsafe {
        let f: fn(*mut u8);
        let p: *mut u8;
        let m: *mut u8;
        p = allocate(program.len()) as *mut u8;
        m = allocate(PAGE_SIZE) as *mut u8;
        p.copy_from(program.as_ptr(), program.len());
        f = mem::transmute(p);
        f(m);
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

fn compile(src: &str) -> Box<[u8]> {
    let mut program: Vec<u8> = vec!(
        // mov r15, rdi ; the current pointer at r15
        0x49, 0x89, 0xff,
        // mov rax, 0x0
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00,
    );
    let (ops, _) = parse(src, 0);
    let mut asm = assemble(ops.unwrap());
    program.append(&mut asm);    
    program.append(&mut vec!(0xc3)); // ret
    program.into_boxed_slice()
}

fn assemble(ops: Vec<Op>) -> Vec<u8> {
    let mut output: Vec<u8> = vec!();
    for op in ops {
        output.append(&mut match op {
            Op::Put => vec!(
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
            Op::Get => vec!(
                // mov rax, 0x00
                0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00,
                // mov rdi, 0x00
                0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00,
                // mov rsi, r15
                0x4c, 0x89, 0xfe,
                // mov rdx, 0x01
                0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00,
                // syscall
                0x0f, 0x05,
            ),
            Op::IncVal => vec!(
                // inc BYTE PTR [r15]
                0x41, 0xfe, 0x07,
            ),
            Op::DecVal => vec!(
                // dec BYTE PTR [r15]
                0x41, 0xfe, 0x0f,
            ),
            Op::IncPtr => vec!(
                // inc r15
                0x49, 0xff, 0xc7,
            ),
            Op::DecPtr => vec!(
                // dec r15
                0x49, 0xff, 0xcf,
            ),
            Op::Loop(l) => {
                let mut iter = assemble(l.to_vec());
                let bs = (iter.len() as i32 + 12).to_le_bytes();
                let cs = (-(iter.len() as i32) - 12).to_le_bytes();
                let mut asm: Vec<u8> = vec!();
                // the start of the loop
                asm.append(&mut vec!(
                    // mov al, BYTE PTR [r15]
                    0x41, 0x8a, 0x07,
                    // test al, al
                    0x84, 0xc0,
                    // jne <rel 0x05>
                    0x75, 0x05,
                    // jmp <rel offset>
                    0xe9, bs[0], bs[1], bs[2], bs[3],
                ));
                // inside the loop
                asm.append(&mut iter);
                // the end of the loop
                asm.append(&mut vec!(
                    // mov al, BYTE PTR [r15]
                    0x41, 0x8a, 0x07,
                    // test al, al
                    0x84, 0xc0,
                    // je <rel 0x05>
                    0x74, 0x05,
                    // jmp <rel offset>
                    0xe9, cs[0], cs[1], cs[2], cs[3],
                ));
                asm
            },
        })
    }
    output 
}

fn parse(src: &str, depth: i64) -> (Result<Vec<Op>, &str>, &str) {
    let mut rest: &str = src;
    let mut ops: Vec<Op> = vec!();
    loop {
        if rest.len() == 0 && depth == 0 {
            return (Ok(ops), "")
        } else if rest.len() == 0 && depth > 0 {
            return (Err("unbalanced bracket"), "")
        }
        let (head, tail) = rest.split_at(1);
        rest = tail;
        match head {
            "." => ops.push(Op::Put),
            "," => ops.push(Op::Get),
            "+" => ops.push(Op::IncVal),
            "-" => ops.push(Op::DecVal),
            ">" => ops.push(Op::IncPtr),
            "<" => ops.push(Op::DecPtr),
            "[" => {
                match parse(tail, depth + 1) {
                    (Ok(xs), ys) => {
                        ops.push(Op::Loop(Box::new(xs)));
                        rest = ys;
                    },
                    (Err(e), ys) => {
                        return (Err(e), ys)
                    },
                }
            },
            "]" => {
                if depth - 1 >= 0 {
                    return (Ok(ops), rest)
                } else {
                    return (Err("unexpected bracket"), rest)
                }
            },
            _ => {},
        }
    }
}