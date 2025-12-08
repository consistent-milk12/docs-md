*[clap_builder](../../../index.md) / [parser](../../index.md) / [features](../index.md) / [suggestions](index.md)*

---

# Module `suggestions`

## Functions

### `did_you_mean`

```rust
fn did_you_mean<T, I>(v: &str, possible_values: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>
```

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

Returns a suffix that can be empty, or is the standard 'did you mean' phrase

