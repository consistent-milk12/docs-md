*[rayon](../../../index.md) / [compile_fail](../../index.md) / [must_use](../index.md) / [chunks](index.md)*

---

# Module `chunks`

First sanity check that the expression is OK.

```rust
#![deny(unused_must_use)]

use rayon::prelude::*;

let v: Vec<_> = (0..100).map(Some).collect();
let _ =
 v.par_iter().chunks(2); 
```

Now trigger the `must_use`.

```compile_fail
#![deny(unused_must_use)]

use rayon::prelude::*;

let v: Vec<_> = (0..100).map(Some).collect();
 v.par_iter().chunks(2); 
```

