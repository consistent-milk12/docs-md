*[rayon_core](../../index.md) / [compile_fail](../index.md) / [rc_upvar](index.md)*

---

# Module `rc_upvar`

 ```compile_fail,E0277

use std::rc::Rc;

let r = Rc::new(22);
rayon_core::join(|| r.clone(), || r.clone());
//~^ ERROR

``` 

