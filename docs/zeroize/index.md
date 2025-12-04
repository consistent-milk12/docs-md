# Crate `zeroize`

Securely zero memory with a simple trait ([`Zeroize`](zeroize/index.md)) built on stable Rust
primitives which guarantee the operation will not be "optimized away".

## About

[Zeroing memory securely is hard] - compilers optimize for performance, and
in doing so they love to "optimize away" unnecessary zeroing calls. There are
many documented "tricks" to attempt to avoid these optimizations and ensure
that a zeroing routine is performed reliably.

This crate isn't about tricks: it uses [`core::ptr::write_volatile`](#write-volatile)
and [`core::sync::atomic`](#atomic) memory fences to provide easy-to-use, portable
zeroing behavior which works on all of Rust's core number types and slices
thereof, implemented in pure Rust with no usage of FFI or assembly.

- No insecure fallbacks!
- No dependencies!
- No FFI or inline assembly! **WASM friendly** (and tested)!
- `#![no_std](#no-std)
` i.e. **embedded-friendly**!
- No functionality besides securely zeroing memory!
- (Optional) Custom derive support for zeroing complex structures

## Minimum Supported Rust Version

Requires Rust **1.72** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope
for this crate's SemVer guarantees), however when we do it will be accompanied
by a minor version bump.

## Usage

```
use zeroize::Zeroize;

// Protip: don't embed secrets in your source code.
// This is just an example.
let mut secret = b"Air shield password: 1,2,3,4,5".to_vec();
// [ ... ] open the air shield here

// Now that we're done using the secret, zero it out.
secret.zeroize();
```

The [`Zeroize`](zeroize/index.md) trait is impl'd on all of Rust's core scalar types including
integers, floats, `bool`, and `char`.

Additionally, it's implemented on slices and `IterMut`s of the above types.

When the `alloc` feature is enabled (which it is by default), it's also
impl'd for `Vec<T>` for the above types as well as `String`, where it provides
[`Vec::clear`](#clear) / [`String::clear`](#clear)-like behavior (truncating to zero-length)
but ensures the backing memory is securely zeroed with some caveats.

With the `std` feature enabled (which it is **not** by default), [`Zeroize`](zeroize/index.md)
is also implemented for [`CString`](#cstring). After calling `zeroize()` on a `CString`,
its internal buffer will contain exactly one nul byte. The backing
memory is zeroed by converting it to a `Vec<u8>` and back into a `CString`.
(NOTE: see "Stack/Heap Zeroing Notes" for important `Vec`/`String`/`CString` details)

The [`DefaultIsZeroes`](zeroize/index.md) marker trait can be impl'd on types which also
impl [`Default`](../owo_colors/owo_colors/colors/index.md), which implements [`Zeroize`](zeroize/index.md) by overwriting a value with
the default value.

## Custom Derive Support

This crate has custom derive support for the `Zeroize` trait,
gated under the `zeroize` crate's `zeroize_derive` Cargo feature,
which automatically calls `zeroize()` on all members of a struct
or tuple struct.

Attributes supported for `Zeroize`:

On the item level:
- `#[zeroize(drop)]`: *deprecated* use `ZeroizeOnDrop` instead
- `#[zeroize(bound = "T: MyTrait")]`: this replaces any trait bounds
  inferred by zeroize

On the field level:
- `#[zeroize(skip)]`: skips this field or variant when calling `zeroize()`

Attributes supported for `ZeroizeOnDrop`:

On the field level:
- `#[zeroize(skip)]`: skips this field or variant when calling `zeroize()`

Example which derives `Drop`:

```
# #[cfg(feature = "zeroize_derive")]
# {
use zeroize::{Zeroize, ZeroizeOnDrop};

// This struct will be zeroized on drop
#[derive(Zeroize, ZeroizeOnDrop)]
struct MyStruct([u8; 32]);
# }
```

Example which does not derive `Drop` (useful for e.g. `Copy` types)

```
#[cfg(feature = "zeroize_derive")]
# {
use zeroize::Zeroize;

// This struct will *NOT* be zeroized on drop
#[derive(Copy, Clone, Zeroize)]
struct MyStruct([u8; 32]);
# }
```

Example which only derives `Drop`:

```
# #[cfg(feature = "zeroize_derive")]
# {
use zeroize::ZeroizeOnDrop;

// This struct will be zeroized on drop
#[derive(ZeroizeOnDrop)]
struct MyStruct([u8; 32]);
# }
```

## `Zeroizing<Z>`: wrapper for zeroizing arbitrary values on drop

`Zeroizing<Z: Zeroize>` is a generic wrapper type that impls `Deref`
and `DerefMut`, allowing access to an inner value of type `Z`, and also
impls a `Drop` handler which calls `zeroize()` on its contents:

```
use zeroize::Zeroizing;

fn use_secret() {
    let mut secret = Zeroizing::new([0u8; 5]);

    // Set the air shield password
    // Protip (again): don't embed secrets in your source code.
    secret.copy_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(secret.as_ref(), &[1, 2, 3, 4, 5]);

    // The contents of `secret` will be automatically zeroized on drop
}

# use_secret()
```

## What guarantees does this crate provide?

This crate guarantees the following:

1. The zeroing operation can't be "optimized away" by the compiler.
2. All subsequent reads to memory will see "zeroized" values.

LLVM's volatile semantics ensure #1 is true.

Additionally, thanks to work by the [Unsafe Code Guidelines Working Group],
we can now fairly confidently say #2 is true as well. Previously there were
worries that the approach used by this crate (mixing volatile and
non-volatile accesses) was undefined behavior due to language contained
in the documentation for `write_volatile`, however after some discussion
[these remarks have been removed] and the specific usage pattern in this
crate is considered to be well-defined.

Additionally this crate leverages [`core::sync::atomic::compiler_fence`](#compiler-fence)
with the strictest ordering
([`Ordering::SeqCst`](#seqcst)) as a
precaution to help ensure reads are not reordered before memory has been
zeroed.

All of that said, there is still potential for microarchitectural attacks
(ala Spectre/Meltdown) to leak "zeroized" secrets through covert channels.
This crate makes no guarantees that zeroized values cannot be leaked
through such channels, as they represent flaws in the underlying hardware.

## Stack/Heap Zeroing Notes

This crate can be used to zero values from either the stack or the heap.

However, be aware several operations in Rust can unintentionally leave
copies of data in memory. This includes but is not limited to:

- Moves and [`Copy`](#copy)
- Heap reallocation when using [`Vec`](#vec) and [`String`](#string)
- Borrowers of a reference making copies of the data

[`Pin`][`core::pin::Pin`](#pin) can be leveraged in conjunction with this crate
to ensure data kept on the stack isn't moved.

The `Zeroize` impls for `Vec`, `String` and `CString` zeroize the entire
capacity of their backing buffer, but cannot guarantee copies of the data
were not previously made by buffer reallocation. It's therefore important
when attempting to zeroize such buffers to initialize them to the correct
capacity, and take care to prevent subsequent reallocation.

The `secrecy` crate provides higher-level abstractions for eliminating
usage patterns which can cause reallocations:

<https://crates.io/crates/secrecy>

## What about: clearing registers, mlock, mprotect, etc?

This crate is focused on providing simple, unobtrusive support for reliably
zeroing memory using the best approach possible on stable Rust.

Clearing registers is a difficult problem that can't easily be solved by
something like a crate, and requires either inline ASM or rustc support.
See <https://github.com/rust-lang/rust/issues/17046> for background on
this particular problem.

Other memory protection mechanisms are interesting and useful, but often
overkill (e.g. defending against RAM scraping or attackers with swap access).
In as much as there may be merit to these approaches, there are also many
other crates that already implement more sophisticated memory protections.
Such protections are explicitly out-of-scope for this crate.

Zeroing memory is [good cryptographic hygiene] and this crate seeks to promote
it in the most unobtrusive manner possible. This includes omitting complex
`unsafe` memory protection systems and just trying to make the best memory
zeroing crate available.

[Zeroing memory securely is hard]: http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html
[Unsafe Code Guidelines Working Group]: https://github.com/rust-lang/unsafe-code-guidelines
[these remarks have been removed]: https://github.com/rust-lang/rust/pull/60972
[good cryptographic hygiene]: https://github.com/veorq/cryptocoding#clean-memory-of-secret-data


## Structs

### `Zeroizing<Z: Zeroize>`

```rust
struct Zeroizing<Z: Zeroize>();
```

`Zeroizing` is a a wrapper for any `Z: Zeroize` type which implements a
`Drop` handler which zeroizes dropped values.

#### Implementations

- `fn new(value: Z) -> Self`
  Move value inside a `Zeroizing` wrapper which ensures it will be

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<Z>`

- `fn from(value: Z) -> Zeroizing<Z>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsMut<T, Z>`

- `fn as_mut(self: &mut Self) -> &mut T`

##### `impl AsRef<T, Z>`

- `fn as_ref(self: &Self) -> &T`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<Z: Zeroize + Clone>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Drop<Z>`

- `fn drop(self: &mut Self)`

##### `impl Eq<Z: $crate::cmp::Eq + Zeroize>`

##### `impl PartialEq<Z: $crate::cmp::PartialEq + Zeroize>`

- `fn eq(self: &Self, other: &Zeroizing<Z>) -> bool`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq<Z: Zeroize>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Zeroize<Z>`

- `fn zeroize(self: &mut Self)`

##### `impl ZeroizeOnDrop<Z>`

##### `impl Debug<Z: $crate::fmt::Debug + Zeroize>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<Z: $crate::default::Default + Zeroize>`

- `fn default() -> Zeroizing<Z>`

##### `impl Deref<Z>`

- `type Target = Z`

- `fn deref(self: &Self) -> &Z`

##### `impl DerefMut<Z>`

- `fn deref_mut(self: &mut Self) -> &mut Z`

## Traits

### `Zeroize`

```rust
trait Zeroize { ... }
```

Trait for securely erasing values from memory.

#### Required Methods

- `fn zeroize(self: &mut Self)`

  Zero out this object from memory using Rust intrinsics which ensure the

### `ZeroizeOnDrop`

```rust
trait ZeroizeOnDrop { ... }
```

Marker trait signifying that this type will [`Zeroize::zeroize`](#zeroize) itself on [`Drop`](#drop).

### `DefaultIsZeroes`

```rust
trait DefaultIsZeroes: Copy + Default + Sized { ... }
```

Marker trait for types whose [`Default`](../owo_colors/owo_colors/colors/index.md) is the desired zeroization result

### `TryZeroize`

```rust
trait TryZeroize { ... }
```

Fallible trait for representing cases where zeroization may or may not be
possible.

This is primarily useful for scenarios like reference counted data, where
zeroization is only possible when the last reference is dropped.

#### Required Methods

- `fn try_zeroize(self: &mut Self) -> bool`

  Try to zero out this object from memory using Rust intrinsics which

## Functions

### `zeroize_flat_type`

```rust
unsafe fn zeroize_flat_type<F: Sized>(data: *mut F)
```

Zeroizes a flat type/struct. Only zeroizes the values that it owns, and it does not work on
dynamically sized values or trait objects. It would be inefficient to use this function on a
type that already implements `ZeroizeOnDrop`.

# Safety
- The type must not contain references to outside data or dynamically sized data, such as
  `Vec<T>` or `String`.
- Values stored in the type must not have `Drop` impls.
- This function can invalidate the type if it is used after this function is called on it.
  It is advisable to call this function only in `impl Drop`.
- The bit pattern of all zeroes must be valid for the data being zeroized. This may not be
  true for enums and pointers.

# Incompatible data types
Some data types that cannot be safely zeroized using `zeroize_flat_type` include,
but are not limited to:
- References: `&T` and `&mut T`
- Non-nullable types: `NonNull<T>`, `NonZeroU32`, etc.
- Enums with explicit non-zero tags.
- Smart pointers and collections: `Arc<T>`, `Box<T>`, `Vec<T>`, `HashMap<K, V>`, `String`, etc.

# Examples
Safe usage for a struct containing strictly flat data:
```
use zeroize::{ZeroizeOnDrop, zeroize_flat_type};

struct DataToZeroize {
    flat_data_1: [u8; 32],
    flat_data_2: SomeMoreFlatData,
}

struct SomeMoreFlatData(u64);

impl Drop for DataToZeroize {
    fn drop(&mut self) {
        unsafe { zeroize_flat_type(self as *mut Self) }
    }
}
impl ZeroizeOnDrop for DataToZeroize {}

let mut data = DataToZeroize {
    flat_data_1: [3u8; 32],
    flat_data_2: SomeMoreFlatData(123u64)
};

// data gets zeroized when dropped
```

