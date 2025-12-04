*[base64](../index.md) / [prelude](index.md)*

---

# Module `prelude`

Preconfigured engines for common use cases.

These are re-exports of `const` engines in [crate::engine::general_purpose], renamed with a `BASE64_`
prefix for those who prefer to `use` the entire path to a name.

# Examples

```
use base64::prelude::{Engine as _, BASE64_STANDARD_NO_PAD};

assert_eq!("c29tZSBieXRlcw", &BASE64_STANDARD_NO_PAD.encode(b"some bytes"));
```

## Traits

## Constants

