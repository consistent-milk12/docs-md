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

#### Implementations

- <span id="usage-write-help-usage"></span>`fn write_help_usage(&self, styled: &mut StyledStr)` — [`StyledStr`](../../builder/index.md)

- <span id="usage-write-smart-usage"></span>`fn write_smart_usage(&self, styled: &mut StyledStr, used: &[Id])` — [`StyledStr`](../../builder/index.md), [`Id`](../../index.md)

- <span id="usage-write-arg-usage"></span>`fn write_arg_usage(&self, styled: &mut StyledStr, used: &[Id], incl_reqs: bool)` — [`StyledStr`](../../builder/index.md), [`Id`](../../index.md)

- <span id="usage-write-subcommand-usage"></span>`fn write_subcommand_usage(&self, styled: &mut StyledStr)` — [`StyledStr`](../../builder/index.md)

- <span id="usage-needs-options-tag"></span>`fn needs_options_tag(&self) -> bool`

- <span id="usage-write-args"></span>`fn write_args(&self, styled: &mut StyledStr, incls: &[Id], force_optional: bool)` — [`StyledStr`](../../builder/index.md), [`Id`](../../index.md)

- <span id="usage-get-required-usage-from"></span>`fn get_required_usage_from(&self, incls: &[Id], matcher: Option<&ArgMatcher>, incl_last: bool) -> Vec<StyledStr>` — [`Id`](../../index.md), [`ArgMatcher`](../../parser/arg_matcher/index.md), [`StyledStr`](../../builder/index.md)

## Constants

### `USAGE_SEP`

```rust
const USAGE_SEP: &str;
```

