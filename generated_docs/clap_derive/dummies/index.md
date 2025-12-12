*[clap_derive](../index.md) / [dummies](index.md)*

---

# Module `dummies`

Dummy implementations that we emit along with an error.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parser`](#parser) | fn |  |
| [`into_app`](#into-app) | fn |  |
| [`from_arg_matches`](#from-arg-matches) | fn |  |
| [`subcommand`](#subcommand) | fn |  |
| [`args`](#args) | fn |  |
| [`value_enum`](#value-enum) | fn |  |

## Functions

### `parser`

```rust
fn parser(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/dummies.rs:7-14`](../../../.source_1765210505/clap_derive-4.5.49/src/dummies.rs#L7-L14)*

### `into_app`

```rust
fn into_app(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/dummies.rs:17-29`](../../../.source_1765210505/clap_derive-4.5.49/src/dummies.rs#L17-L29)*

### `from_arg_matches`

```rust
fn from_arg_matches(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/dummies.rs:32-44`](../../../.source_1765210505/clap_derive-4.5.49/src/dummies.rs#L32-L44)*

### `subcommand`

```rust
fn subcommand(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/dummies.rs:47-64`](../../../.source_1765210505/clap_derive-4.5.49/src/dummies.rs#L47-L64)*

### `args`

```rust
fn args(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/dummies.rs:67-81`](../../../.source_1765210505/clap_derive-4.5.49/src/dummies.rs#L67-L81)*

### `value_enum`

```rust
fn value_enum(name: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/dummies.rs:84-99`](../../../.source_1765210505/clap_derive-4.5.49/src/dummies.rs#L84-L99)*

