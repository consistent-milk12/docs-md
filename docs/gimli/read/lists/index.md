*[gimli](../../index.md) / [read](../index.md) / [lists](index.md)*

---

# Module `lists`

## Structs

### `ListsHeader`

```rust
struct ListsHeader {
    encoding: crate::common::Encoding,
    offset_entry_count: u32,
}
```

#### Implementations

- `fn size(self: Self) -> u8`

- `fn size_for_encoding(encoding: Encoding) -> u8` — [`Encoding`](../../index.md)

#### Trait Implementations

##### `impl Clone for ListsHeader`

- `fn clone(self: &Self) -> ListsHeader` — [`ListsHeader`](#listsheader)

##### `impl Copy for ListsHeader`

##### `impl Debug for ListsHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ListsHeader`

- `fn default() -> Self`

## Functions

### `parse_header`

```rust
fn parse_header<R: Reader>(input: &mut R) -> crate::read::Result<ListsHeader>
```

