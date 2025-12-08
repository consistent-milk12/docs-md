*[clap_builder](../../index.md) / [builder](../index.md) / [debug_asserts](index.md)*

---

# Module `debug_asserts`

## Enums

### `Flag<'a>`

```rust
enum Flag<'a> {
    Command(String, &'a str),
    Arg(String, &'a str),
}
```

#### Trait Implementations

##### `impl<'a> Eq for Flag<'a>`

##### `impl Ord for Flag<'_>`

- `fn cmp(self: &Self, other: &Self) -> Ordering`

##### `impl PartialEq for Flag<'_>`

- `fn eq(self: &Self, other: &Flag<'_>) -> bool` — [`Flag`](#flag)

##### `impl PartialOrd for Flag<'_>`

- `fn partial_cmp(self: &Self, other: &Flag<'_>) -> Option<Ordering>` — [`Flag`](#flag)

## Functions

### `assert_app`

```rust
fn assert_app(cmd: &crate::Command)
```

### `duplicate_tip`

```rust
fn duplicate_tip(cmd: &crate::Command, first: &crate::Arg, second: &crate::Arg) -> &'static str
```

### `detect_duplicate_flags`

```rust
fn detect_duplicate_flags(flags: &[Flag<'_>], short_or_long: &str)
```

### `find_duplicates`

```rust
fn find_duplicates<T: PartialEq>(slice: &[T]) -> impl Iterator<Item = (&T, &T)>
```

Find duplicates in a sorted array.

The algorithm is simple: the array is sorted, duplicates
must be placed next to each other, we can check only adjacent elements.

### `assert_app_flags`

```rust
fn assert_app_flags(cmd: &crate::Command)
```

### `_verify_positionals`

```rust
fn _verify_positionals(cmd: &crate::Command) -> bool
```

### `assert_arg`

```rust
fn assert_arg(arg: &crate::Arg)
```

### `assert_arg_flags`

```rust
fn assert_arg_flags(arg: &crate::Arg)
```

