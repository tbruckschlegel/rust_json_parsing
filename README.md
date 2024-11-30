# Rust JSON parsing

Simple high performance JSON parsing implementation that easily beats the fastest Property-parsing library gjson [https://github.com/AnnikaCodes/rust-json-parsing-benchmarks/tree/main].

## Requirements

*install Rust

## Building

**Clone the repo**

```$ git clone https://github.com/tbruckschlegel/rust_json_parsing.git```

To run the code with debug output:
```
cargo build --features debug
cargo run --features debug
```
otherwise:
```
cargo build --release
cargo run --release
```


**Tests & performance comparison against gjson**
```
cargo test -- --nocapture
```


