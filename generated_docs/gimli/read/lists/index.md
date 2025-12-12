*[gimli](../../index.md) / [read](../index.md) / [lists](index.md)*

---

# Module `lists`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ListsHeader`](#listsheader) | struct |  |
| [`parse_header`](#parse-header) | fn |  |

## Structs

### `ListsHeader`

```rust
struct ListsHeader {
    encoding: crate::common::Encoding,
    offset_entry_count: u32,
}
```

*Defined in [`gimli-0.32.3/src/read/lists.rs:5-9`](../../../../.source_1765210505/gimli-0.32.3/src/read/lists.rs#L5-L9)*

#### Implementations

- <span id="listsheader-size"></span>`fn size(self) -> u8`

- <span id="listsheader-size-for-encoding"></span>`fn size_for_encoding(encoding: Encoding) -> u8` — [`Encoding`](../../index.md#encoding)

#### Trait Implementations

##### `impl Clone for ListsHeader`

- <span id="listsheader-clone"></span>`fn clone(&self) -> ListsHeader` — [`ListsHeader`](#listsheader)

##### `impl Copy for ListsHeader`

##### `impl Debug for ListsHeader`

- <span id="listsheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ListsHeader`

- <span id="listsheader-default"></span>`fn default() -> Self`

## Functions

### `parse_header`

```rust
fn parse_header<R: Reader>(input: &mut R) -> crate::read::Result<ListsHeader>
```

*Defined in [`gimli-0.32.3/src/read/lists.rs:43-68`](../../../../.source_1765210505/gimli-0.32.3/src/read/lists.rs#L43-L68)*

