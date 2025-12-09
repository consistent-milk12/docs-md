*[clap_builder](../../index.md) / [output](../index.md) / [usage](index.md)*

---

# Module `usage`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Usage`](#usage) | struct |  |
| [`USAGE_SEP`](#usage_sep) | const |  |

## Structs

### `Usage<'cmd>`

```rust
struct Usage<'cmd> {
    cmd: &'cmd crate::builder::Command,
    styles: &'cmd crate::builder::Styles,
    required: Option<&'cmd self::graph::ChildGraph<crate::util::Id>>,
}
```

*Defined in [`clap_builder-4.5.53/src/output/usage.rs:19-23`](../../../../.source_1765210505/clap_builder-4.5.53/src/output/usage.rs#L19-L23)*

#### Implementations

- <span id="usage-new"></span>`fn new(cmd: &'cmd Command) -> Self` — [`Command`](../../builder/command/index.md)

- <span id="usage-required"></span>`fn required(self, required: &'cmd ChildGraph<Id>) -> Self` — [`ChildGraph`](../../util/graph/index.md), [`Id`](../../util/id/index.md)

- <span id="usage-create-usage-with-title"></span>`fn create_usage_with_title(&self, used: &[Id]) -> Option<StyledStr>` — [`Id`](../../util/id/index.md), [`StyledStr`](../../builder/styled_str/index.md)

- <span id="usage-create-usage-no-title"></span>`fn create_usage_no_title(&self, used: &[Id]) -> Option<StyledStr>` — [`Id`](../../util/id/index.md), [`StyledStr`](../../builder/styled_str/index.md)

- <span id="usage-write-usage-no-title"></span>`fn write_usage_no_title(&self, styled: &mut StyledStr, used: &[Id]) -> bool` — [`StyledStr`](../../builder/styled_str/index.md), [`Id`](../../util/id/index.md)

## Constants

### `USAGE_SEP`
```rust
const USAGE_SEP: &str;
```

*Defined in [`clap_builder-4.5.53/src/output/usage.rs:17`](../../../../.source_1765210505/clap_builder-4.5.53/src/output/usage.rs#L17)*

