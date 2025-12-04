*[object](../../index.md) / [read](../index.md) / [elf](index.md)*

---

# Module `elf`

Support for reading ELF files.

Traits are used to abstract over the difference between 32-bit and 64-bit ELF.
The primary trait for this is [`FileHeader`](#fileheader).

## High level API

[`ElfFile`](#elffile) implements the [`Object`](crate::read::Object) trait for ELF files.
[`ElfFile`](#elffile) is parameterised by [`FileHeader`](#fileheader) to allow reading both 32-bit and
64-bit ELF. There are type aliases for these parameters ([`ElfFile32`](#elffile32) and
[`ElfFile64`](#elffile64)).

## Low level API

The [`FileHeader`](#fileheader) trait can be directly used to parse both [`elf::FileHeader32`](#fileheader32)
and [`elf::FileHeader64`](#fileheader64).

### Example for low level API
 ```no_run
use object::elf;
use object::read::elf::{FileHeader, Sym};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each symbol.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let elf = elf::FileHeader64::<object::Endianness>::parse(&*data)?;
    let endian = elf.endian()?;
    let sections = elf.sections(endian, &*data)?;
    let symbols = sections.symbols(endian, &*data, elf::SHT_SYMTAB)?;
    for symbol in symbols.iter() {
        let name = symbol.name(endian, symbols.strings())?;
        println!("{}", String::from_utf8_lossy(name));
    }
#   }
    Ok(())
}
```

