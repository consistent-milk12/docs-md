*[object](../../index.md) / [read](../index.md) / [elf](index.md)*

---

# Module `elf`

Support for reading ELF files.

Traits are used to abstract over the difference between 32-bit and 64-bit ELF.
The primary trait for this is [`FileHeader`](../../index.md).

## High level API

[`ElfFile`](../../index.md) implements the [`Object`](crate::read::Object) trait for ELF files.
[`ElfFile`](../../index.md) is parameterised by [`FileHeader`](../../index.md) to allow reading both 32-bit and
64-bit ELF. There are type aliases for these parameters ([`ElfFile32`](../../index.md) and
[`ElfFile64`](../../index.md)).

## Low level API

The [`FileHeader`](../../index.md) trait can be directly used to parse both `elf::FileHeader32`
and `elf::FileHeader64`.

### Example for low level API
 ```no_run
use object::elf;
use object::read::elf::{FileHeader, Sym};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let elf = elf::FileHeader64::<object::Endianness>::parse(&*data)?;
    let endian = elf.endian()?;
    let sections = elf.sections(endian, &*data)?;
    let symbols = sections.symbols(endian, &*data, elf::SHT_SYMTAB)?;
    for symbol in symbols.iter() {
        let name = symbol.name(endian, symbols.strings())?;
        println!("{}", String::from_utf8_lossy(name));
    }
  }
    Ok(())
}
```

