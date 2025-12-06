# Crate `serde_derive`

This crate provides Serde's two derive macros.

```edition2021
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct S;

fn main() {}
```

Please refer to [https://serde.rs/derive.html] for how to set this up.


## Overview

This is a **procedural macro crate** that provides derive macros.

The macros from this crate are typically re-exported from the parent crate [`serde`](../serde/index.md) for convenience. You should generally depend on the parent crate rather than this one directly.

### Usage

```toml
[dependencies]
serde = { version = "*", features = ["derive"] }
```
