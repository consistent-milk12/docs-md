*[addr2line](../index.md) / [lookup](index.md)*

---

# Module `lookup`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SplitDwarfLoad`](#splitdwarfload) | struct | This struct contains the information needed to find split DWARF data |
| [`SimpleLookup`](#simplelookup) | struct |  |
| [`MappedLookup`](#mappedlookup) | struct |  |
| [`LoopingLookup`](#loopinglookup) | struct | Some functions (e.g. `find_frames`) require considering multiple |
| [`LookupResult`](#lookupresult) | enum | Operations that consult debug information may require additional files |
| [`LookupContinuation`](#lookupcontinuation) | trait | This trait represents a partially complete operation that can be resumed |

## Structs

### `SplitDwarfLoad<R>`

```rust
struct SplitDwarfLoad<R> {
    pub dwo_id: gimli::DwoId,
    pub comp_dir: Option<R>,
    pub path: Option<R>,
    pub parent: alloc::sync::Arc<gimli::Dwarf<R>>,
}
```

This struct contains the information needed to find split DWARF data
and to produce a `gimli::Dwarf<R>` for it.

#### Fields

- **`dwo_id`**: `gimli::DwoId`

  The dwo id, for looking up in a DWARF package, or for
  verifying an unpacked dwo found on the file system

- **`comp_dir`**: `Option<R>`

  The compilation directory `path` is relative to.

- **`path`**: `Option<R>`

  A path on the filesystem, relative to `comp_dir` to find this dwo.

- **`parent`**: `alloc::sync::Arc<gimli::Dwarf<R>>`

  Once the split DWARF data is loaded, the loader is expected
  to call [make_dwo(parent)](gimli::read::Dwarf::make_dwo) before
  returning the data.

### `SimpleLookup<T, R, F>`

```rust
struct SimpleLookup<T, R, F>
where
    F: FnOnce(Option<alloc::sync::Arc<gimli::Dwarf<R>>>) -> T,
    R: gimli::Reader {
    f: F,
    phantom: core::marker::PhantomData<(T, R)>,
}
```

#### Implementations

- <span id="simplelookup-new-complete"></span>`fn new_complete(t: <F as >::Output) -> LookupResult<SimpleLookup<T, R, F>>` — [`LookupResult`](../index.md), [`SimpleLookup`](#simplelookup)

- <span id="simplelookup-new-needs-load"></span>`fn new_needs_load(load: SplitDwarfLoad<R>, f: F) -> LookupResult<SimpleLookup<T, R, F>>` — [`SplitDwarfLoad`](../index.md), [`LookupResult`](../index.md), [`SimpleLookup`](#simplelookup)

#### Trait Implementations

##### `impl<T, R, F> LookupContinuation for SimpleLookup<T, R, F>`

- <span id="simplelookup-output"></span>`type Output = T`

- <span id="simplelookup-buf"></span>`type Buf = R`

- <span id="simplelookup-resume"></span>`fn resume(self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](../index.md), [`LookupResult`](../index.md)

### `MappedLookup<T, L, F>`

```rust
struct MappedLookup<T, L, F>
where
    L: LookupContinuation,
    F: FnOnce(<L as >::Output) -> T {
    original: L,
    mutator: F,
}
```

#### Trait Implementations

##### `impl<T, L, F> LookupContinuation for MappedLookup<T, L, F>`

- <span id="mappedlookup-output"></span>`type Output = T`

- <span id="mappedlookup-buf"></span>`type Buf = <L as LookupContinuation>::Buf`

- <span id="mappedlookup-resume"></span>`fn resume(self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](../index.md), [`LookupResult`](../index.md)

### `LoopingLookup<T, L, F>`

```rust
struct LoopingLookup<T, L, F>
where
    L: LookupContinuation,
    F: FnMut(<L as >::Output) -> core::ops::ControlFlow<T, LookupResult<L>> {
    continuation: L,
    mutator: F,
}
```

Some functions (e.g. `find_frames`) require considering multiple
compilation units, each of which might require their own split DWARF
lookup (and thus produce a continuation).

We store the underlying continuation here as well as a mutator function
that will either a) decide that the result of this continuation is
what is needed and mutate it to the final result or b) produce another
`LookupResult`. `new_lookup` will in turn eagerly drive any non-continuation
`LookupResult` with successive invocations of the mutator, until a new
continuation or a final result is produced. And finally, the impl of
`LookupContinuation::resume` will call `new_lookup` each time the
computation is resumed.

#### Implementations

- <span id="loopinglookup-new-complete"></span>`fn new_complete(t: T) -> LookupResult<Self>` — [`LookupResult`](../index.md)

- <span id="loopinglookup-new-lookup"></span>`fn new_lookup(r: LookupResult<L>, mutator: F) -> LookupResult<Self>` — [`LookupResult`](../index.md)

#### Trait Implementations

##### `impl<T, L, F> LookupContinuation for LoopingLookup<T, L, F>`

- <span id="loopinglookup-output"></span>`type Output = T`

- <span id="loopinglookup-buf"></span>`type Buf = <L as LookupContinuation>::Buf`

- <span id="loopinglookup-resume"></span>`fn resume(self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](../index.md), [`LookupResult`](../index.md)

## Enums

### `LookupResult<L: LookupContinuation>`

```rust
enum LookupResult<L: LookupContinuation> {
    Load {
        load: SplitDwarfLoad<<L as LookupContinuation>::Buf>,
        continuation: L,
    },
    Output(<L as LookupContinuation>::Output),
}
```

Operations that consult debug information may require additional files
to be loaded if split DWARF is being used. This enum returns the result
of the operation in the `Output` variant, or information about the split
DWARF that is required and a continuation to invoke once it is available
in the `Load` variant.

This enum is intended to be used in a loop like so:
```no_run
  use addr2line::*;
  use std::sync::Arc;
  let ctx: Context<gimli::EndianSlice<gimli::RunTimeEndian>> = todo!();
  let do_split_dwarf_load = |load: SplitDwarfLoad<gimli::EndianSlice<gimli::RunTimeEndian>>| -> Option<Arc<gimli::Dwarf<gimli::EndianSlice<gimli::RunTimeEndian>>>> { None };
  const ADDRESS: u64 = 0xdeadbeef;
  let mut r = ctx.find_frames(ADDRESS);
  let result = loop {
    match r {
      LookupResult::Output(result) => break result,
      LookupResult::Load { load, continuation } => {
        let dwo = do_split_dwarf_load(load);
        r = continuation.resume(dwo);
      }
    }
  };
```

#### Variants

- **`Load`**

  The lookup requires split DWARF data to be loaded.

- **`Output`**

  The lookup has completed and produced an output.

#### Implementations

- <span id="lookupresult-skip-all-loads"></span>`fn skip_all_loads(self) -> <L as >::Output` — [`LookupContinuation`](../index.md)

- <span id="lookupresult-map"></span>`fn map<T, F: FnOnce(<L as >::Output) -> T>(self, f: F) -> LookupResult<MappedLookup<T, L, F>>` — [`LookupResult`](../index.md), [`MappedLookup`](#mappedlookup)

- <span id="lookupresult-unwrap"></span>`fn unwrap(self) -> <L as >::Output` — [`LookupContinuation`](../index.md)

## Traits

### `LookupContinuation`

```rust
trait LookupContinuation: Sized { ... }
```

This trait represents a partially complete operation that can be resumed
once a load of needed split DWARF data is completed or abandoned by the
API consumer.

#### Required Methods

- `type Output`

- `type Buf: 1`

- `fn resume(self, input: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>`

  Resumes the operation with the provided data.

