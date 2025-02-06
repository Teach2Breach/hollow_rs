#### Process Hollowing in Rust

A Rust PoC implementation of the Early Bird process hollowing technique, inspired by [@boku7/HOLLOW](https://github.com/boku7/HOLLOW). This version reimplements the core functionality in safe Rust while maintaining direct Windows API interactions through the `winapi` crate.

##### Overview

This library provides process hollowing capabilities using the Early Bird injection technique, implemented in Rust. It:
- Creates a new process in a suspended state
- Allocates memory in the target process
- Injects shellcode into the allocated memory
- Queues an APC to execute the shellcode
- Resumes the target process

##### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hollow_rs = { git = "https://github.com/Teach2Breach/hollow_rs" }
```

##### Example

```rust
use hollow_rs::wrapper;

wrapper("notepad.exe", &SHELL_CODE);
```

##### Video of creating this PoC

[watch on X](https://x.com/Teach2Breach/status/1887594765165752772)
