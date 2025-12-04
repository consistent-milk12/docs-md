# Crate `miette_derive`

## Overview

This is a **procedural macro crate** that provides derive macros.

The macros from this crate are typically re-exported from the parent crate [`miette`](../miette/index.md) for convenience. You should generally depend on the parent crate rather than this one directly.

### Usage

```toml
[dependencies]
miette = { version = "*", features = ["derive"] }
```
