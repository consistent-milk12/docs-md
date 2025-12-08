*[compact_str](../index.md) / [traits](index.md)*

---

# Module `traits`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ToCompactString`](#tocompactstring) | trait | A trait for converting a value to a `CompactString`. |
| [`CompactStringExt`](#compactstringext) | trait | A trait that provides convenience methods for creating a [`CompactString`] from a collection of |

## Traits

### `ToCompactString`

```rust
trait ToCompactString { ... }
```

A trait for converting a value to a `CompactString`.

This trait is automatically implemented for any type which implements the
[`fmt::Display`](../../miette_derive/fmt/index.md) trait. As such, [`ToCompactString`](../index.md) shouldn't be implemented directly:
[`fmt::Display`](../../miette_derive/fmt/index.md) should be implemented instead, and you get the [`ToCompactString`](../index.md)
implementation for free.

#### Required Methods

- `fn to_compact_string(&self) -> CompactString`

  Converts the given value to a [`CompactString`](../index.md).

- `fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>`

  Fallible version of `ToCompactString::to_compact_string()`

### `CompactStringExt`

```rust
trait CompactStringExt { ... }
```

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

