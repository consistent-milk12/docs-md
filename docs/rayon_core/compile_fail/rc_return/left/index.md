*[rayon_core](../../../index.md) / [compile_fail](../../index.md) / [rc_return](../index.md) / [left](index.md)*

---

# Module `left`

 ```compile_fail,E0277

use std::rc::Rc;

rayon_core::join(|| Rc::new(22), || ()); //~ ERROR

``` 

