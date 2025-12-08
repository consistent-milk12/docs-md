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

- `fn write_help_usage(self: &Self, styled: &mut StyledStr)` — [`StyledStr`](../../builder/index.md)

- `fn write_smart_usage(self: &Self, styled: &mut StyledStr, used: &[Id])` — [`StyledStr`](../../builder/index.md), [`Id`](../../index.md)

- `fn write_arg_usage(self: &Self, styled: &mut StyledStr, used: &[Id], incl_reqs: bool)` — [`StyledStr`](../../builder/index.md), [`Id`](../../index.md)

- `fn write_subcommand_usage(self: &Self, styled: &mut StyledStr)` — [`StyledStr`](../../builder/index.md)

- `fn needs_options_tag(self: &Self) -> bool`

- `fn write_args(self: &Self, styled: &mut StyledStr, incls: &[Id], force_optional: bool)` — [`StyledStr`](../../builder/index.md), [`Id`](../../index.md)

- `fn get_required_usage_from(self: &Self, incls: &[Id], matcher: Option<&ArgMatcher>, incl_last: bool) -> Vec<StyledStr>` — [`Id`](../../index.md), [`ArgMatcher`](../../parser/arg_matcher/index.md), [`StyledStr`](../../builder/index.md)

## Constants

### `USAGE_SEP`

```rust
const USAGE_SEP: &str;
```

