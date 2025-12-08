*[syn](../index.md) / [tt](index.md)*

---

# Module `tt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokenTreeHelper`](#tokentreehelper) | struct |  |
| [`TokenStreamHelper`](#tokenstreamhelper) | struct |  |

## Structs

### `TokenTreeHelper<'a>`

```rust
struct TokenTreeHelper<'a>(&'a proc_macro2::TokenTree);
```

#### Trait Implementations

##### `impl<'a> Hash for TokenTreeHelper<'a>`

- <span id="tokentreehelper-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl<'a> PartialEq for TokenTreeHelper<'a>`

- <span id="tokentreehelper-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `TokenStreamHelper<'a>`

```rust
struct TokenStreamHelper<'a>(&'a proc_macro2::TokenStream);
```

#### Trait Implementations

##### `impl<'a> Hash for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<'a> PartialEq for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-eq"></span>`fn eq(&self, other: &Self) -> bool`

