*[gimli](../../index.md) / [read](../index.md) / [addr](index.md)*

---

# Module `addr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAddr`](#debugaddr) | struct | The raw contents of the `.debug_addr` section. |
| [`AddrHeaderIter`](#addrheaderiter) | struct | An iterator over the headers of a `.debug_addr` section. |
| [`AddrHeader`](#addrheader) | struct | A header for a set of entries in the `.debug_addr` section. |
| [`AddrEntryIter`](#addrentryiter) | struct | An iterator over the addresses from a `.debug_addr` section. |

## Structs

### `DebugAddr<R>`

```rust
struct DebugAddr<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:6-8`](../../../../.source_1765521767/gimli-0.32.3/src/read/addr.rs#L6-L8)*

The raw contents of the `.debug_addr` section.

#### Implementations

- <span id="debugaddr-get-address"></span>`fn get_address(&self, address_size: u8, base: DebugAddrBase<<R as >::Offset>, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrBase`](../../index.md#debugaddrbase), [`Reader`](../index.md#reader), [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Result`](../../index.md#result)

  Returns the address at the given `base` and `index`.

  

  A set of addresses in the `.debug_addr` section consists of a header

  followed by a series of addresses.

  

  The `base` must be the `DW_AT_addr_base` value from the compilation unit DIE.

  This is an offset that points to the first address following the header.

  

  The `index` is the value of a `DW_FORM_addrx` attribute.

  

  The `address_size` must be the size of the address for the compilation unit.

  This value must also match the header. However, note that we do not parse the

  header to validate this, since locating the header is unreliable, and the GNU

  extensions do not emit it.

- <span id="debugaddr-headers"></span>`fn headers(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](../index.md#addrheaderiter)

  Iterate the sets of entries in the `.debug_addr` section.

  

  Each set of entries belongs to a single unit.

#### Trait Implementations

##### `impl Any for DebugAddr<R>`

- <span id="debugaddr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAddr<R>`

- <span id="debugaddr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAddr<R>`

- <span id="debugaddr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugAddr<R>`

- <span id="debugaddr-clone"></span>`fn clone(&self) -> DebugAddr<R>` — [`DebugAddr`](../index.md#debugaddr)

##### `impl CloneToUninit for DebugAddr<R>`

- <span id="debugaddr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugAddr<R>`

##### `impl<R: fmt::Debug> Debug for DebugAddr<R>`

- <span id="debugaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAddr<R>`

- <span id="debugaddr-default"></span>`fn default() -> DebugAddr<R>` — [`DebugAddr`](../index.md#debugaddr)

##### `impl<T> From for DebugAddr<R>`

- <span id="debugaddr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugAddr<R>`

- <span id="debugaddr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugAddr<R>`

- <span id="debugaddr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugaddr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugAddr<R>`

- <span id="debugaddr-toowned-type-owned"></span>`type Owned = T`

- <span id="debugaddr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugaddr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugAddr<R>`

- <span id="debugaddr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugaddr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugAddr<R>`

- <span id="debugaddr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugaddr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AddrHeaderIter<R: Reader>`

```rust
struct AddrHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugAddrOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:82-85`](../../../../.source_1765521767/gimli-0.32.3/src/read/addr.rs#L82-L85)*

An iterator over the headers of a `.debug_addr` section.

#### Implementations

- <span id="addrheaderiter-next"></span>`fn next(&mut self) -> Result<Option<AddrHeader<R>>>` — [`Result`](../../index.md#result), [`AddrHeader`](../index.md#addrheader)

  Advance the iterator to the next header.

#### Trait Implementations

##### `impl Any for AddrHeaderIter<R>`

- <span id="addrheaderiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AddrHeaderIter<R>`

- <span id="addrheaderiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AddrHeaderIter<R>`

- <span id="addrheaderiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for AddrHeaderIter<R>`

- <span id="addrheaderiter-clone"></span>`fn clone(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](../index.md#addrheaderiter)

##### `impl CloneToUninit for AddrHeaderIter<R>`

- <span id="addrheaderiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for AddrHeaderIter<R>`

- <span id="addrheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AddrHeaderIter<R>`

- <span id="addrheaderiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AddrHeaderIter<R>`

- <span id="addrheaderiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AddrHeaderIter<R>`

- <span id="addrheaderiter-toowned-type-owned"></span>`type Owned = T`

- <span id="addrheaderiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="addrheaderiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AddrHeaderIter<R>`

- <span id="addrheaderiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="addrheaderiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AddrHeaderIter<R>`

- <span id="addrheaderiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="addrheaderiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AddrHeader<R, Offset>`

```rust
struct AddrHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugAddrOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    entries: R,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:122-131`](../../../../.source_1765521767/gimli-0.32.3/src/read/addr.rs#L122-L131)*

A header for a set of entries in the `.debug_addr` section.

These entries all belong to a single unit.

#### Implementations

- <span id="addrheader-parse"></span>`fn parse(input: &mut R, offset: DebugAddrOffset<Offset>) -> Result<Self>` — [`DebugAddrOffset`](../../index.md#debugaddroffset), [`Result`](../../index.md#result)

- <span id="addrheader-offset"></span>`fn offset(&self) -> DebugAddrOffset<Offset>` — [`DebugAddrOffset`](../../index.md#debugaddroffset)

  Return the offset of this header within the `.debug_addr` section.

- <span id="addrheader-length"></span>`fn length(&self) -> Offset`

  Return the length of this set of entries, including the header.

- <span id="addrheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md#encoding)

  Return the encoding parameters for this set of entries.

- <span id="addrheader-entries"></span>`fn entries(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](../index.md#addrentryiter)

  Return the address entries in this set.

#### Trait Implementations

##### `impl Any for AddrHeader<R, Offset>`

- <span id="addrheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AddrHeader<R, Offset>`

- <span id="addrheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AddrHeader<R, Offset>`

- <span id="addrheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for AddrHeader<R, Offset>`

- <span id="addrheader-clone"></span>`fn clone(&self) -> AddrHeader<R, Offset>` — [`AddrHeader`](../index.md#addrheader)

##### `impl CloneToUninit for AddrHeader<R, Offset>`

- <span id="addrheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for AddrHeader<R, Offset>`

- <span id="addrheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for AddrHeader<R, Offset>`

##### `impl<T> From for AddrHeader<R, Offset>`

- <span id="addrheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AddrHeader<R, Offset>`

- <span id="addrheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for AddrHeader<R, Offset>`

- <span id="addrheader-partialeq-eq"></span>`fn eq(&self, other: &AddrHeader<R, Offset>) -> bool` — [`AddrHeader`](../index.md#addrheader)

##### `impl<R, Offset> StructuralPartialEq for AddrHeader<R, Offset>`

##### `impl ToOwned for AddrHeader<R, Offset>`

- <span id="addrheader-toowned-type-owned"></span>`type Owned = T`

- <span id="addrheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="addrheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AddrHeader<R, Offset>`

- <span id="addrheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="addrheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AddrHeader<R, Offset>`

- <span id="addrheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="addrheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AddrEntryIter<R: Reader>`

```rust
struct AddrEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:217-220`](../../../../.source_1765521767/gimli-0.32.3/src/read/addr.rs#L217-L220)*

An iterator over the addresses from a `.debug_addr` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="addrentryiter-next"></span>`fn next(&mut self) -> Result<Option<u64>>` — [`Result`](../../index.md#result)

  Advance the iterator and return the next address.

  

  Returns the newly parsed address as `Ok(Some(addr))`. Returns `Ok(None)`

  when iteration is complete and all addresses have already been parsed and

  yielded. If an error occurs while parsing the next address, then this error

  is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.

#### Trait Implementations

##### `impl Any for AddrEntryIter<R>`

- <span id="addrentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AddrEntryIter<R>`

- <span id="addrentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AddrEntryIter<R>`

- <span id="addrentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for AddrEntryIter<R>`

- <span id="addrentryiter-clone"></span>`fn clone(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](../index.md#addrentryiter)

##### `impl CloneToUninit for AddrEntryIter<R>`

- <span id="addrentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for AddrEntryIter<R>`

- <span id="addrentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AddrEntryIter<R>`

- <span id="addrentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AddrEntryIter<R>`

- <span id="addrentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AddrEntryIter<R>`

- <span id="addrentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="addrentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="addrentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AddrEntryIter<R>`

- <span id="addrentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="addrentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AddrEntryIter<R>`

- <span id="addrentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="addrentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

