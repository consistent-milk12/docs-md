*[object](../../index.md) / [read](../index.md) / [coff](index.md)*

---

# Module `coff`

Support for reading Windows COFF files.

Traits are used to abstract over the difference between COFF object files
and COFF bigobj files. The primary trait for this is [`CoffHeader`](../../index.md).

## High level API

[`CoffFile`](../../index.md) implements the [`Object`](crate::read::Object) trait for
COFF files. [`CoffFile`](../../index.md) is parameterised by [`CoffHeader`](../../index.md).
The default parameter allows reading regular COFF object files,
while the type alias [`CoffBigFile`](../../index.md) allows reading COFF bigobj files.

[`ImportFile`](../../index.md) allows reading COFF short imports that are used in import
libraries. Currently these are not integrated with the unified read API.

## Low level API

The [`CoffHeader`](../../index.md) trait can be directly used to parse both COFF
object files (which start with `pe::ImageFileHeader`) and COFF bigobj
files (which start with `pe::AnonObjectHeaderBigobj`).

### Example for low level API
 ```no_run
use object::pe;
use object::read::coff::{CoffHeader, ImageSymbol as _};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = pe::ImageFileHeader::parse(&*data, &mut offset)?;
    let sections = header.sections(&*data, offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name(symbols.strings())?));
    }
    for (_index, symbol) in symbols.iter() {
        println!("{}", String::from_utf8_lossy(symbol.name(symbols.strings())?));
    }
  }
    Ok(())
}
```

