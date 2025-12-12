*[rustversion](../index.md) / [release](index.md)*

---

# Module `release`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Release`](#release) | struct |  |
| [`parse`](#parse) | fn |  |
| [`try_parse`](#try-parse) | fn |  |

## Structs

### `Release`

```rust
struct Release {
    pub minor: u16,
    pub patch: Option<u16>,
}
```

*Defined in [`rustversion-1.0.22/src/release.rs:7-10`](../../../.source_1765521767/rustversion-1.0.22/src/release.rs#L7-L10)*

#### Trait Implementations

##### `impl Clone for Release`

- <span id="release-clone"></span>`fn clone(&self) -> Release` — [`Release`](#release)

##### `impl Copy for Release`

##### `impl Debug for Release`

- <span id="release-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Release`

##### `impl Ord for Release`

- <span id="release-cmp"></span>`fn cmp(&self, other: &Release) -> cmp::Ordering` — [`Release`](#release)

##### `impl PartialEq for Release`

- <span id="release-eq"></span>`fn eq(&self, other: &Release) -> bool` — [`Release`](#release)

##### `impl PartialOrd for Release`

- <span id="release-partial-cmp"></span>`fn partial_cmp(&self, other: &Release) -> option::Option<cmp::Ordering>` — [`Release`](#release)

##### `impl StructuralPartialEq for Release`

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Release, Error>
```

*Defined in [`rustversion-1.0.22/src/release.rs:12-14`](../../../.source_1765521767/rustversion-1.0.22/src/release.rs#L12-L14)*

### `try_parse`

```rust
fn try_parse(iter: &'_ mut IterImpl) -> std::result::Result<Release, ()>
```

*Defined in [`rustversion-1.0.22/src/release.rs:16-34`](../../../.source_1765521767/rustversion-1.0.22/src/release.rs#L16-L34)*

