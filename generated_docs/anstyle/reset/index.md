*[anstyle](../index.md) / [reset](index.md)*

---

# Module `reset`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Reset`](#reset) | struct | Reset terminal formatting |
| [`RESET`](#reset) | const |  |

## Structs

### `Reset`

```rust
struct Reset;
```

*Defined in [`anstyle-1.0.13/src/reset.rs:4`](../../../.source_1765210505/anstyle-1.0.13/src/reset.rs#L4)*

Reset terminal formatting

#### Implementations

- <span id="reset-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

#### Trait Implementations

##### `impl Clone for Reset`

- <span id="reset-clone"></span>`fn clone(&self) -> Reset` — [`Reset`](../index.md)

##### `impl Copy for Reset`

##### `impl Debug for Reset`

- <span id="reset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Reset`

- <span id="reset-default"></span>`fn default() -> Reset` — [`Reset`](../index.md)

##### `impl Display for Reset`

- <span id="reset-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Reset`

##### `impl Hash for Reset`

- <span id="reset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Reset`

- <span id="reset-cmp"></span>`fn cmp(&self, other: &Reset) -> cmp::Ordering` — [`Reset`](../index.md)

##### `impl PartialEq for Reset`

- <span id="reset-eq"></span>`fn eq(&self, other: &Reset) -> bool` — [`Reset`](../index.md)

##### `impl PartialOrd for Reset`

- <span id="reset-partial-cmp"></span>`fn partial_cmp(&self, other: &Reset) -> option::Option<cmp::Ordering>` — [`Reset`](../index.md)

##### `impl StructuralPartialEq for Reset`

##### `impl ToString for Reset`

- <span id="reset-to-string"></span>`fn to_string(&self) -> String`

## Constants

### `RESET`
```rust
const RESET: &str;
```

*Defined in [`anstyle-1.0.13/src/reset.rs:22`](../../../.source_1765210505/anstyle-1.0.13/src/reset.rs#L22)*

