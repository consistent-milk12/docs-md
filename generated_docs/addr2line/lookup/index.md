*[addr2line](../index.md) / [lookup](index.md)*

---

# Module `lookup`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SplitDwarfLoad`](#splitdwarfload) | struct | This struct contains the information needed to find split DWARF data and to produce a `gimli::Dwarf<R>` for it. |
| [`SimpleLookup`](#simplelookup) | struct |  |
| [`MappedLookup`](#mappedlookup) | struct |  |
| [`LoopingLookup`](#loopinglookup) | struct | Some functions (e.g. `find_frames`) require considering multiple compilation units, each of which might require their own split DWARF lookup (and thus produce a continuation). |
| [`LookupResult`](#lookupresult) | enum | Operations that consult debug information may require additional files to be loaded if split DWARF is being used. |
| [`LookupContinuation`](#lookupcontinuation) | trait | This trait represents a partially complete operation that can be resumed once a load of needed split DWARF data is completed or abandoned by the API consumer. |

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

*Defined in [`addr2line-0.25.1/src/lookup.rs:7-19`](../../../.source_1765633015/addr2line-0.25.1/src/lookup.rs#L7-L19)*

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

#### Trait Implementations

##### `impl Any for SplitDwarfLoad<R>`

- <span id="splitdwarfload-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitDwarfLoad<R>`

- <span id="splitdwarfload-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitDwarfLoad<R>`

- <span id="splitdwarfload-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SplitDwarfLoad<R>`

- <span id="splitdwarfload-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitDwarfLoad<R>`

- <span id="splitdwarfload-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SplitDwarfLoad<R>`

- <span id="splitdwarfload-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitdwarfload-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitDwarfLoad<R>`

- <span id="splitdwarfload-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitdwarfload-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`addr2line-0.25.1/src/lookup.rs:118-125`](../../../.source_1765633015/addr2line-0.25.1/src/lookup.rs#L118-L125)*

#### Implementations

- <span id="simplelookup-new-complete"></span>`fn new_complete(t: <F as >::Output) -> LookupResult<SimpleLookup<T, R, F>>` — [`LookupResult`](#lookupresult), [`SimpleLookup`](#simplelookup)

- <span id="simplelookup-new-needs-load"></span>`fn new_needs_load(load: SplitDwarfLoad<R>, f: F) -> LookupResult<SimpleLookup<T, R, F>>` — [`SplitDwarfLoad`](#splitdwarfload), [`LookupResult`](#lookupresult), [`SimpleLookup`](#simplelookup)

#### Trait Implementations

##### `impl<T> Any for SimpleLookup<T, R, F>`

- <span id="simplelookup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SimpleLookup<T, R, F>`

- <span id="simplelookup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SimpleLookup<T, R, F>`

- <span id="simplelookup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SimpleLookup<T, R, F>`

- <span id="simplelookup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for SimpleLookup<T, R, F>`

- <span id="simplelookup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, R, F> LookupContinuation for SimpleLookup<T, R, F>`

- <span id="simplelookup-lookupcontinuation-type-output"></span>`type Output = T`

- <span id="simplelookup-lookupcontinuation-type-buf"></span>`type Buf = R`

- <span id="simplelookup-lookupcontinuation-resume"></span>`fn resume(self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](#lookupcontinuation), [`LookupResult`](#lookupresult)

##### `impl<T, U> TryFrom for SimpleLookup<T, R, F>`

- <span id="simplelookup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="simplelookup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SimpleLookup<T, R, F>`

- <span id="simplelookup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="simplelookup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`addr2line-0.25.1/src/lookup.rs:163-170`](../../../.source_1765633015/addr2line-0.25.1/src/lookup.rs#L163-L170)*

#### Trait Implementations

##### `impl<T> Any for MappedLookup<T, L, F>`

- <span id="mappedlookup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MappedLookup<T, L, F>`

- <span id="mappedlookup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MappedLookup<T, L, F>`

- <span id="mappedlookup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MappedLookup<T, L, F>`

- <span id="mappedlookup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for MappedLookup<T, L, F>`

- <span id="mappedlookup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, L, F> LookupContinuation for MappedLookup<T, L, F>`

- <span id="mappedlookup-lookupcontinuation-type-output"></span>`type Output = T`

- <span id="mappedlookup-lookupcontinuation-type-buf"></span>`type Buf = <L as LookupContinuation>::Buf`

- <span id="mappedlookup-lookupcontinuation-resume"></span>`fn resume(self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](#lookupcontinuation), [`LookupResult`](#lookupresult)

##### `impl<T, U> TryFrom for MappedLookup<T, L, F>`

- <span id="mappedlookup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mappedlookup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for MappedLookup<T, L, F>`

- <span id="mappedlookup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mappedlookup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`addr2line-0.25.1/src/lookup.rs:206-213`](../../../.source_1765633015/addr2line-0.25.1/src/lookup.rs#L206-L213)*

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

- <span id="loopinglookup-new-complete"></span>`fn new_complete(t: T) -> LookupResult<Self>` — [`LookupResult`](#lookupresult)

- <span id="loopinglookup-new-lookup"></span>`fn new_lookup(r: LookupResult<L>, mutator: F) -> LookupResult<Self>` — [`LookupResult`](#lookupresult)

#### Trait Implementations

##### `impl<T> Any for LoopingLookup<T, L, F>`

- <span id="loopinglookup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LoopingLookup<T, L, F>`

- <span id="loopinglookup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LoopingLookup<T, L, F>`

- <span id="loopinglookup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LoopingLookup<T, L, F>`

- <span id="loopinglookup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for LoopingLookup<T, L, F>`

- <span id="loopinglookup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, L, F> LookupContinuation for LoopingLookup<T, L, F>`

- <span id="loopinglookup-lookupcontinuation-type-output"></span>`type Output = T`

- <span id="loopinglookup-lookupcontinuation-type-buf"></span>`type Buf = <L as LookupContinuation>::Buf`

- <span id="loopinglookup-lookupcontinuation-resume"></span>`fn resume(self, v: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>` — [`LookupContinuation`](#lookupcontinuation), [`LookupResult`](#lookupresult)

##### `impl<T, U> TryFrom for LoopingLookup<T, L, F>`

- <span id="loopinglookup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loopinglookup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for LoopingLookup<T, L, F>`

- <span id="loopinglookup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loopinglookup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`addr2line-0.25.1/src/lookup.rs:45-55`](../../../.source_1765633015/addr2line-0.25.1/src/lookup.rs#L45-L55)*

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

- <span id="lookupresult-skip-all-loads"></span>`fn skip_all_loads(self) -> <L as >::Output` — [`LookupContinuation`](#lookupcontinuation)

  Callers that do not handle split DWARF can call `skip_all_loads`

  to fast-forward to the end result. This result is produced with

  the data that is available and may be less accurate than the

  the results that would be produced if the caller did properly

  support split DWARF.

- <span id="lookupresult-map"></span>`fn map<T, F: FnOnce(<L as >::Output) -> T>(self, f: F) -> LookupResult<MappedLookup<T, L, F>>` — [`LookupResult`](#lookupresult), [`MappedLookup`](#mappedlookup)

- <span id="lookupresult-unwrap"></span>`fn unwrap(self) -> <L as >::Output` — [`LookupContinuation`](#lookupcontinuation)

#### Trait Implementations

##### `impl Any for LookupResult<L>`

- <span id="lookupresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LookupResult<L>`

- <span id="lookupresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LookupResult<L>`

- <span id="lookupresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LookupResult<L>`

- <span id="lookupresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LookupResult<L>`

- <span id="lookupresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LookupResult<L>`

- <span id="lookupresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lookupresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LookupResult<L>`

- <span id="lookupresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lookupresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `LookupContinuation`

```rust
trait LookupContinuation: Sized { ... }
```

*Defined in [`addr2line-0.25.1/src/lookup.rs:60-77`](../../../.source_1765633015/addr2line-0.25.1/src/lookup.rs#L60-L77)*

This trait represents a partially complete operation that can be resumed
once a load of needed split DWARF data is completed or abandoned by the
API consumer.

#### Associated Types

- `type Output`

- `type Buf: 1`

#### Required Methods

- `fn resume(self, input: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>`

  Resumes the operation with the provided data.

#### Implementors

- [`LoopingLookup`](#loopinglookup)
- [`MappedLookup`](#mappedlookup)
- [`SimpleLookup`](#simplelookup)

