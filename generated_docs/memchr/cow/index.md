*[memchr](../index.md) / [cow](index.md)*

---

# Module `cow`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CowBytes`](#cowbytes) | struct | A specialized copy-on-write byte string. |
| [`Imp`](#imp) | enum |  |

## Structs

### `CowBytes<'a>`

```rust
struct CowBytes<'a>(Imp<'a>);
```

*Defined in [`memchr-2.7.6/src/cow.rs:11`](../../../.source_1765521767/memchr-2.7.6/src/cow.rs#L11)*

A specialized copy-on-write byte string.

The purpose of this type is to permit usage of a "borrowed or owned
byte string" in a way that keeps std/no-std compatibility. That is, in
no-std/alloc mode, this type devolves into a simple &[u8] with no owned
variant available. We can't just use a plain Cow because Cow is not in
core.

#### Implementations

- <span id="cowbytes-new"></span>`fn new<B: ?Sized + AsRef<[u8]>>(bytes: &'a B) -> CowBytes<'a>` — [`CowBytes`](#cowbytes)

- <span id="cowbytes-new-owned"></span>`fn new_owned(bytes: alloc::boxed::Box<[u8]>) -> CowBytes<'static>` — [`CowBytes`](#cowbytes)

- <span id="cowbytes-as-slice"></span>`fn as_slice(&self) -> &[u8]`

- <span id="cowbytes-into-owned"></span>`fn into_owned(self) -> CowBytes<'static>` — [`CowBytes`](#cowbytes)

#### Trait Implementations

##### `impl Clone for CowBytes<'a>`

- <span id="cowbytes-clone"></span>`fn clone(&self) -> CowBytes<'a>` — [`CowBytes`](#cowbytes)

##### `impl Debug for CowBytes<'a>`

- <span id="cowbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for CowBytes<'a>`

- <span id="cowbytes-deref-type-target"></span>`type Target = [u8]`

- <span id="cowbytes-deref"></span>`fn deref(&self) -> &[u8]`

##### `impl Receiver for CowBytes<'a>`

- <span id="cowbytes-receiver-type-target"></span>`type Target = T`

## Enums

### `Imp<'a>`

```rust
enum Imp<'a> {
    Borrowed(&'a [u8]),
    Owned(alloc::boxed::Box<[u8]>),
}
```

*Defined in [`memchr-2.7.6/src/cow.rs:18-21`](../../../.source_1765521767/memchr-2.7.6/src/cow.rs#L18-L21)*

#### Implementations

- <span id="imp-new"></span>`fn new(bytes: &'a [u8]) -> Imp<'a>` — [`Imp`](#imp)

- <span id="imp-as-slice"></span>`fn as_slice(&self) -> &[u8]`

#### Trait Implementations

##### `impl Clone for Imp<'a>`

- <span id="imp-clone"></span>`fn clone(&self) -> Imp<'a>` — [`Imp`](#imp)

##### `impl Debug for Imp<'a>`

- <span id="imp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

