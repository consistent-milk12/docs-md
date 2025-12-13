*[serde_derive](../../index.md) / [internals](../index.md) / [case](index.md)*

---

# Module `case`

Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the
case of the source (e.g. `my-field`, `MY_FIELD`).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParseError`](#parseerror) | struct |  |
| [`RenameRule`](#renamerule) | enum | The different possible ways to change case of fields in a struct, or variants in an enum. |

## Structs

### `ParseError<'a>`

```rust
struct ParseError<'a> {
    unknown: &'a str,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/case.rs:120-122`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/case.rs#L120-L122)*

#### Trait Implementations

##### `impl Any for ParseError<'a>`

- <span id="parseerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseError<'a>`

- <span id="parseerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseError<'a>`

- <span id="parseerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for ParseError<'a>`

- <span id="parseerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ParseError<'a>`

- <span id="parseerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseError<'a>`

- <span id="parseerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for ParseError<'a>`

- <span id="parseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ParseError<'a>`

- <span id="parseerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseError<'a>`

- <span id="parseerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `RenameRule`

```rust
enum RenameRule {
    None,
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/case.rs:9-31`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/case.rs#L9-L31)*

The different possible ways to change case of fields in a struct, or variants in an enum.

#### Variants

- **`None`**

  Don't apply a default rename rule.

- **`LowerCase`**

  Rename direct children to "lowercase" style.

- **`UpperCase`**

  Rename direct children to "UPPERCASE" style.

- **`PascalCase`**

  Rename direct children to "PascalCase" style, as typically used for
  enum variants.

- **`CamelCase`**

  Rename direct children to "camelCase" style.

- **`SnakeCase`**

  Rename direct children to "snake_case" style, as commonly used for
  fields.

- **`ScreamingSnakeCase`**

  Rename direct children to "SCREAMING_SNAKE_CASE" style, as commonly
  used for constants.

- **`KebabCase`**

  Rename direct children to "kebab-case" style.

- **`ScreamingKebabCase`**

  Rename direct children to "SCREAMING-KEBAB-CASE" style.

#### Implementations

- <span id="renamerule-from-str"></span>`fn from_str(rename_all_str: &str) -> Result<Self, ParseError<'_>>` — [`ParseError`](#parseerror)

- <span id="renamerule-apply-to-variant"></span>`fn apply_to_variant(self, variant: &str) -> String`

  Apply a renaming rule to an enum variant, returning the version expected in the source.

- <span id="renamerule-apply-to-field"></span>`fn apply_to_field(self, field: &str) -> String`

  Apply a renaming rule to a struct field, returning the version expected in the source.

- <span id="renamerule-or"></span>`fn or(self, rule_b: Self) -> Self`

  Returns the `RenameRule` if it is not `None`, `rule_b` otherwise.

#### Trait Implementations

##### `impl Any for RenameRule`

- <span id="renamerule-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RenameRule`

- <span id="renamerule-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RenameRule`

- <span id="renamerule-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RenameRule`

- <span id="renamerule-clone"></span>`fn clone(&self) -> RenameRule` — [`RenameRule`](#renamerule)

##### `impl CloneToUninit for RenameRule`

- <span id="renamerule-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RenameRule`

##### `impl<T> From for RenameRule`

- <span id="renamerule-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RenameRule`

- <span id="renamerule-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RenameRule`

- <span id="renamerule-partialeq-eq"></span>`fn eq(&self, other: &RenameRule) -> bool` — [`RenameRule`](#renamerule)

##### `impl StructuralPartialEq for RenameRule`

##### `impl ToOwned for RenameRule`

- <span id="renamerule-toowned-type-owned"></span>`type Owned = T`

- <span id="renamerule-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="renamerule-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RenameRule`

- <span id="renamerule-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="renamerule-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RenameRule`

- <span id="renamerule-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="renamerule-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

