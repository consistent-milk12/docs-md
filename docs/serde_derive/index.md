# Crate `serde_derive`

This crate provides Serde's two derive macros.

```edition2021
# use serde_derive::{Deserialize, Serialize};
#
#[derive(Serialize, Deserialize)]
# struct S;
#
# fn main() {}
```

Please refer to [https://serde.rs/derive.html] for how to set this up.

[https://serde.rs/derive.html]: https://serde.rs/derive.html

*This is a procedural macro crate. The derive macros are re-exported from the parent crate.*
