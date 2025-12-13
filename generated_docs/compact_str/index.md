# Crate `compact_str`

<div align="center">
  <h1><code>compact_str</code></h1>
  <p><strong>A memory efficient string type that can store up to 24* bytes on the stack.</strong></p>

  <a href="https://crates.io/crates/compact_str">
    <img alt="version on crates.io" src="https://img.shields.io/crates/v/compact_str"/>
  </a>
  <img alt="Minimum supported Rust Version: 1.60" src="https://img.shields.io/badge/MSRV-1.60-blueviolet">
  <a href="LICENSE">
    <img alt="mit license" src="https://img.shields.io/crates/l/compact_str"/>
  </a>

   <br />

  <a href="https://github.com/ParkMyCar/compact_str/actions/workflows/ci.yml">
    <img alt="Continuous Integration Status" src="https://github.com/ParkMyCar/compact_str/actions/workflows/ci.yml/badge.svg?branch=main&event=push"/>
  </a>
  <a href="https://github.com/ParkMyCar/compact_str/actions/workflows/cross_platform.yml">
    <img alt="Cross Platform Status" src="https://github.com/ParkMyCar/compact_str/actions/workflows/cross_platform.yml/badge.svg?branch=main&event=push"/>
  </a>
  <a href="https://github.com/ParkMyCar/compact_str/actions/workflows/msrv.yml">
    <img alt="Minimum Supported Rust Version Status" src="https://github.com/ParkMyCar/compact_str/actions/workflows/msrv.yml/badge.svg?branch=main&event=push"/>
  </a>
  <a href="https://github.com/ParkMyCar/compact_str/actions/workflows/clippy.yml">
    <img alt="Clippy Status" src="https://github.com/ParkMyCar/compact_str/actions/workflows/clippy.yml/badge.svg?branch=main&event=push"/>
  </a>

  <p  align=right><sub>* 12 bytes for 32-bit architectures</sub></p>
</div>

<br />

### About
A `CompactString` is a more memory efficient string type, that can store smaller strings on the stack, and transparently stores longer strings on the heap (aka a small string optimization).
It can mostly be used as a drop in replacement for `String` and are particularly useful in parsing, deserializing, or any other application where you may
have smaller strings.

