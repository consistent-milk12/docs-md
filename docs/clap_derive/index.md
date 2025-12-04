# Crate `clap_derive`

# `clap_derive`

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

## Overview

This is a **procedural macro crate** that provides derive macros.

The macros from this crate are typically re-exported from the parent crate [`clap`](../clap/index.md) for convenience. You should generally depend on the parent crate rather than this one directly.

### Usage

```toml
[dependencies]
clap = { version = "*", features = ["derive"] }
```
