*[memchr](../index.md) / [cow](index.md)*

---

# Module `cow`

## Structs

### `CowBytes<'a>`

```rust
struct CowBytes<'a>(Imp<'a>);
```

A specialized copy-on-write byte string.

The purpose of this type is to permit usage of a "borrowed or owned
byte string" in a way that keeps std/no-std compatibility. That is, in
no-std/alloc mode, this type devolves into a simple &[u8] with no owned
variant available. We can't just use a plain Cow because Cow is not in
core.

#### Implementations

- `fn new<B: ?Sized + AsRef<[u8]>>(bytes: &'a B) -> CowBytes<'a>` — [`CowBytes`](#cowbytes)

- `fn new_owned(bytes: alloc::boxed::Box<[u8]>) -> CowBytes<'static>` — [`CowBytes`](#cowbytes)

- `fn as_slice(self: &Self) -> &[u8]`

- `fn into_owned(self: Self) -> CowBytes<'static>` — [`CowBytes`](#cowbytes)

#### Trait Implementations

##### `impl<'a> Clone for CowBytes<'a>`

- `fn clone(self: &Self) -> CowBytes<'a>` — [`CowBytes`](#cowbytes)

##### `impl<'a> Debug for CowBytes<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> Deref for CowBytes<'a>`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &[u8]`

##### `impl<P, T> Receiver for CowBytes<'a>`

- `type Target = T`

## Enums

### `Imp<'a>`

```rust
enum Imp<'a> {
    Borrowed(&'a [u8]),
    Owned(alloc::boxed::Box<[u8]>),
}
```

#### Implementations

- `fn new(bytes: &'a [u8]) -> Imp<'a>` — [`Imp`](#imp)

- `fn as_slice(self: &Self) -> &[u8]`

#### Trait Implementations

##### `impl<'a> Clone for Imp<'a>`

- `fn clone(self: &Self) -> Imp<'a>` — [`Imp`](#imp)

##### `impl<'a> Debug for Imp<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

