## Installing Rust

See https://doc.rust-lang.org/stable/book/ch01-01-installation.html

Easier way: nstall `rustup` from brew

To install older version of toolchain:
`rustup install 1.58.1`

To change default toolchain:
`rustup default 1.58.1`

Compiler is called `rustc`

If zsh uses the wrong rust binary:
`export PATH="$HOME/.cargo/bin:$PATH"`

## Hello World

See `hello_world/main.rs`

## Package Management

- `cargo new project_name` to gen new project
- `cargo build` to build (in debug mode). The executable will be at `./target/debug/project_name`
- `cargo run` to compile and run
- `cargo check` to check if it can compile
- `cargo build --release` compiles with optimisation for production