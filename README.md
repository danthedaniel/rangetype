Rust RangeType
===

An Ada-inspired Range type for Rust.

### Features

**Compile-time checks**

The following will fail to compile:

```rust
#[macro_use]
extern crate rangetype;

fn main() {
    // Range with a value of 5 that should be within in range [-5, 2]
    let x = range!(5, -5..2);
}
```

**Run-time checks**

```rust
// Will panic since the two numbers are for different ranges
let x = range!(5, 0..10);
let y = range!(10, 10..128);

x + y // panic!
```

```rust
// Will panic because 5 + 10 = 15 which will overflow the range of 0..10
let x = range!(5, 0..10);
let y = range!(10, 0..10);

let z = x + y; // panic!
```