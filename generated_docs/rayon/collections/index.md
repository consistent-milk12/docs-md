*[rayon](../index.md) / [collections](index.md)*

---

# Module `collections`

Parallel iterator types for [standard collections]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Contents

- [Modules](#modules)
  - [`binary_heap`](#binary-heap)
  - [`btree_map`](#btree-map)
  - [`btree_set`](#btree-set)
  - [`hash_map`](#hash-map)
  - [`hash_set`](#hash-set)
  - [`linked_list`](#linked-list)
  - [`vec_deque`](#vec-deque)
  - [`drain_guard`](#drain-guard)
- [Macros](#macros)
  - [`into_par_vec!`](#into-par-vec)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`binary_heap`](#binary-heap) | mod | This module contains the parallel iterator types for heaps (`BinaryHeap<T>`). |
| [`btree_map`](#btree-map) | mod | This module contains the parallel iterator types for B-Tree maps (`BTreeMap<K, V>`). |
| [`btree_set`](#btree-set) | mod | This module contains the parallel iterator types for B-Tree sets (`BTreeSet<T>`). |
| [`hash_map`](#hash-map) | mod | This module contains the parallel iterator types for hash maps (`HashMap<K, V>`). |
| [`hash_set`](#hash-set) | mod | This module contains the parallel iterator types for hash sets (`HashSet<T>`). |
| [`linked_list`](#linked-list) | mod | This module contains the parallel iterator types for linked lists (`LinkedList<T>`). |
| [`vec_deque`](#vec-deque) | mod | This module contains the parallel iterator types for double-ended queues (`VecDeque<T>`). |
| [`drain_guard`](#drain-guard) | mod |  |
| [`into_par_vec!`](#into-par-vec) | macro | Convert an iterable collection into a parallel iterator by first collecting into a temporary `Vec`, then iterating that. |

## Modules

- [`binary_heap`](binary_heap/index.md) — This module contains the parallel iterator types for heaps
- [`btree_map`](btree_map/index.md) — This module contains the parallel iterator types for B-Tree maps
- [`btree_set`](btree_set/index.md) — This module contains the parallel iterator types for B-Tree sets
- [`hash_map`](hash_map/index.md) — This module contains the parallel iterator types for hash maps
- [`hash_set`](hash_set/index.md) — This module contains the parallel iterator types for hash sets
- [`linked_list`](linked_list/index.md) — This module contains the parallel iterator types for linked lists
- [`vec_deque`](vec_deque/index.md) — This module contains the parallel iterator types for double-ended queues
- [`drain_guard`](drain_guard/index.md)

## Macros

### `into_par_vec!`

*Defined in [`rayon-1.11.0/src/collections/mod.rs:10-22`](../../../.source_1765210505/rayon-1.11.0/src/collections/mod.rs#L10-L22)*

Convert an iterable collection into a parallel iterator by first
collecting into a temporary `Vec`, then iterating that.

