# num-digitize
![Image of Travis CI build status]
(https://travis-ci.org/lunemec/rust-num-digitize.svg?branch=master)

Converts integer of type N (all implementations supported by num::Integer)
and returns a `Vec<N>` of its digits (base 10).

[Documentation](https://lunemec.github.io/rust-num-digitize/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
num-digitize = "0.1"
```

and this to your crate root:

```rust
extern crate num-digitize;
```