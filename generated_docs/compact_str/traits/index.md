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

*Defined in [`compact_str-0.9.0/src/traits.rs:16-49`](../../../.source_1765900590/compact_str-0.9.0/src/traits.rs#L16-L49)*

A trait for converting a value to a `CompactString`.

This trait is automatically implemented for any type which implements the
`fmt::Display` trait. As such, [`ToCompactString`](#tocompactstring) shouldn't be implemented directly:
`fmt::Display` should be implemented instead, and you get the [`ToCompactString`](#tocompactstring)
implementation for free.

#### Required Methods

- `fn ToCompactString::try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>`

  Fallible version of `ToCompactString::to_compact_string()`
  
  This method won't panic if the system is out-of-memory, but return a
  `ReserveError`.
  Otherwise it behaves the same as `ToCompactString::to_compact_string()`.

#### Provided Methods

- `fn ToCompactString::to_compact_string(&self) -> CompactString`

  Converts the given value to a [`CompactString`](../index.md).
  
  ##### Panics
  
  Panics if the system runs out of memory and it cannot hold the whole string,
  or if `Display::fmt()` returns an error.
  
  ##### Examples
  
  Basic usage:
  
  ```rust
  use compact_str::ToCompactString;
  use compact_str::CompactString;
  
  let i = 5;
  let five = CompactString::new("5");
  
  assert_eq!(i.to_compact_string(), five);
  ```

#### Implementors

- `T`

### `CompactStringExt`

```rust
trait CompactStringExt { ... }
```

*Defined in [`compact_str-0.9.0/src/traits.rs:142-169`](../../../.source_1765900590/compact_str-0.9.0/src/traits.rs#L142-L169)*

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

- `fn CompactStringExt::concat_compact(self) -> CompactString`

  Concatenates all the items of a collection into a [`CompactString`](../index.md)
  
  ##### Example
  ```rust
  use compact_str::CompactStringExt;
  
  let items = ["hello", " ", "world", "!"];
  let compact = items.concat_compact();
  
  assert_eq!(compact, "hello world!");
  ```

- `fn CompactStringExt::join_compact<S: AsRef<str>>(self, separator: S) -> CompactString`

  Joins all the items of a collection, placing a separator between them, forming a
  [`CompactString`](../index.md)
  
  ##### Example
  ```rust
  use compact_str::CompactStringExt;
  
  let fruits = vec!["apples", "oranges", "bananas"];
  let compact = fruits.join_compact(", ");
  
  assert_eq!(compact, "apples, oranges, bananas");
  ```

#### Implementors

- `C`

