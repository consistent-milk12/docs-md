*[gimli](../../index.md) / [read](../index.md) / [lookup](index.md)*

---

# Module `lookup`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugLookup`](#debuglookup) | struct |  |
| [`LookupEntryIter`](#lookupentryiter) | struct |  |
| [`PubStuffHeader`](#pubstuffheader) | struct |  |
| [`PubStuffParser`](#pubstuffparser) | struct |  |
| [`LookupParser`](#lookupparser) | trait |  |
| [`PubStuffEntry`](#pubstuffentry) | trait |  |

## Structs

### `DebugLookup<R, Parser>`

```rust
struct DebugLookup<R, Parser>
where
    R: Reader,
    Parser: LookupParser<R> {
    input_buffer: R,
    phantom: core::marker::PhantomData<Parser>,
}
```

*Defined in [`gimli-0.32.3/src/read/lookup.rs:32-39`](../../../../.source_1765521767/gimli-0.32.3/src/read/lookup.rs#L32-L39)*

#### Implementations

- <span id="debuglookup-items"></span>`fn items(&self) -> LookupEntryIter<R, Parser>` — [`LookupEntryIter`](#lookupentryiter)

- <span id="debuglookup-reader"></span>`fn reader(&self) -> &R`

#### Trait Implementations

##### `impl Any for DebugLookup<R, Parser>`

- <span id="debuglookup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLookup<R, Parser>`

- <span id="debuglookup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLookup<R, Parser>`

- <span id="debuglookup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Parser> Clone for DebugLookup<R, Parser>`

- <span id="debuglookup-clone"></span>`fn clone(&self) -> DebugLookup<R, Parser>` — [`DebugLookup`](#debuglookup)

##### `impl CloneToUninit for DebugLookup<R, Parser>`

- <span id="debuglookup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Parser> Debug for DebugLookup<R, Parser>`

- <span id="debuglookup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugLookup<R, Parser>`

- <span id="debuglookup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLookup<R, Parser>`

- <span id="debuglookup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DebugLookup<R, Parser>`

- <span id="debuglookup-toowned-type-owned"></span>`type Owned = T`

- <span id="debuglookup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debuglookup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugLookup<R, Parser>`

- <span id="debuglookup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuglookup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLookup<R, Parser>`

- <span id="debuglookup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuglookup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LookupEntryIter<R, Parser>`

```rust
struct LookupEntryIter<R, Parser>
where
    R: Reader,
    Parser: LookupParser<R> {
    current_set: Option<(R, <Parser as >::Header)>,
    remaining_input: R,
}
```

*Defined in [`gimli-0.32.3/src/read/lookup.rs:72-79`](../../../../.source_1765521767/gimli-0.32.3/src/read/lookup.rs#L72-L79)*

#### Implementations

- <span id="lookupentryiter-next"></span>`fn next(&mut self) -> Result<Option<<Parser as >::Entry>>` — [`Result`](../../index.md#result), [`LookupParser`](#lookupparser)

  Advance the iterator and return the next entry.

  

  Returns the newly parsed entry as `Ok(Some(Parser::Entry))`. Returns

  `Ok(None)` when iteration is complete and all entries have already been

  parsed and yielded. If an error occurs while parsing the next entry,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

  

  Can be [used with `FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Trait Implementations

##### `impl Any for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Parser> Clone for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-clone"></span>`fn clone(&self) -> LookupEntryIter<R, Parser>` — [`LookupEntryIter`](#lookupentryiter)

##### `impl CloneToUninit for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Parser> Debug for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="lookupentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lookupentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lookupentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lookupentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PubStuffHeader<T>`

```rust
struct PubStuffHeader<T> {
    format: crate::common::Format,
    length: T,
    version: u16,
    unit_offset: crate::common::DebugInfoOffset<T>,
    unit_length: T,
}
```

*Defined in [`gimli-0.32.3/src/read/lookup.rs:129-135`](../../../../.source_1765521767/gimli-0.32.3/src/read/lookup.rs#L129-L135)*

#### Trait Implementations

##### `impl<T> Any for PubStuffHeader<T>`

- <span id="pubstuffheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubStuffHeader<T>`

- <span id="pubstuffheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubStuffHeader<T>`

- <span id="pubstuffheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for PubStuffHeader<T>`

- <span id="pubstuffheader-clone"></span>`fn clone(&self) -> PubStuffHeader<T>` — [`PubStuffHeader`](#pubstuffheader)

##### `impl<T> CloneToUninit for PubStuffHeader<T>`

- <span id="pubstuffheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for PubStuffHeader<T>`

- <span id="pubstuffheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for PubStuffHeader<T>`

##### `impl<T> From for PubStuffHeader<T>`

- <span id="pubstuffheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for PubStuffHeader<T>`

- <span id="pubstuffheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for PubStuffHeader<T>`

- <span id="pubstuffheader-partialeq-eq"></span>`fn eq(&self, other: &PubStuffHeader<T>) -> bool` — [`PubStuffHeader`](#pubstuffheader)

##### `impl<T> StructuralPartialEq for PubStuffHeader<T>`

##### `impl<T> ToOwned for PubStuffHeader<T>`

- <span id="pubstuffheader-toowned-type-owned"></span>`type Owned = T`

- <span id="pubstuffheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubstuffheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for PubStuffHeader<T>`

- <span id="pubstuffheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubstuffheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for PubStuffHeader<T>`

- <span id="pubstuffheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubstuffheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PubStuffParser<R, Entry>`

```rust
struct PubStuffParser<R, Entry>
where
    R: Reader,
    Entry: PubStuffEntry<R> {
    phantom: core::marker::PhantomData<(R, Entry)>,
}
```

*Defined in [`gimli-0.32.3/src/read/lookup.rs:146-153`](../../../../.source_1765521767/gimli-0.32.3/src/read/lookup.rs#L146-L153)*

#### Trait Implementations

##### `impl Any for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Entry> Clone for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-clone"></span>`fn clone(&self) -> PubStuffParser<R, Entry>` — [`PubStuffParser`](#pubstuffparser)

##### `impl CloneToUninit for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Entry> Debug for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Entry> LookupParser for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-lookupparser-type-header"></span>`type Header = PubStuffHeader<<R as Reader>::Offset>`

- <span id="pubstuffparser-lookupparser-type-entry"></span>`type Entry = Entry`

- <span id="pubstuffparser-lookupparser-parse-header"></span>`fn parse_header(input: &mut R) -> Result<(R, <Self as >::Header)>` — [`Result`](../../index.md#result), [`LookupParser`](#lookupparser)

  Parse an pubthings set header. Returns a tuple of the

  pubthings to be parsed for this set, and the newly created PubThingHeader struct.

- <span id="pubstuffparser-lookupparser-parse-entry"></span>`fn parse_entry(input: &mut R, header: &<Self as >::Header) -> Result<Option<<Self as >::Entry>>` — [`LookupParser`](#lookupparser), [`Result`](../../index.md#result)

  Parse a single pubthing. Return `None` for the null pubthing, `Some` for an actual pubthing.

##### `impl ToOwned for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-toowned-type-owned"></span>`type Owned = T`

- <span id="pubstuffparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubstuffparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubstuffparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubstuffparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `LookupParser<R: Reader>`

```rust
trait LookupParser<R: Reader> { ... }
```

*Defined in [`gimli-0.32.3/src/read/lookup.rs:15-29`](../../../../.source_1765521767/gimli-0.32.3/src/read/lookup.rs#L15-L29)*

#### Associated Types

- `type Header`

- `type Entry`

#### Required Methods

- `fn parse_header(input: &mut R) -> Result<(R, <Self as >::Header)>`

  Parse a header from `input`. Returns a tuple of `input` sliced to contain just the entries

- `fn parse_entry(input: &mut R, header: &<Self as >::Header) -> Result<Option<<Self as >::Entry>>`

  Parse a single entry from `input`. Returns either a parsed representation of the entry

#### Implementors

- [`PubStuffParser`](#pubstuffparser)

### `PubStuffEntry<R: Reader>`

```rust
trait PubStuffEntry<R: Reader> { ... }
```

*Defined in [`gimli-0.32.3/src/read/lookup.rs:137-143`](../../../../.source_1765521767/gimli-0.32.3/src/read/lookup.rs#L137-L143)*

#### Required Methods

- `fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self`

#### Implementors

- [`PubNamesEntry`](../index.md#pubnamesentry)
- [`PubTypesEntry`](../index.md#pubtypesentry)

