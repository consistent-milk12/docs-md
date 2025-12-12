*[proc_macro2](../index.md) / [extra](index.md)*

---

# Module `extra`

Items which do not have a correspondence to any API in the proc_macro crate,
but are necessary to include in proc-macro2.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DelimSpan`](#delimspan) | struct | An object that holds a [`Group`]'s `span_open()` and `span_close()` together in a more compact representation than holding those 2 spans individually. |
| [`DelimSpanEnum`](#delimspanenum) | enum |  |
| [`invalidate_current_thread_spans`](#invalidate-current-thread-spans) | fn | Invalidate any `proc_macro2::Span` that exist on the current thread. |

## Structs

### `DelimSpan`

```rust
struct DelimSpan {
    inner: DelimSpanEnum,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

*Defined in [`proc-macro2-1.0.103/src/extra.rs:82-85`](../../../.source_1765521767/proc-macro2-1.0.103/src/extra.rs#L82-L85)*

An object that holds a [`Group`](../index.md)'s `span_open()` and `span_close()` together
in a more compact representation than holding those 2 spans individually.


#### Implementations

- <span id="delimspan-new"></span>`fn new(group: &imp::Group) -> Self` — [`Group`](../imp/index.md#group)

- <span id="delimspan-join"></span>`fn join(&self) -> Span` — [`Span`](../index.md#span)

- <span id="delimspan-open"></span>`fn open(&self) -> Span` — [`Span`](../index.md#span)

- <span id="delimspan-close"></span>`fn close(&self) -> Span` — [`Span`](../index.md#span)

#### Trait Implementations

##### `impl Clone for DelimSpan`

- <span id="delimspan-clone"></span>`fn clone(&self) -> DelimSpan` — [`DelimSpan`](#delimspan)

##### `impl Copy for DelimSpan`

##### `impl Debug for DelimSpan`

- <span id="delimspan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `DelimSpanEnum`

```rust
enum DelimSpanEnum {
    Compiler {
        join: proc_macro::Span,
        open: proc_macro::Span,
        close: proc_macro::Span,
    },
    Fallback(fallback::Span),
}
```

*Defined in [`proc-macro2-1.0.103/src/extra.rs:88-96`](../../../.source_1765521767/proc-macro2-1.0.103/src/extra.rs#L88-L96)*

#### Trait Implementations

##### `impl Clone for DelimSpanEnum`

- <span id="delimspanenum-clone"></span>`fn clone(&self) -> DelimSpanEnum` — [`DelimSpanEnum`](#delimspanenum)

##### `impl Copy for DelimSpanEnum`

## Functions

### `invalidate_current_thread_spans`

```rust
fn invalidate_current_thread_spans()
```

*Defined in [`proc-macro2-1.0.103/src/extra.rs:73-75`](../../../.source_1765521767/proc-macro2-1.0.103/src/extra.rs#L73-L75)*

Invalidate any `proc_macro2::Span` that exist on the current thread.

The implementation of `Span` uses thread-local data structures and this
function clears them. Calling any method on a `Span` on the current thread
created prior to the invalidation will return incorrect values or crash.

This function is useful for programs that process more than 2<sup>32</sup>
bytes of Rust source code on the same thread. Just like rustc, proc-macro2
uses 32-bit source locations, and these wrap around when the total source
code processed by the same thread exceeds 2<sup>32</sup> bytes (4
gigabytes). After a wraparound, `Span` methods such as `source_text()` can
return wrong data.

# Example

As of late 2023, there is 200 GB of Rust code published on crates.io.
Looking at just the newest version of every crate, it is 16 GB of code. So a
workload that involves parsing it all would overflow a 32-bit source
location unless spans are being invalidated.

```rust
use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::io::{BufReader, Read};
use std::str::FromStr;
use tar::Archive;

rayon::scope(|s| {
    for krate in every_version_of_every_crate() {
        s.spawn(move |_| {
            proc_macro2::extra::invalidate_current_thread_spans();

            let reader = BufReader::new(krate);
            let tar = GzDecoder::new(reader);
            let mut archive = Archive::new(tar);
            for entry in archive.entries().unwrap() {
                let mut entry = entry.unwrap();
                let path = entry.path().unwrap();
                if path.extension() != Some(OsStr::new("rs")) {
                    continue;
                }
                let mut content = String::new();
                entry.read_to_string(&mut content).unwrap();
                match proc_macro2::TokenStream::from_str(&content) {
                    Ok(tokens) => {/* ... */},
                    Err(_) => continue,
                }
            }
        });
    }
});

fn every_version_of_every_crate() -> Vec<std::fs::File> {
    Vec::new()
}
```

# Panics

This function is not applicable to and will panic if called from a
procedural macro.

