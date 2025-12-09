*[rayon](../../index.md) / [compile_fail](../index.md) / [must_use](index.md)*

---

# Module `must_use`

## Contents

- [Modules](#modules)
  - [`by_exponential_blocks`](#by_exponential_blocks)
  - [`by_uniform_blocks`](#by_uniform_blocks)
  - [`step_by`](#step_by)
  - [`chain`](#chain)
  - [`chunks`](#chunks)
  - [`fold_chunks`](#fold_chunks)
  - [`fold_chunks_with`](#fold_chunks_with)
  - [`cloned`](#cloned)
  - [`copied`](#copied)
  - [`enumerate`](#enumerate)
  - [`filter`](#filter)
  - [`filter_map`](#filter_map)
  - [`flat_map`](#flat_map)
  - [`flat_map_iter`](#flat_map_iter)
  - [`flatten`](#flatten)
  - [`flatten_iter`](#flatten_iter)
  - [`fold`](#fold)
  - [`fold_with`](#fold_with)
  - [`try_fold`](#try_fold)
  - [`try_fold_with`](#try_fold_with)
  - [`inspect`](#inspect)
  - [`interleave`](#interleave)
  - [`interleave_shortest`](#interleave_shortest)
  - [`intersperse`](#intersperse)
  - [`map`](#map)
  - [`map_with`](#map_with)
  - [`map_init`](#map_init)
  - [`panic_fuse`](#panic_fuse)
  - [`positions`](#positions)
  - [`rev`](#rev)
  - [`skip`](#skip)
  - [`take`](#take)
  - [`update`](#update)
  - [`while_some`](#while_some)
  - [`with_max_len`](#with_max_len)
  - [`with_min_len`](#with_min_len)
  - [`zip`](#zip)
  - [`zip_eq`](#zip_eq)
- [Macros](#macros)
  - [`must_use!`](#must_use)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`by_exponential_blocks`](#by_exponential_blocks) | mod | First sanity check that the expression is OK. |
| [`by_uniform_blocks`](#by_uniform_blocks) | mod | First sanity check that the expression is OK. |
| [`step_by`](#step_by) | mod | First sanity check that the expression is OK. |
| [`chain`](#chain) | mod | First sanity check that the expression is OK. |
| [`chunks`](#chunks) | mod | First sanity check that the expression is OK. |
| [`fold_chunks`](#fold_chunks) | mod | First sanity check that the expression is OK. |
| [`fold_chunks_with`](#fold_chunks_with) | mod | First sanity check that the expression is OK. |
| [`cloned`](#cloned) | mod | First sanity check that the expression is OK. |
| [`copied`](#copied) | mod | First sanity check that the expression is OK. |
| [`enumerate`](#enumerate) | mod | First sanity check that the expression is OK. |
| [`filter`](#filter) | mod | First sanity check that the expression is OK. |
| [`filter_map`](#filter_map) | mod | First sanity check that the expression is OK. |
| [`flat_map`](#flat_map) | mod | First sanity check that the expression is OK. |
| [`flat_map_iter`](#flat_map_iter) | mod | First sanity check that the expression is OK. |
| [`flatten`](#flatten) | mod | First sanity check that the expression is OK. |
| [`flatten_iter`](#flatten_iter) | mod | First sanity check that the expression is OK. |
| [`fold`](#fold) | mod | First sanity check that the expression is OK. |
| [`fold_with`](#fold_with) | mod | First sanity check that the expression is OK. |
| [`try_fold`](#try_fold) | mod | First sanity check that the expression is OK. |
| [`try_fold_with`](#try_fold_with) | mod | First sanity check that the expression is OK. |
| [`inspect`](#inspect) | mod | First sanity check that the expression is OK. |
| [`interleave`](#interleave) | mod | First sanity check that the expression is OK. |
| [`interleave_shortest`](#interleave_shortest) | mod | First sanity check that the expression is OK. |
| [`intersperse`](#intersperse) | mod | First sanity check that the expression is OK. |
| [`map`](#map) | mod | First sanity check that the expression is OK. |
| [`map_with`](#map_with) | mod | First sanity check that the expression is OK. |
| [`map_init`](#map_init) | mod | First sanity check that the expression is OK. |
| [`panic_fuse`](#panic_fuse) | mod | First sanity check that the expression is OK. |
| [`positions`](#positions) | mod | First sanity check that the expression is OK. |
| [`rev`](#rev) | mod | First sanity check that the expression is OK. |
| [`skip`](#skip) | mod | First sanity check that the expression is OK. |
| [`take`](#take) | mod | First sanity check that the expression is OK. |
| [`update`](#update) | mod | First sanity check that the expression is OK. |
| [`while_some`](#while_some) | mod | First sanity check that the expression is OK. |
| [`with_max_len`](#with_max_len) | mod | First sanity check that the expression is OK. |
| [`with_min_len`](#with_min_len) | mod | First sanity check that the expression is OK. |
| [`zip`](#zip) | mod | First sanity check that the expression is OK. |
| [`zip_eq`](#zip_eq) | mod | First sanity check that the expression is OK. |
| [`must_use!`](#must_use) | macro |  |

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

*Defined in [`rayon-1.11.0/src/compile_fail/must_use.rs:4-30`](../../../../.source_1765210505/rayon-1.11.0/src/compile_fail/must_use.rs#L4-L30)*

