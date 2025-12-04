# Crate `clap_lex`

Minimal, flexible command-line parser

As opposed to a declarative parser, this processes arguments as a stream of tokens.  As lexing
a command-line is not context-free, we rely on the caller to decide how to interpret the
arguments.

# Examples

```rust
use std::path::PathBuf;
use std::ffi::OsStr;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
struct Args {
    paths: Vec<PathBuf>,
    color: Color,
    verbosity: usize,
}

#[derive(Debug)]
enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn parse(s: Option<&OsStr>) -> Result<Self, BoxedError> {
        let s = s.map(|s| s.to_str().ok_or(s));
        match s {
            Some(Ok("always")) | Some(Ok("")) | None => {
                Ok(Color::Always)
            }
            Some(Ok("auto")) => {
                Ok(Color::Auto)
            }
            Some(Ok("never")) => {
                Ok(Color::Never)
            }
            Some(invalid) => {
                Err(format!("Invalid value for `--color`, {invalid:?}").into())
            }
        }
    }
}

fn parse_args(
    raw: impl IntoIterator<Item=impl Into<std::ffi::OsString>>
) -> Result<Args, BoxedError> {
    let mut args = Args {
        paths: Vec::new(),
        color: Color::Auto,
        verbosity: 0,
    };

    let raw = clap_lex::RawArgs::new(raw);
    let mut cursor = raw.cursor();
    raw.next(&mut cursor);  // Skip the bin
    while let Some(arg) = raw.next(&mut cursor) {
        if arg.is_escape() {
            args.paths.extend(raw.remaining(&mut cursor).map(PathBuf::from));
        } else if arg.is_stdio() {
            args.paths.push(PathBuf::from("-"));
        } else if let Some((long, value)) = arg.to_long() {
            match long {
                Ok("verbose") => {
                    if let Some(value) = value {
                        return Err(format!("`--verbose` does not take a value, got `{value:?}`").into());
                    }
                    args.verbosity += 1;
                }
                Ok("color") => {
                    args.color = Color::parse(value)?;
                }
                _ => {
                    return Err(
                        format!("Unexpected flag: --{}", arg.display()).into()
                    );
                }
            }
        } else if let Some(mut shorts) = arg.to_short() {
            while let Some(short) = shorts.next_flag() {
                match short {
                    Ok('v') => {
                        args.verbosity += 1;
                    }
                    Ok('c') => {
                        let value = shorts.next_value_os();
                        args.color = Color::parse(value)?;
                    }
                    Ok(c) => {
                        return Err(format!("Unexpected flag: -{c}").into());
                    }
                    Err(e) => {
                        return Err(format!("Unexpected flag: -{}", e.to_string_lossy()).into());
                    }
                }
            }
        } else {
            args.paths.push(PathBuf::from(arg.to_value_os().to_owned()));
        }
    }

    Ok(args)
}

let args = parse_args(["bin", "--hello", "world"]);
println!("{args:?}");
```

## Structs

### `RawArgs`

```rust
struct RawArgs {
    // [REDACTED: Private Fields]
}
```

Command-line arguments

#### Implementations

- `fn from_args() -> Self`
  

- `fn new(iter: impl IntoIterator<Item = impl Into<OsString>>) -> Self`
  

- `fn cursor(self: &Self) -> ArgCursor`
  Create a cursor for walking the arguments

- `fn next(self: &Self, cursor: &mut ArgCursor) -> Option<ParsedArg<'_>>`
  Advance the cursor, returning the next [`ParsedArg`]

- `fn next_os(self: &Self, cursor: &mut ArgCursor) -> Option<&OsStr>`
  Advance the cursor, returning a raw argument value.

- `fn peek(self: &Self, cursor: &ArgCursor) -> Option<ParsedArg<'_>>`
  Return the next [`ParsedArg`]

- `fn peek_os(self: &Self, cursor: &ArgCursor) -> Option<&OsStr>`
  Return a raw argument value.

