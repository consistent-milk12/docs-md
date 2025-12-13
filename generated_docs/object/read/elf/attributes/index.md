*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [attributes](index.md)*

---

# Module `attributes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AttributesSection`](#attributessection) | struct | An ELF attributes section. |
| [`AttributesSubsectionIterator`](#attributessubsectioniterator) | struct | An iterator for the subsections in an [`AttributesSection`]. |
| [`AttributesSubsection`](#attributessubsection) | struct | A subsection in an [`AttributesSection`]. |
| [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator) | struct | An iterator for the sub-subsections in an [`AttributesSubsection`]. |
| [`AttributesSubsubsection`](#attributessubsubsection) | struct | A sub-subsection in an [`AttributesSubsection`]. |
| [`AttributeIndexIterator`](#attributeindexiterator) | struct | An iterator over the indices in an [`AttributesSubsubsection`]. |
| [`AttributeReader`](#attributereader) | struct | A parser for the attributes in an [`AttributesSubsubsection`]. |

## Structs

### `AttributesSection<'data, Elf: FileHeader>`

```rust
struct AttributesSection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    version: u8,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:18-22`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L18-L22)*

An ELF attributes section.

This may be a GNU attributes section, or an architecture specific attributes section.

An attributes section contains a series of [`AttributesSubsection`](../index.md).

Returned by [`SectionHeader::attributes`](super::SectionHeader::attributes)
and [`SectionHeader::gnu_attributes`](super::SectionHeader::gnu_attributes).

#### Implementations

- <span id="attributessection-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result)

  Parse an ELF attributes section given the section data.

- <span id="attributessection-version"></span>`fn version(&self) -> u8`

  Return the version of the attributes section.

- <span id="attributessection-subsections"></span>`fn subsections(&self) -> Result<AttributesSubsectionIterator<'data, Elf>>` — [`Result`](../../../index.md#result), [`AttributesSubsectionIterator`](../index.md#attributessubsectioniterator)

  Return an iterator over the subsections.

#### Trait Implementations

##### `impl Any for AttributesSection<'data, Elf>`

- <span id="attributessection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSection<'data, Elf>`

- <span id="attributessection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSection<'data, Elf>`

- <span id="attributessection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSection<'data, Elf>`

- <span id="attributessection-clone"></span>`fn clone(&self) -> AttributesSection<'data, Elf>` — [`AttributesSection`](../index.md#attributessection)

##### `impl CloneToUninit for AttributesSection<'data, Elf>`

- <span id="attributessection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSection<'data, Elf>`

- <span id="attributessection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSection<'data, Elf>`

- <span id="attributessection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSection<'data, Elf>`

- <span id="attributessection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttributesSection<'data, Elf>`

- <span id="attributessection-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSection<'data, Elf>`

- <span id="attributessection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSection<'data, Elf>`

- <span id="attributessection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:61-64`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L61-L64)*

An iterator for the subsections in an [`AttributesSection`](../index.md).

#### Implementations

- <span id="attributessubsectioniterator-next"></span>`fn next(&mut self) -> Result<Option<AttributesSubsection<'data, Elf>>>` — [`Result`](../../../index.md#result), [`AttributesSubsection`](../index.md#attributessubsection)

  Return the next subsection.

- <span id="attributessubsectioniterator-parse"></span>`fn parse(&mut self) -> Result<AttributesSubsection<'data, Elf>>` — [`Result`](../../../index.md#result), [`AttributesSubsection`](../index.md#attributessubsection)

#### Trait Implementations

##### `impl Any for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-clone"></span>`fn clone(&self) -> AttributesSubsectionIterator<'data, Elf>` — [`AttributesSubsectionIterator`](../index.md#attributessubsectioniterator)

##### `impl CloneToUninit for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributessubsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributessubsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-iterator-type-item"></span>`type Item = Result<AttributesSubsection<'data, Elf>, Error>`

- <span id="attributessubsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessubsectioniterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessubsectioniterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessubsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessubsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSubsection<'data, Elf: FileHeader>`

```rust
struct AttributesSubsection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    length: u32,
    vendor: &'data [u8],
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:124-129`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L124-L129)*

A subsection in an [`AttributesSection`](../index.md).

A subsection is identified by a vendor name.  It contains a series of
[`AttributesSubsubsection`](../index.md).

#### Implementations

- <span id="attributessubsection-length"></span>`fn length(&self) -> u32`

  Return the length of the attributes subsection.

- <span id="attributessubsection-vendor"></span>`fn vendor(&self) -> &'data [u8]`

  Return the vendor name of the attributes subsection.

- <span id="attributessubsection-subsubsections"></span>`fn subsubsections(&self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](../index.md#attributessubsubsectioniterator)

  Return an iterator over the sub-subsections.

#### Trait Implementations

##### `impl Any for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-clone"></span>`fn clone(&self) -> AttributesSubsection<'data, Elf>` — [`AttributesSubsection`](../index.md#attributessubsection)

##### `impl CloneToUninit for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessubsection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessubsection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessubsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessubsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSubsubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:153-156`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L153-L156)*

An iterator for the sub-subsections in an [`AttributesSubsection`](../index.md).

#### Implementations

- <span id="attributessubsubsectioniterator-next"></span>`fn next(&mut self) -> Result<Option<AttributesSubsubsection<'data>>>` — [`Result`](../../../index.md#result), [`AttributesSubsubsection`](../index.md#attributessubsubsection)

  Return the next sub-subsection.

- <span id="attributessubsubsectioniterator-parse"></span>`fn parse(&mut self) -> Result<AttributesSubsubsection<'data>>` — [`Result`](../../../index.md#result), [`AttributesSubsubsection`](../index.md#attributessubsubsection)

#### Trait Implementations

##### `impl Any for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-clone"></span>`fn clone(&self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](../index.md#attributessubsubsectioniterator)

##### `impl CloneToUninit for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributessubsubsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributessubsubsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-iterator-type-item"></span>`type Item = Result<AttributesSubsubsection<'data>, Error>`

- <span id="attributessubsubsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessubsubsectioniterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessubsubsectioniterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessubsubsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessubsubsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSubsubsection<'data>`

```rust
struct AttributesSubsubsection<'data> {
    tag: u8,
    length: u32,
    indices: crate::read::Bytes<'data>,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:229-234`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L229-L234)*

A sub-subsection in an [`AttributesSubsection`](../index.md).

A sub-subsection is identified by a tag.  It contains an optional series of indices,
followed by a series of attributes.

#### Implementations

- <span id="attributessubsubsection-tag"></span>`fn tag(&self) -> u8`

  Return the tag of the attributes sub-subsection.

- <span id="attributessubsubsection-length"></span>`fn length(&self) -> u32`

  Return the length of the attributes sub-subsection.

- <span id="attributessubsubsection-indices-data"></span>`fn indices_data(&self) -> &'data [u8]`

  Return the data containing the indices.

- <span id="attributessubsubsection-indices"></span>`fn indices(&self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](../index.md#attributeindexiterator)

  Return the indices.

  

  This will be section indices if the tag is `Tag_Section`,

  or symbol indices if the tag is `Tag_Symbol`,

  and otherwise it will be empty.

- <span id="attributessubsubsection-attributes-data"></span>`fn attributes_data(&self) -> &'data [u8]`

  Return the data containing the attributes.

- <span id="attributessubsubsection-attributes"></span>`fn attributes(&self) -> AttributeReader<'data>` — [`AttributeReader`](../index.md#attributereader)

  Return a parser for the data containing the attributes.

#### Trait Implementations

##### `impl Any for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-clone"></span>`fn clone(&self) -> AttributesSubsubsection<'data>` — [`AttributesSubsubsection`](../index.md#attributessubsubsection)

##### `impl CloneToUninit for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessubsubsection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessubsubsection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessubsubsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessubsubsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributeIndexIterator<'data>`

```rust
struct AttributeIndexIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:274-276`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L274-L276)*

An iterator over the indices in an [`AttributesSubsubsection`](../index.md).

#### Implementations

- <span id="attributeindexiterator-next"></span>`fn next(&mut self) -> Result<Option<u32>>` — [`Result`](../../../index.md#result)

  Parse the next index.

- <span id="attributeindexiterator-parse"></span>`fn parse(&mut self) -> Result<u32>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Any for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-clone"></span>`fn clone(&self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](../index.md#attributeindexiterator)

##### `impl CloneToUninit for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributeindexiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributeindexiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-iterator-type-item"></span>`type Item = Result<u32, Error>`

- <span id="attributeindexiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="attributeindexiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributeindexiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributeindexiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributeindexiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributeReader<'data>`

```rust
struct AttributeReader<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:315-317`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L315-L317)*

A parser for the attributes in an [`AttributesSubsubsection`](../index.md).

The parser relies on the caller to know the format of the data for each attribute tag.

#### Implementations

- <span id="attributereader-read-tag"></span>`fn read_tag(&mut self) -> Result<Option<u64>>` — [`Result`](../../../index.md#result)

  Parse a tag.

- <span id="attributereader-read-integer"></span>`fn read_integer(&mut self) -> Result<u64>` — [`Result`](../../../index.md#result)

  Parse an integer value.

- <span id="attributereader-read-string"></span>`fn read_string(&mut self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

  Parse a string value.

#### Trait Implementations

##### `impl Any for AttributeReader<'data>`

- <span id="attributereader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributeReader<'data>`

- <span id="attributereader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributeReader<'data>`

- <span id="attributereader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttributeReader<'data>`

- <span id="attributereader-clone"></span>`fn clone(&self) -> AttributeReader<'data>` — [`AttributeReader`](../index.md#attributereader)

##### `impl CloneToUninit for AttributeReader<'data>`

- <span id="attributereader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AttributeReader<'data>`

- <span id="attributereader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributeReader<'data>`

- <span id="attributereader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributeReader<'data>`

- <span id="attributereader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttributeReader<'data>`

- <span id="attributereader-toowned-type-owned"></span>`type Owned = T`

- <span id="attributereader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributereader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributeReader<'data>`

- <span id="attributereader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributereader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributeReader<'data>`

- <span id="attributereader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributereader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

