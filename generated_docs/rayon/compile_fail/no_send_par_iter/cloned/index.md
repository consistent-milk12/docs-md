*[rayon](../../../index.md) / [compile_fail](../../index.md) / [no_send_par_iter](../index.md) / [cloned](index.md)*

---

# Module `cloned`

 ```compile_fail,E0277

use rayon::prelude::*;
use std::ptr::null;

#[derive(Copy, Clone)]
struct NoSend(*const ());

unsafe impl Sync for NoSend {}

let x = Some(NoSend(null()));

x.par_iter()
    .cloned() //~ ERROR
    .count(); //~ ERROR

``` 

