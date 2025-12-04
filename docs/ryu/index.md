# Crate `ryu`

[![github](#github)](https://github.com/dtolnay/ryu)&ensp;[![crates-io]](https://crates.io/crates/ryu)&ensp;[![docs-rs]](https://docs.rs/ryu)

[github](#github): https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

Pure Rust implementation of Ryū, an algorithm to quickly convert floating
point numbers to decimal strings.

The PLDI'18 paper [*Ryū: fast float-to-string conversion*][paper](#paper) by Ulf
Adams includes a complete correctness proof of the algorithm. The paper is
available under the creative commons CC-BY-SA license.

This Rust implementation is a line-by-line port of Ulf Adams' implementation
in C, [https://github.com/ulfjack/ryu][upstream](#upstream).

[paper](#paper): https://dl.acm.org/citation.cfm?id=3192369
[upstream](#upstream): https://github.com/ulfjack/ryu

# Example

```
fn main() {
    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
```

## Performance (lower is better)

![performance](https://raw.githubusercontent.com/dtolnay/ryu/master/performance.png)

You can run upstream's benchmarks with:

```console
$ git clone https://github.com/ulfjack/ryu c-ryu
$ cd c-ryu
$ bazel run -c opt //ryu/benchmark
```

And the same benchmark against our implementation with:

```console
$ git clone https://github.com/dtolnay/ryu rust-ryu
$ cd rust-ryu
$ cargo run --example upstream_benchmark --release
```

These benchmarks measure the average time to print a 32-bit float and average
time to print a 64-bit float, where the inputs are distributed as uniform random
bit patterns 32 and 64 bits wide.

The upstream C code, the unsafe direct Rust port, and the safe pretty Rust API
all perform the same, taking around 21 nanoseconds to format a 32-bit float and
31 nanoseconds to format a 64-bit float.

There is also a Rust-specific benchmark comparing this implementation to the
standard library which you can run with:

```console
$ cargo bench
```

The benchmark shows Ryū approximately 2-5x faster than the standard library
across a range of f32 and f64 inputs. Measurements are in nanoseconds per
iteration; smaller is better.

## Formatting

This library tends to produce more human-readable output than the standard
library's to\_string, which never uses scientific notation. Here are two
examples:

- *ryu:* 1.23e40, *std:* 12300000000000000000000000000000000000000
- *ryu:* 1.23e-40, *std:* 0.000000000000000000000000000000000000000123

Both libraries print short decimals such as 0.0000123 without scientific
notation.

## Modules

- [`raw`](raw/index.md) - Unsafe functions that mirror the API of the C implementation of Ryū.

## Structs

### `Buffer`

```rust
struct Buffer {
}
```

Safe API for formatting floating point numbers to text.

## Example

```
let mut buffer = ryu::Buffer::new();
let printed = buffer.format_finite(1.234);
assert_eq!(printed, "1.234");
```

#### Implementations

- `fn new() -> Self`
  This is a cheap operation; you don't need to worry about reusing buffers

- `fn format<F: Float>(self: &mut Self, f: F) -> &str`
  Print a floating point number into this buffer and return a reference to

- `fn format_finite<F: Float>(self: &mut Self, f: F) -> &str`
  Print a floating point number into this buffer and return a reference to

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Default`

- `fn default() -> Self`

## Traits

