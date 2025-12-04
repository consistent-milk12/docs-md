*[syn](../index.md) / [buffer](index.md)*

---

# Module `buffer`

A stably addressed token buffer supporting efficient traversal based on a
cheaply copyable cursor.

## Structs

### `TokenBuffer`

```rust
struct TokenBuffer {
}
```

A buffer that can be efficiently traversed multiple times, unlike
`TokenStream` which requires a deep copy in order to traverse more than
once.

#### Implementations

- `fn new(stream: proc_macro::TokenStream) -> Self`
  Creates a `TokenBuffer` containing all the tokens from the input

- `fn new2(stream: TokenStream) -> Self`
  Creates a `TokenBuffer` containing all the tokens from the input

- `fn begin(self: &Self) -> Cursor<'_>`
  Creates a cursor referencing the first token in the buffer and able to

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Cursor<'a>`

```rust
struct Cursor<'a> {
}
```

A cheaply copyable cursor into a `TokenBuffer`.

This cursor holds a shared reference into the immutable data which is used
internally to represent a `TokenStream`, and can be efficiently manipulated
and copied around.

An empty `Cursor` can be created directly, or one may create a `TokenBuffer`
object and get a cursor to its first token with `begin()`.

#### Implementations

- `fn empty() -> Self`
  Creates a cursor referencing a static empty TokenStream.

- `fn eof(self: Self) -> bool`
  Checks whether the cursor is currently pointing at the end of its valid

- `fn ident(self: Self) -> Option<(Ident, Cursor<'a>)>`
  If the cursor is pointing at a `Ident`, returns it along with a cursor

- `fn punct(self: Self) -> Option<(Punct, Cursor<'a>)>`
  If the cursor is pointing at a `Punct`, returns it along with a cursor

- `fn literal(self: Self) -> Option<(Literal, Cursor<'a>)>`
  If the cursor is pointing at a `Literal`, return it along with a cursor

- `fn lifetime(self: Self) -> Option<(Lifetime, Cursor<'a>)>`
  If the cursor is pointing at a `Lifetime`, returns it along with a

- `fn group(self: Self, delim: Delimiter) -> Option<(Cursor<'a>, DelimSpan, Cursor<'a>)>`
  If the cursor is pointing at a `Group` with the given delimiter, returns

- `fn any_group(self: Self) -> Option<(Cursor<'a>, Delimiter, DelimSpan, Cursor<'a>)>`
  If the cursor is pointing at a `Group`, returns a cursor into the group

- `fn token_stream(self: Self) -> TokenStream`
  Copies all remaining tokens visible from this cursor into a

- `fn token_tree(self: Self) -> Option<(TokenTree, Cursor<'a>)>`
  If the cursor is pointing at a `TokenTree`, returns it along with a

- `fn span(self: Self) -> Span`
  Returns the `Span` of the current token, or `Span::call_site()` if this

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

##### `impl Eq<'a>`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl PartialOrd<'a>`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