- `fn remaining(self: &Self, cursor: &mut ArgCursor) -> impl Iterator<Item = &OsStr>`
  Return all remaining raw arguments, advancing the cursor to the end

- `fn seek(self: &Self, cursor: &mut ArgCursor, pos: SeekFrom)`
  Adjust the cursor's position

- `fn insert(self: &mut Self, cursor: &ArgCursor, insert_items: impl IntoIterator<Item = impl Into<OsString>>)`
  Inject arguments before the [`RawArgs::next`]

- `fn is_end(self: &Self, cursor: &ArgCursor) -> bool`
  Any remaining args?

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<I, T>`

- `fn from(val: I) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> RawArgs`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &RawArgs) -> bool`

##### `impl StructuralPartialEq`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> RawArgs`

### `ArgCursor`

```rust
struct ArgCursor {
    // [REDACTED: Private Fields]
}
```

Position within [`RawArgs`](#rawargs)

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

##### `impl Clone`

- `fn clone(self: &Self) -> ArgCursor`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &ArgCursor) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ArgCursor) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &ArgCursor) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ParsedArg<'s>`

```rust
struct ParsedArg<'s> {
    // [REDACTED: Private Fields]
}
```

Command-line Argument

#### Implementations

- `fn is_empty(self: &Self) -> bool`
  Argument is length of 0

- `fn is_stdio(self: &Self) -> bool`
  Does the argument look like a stdio argument (`-`)

- `fn is_escape(self: &Self) -> bool`
  Does the argument look like an argument escape (`--`)

- `fn is_negative_number(self: &Self) -> bool`
  Does the argument look like a negative number?

- `fn to_long(self: &Self) -> Option<(Result<&str, &OsStr>, Option<&OsStr>)>`
  Treat as a long-flag

- `fn is_long(self: &Self) -> bool`
  Can treat as a long-flag

- `fn to_short(self: &Self) -> Option<ShortFlags<'_>>`
  Treat as a short-flag

- `fn is_short(self: &Self) -> bool`
  Can treat as a short-flag

- `fn to_value_os(self: &Self) -> &OsStr`
  Treat as a value

- `fn to_value(self: &Self) -> Result<&str, &OsStr>`
  Treat as a value

- `fn display(self: &Self) -> impl std::fmt::Display + '_`
  Safely print an argument that may contain non-UTF8 content

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

##### `impl Clone<'s>`

- `fn clone(self: &Self) -> ParsedArg<'s>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'s>`

##### `impl Hash<'s>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<'s>`

- `fn cmp(self: &Self, other: &ParsedArg<'s>) -> $crate::cmp::Ordering`

##### `impl PartialEq<'s>`

- `fn eq(self: &Self, other: &ParsedArg<'s>) -> bool`

##### `impl PartialOrd<'s>`

- `fn partial_cmp(self: &Self, other: &ParsedArg<'s>) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq<'s>`

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

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ShortFlags<'s>`

```rust
struct ShortFlags<'s> {
    // [REDACTED: Private Fields]
}
```

Walk through short flags within a [`ParsedArg`](#parsedarg)

#### Implementations

- `fn advance_by(self: &mut Self, n: usize) -> Result<(), usize>`
  Move the iterator forward by `n` short flags

- `fn is_empty(self: &Self) -> bool`
  No short flags left

- `fn is_negative_number(self: &Self) -> bool`
  Does the short flag look like a number

- `fn next_flag(self: &mut Self) -> Option<Result<char, &'s OsStr>>`
  Advance the iterator, returning the next short flag on success

- `fn next_value_os(self: &mut Self) -> Option<&'s OsStr>`
  Advance the iterator, returning everything left as a value

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'s>`

- `fn clone(self: &Self) -> ShortFlags<'s>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Iterator<'s>`

- `type Item = Result<char, &'s OsStr>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

## Functions

