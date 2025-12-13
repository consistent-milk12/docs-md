*[compact_str](../index.md) / [traits](index.md)*

---

# Module `traits`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ToCompactString`](#tocompactstring) | trait | A trait for converting a value to a `CompactString`. |
| [`CompactStringExt`](#compactstringext) | trait | A trait that provides convenience methods for creating a [`CompactString`] from a collection of items. |

## Traits

### `ToCompactString`

```rust
trait ToCompactString { ... }
```

*Defined in [`compact_str-0.9.0/src/traits.rs:16-49`](../../../.source_1765521767/compact_str-0.9.0/src/traits.rs#L16-L49)*

A trait for converting a value to a `CompactString`.

This trait is automatically implemented for any type which implements the
[`fmt::Display`](../../miette_derive/index.md) trait. As such, [`ToCompactString`](#tocompactstring) shouldn't be implemented directly:
[`fmt::Display`](../../miette_derive/index.md) should be implemented instead, and you get the [`ToCompactString`](#tocompactstring)
implementation for free.

#### Required Methods

- `fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>`

  Fallible version of `ToCompactString::to_compact_string()`

#### Provided Methods

- `fn to_compact_string(&self) -> CompactString`

  Converts the given value to a [`CompactString`](../index.md).

#### Implementors

- `T`

### `CompactStringExt`

```rust
trait CompactStringExt { ... }
```

*Defined in [`compact_str-0.9.0/src/traits.rs:142-169`](../../../.source_1765521767/compact_str-0.9.0/src/traits.rs#L142-L169)*

A trait that provides convenience methods for creating a [`CompactString`](../index.md) from a collection of
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

  Concatenates all the items of a collection into a [`CompactString`](../index.md)

- `fn join_compact<S: AsRef<str>>(self, separator: S) -> CompactString`

  Joins all the items of a collection, placing a separator between them, forming a

#### Implementors

- `C`

