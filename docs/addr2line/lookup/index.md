*[addr2line](../index.md) / [lookup](index.md)*

---

# Module `lookup`

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

- `fn new_complete(t: <F as >::Output) -> LookupResult<SimpleLookup<T, R, F>>` — [`LookupResult`](#lookupresult), [`SimpleLookup`](#simplelookup)

- `fn new_needs_load(load: SplitDwarfLoad<R>, f: F) -> LookupResult<SimpleLookup<T, R, F>>` — [`SplitDwarfLoad`](#splitdwarfload), [`LookupResult`](#lookupresult), [`SimpleLookup`](#simplelookup)

#### Trait Implementations

##### `impl<T, R, F> LookupContinuation for SimpleLookup<T, R, F>`

- `type Output = T`

- `type Buf = R`

- `fn resume(self: Self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](#lookupcontinuation), [`LookupResult`](#lookupresult)

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

- `type Output = T`

- `type Buf = <L as LookupContinuation>::Buf`

- `fn resume(self: Self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](#lookupcontinuation), [`LookupResult`](#lookupresult)

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

- `fn new_complete(t: T) -> LookupResult<Self>` — [`LookupResult`](#lookupresult)

- `fn new_lookup(r: LookupResult<L>, mutator: F) -> LookupResult<Self>` — [`LookupResult`](#lookupresult)

#### Trait Implementations

##### `impl<T, L, F> LookupContinuation for LoopingLookup<T, L, F>`

- `type Output = T`

- `type Buf = <L as LookupContinuation>::Buf`

- `fn resume(self: Self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](#lookupcontinuation), [`LookupResult`](#lookupresult)

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

- `fn skip_all_loads(self: Self) -> <L as >::Output` — [`LookupContinuation`](#lookupcontinuation)

- `fn map<T, F: FnOnce(<L as >::Output) -> T>(self: Self, f: F) -> LookupResult<MappedLookup<T, L, F>>` — [`LookupResult`](#lookupresult), [`MappedLookup`](#mappedlookup)

- `fn unwrap(self: Self) -> <L as >::Output` — [`LookupContinuation`](#lookupcontinuation)

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

- `fn resume(self: Self, input: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>`

  Resumes the operation with the provided data.

