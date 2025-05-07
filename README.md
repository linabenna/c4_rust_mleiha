# C4 Compiler Translated to Rust

This project is a Rust-based reimplementation of the original C4 compiler, building upon the in-depth analysis conducted in Assignment 1. The objective was to recreate the same core functionality and compile the same subset of C code supported by the original C4 compiler, while leveraging Rust’s memory safety, strong typing, and modern tooling.

By rewriting the compiler in Rust, we aimed to preserve the original behavior and output, while improving code clarity, maintainability, and structure where possible. This approach has also allowed us to explore the compiler's internal mechanisms in greater detail and apply language-level improvements aligned with Rust’s paradigms.

The complete translation is provided in src/main.rs

## Features
- Supports the same subset of C as the original C4 compiler  
- Rust-based implementation focused on performance and safety  
- Modular and cleanly structured codebase  
- Similar behavior with the C version across supported test cases  

## Usage
Clone the repository and navigate to the compiler source directory:

```bash
git clone https://github.com/linabenna/c4_rust_mleiha.git
cd c4_rust_mleiha/src
```
## Build & Run
To build the project:
```bash
cargo build
```
To run the compiler on a C source file (e.g., hello_world.c):
```bash
cargo run -- hello_world.c
```

## View Documentation
You can generate and view the Rust documentation for the codebase using:
```bash
cargo doc
cargo doc --open
```
This will open the documentation in your default web browser, allowing you to explore modules, functions, and internal structure.

## Deliverables:
- Translation in src/main.rs
- Comparision Document in c4_rust_comparison.pdf
- Bonus Code and Documentation in Bonus/bonus_documented.pdf
