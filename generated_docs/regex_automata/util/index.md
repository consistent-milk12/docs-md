*[regex_automata](../index.md) / [util](index.md)*

---

# Module `util`

A collection of modules that provide APIs that are useful across many regex
engines.

While one should explore the sub-modules directly to get a sense of what's
there, here are some highlights that tie the sub-modules to higher level
use cases:

* `alphabet` contains APIs that are useful if you're doing low level things
with the DFAs in this crate. For example, implementing determinization or
walking its state graph directly.
* `captures` contains APIs for dealing with capture group matches and their
mapping to "slots" used inside an NFA graph. This is also where you can find
iterators over capture group names.
* `escape` contains types for pretty-printing raw byte slices as strings.
* `iter` contains API helpers for writing regex iterators.
* `lazy` contains a no-std and no-alloc variant of `lazy_static!` and
`once_cell`.
* `look` contains APIs for matching and configuring look-around assertions.
* `pool` provides a way to reuse mutable memory allocated in a thread safe
manner.
* `prefilter` provides APIs for building prefilters and using them in searches.
* `primitives` are what you might use if you're doing lower level work on
automata, such as walking an NFA state graph.
* `syntax` provides some higher level convenience functions for interacting
with the `regex-syntax` crate.
* `wire` is useful if you're working with DFA serialization.

## Contents

- [Modules](#modules)
  - [`alphabet`](#alphabet)
  - [`captures`](#captures)
  - [`escape`](#escape)
  - [`interpolate`](#interpolate)
  - [`iter`](#iter)
  - [`lazy`](#lazy)
  - [`look`](#look)
  - [`pool`](#pool)
  - [`prefilter`](#prefilter)
  - [`primitives`](#primitives)
  - [`start`](#start)
  - [`syntax`](#syntax)
  - [`wire`](#wire)
  - [`determinize`](#determinize)
  - [`empty`](#empty)
  - [`int`](#int)
  - [`memchr`](#memchr)
  - [`search`](#search)
  - [`sparse_set`](#sparse-set)
  - [`unicode_data`](#unicode-data)
  - [`utf8`](#utf8)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alphabet`](#alphabet) | mod | This module provides APIs for dealing with the alphabets of finite state machines. |
| [`captures`](#captures) | mod | Provides types for dealing with capturing groups. |
| [`escape`](#escape) | mod | Provides convenience routines for escaping raw bytes. |
| [`interpolate`](#interpolate) | mod | Provides routines for interpolating capture group references. |
| [`iter`](#iter) | mod | Generic helpers for iteration of matches from a regex engine in a haystack. |
| [`lazy`](#lazy) | mod | A lazily initialized value for safe sharing between threads. |
| [`look`](#look) | mod | Types and routines for working with look-around assertions. |
| [`pool`](#pool) | mod | A thread safe memory pool. |
| [`prefilter`](#prefilter) | mod | Defines a prefilter for accelerating regex searches. |
| [`primitives`](#primitives) | mod | Lower level primitive types that are useful in a variety of circumstances. |
| [`start`](#start) | mod | Provides helpers for dealing with start state configurations in DFAs. |
| [`syntax`](#syntax) | mod | Utilities for dealing with the syntax of a regular expression. |
| [`wire`](#wire) | mod | Types and routines that support the wire format of finite automata. |
| [`determinize`](#determinize) | mod | This module contains types and routines for implementing determinization. |
| [`empty`](#empty) | mod | This module provides helper routines for dealing with zero-width matches. |
| [`int`](#int) | mod | This module provides several integer oriented traits for converting between both fixed size integers and integers whose size varies based on the target (like `usize`). |
| [`memchr`](#memchr) | mod | This module defines simple wrapper routines for the memchr functions from the `memchr` crate. |
| [`search`](#search) | mod | Types and routines that support the search APIs of most regex engines. |
| [`sparse_set`](#sparse-set) | mod | This module defines a sparse set data structure. |
| [`unicode_data`](#unicode-data) | mod |  |
| [`utf8`](#utf8) | mod | Utilities for dealing with UTF-8. |

## Modules

- [`alphabet`](alphabet/index.md) — This module provides APIs for dealing with the alphabets of finite state
- [`captures`](captures/index.md) — Provides types for dealing with capturing groups.
- [`escape`](escape/index.md) — Provides convenience routines for escaping raw bytes.
- [`interpolate`](interpolate/index.md) — Provides routines for interpolating capture group references.
- [`iter`](iter/index.md) — Generic helpers for iteration of matches from a regex engine in a haystack.
- [`lazy`](lazy/index.md) — A lazily initialized value for safe sharing between threads.
- [`look`](look/index.md) — Types and routines for working with look-around assertions.
- [`pool`](pool/index.md) — A thread safe memory pool.
- [`prefilter`](prefilter/index.md) — Defines a prefilter for accelerating regex searches.
- [`primitives`](primitives/index.md) — Lower level primitive types that are useful in a variety of circumstances.
- [`start`](start/index.md) — Provides helpers for dealing with start state configurations in DFAs.
- [`syntax`](syntax/index.md) — Utilities for dealing with the syntax of a regular expression.
- [`wire`](wire/index.md) — Types and routines that support the wire format of finite automata.
- [`determinize`](determinize/index.md) — This module contains types and routines for implementing determinization.
- [`empty`](empty/index.md) — This module provides helper routines for dealing with zero-width matches.
- [`int`](int/index.md) — This module provides several integer oriented traits for converting between
- [`memchr`](memchr/index.md) — This module defines simple wrapper routines for the memchr functions from the
- [`search`](search/index.md) — Types and routines that support the search APIs of most regex engines.
- [`sparse_set`](sparse_set/index.md) — This module defines a sparse set data structure. Its most interesting
- [`unicode_data`](unicode_data/index.md)
- [`utf8`](utf8/index.md) — Utilities for dealing with UTF-8.

