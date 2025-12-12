*[rayon](../../index.md) / [compile_fail](../index.md) / [must_use](index.md)*

---

# Module `must_use`

## Contents

- [Modules](#modules)
  - [`by_exponential_blocks`](#by-exponential-blocks)
  - [`by_uniform_blocks`](#by-uniform-blocks)
  - [`step_by`](#step-by)
  - [`chain`](#chain)
  - [`chunks`](#chunks)
  - [`fold_chunks`](#fold-chunks)
  - [`fold_chunks_with`](#fold-chunks-with)
  - [`cloned`](#cloned)
  - [`copied`](#copied)
  - [`enumerate`](#enumerate)
  - [`filter`](#filter)
  - [`filter_map`](#filter-map)
  - [`flat_map`](#flat-map)
  - [`flat_map_iter`](#flat-map-iter)
  - [`flatten`](#flatten)
  - [`flatten_iter`](#flatten-iter)
  - [`fold`](#fold)
  - [`fold_with`](#fold-with)
  - [`try_fold`](#try-fold)
  - [`try_fold_with`](#try-fold-with)
  - [`inspect`](#inspect)
  - [`interleave`](#interleave)
  - [`interleave_shortest`](#interleave-shortest)
  - [`intersperse`](#intersperse)
  - [`map`](#map)
  - [`map_with`](#map-with)
  - [`map_init`](#map-init)
  - [`panic_fuse`](#panic-fuse)
  - [`positions`](#positions)
  - [`rev`](#rev)
  - [`skip`](#skip)
  - [`take`](#take)
  - [`update`](#update)
  - [`while_some`](#while-some)
  - [`with_max_len`](#with-max-len)
  - [`with_min_len`](#with-min-len)
  - [`zip`](#zip)
  - [`zip_eq`](#zip-eq)
- [Macros](#macros)
  - [`must_use!`](#must-use)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`by_exponential_blocks`](#by-exponential-blocks) | mod | First sanity check that the expression is OK. |
| [`by_uniform_blocks`](#by-uniform-blocks) | mod | First sanity check that the expression is OK. |
| [`step_by`](#step-by) | mod | First sanity check that the expression is OK. |
| [`chain`](#chain) | mod | First sanity check that the expression is OK. |
| [`chunks`](#chunks) | mod | First sanity check that the expression is OK. |
| [`fold_chunks`](#fold-chunks) | mod | First sanity check that the expression is OK. |
| [`fold_chunks_with`](#fold-chunks-with) | mod | First sanity check that the expression is OK. |
| [`cloned`](#cloned) | mod | First sanity check that the expression is OK. |
| [`copied`](#copied) | mod | First sanity check that the expression is OK. |
| [`enumerate`](#enumerate) | mod | First sanity check that the expression is OK. |
| [`filter`](#filter) | mod | First sanity check that the expression is OK. |
| [`filter_map`](#filter-map) | mod | First sanity check that the expression is OK. |
| [`flat_map`](#flat-map) | mod | First sanity check that the expression is OK. |
| [`flat_map_iter`](#flat-map-iter) | mod | First sanity check that the expression is OK. |
| [`flatten`](#flatten) | mod | First sanity check that the expression is OK. |
| [`flatten_iter`](#flatten-iter) | mod | First sanity check that the expression is OK. |
| [`fold`](#fold) | mod | First sanity check that the expression is OK. |
| [`fold_with`](#fold-with) | mod | First sanity check that the expression is OK. |
| [`try_fold`](#try-fold) | mod | First sanity check that the expression is OK. |
| [`try_fold_with`](#try-fold-with) | mod | First sanity check that the expression is OK. |
| [`inspect`](#inspect) | mod | First sanity check that the expression is OK. |
| [`interleave`](#interleave) | mod | First sanity check that the expression is OK. |
| [`interleave_shortest`](#interleave-shortest) | mod | First sanity check that the expression is OK. |
| [`intersperse`](#intersperse) | mod | First sanity check that the expression is OK. |
| [`map`](#map) | mod | First sanity check that the expression is OK. |
| [`map_with`](#map-with) | mod | First sanity check that the expression is OK. |
| [`map_init`](#map-init) | mod | First sanity check that the expression is OK. |
| [`panic_fuse`](#panic-fuse) | mod | First sanity check that the expression is OK. |
| [`positions`](#positions) | mod | First sanity check that the expression is OK. |
| [`rev`](#rev) | mod | First sanity check that the expression is OK. |
| [`skip`](#skip) | mod | First sanity check that the expression is OK. |
| [`take`](#take) | mod | First sanity check that the expression is OK. |
| [`update`](#update) | mod | First sanity check that the expression is OK. |
| [`while_some`](#while-some) | mod | First sanity check that the expression is OK. |
| [`with_max_len`](#with-max-len) | mod | First sanity check that the expression is OK. |
| [`with_min_len`](#with-min-len) | mod | First sanity check that the expression is OK. |
| [`zip`](#zip) | mod | First sanity check that the expression is OK. |
| [`zip_eq`](#zip-eq) | mod | First sanity check that the expression is OK. |
| [`must_use!`](#must-use) | macro |  |

## Modules

- [`by_exponential_blocks`](by_exponential_blocks/index.md) — First sanity check that the expression is OK.
- [`by_uniform_blocks`](by_uniform_blocks/index.md) — First sanity check that the expression is OK.
- [`step_by`](step_by/index.md) — First sanity check that the expression is OK.
- [`chain`](chain/index.md) — First sanity check that the expression is OK.
- [`chunks`](chunks/index.md) — First sanity check that the expression is OK.
- [`fold_chunks`](fold_chunks/index.md) — First sanity check that the expression is OK.
- [`fold_chunks_with`](fold_chunks_with/index.md) — First sanity check that the expression is OK.
- [`cloned`](cloned/index.md) — First sanity check that the expression is OK.
- [`copied`](copied/index.md) — First sanity check that the expression is OK.
- [`enumerate`](enumerate/index.md) — First sanity check that the expression is OK.
- [`filter`](filter/index.md) — First sanity check that the expression is OK.
- [`filter_map`](filter_map/index.md) — First sanity check that the expression is OK.
- [`flat_map`](flat_map/index.md) — First sanity check that the expression is OK.
- [`flat_map_iter`](flat_map_iter/index.md) — First sanity check that the expression is OK.
- [`flatten`](flatten/index.md) — First sanity check that the expression is OK.
- [`flatten_iter`](flatten_iter/index.md) — First sanity check that the expression is OK.
- [`fold`](fold/index.md) — First sanity check that the expression is OK.
- [`fold_with`](fold_with/index.md) — First sanity check that the expression is OK.
- [`try_fold`](try_fold/index.md) — First sanity check that the expression is OK.
- [`try_fold_with`](try_fold_with/index.md) — First sanity check that the expression is OK.
- [`inspect`](inspect/index.md) — First sanity check that the expression is OK.
- [`interleave`](interleave/index.md) — First sanity check that the expression is OK.
- [`interleave_shortest`](interleave_shortest/index.md) — First sanity check that the expression is OK.
- [`intersperse`](intersperse/index.md) — First sanity check that the expression is OK.
- [`map`](map/index.md) — First sanity check that the expression is OK.
- [`map_with`](map_with/index.md) — First sanity check that the expression is OK.
- [`map_init`](map_init/index.md) — First sanity check that the expression is OK.
- [`panic_fuse`](panic_fuse/index.md) — First sanity check that the expression is OK.
- [`positions`](positions/index.md) — First sanity check that the expression is OK.
- [`rev`](rev/index.md) — First sanity check that the expression is OK.
- [`skip`](skip/index.md) — First sanity check that the expression is OK.
- [`take`](take/index.md) — First sanity check that the expression is OK.
- [`update`](update/index.md) — First sanity check that the expression is OK.
- [`while_some`](while_some/index.md) — First sanity check that the expression is OK.
- [`with_max_len`](with_max_len/index.md) — First sanity check that the expression is OK.
- [`with_min_len`](with_min_len/index.md) — First sanity check that the expression is OK.
- [`zip`](zip/index.md) — First sanity check that the expression is OK.
- [`zip_eq`](zip_eq/index.md) — First sanity check that the expression is OK.

## Macros

### `must_use!`

*Defined in [`rayon-1.11.0/src/compile_fail/must_use.rs:4-30`](../../../../.source_1765521767/rayon-1.11.0/src/compile_fail/must_use.rs#L4-L30)*

