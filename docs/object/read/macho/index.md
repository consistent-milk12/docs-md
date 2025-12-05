*[object](../../index.md) / [read](../index.md) / [macho](index.md)*

---

# Module `macho`

Support for reading Mach-O files.

Traits are used to abstract over the difference between 32-bit and 64-bit Mach-O
files. The primary trait for this is [`MachHeader`](file/index.md).

## High level API

[`MachOFile`](file/index.md) implements the [`Object`](crate::read::Object) trait for Mach-O files.
[`MachOFile`](file/index.md) is parameterised by [`MachHeader`](file/index.md) to allow reading both 32-bit and
64-bit Mach-O files. There are type aliases for these parameters ([`MachOFile32`](file/index.md) and
[`MachOFile64`](file/index.md)).

## Low level API

The [`MachHeader`](file/index.md) trait can be directly used to parse both `macho::MachHeader32`
and `macho::MachHeader64`. Additionally, [`FatHeader`](../../macho/index.md) and the [`FatArch`](fat/index.md) trait
can be used to iterate images in multi-architecture binaries, and [`DyldCache`](dyld_cache/index.md) can
be used to locate images in a dyld shared cache.

### Example for low level API
 ```no_run
use object::macho;
use object::read::macho::{MachHeader, Nlist};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let header = macho::MachHeader64::<object::Endianness>::parse(&*data, 0)?;
    let endian = header.endian()?;
    let mut commands = header.load_commands(endian, &*data, 0)?;
    while let Some(command) = commands.next()? {
        if let Some(symtab_command) = command.symtab()? {
            let symbols = symtab_command.symbols::<macho::MachHeader64<_>, _>(endian, &*data)?;
            for symbol in symbols.iter() {
                let name = symbol.name(endian, symbols.strings())?;
                println!("{}", String::from_utf8_lossy(name));
            }
        }
    }
  }
    Ok(())
}
```

