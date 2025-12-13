# Crate `clap_derive`

Macro implementation for clap's derives.

[docs.rs](https://docs.rs/clap)
- [Derive Tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [Derive Reference](https://docs.rs/clap/latest/clap/_derive/index.html)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/license/mit>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more details.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`attr`](#attr) | mod |  |
| [`derives`](#derives) | mod |  |
| [`dummies`](#dummies) | mod | Dummy implementations that we emit along with an error. |
| [`item`](#item) | mod |  |
| [`utils`](#utils) | mod |  |
| [`to_compile_error`](#to-compile-error) | fn |  |

## Modules

- [`macros`](macros/index.md)
- [`attr`](attr/index.md)
- [`derives`](derives/index.md)
- [`dummies`](dummies/index.md) — Dummy implementations that we emit along with an error.
- [`item`](item/index.md)
- [`utils`](utils/index.md)

## Functions

### `to_compile_error`

```rust
fn to_compile_error(error: syn::Error, dummy: proc_macro2::TokenStream) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/lib.rs:109-118`](../../.source_1765633015/clap_derive-4.5.49/src/lib.rs#L109-L118)*

