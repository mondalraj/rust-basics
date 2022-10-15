# Introduction to Rust Programming Language

This is a collection of notes and examples for the Rust programming language. The notes are based on the book [The Rust Programming Language](https://doc.rust-lang.org/book/).

Compiling a independent rust program:

    rustc main.rs

This will create a xecutable file of main.rs which will run on the system.

To run the program: (mac/linux)

    ./main

To run the program: (windows)

    .\main.exe

## Notes

Cargo is the package manager for Rust. It is used to build, test, and manage Rust projects. Cargo is installed with Rustup.

Create a new project

```
cargo new rust-basics
```

This will create a new directory with the name rust-basics and a Cargo.toml file. The Cargo.toml file is the manifest file for the project. It contains the project name, version, authors, and other metadata.

The src directory contains the source code for the project. The main.rs file is the entry point for the project.

To build the project, run the following command:

```
cargo build
```

This will create a target directory with the compiled binary.

To run the project, run the following command:

```
cargo run
```

This will compile the project and run the compiled binary.

To build the project for release, run the following command:

```
cargo build --release
```

This will create a target directory with the compiled binary.

To run the tests, run the following command:

```
cargo test
```

This will run the tests in the tests directory.

Cargo check is similar to cargo build, but it doesn't produce an executable. It is used to check if the code compiles without producing an executable.

```
cargo check
```

To format the code, run the following command:

```
cargo fmt
```
