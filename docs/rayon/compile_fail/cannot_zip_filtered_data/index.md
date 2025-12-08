*[rayon](../../index.md) / [compile_fail](../index.md) / [cannot_zip_filtered_data](index.md)*

---

# Module `cannot_zip_filtered_data`

 ```compile_fail,E0277

use rayon::prelude::*;

// zip requires data of exact size, but filter yields only bounded
// size, so check that we cannot apply it.

let mut a: Vec<usize> = (0..1024).rev().collect();
let b: Vec<usize> = (0..1024).collect();

a.par_iter()
    .zip(b.par_iter().filter(|&&x| x > 3)); //~ ERROR

``` 