### Properties
A `CompactString` specifically has the following properties:
  * `size_of::<CompactString>() == size_of::<String>()`
  * Stores up to 24 bytes on the stack
    * 12 bytes if running on a 32 bit architecture
  * Strings longer than 24 bytes are stored on the heap
  * `Clone` is `O(n)`
  * `From<String>` or `From<Box<str>>` re-uses underlying buffer
    * Eagerly inlines small strings
  * `O(1)` creation from `&'static str` with `CompactString::const_new`
  * Heap based string grows at a rate of 1.5x
    * The std library `String` grows at a rate of 2x
  * Space optimized for `Option<_>`
    * `size_of::<CompactString>() == size_of::<Option<CompactString>>()`
  * Uses [branchless instructions](https://en.algorithmica.org/hpc/pipelining/branchless/) for string accesses
  * Supports `no_std` environments

### Traits
This crate exposes two traits, `ToCompactString` and `CompactStringExt`.

#### `ToCompactString`
Provides the `to_compact_string(&self)` method for converting types into a `CompactString`. This trait is automatically implemented for all types that are `std::fmt::Display`, with specialized higher performance impls for:
* `u8`, `u16`, `u32`, `u64`, `usize`, `u128`
* `i8`, `i16`, `i32`, `i64`, `isize`, `i128`
* `f32`, `f64`
* `bool`, `char`
* `NonZeroU*`, `NonZeroI*`
* `String`, `CompactString`

#### `CompactStringExt`
Provides two methods `join_compact(seperator: impl AsRef<str>)` and `concat_compact()`. This trait is automatically implemented for all types that can be converted into an iterator and yield types that `impl AsRef<str>`. This allows you to join Vec's, slices, and any other collection to form `CompactString`s.

### Macros
This crate exposes one macro `format_compact!` that can be used to create `CompactString`s from arguments, like you can `String`s with the `std::format!` macro.

### Features
`compact_str` has the following optional features:
* `serde`, which implements [`Deserialize`](https://docs.rs/serde/1/serde/trait.Deserialize.html) and [`Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html) from the popular [`serde`](https://docs.rs/serde/1/serde/) crate, for `CompactString`
* `bytes`, which provides two methods `from_utf8_buf<B: Buf>(buf: &mut B)` and `from_utf8_buf_unchecked<B: Buf>(buf: &mut B)`, which allows for the creation of a `CompactString` from a [`bytes::Buf`](https://docs.rs/bytes/1/bytes/trait.Buf.html)
* `markup`, which implements [`Render`](https://docs.rs/markup/0.13/markup/trait.Render.html) trait, so `CompactString`s can be used in templates as HTML escaped strings
* `diesel`, which allows using CompactStrings in [`diesel`](https://diesel.rs/) text columns
* `sqlx-mysql` / `sqlx-postgres` / `sqlx-sqlite`, which allows using CompactStrings in [`sqlx`](https://github.com/launchbadge/sqlx) text columns
* `arbitrary`, which implements the [`arbitrary::Arbitrary`](https://docs.rs/arbitrary/1/arbitrary/trait.Arbitrary.html) trait for fuzzing
* `proptest`, which implements the [`proptest::arbitrary::Arbitrary`](https://docs.rs/proptest/1/proptest/arbitrary/trait.Arbitrary.html) trait for fuzzing
* `quickcheck`, which implements the [`quickcheck::Arbitrary`](https://docs.rs/quickcheck/1/quickcheck/trait.Arbitrary.html) trait for fuzzing
* `rkyv`, which implements [`rkyv::Archive`](https://docs.rs/rkyv/0.7/rkyv/trait.Archive.html), [`rkyv::Serialize`](https://docs.rs/rkyv/0.7/rkyv/trait.Serialize.html) and [`rkyv::Deserialize`](https://docs.rs/rkyv/0.7/rkyv/trait.Deserialize.html) for fast zero-copy serialization, interchangable with serialized Strings
* `smallvec`, provides the `into_bytes()` method which enables you to convert a `CompactString` into a byte vector, using [`smallvec::SmallVec`](https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html)

### How it works
Note: this explanation assumes a 64-bit architecture, for 32-bit architectures generally divide any number by 2.

Normally strings are stored on the heap since they're dynamically sized. In Rust a `String` consists of three fields, each of which are the size of a `usize`.
e.g. its layout is something like the following:

`String: [ ptr<8> | len<8> | cap<8> ]`
1. `ptr` is a pointer to a location on the heap that stores the string
2. `len` is the length of the string
3. `cap` is the total capacity of the buffer being pointed to

This results in 24 bytes being stored on the stack, 8 bytes for each field. Then the actual string is stored on the heap, usually with additional memory allocated to prevent re-allocating if the string is mutated.

The idea of `CompactString` is instead of storing metadata on the stack, just store the string itself. This way for smaller strings we save a bit of memory, and we
don't have to heap allocate so it's more performant. A `CompactString` is limited to 24 bytes (aka `size_of::<String>()`) so it won't ever use more memory than a
`String` would.

The memory layout of a `CompactString` looks something like:

`CompactString: [ buffer<23> | len<1> ]`

#### Memory Layout
Internally a `CompactString` has two variants:

1. **Inline**, a string <= 24 bytes long
2. **Heap** allocated, a string > 24 bytes long

We define a discriminant (aka track which variant we are) *within* the last byte, specifically:

1. `0b11111110` - All 1s with a trailing 0, indicates **heap** allocated
2. `0b11XXXXXX` - Two leading 1s, indicates **inline**, with the trailing 6 bits used to store the length

and the overall memory layout of a `CompactString` is:

1. `heap:   { ptr: NonNull<u8>, len: usize, cap: Capacity }`
2. `inline: { buffer: [u8; 24] }`

<sub>Both variants are 24 bytes long</sub>

For **heap** allocated strings we use a custom `HeapBuffer` which normally stores the capacity of the string on the stack, but also optionally allows us to store it on the heap. Since we use the last byte to track our discriminant, we only have 7 bytes to store the capacity, or 3 bytes on a 32-bit architecture. 7 bytes allows us to store a value up to `2^56`, aka 64 petabytes, while 3 bytes only allows us to store a value up to `2^24`, aka 16 megabytes.

For 64-bit architectures we always inline the capacity, because we can safely assume our strings will never be larger than 64 petabytes, but on 32-bit architectures, when creating or growing a `CompactString`, if the text is larger than 16MB then we move the capacity onto the heap.

We handle the capacity in this way for two reasons:
1. Users shouldn't have to pay for what they don't use. Meaning, in the _majority_ of cases the capacity of the buffer could easily fit into 7 or 3 bytes, so the user shouldn't have to pay the memory cost of storing the capacity on the heap, if they don't need to.
2. Allows us to convert `From<String>` in `O(1)` time, by taking the parts of a `String` (e.g. `ptr`, `len`, and `cap`) and using those to create a `CompactString`, without having to do any heap allocations. This is important when using `CompactString` in large codebases where you might have `CompactString` working alongside of `String`.

For **inline** strings we only have a 24 byte buffer on the stack. This might make you wonder how can we store a 24 byte long string, inline? Don't we also need to store the length somewhere?

To do this, we utilize the fact that the last byte of our string could only ever have a value in the range `[0, 192)`. We know this because all strings in Rust are valid [UTF-8](https://en.wikipedia.org/wiki/UTF-8), and the only valid byte pattern for the last byte of a UTF-8 character (and thus the possible last byte of a string) is `0b0XXXXXXX` aka `[0, 128)` or `0b10XXXXXX` aka `[128, 192)`. This leaves all values in `[192, 255]` as unused in our last byte. Therefore, we can use values in the range of `[192, 215]` to represent a length in the range of `[0, 23]`, and if our last byte has a value `< 192`, we know that's a UTF-8 character, and can interpret the length of our string as `24`.

Specifically, the last byte on the stack for a `CompactString` has the following uses:
* `[0, 191]` - Is the last byte of a UTF-8 char, the `CompactString` is stored on the stack and implicitly has a length of `24`
* `[192, 215]` - Denotes a length in the range of `[0, 23]`, this `CompactString` is stored on the stack.
* `216` - Denotes this `CompactString` is stored on the heap
* `217` - Denotes this `CompactString` stores a `&'static str`.
* `[218, 255]` - Unused, denotes e.g. the `None` variant for `Option<CompactString>`

### Testing
Strings and unicode can be quite messy, even further, we're working with things at the bit level. `compact_str` has an _extensive_ test suite comprised of unit testing, property testing, and fuzz testing, to ensure our invariants are upheld. We test across all major OSes (Windows, macOS, and Linux), architectures (64-bit and 32-bit), and endian-ness (big endian and little endian).

Fuzz testing is run with `libFuzzer`, `AFL++`, *and* `honggfuzz`, with `AFL++` running on both `x86_64` and `ARMv7` architectures. We test with [`miri`](https://github.com/rust-lang/miri) to catch cases of undefined behavior, and run all tests on every Rust compiler since `v1.60` to ensure support for our minimum supported Rust version (MSRV).

### `unsafe` code
`CompactString` uses a bit of unsafe code because we manually define what variant we are, so unlike an enum, the compiler can't guarantee what value is actually stored.
We also have some manually implemented heap data structures, i.e. `HeapBuffer`, and mess with bytes at a bit level, to make the most out of our resources.
That being said, uses of unsafe code in this library are constrained to only where *absolutely* necessary, and always documented with
`// SAFETY: <reason>`.

### Similar Crates
Storing strings on the stack is not a new idea, in fact there are a few other crates in the Rust ecosystem that do similar things, an incomplete list:

*  [`arcstr`](https://crates.io/crates/arcstr)
*  [`byteyarn`](https://crates.io/crates/byteyarn)
*  [`ecow`](https://crates.io/crates/ecow)
*  [`flexstr`](https://crates.io/crates/flexstr)
*  [`hipstr`](https://crates.io/crates/hipstr)
*  [`imstr`](https://crates.io/crates/imstr)
*  [`kstring`](https://crates.io/crates/kstring)
*  [`smartstring`](https://crates.io/crates/smartstring)

For a comparison of all these crates (and possibly more!) please see the [Rust String Benchmarks](https://github.com/rosetta-rs/string-rosetta-rs).

<br />
Thanks for readingme!

## Contents

- [Modules](#modules)
  - [`features`](#features)
  - [`macros`](#macros)
  - [`unicode_data`](#unicode-data)
  - [`repr`](#repr)
  - [`traits`](#traits)
- [Structs](#structs)
  - [`CompactString`](#compactstring)
  - [`Utf16Error`](#utf16error)
  - [`Drain`](#drain)
  - [`ReserveError`](#reserveerror)
- [Enums](#enums)
  - [`ToCompactStringError`](#tocompactstringerror)
- [Traits](#traits)
  - [`CompactStringExt`](#compactstringext)
  - [`ToCompactString`](#tocompactstring)
  - [`UnwrapWithMsg`](#unwrapwithmsg)
- [Functions](#functions)
  - [`convert_while_ascii`](#convert-while-ascii)
  - [`unwrap_with_msg_fail`](#unwrap-with-msg-fail)
- [Macros](#macros)
  - [`format_compact!`](#format-compact)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`features`](#features) | mod | A module that contains the implementations for optional features. |
| [`macros`](#macros) | mod |  |
| [`unicode_data`](#unicode-data) | mod | Adapted from <https://doc.rust-lang.org/nightly/src/core/unicode/unicode_data.rs.html> |
| [`repr`](#repr) | mod |  |
| [`traits`](#traits) | mod |  |
| [`CompactString`](#compactstring) | struct | A [`CompactString`] is a compact string type that can be used almost anywhere a [`String`] or [`str`] can be used. |
| [`Utf16Error`](#utf16error) | struct | A possible error value when converting a [`CompactString`] from a UTF-16 byte slice. |
| [`Drain`](#drain) | struct | An iterator over the exacted data by [`CompactString::drain()`]. |
| [`ReserveError`](#reserveerror) | struct | A possible error value if allocating or resizing a [`CompactString`] failed. |
| [`ToCompactStringError`](#tocompactstringerror) | enum | A possible error value if [`ToCompactString::try_to_compact_string()`] failed. |
| [`CompactStringExt`](#compactstringext) | trait |  |
| [`ToCompactString`](#tocompactstring) | trait |  |
| [`UnwrapWithMsg`](#unwrapwithmsg) | trait |  |
| [`convert_while_ascii`](#convert-while-ascii) | fn | Converts the bytes while the bytes are still ascii. |
| [`unwrap_with_msg_fail`](#unwrap-with-msg-fail) | fn |  |
| [`format_compact!`](#format-compact) | macro | Creates a `CompactString` using interpolation of runtime expressions. |

## Modules

- [`features`](features/index.md) — A module that contains the implementations for optional features. For example `serde` support
- [`macros`](macros/index.md)
- [`unicode_data`](unicode_data/index.md) — Adapted from
- [`repr`](repr/index.md)
- [`traits`](traits/index.md)

## Structs

### `CompactString`

```rust
struct CompactString(repr::Repr);
```

*Defined in [`compact_str-0.9.0/src/lib.rs:128`](../../.source_1765633015/compact_str-0.9.0/src/lib.rs#L128)*

A [`CompactString`](#compactstring) is a compact string type that can be used almost anywhere a
[`String`](../cargo_platform/index.md) or [`str`](../clap_builder/builder/str/index.md) can be used.

## Using `CompactString`
```rust
use compact_str::CompactString;
use std::collections::HashMap;

// CompactString auto derefs into a str so you can use all methods from `str`
// that take a `&self`
if CompactString::new("hello world!").is_ascii() {
    println!("we're all ASCII")
}

// You can use a CompactString in collections like you would a String or &str
let mut map: HashMap<CompactString, CompactString> = HashMap::new();

// directly construct a new `CompactString`
map.insert(CompactString::new("nyc"), CompactString::new("empire state building"));
// create a `CompactString` from a `&str`
map.insert("sf".into(), "transamerica pyramid".into());
// create a `CompactString` from a `String`
map.insert(String::from("sea").into(), String::from("space needle").into());

fn wrapped_print<T: AsRef<str>>(text: T) {
    println!("{}", text.as_ref());
}

// CompactString impls AsRef<str> and Borrow<str>, so it can be used anywhere
// that expects a generic string
if let Some(building) = map.get("nyc") {
    wrapped_print(building);
}

// CompactString can also be directly compared to a String or &str
assert_eq!(CompactString::new("chicago"), "chicago");
assert_eq!(CompactString::new("houston"), String::from("houston"));
```

# Converting from a `String`
It's important that a `CompactString` interops well with `String`, so you can easily use both in
your code base.

`CompactString` implements `From<String>` and operates in the following manner:
- Eagerly inlines the string, possibly dropping excess capacity
- Otherwise re-uses the same underlying buffer from `String`

```rust
use compact_str::CompactString;

// eagerly inlining
let short = String::from("hello world");
let short_c = CompactString::from(short);
assert!(!short_c.is_heap_allocated());

// dropping excess capacity
let mut excess = String::with_capacity(256);
excess.push_str("abc");

let excess_c = CompactString::from(excess);
assert!(!excess_c.is_heap_allocated());
assert!(excess_c.capacity() < 256);

// re-using the same buffer
let long = String::from("this is a longer string that will be heap allocated");

let long_ptr = long.as_ptr();
let long_len = long.len();
let long_cap = long.capacity();

let mut long_c = CompactString::from(long);
assert!(long_c.is_heap_allocated());

let cpt_ptr = long_c.as_ptr();
let cpt_len = long_c.len();
let cpt_cap = long_c.capacity();

// the original String and the CompactString point to the same place in memory, buffer re-use!
assert_eq!(cpt_ptr, long_ptr);
assert_eq!(cpt_len, long_len);
assert_eq!(cpt_cap, long_cap);
```

### Prevent Eagerly Inlining
A consequence of eagerly inlining is you then need to de-allocate the existing buffer, which
might not always be desirable if you're converting a very large amount of `String`s. If your
code is very sensitive to allocations, consider the `CompactString::from_string_buffer` API.

#### Implementations

- <span id="compactstring-new"></span>`fn new<T: AsRef<str>>(text: T) -> Self`

  Creates a new [`CompactString`](#compactstring) from any type that implements `AsRef<str>`.

  If the string is short enough, then it will be inlined on the stack!

  

  In a `static` or `const` context you can use the method `CompactString::const_new()`.

  

  # Examples

  

  ### Inlined

  ```rust

  use compact_str::CompactString;

  // We can inline strings up to 12 characters long on 32-bit architectures...

  #[cfg(target_pointer_width = "32")]

  let s = "i'm 12 chars";

  // ...and up to 24 characters on 64-bit architectures!

  #[cfg(target_pointer_width = "64")]

  let s = "i am 24 characters long!";

  

  let compact = CompactString::new(&s);

  

  assert_eq!(compact, s);

  // we are not allocated on the heap!

  assert!(!compact.is_heap_allocated());

  ```

  

  ### Heap

  ```rust

  use compact_str::CompactString;

  // For longer strings though, we get allocated on the heap

  let long = "I am a longer string that will be allocated on the heap";

  let compact = CompactString::new(long);

  

  assert_eq!(compact, long);

  // we are allocated on the heap!

  assert!(compact.is_heap_allocated());

  ```

  

  ### Creation

  ```rust

  use compact_str::CompactString;

  

  // Using a `&'static str`

  let s = "hello world!";

  let hello = CompactString::new(&s);

  

  // Using a `String`

  let u = String::from("🦄🌈");

  let unicorn = CompactString::new(u);

  

  // Using a `Box<str>`

  let b: Box<str> = String::from("📦📦📦").into_boxed_str();

  let boxed = CompactString::new(&b);

  ```

- <span id="compactstring-try-new"></span>`fn try_new<T: AsRef<str>>(text: T) -> Result<Self, ReserveError>` — [`ReserveError`](#reserveerror)

  Fallible version of `CompactString::new()`

  

  This method won't panic if the system is out-of-memory, but return an [`ReserveError`](#reserveerror).

  Otherwise it behaves the same as `CompactString::new()`.

- <span id="compactstring-const-new"></span>`const fn const_new(text: &'static str) -> Self`

  Creates a new inline [`CompactString`](#compactstring) from `&'static str` at compile time.

  Complexity: O(1). As an optimization, short strings get inlined.

  

  In a dynamic context you can use the method `CompactString::new()`.

  

  # Examples

  ```rust

  use compact_str::CompactString;

  

  const DEFAULT_NAME: CompactString = CompactString::const_new("untitled");

  ```

- <span id="compactstring-as-static-str"></span>`const fn as_static_str(&self) -> Option<&'static str>`

  Get back the `&'static str` constructed by `CompactString::const_new`.

  

  If the string was short enough that it could be inlined, then it was inline, and

  this method will return `None`.

  

  # Examples

  ```rust

  use compact_str::CompactString;

  

  const DEFAULT_NAME: CompactString =

      CompactString::const_new("That is not dead which can eternal lie.");

  assert_eq!(

      DEFAULT_NAME.as_static_str().unwrap(),

      "That is not dead which can eternal lie.",

  );

  ```

- <span id="compactstring-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Creates a new empty [`CompactString`](#compactstring) with the capacity to fit at least `capacity` bytes.

  

  A `CompactString` will inline strings on the stack, if they're small enough. Specifically,

  if the string has a length less than or equal to `std::mem::size_of::<String>` bytes

  then it will be inlined. This also means that `CompactString`s have a minimum capacity

  of `std::mem::size_of::<String>`.

  

  # Panics

  

  This method panics if the system is out-of-memory.

  Use `CompactString::try_with_capacity()` if you want to handle such a problem manually.

  

  # Examples

  

  ### "zero" Capacity

  ```rust

  use compact_str::CompactString;

  // Creating a CompactString with a capacity of 0 will create

  // one with capacity of std::mem::size_of::<String>();

  let empty = CompactString::with_capacity(0);

  let min_size = std::mem::size_of::<String>();

  

  assert_eq!(empty.capacity(), min_size);

  assert_ne!(0, min_size);

  assert!(!empty.is_heap_allocated());

  ```

  

  ### Max Inline Size

  ```rust

  use compact_str::CompactString;

  // Creating a CompactString with a capacity of std::mem::size_of::<String>()

  // will not heap allocate.

  let str_size = std::mem::size_of::<String>();

  let empty = CompactString::with_capacity(str_size);

  

  assert_eq!(empty.capacity(), str_size);

  assert!(!empty.is_heap_allocated());

  ```

  

  ### Heap Allocating

  ```rust

  use compact_str::CompactString;

  // If you create a `CompactString` with a capacity greater than

  // `std::mem::size_of::<String>`, it will heap allocated. For heap

  // allocated strings we have a minimum capacity

  

  const MIN_HEAP_CAPACITY: usize = std::mem::size_of::<usize>() * 4;

  

  let heap_size = std::mem::size_of::<String>() + 1;

  let empty = CompactString::with_capacity(heap_size);

  

  assert_eq!(empty.capacity(), MIN_HEAP_CAPACITY);

  assert!(empty.is_heap_allocated());

  ```

- <span id="compactstring-try-with-capacity"></span>`fn try_with_capacity(capacity: usize) -> Result<Self, ReserveError>` — [`ReserveError`](#reserveerror)

  Fallible version of `CompactString::with_capacity()`

  

  This method won't panic if the system is out-of-memory, but return an [`ReserveError`](#reserveerror).

  Otherwise it behaves the same as `CompactString::with_capacity()`.

- <span id="compactstring-from-utf8"></span>`fn from_utf8<B: AsRef<[u8]>>(buf: B) -> Result<Self, Utf8Error>`

  Convert a slice of bytes into a [`CompactString`](#compactstring).

  

  A [`CompactString`](#compactstring) is a contiguous collection of bytes (`u8`s) that is valid [`UTF-8`](https://en.wikipedia.org/wiki/UTF-8).

  This method converts from an arbitrary contiguous collection of bytes into a

  [`CompactString`](#compactstring), failing if the provided bytes are not `UTF-8`.

  

  Note: If you want to create a [`CompactString`](#compactstring) from a non-contiguous collection of bytes,

  enable the `bytes` feature of this crate, and see `CompactString::from_utf8_buf`

  

  # Examples

  ### Valid UTF-8

  ```rust

  use compact_str::CompactString;

  let bytes = vec![240, 159, 166, 128, 240, 159, 146, 175];

  let compact = CompactString::from_utf8(bytes).expect("valid UTF-8");

  

  assert_eq!(compact, "🦀💯");

  ```

  

  ### Invalid UTF-8

  ```rust

  use compact_str::CompactString;

  let bytes = vec![255, 255, 255];

  let result = CompactString::from_utf8(bytes);

  

  assert!(result.is_err());

  ```

- <span id="compactstring-from-utf8-unchecked"></span>`unsafe fn from_utf8_unchecked<B: AsRef<[u8]>>(buf: B) -> Self`

  Converts a vector of bytes to a [`CompactString`](#compactstring) without checking that the string contains

  valid UTF-8.

  

  See the safe version, `CompactString::from_utf8`, for more details.

  

  # Safety

  

  * The contents pased to this method must be valid UTF-8.

  

  It's very important that this constraint is upheld because the internals of a

  [`CompactString`](#compactstring) (e.g. determing an inline string versus a heap allocated string) rely on

  the [`CompactString`](#compactstring) containing valid UTF-8. If this constraint is violated any further

  use of the returned [`CompactString`](#compactstring) (including dropping it) can cause undefined behavior.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  // some bytes, in a vector

  let sparkle_heart = vec![240, 159, 146, 150];

  

  let sparkle_heart = unsafe {

      CompactString::from_utf8_unchecked(sparkle_heart)

  };

  

  assert_eq!("💖", sparkle_heart);

  ```

- <span id="compactstring-from-utf16"></span>`fn from_utf16<B: AsRef<[u16]>>(buf: B) -> Result<Self, Utf16Error>` — [`Utf16Error`](#utf16error)

  Decode a [`UTF-16`](https://en.wikipedia.org/wiki/UTF-16) slice of bytes into a

  [`CompactString`](#compactstring), returning an `Err` if the slice contains any invalid data.

  

  # Examples

  ### Valid UTF-16

  ```rust

  use compact_str::CompactString;

  let buf: &[u16] = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063];

  let compact = CompactString::from_utf16(buf).unwrap();

  

  assert_eq!(compact, "𝄞music");

  ```

  

  ### Invalid UTF-16

  ```rust

  use compact_str::CompactString;

  let buf: &[u16] = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];

  let res = CompactString::from_utf16(buf);

  

  assert!(res.is_err());

  ```

- <span id="compactstring-from-utf16-lossy"></span>`fn from_utf16_lossy<B: AsRef<[u16]>>(buf: B) -> Self`

  Decode a UTF-16–encoded slice `v` into a `CompactString`, replacing invalid data with

  the replacement character (`U+FFFD`), �.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  // 𝄞mus<invalid>ic<invalid>

  let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,

            0x0073, 0xDD1E, 0x0069, 0x0063,

            0xD834];

  

  assert_eq!(CompactString::from("𝄞mus\u{FFFD}ic\u{FFFD}"),

             CompactString::from_utf16_lossy(v));

  ```

- <span id="compactstring-len"></span>`fn len(&self) -> usize`

  Returns the length of the [`CompactString`](#compactstring) in `bytes`, not [`char`](../unicode_normalization/char/index.md)s or graphemes.

  

  When using `UTF-8` encoding (which all strings in Rust do) a single character will be 1 to 4

  bytes long, therefore the return value of this method might not be what a human considers

  the length of the string.

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let ascii = CompactString::new("hello world");

  assert_eq!(ascii.len(), 11);

  

  let emoji = CompactString::new("👱");

  assert_eq!(emoji.len(), 4);

  ```

- <span id="compactstring-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the [`CompactString`](#compactstring) has a length of 0, `false` otherwise

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let mut msg = CompactString::new("");

  assert!(msg.is_empty());

  

  // add some characters

  msg.push_str("hello reader!");

  assert!(!msg.is_empty());

  ```

- <span id="compactstring-capacity"></span>`fn capacity(&self) -> usize`

  Returns the capacity of the [`CompactString`](#compactstring), in bytes.

  

  # Note

  * A `CompactString` will always have a capacity of at least `std::mem::size_of::<String>()`

  

  # Examples

  ### Minimum Size

  ```rust

  use compact_str::CompactString;

  let min_size = std::mem::size_of::<String>();

  let compact = CompactString::new("");

  

  assert!(compact.capacity() >= min_size);

  ```

  

  ### Heap Allocated

  ```rust

  use compact_str::CompactString;

  let compact = CompactString::with_capacity(128);

  assert_eq!(compact.capacity(), 128);

  ```

- <span id="compactstring-reserve"></span>`fn reserve(&mut self, additional: usize)`

  Ensures that this [`CompactString`](#compactstring)'s capacity is at least `additional` bytes longer than

  its length. The capacity may be increased by more than `additional` bytes if it chooses,

  to prevent frequent reallocations.

  

  # Note

  * A `CompactString` will always have at least a capacity of `std::mem::size_of::<String>()`

  * Reserving additional bytes may cause the `CompactString` to become heap allocated

  

  # Panics

  This method panics if the new capacity overflows `usize` or if the system is out-of-memory.

  Use `CompactString::try_reserve()` if you want to handle such a problem manually.

  

  # Examples

  ```rust

  use compact_str::CompactString;

  

  const WORD: usize = std::mem::size_of::<usize>();

  let mut compact = CompactString::default();

  assert!(compact.capacity() >= (WORD * 3) - 1);

  

  compact.reserve(200);

  assert!(compact.is_heap_allocated());

  assert!(compact.capacity() >= 200);

  ```

- <span id="compactstring-try-reserve"></span>`fn try_reserve(&mut self, additional: usize) -> Result<(), ReserveError>` — [`ReserveError`](#reserveerror)

  Fallible version of `CompactString::reserve()`

  

  This method won't panic if the system is out-of-memory, but return an [`ReserveError`](#reserveerror)

  Otherwise it behaves the same as `CompactString::reserve()`.

- <span id="compactstring-as-str"></span>`fn as_str(&self) -> &str`

  Returns a string slice containing the entire [`CompactString`](#compactstring).

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let s = CompactString::new("hello");

  

  assert_eq!(s.as_str(), "hello");

  ```

- <span id="compactstring-as-mut-str"></span>`fn as_mut_str(&mut self) -> &mut str`

  Returns a mutable string slice containing the entire [`CompactString`](#compactstring).

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("hello");

  s.as_mut_str().make_ascii_uppercase();

  

  assert_eq!(s.as_str(), "HELLO");

  ```

- <span id="compactstring-spare-capacity-mut"></span>`unsafe fn spare_capacity_mut(&mut self) -> &mut [mem::MaybeUninit<u8>]`

- <span id="compactstring-as-bytes"></span>`fn as_bytes(&self) -> &[u8]`

  Returns a byte slice of the [`CompactString`](#compactstring)'s contents.

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let s = CompactString::new("hello");

  

  assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());

  ```

- <span id="compactstring-as-mut-bytes"></span>`unsafe fn as_mut_bytes(&mut self) -> &mut [u8]`

  Provides a mutable reference to the underlying buffer of bytes.

  

  # Safety

  * All Rust strings, including `CompactString`, must be valid UTF-8. The caller must

    guarantee that any modifications made to the underlying buffer are valid UTF-8.

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("hello");

  

  let slice = unsafe { s.as_mut_bytes() };

  // copy bytes into our string

  slice[5..11].copy_from_slice(" world".as_bytes());

  // set the len of the string

  unsafe { s.set_len(11) };

  

  assert_eq!(s, "hello world");

  ```

- <span id="compactstring-push"></span>`fn push(&mut self, ch: char)`

  Appends the given [`char`](../unicode_normalization/char/index.md) to the end of this [`CompactString`](#compactstring).

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("foo");

  

  s.push('b');

  s.push('a');

  s.push('r');

  

  assert_eq!("foobar", s);

  ```

- <span id="compactstring-pop"></span>`fn pop(&mut self) -> Option<char>`

  Removes the last character from the [`CompactString`](#compactstring) and returns it.

  Returns `None` if this [`CompactString`](#compactstring) is empty.

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("abc");

  

  assert_eq!(s.pop(), Some('c'));

  assert_eq!(s.pop(), Some('b'));

  assert_eq!(s.pop(), Some('a'));

  

  assert_eq!(s.pop(), None);

  ```

- <span id="compactstring-push-str"></span>`fn push_str(&mut self, s: &str)`

  Appends a given string slice onto the end of this [`CompactString`](#compactstring)

  

  # Examples

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("abc");

  

  s.push_str("123");

  

  assert_eq!("abc123", s);

  ```

- <span id="compactstring-remove"></span>`fn remove(&mut self, idx: usize) -> char`

  Removes a [`char`](../unicode_normalization/char/index.md) from this [`CompactString`](#compactstring) at a byte position and returns it.

  

  This is an *O*(*n*) operation, as it requires copying every element in the

  buffer.

  

  # Panics

  

  Panics if `idx` is larger than or equal to the [`CompactString`](#compactstring)'s length,

  or if it does not lie on a [`char`](../unicode_normalization/char/index.md) boundary.

  

  # Examples

  

  ### Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut c = CompactString::from("hello world");

  

  assert_eq!(c.remove(0), 'h');

  assert_eq!(c, "ello world");

  

  assert_eq!(c.remove(5), 'w');

  assert_eq!(c, "ello orld");

  ```

  

  ### Past total length:

  

  ```should_panic

  use compact_str::CompactString;

  let mut c = CompactString::from("hello there!");

  c.remove(100);

  ```

  

  ### Not on char boundary:

  

  ```should_panic

  use compact_str::CompactString;

  let mut c = CompactString::from("🦄");

  c.remove(1);

  ```

- <span id="compactstring-set-len"></span>`unsafe fn set_len(&mut self, new_len: usize)`

  Forces the length of the [`CompactString`](#compactstring) to `new_len`.

  

  This is a low-level operation that maintains none of the normal invariants for

  `CompactString`. If you want to modify the `CompactString` you should use methods like

  `push`, `push_str` or `pop`.

  

  # Safety

  * `new_len` must be less than or equal to `capacity()`

  * The elements at `old_len..new_len` must be initialized

- <span id="compactstring-is-heap-allocated"></span>`fn is_heap_allocated(&self) -> bool`

  Returns whether or not the [`CompactString`](#compactstring) is heap allocated.

  

  # Examples

  ### Inlined

  ```rust

  use compact_str::CompactString;

  let hello = CompactString::new("hello world");

  

  assert!(!hello.is_heap_allocated());

  ```

  

  ### Heap Allocated

  ```rust

  use compact_str::CompactString;

  let msg = CompactString::new("this message will self destruct in 5, 4, 3, 2, 1 💥");

  

  assert!(msg.is_heap_allocated());

  ```

- <span id="compactstring-ensure-range"></span>`fn ensure_range(&self, range: impl RangeBounds<usize>) -> (usize, usize)`

  Ensure that the given range is inside the set data, and that no codepoints are split.

  

  Returns the range `start..end` as a tuple.

- <span id="compactstring-replace-range"></span>`fn replace_range(&mut self, range: impl RangeBounds<usize>, replace_with: &str)`

  Removes the specified range in the [`CompactString`](#compactstring),

  and replaces it with the given string.

  The given string doesn't need to be the same length as the range.

  

  # Panics

  

  Panics if the starting point or end point do not lie on a [`char`](../unicode_normalization/char/index.md)

  boundary, or if they're out of bounds.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("Hello, world!");

  

  s.replace_range(7..12, "WORLD");

  assert_eq!(s, "Hello, WORLD!");

  

  s.replace_range(7..=11, "you");

  assert_eq!(s, "Hello, you!");

  

  s.replace_range(5.., "! Is it me you're looking for?");

  assert_eq!(s, "Hello! Is it me you're looking for?");

  ```

- <span id="compactstring-replace-range-same-size"></span>`unsafe fn replace_range_same_size(&mut self, start: usize, end: usize, replace_with: &str)`

  Replace into the same size.

- <span id="compactstring-replace-range-shrink"></span>`unsafe fn replace_range_shrink(&mut self, start: usize, end: usize, replace_with: &str)`

  Replace, so self.len() gets smaller.

- <span id="compactstring-replace-range-grow"></span>`unsafe fn replace_range_grow(&mut self, start: usize, end: usize, replace_with: &str)`

  Replace, so self.len() gets bigger.

- <span id="compactstring-repeat"></span>`fn repeat(&self, n: usize) -> Self`

  Creates a new [`CompactString`](#compactstring) by repeating a string `n` times.

  

  # Panics

  

  This function will panic if the capacity would overflow.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  assert_eq!(CompactString::new("abc").repeat(4), CompactString::new("abcabcabcabc"));

  ```

  

  A panic upon overflow:

  

  ```should_panic

  use compact_str::CompactString;

  

  // this will panic at runtime

  let huge = CompactString::new("0123456789abcdef").repeat(usize::MAX);

  ```

- <span id="compactstring-truncate"></span>`fn truncate(&mut self, new_len: usize)`

  Truncate the [`CompactString`](#compactstring) to a shorter length.

  

  If the length of the [`CompactString`](#compactstring) is less or equal to `new_len`, the call is a no-op.

  

  Calling this function does not change the capacity of the [`CompactString`](#compactstring).

  

  # Panics

  

  Panics if the new end of the string does not lie on a [`char`](../unicode_normalization/char/index.md) boundary.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("Hello, world!");

  s.truncate(5);

  assert_eq!(s, "Hello");

  ```

- <span id="compactstring-as-ptr"></span>`fn as_ptr(&self) -> *const u8`

  Converts a [`CompactString`](#compactstring) to a raw pointer.

- <span id="compactstring-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut u8`

  Converts a mutable [`CompactString`](#compactstring) to a raw pointer.

- <span id="compactstring-insert-str"></span>`fn insert_str(&mut self, idx: usize, string: &str)`

  Insert string character at an index.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("Hello!");

  s.insert_str(5, ", world");

  assert_eq!(s, "Hello, world!");

  ```

- <span id="compactstring-insert"></span>`fn insert(&mut self, idx: usize, ch: char)`

  Insert a character at an index.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("Hello world!");

  s.insert(5, ',');

  assert_eq!(s, "Hello, world!");

  ```

- <span id="compactstring-clear"></span>`fn clear(&mut self)`

  Reduces the length of the [`CompactString`](#compactstring) to zero.

  

  Calling this function does not change the capacity of the [`CompactString`](#compactstring).

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("Rust is the most loved language on Stackoverflow!");

  assert_eq!(s.capacity(), 49);

  

  s.clear();

  

  assert_eq!(s, "");

  assert_eq!(s.capacity(), 49);

  ```

- <span id="compactstring-split-off"></span>`fn split_off(&mut self, at: usize) -> Self`

  Split the [`CompactString`](#compactstring) into at the given byte index.

  

  Calling this function does not change the capacity of the [`CompactString`](#compactstring), unless the

  [`CompactString`](#compactstring) is backed by a `&'static str`.

  

  # Panics

  

  Panics if `at` does not lie on a [`char`](../unicode_normalization/char/index.md) boundary.

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::const_new("Hello, world!");

  let w = s.split_off(5);

  

  assert_eq!(w, ", world!");

  assert_eq!(s, "Hello");

  ```

- <span id="compactstring-drain"></span>`fn drain(&mut self, range: impl RangeBounds<usize>) -> Drain<'_>` — [`Drain`](#drain)

  Remove a range from the [`CompactString`](#compactstring), and return it as an iterator.

  

  Calling this function does not change the capacity of the [`CompactString`](#compactstring).

  

  # Panics

  

  Panics if the start or end of the range does not lie on a [`char`](../unicode_normalization/char/index.md) boundary.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::new("Hello, world!");

  

  let mut d = s.drain(5..12);

  assert_eq!(d.next(), Some(','));   // iterate over the extracted data

  assert_eq!(d.as_str(), " world"); // or get the whole data as &str

  

  // The iterator keeps a reference to `s`, so you have to drop() the iterator,

  // before you can access `s` again.

  drop(d);

  assert_eq!(s, "Hello!");

  ```

- <span id="compactstring-shrink-to"></span>`fn shrink_to(&mut self, min_capacity: usize)`

  Shrinks the capacity of this [`CompactString`](#compactstring) with a lower bound.

  

  The resulting capactity is never less than the size of 3×`usize`,

  i.e. the capacity than can be inlined.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::with_capacity(100);

  assert_eq!(s.capacity(), 100);

  

  // if the capacity was already bigger than the argument, the call is a no-op

  s.shrink_to(100);

  assert_eq!(s.capacity(), 100);

  

  s.shrink_to(50);

  assert_eq!(s.capacity(), 50);

  

  // if the string can be inlined, it is

  s.shrink_to(10);

  assert_eq!(s.capacity(), 3 * std::mem::size_of::<usize>());

  ```

- <span id="compactstring-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

  Shrinks the capacity of this [`CompactString`](#compactstring) to match its length.

  

  The resulting capactity is never less than the size of 3×`usize`,

  i.e. the capacity than can be inlined.

  

  This method is effectively the same as calling `string.shrink_to(0)`.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::from("This is a string with more than 24 characters.");

  

  s.reserve(100);

  assert!(s.capacity() >= 100);

  

   s.shrink_to_fit();

  assert_eq!(s.len(), s.capacity());

  ```

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::from("short string");

  

  s.reserve(100);

  assert!(s.capacity() >= 100);

  

  s.shrink_to_fit();

  assert_eq!(s.capacity(), 3 * std::mem::size_of::<usize>());

  ```

- <span id="compactstring-retain"></span>`fn retain(&mut self, predicate: impl FnMut(char) -> bool)`

  Retains only the characters specified by the predicate.

  

  The method iterates over the characters in the string and calls the `predicate`.

  

  If the `predicate` returns `false`, then the character gets removed.

  If the `predicate` returns `true`, then the character is kept.

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  let mut s = CompactString::from("äb𝄞d€");

  

  let keep = [false, true, true, false, true];

  let mut iter = keep.iter();

  s.retain(|_| *iter.next().unwrap());

  

  assert_eq!(s, "b𝄞€");

  ```

- <span id="compactstring-from-utf8-lossy"></span>`fn from_utf8_lossy(v: &[u8]) -> Self`

  Decode a bytes slice as UTF-8 string, replacing any illegal codepoints

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  let chess_knight = b"\xf0\x9f\xa8\x84";

  

  assert_eq!(

      "🨄",

      CompactString::from_utf8_lossy(chess_knight),

  );

  

  // For valid UTF-8 slices, this is the same as:

  assert_eq!(

      "🨄",

      CompactString::new(std::str::from_utf8(chess_knight).unwrap()),

  );

  ```

  

  Incorrect bytes:

  

  ```rust

  use compact_str::CompactString;

  let broken = b"\xf0\x9f\xc8\x84";

  

  assert_eq!(

      "�Ȅ",

      CompactString::from_utf8_lossy(broken),

  );

  

  // For invalid UTF-8 slices, this is an optimized implemented for:

  assert_eq!(

      "�Ȅ",

      CompactString::from(String::from_utf8_lossy(broken)),

  );

  ```

- <span id="compactstring-from-utf16x"></span>`fn from_utf16x(v: &[u8], from_int: impl Fn(u16) -> u16, from_bytes: impl Fn([u8; 2]) -> u16) -> Result<Self, Utf16Error>` — [`Utf16Error`](#utf16error)

- <span id="compactstring-from-utf16x-lossy"></span>`fn from_utf16x_lossy(v: &[u8], from_int: impl Fn(u16) -> u16, from_bytes: impl Fn([u8; 2]) -> u16) -> Self`

- <span id="compactstring-from-utf16le"></span>`fn from_utf16le(v: impl AsRef<[u8]>) -> Result<Self, Utf16Error>` — [`Utf16Error`](#utf16error)

  Decode a slice of bytes as UTF-16 encoded string, in little endian.

  

  # Errors

  

  If the slice has an odd number of bytes, or if it did not contain valid UTF-16 characters,

  a [`Utf16Error`](#utf16error) is returned.

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  const DANCING_MEN: &[u8] = b"\x3d\xd8\x6f\xdc\x0d\x20\x42\x26\x0f\xfe";

  let dancing_men = CompactString::from_utf16le(DANCING_MEN).unwrap();

  assert_eq!(dancing_men, "👯‍♂️");

  ```

- <span id="compactstring-from-utf16be"></span>`fn from_utf16be(v: impl AsRef<[u8]>) -> Result<Self, Utf16Error>` — [`Utf16Error`](#utf16error)

  Decode a slice of bytes as UTF-16 encoded string, in big endian.

  

  # Errors

  

  If the slice has an odd number of bytes, or if it did not contain valid UTF-16 characters,

  a [`Utf16Error`](#utf16error) is returned.

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  const DANCING_WOMEN: &[u8] = b"\xd8\x3d\xdc\x6f\x20\x0d\x26\x40\xfe\x0f";

  let dancing_women = CompactString::from_utf16be(DANCING_WOMEN).unwrap();

  assert_eq!(dancing_women, "👯‍♀️");

  ```

- <span id="compactstring-from-utf16le-lossy"></span>`fn from_utf16le_lossy(v: impl AsRef<[u8]>) -> Self`

  Lossy decode a slice of bytes as UTF-16 encoded string, in little endian.

  

  In this context "lossy" means that any broken characters in the input are replaced by the

  \<REPLACEMENT CHARACTER\> `'�'`. Please notice that, unlike UTF-8, UTF-16 is not self

  synchronizing. I.e. if a byte in the input is dropped, all following data is broken.

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  // A "random" bit was flipped in the 4th byte:

  const DANCING_MEN: &[u8] = b"\x3d\xd8\x6f\xfc\x0d\x20\x42\x26\x0f\xfe";

  let dancing_men = CompactString::from_utf16le_lossy(DANCING_MEN);

  assert_eq!(dancing_men, "�\u{fc6f}\u{200d}♂️");

  ```

- <span id="compactstring-from-utf16be-lossy"></span>`fn from_utf16be_lossy(v: impl AsRef<[u8]>) -> Self`

  Lossy decode a slice of bytes as UTF-16 encoded string, in big endian.

  

  In this context "lossy" means that any broken characters in the input are replaced by the

  \<REPLACEMENT CHARACTER\> `'�'`. Please notice that, unlike UTF-8, UTF-16 is not self

  synchronizing. I.e. if a byte in the input is dropped, all following data is broken.

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  // A "random" bit was flipped in the 9th byte:

  const DANCING_WOMEN: &[u8] = b"\xd8\x3d\xdc\x6f\x20\x0d\x26\x40\xde\x0f";

  let dancing_women = CompactString::from_utf16be_lossy(DANCING_WOMEN);

  assert_eq!(dancing_women, "👯\u{200d}♀�");

  ```

- <span id="compactstring-into-string"></span>`fn into_string(self) -> String`

  Convert the [`CompactString`](#compactstring) into a [`String`](../cargo_platform/index.md).

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  let s = CompactString::new("Hello world");

  let s = s.into_string();

  assert_eq!(s, "Hello world");

  ```

- <span id="compactstring-from-string-buffer"></span>`fn from_string_buffer(s: String) -> Self`

  Convert a [`String`](../cargo_platform/index.md) into a [`CompactString`](#compactstring) _without inlining_.

  

  Note: You probably don't need to use this method, instead you should use `From<String>`

  which is implemented for [`CompactString`](#compactstring).

  

  This method exists incase your code is very sensitive to memory allocations. Normally when

  converting a [`String`](../cargo_platform/index.md) to a [`CompactString`](#compactstring) we'll inline short strings onto the stack.

  But this results in [`Drop`](../gimli/index.md)-ing the original [`String`](../cargo_platform/index.md), which causes memory it owned on

  the heap to be deallocated. Instead when using this method, we always reuse the buffer that

  was previously owned by the [`String`](../cargo_platform/index.md), so no trips to the allocator are needed.

  

  # Examples

  

  ### Short Strings

  ```rust

  use compact_str::CompactString;

  

  let short = "hello world".to_string();

  let c_heap = CompactString::from_string_buffer(short);

  

  // using CompactString::from_string_buffer, we'll re-use the String's underlying buffer

  assert!(c_heap.is_heap_allocated());

  

  // note: when Clone-ing a short heap allocated string, we'll eagerly inline at that point

  let c_inline = c_heap.clone();

  assert!(!c_inline.is_heap_allocated());

  

  assert_eq!(c_heap, c_inline);

  ```

  

  ### Longer Strings

  ```rust

  use compact_str::CompactString;

  

  let x = "longer string that will be on the heap".to_string();

  let c1 = CompactString::from(x);

  

  let y = "longer string that will be on the heap".to_string();

  let c2 = CompactString::from_string_buffer(y);

  

  // for longer strings, we re-use the underlying String's buffer in both cases

  assert!(c1.is_heap_allocated());

  assert!(c2.is_heap_allocated());

  ```

  

  ### Buffer Re-use

  ```rust

  use compact_str::CompactString;

  

  let og = "hello world".to_string();

  let og_addr = og.as_ptr();

  

  let mut c = CompactString::from_string_buffer(og);

  let ex_addr = c.as_ptr();

  

  // When converting to/from String and CompactString with from_string_buffer we always re-use

  // the same underlying allocated memory/buffer

  assert_eq!(og_addr, ex_addr);

  

  let long = "this is a long string that will be on the heap".to_string();

  let long_addr = long.as_ptr();

  

  let mut long_c = CompactString::from(long);

  let long_ex_addr = long_c.as_ptr();

  

  // When converting to/from String and CompactString with From<String>, we'll also re-use the

  // underlying buffer, if the string is long, otherwise when converting to CompactString we

  // eagerly inline

  assert_eq!(long_addr, long_ex_addr);

  ```

- <span id="compactstring-to-ascii-lowercase"></span>`fn to_ascii_lowercase(&self) -> Self`

  Returns a copy of this string where each character is mapped to its

  ASCII lower case equivalent.

  

  ASCII letters 'A' to 'Z' are mapped to 'a' to 'z',

  but non-ASCII letters are unchanged.

  

  To lowercase the value in-place, use `str::make_ascii_lowercase`.

  

  To lowercase ASCII characters in addition to non-ASCII characters, use

  `CompactString::to_lowercase`.

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  let s = CompactString::new("Grüße, Jürgen ❤");

  

  assert_eq!("grüße, jürgen ❤", s.to_ascii_lowercase());

  ```

- <span id="compactstring-to-ascii-uppercase"></span>`fn to_ascii_uppercase(&self) -> Self`

  Returns a copy of this string where each character is mapped to its

  ASCII upper case equivalent.

  

  ASCII letters 'a' to 'z' are mapped to 'A' to 'Z',

  but non-ASCII letters are unchanged.

  

  To uppercase the value in-place, use `str::make_ascii_uppercase`.

  

  To uppercase ASCII characters in addition to non-ASCII characters, use

  `CompactString::to_uppercase`.

  

  # Examples

  

  ```rust

  use compact_str::CompactString;

  let s = CompactString::new("Grüße, Jürgen ❤");

  

  assert_eq!("GRüßE, JüRGEN ❤", s.to_ascii_uppercase());

  ```

- <span id="compactstring-to-lowercase"></span>`fn to_lowercase(&self) -> Self`

  Returns the lowercase equivalent of this string slice, as a new [`CompactString`](#compactstring).

  

  'Lowercase' is defined according to the terms of the Unicode Derived Core Property

  `Lowercase`.

  

  Since some characters can expand into multiple characters when changing

  the case, this function returns a [`CompactString`](#compactstring) instead of modifying the

  parameter in-place.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let s = CompactString::new("HELLO");

  

  assert_eq!("hello", s.to_lowercase());

  ```

  

  A tricky example, with sigma:

  

  ```rust

  use compact_str::CompactString;

  let sigma = CompactString::new("Σ");

  

  assert_eq!("σ", sigma.to_lowercase());

  

  // but at the end of a word, it's ς, not σ:

  let odysseus = CompactString::new("ὈΔΥΣΣΕΎΣ");

  

  assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());

  ```

  

  Languages without case are not changed:

  

  ```rust

  use compact_str::CompactString;

  let new_year = CompactString::new("农历新年");

  

  assert_eq!(new_year, new_year.to_lowercase());

  ```

- <span id="compactstring-from-str-to-lowercase"></span>`fn from_str_to_lowercase(input: &str) -> Self`

  Returns the lowercase equivalent of this string slice, as a new [`CompactString`](#compactstring).

  

  'Lowercase' is defined according to the terms of the Unicode Derived Core Property

  `Lowercase`.

  

  Since some characters can expand into multiple characters when changing

  the case, this function returns a [`CompactString`](#compactstring) instead of modifying the

  parameter in-place.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  

  assert_eq!("hello", CompactString::from_str_to_lowercase("HELLO"));

  ```

  

  A tricky example, with sigma:

  

  ```rust

  use compact_str::CompactString;

  

  assert_eq!("σ", CompactString::from_str_to_lowercase("Σ"));

  

  // but at the end of a word, it's ς, not σ:

  assert_eq!("ὀδυσσεύς", CompactString::from_str_to_lowercase("ὈΔΥΣΣΕΎΣ"));

  ```

  

  Languages without case are not changed:

  

  ```rust

  use compact_str::CompactString;

  

  let new_year = "农历新年";

  assert_eq!(new_year, CompactString::from_str_to_lowercase(new_year));

  ```

- <span id="compactstring-to-uppercase"></span>`fn to_uppercase(&self) -> Self`

  Returns the uppercase equivalent of this string slice, as a new [`CompactString`](#compactstring).

  

  'Uppercase' is defined according to the terms of the Unicode Derived Core Property

  `Uppercase`.

  

  Since some characters can expand into multiple characters when changing

  the case, this function returns a [`CompactString`](#compactstring) instead of modifying the

  parameter in-place.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  let s = CompactString::new("hello");

  

  assert_eq!("HELLO", s.to_uppercase());

  ```

  

  Scripts without case are not changed:

  

  ```rust

  use compact_str::CompactString;

  let new_year = CompactString::new("农历新年");

  

  assert_eq!(new_year, new_year.to_uppercase());

  ```

  

  One character can become multiple:

  ```rust

  use compact_str::CompactString;

  let s = CompactString::new("tschüß");

  

  assert_eq!("TSCHÜSS", s.to_uppercase());

  ```

- <span id="compactstring-from-str-to-uppercase"></span>`fn from_str_to_uppercase(input: &str) -> Self`

  Returns the uppercase equivalent of this string slice, as a new [`CompactString`](#compactstring).

  

  'Uppercase' is defined according to the terms of the Unicode Derived Core Property

  `Uppercase`.

  

  Since some characters can expand into multiple characters when changing

  the case, this function returns a [`CompactString`](#compactstring) instead of modifying the

  parameter in-place.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use compact_str::CompactString;

  

  assert_eq!("HELLO", CompactString::from_str_to_uppercase("hello"));

  ```

  

  Scripts without case are not changed:

  

  ```rust

  use compact_str::CompactString;

  

  let new_year = "农历新年";

  assert_eq!(new_year, CompactString::from_str_to_uppercase(new_year));

  ```

  

  One character can become multiple:

  ```rust

  use compact_str::CompactString;

  

  assert_eq!("TSCHÜSS", CompactString::from_str_to_uppercase("tschüß"));

  ```

#### Trait Implementations

##### `impl Add for CompactString`

- <span id="compactstring-add-type-output"></span>`type Output = CompactString`

- <span id="compactstring-add"></span>`fn add(self, rhs: &str) -> <Self as >::Output`

##### `impl AddAssign for CompactString`

- <span id="compactstring-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: &str)`

##### `impl Any for CompactString`

- <span id="compactstring-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for CompactString`

- <span id="compactstring-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for CompactString`

- <span id="compactstring-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CompactString`

- <span id="compactstring-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CompactString`

- <span id="compactstring-clone"></span>`fn clone(&self) -> Self`

- <span id="compactstring-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl CloneToUninit for CompactString`

- <span id="compactstring-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CompactString`

- <span id="compactstring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CompactString`

- <span id="compactstring-default"></span>`fn default() -> Self`

##### `impl Deref for CompactString`

- <span id="compactstring-deref-type-target"></span>`type Target = str`

- <span id="compactstring-deref"></span>`fn deref(&self) -> &str`

##### `impl DerefMut for CompactString`

- <span id="compactstring-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut str`

##### `impl Display for CompactString`

- <span id="compactstring-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompactString`

##### `impl Extend for CompactString`

- <span id="compactstring-extend"></span>`fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T)`

##### `impl<T> From for CompactString`

- <span id="compactstring-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for super::Repr`

- <span id="superrepr-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = CompactString>>(iter: T) -> Self`

##### `impl FromStr for CompactString`

- <span id="compactstring-fromstr-type-err"></span>`type Err = Infallible`

- <span id="compactstring-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<CompactString, <Self as >::Err>` — [`CompactString`](#compactstring)

##### `impl Hash for CompactString`

- <span id="compactstring-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for CompactString`

- <span id="compactstring-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl LifetimeFree for crate::CompactString`

##### `impl Ord for CompactString`

- <span id="compactstring-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T: AsRef<str> + ?Sized> PartialEq for CompactString`

- <span id="compactstring-partialeq-eq"></span>`fn eq(&self, other: &T) -> bool`

##### `impl PartialOrd for CompactString`

- <span id="compactstring-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl Receiver for CompactString`

- <span id="compactstring-receiver-type-target"></span>`type Target = T`

##### `impl ToCompactString for CompactString`

- <span id="compactstring-tocompactstring-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToOwned for CompactString`

- <span id="compactstring-toowned-type-owned"></span>`type Owned = T`

- <span id="compactstring-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="compactstring-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for CompactString`

- <span id="compactstring-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for CompactString`

- <span id="compactstring-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compactstring-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CompactString`

- <span id="compactstring-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compactstring-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for CompactString`

- <span id="compactstring-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

- <span id="compactstring-write-write-fmt"></span>`fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result`

### `Utf16Error`

```rust
struct Utf16Error(());
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2483`](../../.source_1765633015/compact_str-0.9.0/src/lib.rs#L2483)*

A possible error value when converting a [`CompactString`](#compactstring) from a UTF-16 byte slice.

This type is the error type for the `from_utf16` method on [`CompactString`](#compactstring).

# Examples

Basic usage:

```rust
use compact_str::CompactString;
// 𝄞mu<invalid>ic
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0xD800, 0x0069, 0x0063];

assert!(CompactString::from_utf16(v).is_err());
```

#### Trait Implementations

##### `impl Any for Utf16Error`

- <span id="utf16error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf16Error`

- <span id="utf16error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf16Error`

- <span id="utf16error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf16Error`

- <span id="utf16error-clone"></span>`fn clone(&self) -> Utf16Error` — [`Utf16Error`](#utf16error)

##### `impl CloneToUninit for Utf16Error`

- <span id="utf16error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Utf16Error`

##### `impl Debug for Utf16Error`

- <span id="utf16error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utf16Error`

- <span id="utf16error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf16Error`

- <span id="utf16error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf16Error`

- <span id="utf16error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToCompactString for Utf16Error`

- <span id="utf16error-tocompactstring-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToOwned for Utf16Error`

- <span id="utf16error-toowned-type-owned"></span>`type Owned = T`

- <span id="utf16error-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf16error-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Utf16Error`

- <span id="utf16error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Utf16Error`

- <span id="utf16error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf16error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf16Error`

- <span id="utf16error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf16error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Drain<'a>`

```rust
struct Drain<'a> {
    compact_string: *mut CompactString,
    start: usize,
    end: usize,
    chars: core::str::Chars<'a>,
}
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2493-2498`](../../.source_1765633015/compact_str-0.9.0/src/lib.rs#L2493-L2498)*

An iterator over the exacted data by `CompactString::drain()`.

#### Implementations

- <span id="drain-as-str"></span>`fn as_str(&self) -> &str`

  The remaining, unconsumed characters of the extracted substring.

#### Trait Implementations

##### `impl Any for Drain<'a>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drain<'a>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Drain<'_>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Drain<'_>`

- <span id="drain-deref-type-target"></span>`type Target = str`

- <span id="drain-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Display for Drain<'_>`

- <span id="drain-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Drain<'_>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<char>`

##### `impl Drop for Drain<'_>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Drain<'a>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Drain<'_>`

##### `impl<U> Into for Drain<'a>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Drain<'a>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Drain<'_>`

- <span id="drain-iterator-type-item"></span>`type Item = char`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="drain-iterator-count"></span>`fn count(self) -> usize`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-iterator-last"></span>`fn last(self) -> Option<char>`

##### `impl Receiver for Drain<'a>`

- <span id="drain-receiver-type-target"></span>`type Target = T`

##### `impl Send for Drain<'_>`

##### `impl Sync for Drain<'_>`

##### `impl ToCompactString for Drain<'a>`

- <span id="drain-tocompactstring-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToString for Drain<'a>`

- <span id="drain-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Drain<'a>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Drain<'a>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReserveError`

```rust
struct ReserveError(());
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2579`](../../.source_1765633015/compact_str-0.9.0/src/lib.rs#L2579)*

A possible error value if allocating or resizing a [`CompactString`](#compactstring) failed.

#### Trait Implementations

##### `impl Any for ReserveError`

- <span id="reserveerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReserveError`

- <span id="reserveerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReserveError`

- <span id="reserveerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ReserveError`

- <span id="reserveerror-clone"></span>`fn clone(&self) -> ReserveError` — [`ReserveError`](#reserveerror)

##### `impl CloneToUninit for ReserveError`

- <span id="reserveerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ReserveError`

##### `impl Debug for ReserveError`

- <span id="reserveerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ReserveError`

- <span id="reserveerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ReserveError`

##### `impl<T> From for ReserveError`

- <span id="reserveerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReserveError`

- <span id="reserveerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ReserveError`

- <span id="reserveerror-partialeq-eq"></span>`fn eq(&self, other: &ReserveError) -> bool` — [`ReserveError`](#reserveerror)

##### `impl StructuralPartialEq for ReserveError`

##### `impl ToCompactString for ReserveError`

- <span id="reserveerror-tocompactstring-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToOwned for ReserveError`

- <span id="reserveerror-toowned-type-owned"></span>`type Owned = T`

- <span id="reserveerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="reserveerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ReserveError`

- <span id="reserveerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ReserveError`

- <span id="reserveerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reserveerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReserveError`

- <span id="reserveerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reserveerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ToCompactStringError`

```rust
enum ToCompactStringError {
    Reserve(ReserveError),
    Fmt(fmt::Error),
}
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2594-2599`](../../.source_1765633015/compact_str-0.9.0/src/lib.rs#L2594-L2599)*

A possible error value if `ToCompactString::try_to_compact_string()` failed.

#### Variants

- **`Reserve`**

  Cannot allocate memory to hold CompactString

- **`Fmt`**

  `Display::fmt()` returned an error

#### Trait Implementations

##### `impl Any for ToCompactStringError`

- <span id="tocompactstringerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ToCompactStringError`

- <span id="tocompactstringerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ToCompactStringError`

- <span id="tocompactstringerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ToCompactStringError`

- <span id="tocompactstringerror-clone"></span>`fn clone(&self) -> ToCompactStringError` — [`ToCompactStringError`](#tocompactstringerror)

##### `impl CloneToUninit for ToCompactStringError`

- <span id="tocompactstringerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ToCompactStringError`

##### `impl Debug for ToCompactStringError`

- <span id="tocompactstringerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ToCompactStringError`

- <span id="tocompactstringerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ToCompactStringError`

- <span id="tocompactstringerror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> From for ToCompactStringError`

- <span id="tocompactstringerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ToCompactStringError`

- <span id="tocompactstringerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ToCompactStringError`

- <span id="tocompactstringerror-partialeq-eq"></span>`fn eq(&self, other: &ToCompactStringError) -> bool` — [`ToCompactStringError`](#tocompactstringerror)

##### `impl StructuralPartialEq for ToCompactStringError`

##### `impl ToCompactString for ToCompactStringError`

- <span id="tocompactstringerror-tocompactstring-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToOwned for ToCompactStringError`

- <span id="tocompactstringerror-toowned-type-owned"></span>`type Owned = T`

- <span id="tocompactstringerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tocompactstringerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ToCompactStringError`

- <span id="tocompactstringerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ToCompactStringError`

- <span id="tocompactstringerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tocompactstringerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ToCompactStringError`

- <span id="tocompactstringerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tocompactstringerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `CompactStringExt`

```rust
trait CompactStringExt { ... }
```

*Defined in [`compact_str-0.9.0/src/traits.rs:142-169`](../../.source_1765633015/compact_str-0.9.0/src/traits.rs#L142-L169)*

A trait that provides convenience methods for creating a [`CompactString`](#compactstring) from a collection of
items. It is implemented for all types that can be converted into an iterator, and that iterator
yields types that can be converted into a `str`.

i.e. `C: IntoIterator<Item = AsRef<str>>`.

# Concatenate and Join
Two methods that this trait provides are `concat_compact(...)` and `join_compact(...)`
```rust
use compact_str::CompactStringExt;

let words = vec!["☀️", "🌕", "🌑", "☀️"];

// directly concatenate all the words together
let concat = words.iter().concat_compact();
assert_eq!(concat, "☀️🌕🌑☀️");

// join the words, with a separator
let join = words.iter().join_compact(" ➡️ ");
assert_eq!(join, "☀️ ➡️ 🌕 ➡️ 🌑 ➡️ ☀️");
```

#### Required Methods

- `fn concat_compact(self) -> CompactString`

  Concatenates all the items of a collection into a [`CompactString`](#compactstring)

- `fn join_compact<S: AsRef<str>>(self, separator: S) -> CompactString`

  Joins all the items of a collection, placing a separator between them, forming a

#### Implementors

- `C`

### `ToCompactString`

```rust
trait ToCompactString { ... }
```

*Defined in [`compact_str-0.9.0/src/traits.rs:16-49`](../../.source_1765633015/compact_str-0.9.0/src/traits.rs#L16-L49)*

A trait for converting a value to a `CompactString`.

This trait is automatically implemented for any type which implements the
[`fmt::Display`](../miette_derive/index.md) trait. As such, [`ToCompactString`](traits/index.md) shouldn't be implemented directly:
[`fmt::Display`](../miette_derive/index.md) should be implemented instead, and you get the [`ToCompactString`](traits/index.md)
implementation for free.

#### Required Methods

- `fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>`

  Fallible version of `ToCompactString::to_compact_string()`

#### Provided Methods

- `fn to_compact_string(&self) -> CompactString`

  Converts the given value to a [`CompactString`](#compactstring).

#### Implementors

- `T`

### `UnwrapWithMsg`

```rust
trait UnwrapWithMsg { ... }
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2635-2639`](../../.source_1765633015/compact_str-0.9.0/src/lib.rs#L2635-L2639)*

#### Associated Types

- `type T`

#### Required Methods

- `fn unwrap_with_msg(self) -> <Self as >::T`

#### Implementors

- `Result<T, E>`

## Functions

### `convert_while_ascii`

```rust
fn convert_while_ascii(b: &[u8], convert: fn(&u8) -> u8) -> CompactString
```

*Defined in [`compact_str-0.9.0/src/lib.rs:1907-1947`](../../.source_1765633015/compact_str-0.9.0/src/lib.rs#L1907-L1947)*

Converts the bytes while the bytes are still ascii.
For better average performance, this is happens in chunks of `2*size_of::<usize>()`.
Returns a vec with the converted bytes.

Copied from https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#623-666

### `unwrap_with_msg_fail`

```rust
fn unwrap_with_msg_fail<E: fmt::Display>(error: E) -> never
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2657-2659`](../../.source_1765633015/compact_str-0.9.0/src/lib.rs#L2657-L2659)*

## Macros

### `format_compact!`

*Defined in [`compact_str-0.9.0/src/macros.rs:28-32`](../../.source_1765633015/compact_str-0.9.0/src/macros.rs#L28-L32)*

Creates a `CompactString` using interpolation of runtime expressions.

The first argument `format_compact!` receives is a format string.
This must be a string literal.
The power of the formatting string is in the `{}`s contained.

Additional parameters passed to `format_compact!` replace the `{}`s within
the formatting string in the order given unless named or
positional parameters are used; see [`std::fmt`](../anstream/fmt/index.md) for more information.

A common use for `format_compact!` is concatenation and interpolation
of strings.
The same convention is used with [`print!`](../backtrace/print/index.md) and [`write!`](../anstream/strip/index.md) macros,
depending on the intended destination of the string.

To convert a single value to a string, use the
`ToCompactString::to_compact_string` method, which uses
the [`std::fmt::Display`](../miette_derive/index.md) formatting trait.

# Panics

`format_compact!` panics if a formatting trait implementation returns
an error.

This indicates an incorrect implementation since
`ToCompactString::to_compact_string` never returns an error itself.

