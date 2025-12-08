*[rayon](../index.md) / [collections](index.md)*

---

# Module `collections`

Parallel iterator types for [standard collections]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Modules

- [`binary_heap`](binary_heap/index.md) - This module contains the parallel iterator types for heaps
- [`btree_map`](btree_map/index.md) - This module contains the parallel iterator types for B-Tree maps
- [`btree_set`](btree_set/index.md) - This module contains the parallel iterator types for B-Tree sets
- [`hash_map`](hash_map/index.md) - This module contains the parallel iterator types for hash maps
- [`hash_set`](hash_set/index.md) - This module contains the parallel iterator types for hash sets
- [`linked_list`](linked_list/index.md) - This module contains the parallel iterator types for linked lists
- [`vec_deque`](vec_deque/index.md) - This module contains the parallel iterator types for double-ended queues
- [`drain_guard`](drain_guard/index.md) - 

## Macros

### `into_par_vec!`

Convert an iterable collection into a parallel iterator by first
collecting into a temporary `Vec`, then iterating that.

