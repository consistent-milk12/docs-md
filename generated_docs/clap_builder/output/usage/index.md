*[clap_builder](../../index.md) / [output](../index.md) / [usage](index.md)*

---

# Module `usage`

## Structs

### `Usage<'cmd>`

```rust
struct Usage<'cmd> {
    cmd: &'cmd crate::builder::Command,
    styles: &'cmd crate::builder::Styles,
    required: Option<&'cmd self::graph::ChildGraph<crate::util::Id>>,
}
```

#### Implementations

- `fn new(cmd: &'cmd Command) -> Self` — [`Command`](../../index.md)

- `fn required(self: Self, required: &'cmd ChildGraph<Id>) -> Self` — [`ChildGraph`](../../util/graph/index.md), [`Id`](../../index.md)

- `fn create_usage_with_title(self: &Self, used: &[Id]) -> Option<StyledStr>` — [`Id`](../../index.md), [`StyledStr`](../../builder/index.md)

- `fn create_usage_no_title(self: &Self, used: &[Id]) -> Option<StyledStr>` — [`Id`](../../index.md), [`StyledStr`](../../builder/index.md)

- `fn write_usage_no_title(self: &Self, styled: &mut StyledStr, used: &[Id]) -> bool` — [`StyledStr`](../../builder/index.md), [`Id`](../../index.md)

## Constants

### `USAGE_SEP`

```rust
const USAGE_SEP: &str;
```

