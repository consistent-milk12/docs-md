*[clap_builder](../../../index.md) / [parser](../../index.md) / [features](../index.md) / [suggestions](index.md)*

---

# Module `suggestions`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`did_you_mean`](#did-you-mean) | fn | Find strings from an iterable of `possible_values` similar to a given value `v` Returns a Vec of all possible values that exceed a similarity threshold sorted by ascending similarity, most similar comes last |
| [`did_you_mean_flag`](#did-you-mean-flag) | fn | Returns a suffix that can be empty, or is the standard 'did you mean' phrase |

## Functions

### `did_you_mean`

```rust
fn did_you_mean<T, I>(v: &str, possible_values: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>
```

*Defined in [`clap_builder-4.5.53/src/parser/features/suggestions.rs:8-37`](../../../../../.source_1765633015/clap_builder-4.5.53/src/parser/features/suggestions.rs#L8-L37)*

Find strings from an iterable of `possible_values` similar to a given value `v`
Returns a Vec of all possible values that exceed a similarity threshold
sorted by ascending similarity, most similar comes last

### `did_you_mean_flag`

```rust
fn did_you_mean_flag<'a, 'help, I, T>(arg: &str, remaining_args: &[&std::ffi::OsStr], longs: I, subcommands: impl IntoIterator<Item = &'a mut crate::builder::Command>) -> Option<(String, Option<String>)>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
    'help: 'a
```

*Defined in [`clap_builder-4.5.53/src/parser/features/suggestions.rs:49-86`](../../../../../.source_1765633015/clap_builder-4.5.53/src/parser/features/suggestions.rs#L49-L86)*

Returns a suffix that can be empty, or is the standard 'did you mean' phrase

