*[clap_derive](../index.md) / [dummies](index.md)*

---

# Module `dummies`

Dummy implementations that we emit along with an error.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parser`](#parser) | fn |  |
| [`into_app`](#into_app) | fn |  |
| [`from_arg_matches`](#from_arg_matches) | fn |  |
| [`subcommand`](#subcommand) | fn |  |
| [`args`](#args) | fn |  |
| [`value_enum`](#value_enum) | fn |  |

## Functions

### `parser`

```rust
fn parser(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

### `into_app`

```rust
fn into_app(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

### `from_arg_matches`

```rust
fn from_arg_matches(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

### `subcommand`

```rust
fn subcommand(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

### `args`

```rust
fn args(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

### `value_enum`

```rust
fn value_enum(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

