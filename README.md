# jit-compiler

An example of Brainf\*\*\* JIT-compiler with only the standard library.

## Prerequisite

- Rust(1.56.0-nightly or later, but it must work kind of older version)
- Linux or WSL, on X64

## How to run

```console
$ cargo run
Hello World!
```

## How it works

`allocate` function calls the systemcall `mmap` directly with an executable flag.
This allocates memories when a file descriptor isn't given.
In macOS, how to call `mmap` is different, so rewrite the code appropriately.
Also, if you use M1 mac,  ISA is different from X86/64 and must rewrite the code.
I don't know the details about Windows API, but to just try it, you should use WSL.

`assemble` function translates an AST into machine code.
If you rewrite it, refer to mnemonics at the comments.
It must be easier to do on other architecture than X86/64 ISA.

## License
This code is licensed under [Unlicense](LICENSE).
