

## Cross compiling

```rust
$ cd app
$ rustup target add thumbv7m-none-eabi
$ cargo build --target thumbv7m-none-eabi
$ cargo build
```

## Inspecting

```rust
// print the ELF headers to confirm that this is an ARM binary
$ cargo readobj target/thumbv7m-none-eabi/debug/app -- -file-headers
// print the size of the linker sections of the binary.
$ cargo size target/thumbv7m-none-eabi/debug/app --release -- -A
```

