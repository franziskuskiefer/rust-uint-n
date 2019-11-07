# Rust uint_n

Stack-allocated, arbitrary-length integers for Rust (based on [BigUint](https://github.com/rust-num/num-bigint)).

## Usage

```Rust
#[bits(256)]
struct Uint256;

let a = Uint256::from(254);
let b = Uint256::from("ABCDEF0123456789ABCDEF0123456789");
let c = a + b;
let d = a * b;
```
