*[object](../../index.md) / [read](../index.md) / [xcoff](index.md)*

---

# Module `xcoff`

Support for reading AIX XCOFF files.

Traits are used to abstract over the difference between 32-bit and 64-bit XCOFF.
The primary trait for this is [`FileHeader`](#fileheader).

## High level API

[`XcoffFile`](#xcofffile) implements the [`Object`](crate::read::Object) trait for XCOFF files.
[`XcoffFile`](#xcofffile) is parameterised by [`FileHeader`](#fileheader) to allow reading both 32-bit and
64-bit XCOFF. There are type aliases for these parameters ([`XcoffFile32`](#xcofffile32) and
[`XcoffFile64`](#xcofffile64)).

## Low level API

The [`FileHeader`](#fileheader) trait can be directly used to parse both [`xcoff::FileHeader32`](#fileheader32)
and [`xcoff::FileHeader64`](#fileheader64).

### Example for low level API
 ```no_run
use object::xcoff;
use object::read::xcoff::{FileHeader, SectionHeader, Symbol};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = xcoff::FileHeader64::parse(&*data, &mut offset)?;
    let aux_header = header.aux_header(&*data, &mut offset)?;
    let sections = header.sections(&*data, &mut offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name()));
    }
    for (_index, symbol) in symbols.iter() {
        println!("{}", String::from_utf8_lossy(symbol.name(symbols.strings())?));
    }
#   }
    Ok(())
}
```

