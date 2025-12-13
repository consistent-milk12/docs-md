*[compact_str](../../index.md) / [repr](../index.md) / [last_utf8_char](index.md)*

---

# Module `last_utf8_char`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LastByte`](#lastbyte) | enum | [`LastByte`] is an unsigned 8-bit integer data type that has a valid range of `[0, 217]`. |

## Enums

### `LastByte`

```rust
enum LastByte {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
    V32,
    V33,
    V34,
    V35,
    V36,
    V37,
    V38,
    V39,
    V40,
    V41,
    V42,
    V43,
    V44,
    V45,
    V46,
    V47,
    V48,
    V49,
    V50,
    V51,
    V52,
    V53,
    V54,
    V55,
    V56,
    V57,
    V58,
    V59,
    V60,
    V61,
    V62,
    V63,
    V64,
    V65,
    V66,
    V67,
    V68,
    V69,
    V70,
    V71,
    V72,
    V73,
    V74,
    V75,
    V76,
    V77,
    V78,
    V79,
    V80,
    V81,
    V82,
    V83,
    V84,
    V85,
    V86,
    V87,
    V88,
    V89,
    V90,
    V91,
    V92,
    V93,
    V94,
    V95,
    V96,
    V97,
    V98,
    V99,
    V100,
    V101,
    V102,
    V103,
    V104,
    V105,
    V106,
    V107,
    V108,
    V109,
    V110,
    V111,
    V112,
    V113,
    V114,
    V115,
    V116,
    V117,
    V118,
    V119,
    V120,
    V121,
    V122,
    V123,
    V124,
    V125,
    V126,
    V127,
    V128,
    V129,
    V130,
    V131,
    V132,
    V133,
    V134,
    V135,
    V136,
    V137,
    V138,
    V139,
    V140,
    V141,
    V142,
    V143,
    V144,
    V145,
    V146,
    V147,
    V148,
    V149,
    V150,
    V151,
    V152,
    V153,
    V154,
    V155,
    V156,
    V157,
    V158,
    V159,
    V160,
    V161,
    V162,
    V163,
    V164,
    V165,
    V166,
    V167,
    V168,
    V169,
    V170,
    V171,
    V172,
    V173,
    V174,
    V175,
    V176,
    V177,
    V178,
    V179,
    V180,
    V181,
    V182,
    V183,
    V184,
    V185,
    V186,
    V187,
    V188,
    V189,
    V190,
    V191,
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    L10,
    L11,
    L12,
    L13,
    L14,
    L15,
    L16,
    L17,
    L18,
    L19,
    L20,
    L21,
    L22,
    L23,
    Heap,
    Static,
}
```

*Defined in [`compact_str-0.9.0/src/repr/last_utf8_char.rs:12-242`](../../../../.source_1765633015/compact_str-0.9.0/src/repr/last_utf8_char.rs#L12-L242)*

[`LastByte`](#lastbyte) is an unsigned 8-bit integer data type that has a valid range of `[0, 217]`.
Excluding `[218, 255]` allows the Rust compiler to use these values as niches.

Specifically the compiler can use a value in this range to encode the `None` variant of
`Option<LastByte>` allowing:
`std::mem::size_of::<LastByte> == std::mem::size_of::<Option<LastByte>>()`

#### Trait Implementations

##### `impl Any for LastByte`

- <span id="lastbyte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LastByte`

- <span id="lastbyte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LastByte`

- <span id="lastbyte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LastByte`

- <span id="lastbyte-clone"></span>`fn clone(&self) -> LastByte` — [`LastByte`](#lastbyte)

##### `impl CloneToUninit for LastByte`

- <span id="lastbyte-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LastByte`

##### `impl Debug for LastByte`

- <span id="lastbyte-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LastByte`

- <span id="lastbyte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LastByte`

- <span id="lastbyte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LastByte`

- <span id="lastbyte-toowned-type-owned"></span>`type Owned = T`

- <span id="lastbyte-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lastbyte-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LastByte`

- <span id="lastbyte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lastbyte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LastByte`

- <span id="lastbyte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lastbyte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

