# C4 Compiler Translated to Rust

This project is a Rust-based reimplementation of the original C4 compiler, building upon the in-depth analysis conducted in Assignment 1. The objective was to recreate the same core functionality and compile the same subset of C code supported by the original C4 compiler, while leveraging Rust’s memory safety, strong typing, and modern tooling.

By rewriting the compiler in Rust, we aimed to preserve the original behavior and output, while improving code clarity, maintainability, and structure where possible. This approach has also allowed us to explore the compiler's internal mechanisms in greater detail and apply language-level improvements aligned with Rust’s paradigms.

## Features

- Supports the same subset of C as the original C4 compiler  
- Rust-based implementation focused on performance and safety  
- Modular and cleanly structured codebase  
- Similar behavior with the C version across supported test cases  

## Usage

Clone the repository and navigate to the compiler source directory:

```bash
git clone https://github.com/<your-username>/c4_rust_mleiha.git
cd c4_rust_mleiha/src
