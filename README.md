# Rust JSON parsing

Simple high performance JSON parsing implementation that easily beats the fastest Property-parsing library gjson [https://github.com/AnnikaCodes/rust-json-parsing-benchmarks/tree/main].
Results from a i7-12700 running Windows 11:
```
best case:
own_parser_parse_first: the value of closeTime   1732985135837     4.40µs
gjson_parse_first: the value of closeTime        1732985135837    56.00µs

worst case:
own_parser_parse_all: the value of closeTime   1732946532759       5.20ms
gjson_parse_all: The value of closeTime        1732946532759       6.44ms
```
P.S. My implementaion is not feature complete or comparable to gjson,
just a impl. for a specialized use case.



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


