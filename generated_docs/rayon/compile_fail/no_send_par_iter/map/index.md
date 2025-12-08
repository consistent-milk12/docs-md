*[rayon](../../../index.md) / [compile_fail](../../index.md) / [no_send_par_iter](../index.md) / [map](index.md)*

---

# Module `map`

 ```compile_fail,E0277

use rayon::prelude::*;
use std::ptr::null;

#[derive(Copy, Clone)]
struct NoSend(*const ());

unsafe impl Sync for NoSend {}

let x = Some(NoSend(null()));

x.par_iter()
    .map(|&x| x) //~ ERROR
    .count(); //~ ERROR

``` 

