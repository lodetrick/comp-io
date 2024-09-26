A Rust library for competitive programming to make io operations simpler.

The `comp-io` crate provides methods to read numerical values, as well as chars from stdin

# Usage

The usage of `comp-io` typically looks like this:

```rust
let mut reader = comp_io::Reader::new();

let num1: i32 = reader.next_i32().unwrap();
let num2: f64 = reader.next_f64().unwrap();

println!("read: {num1} {num2}");
```

Note: The `Reader` struct expects an EOF at the end of input. To enter this in the terminal, press `CTRL + D`