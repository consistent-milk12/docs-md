# Crate `backtrace_ext`

Minor conveniences on top of the backtrace crate

See [`short_frames_strict`][] for details.

## Functions

### `short_frames_strict`

```rust
fn short_frames_strict(backtrace: &Backtrace) -> impl Iterator<Item = (&BacktraceFrame, std::ops::Range<usize>)>
```

Gets an iterator over the frames that are part of Rust's "short backtrace" range.
If no such range is found, the full stack is yielded.

Rust generally tries to include special frames on the stack called `rust_end_short_backtrace`
and `rust_begin_short_backtrace` which delimit the "real" stackframes from "gunk" stackframes
like setting up main and invoking the panic runtime. This yields all the "real" frames between
those two (which theoretically can be nothing with enough optimization, although that's unlikely
for any non-trivial program).

If only one of the special frames is present we will only clamp one side of the stack
(similar to `a..` or `..a`). If the special frames are in the wrong order we will discard
them and produce the full stack. If multiple versions of a special frame are found
(I've seen it in the wild), we will pick the "innermost" ones, producing the smallest
possible backtrace (and excluding all special frames from the output).

Each element of the iterator includes a Range which you should use to slice
the frame's `symbols()` array. This handles the theoretical situation where "real" frames
got inlined together with the special marker frames. I want to believe this can't happen
but you can never trust backtraces to be reasonable! We will never yield a Frame to you
with an empty Range.

Note that some "gunk" frames may still be found within the short backtrace, as there is still some
platform-specific and optimization-specific glue around the edges because compilers are
complicated and nothing's perfect. This can include:

* `core::ops::function::FnOnce::call_once`
* `std::panicking::begin_panic_handler`
* `core::panicking::panic_fmt`
* `rust_begin_unwind`

In the future we may introduce a non-strict short_frames which heuristically filters
those frames out too. Until then, the strict approach is safe.

# Example

Here's an example simple "short backtrace" implementation.
Note the use of `sub_frames` for the inner loop to restrict `symbols`!

This example is based off of code found in `miette` (Apache-2.0), which itself
copied the logic from `human-panic` (MIT/Apache-2.0).

FIXME: it would be nice if this example consulted `RUST_BACKTRACE=full`,
and maybe other vars used by rust's builtin panic handler..?

```
fn backtrace() -> String {
    use std::fmt::Write;
    if let Ok(var) = std::env::var("RUST_BACKTRACE") {
        if !var.is_empty() && var != "0" {
            const HEX_WIDTH: usize = std::mem::size_of::<usize>() + 2;
            // Padding for next lines after frame's address
            const NEXT_SYMBOL_PADDING: usize = HEX_WIDTH + 6;
            let mut backtrace = String::new();
            let trace = backtrace::Backtrace::new();
            let frames = backtrace_ext::short_frames_strict(&trace).enumerate();
            for (idx, (frame, subframes)) in frames {
                let ip = frame.ip();
                let _ = write!(backtrace, "\n{:4}: {:2$?}", idx, ip, HEX_WIDTH);
     
                let symbols = frame.symbols();
                if symbols.is_empty() {
                    let _ = write!(backtrace, " - <unresolved>");
                    continue;
                }
     
                for (idx, symbol) in symbols[subframes](#subframes).iter().enumerate() {
                    // Print symbols from this address,
                    // if there are several addresses
                    // we need to put it on next line
                    if idx != 0 {
                        let _ = write!(backtrace, "\n{:1$}", "", NEXT_SYMBOL_PADDING);
                    }
     
                    if let Some(name) = symbol.name() {
                        let _ = write!(backtrace, " - {}", name);
                    } else {
                        let _ = write!(backtrace, " - <unknown>");
                    }
     
                    // See if there is debug information with file name and line
                    if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                        let _ = write!(
                            backtrace,
                            "\n{:3$}at {}:{}",
                            "",
                            file.display(),
                            line,
                            NEXT_SYMBOL_PADDING
                        );
                    }
                }
            }
            return backtrace;
        }
    }
    "".into()
}
```

