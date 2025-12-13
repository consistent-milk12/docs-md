*[object](../index.md) / [macho](index.md)*

---

# Module `macho`

Mach-O definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is based heavily on header files from MacOSX11.1.sdk.

## Contents

- [Structs](#structs)
  - [`DyldCacheHeader`](#dyldcacheheader)
  - [`DyldCacheMappingInfo`](#dyldcachemappinginfo)
  - [`DyldCacheMappingAndSlideInfo`](#dyldcachemappingandslideinfo)
  - [`DyldCacheImageInfo`](#dyldcacheimageinfo)
  - [`DyldCacheSlideInfo2`](#dyldcacheslideinfo2)
  - [`DyldCacheSlideInfo3`](#dyldcacheslideinfo3)
  - [`DyldCacheSlidePointer3`](#dyldcacheslidepointer3)
  - [`DyldCacheSlideInfo5`](#dyldcacheslideinfo5)
  - [`DyldCacheSlidePointer5`](#dyldcacheslidepointer5)
  - [`DyldSubCacheEntryV1`](#dyldsubcacheentryv1)
  - [`DyldSubCacheEntryV2`](#dyldsubcacheentryv2)
  - [`FatHeader`](#fatheader)
  - [`FatArch32`](#fatarch32)
  - [`FatArch64`](#fatarch64)
  - [`MachHeader32`](#machheader32)
  - [`MachHeader64`](#machheader64)
  - [`LoadCommand`](#loadcommand)
  - [`LcStr`](#lcstr)
  - [`SegmentCommand32`](#segmentcommand32)
  - [`SegmentCommand64`](#segmentcommand64)
  - [`Section32`](#section32)
  - [`Section64`](#section64)
  - [`Fvmlib`](#fvmlib)
  - [`FvmlibCommand`](#fvmlibcommand)
  - [`Dylib`](#dylib)
  - [`DylibCommand`](#dylibcommand)
  - [`SubFrameworkCommand`](#subframeworkcommand)
  - [`SubClientCommand`](#subclientcommand)
  - [`SubUmbrellaCommand`](#subumbrellacommand)
  - [`SubLibraryCommand`](#sublibrarycommand)
  - [`PreboundDylibCommand`](#prebounddylibcommand)
  - [`DylinkerCommand`](#dylinkercommand)
  - [`ThreadCommand`](#threadcommand)
  - [`RoutinesCommand32`](#routinescommand32)
  - [`RoutinesCommand64`](#routinescommand64)
  - [`SymtabCommand`](#symtabcommand)
  - [`DysymtabCommand`](#dysymtabcommand)
  - [`DylibTableOfContents`](#dylibtableofcontents)
  - [`DylibModule32`](#dylibmodule32)
  - [`DylibModule64`](#dylibmodule64)
  - [`DylibReference`](#dylibreference)
  - [`TwolevelHintsCommand`](#twolevelhintscommand)
  - [`TwolevelHint`](#twolevelhint)
  - [`PrebindCksumCommand`](#prebindcksumcommand)
  - [`UuidCommand`](#uuidcommand)
  - [`RpathCommand`](#rpathcommand)
  - [`LinkeditDataCommand`](#linkeditdatacommand)
  - [`FilesetEntryCommand`](#filesetentrycommand)
  - [`EncryptionInfoCommand32`](#encryptioninfocommand32)
  - [`EncryptionInfoCommand64`](#encryptioninfocommand64)
  - [`VersionMinCommand`](#versionmincommand)
  - [`BuildVersionCommand`](#buildversioncommand)
  - [`BuildToolVersion`](#buildtoolversion)
  - [`DyldInfoCommand`](#dyldinfocommand)
  - [`LinkerOptionCommand`](#linkeroptioncommand)
  - [`SymsegCommand`](#symsegcommand)
  - [`IdentCommand`](#identcommand)
  - [`FvmfileCommand`](#fvmfilecommand)
  - [`EntryPointCommand`](#entrypointcommand)
  - [`SourceVersionCommand`](#sourceversioncommand)
  - [`DataInCodeEntry`](#dataincodeentry)
  - [`NoteCommand`](#notecommand)
  - [`Nlist32`](#nlist32)
  - [`Nlist64`](#nlist64)
  - [`Relocation`](#relocation)
  - [`RelocationInfo`](#relocationinfo)
  - [`ScatteredRelocationInfo`](#scatteredrelocationinfo)
- [Enums](#enums)
  - [`PtrauthKey`](#ptrauthkey)
- [Functions](#functions)
  - [`cpu_subtype_intel`](#cpu-subtype-intel)
  - [`cpu_subtype_intel_family`](#cpu-subtype-intel-family)
  - [`cpu_subtype_intel_model`](#cpu-subtype-intel-model)
- [Constants](#constants)
  - [`CPU_ARCH_MASK`](#cpu-arch-mask)
  - [`CPU_ARCH_ABI64`](#cpu-arch-abi64)
  - [`CPU_ARCH_ABI64_32`](#cpu-arch-abi64-32)
  - [`CPU_TYPE_ANY`](#cpu-type-any)
  - [`CPU_TYPE_VAX`](#cpu-type-vax)
  - [`CPU_TYPE_MC680X0`](#cpu-type-mc680x0)
  - [`CPU_TYPE_X86`](#cpu-type-x86)
  - [`CPU_TYPE_X86_64`](#cpu-type-x86-64)
  - [`CPU_TYPE_MIPS`](#cpu-type-mips)
  - [`CPU_TYPE_MC98000`](#cpu-type-mc98000)
  - [`CPU_TYPE_HPPA`](#cpu-type-hppa)
  - [`CPU_TYPE_ARM`](#cpu-type-arm)
  - [`CPU_TYPE_ARM64`](#cpu-type-arm64)
  - [`CPU_TYPE_ARM64_32`](#cpu-type-arm64-32)
  - [`CPU_TYPE_MC88000`](#cpu-type-mc88000)
  - [`CPU_TYPE_SPARC`](#cpu-type-sparc)
  - [`CPU_TYPE_I860`](#cpu-type-i860)
  - [`CPU_TYPE_ALPHA`](#cpu-type-alpha)
  - [`CPU_TYPE_POWERPC`](#cpu-type-powerpc)
  - [`CPU_TYPE_POWERPC64`](#cpu-type-powerpc64)
  - [`CPU_SUBTYPE_MASK`](#cpu-subtype-mask)
  - [`CPU_SUBTYPE_LIB64`](#cpu-subtype-lib64)
  - [`CPU_SUBTYPE_PTRAUTH_ABI`](#cpu-subtype-ptrauth-abi)
  - [`CPU_SUBTYPE_ANY`](#cpu-subtype-any)
  - [`CPU_SUBTYPE_MULTIPLE`](#cpu-subtype-multiple)
  - [`CPU_SUBTYPE_LITTLE_ENDIAN`](#cpu-subtype-little-endian)
  - [`CPU_SUBTYPE_BIG_ENDIAN`](#cpu-subtype-big-endian)
  - [`CPU_SUBTYPE_VAX_ALL`](#cpu-subtype-vax-all)
  - [`CPU_SUBTYPE_VAX780`](#cpu-subtype-vax780)
  - [`CPU_SUBTYPE_VAX785`](#cpu-subtype-vax785)
  - [`CPU_SUBTYPE_VAX750`](#cpu-subtype-vax750)
  - [`CPU_SUBTYPE_VAX730`](#cpu-subtype-vax730)
  - [`CPU_SUBTYPE_UVAXI`](#cpu-subtype-uvaxi)
  - [`CPU_SUBTYPE_UVAXII`](#cpu-subtype-uvaxii)
  - [`CPU_SUBTYPE_VAX8200`](#cpu-subtype-vax8200)
  - [`CPU_SUBTYPE_VAX8500`](#cpu-subtype-vax8500)
  - [`CPU_SUBTYPE_VAX8600`](#cpu-subtype-vax8600)
  - [`CPU_SUBTYPE_VAX8650`](#cpu-subtype-vax8650)
  - [`CPU_SUBTYPE_VAX8800`](#cpu-subtype-vax8800)
  - [`CPU_SUBTYPE_UVAXIII`](#cpu-subtype-uvaxiii)
  - [`CPU_SUBTYPE_MC680X0_ALL`](#cpu-subtype-mc680x0-all)
  - [`CPU_SUBTYPE_MC68030`](#cpu-subtype-mc68030)
  - [`CPU_SUBTYPE_MC68040`](#cpu-subtype-mc68040)
  - [`CPU_SUBTYPE_MC68030_ONLY`](#cpu-subtype-mc68030-only)
  - [`CPU_SUBTYPE_I386_ALL`](#cpu-subtype-i386-all)
  - [`CPU_SUBTYPE_386`](#cpu-subtype-386)
  - [`CPU_SUBTYPE_486`](#cpu-subtype-486)
  - [`CPU_SUBTYPE_486SX`](#cpu-subtype-486sx)
  - [`CPU_SUBTYPE_586`](#cpu-subtype-586)
  - [`CPU_SUBTYPE_PENT`](#cpu-subtype-pent)
  - [`CPU_SUBTYPE_PENTPRO`](#cpu-subtype-pentpro)
  - [`CPU_SUBTYPE_PENTII_M3`](#cpu-subtype-pentii-m3)
  - [`CPU_SUBTYPE_PENTII_M5`](#cpu-subtype-pentii-m5)
  - [`CPU_SUBTYPE_CELERON`](#cpu-subtype-celeron)
  - [`CPU_SUBTYPE_CELERON_MOBILE`](#cpu-subtype-celeron-mobile)
  - [`CPU_SUBTYPE_PENTIUM_3`](#cpu-subtype-pentium-3)
  - [`CPU_SUBTYPE_PENTIUM_3_M`](#cpu-subtype-pentium-3-m)
  - [`CPU_SUBTYPE_PENTIUM_3_XEON`](#cpu-subtype-pentium-3-xeon)
  - [`CPU_SUBTYPE_PENTIUM_M`](#cpu-subtype-pentium-m)
  - [`CPU_SUBTYPE_PENTIUM_4`](#cpu-subtype-pentium-4)
  - [`CPU_SUBTYPE_PENTIUM_4_M`](#cpu-subtype-pentium-4-m)
  - [`CPU_SUBTYPE_ITANIUM`](#cpu-subtype-itanium)
  - [`CPU_SUBTYPE_ITANIUM_2`](#cpu-subtype-itanium-2)
  - [`CPU_SUBTYPE_XEON`](#cpu-subtype-xeon)
  - [`CPU_SUBTYPE_XEON_MP`](#cpu-subtype-xeon-mp)
  - [`CPU_SUBTYPE_INTEL_FAMILY_MAX`](#cpu-subtype-intel-family-max)
  - [`CPU_SUBTYPE_INTEL_MODEL_ALL`](#cpu-subtype-intel-model-all)
  - [`CPU_SUBTYPE_X86_ALL`](#cpu-subtype-x86-all)
  - [`CPU_SUBTYPE_X86_64_ALL`](#cpu-subtype-x86-64-all)
  - [`CPU_SUBTYPE_X86_ARCH1`](#cpu-subtype-x86-arch1)
  - [`CPU_SUBTYPE_X86_64_H`](#cpu-subtype-x86-64-h)
  - [`CPU_SUBTYPE_MIPS_ALL`](#cpu-subtype-mips-all)
  - [`CPU_SUBTYPE_MIPS_R2300`](#cpu-subtype-mips-r2300)
  - [`CPU_SUBTYPE_MIPS_R2600`](#cpu-subtype-mips-r2600)
  - [`CPU_SUBTYPE_MIPS_R2800`](#cpu-subtype-mips-r2800)
  - [`CPU_SUBTYPE_MIPS_R2000A`](#cpu-subtype-mips-r2000a)
  - [`CPU_SUBTYPE_MIPS_R2000`](#cpu-subtype-mips-r2000)
  - [`CPU_SUBTYPE_MIPS_R3000A`](#cpu-subtype-mips-r3000a)
  - [`CPU_SUBTYPE_MIPS_R3000`](#cpu-subtype-mips-r3000)
  - [`CPU_SUBTYPE_MC98000_ALL`](#cpu-subtype-mc98000-all)
  - [`CPU_SUBTYPE_MC98601`](#cpu-subtype-mc98601)
  - [`CPU_SUBTYPE_HPPA_ALL`](#cpu-subtype-hppa-all)
  - [`CPU_SUBTYPE_HPPA_7100LC`](#cpu-subtype-hppa-7100lc)
  - [`CPU_SUBTYPE_MC88000_ALL`](#cpu-subtype-mc88000-all)
  - [`CPU_SUBTYPE_MC88100`](#cpu-subtype-mc88100)
  - [`CPU_SUBTYPE_MC88110`](#cpu-subtype-mc88110)
  - [`CPU_SUBTYPE_SPARC_ALL`](#cpu-subtype-sparc-all)
  - [`CPU_SUBTYPE_I860_ALL`](#cpu-subtype-i860-all)
  - [`CPU_SUBTYPE_I860_860`](#cpu-subtype-i860-860)
  - [`CPU_SUBTYPE_POWERPC_ALL`](#cpu-subtype-powerpc-all)
  - [`CPU_SUBTYPE_POWERPC_601`](#cpu-subtype-powerpc-601)
  - [`CPU_SUBTYPE_POWERPC_602`](#cpu-subtype-powerpc-602)
  - [`CPU_SUBTYPE_POWERPC_603`](#cpu-subtype-powerpc-603)
  - [`CPU_SUBTYPE_POWERPC_603E`](#cpu-subtype-powerpc-603e)
  - [`CPU_SUBTYPE_POWERPC_603EV`](#cpu-subtype-powerpc-603ev)
  - [`CPU_SUBTYPE_POWERPC_604`](#cpu-subtype-powerpc-604)
  - [`CPU_SUBTYPE_POWERPC_604E`](#cpu-subtype-powerpc-604e)
  - [`CPU_SUBTYPE_POWERPC_620`](#cpu-subtype-powerpc-620)
  - [`CPU_SUBTYPE_POWERPC_750`](#cpu-subtype-powerpc-750)
  - [`CPU_SUBTYPE_POWERPC_7400`](#cpu-subtype-powerpc-7400)
  - [`CPU_SUBTYPE_POWERPC_7450`](#cpu-subtype-powerpc-7450)
  - [`CPU_SUBTYPE_POWERPC_970`](#cpu-subtype-powerpc-970)
  - [`CPU_SUBTYPE_ARM_ALL`](#cpu-subtype-arm-all)
  - [`CPU_SUBTYPE_ARM_V4T`](#cpu-subtype-arm-v4t)
  - [`CPU_SUBTYPE_ARM_V6`](#cpu-subtype-arm-v6)
  - [`CPU_SUBTYPE_ARM_V5TEJ`](#cpu-subtype-arm-v5tej)
  - [`CPU_SUBTYPE_ARM_XSCALE`](#cpu-subtype-arm-xscale)
  - [`CPU_SUBTYPE_ARM_V7`](#cpu-subtype-arm-v7)
  - [`CPU_SUBTYPE_ARM_V7F`](#cpu-subtype-arm-v7f)
  - [`CPU_SUBTYPE_ARM_V7S`](#cpu-subtype-arm-v7s)
  - [`CPU_SUBTYPE_ARM_V7K`](#cpu-subtype-arm-v7k)
  - [`CPU_SUBTYPE_ARM_V8`](#cpu-subtype-arm-v8)
  - [`CPU_SUBTYPE_ARM_V6M`](#cpu-subtype-arm-v6m)
  - [`CPU_SUBTYPE_ARM_V7M`](#cpu-subtype-arm-v7m)
  - [`CPU_SUBTYPE_ARM_V7EM`](#cpu-subtype-arm-v7em)
  - [`CPU_SUBTYPE_ARM_V8M`](#cpu-subtype-arm-v8m)
  - [`CPU_SUBTYPE_ARM64_ALL`](#cpu-subtype-arm64-all)
  - [`CPU_SUBTYPE_ARM64_V8`](#cpu-subtype-arm64-v8)
  - [`CPU_SUBTYPE_ARM64E`](#cpu-subtype-arm64e)
  - [`CPU_SUBTYPE_ARM64_32_ALL`](#cpu-subtype-arm64-32-all)
  - [`CPU_SUBTYPE_ARM64_32_V8`](#cpu-subtype-arm64-32-v8)
  - [`VM_PROT_READ`](#vm-prot-read)
  - [`VM_PROT_WRITE`](#vm-prot-write)
  - [`VM_PROT_EXECUTE`](#vm-prot-execute)
  - [`DYLD_CACHE_MAPPING_AUTH_DATA`](#dyld-cache-mapping-auth-data)
  - [`DYLD_CACHE_MAPPING_DIRTY_DATA`](#dyld-cache-mapping-dirty-data)
  - [`DYLD_CACHE_MAPPING_CONST_DATA`](#dyld-cache-mapping-const-data)
  - [`DYLD_CACHE_MAPPING_TEXT_STUBS`](#dyld-cache-mapping-text-stubs)
  - [`DYLD_CACHE_DYNAMIC_CONFIG_DATA`](#dyld-cache-dynamic-config-data)
  - [`DYLD_CACHE_SLIDE_PAGE_ATTRS`](#dyld-cache-slide-page-attrs)
  - [`DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA`](#dyld-cache-slide-page-attr-extra)
  - [`DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE`](#dyld-cache-slide-page-attr-no-rebase)
  - [`DYLD_CACHE_SLIDE_PAGE_ATTR_END`](#dyld-cache-slide-page-attr-end)
  - [`DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE`](#dyld-cache-slide-v3-page-attr-no-rebase)
  - [`DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE`](#dyld-cache-slide-v5-page-attr-no-rebase)
  - [`FAT_MAGIC`](#fat-magic)
  - [`FAT_CIGAM`](#fat-cigam)
  - [`FAT_MAGIC_64`](#fat-magic-64)
  - [`FAT_CIGAM_64`](#fat-cigam-64)
  - [`MH_MAGIC`](#mh-magic)
  - [`MH_CIGAM`](#mh-cigam)
  - [`MH_MAGIC_64`](#mh-magic-64)
  - [`MH_CIGAM_64`](#mh-cigam-64)
  - [`MH_OBJECT`](#mh-object)
  - [`MH_EXECUTE`](#mh-execute)
  - [`MH_FVMLIB`](#mh-fvmlib)
  - [`MH_CORE`](#mh-core)
  - [`MH_PRELOAD`](#mh-preload)
  - [`MH_DYLIB`](#mh-dylib)
  - [`MH_DYLINKER`](#mh-dylinker)
  - [`MH_BUNDLE`](#mh-bundle)
  - [`MH_DYLIB_STUB`](#mh-dylib-stub)
  - [`MH_DSYM`](#mh-dsym)
  - [`MH_KEXT_BUNDLE`](#mh-kext-bundle)
  - [`MH_FILESET`](#mh-fileset)
  - [`MH_NOUNDEFS`](#mh-noundefs)
  - [`MH_INCRLINK`](#mh-incrlink)
  - [`MH_DYLDLINK`](#mh-dyldlink)
  - [`MH_BINDATLOAD`](#mh-bindatload)
  - [`MH_PREBOUND`](#mh-prebound)
  - [`MH_SPLIT_SEGS`](#mh-split-segs)
  - [`MH_LAZY_INIT`](#mh-lazy-init)
  - [`MH_TWOLEVEL`](#mh-twolevel)
  - [`MH_FORCE_FLAT`](#mh-force-flat)
  - [`MH_NOMULTIDEFS`](#mh-nomultidefs)
  - [`MH_NOFIXPREBINDING`](#mh-nofixprebinding)
  - [`MH_PREBINDABLE`](#mh-prebindable)
  - [`MH_ALLMODSBOUND`](#mh-allmodsbound)
  - [`MH_SUBSECTIONS_VIA_SYMBOLS`](#mh-subsections-via-symbols)
  - [`MH_CANONICAL`](#mh-canonical)
  - [`MH_WEAK_DEFINES`](#mh-weak-defines)
  - [`MH_BINDS_TO_WEAK`](#mh-binds-to-weak)
  - [`MH_ALLOW_STACK_EXECUTION`](#mh-allow-stack-execution)
  - [`MH_ROOT_SAFE`](#mh-root-safe)
  - [`MH_SETUID_SAFE`](#mh-setuid-safe)
  - [`MH_NO_REEXPORTED_DYLIBS`](#mh-no-reexported-dylibs)
  - [`MH_PIE`](#mh-pie)
  - [`MH_DEAD_STRIPPABLE_DYLIB`](#mh-dead-strippable-dylib)
  - [`MH_HAS_TLV_DESCRIPTORS`](#mh-has-tlv-descriptors)
  - [`MH_NO_HEAP_EXECUTION`](#mh-no-heap-execution)
  - [`MH_APP_EXTENSION_SAFE`](#mh-app-extension-safe)
  - [`MH_NLIST_OUTOFSYNC_WITH_DYLDINFO`](#mh-nlist-outofsync-with-dyldinfo)
  - [`MH_SIM_SUPPORT`](#mh-sim-support)
  - [`MH_DYLIB_IN_CACHE`](#mh-dylib-in-cache)
  - [`LC_REQ_DYLD`](#lc-req-dyld)
  - [`LC_SEGMENT`](#lc-segment)
  - [`LC_SYMTAB`](#lc-symtab)
  - [`LC_SYMSEG`](#lc-symseg)
  - [`LC_THREAD`](#lc-thread)
  - [`LC_UNIXTHREAD`](#lc-unixthread)
  - [`LC_LOADFVMLIB`](#lc-loadfvmlib)
  - [`LC_IDFVMLIB`](#lc-idfvmlib)
  - [`LC_IDENT`](#lc-ident)
  - [`LC_FVMFILE`](#lc-fvmfile)
  - [`LC_PREPAGE`](#lc-prepage)
  - [`LC_DYSYMTAB`](#lc-dysymtab)
  - [`LC_LOAD_DYLIB`](#lc-load-dylib)
  - [`LC_ID_DYLIB`](#lc-id-dylib)
  - [`LC_LOAD_DYLINKER`](#lc-load-dylinker)
  - [`LC_ID_DYLINKER`](#lc-id-dylinker)
  - [`LC_PREBOUND_DYLIB`](#lc-prebound-dylib)
  - [`LC_ROUTINES`](#lc-routines)
  - [`LC_SUB_FRAMEWORK`](#lc-sub-framework)
  - [`LC_SUB_UMBRELLA`](#lc-sub-umbrella)
  - [`LC_SUB_CLIENT`](#lc-sub-client)
  - [`LC_SUB_LIBRARY`](#lc-sub-library)
  - [`LC_TWOLEVEL_HINTS`](#lc-twolevel-hints)
  - [`LC_PREBIND_CKSUM`](#lc-prebind-cksum)
  - [`LC_LOAD_WEAK_DYLIB`](#lc-load-weak-dylib)
  - [`LC_SEGMENT_64`](#lc-segment-64)
  - [`LC_ROUTINES_64`](#lc-routines-64)
  - [`LC_UUID`](#lc-uuid)
  - [`LC_RPATH`](#lc-rpath)
  - [`LC_CODE_SIGNATURE`](#lc-code-signature)
  - [`LC_SEGMENT_SPLIT_INFO`](#lc-segment-split-info)
  - [`LC_REEXPORT_DYLIB`](#lc-reexport-dylib)
  - [`LC_LAZY_LOAD_DYLIB`](#lc-lazy-load-dylib)
  - [`LC_ENCRYPTION_INFO`](#lc-encryption-info)
  - [`LC_DYLD_INFO`](#lc-dyld-info)
  - [`LC_DYLD_INFO_ONLY`](#lc-dyld-info-only)
  - [`LC_LOAD_UPWARD_DYLIB`](#lc-load-upward-dylib)
  - [`LC_VERSION_MIN_MACOSX`](#lc-version-min-macosx)
  - [`LC_VERSION_MIN_IPHONEOS`](#lc-version-min-iphoneos)
  - [`LC_FUNCTION_STARTS`](#lc-function-starts)
  - [`LC_DYLD_ENVIRONMENT`](#lc-dyld-environment)
  - [`LC_MAIN`](#lc-main)
  - [`LC_DATA_IN_CODE`](#lc-data-in-code)
  - [`LC_SOURCE_VERSION`](#lc-source-version)
  - [`LC_DYLIB_CODE_SIGN_DRS`](#lc-dylib-code-sign-drs)
  - [`LC_ENCRYPTION_INFO_64`](#lc-encryption-info-64)
  - [`LC_LINKER_OPTION`](#lc-linker-option)
  - [`LC_LINKER_OPTIMIZATION_HINT`](#lc-linker-optimization-hint)
  - [`LC_VERSION_MIN_TVOS`](#lc-version-min-tvos)
  - [`LC_VERSION_MIN_WATCHOS`](#lc-version-min-watchos)
  - [`LC_NOTE`](#lc-note)
  - [`LC_BUILD_VERSION`](#lc-build-version)
  - [`LC_DYLD_EXPORTS_TRIE`](#lc-dyld-exports-trie)
  - [`LC_DYLD_CHAINED_FIXUPS`](#lc-dyld-chained-fixups)
  - [`LC_FILESET_ENTRY`](#lc-fileset-entry)
  - [`SG_HIGHVM`](#sg-highvm)
  - [`SG_FVMLIB`](#sg-fvmlib)
  - [`SG_NORELOC`](#sg-noreloc)
  - [`SG_PROTECTED_VERSION_1`](#sg-protected-version-1)
  - [`SG_READ_ONLY`](#sg-read-only)
  - [`SECTION_TYPE`](#section-type)
  - [`SECTION_ATTRIBUTES`](#section-attributes)
  - [`S_REGULAR`](#s-regular)
  - [`S_ZEROFILL`](#s-zerofill)
  - [`S_CSTRING_LITERALS`](#s-cstring-literals)
  - [`S_4BYTE_LITERALS`](#s-4byte-literals)
  - [`S_8BYTE_LITERALS`](#s-8byte-literals)
  - [`S_LITERAL_POINTERS`](#s-literal-pointers)
  - [`S_NON_LAZY_SYMBOL_POINTERS`](#s-non-lazy-symbol-pointers)
  - [`S_LAZY_SYMBOL_POINTERS`](#s-lazy-symbol-pointers)
  - [`S_SYMBOL_STUBS`](#s-symbol-stubs)
  - [`S_MOD_INIT_FUNC_POINTERS`](#s-mod-init-func-pointers)
  - [`S_MOD_TERM_FUNC_POINTERS`](#s-mod-term-func-pointers)
  - [`S_COALESCED`](#s-coalesced)
  - [`S_GB_ZEROFILL`](#s-gb-zerofill)
  - [`S_INTERPOSING`](#s-interposing)
  - [`S_16BYTE_LITERALS`](#s-16byte-literals)
  - [`S_DTRACE_DOF`](#s-dtrace-dof)
  - [`S_LAZY_DYLIB_SYMBOL_POINTERS`](#s-lazy-dylib-symbol-pointers)
  - [`S_THREAD_LOCAL_REGULAR`](#s-thread-local-regular)
  - [`S_THREAD_LOCAL_ZEROFILL`](#s-thread-local-zerofill)
  - [`S_THREAD_LOCAL_VARIABLES`](#s-thread-local-variables)
  - [`S_THREAD_LOCAL_VARIABLE_POINTERS`](#s-thread-local-variable-pointers)
  - [`S_THREAD_LOCAL_INIT_FUNCTION_POINTERS`](#s-thread-local-init-function-pointers)
  - [`S_INIT_FUNC_OFFSETS`](#s-init-func-offsets)
  - [`SECTION_ATTRIBUTES_USR`](#section-attributes-usr)
  - [`S_ATTR_PURE_INSTRUCTIONS`](#s-attr-pure-instructions)
  - [`S_ATTR_NO_TOC`](#s-attr-no-toc)
  - [`S_ATTR_STRIP_STATIC_SYMS`](#s-attr-strip-static-syms)
  - [`S_ATTR_NO_DEAD_STRIP`](#s-attr-no-dead-strip)
  - [`S_ATTR_LIVE_SUPPORT`](#s-attr-live-support)
  - [`S_ATTR_SELF_MODIFYING_CODE`](#s-attr-self-modifying-code)
  - [`S_ATTR_DEBUG`](#s-attr-debug)
  - [`SECTION_ATTRIBUTES_SYS`](#section-attributes-sys)
  - [`S_ATTR_SOME_INSTRUCTIONS`](#s-attr-some-instructions)
  - [`S_ATTR_EXT_RELOC`](#s-attr-ext-reloc)
  - [`S_ATTR_LOC_RELOC`](#s-attr-loc-reloc)
  - [`SEG_PAGEZERO`](#seg-pagezero)
  - [`SEG_TEXT`](#seg-text)
  - [`SECT_TEXT`](#sect-text)
  - [`SECT_FVMLIB_INIT0`](#sect-fvmlib-init0)
  - [`SECT_FVMLIB_INIT1`](#sect-fvmlib-init1)
  - [`SEG_DATA`](#seg-data)
  - [`SECT_DATA`](#sect-data)
  - [`SECT_BSS`](#sect-bss)
  - [`SECT_COMMON`](#sect-common)
  - [`SEG_OBJC`](#seg-objc)
  - [`SECT_OBJC_SYMBOLS`](#sect-objc-symbols)
  - [`SECT_OBJC_MODULES`](#sect-objc-modules)
  - [`SECT_OBJC_STRINGS`](#sect-objc-strings)
  - [`SECT_OBJC_REFS`](#sect-objc-refs)
  - [`SEG_ICON`](#seg-icon)
  - [`SECT_ICON_HEADER`](#sect-icon-header)
  - [`SECT_ICON_TIFF`](#sect-icon-tiff)
  - [`SEG_LINKEDIT`](#seg-linkedit)
  - [`SEG_LINKINFO`](#seg-linkinfo)
  - [`SEG_UNIXSTACK`](#seg-unixstack)
  - [`SEG_IMPORT`](#seg-import)
  - [`INDIRECT_SYMBOL_LOCAL`](#indirect-symbol-local)
  - [`INDIRECT_SYMBOL_ABS`](#indirect-symbol-abs)
  - [`PLATFORM_MACOS`](#platform-macos)
  - [`PLATFORM_IOS`](#platform-ios)
  - [`PLATFORM_TVOS`](#platform-tvos)
  - [`PLATFORM_WATCHOS`](#platform-watchos)
  - [`PLATFORM_BRIDGEOS`](#platform-bridgeos)
  - [`PLATFORM_MACCATALYST`](#platform-maccatalyst)
  - [`PLATFORM_IOSSIMULATOR`](#platform-iossimulator)
  - [`PLATFORM_TVOSSIMULATOR`](#platform-tvossimulator)
  - [`PLATFORM_WATCHOSSIMULATOR`](#platform-watchossimulator)
  - [`PLATFORM_DRIVERKIT`](#platform-driverkit)
  - [`PLATFORM_XROS`](#platform-xros)
  - [`PLATFORM_XROSSIMULATOR`](#platform-xrossimulator)
  - [`TOOL_CLANG`](#tool-clang)
  - [`TOOL_SWIFT`](#tool-swift)
  - [`TOOL_LD`](#tool-ld)
  - [`REBASE_TYPE_POINTER`](#rebase-type-pointer)
  - [`REBASE_TYPE_TEXT_ABSOLUTE32`](#rebase-type-text-absolute32)
  - [`REBASE_TYPE_TEXT_PCREL32`](#rebase-type-text-pcrel32)
  - [`REBASE_OPCODE_MASK`](#rebase-opcode-mask)
  - [`REBASE_IMMEDIATE_MASK`](#rebase-immediate-mask)
  - [`REBASE_OPCODE_DONE`](#rebase-opcode-done)
  - [`REBASE_OPCODE_SET_TYPE_IMM`](#rebase-opcode-set-type-imm)
  - [`REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`](#rebase-opcode-set-segment-and-offset-uleb)
  - [`REBASE_OPCODE_ADD_ADDR_ULEB`](#rebase-opcode-add-addr-uleb)
  - [`REBASE_OPCODE_ADD_ADDR_IMM_SCALED`](#rebase-opcode-add-addr-imm-scaled)
  - [`REBASE_OPCODE_DO_REBASE_IMM_TIMES`](#rebase-opcode-do-rebase-imm-times)
  - [`REBASE_OPCODE_DO_REBASE_ULEB_TIMES`](#rebase-opcode-do-rebase-uleb-times)
  - [`REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB`](#rebase-opcode-do-rebase-add-addr-uleb)
  - [`REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB`](#rebase-opcode-do-rebase-uleb-times-skipping-uleb)
  - [`BIND_TYPE_POINTER`](#bind-type-pointer)
  - [`BIND_TYPE_TEXT_ABSOLUTE32`](#bind-type-text-absolute32)
  - [`BIND_TYPE_TEXT_PCREL32`](#bind-type-text-pcrel32)
  - [`BIND_SPECIAL_DYLIB_SELF`](#bind-special-dylib-self)
  - [`BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE`](#bind-special-dylib-main-executable)
  - [`BIND_SPECIAL_DYLIB_FLAT_LOOKUP`](#bind-special-dylib-flat-lookup)
  - [`BIND_SPECIAL_DYLIB_WEAK_LOOKUP`](#bind-special-dylib-weak-lookup)
  - [`BIND_SYMBOL_FLAGS_WEAK_IMPORT`](#bind-symbol-flags-weak-import)
  - [`BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION`](#bind-symbol-flags-non-weak-definition)
  - [`BIND_OPCODE_MASK`](#bind-opcode-mask)
  - [`BIND_IMMEDIATE_MASK`](#bind-immediate-mask)
  - [`BIND_OPCODE_DONE`](#bind-opcode-done)
  - [`BIND_OPCODE_SET_DYLIB_ORDINAL_IMM`](#bind-opcode-set-dylib-ordinal-imm)
  - [`BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB`](#bind-opcode-set-dylib-ordinal-uleb)
  - [`BIND_OPCODE_SET_DYLIB_SPECIAL_IMM`](#bind-opcode-set-dylib-special-imm)
  - [`BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM`](#bind-opcode-set-symbol-trailing-flags-imm)
  - [`BIND_OPCODE_SET_TYPE_IMM`](#bind-opcode-set-type-imm)
  - [`BIND_OPCODE_SET_ADDEND_SLEB`](#bind-opcode-set-addend-sleb)
  - [`BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`](#bind-opcode-set-segment-and-offset-uleb)
  - [`BIND_OPCODE_ADD_ADDR_ULEB`](#bind-opcode-add-addr-uleb)
  - [`BIND_OPCODE_DO_BIND`](#bind-opcode-do-bind)
  - [`BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB`](#bind-opcode-do-bind-add-addr-uleb)
  - [`BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED`](#bind-opcode-do-bind-add-addr-imm-scaled)
  - [`BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB`](#bind-opcode-do-bind-uleb-times-skipping-uleb)
  - [`BIND_OPCODE_THREADED`](#bind-opcode-threaded)
  - [`BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB`](#bind-subopcode-threaded-set-bind-ordinal-table-size-uleb)
  - [`BIND_SUBOPCODE_THREADED_APPLY`](#bind-subopcode-threaded-apply)
  - [`EXPORT_SYMBOL_FLAGS_KIND_MASK`](#export-symbol-flags-kind-mask)
  - [`EXPORT_SYMBOL_FLAGS_KIND_REGULAR`](#export-symbol-flags-kind-regular)
  - [`EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL`](#export-symbol-flags-kind-thread-local)
  - [`EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE`](#export-symbol-flags-kind-absolute)
  - [`EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION`](#export-symbol-flags-weak-definition)
  - [`EXPORT_SYMBOL_FLAGS_REEXPORT`](#export-symbol-flags-reexport)
  - [`EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER`](#export-symbol-flags-stub-and-resolver)
  - [`DICE_KIND_DATA`](#dice-kind-data)
  - [`DICE_KIND_JUMP_TABLE8`](#dice-kind-jump-table8)
  - [`DICE_KIND_JUMP_TABLE16`](#dice-kind-jump-table16)
  - [`DICE_KIND_JUMP_TABLE32`](#dice-kind-jump-table32)
  - [`DICE_KIND_ABS_JUMP_TABLE32`](#dice-kind-abs-jump-table32)
  - [`N_STAB`](#n-stab)
  - [`N_PEXT`](#n-pext)
  - [`N_TYPE`](#n-type)
  - [`N_EXT`](#n-ext)
  - [`N_UNDF`](#n-undf)
  - [`N_ABS`](#n-abs)
  - [`N_SECT`](#n-sect)
  - [`N_PBUD`](#n-pbud)
  - [`N_INDR`](#n-indr)
  - [`NO_SECT`](#no-sect)
  - [`MAX_SECT`](#max-sect)
  - [`REFERENCE_TYPE`](#reference-type)
  - [`REFERENCE_FLAG_UNDEFINED_NON_LAZY`](#reference-flag-undefined-non-lazy)
  - [`REFERENCE_FLAG_UNDEFINED_LAZY`](#reference-flag-undefined-lazy)
  - [`REFERENCE_FLAG_DEFINED`](#reference-flag-defined)
  - [`REFERENCE_FLAG_PRIVATE_DEFINED`](#reference-flag-private-defined)
  - [`REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY`](#reference-flag-private-undefined-non-lazy)
  - [`REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY`](#reference-flag-private-undefined-lazy)
  - [`REFERENCED_DYNAMICALLY`](#referenced-dynamically)
  - [`SELF_LIBRARY_ORDINAL`](#self-library-ordinal)
  - [`MAX_LIBRARY_ORDINAL`](#max-library-ordinal)
  - [`DYNAMIC_LOOKUP_ORDINAL`](#dynamic-lookup-ordinal)
  - [`EXECUTABLE_ORDINAL`](#executable-ordinal)
  - [`N_NO_DEAD_STRIP`](#n-no-dead-strip)
  - [`N_DESC_DISCARDED`](#n-desc-discarded)
  - [`N_WEAK_REF`](#n-weak-ref)
  - [`N_WEAK_DEF`](#n-weak-def)
  - [`N_REF_TO_WEAK`](#n-ref-to-weak)
  - [`N_ARM_THUMB_DEF`](#n-arm-thumb-def)
  - [`N_SYMBOL_RESOLVER`](#n-symbol-resolver)
  - [`N_ALT_ENTRY`](#n-alt-entry)
  - [`N_GSYM`](#n-gsym)
  - [`N_FNAME`](#n-fname)
  - [`N_FUN`](#n-fun)
  - [`N_STSYM`](#n-stsym)
  - [`N_LCSYM`](#n-lcsym)
  - [`N_BNSYM`](#n-bnsym)
  - [`N_AST`](#n-ast)
  - [`N_OPT`](#n-opt)
  - [`N_RSYM`](#n-rsym)
  - [`N_SLINE`](#n-sline)
  - [`N_ENSYM`](#n-ensym)
  - [`N_SSYM`](#n-ssym)
  - [`N_SO`](#n-so)
  - [`N_OSO`](#n-oso)
  - [`N_LSYM`](#n-lsym)
  - [`N_BINCL`](#n-bincl)
  - [`N_SOL`](#n-sol)
  - [`N_PARAMS`](#n-params)
  - [`N_VERSION`](#n-version)
  - [`N_OLEVEL`](#n-olevel)
  - [`N_PSYM`](#n-psym)
  - [`N_EINCL`](#n-eincl)
  - [`N_ENTRY`](#n-entry)
  - [`N_LBRAC`](#n-lbrac)
  - [`N_EXCL`](#n-excl)
  - [`N_RBRAC`](#n-rbrac)
  - [`N_BCOMM`](#n-bcomm)
  - [`N_ECOMM`](#n-ecomm)
  - [`N_ECOML`](#n-ecoml)
  - [`N_LENG`](#n-leng)
  - [`N_PC`](#n-pc)
  - [`R_ABS`](#r-abs)
  - [`R_SCATTERED`](#r-scattered)
  - [`GENERIC_RELOC_VANILLA`](#generic-reloc-vanilla)
  - [`GENERIC_RELOC_PAIR`](#generic-reloc-pair)
  - [`GENERIC_RELOC_SECTDIFF`](#generic-reloc-sectdiff)
  - [`GENERIC_RELOC_PB_LA_PTR`](#generic-reloc-pb-la-ptr)
  - [`GENERIC_RELOC_LOCAL_SECTDIFF`](#generic-reloc-local-sectdiff)
  - [`GENERIC_RELOC_TLV`](#generic-reloc-tlv)
  - [`ARM_RELOC_VANILLA`](#arm-reloc-vanilla)
  - [`ARM_RELOC_PAIR`](#arm-reloc-pair)
  - [`ARM_RELOC_SECTDIFF`](#arm-reloc-sectdiff)
  - [`ARM_RELOC_LOCAL_SECTDIFF`](#arm-reloc-local-sectdiff)
  - [`ARM_RELOC_PB_LA_PTR`](#arm-reloc-pb-la-ptr)
  - [`ARM_RELOC_BR24`](#arm-reloc-br24)
  - [`ARM_THUMB_RELOC_BR22`](#arm-thumb-reloc-br22)
  - [`ARM_THUMB_32BIT_BRANCH`](#arm-thumb-32bit-branch)
  - [`ARM_RELOC_HALF`](#arm-reloc-half)
  - [`ARM_RELOC_HALF_SECTDIFF`](#arm-reloc-half-sectdiff)
  - [`ARM64_RELOC_UNSIGNED`](#arm64-reloc-unsigned)
  - [`ARM64_RELOC_SUBTRACTOR`](#arm64-reloc-subtractor)
  - [`ARM64_RELOC_BRANCH26`](#arm64-reloc-branch26)
  - [`ARM64_RELOC_PAGE21`](#arm64-reloc-page21)
  - [`ARM64_RELOC_PAGEOFF12`](#arm64-reloc-pageoff12)
  - [`ARM64_RELOC_GOT_LOAD_PAGE21`](#arm64-reloc-got-load-page21)
  - [`ARM64_RELOC_GOT_LOAD_PAGEOFF12`](#arm64-reloc-got-load-pageoff12)
  - [`ARM64_RELOC_POINTER_TO_GOT`](#arm64-reloc-pointer-to-got)
  - [`ARM64_RELOC_TLVP_LOAD_PAGE21`](#arm64-reloc-tlvp-load-page21)
  - [`ARM64_RELOC_TLVP_LOAD_PAGEOFF12`](#arm64-reloc-tlvp-load-pageoff12)
  - [`ARM64_RELOC_ADDEND`](#arm64-reloc-addend)
  - [`ARM64_RELOC_AUTHENTICATED_POINTER`](#arm64-reloc-authenticated-pointer)
  - [`PPC_RELOC_VANILLA`](#ppc-reloc-vanilla)
  - [`PPC_RELOC_PAIR`](#ppc-reloc-pair)
  - [`PPC_RELOC_BR14`](#ppc-reloc-br14)
  - [`PPC_RELOC_BR24`](#ppc-reloc-br24)
  - [`PPC_RELOC_HI16`](#ppc-reloc-hi16)
  - [`PPC_RELOC_LO16`](#ppc-reloc-lo16)
  - [`PPC_RELOC_HA16`](#ppc-reloc-ha16)
  - [`PPC_RELOC_LO14`](#ppc-reloc-lo14)
  - [`PPC_RELOC_SECTDIFF`](#ppc-reloc-sectdiff)
  - [`PPC_RELOC_PB_LA_PTR`](#ppc-reloc-pb-la-ptr)
  - [`PPC_RELOC_HI16_SECTDIFF`](#ppc-reloc-hi16-sectdiff)
  - [`PPC_RELOC_LO16_SECTDIFF`](#ppc-reloc-lo16-sectdiff)
  - [`PPC_RELOC_HA16_SECTDIFF`](#ppc-reloc-ha16-sectdiff)
  - [`PPC_RELOC_JBSR`](#ppc-reloc-jbsr)
  - [`PPC_RELOC_LO14_SECTDIFF`](#ppc-reloc-lo14-sectdiff)
  - [`PPC_RELOC_LOCAL_SECTDIFF`](#ppc-reloc-local-sectdiff)
  - [`X86_64_RELOC_UNSIGNED`](#x86-64-reloc-unsigned)
  - [`X86_64_RELOC_SIGNED`](#x86-64-reloc-signed)
  - [`X86_64_RELOC_BRANCH`](#x86-64-reloc-branch)
  - [`X86_64_RELOC_GOT_LOAD`](#x86-64-reloc-got-load)
  - [`X86_64_RELOC_GOT`](#x86-64-reloc-got)
  - [`X86_64_RELOC_SUBTRACTOR`](#x86-64-reloc-subtractor)
  - [`X86_64_RELOC_SIGNED_1`](#x86-64-reloc-signed-1)
  - [`X86_64_RELOC_SIGNED_2`](#x86-64-reloc-signed-2)
  - [`X86_64_RELOC_SIGNED_4`](#x86-64-reloc-signed-4)
  - [`X86_64_RELOC_TLV`](#x86-64-reloc-tlv)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DyldCacheHeader`](#dyldcacheheader) | struct | The dyld cache header. |
| [`DyldCacheMappingInfo`](#dyldcachemappinginfo) | struct | Corresponds to struct dyld_cache_mapping_info from dyld_cache_format.h. |
| [`DyldCacheMappingAndSlideInfo`](#dyldcachemappingandslideinfo) | struct | Corresponds to struct dyld_cache_mapping_and_slide_info from dyld_cache_format.h. |
| [`DyldCacheImageInfo`](#dyldcacheimageinfo) | struct | Corresponds to struct dyld_cache_image_info from dyld_cache_format.h. |
| [`DyldCacheSlideInfo2`](#dyldcacheslideinfo2) | struct | Corresponds to struct dyld_cache_slide_info2 from dyld_cache_format.h. |
| [`DyldCacheSlideInfo3`](#dyldcacheslideinfo3) | struct | Corresponds to struct dyld_cache_slide_info3 from dyld_cache_format.h. |
| [`DyldCacheSlidePointer3`](#dyldcacheslidepointer3) | struct | Corresponds to union dyld_cache_slide_pointer3 from dyld_cache_format.h. |
| [`DyldCacheSlideInfo5`](#dyldcacheslideinfo5) | struct | Corresponds to struct dyld_cache_slide_info5 from dyld_cache_format.h. |
| [`DyldCacheSlidePointer5`](#dyldcacheslidepointer5) | struct | Corresponds to struct dyld_cache_slide_pointer5 from dyld_cache_format.h. |
| [`DyldSubCacheEntryV1`](#dyldsubcacheentryv1) | struct | Added in dyld-940, which shipped with macOS 12 / iOS 15. |
| [`DyldSubCacheEntryV2`](#dyldsubcacheentryv2) | struct | Added in dyld-1042.1, which shipped with macOS 13 / iOS 16. |
| [`FatHeader`](#fatheader) | struct |  |
| [`FatArch32`](#fatarch32) | struct |  |
| [`FatArch64`](#fatarch64) | struct |  |
| [`MachHeader32`](#machheader32) | struct | The 32-bit mach header. |
| [`MachHeader64`](#machheader64) | struct | The 64-bit mach header. |
| [`LoadCommand`](#loadcommand) | struct | Common fields at the start of every load command. |
| [`LcStr`](#lcstr) | struct | A variable length string in a load command. |
| [`SegmentCommand32`](#segmentcommand32) | struct | 32-bit segment load command. |
| [`SegmentCommand64`](#segmentcommand64) | struct | 64-bit segment load command. |
| [`Section32`](#section32) | struct | 32-bit section. |
| [`Section64`](#section64) | struct | 64-bit section. |
| [`Fvmlib`](#fvmlib) | struct |  |
| [`FvmlibCommand`](#fvmlibcommand) | struct |  |
| [`Dylib`](#dylib) | struct |  |
| [`DylibCommand`](#dylibcommand) | struct |  |
| [`SubFrameworkCommand`](#subframeworkcommand) | struct |  |
| [`SubClientCommand`](#subclientcommand) | struct |  |
| [`SubUmbrellaCommand`](#subumbrellacommand) | struct |  |
| [`SubLibraryCommand`](#sublibrarycommand) | struct |  |
| [`PreboundDylibCommand`](#prebounddylibcommand) | struct |  |
| [`DylinkerCommand`](#dylinkercommand) | struct |  |
| [`ThreadCommand`](#threadcommand) | struct |  |
| [`RoutinesCommand32`](#routinescommand32) | struct |  |
| [`RoutinesCommand64`](#routinescommand64) | struct |  |
| [`SymtabCommand`](#symtabcommand) | struct |  |
| [`DysymtabCommand`](#dysymtabcommand) | struct |  |
| [`DylibTableOfContents`](#dylibtableofcontents) | struct |  |
| [`DylibModule32`](#dylibmodule32) | struct |  |
| [`DylibModule64`](#dylibmodule64) | struct |  |
| [`DylibReference`](#dylibreference) | struct |  |
| [`TwolevelHintsCommand`](#twolevelhintscommand) | struct |  |
| [`TwolevelHint`](#twolevelhint) | struct |  |
| [`PrebindCksumCommand`](#prebindcksumcommand) | struct |  |
| [`UuidCommand`](#uuidcommand) | struct |  |
| [`RpathCommand`](#rpathcommand) | struct |  |
| [`LinkeditDataCommand`](#linkeditdatacommand) | struct |  |
| [`FilesetEntryCommand`](#filesetentrycommand) | struct |  |
| [`EncryptionInfoCommand32`](#encryptioninfocommand32) | struct |  |
| [`EncryptionInfoCommand64`](#encryptioninfocommand64) | struct |  |
| [`VersionMinCommand`](#versionmincommand) | struct |  |
| [`BuildVersionCommand`](#buildversioncommand) | struct |  |
| [`BuildToolVersion`](#buildtoolversion) | struct |  |
| [`DyldInfoCommand`](#dyldinfocommand) | struct |  |
| [`LinkerOptionCommand`](#linkeroptioncommand) | struct |  |
| [`SymsegCommand`](#symsegcommand) | struct |  |
| [`IdentCommand`](#identcommand) | struct |  |
| [`FvmfileCommand`](#fvmfilecommand) | struct |  |
| [`EntryPointCommand`](#entrypointcommand) | struct |  |
| [`SourceVersionCommand`](#sourceversioncommand) | struct |  |
| [`DataInCodeEntry`](#dataincodeentry) | struct |  |
| [`NoteCommand`](#notecommand) | struct |  |
| [`Nlist32`](#nlist32) | struct |  |
| [`Nlist64`](#nlist64) | struct |  |
| [`Relocation`](#relocation) | struct | A relocation entry. |
| [`RelocationInfo`](#relocationinfo) | struct |  |
| [`ScatteredRelocationInfo`](#scatteredrelocationinfo) | struct |  |
| [`PtrauthKey`](#ptrauthkey) | enum | The key used to sign a pointer for authentication. |
| [`cpu_subtype_intel`](#cpu-subtype-intel) | fn |  |
| [`cpu_subtype_intel_family`](#cpu-subtype-intel-family) | fn |  |
| [`cpu_subtype_intel_model`](#cpu-subtype-intel-model) | fn |  |
| [`CPU_ARCH_MASK`](#cpu-arch-mask) | const | mask for architecture bits |
| [`CPU_ARCH_ABI64`](#cpu-arch-abi64) | const | 64 bit ABI |
| [`CPU_ARCH_ABI64_32`](#cpu-arch-abi64-32) | const | ABI for 64-bit hardware with 32-bit types; LP32 |
| [`CPU_TYPE_ANY`](#cpu-type-any) | const |  |
| [`CPU_TYPE_VAX`](#cpu-type-vax) | const |  |
| [`CPU_TYPE_MC680X0`](#cpu-type-mc680x0) | const |  |
| [`CPU_TYPE_X86`](#cpu-type-x86) | const |  |
| [`CPU_TYPE_X86_64`](#cpu-type-x86-64) | const |  |
| [`CPU_TYPE_MIPS`](#cpu-type-mips) | const |  |
| [`CPU_TYPE_MC98000`](#cpu-type-mc98000) | const |  |
| [`CPU_TYPE_HPPA`](#cpu-type-hppa) | const |  |
| [`CPU_TYPE_ARM`](#cpu-type-arm) | const |  |
| [`CPU_TYPE_ARM64`](#cpu-type-arm64) | const |  |
| [`CPU_TYPE_ARM64_32`](#cpu-type-arm64-32) | const |  |
| [`CPU_TYPE_MC88000`](#cpu-type-mc88000) | const |  |
| [`CPU_TYPE_SPARC`](#cpu-type-sparc) | const |  |
| [`CPU_TYPE_I860`](#cpu-type-i860) | const |  |
| [`CPU_TYPE_ALPHA`](#cpu-type-alpha) | const |  |
| [`CPU_TYPE_POWERPC`](#cpu-type-powerpc) | const |  |
| [`CPU_TYPE_POWERPC64`](#cpu-type-powerpc64) | const |  |
| [`CPU_SUBTYPE_MASK`](#cpu-subtype-mask) | const | mask for feature flags |
| [`CPU_SUBTYPE_LIB64`](#cpu-subtype-lib64) | const | 64 bit libraries |
| [`CPU_SUBTYPE_PTRAUTH_ABI`](#cpu-subtype-ptrauth-abi) | const | pointer authentication with versioned ABI |
| [`CPU_SUBTYPE_ANY`](#cpu-subtype-any) | const | When selecting a slice, ANY will pick the slice with the best grading for the selected cpu_type_t, unlike the "ALL" subtypes, which are the slices that can run on any hardware for that cpu type. |
| [`CPU_SUBTYPE_MULTIPLE`](#cpu-subtype-multiple) | const |  |
| [`CPU_SUBTYPE_LITTLE_ENDIAN`](#cpu-subtype-little-endian) | const |  |
| [`CPU_SUBTYPE_BIG_ENDIAN`](#cpu-subtype-big-endian) | const |  |
| [`CPU_SUBTYPE_VAX_ALL`](#cpu-subtype-vax-all) | const |  |
| [`CPU_SUBTYPE_VAX780`](#cpu-subtype-vax780) | const |  |
| [`CPU_SUBTYPE_VAX785`](#cpu-subtype-vax785) | const |  |
| [`CPU_SUBTYPE_VAX750`](#cpu-subtype-vax750) | const |  |
| [`CPU_SUBTYPE_VAX730`](#cpu-subtype-vax730) | const |  |
| [`CPU_SUBTYPE_UVAXI`](#cpu-subtype-uvaxi) | const |  |
| [`CPU_SUBTYPE_UVAXII`](#cpu-subtype-uvaxii) | const |  |
| [`CPU_SUBTYPE_VAX8200`](#cpu-subtype-vax8200) | const |  |
| [`CPU_SUBTYPE_VAX8500`](#cpu-subtype-vax8500) | const |  |
| [`CPU_SUBTYPE_VAX8600`](#cpu-subtype-vax8600) | const |  |
| [`CPU_SUBTYPE_VAX8650`](#cpu-subtype-vax8650) | const |  |
| [`CPU_SUBTYPE_VAX8800`](#cpu-subtype-vax8800) | const |  |
| [`CPU_SUBTYPE_UVAXIII`](#cpu-subtype-uvaxiii) | const |  |
| [`CPU_SUBTYPE_MC680X0_ALL`](#cpu-subtype-mc680x0-all) | const |  |
| [`CPU_SUBTYPE_MC68030`](#cpu-subtype-mc68030) | const |  |
| [`CPU_SUBTYPE_MC68040`](#cpu-subtype-mc68040) | const |  |
| [`CPU_SUBTYPE_MC68030_ONLY`](#cpu-subtype-mc68030-only) | const |  |
| [`CPU_SUBTYPE_I386_ALL`](#cpu-subtype-i386-all) | const |  |
| [`CPU_SUBTYPE_386`](#cpu-subtype-386) | const |  |
| [`CPU_SUBTYPE_486`](#cpu-subtype-486) | const |  |
| [`CPU_SUBTYPE_486SX`](#cpu-subtype-486sx) | const |  |
| [`CPU_SUBTYPE_586`](#cpu-subtype-586) | const |  |
| [`CPU_SUBTYPE_PENT`](#cpu-subtype-pent) | const |  |
| [`CPU_SUBTYPE_PENTPRO`](#cpu-subtype-pentpro) | const |  |
| [`CPU_SUBTYPE_PENTII_M3`](#cpu-subtype-pentii-m3) | const |  |
| [`CPU_SUBTYPE_PENTII_M5`](#cpu-subtype-pentii-m5) | const |  |
| [`CPU_SUBTYPE_CELERON`](#cpu-subtype-celeron) | const |  |
| [`CPU_SUBTYPE_CELERON_MOBILE`](#cpu-subtype-celeron-mobile) | const |  |
| [`CPU_SUBTYPE_PENTIUM_3`](#cpu-subtype-pentium-3) | const |  |
| [`CPU_SUBTYPE_PENTIUM_3_M`](#cpu-subtype-pentium-3-m) | const |  |
| [`CPU_SUBTYPE_PENTIUM_3_XEON`](#cpu-subtype-pentium-3-xeon) | const |  |
| [`CPU_SUBTYPE_PENTIUM_M`](#cpu-subtype-pentium-m) | const |  |
| [`CPU_SUBTYPE_PENTIUM_4`](#cpu-subtype-pentium-4) | const |  |
| [`CPU_SUBTYPE_PENTIUM_4_M`](#cpu-subtype-pentium-4-m) | const |  |
| [`CPU_SUBTYPE_ITANIUM`](#cpu-subtype-itanium) | const |  |
| [`CPU_SUBTYPE_ITANIUM_2`](#cpu-subtype-itanium-2) | const |  |
| [`CPU_SUBTYPE_XEON`](#cpu-subtype-xeon) | const |  |
| [`CPU_SUBTYPE_XEON_MP`](#cpu-subtype-xeon-mp) | const |  |
| [`CPU_SUBTYPE_INTEL_FAMILY_MAX`](#cpu-subtype-intel-family-max) | const |  |
| [`CPU_SUBTYPE_INTEL_MODEL_ALL`](#cpu-subtype-intel-model-all) | const |  |
| [`CPU_SUBTYPE_X86_ALL`](#cpu-subtype-x86-all) | const |  |
| [`CPU_SUBTYPE_X86_64_ALL`](#cpu-subtype-x86-64-all) | const |  |
| [`CPU_SUBTYPE_X86_ARCH1`](#cpu-subtype-x86-arch1) | const |  |
| [`CPU_SUBTYPE_X86_64_H`](#cpu-subtype-x86-64-h) | const | Haswell feature subset |
| [`CPU_SUBTYPE_MIPS_ALL`](#cpu-subtype-mips-all) | const |  |
| [`CPU_SUBTYPE_MIPS_R2300`](#cpu-subtype-mips-r2300) | const |  |
| [`CPU_SUBTYPE_MIPS_R2600`](#cpu-subtype-mips-r2600) | const |  |
| [`CPU_SUBTYPE_MIPS_R2800`](#cpu-subtype-mips-r2800) | const |  |
| [`CPU_SUBTYPE_MIPS_R2000A`](#cpu-subtype-mips-r2000a) | const | pmax |
| [`CPU_SUBTYPE_MIPS_R2000`](#cpu-subtype-mips-r2000) | const |  |
| [`CPU_SUBTYPE_MIPS_R3000A`](#cpu-subtype-mips-r3000a) | const | 3max |
| [`CPU_SUBTYPE_MIPS_R3000`](#cpu-subtype-mips-r3000) | const |  |
| [`CPU_SUBTYPE_MC98000_ALL`](#cpu-subtype-mc98000-all) | const |  |
| [`CPU_SUBTYPE_MC98601`](#cpu-subtype-mc98601) | const |  |
| [`CPU_SUBTYPE_HPPA_ALL`](#cpu-subtype-hppa-all) | const |  |
| [`CPU_SUBTYPE_HPPA_7100LC`](#cpu-subtype-hppa-7100lc) | const |  |
| [`CPU_SUBTYPE_MC88000_ALL`](#cpu-subtype-mc88000-all) | const |  |
| [`CPU_SUBTYPE_MC88100`](#cpu-subtype-mc88100) | const |  |
| [`CPU_SUBTYPE_MC88110`](#cpu-subtype-mc88110) | const |  |
| [`CPU_SUBTYPE_SPARC_ALL`](#cpu-subtype-sparc-all) | const |  |
| [`CPU_SUBTYPE_I860_ALL`](#cpu-subtype-i860-all) | const |  |
| [`CPU_SUBTYPE_I860_860`](#cpu-subtype-i860-860) | const |  |
| [`CPU_SUBTYPE_POWERPC_ALL`](#cpu-subtype-powerpc-all) | const |  |
| [`CPU_SUBTYPE_POWERPC_601`](#cpu-subtype-powerpc-601) | const |  |
| [`CPU_SUBTYPE_POWERPC_602`](#cpu-subtype-powerpc-602) | const |  |
| [`CPU_SUBTYPE_POWERPC_603`](#cpu-subtype-powerpc-603) | const |  |
| [`CPU_SUBTYPE_POWERPC_603E`](#cpu-subtype-powerpc-603e) | const |  |
| [`CPU_SUBTYPE_POWERPC_603EV`](#cpu-subtype-powerpc-603ev) | const |  |
| [`CPU_SUBTYPE_POWERPC_604`](#cpu-subtype-powerpc-604) | const |  |
| [`CPU_SUBTYPE_POWERPC_604E`](#cpu-subtype-powerpc-604e) | const |  |
| [`CPU_SUBTYPE_POWERPC_620`](#cpu-subtype-powerpc-620) | const |  |
| [`CPU_SUBTYPE_POWERPC_750`](#cpu-subtype-powerpc-750) | const |  |
| [`CPU_SUBTYPE_POWERPC_7400`](#cpu-subtype-powerpc-7400) | const |  |
| [`CPU_SUBTYPE_POWERPC_7450`](#cpu-subtype-powerpc-7450) | const |  |
| [`CPU_SUBTYPE_POWERPC_970`](#cpu-subtype-powerpc-970) | const |  |
| [`CPU_SUBTYPE_ARM_ALL`](#cpu-subtype-arm-all) | const |  |
| [`CPU_SUBTYPE_ARM_V4T`](#cpu-subtype-arm-v4t) | const |  |
| [`CPU_SUBTYPE_ARM_V6`](#cpu-subtype-arm-v6) | const |  |
| [`CPU_SUBTYPE_ARM_V5TEJ`](#cpu-subtype-arm-v5tej) | const |  |
| [`CPU_SUBTYPE_ARM_XSCALE`](#cpu-subtype-arm-xscale) | const |  |
| [`CPU_SUBTYPE_ARM_V7`](#cpu-subtype-arm-v7) | const | ARMv7-A and ARMv7-R |
| [`CPU_SUBTYPE_ARM_V7F`](#cpu-subtype-arm-v7f) | const | Cortex A9 |
| [`CPU_SUBTYPE_ARM_V7S`](#cpu-subtype-arm-v7s) | const | Swift |
| [`CPU_SUBTYPE_ARM_V7K`](#cpu-subtype-arm-v7k) | const |  |
| [`CPU_SUBTYPE_ARM_V8`](#cpu-subtype-arm-v8) | const |  |
| [`CPU_SUBTYPE_ARM_V6M`](#cpu-subtype-arm-v6m) | const | Not meant to be run under xnu |
| [`CPU_SUBTYPE_ARM_V7M`](#cpu-subtype-arm-v7m) | const | Not meant to be run under xnu |
| [`CPU_SUBTYPE_ARM_V7EM`](#cpu-subtype-arm-v7em) | const | Not meant to be run under xnu |
| [`CPU_SUBTYPE_ARM_V8M`](#cpu-subtype-arm-v8m) | const | Not meant to be run under xnu |
| [`CPU_SUBTYPE_ARM64_ALL`](#cpu-subtype-arm64-all) | const |  |
| [`CPU_SUBTYPE_ARM64_V8`](#cpu-subtype-arm64-v8) | const |  |
| [`CPU_SUBTYPE_ARM64E`](#cpu-subtype-arm64e) | const |  |
| [`CPU_SUBTYPE_ARM64_32_ALL`](#cpu-subtype-arm64-32-all) | const |  |
| [`CPU_SUBTYPE_ARM64_32_V8`](#cpu-subtype-arm64-32-v8) | const |  |
| [`VM_PROT_READ`](#vm-prot-read) | const | read permission |
| [`VM_PROT_WRITE`](#vm-prot-write) | const | write permission |
| [`VM_PROT_EXECUTE`](#vm-prot-execute) | const | execute permission |
| [`DYLD_CACHE_MAPPING_AUTH_DATA`](#dyld-cache-mapping-auth-data) | const |  |
| [`DYLD_CACHE_MAPPING_DIRTY_DATA`](#dyld-cache-mapping-dirty-data) | const |  |
| [`DYLD_CACHE_MAPPING_CONST_DATA`](#dyld-cache-mapping-const-data) | const |  |
| [`DYLD_CACHE_MAPPING_TEXT_STUBS`](#dyld-cache-mapping-text-stubs) | const |  |
| [`DYLD_CACHE_DYNAMIC_CONFIG_DATA`](#dyld-cache-dynamic-config-data) | const |  |
| [`DYLD_CACHE_SLIDE_PAGE_ATTRS`](#dyld-cache-slide-page-attrs) | const |  |
| [`DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA`](#dyld-cache-slide-page-attr-extra) | const |  |
| [`DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE`](#dyld-cache-slide-page-attr-no-rebase) | const |  |
| [`DYLD_CACHE_SLIDE_PAGE_ATTR_END`](#dyld-cache-slide-page-attr-end) | const |  |
| [`DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE`](#dyld-cache-slide-v3-page-attr-no-rebase) | const | Page has no rebasing. |
| [`DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE`](#dyld-cache-slide-v5-page-attr-no-rebase) | const | Page has no rebasing. |
| [`FAT_MAGIC`](#fat-magic) | const |  |
| [`FAT_CIGAM`](#fat-cigam) | const | NXSwapLong(FAT_MAGIC) |
| [`FAT_MAGIC_64`](#fat-magic-64) | const |  |
| [`FAT_CIGAM_64`](#fat-cigam-64) | const | NXSwapLong(FAT_MAGIC_64) |
| [`MH_MAGIC`](#mh-magic) | const | the mach magic number |
| [`MH_CIGAM`](#mh-cigam) | const | NXSwapInt(MH_MAGIC) |
| [`MH_MAGIC_64`](#mh-magic-64) | const | the 64-bit mach magic number |
| [`MH_CIGAM_64`](#mh-cigam-64) | const | NXSwapInt(MH_MAGIC_64) |
| [`MH_OBJECT`](#mh-object) | const | relocatable object file |
| [`MH_EXECUTE`](#mh-execute) | const | demand paged executable file |
| [`MH_FVMLIB`](#mh-fvmlib) | const | fixed VM shared library file |
| [`MH_CORE`](#mh-core) | const | core file |
| [`MH_PRELOAD`](#mh-preload) | const | preloaded executable file |
| [`MH_DYLIB`](#mh-dylib) | const | dynamically bound shared library |
| [`MH_DYLINKER`](#mh-dylinker) | const | dynamic link editor |
| [`MH_BUNDLE`](#mh-bundle) | const | dynamically bound bundle file |
| [`MH_DYLIB_STUB`](#mh-dylib-stub) | const | shared library stub for static linking only, no section contents |
| [`MH_DSYM`](#mh-dsym) | const | companion file with only debug sections |
| [`MH_KEXT_BUNDLE`](#mh-kext-bundle) | const | x86_64 kexts |
| [`MH_FILESET`](#mh-fileset) | const | set of mach-o's |
| [`MH_NOUNDEFS`](#mh-noundefs) | const | the object file has no undefined references |
| [`MH_INCRLINK`](#mh-incrlink) | const | the object file is the output of an incremental link against a base file and can't be link edited again |
| [`MH_DYLDLINK`](#mh-dyldlink) | const | the object file is input for the dynamic linker and can't be statically link edited again |
| [`MH_BINDATLOAD`](#mh-bindatload) | const | the object file's undefined references are bound by the dynamic linker when loaded. |
| [`MH_PREBOUND`](#mh-prebound) | const | the file has its dynamic undefined references prebound. |
| [`MH_SPLIT_SEGS`](#mh-split-segs) | const | the file has its read-only and read-write segments split |
| [`MH_LAZY_INIT`](#mh-lazy-init) | const | the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete) |
| [`MH_TWOLEVEL`](#mh-twolevel) | const | the image is using two-level name space bindings |
| [`MH_FORCE_FLAT`](#mh-force-flat) | const | the executable is forcing all images to use flat name space bindings |
| [`MH_NOMULTIDEFS`](#mh-nomultidefs) | const | this umbrella guarantees no multiple definitions of symbols in its sub-images so the two-level namespace hints can always be used. |
| [`MH_NOFIXPREBINDING`](#mh-nofixprebinding) | const | do not have dyld notify the prebinding agent about this executable |
| [`MH_PREBINDABLE`](#mh-prebindable) | const | the binary is not prebound but can have its prebinding redone. |
| [`MH_ALLMODSBOUND`](#mh-allmodsbound) | const | indicates that this binary binds to all two-level namespace modules of its dependent libraries. |
| [`MH_SUBSECTIONS_VIA_SYMBOLS`](#mh-subsections-via-symbols) | const | safe to divide up the sections into sub-sections via symbols for dead code stripping |
| [`MH_CANONICAL`](#mh-canonical) | const | the binary has been canonicalized via the unprebind operation |
| [`MH_WEAK_DEFINES`](#mh-weak-defines) | const | the final linked image contains external weak symbols |
| [`MH_BINDS_TO_WEAK`](#mh-binds-to-weak) | const | the final linked image uses weak symbols |
| [`MH_ALLOW_STACK_EXECUTION`](#mh-allow-stack-execution) | const | When this bit is set, all stacks in the task will be given stack execution privilege. |
| [`MH_ROOT_SAFE`](#mh-root-safe) | const | When this bit is set, the binary declares it is safe for use in processes with uid zero |
| [`MH_SETUID_SAFE`](#mh-setuid-safe) | const | When this bit is set, the binary declares it is safe for use in processes when issetugid() is true |
| [`MH_NO_REEXPORTED_DYLIBS`](#mh-no-reexported-dylibs) | const | When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported |
| [`MH_PIE`](#mh-pie) | const | When this bit is set, the OS will load the main executable at a random address. |
| [`MH_DEAD_STRIPPABLE_DYLIB`](#mh-dead-strippable-dylib) | const | Only for use on dylibs. |
| [`MH_HAS_TLV_DESCRIPTORS`](#mh-has-tlv-descriptors) | const | Contains a section of type S_THREAD_LOCAL_VARIABLES |
| [`MH_NO_HEAP_EXECUTION`](#mh-no-heap-execution) | const | When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that don't require it. |
| [`MH_APP_EXTENSION_SAFE`](#mh-app-extension-safe) | const | The code was linked for use in an application extension. |
| [`MH_NLIST_OUTOFSYNC_WITH_DYLDINFO`](#mh-nlist-outofsync-with-dyldinfo) | const | The external symbols listed in the nlist symbol table do not include all the symbols listed in the dyld info. |
| [`MH_SIM_SUPPORT`](#mh-sim-support) | const | Allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with the platforms macOS, iOSMac, iOSSimulator, tvOSSimulator and watchOSSimulator. |
| [`MH_DYLIB_IN_CACHE`](#mh-dylib-in-cache) | const | Only for use on dylibs. |
| [`LC_REQ_DYLD`](#lc-req-dyld) | const |  |
| [`LC_SEGMENT`](#lc-segment) | const | segment of this file to be mapped |
| [`LC_SYMTAB`](#lc-symtab) | const | link-edit stab symbol table info |
| [`LC_SYMSEG`](#lc-symseg) | const | link-edit gdb symbol table info (obsolete) |
| [`LC_THREAD`](#lc-thread) | const | thread |
| [`LC_UNIXTHREAD`](#lc-unixthread) | const | unix thread (includes a stack) |
| [`LC_LOADFVMLIB`](#lc-loadfvmlib) | const | load a specified fixed VM shared library |
| [`LC_IDFVMLIB`](#lc-idfvmlib) | const | fixed VM shared library identification |
| [`LC_IDENT`](#lc-ident) | const | object identification info (obsolete) |
| [`LC_FVMFILE`](#lc-fvmfile) | const | fixed VM file inclusion (internal use) |
| [`LC_PREPAGE`](#lc-prepage) | const | prepage command (internal use) |
| [`LC_DYSYMTAB`](#lc-dysymtab) | const | dynamic link-edit symbol table info |
| [`LC_LOAD_DYLIB`](#lc-load-dylib) | const | load a dynamically linked shared library |
| [`LC_ID_DYLIB`](#lc-id-dylib) | const | dynamically linked shared lib ident |
| [`LC_LOAD_DYLINKER`](#lc-load-dylinker) | const | load a dynamic linker |
| [`LC_ID_DYLINKER`](#lc-id-dylinker) | const | dynamic linker identification |
| [`LC_PREBOUND_DYLIB`](#lc-prebound-dylib) | const | modules prebound for a dynamically linked shared library |
| [`LC_ROUTINES`](#lc-routines) | const | image routines |
| [`LC_SUB_FRAMEWORK`](#lc-sub-framework) | const | sub framework |
| [`LC_SUB_UMBRELLA`](#lc-sub-umbrella) | const | sub umbrella |
| [`LC_SUB_CLIENT`](#lc-sub-client) | const | sub client |
| [`LC_SUB_LIBRARY`](#lc-sub-library) | const | sub library |
| [`LC_TWOLEVEL_HINTS`](#lc-twolevel-hints) | const | two-level namespace lookup hints |
| [`LC_PREBIND_CKSUM`](#lc-prebind-cksum) | const | prebind checksum |
| [`LC_LOAD_WEAK_DYLIB`](#lc-load-weak-dylib) | const | load a dynamically linked shared library that is allowed to be missing (all symbols are weak imported). |
| [`LC_SEGMENT_64`](#lc-segment-64) | const | 64-bit segment of this file to be mapped |
| [`LC_ROUTINES_64`](#lc-routines-64) | const | 64-bit image routines |
| [`LC_UUID`](#lc-uuid) | const | the uuid |
| [`LC_RPATH`](#lc-rpath) | const | runpath additions |
| [`LC_CODE_SIGNATURE`](#lc-code-signature) | const | local of code signature |
| [`LC_SEGMENT_SPLIT_INFO`](#lc-segment-split-info) | const | local of info to split segments |
| [`LC_REEXPORT_DYLIB`](#lc-reexport-dylib) | const | load and re-export dylib |
| [`LC_LAZY_LOAD_DYLIB`](#lc-lazy-load-dylib) | const | delay load of dylib until first use |
| [`LC_ENCRYPTION_INFO`](#lc-encryption-info) | const | encrypted segment information |
| [`LC_DYLD_INFO`](#lc-dyld-info) | const | compressed dyld information |
| [`LC_DYLD_INFO_ONLY`](#lc-dyld-info-only) | const | compressed dyld information only |
| [`LC_LOAD_UPWARD_DYLIB`](#lc-load-upward-dylib) | const | load upward dylib |
| [`LC_VERSION_MIN_MACOSX`](#lc-version-min-macosx) | const | build for MacOSX min OS version |
| [`LC_VERSION_MIN_IPHONEOS`](#lc-version-min-iphoneos) | const | build for iPhoneOS min OS version |
| [`LC_FUNCTION_STARTS`](#lc-function-starts) | const | compressed table of function start addresses |
| [`LC_DYLD_ENVIRONMENT`](#lc-dyld-environment) | const | string for dyld to treat like environment variable |
| [`LC_MAIN`](#lc-main) | const | replacement for LC_UNIXTHREAD |
| [`LC_DATA_IN_CODE`](#lc-data-in-code) | const | table of non-instructions in __text |
| [`LC_SOURCE_VERSION`](#lc-source-version) | const | source version used to build binary |
| [`LC_DYLIB_CODE_SIGN_DRS`](#lc-dylib-code-sign-drs) | const | Code signing DRs copied from linked dylibs |
| [`LC_ENCRYPTION_INFO_64`](#lc-encryption-info-64) | const | 64-bit encrypted segment information |
| [`LC_LINKER_OPTION`](#lc-linker-option) | const | linker options in MH_OBJECT files |
| [`LC_LINKER_OPTIMIZATION_HINT`](#lc-linker-optimization-hint) | const | optimization hints in MH_OBJECT files |
| [`LC_VERSION_MIN_TVOS`](#lc-version-min-tvos) | const | build for AppleTV min OS version |
| [`LC_VERSION_MIN_WATCHOS`](#lc-version-min-watchos) | const | build for Watch min OS version |
| [`LC_NOTE`](#lc-note) | const | arbitrary data included within a Mach-O file |
| [`LC_BUILD_VERSION`](#lc-build-version) | const | build for platform min OS version |
| [`LC_DYLD_EXPORTS_TRIE`](#lc-dyld-exports-trie) | const | used with `LinkeditDataCommand`, payload is trie |
| [`LC_DYLD_CHAINED_FIXUPS`](#lc-dyld-chained-fixups) | const | used with `LinkeditDataCommand` |
| [`LC_FILESET_ENTRY`](#lc-fileset-entry) | const | used with `FilesetEntryCommand` |
| [`SG_HIGHVM`](#sg-highvm) | const | the file contents for this segment is for the high part of the VM space, the low part is zero filled (for stacks in core files) |
| [`SG_FVMLIB`](#sg-fvmlib) | const | this segment is the VM that is allocated by a fixed VM library, for overlap checking in the link editor |
| [`SG_NORELOC`](#sg-noreloc) | const | this segment has nothing that was relocated in it and nothing relocated to it, that is it maybe safely replaced without relocation |
| [`SG_PROTECTED_VERSION_1`](#sg-protected-version-1) | const | This segment is protected. |
| [`SG_READ_ONLY`](#sg-read-only) | const | This segment is made read-only after fixups |
| [`SECTION_TYPE`](#section-type) | const | 256 section types |
| [`SECTION_ATTRIBUTES`](#section-attributes) | const | 24 section attributes |
| [`S_REGULAR`](#s-regular) | const | regular section |
| [`S_ZEROFILL`](#s-zerofill) | const | zero fill on demand section |
| [`S_CSTRING_LITERALS`](#s-cstring-literals) | const | section with only literal C strings |
| [`S_4BYTE_LITERALS`](#s-4byte-literals) | const | section with only 4 byte literals |
| [`S_8BYTE_LITERALS`](#s-8byte-literals) | const | section with only 8 byte literals |
| [`S_LITERAL_POINTERS`](#s-literal-pointers) | const | section with only pointers to literals |
| [`S_NON_LAZY_SYMBOL_POINTERS`](#s-non-lazy-symbol-pointers) | const | section with only non-lazy symbol pointers |
| [`S_LAZY_SYMBOL_POINTERS`](#s-lazy-symbol-pointers) | const | section with only lazy symbol pointers |
| [`S_SYMBOL_STUBS`](#s-symbol-stubs) | const | section with only symbol stubs, byte size of stub in the reserved2 field |
| [`S_MOD_INIT_FUNC_POINTERS`](#s-mod-init-func-pointers) | const | section with only function pointers for initialization |
| [`S_MOD_TERM_FUNC_POINTERS`](#s-mod-term-func-pointers) | const | section with only function pointers for termination |
| [`S_COALESCED`](#s-coalesced) | const | section contains symbols that are to be coalesced |
| [`S_GB_ZEROFILL`](#s-gb-zerofill) | const | zero fill on demand section (that can be larger than 4 gigabytes) |
| [`S_INTERPOSING`](#s-interposing) | const | section with only pairs of function pointers for interposing |
| [`S_16BYTE_LITERALS`](#s-16byte-literals) | const | section with only 16 byte literals |
| [`S_DTRACE_DOF`](#s-dtrace-dof) | const | section contains DTrace Object Format |
| [`S_LAZY_DYLIB_SYMBOL_POINTERS`](#s-lazy-dylib-symbol-pointers) | const | section with only lazy symbol pointers to lazy loaded dylibs |
| [`S_THREAD_LOCAL_REGULAR`](#s-thread-local-regular) | const | template of initial values for TLVs |
| [`S_THREAD_LOCAL_ZEROFILL`](#s-thread-local-zerofill) | const | template of initial values for TLVs |
| [`S_THREAD_LOCAL_VARIABLES`](#s-thread-local-variables) | const | TLV descriptors |
| [`S_THREAD_LOCAL_VARIABLE_POINTERS`](#s-thread-local-variable-pointers) | const | pointers to TLV descriptors |
| [`S_THREAD_LOCAL_INIT_FUNCTION_POINTERS`](#s-thread-local-init-function-pointers) | const | functions to call to initialize TLV values |
| [`S_INIT_FUNC_OFFSETS`](#s-init-func-offsets) | const | 32-bit offsets to initializers |
| [`SECTION_ATTRIBUTES_USR`](#section-attributes-usr) | const | User setable attributes |
| [`S_ATTR_PURE_INSTRUCTIONS`](#s-attr-pure-instructions) | const | section contains only true machine instructions |
| [`S_ATTR_NO_TOC`](#s-attr-no-toc) | const | section contains coalesced symbols that are not to be in a ranlib table of contents |
| [`S_ATTR_STRIP_STATIC_SYMS`](#s-attr-strip-static-syms) | const | ok to strip static symbols in this section in files with the MH_DYLDLINK flag |
| [`S_ATTR_NO_DEAD_STRIP`](#s-attr-no-dead-strip) | const | no dead stripping |
| [`S_ATTR_LIVE_SUPPORT`](#s-attr-live-support) | const | blocks are live if they reference live blocks |
| [`S_ATTR_SELF_MODIFYING_CODE`](#s-attr-self-modifying-code) | const | Used with i386 code stubs written on by dyld |
| [`S_ATTR_DEBUG`](#s-attr-debug) | const | a debug section |
| [`SECTION_ATTRIBUTES_SYS`](#section-attributes-sys) | const | system setable attributes |
| [`S_ATTR_SOME_INSTRUCTIONS`](#s-attr-some-instructions) | const | section contains some machine instructions |
| [`S_ATTR_EXT_RELOC`](#s-attr-ext-reloc) | const | section has external relocation entries |
| [`S_ATTR_LOC_RELOC`](#s-attr-loc-reloc) | const | section has local relocation entries |
| [`SEG_PAGEZERO`](#seg-pagezero) | const | the pagezero segment which has no protections and catches NULL references for MH_EXECUTE files |
| [`SEG_TEXT`](#seg-text) | const | the tradition UNIX text segment |
| [`SECT_TEXT`](#sect-text) | const | the real text part of the text section no headers, and no padding |
| [`SECT_FVMLIB_INIT0`](#sect-fvmlib-init0) | const | the fvmlib initialization section |
| [`SECT_FVMLIB_INIT1`](#sect-fvmlib-init1) | const | the section following the fvmlib initialization section |
| [`SEG_DATA`](#seg-data) | const | the tradition UNIX data segment |
| [`SECT_DATA`](#sect-data) | const | the real initialized data section no padding, no bss overlap |
| [`SECT_BSS`](#sect-bss) | const | the real uninitialized data section no padding |
| [`SECT_COMMON`](#sect-common) | const | the section common symbols are allocated in by the link editor |
| [`SEG_OBJC`](#seg-objc) | const | objective-C runtime segment |
| [`SECT_OBJC_SYMBOLS`](#sect-objc-symbols) | const | symbol table |
| [`SECT_OBJC_MODULES`](#sect-objc-modules) | const | module information |
| [`SECT_OBJC_STRINGS`](#sect-objc-strings) | const | string table |
| [`SECT_OBJC_REFS`](#sect-objc-refs) | const | string table |
| [`SEG_ICON`](#seg-icon) | const | the icon segment |
| [`SECT_ICON_HEADER`](#sect-icon-header) | const | the icon headers |
| [`SECT_ICON_TIFF`](#sect-icon-tiff) | const | the icons in tiff format |
| [`SEG_LINKEDIT`](#seg-linkedit) | const | the segment containing all structs created and maintained by the link editor. |
| [`SEG_LINKINFO`](#seg-linkinfo) | const | the segment overlapping with linkedit containing linking information |
| [`SEG_UNIXSTACK`](#seg-unixstack) | const | the unix stack segment |
| [`SEG_IMPORT`](#seg-import) | const | the segment for the self (dyld) modifying code stubs that has read, write and execute permissions |
| [`INDIRECT_SYMBOL_LOCAL`](#indirect-symbol-local) | const |  |
| [`INDIRECT_SYMBOL_ABS`](#indirect-symbol-abs) | const |  |
| [`PLATFORM_MACOS`](#platform-macos) | const |  |
| [`PLATFORM_IOS`](#platform-ios) | const |  |
| [`PLATFORM_TVOS`](#platform-tvos) | const |  |
| [`PLATFORM_WATCHOS`](#platform-watchos) | const |  |
| [`PLATFORM_BRIDGEOS`](#platform-bridgeos) | const |  |
| [`PLATFORM_MACCATALYST`](#platform-maccatalyst) | const |  |
| [`PLATFORM_IOSSIMULATOR`](#platform-iossimulator) | const |  |
| [`PLATFORM_TVOSSIMULATOR`](#platform-tvossimulator) | const |  |
| [`PLATFORM_WATCHOSSIMULATOR`](#platform-watchossimulator) | const |  |
| [`PLATFORM_DRIVERKIT`](#platform-driverkit) | const |  |
| [`PLATFORM_XROS`](#platform-xros) | const |  |
| [`PLATFORM_XROSSIMULATOR`](#platform-xrossimulator) | const |  |
| [`TOOL_CLANG`](#tool-clang) | const |  |
| [`TOOL_SWIFT`](#tool-swift) | const |  |
| [`TOOL_LD`](#tool-ld) | const |  |
| [`REBASE_TYPE_POINTER`](#rebase-type-pointer) | const |  |
| [`REBASE_TYPE_TEXT_ABSOLUTE32`](#rebase-type-text-absolute32) | const |  |
| [`REBASE_TYPE_TEXT_PCREL32`](#rebase-type-text-pcrel32) | const |  |
| [`REBASE_OPCODE_MASK`](#rebase-opcode-mask) | const |  |
| [`REBASE_IMMEDIATE_MASK`](#rebase-immediate-mask) | const |  |
| [`REBASE_OPCODE_DONE`](#rebase-opcode-done) | const |  |
| [`REBASE_OPCODE_SET_TYPE_IMM`](#rebase-opcode-set-type-imm) | const |  |
| [`REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`](#rebase-opcode-set-segment-and-offset-uleb) | const |  |
| [`REBASE_OPCODE_ADD_ADDR_ULEB`](#rebase-opcode-add-addr-uleb) | const |  |
| [`REBASE_OPCODE_ADD_ADDR_IMM_SCALED`](#rebase-opcode-add-addr-imm-scaled) | const |  |
| [`REBASE_OPCODE_DO_REBASE_IMM_TIMES`](#rebase-opcode-do-rebase-imm-times) | const |  |
| [`REBASE_OPCODE_DO_REBASE_ULEB_TIMES`](#rebase-opcode-do-rebase-uleb-times) | const |  |
| [`REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB`](#rebase-opcode-do-rebase-add-addr-uleb) | const |  |
| [`REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB`](#rebase-opcode-do-rebase-uleb-times-skipping-uleb) | const |  |
| [`BIND_TYPE_POINTER`](#bind-type-pointer) | const |  |
| [`BIND_TYPE_TEXT_ABSOLUTE32`](#bind-type-text-absolute32) | const |  |
| [`BIND_TYPE_TEXT_PCREL32`](#bind-type-text-pcrel32) | const |  |
| [`BIND_SPECIAL_DYLIB_SELF`](#bind-special-dylib-self) | const |  |
| [`BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE`](#bind-special-dylib-main-executable) | const |  |
| [`BIND_SPECIAL_DYLIB_FLAT_LOOKUP`](#bind-special-dylib-flat-lookup) | const |  |
| [`BIND_SPECIAL_DYLIB_WEAK_LOOKUP`](#bind-special-dylib-weak-lookup) | const |  |
| [`BIND_SYMBOL_FLAGS_WEAK_IMPORT`](#bind-symbol-flags-weak-import) | const |  |
| [`BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION`](#bind-symbol-flags-non-weak-definition) | const |  |
| [`BIND_OPCODE_MASK`](#bind-opcode-mask) | const |  |
| [`BIND_IMMEDIATE_MASK`](#bind-immediate-mask) | const |  |
| [`BIND_OPCODE_DONE`](#bind-opcode-done) | const |  |
| [`BIND_OPCODE_SET_DYLIB_ORDINAL_IMM`](#bind-opcode-set-dylib-ordinal-imm) | const |  |
| [`BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB`](#bind-opcode-set-dylib-ordinal-uleb) | const |  |
| [`BIND_OPCODE_SET_DYLIB_SPECIAL_IMM`](#bind-opcode-set-dylib-special-imm) | const |  |
| [`BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM`](#bind-opcode-set-symbol-trailing-flags-imm) | const |  |
| [`BIND_OPCODE_SET_TYPE_IMM`](#bind-opcode-set-type-imm) | const |  |
| [`BIND_OPCODE_SET_ADDEND_SLEB`](#bind-opcode-set-addend-sleb) | const |  |
| [`BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`](#bind-opcode-set-segment-and-offset-uleb) | const |  |
| [`BIND_OPCODE_ADD_ADDR_ULEB`](#bind-opcode-add-addr-uleb) | const |  |
| [`BIND_OPCODE_DO_BIND`](#bind-opcode-do-bind) | const |  |
| [`BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB`](#bind-opcode-do-bind-add-addr-uleb) | const |  |
| [`BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED`](#bind-opcode-do-bind-add-addr-imm-scaled) | const |  |
| [`BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB`](#bind-opcode-do-bind-uleb-times-skipping-uleb) | const |  |
| [`BIND_OPCODE_THREADED`](#bind-opcode-threaded) | const |  |
| [`BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB`](#bind-subopcode-threaded-set-bind-ordinal-table-size-uleb) | const |  |
| [`BIND_SUBOPCODE_THREADED_APPLY`](#bind-subopcode-threaded-apply) | const |  |
| [`EXPORT_SYMBOL_FLAGS_KIND_MASK`](#export-symbol-flags-kind-mask) | const |  |
| [`EXPORT_SYMBOL_FLAGS_KIND_REGULAR`](#export-symbol-flags-kind-regular) | const |  |
| [`EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL`](#export-symbol-flags-kind-thread-local) | const |  |
| [`EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE`](#export-symbol-flags-kind-absolute) | const |  |
| [`EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION`](#export-symbol-flags-weak-definition) | const |  |
| [`EXPORT_SYMBOL_FLAGS_REEXPORT`](#export-symbol-flags-reexport) | const |  |
| [`EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER`](#export-symbol-flags-stub-and-resolver) | const |  |
| [`DICE_KIND_DATA`](#dice-kind-data) | const |  |
| [`DICE_KIND_JUMP_TABLE8`](#dice-kind-jump-table8) | const |  |
| [`DICE_KIND_JUMP_TABLE16`](#dice-kind-jump-table16) | const |  |
| [`DICE_KIND_JUMP_TABLE32`](#dice-kind-jump-table32) | const |  |
| [`DICE_KIND_ABS_JUMP_TABLE32`](#dice-kind-abs-jump-table32) | const |  |
| [`N_STAB`](#n-stab) | const | if any of these bits set, a symbolic debugging entry |
| [`N_PEXT`](#n-pext) | const | private external symbol bit |
| [`N_TYPE`](#n-type) | const | mask for the type bits |
| [`N_EXT`](#n-ext) | const | external symbol bit, set for external symbols |
| [`N_UNDF`](#n-undf) | const | undefined, n_sect == NO_SECT |
| [`N_ABS`](#n-abs) | const | absolute, n_sect == NO_SECT |
| [`N_SECT`](#n-sect) | const | defined in section number n_sect |
| [`N_PBUD`](#n-pbud) | const | prebound undefined (defined in a dylib) |
| [`N_INDR`](#n-indr) | const | indirect |
| [`NO_SECT`](#no-sect) | const | symbol is not in any section |
| [`MAX_SECT`](#max-sect) | const | 1 thru 255 inclusive |
| [`REFERENCE_TYPE`](#reference-type) | const |  |
| [`REFERENCE_FLAG_UNDEFINED_NON_LAZY`](#reference-flag-undefined-non-lazy) | const |  |
| [`REFERENCE_FLAG_UNDEFINED_LAZY`](#reference-flag-undefined-lazy) | const |  |
| [`REFERENCE_FLAG_DEFINED`](#reference-flag-defined) | const |  |
| [`REFERENCE_FLAG_PRIVATE_DEFINED`](#reference-flag-private-defined) | const |  |
| [`REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY`](#reference-flag-private-undefined-non-lazy) | const |  |
| [`REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY`](#reference-flag-private-undefined-lazy) | const |  |
| [`REFERENCED_DYNAMICALLY`](#referenced-dynamically) | const |  |
| [`SELF_LIBRARY_ORDINAL`](#self-library-ordinal) | const |  |
| [`MAX_LIBRARY_ORDINAL`](#max-library-ordinal) | const |  |
| [`DYNAMIC_LOOKUP_ORDINAL`](#dynamic-lookup-ordinal) | const |  |
| [`EXECUTABLE_ORDINAL`](#executable-ordinal) | const |  |
| [`N_NO_DEAD_STRIP`](#n-no-dead-strip) | const | symbol is not to be dead stripped |
| [`N_DESC_DISCARDED`](#n-desc-discarded) | const | symbol is discarded |
| [`N_WEAK_REF`](#n-weak-ref) | const | symbol is weak referenced |
| [`N_WEAK_DEF`](#n-weak-def) | const | coalesced symbol is a weak definition |
| [`N_REF_TO_WEAK`](#n-ref-to-weak) | const | reference to a weak symbol |
| [`N_ARM_THUMB_DEF`](#n-arm-thumb-def) | const | symbol is a Thumb function (ARM) |
| [`N_SYMBOL_RESOLVER`](#n-symbol-resolver) | const |  |
| [`N_ALT_ENTRY`](#n-alt-entry) | const |  |
| [`N_GSYM`](#n-gsym) | const | global symbol: name,,NO_SECT,type,0 |
| [`N_FNAME`](#n-fname) | const | procedure name (f77 kludge): name,,NO_SECT,0,0 |
| [`N_FUN`](#n-fun) | const | procedure: name,,n_sect,linenumber,address |
| [`N_STSYM`](#n-stsym) | const | static symbol: name,,n_sect,type,address |
| [`N_LCSYM`](#n-lcsym) | const | .lcomm symbol: name,,n_sect,type,address |
| [`N_BNSYM`](#n-bnsym) | const | begin nsect sym: 0,,n_sect,0,address |
| [`N_AST`](#n-ast) | const | AST file path: name,,NO_SECT,0,0 |
| [`N_OPT`](#n-opt) | const | emitted with gcc2_compiled and in gcc source |
| [`N_RSYM`](#n-rsym) | const | register sym: name,,NO_SECT,type,register |
| [`N_SLINE`](#n-sline) | const | src line: 0,,n_sect,linenumber,address |
| [`N_ENSYM`](#n-ensym) | const | end nsect sym: 0,,n_sect,0,address |
| [`N_SSYM`](#n-ssym) | const | structure elt: name,,NO_SECT,type,struct_offset |
| [`N_SO`](#n-so) | const | source file name: name,,n_sect,0,address |
| [`N_OSO`](#n-oso) | const | object file name: name,,0,0,st_mtime |
| [`N_LSYM`](#n-lsym) | const | local sym: name,,NO_SECT,type,offset |
| [`N_BINCL`](#n-bincl) | const | include file beginning: name,,NO_SECT,0,sum |
| [`N_SOL`](#n-sol) | const | #included file name: name,,n_sect,0,address |
| [`N_PARAMS`](#n-params) | const | compiler parameters: name,,NO_SECT,0,0 |
| [`N_VERSION`](#n-version) | const | compiler version: name,,NO_SECT,0,0 |
| [`N_OLEVEL`](#n-olevel) | const | compiler -O level: name,,NO_SECT,0,0 |
| [`N_PSYM`](#n-psym) | const | parameter: name,,NO_SECT,type,offset |
| [`N_EINCL`](#n-eincl) | const | include file end: name,,NO_SECT,0,0 |
| [`N_ENTRY`](#n-entry) | const | alternate entry: name,,n_sect,linenumber,address |
| [`N_LBRAC`](#n-lbrac) | const | left bracket: 0,,NO_SECT,nesting level,address |
| [`N_EXCL`](#n-excl) | const | deleted include file: name,,NO_SECT,0,sum |
| [`N_RBRAC`](#n-rbrac) | const | right bracket: 0,,NO_SECT,nesting level,address |
| [`N_BCOMM`](#n-bcomm) | const | begin common: name,,NO_SECT,0,0 |
| [`N_ECOMM`](#n-ecomm) | const | end common: name,,n_sect,0,0 |
| [`N_ECOML`](#n-ecoml) | const | end common (local name): 0,,n_sect,0,address |
| [`N_LENG`](#n-leng) | const | second stab entry with length information |
| [`N_PC`](#n-pc) | const | global pascal symbol: name,,NO_SECT,subtype,line |
| [`R_ABS`](#r-abs) | const | absolute relocation type for Mach-O files |
| [`R_SCATTERED`](#r-scattered) | const | Bit set in `Relocation::r_word0` for scattered relocations. |
| [`GENERIC_RELOC_VANILLA`](#generic-reloc-vanilla) | const | generic relocation as described above |
| [`GENERIC_RELOC_PAIR`](#generic-reloc-pair) | const | Only follows a GENERIC_RELOC_SECTDIFF |
| [`GENERIC_RELOC_SECTDIFF`](#generic-reloc-sectdiff) | const |  |
| [`GENERIC_RELOC_PB_LA_PTR`](#generic-reloc-pb-la-ptr) | const | prebound lazy pointer |
| [`GENERIC_RELOC_LOCAL_SECTDIFF`](#generic-reloc-local-sectdiff) | const |  |
| [`GENERIC_RELOC_TLV`](#generic-reloc-tlv) | const | thread local variables |
| [`ARM_RELOC_VANILLA`](#arm-reloc-vanilla) | const | generic relocation as described above |
| [`ARM_RELOC_PAIR`](#arm-reloc-pair) | const | the second relocation entry of a pair |
| [`ARM_RELOC_SECTDIFF`](#arm-reloc-sectdiff) | const | a PAIR follows with subtract symbol value |
| [`ARM_RELOC_LOCAL_SECTDIFF`](#arm-reloc-local-sectdiff) | const | like ARM_RELOC_SECTDIFF, but the symbol referenced was local. |
| [`ARM_RELOC_PB_LA_PTR`](#arm-reloc-pb-la-ptr) | const | prebound lazy pointer |
| [`ARM_RELOC_BR24`](#arm-reloc-br24) | const | 24 bit branch displacement (to a word address) |
| [`ARM_THUMB_RELOC_BR22`](#arm-thumb-reloc-br22) | const | 22 bit branch displacement (to a half-word address) |
| [`ARM_THUMB_32BIT_BRANCH`](#arm-thumb-32bit-branch) | const | obsolete - a thumb 32-bit branch instruction possibly needing page-spanning branch workaround |
| [`ARM_RELOC_HALF`](#arm-reloc-half) | const |  |
| [`ARM_RELOC_HALF_SECTDIFF`](#arm-reloc-half-sectdiff) | const |  |
| [`ARM64_RELOC_UNSIGNED`](#arm64-reloc-unsigned) | const | for pointers |
| [`ARM64_RELOC_SUBTRACTOR`](#arm64-reloc-subtractor) | const | must be followed by a ARM64_RELOC_UNSIGNED |
| [`ARM64_RELOC_BRANCH26`](#arm64-reloc-branch26) | const | a B/BL instruction with 26-bit displacement |
| [`ARM64_RELOC_PAGE21`](#arm64-reloc-page21) | const | pc-rel distance to page of target |
| [`ARM64_RELOC_PAGEOFF12`](#arm64-reloc-pageoff12) | const | offset within page, scaled by r_length |
| [`ARM64_RELOC_GOT_LOAD_PAGE21`](#arm64-reloc-got-load-page21) | const | pc-rel distance to page of GOT slot |
| [`ARM64_RELOC_GOT_LOAD_PAGEOFF12`](#arm64-reloc-got-load-pageoff12) | const | offset within page of GOT slot, scaled by r_length |
| [`ARM64_RELOC_POINTER_TO_GOT`](#arm64-reloc-pointer-to-got) | const | for pointers to GOT slots |
| [`ARM64_RELOC_TLVP_LOAD_PAGE21`](#arm64-reloc-tlvp-load-page21) | const | pc-rel distance to page of TLVP slot |
| [`ARM64_RELOC_TLVP_LOAD_PAGEOFF12`](#arm64-reloc-tlvp-load-pageoff12) | const | offset within page of TLVP slot, scaled by r_length |
| [`ARM64_RELOC_ADDEND`](#arm64-reloc-addend) | const | must be followed by PAGE21 or PAGEOFF12 |
| [`ARM64_RELOC_AUTHENTICATED_POINTER`](#arm64-reloc-authenticated-pointer) | const |  |
| [`PPC_RELOC_VANILLA`](#ppc-reloc-vanilla) | const | generic relocation as described above |
| [`PPC_RELOC_PAIR`](#ppc-reloc-pair) | const | the second relocation entry of a pair |
| [`PPC_RELOC_BR14`](#ppc-reloc-br14) | const | 14 bit branch displacement (to a word address) |
| [`PPC_RELOC_BR24`](#ppc-reloc-br24) | const | 24 bit branch displacement (to a word address) |
| [`PPC_RELOC_HI16`](#ppc-reloc-hi16) | const | a PAIR follows with the low half |
| [`PPC_RELOC_LO16`](#ppc-reloc-lo16) | const | a PAIR follows with the high half |
| [`PPC_RELOC_HA16`](#ppc-reloc-ha16) | const | Same as the RELOC_HI16 except the low 16 bits and the high 16 bits are added together with the low 16 bits sign extended first. |
| [`PPC_RELOC_LO14`](#ppc-reloc-lo14) | const | Same as the LO16 except that the low 2 bits are not stored in the instruction and are always zero. |
| [`PPC_RELOC_SECTDIFF`](#ppc-reloc-sectdiff) | const | a PAIR follows with subtract symbol value |
| [`PPC_RELOC_PB_LA_PTR`](#ppc-reloc-pb-la-ptr) | const | prebound lazy pointer |
| [`PPC_RELOC_HI16_SECTDIFF`](#ppc-reloc-hi16-sectdiff) | const | section difference forms of above. |
| [`PPC_RELOC_LO16_SECTDIFF`](#ppc-reloc-lo16-sectdiff) | const | follows these with subtract symbol value |
| [`PPC_RELOC_HA16_SECTDIFF`](#ppc-reloc-ha16-sectdiff) | const |  |
| [`PPC_RELOC_JBSR`](#ppc-reloc-jbsr) | const |  |
| [`PPC_RELOC_LO14_SECTDIFF`](#ppc-reloc-lo14-sectdiff) | const |  |
| [`PPC_RELOC_LOCAL_SECTDIFF`](#ppc-reloc-local-sectdiff) | const | like PPC_RELOC_SECTDIFF, but the symbol referenced was local. |
| [`X86_64_RELOC_UNSIGNED`](#x86-64-reloc-unsigned) | const | for absolute addresses |
| [`X86_64_RELOC_SIGNED`](#x86-64-reloc-signed) | const | for signed 32-bit displacement |
| [`X86_64_RELOC_BRANCH`](#x86-64-reloc-branch) | const | a CALL/JMP instruction with 32-bit displacement |
| [`X86_64_RELOC_GOT_LOAD`](#x86-64-reloc-got-load) | const | a MOVQ load of a GOT entry |
| [`X86_64_RELOC_GOT`](#x86-64-reloc-got) | const | other GOT references |
| [`X86_64_RELOC_SUBTRACTOR`](#x86-64-reloc-subtractor) | const | must be followed by a X86_64_RELOC_UNSIGNED |
| [`X86_64_RELOC_SIGNED_1`](#x86-64-reloc-signed-1) | const | for signed 32-bit displacement with a -1 addend |
| [`X86_64_RELOC_SIGNED_2`](#x86-64-reloc-signed-2) | const | for signed 32-bit displacement with a -2 addend |
| [`X86_64_RELOC_SIGNED_4`](#x86-64-reloc-signed-4) | const | for signed 32-bit displacement with a -4 addend |
| [`X86_64_RELOC_TLV`](#x86-64-reloc-tlv) | const | for thread local variables |

## Structs

### `DyldCacheHeader<E: Endian>`

```rust
struct DyldCacheHeader<E: Endian> {
    pub magic: [u8; 16],
    pub mapping_offset: crate::endian::U32<E>,
    pub mapping_count: crate::endian::U32<E>,
    pub images_offset_old: crate::endian::U32<E>,
    pub images_count_old: crate::endian::U32<E>,
    pub dyld_base_address: crate::endian::U64<E>,
    pub code_signature_offset: crate::endian::U64<E>,
    pub code_signature_size: crate::endian::U64<E>,
    pub slide_info_offset_unused: crate::endian::U64<E>,
    pub slide_info_size_unused: crate::endian::U64<E>,
    pub local_symbols_offset: crate::endian::U64<E>,
    pub local_symbols_size: crate::endian::U64<E>,
    pub uuid: [u8; 16],
    pub cache_type: crate::endian::U64<E>,
    pub branch_pools_offset: crate::endian::U32<E>,
    pub branch_pools_count: crate::endian::U32<E>,
    pub dyld_in_cache_mh: crate::endian::U64<E>,
    pub dyld_in_cache_entry: crate::endian::U64<E>,
    pub images_text_offset: crate::endian::U64<E>,
    pub images_text_count: crate::endian::U64<E>,
    pub patch_info_addr: crate::endian::U64<E>,
    pub patch_info_size: crate::endian::U64<E>,
    pub other_image_group_addr_unused: crate::endian::U64<E>,
    pub other_image_group_size_unused: crate::endian::U64<E>,
    pub prog_closures_addr: crate::endian::U64<E>,
    pub prog_closures_size: crate::endian::U64<E>,
    pub prog_closures_trie_addr: crate::endian::U64<E>,
    pub prog_closures_trie_size: crate::endian::U64<E>,
    pub platform: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
    pub shared_region_start: crate::endian::U64<E>,
    pub shared_region_size: crate::endian::U64<E>,
    pub max_slide: crate::endian::U64<E>,
    pub dylibs_image_array_addr: crate::endian::U64<E>,
    pub dylibs_image_array_size: crate::endian::U64<E>,
    pub dylibs_trie_addr: crate::endian::U64<E>,
    pub dylibs_trie_size: crate::endian::U64<E>,
    pub other_image_array_addr: crate::endian::U64<E>,
    pub other_image_array_size: crate::endian::U64<E>,
    pub other_trie_addr: crate::endian::U64<E>,
    pub other_trie_size: crate::endian::U64<E>,
    pub mapping_with_slide_offset: crate::endian::U32<E>,
    pub mapping_with_slide_count: crate::endian::U32<E>,
    pub dylibs_pbl_state_array_addr_unused: crate::endian::U64<E>,
    pub dylibs_pbl_set_addr: crate::endian::U64<E>,
    pub programs_pbl_set_pool_addr: crate::endian::U64<E>,
    pub programs_pbl_set_pool_size: crate::endian::U64<E>,
    pub program_trie_addr: crate::endian::U64<E>,
    pub os_version: crate::endian::U32<E>,
    pub alt_platform: crate::endian::U32<E>,
    pub alt_os_version: crate::endian::U32<E>,
    reserved1: [u8; 4],
    pub swift_opts_offset: crate::endian::U64<E>,
    pub swift_opts_size: crate::endian::U64<E>,
    pub sub_cache_array_offset: crate::endian::U32<E>,
    pub sub_cache_array_count: crate::endian::U32<E>,
    pub symbol_file_uuid: [u8; 16],
    pub rosetta_read_only_addr: crate::endian::U64<E>,
    pub rosetta_read_only_size: crate::endian::U64<E>,
    pub rosetta_read_write_addr: crate::endian::U64<E>,
    pub rosetta_read_write_size: crate::endian::U64<E>,
    pub images_offset: crate::endian::U32<E>,
    pub images_count: crate::endian::U32<E>,
    pub cache_sub_type: crate::endian::U32<E>,
    pub objc_opts_offset: crate::endian::U64<E>,
    pub objc_opts_size: crate::endian::U64<E>,
    pub cache_atlas_offset: crate::endian::U64<E>,
    pub cache_atlas_size: crate::endian::U64<E>,
    pub dynamic_data_offset: crate::endian::U64<E>,
    pub dynamic_data_max_size: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:314-454`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L314-L454)*

The dyld cache header.
Corresponds to struct dyld_cache_header from dyld_cache_format.h.
This header has grown over time. Only the fields up to and including dyld_base_address
are guaranteed to be present. For all other fields, check the header size before
accessing the field. The header size is stored in mapping_offset; the mappings start
right after the theader.

#### Fields

- **`magic`**: `[u8; 16]`

  e.g. "dyld_v0    i386"

- **`mapping_offset`**: `crate::endian::U32<E>`

  file offset to first dyld_cache_mapping_info

- **`mapping_count`**: `crate::endian::U32<E>`

  number of dyld_cache_mapping_info entries

- **`images_offset_old`**: `crate::endian::U32<E>`

  UNUSED: moved to imagesOffset to prevent older dsc_extarctors from crashing

- **`images_count_old`**: `crate::endian::U32<E>`

  UNUSED: moved to imagesCount to prevent older dsc_extarctors from crashing

- **`dyld_base_address`**: `crate::endian::U64<E>`

  base address of dyld when cache was built

- **`code_signature_offset`**: `crate::endian::U64<E>`

  file offset of code signature blob

- **`code_signature_size`**: `crate::endian::U64<E>`

  size of code signature blob (zero means to end of file)

- **`slide_info_offset_unused`**: `crate::endian::U64<E>`

  unused.  Used to be file offset of kernel slid info

- **`slide_info_size_unused`**: `crate::endian::U64<E>`

  unused.  Used to be size of kernel slid info

- **`local_symbols_offset`**: `crate::endian::U64<E>`

  file offset of where local symbols are stored

- **`local_symbols_size`**: `crate::endian::U64<E>`

  size of local symbols information

- **`uuid`**: `[u8; 16]`

  unique value for each shared cache file

- **`cache_type`**: `crate::endian::U64<E>`

  0 for development, 1 for production, 2 for multi-cache

- **`branch_pools_offset`**: `crate::endian::U32<E>`

  file offset to table of uint64_t pool addresses

- **`branch_pools_count`**: `crate::endian::U32<E>`

  number of uint64_t entries

- **`dyld_in_cache_mh`**: `crate::endian::U64<E>`

  (unslid) address of mach_header of dyld in cache

- **`dyld_in_cache_entry`**: `crate::endian::U64<E>`

  (unslid) address of entry point (_dyld_start) of dyld in cache

- **`images_text_offset`**: `crate::endian::U64<E>`

  file offset to first dyld_cache_image_text_info

- **`images_text_count`**: `crate::endian::U64<E>`

  number of dyld_cache_image_text_info entries

- **`patch_info_addr`**: `crate::endian::U64<E>`

  (unslid) address of dyld_cache_patch_info

- **`patch_info_size`**: `crate::endian::U64<E>`

  Size of all of the patch information pointed to via the dyld_cache_patch_info

- **`other_image_group_addr_unused`**: `crate::endian::U64<E>`

  unused

- **`other_image_group_size_unused`**: `crate::endian::U64<E>`

  unused

- **`prog_closures_addr`**: `crate::endian::U64<E>`

  (unslid) address of list of program launch closures

- **`prog_closures_size`**: `crate::endian::U64<E>`

  size of list of program launch closures

- **`prog_closures_trie_addr`**: `crate::endian::U64<E>`

  (unslid) address of trie of indexes into program launch closures

- **`prog_closures_trie_size`**: `crate::endian::U64<E>`

  size of trie of indexes into program launch closures

- **`platform`**: `crate::endian::U32<E>`

  platform number (macOS=1, etc)

- **`shared_region_start`**: `crate::endian::U64<E>`

  base load address of cache if not slid

- **`shared_region_size`**: `crate::endian::U64<E>`

  overall size required to map the cache and all subCaches, if any

- **`max_slide`**: `crate::endian::U64<E>`

  runtime slide of cache can be between zero and this value

- **`dylibs_image_array_addr`**: `crate::endian::U64<E>`

  (unslid) address of ImageArray for dylibs in this cache

- **`dylibs_image_array_size`**: `crate::endian::U64<E>`

  size of ImageArray for dylibs in this cache

- **`dylibs_trie_addr`**: `crate::endian::U64<E>`

  (unslid) address of trie of indexes of all cached dylibs

- **`dylibs_trie_size`**: `crate::endian::U64<E>`

  size of trie of cached dylib paths

- **`other_image_array_addr`**: `crate::endian::U64<E>`

  (unslid) address of ImageArray for dylibs and bundles with dlopen closures

- **`other_image_array_size`**: `crate::endian::U64<E>`

  size of ImageArray for dylibs and bundles with dlopen closures

- **`other_trie_addr`**: `crate::endian::U64<E>`

  (unslid) address of trie of indexes of all dylibs and bundles with dlopen closures

- **`other_trie_size`**: `crate::endian::U64<E>`

  size of trie of dylibs and bundles with dlopen closures

- **`mapping_with_slide_offset`**: `crate::endian::U32<E>`

  file offset to first dyld_cache_mapping_and_slide_info

- **`mapping_with_slide_count`**: `crate::endian::U32<E>`

  number of dyld_cache_mapping_and_slide_info entries

- **`dylibs_pbl_state_array_addr_unused`**: `crate::endian::U64<E>`

  unused

- **`dylibs_pbl_set_addr`**: `crate::endian::U64<E>`

  (unslid) address of PrebuiltLoaderSet of all cached dylibs

- **`programs_pbl_set_pool_addr`**: `crate::endian::U64<E>`

  (unslid) address of pool of PrebuiltLoaderSet for each program

- **`programs_pbl_set_pool_size`**: `crate::endian::U64<E>`

  size of pool of PrebuiltLoaderSet for each program

- **`program_trie_addr`**: `crate::endian::U64<E>`

  (unslid) address of trie mapping program path to PrebuiltLoaderSet

- **`os_version`**: `crate::endian::U32<E>`

  OS Version of dylibs in this cache for the main platform

- **`alt_platform`**: `crate::endian::U32<E>`

  e.g. iOSMac on macOS

- **`alt_os_version`**: `crate::endian::U32<E>`

  e.g. 14.0 for iOSMac

- **`swift_opts_offset`**: `crate::endian::U64<E>`

  VM offset from cache_header* to Swift optimizations header

- **`swift_opts_size`**: `crate::endian::U64<E>`

  size of Swift optimizations header

- **`sub_cache_array_offset`**: `crate::endian::U32<E>`

  file offset to first dyld_subcache_entry

- **`sub_cache_array_count`**: `crate::endian::U32<E>`

  number of subCache entries

- **`symbol_file_uuid`**: `[u8; 16]`

  unique value for the shared cache file containing unmapped local symbols

- **`rosetta_read_only_addr`**: `crate::endian::U64<E>`

  (unslid) address of the start of where Rosetta can add read-only/executable data

- **`rosetta_read_only_size`**: `crate::endian::U64<E>`

  maximum size of the Rosetta read-only/executable region

- **`rosetta_read_write_addr`**: `crate::endian::U64<E>`

  (unslid) address of the start of where Rosetta can add read-write data

- **`rosetta_read_write_size`**: `crate::endian::U64<E>`

  maximum size of the Rosetta read-write region

- **`images_offset`**: `crate::endian::U32<E>`

  file offset to first dyld_cache_image_info

- **`images_count`**: `crate::endian::U32<E>`

  number of dyld_cache_image_info entries

- **`cache_sub_type`**: `crate::endian::U32<E>`

  0 for development, 1 for production, when cacheType is multi-cache(2)

- **`objc_opts_offset`**: `crate::endian::U64<E>`

  VM offset from cache_header* to ObjC optimizations header

- **`objc_opts_size`**: `crate::endian::U64<E>`

  size of ObjC optimizations header

- **`cache_atlas_offset`**: `crate::endian::U64<E>`

  VM offset from cache_header* to embedded cache atlas for process introspection

- **`cache_atlas_size`**: `crate::endian::U64<E>`

  size of embedded cache atlas

- **`dynamic_data_offset`**: `crate::endian::U64<E>`

  VM offset from cache_header* to the location of dyld_cache_dynamic_data_header

- **`dynamic_data_max_size`**: `crate::endian::U64<E>`

  maximum size of space reserved from dynamic data

#### Implementations

- <span id="machodyldcacheheader-parse"></span>`fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<&'data Self>` — [`Result`](../index.md#result)

  Read the dyld cache header.

- <span id="machodyldcacheheader-parse-magic"></span>`fn parse_magic(&self) -> Result<(Architecture, E)>` — [`Result`](../index.md#result), [`Architecture`](../index.md#architecture)

  Returns (arch, endian) based on the magic string.

- <span id="machodyldcacheheader-mappings"></span>`fn mappings<'data, R: ReadRef<'data>>(&self, endian: E, data: R) -> Result<DyldCacheMappingSlice<'data, E>>` — [`Result`](../index.md#result), [`DyldCacheMappingSlice`](../read/macho/index.md#dyldcachemappingslice)

  Return the mapping information table.

- <span id="machodyldcacheheader-subcaches"></span>`fn subcaches<'data, R: ReadRef<'data>>(&self, endian: E, data: R) -> Result<Option<DyldSubCacheSlice<'data, E>>>` — [`Result`](../index.md#result), [`DyldSubCacheSlice`](../read/macho/index.md#dyldsubcacheslice)

  Return the information about subcaches, if present.

  

  Returns `None` for dyld caches produced before dyld-940 (macOS 12).

- <span id="machodyldcacheheader-symbols-subcache-uuid"></span>`fn symbols_subcache_uuid(&self, endian: E) -> Option<[u8; 16]>`

  Return the UUID for the .symbols subcache, if present.

- <span id="machodyldcacheheader-images"></span>`fn images<'data, R: ReadRef<'data>>(&self, endian: E, data: R) -> Result<&'data [macho::DyldCacheImageInfo<E>]>` — [`Result`](../index.md#result), [`DyldCacheImageInfo`](#dyldcacheimageinfo)

  Return the image information table.

#### Trait Implementations

##### `impl Any for DyldCacheHeader<E>`

- <span id="dyldcacheheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheHeader<E>`

- <span id="dyldcacheheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheHeader<E>`

- <span id="dyldcacheheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheHeader<E>`

- <span id="dyldcacheheader-clone"></span>`fn clone(&self) -> DyldCacheHeader<E>` — [`DyldCacheHeader`](#dyldcacheheader)

##### `impl CloneToUninit for DyldCacheHeader<E>`

- <span id="dyldcacheheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheHeader<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheHeader<E>`

- <span id="dyldcacheheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheHeader<E>`

- <span id="dyldcacheheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheHeader<E>`

- <span id="dyldcacheheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldCacheHeader<E>`

##### `impl ToOwned for DyldCacheHeader<E>`

- <span id="dyldcacheheader-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcacheheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcacheheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheHeader<E>`

- <span id="dyldcacheheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheHeader<E>`

- <span id="dyldcacheheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldCacheMappingInfo<E: Endian>`

```rust
struct DyldCacheMappingInfo<E: Endian> {
    pub address: crate::endian::U64<E>,
    pub size: crate::endian::U64<E>,
    pub file_offset: crate::endian::U64<E>,
    pub max_prot: crate::endian::U32<E>,
    pub init_prot: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:459-465`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L459-L465)*

Corresponds to struct dyld_cache_mapping_info from dyld_cache_format.h.

#### Trait Implementations

##### `impl Any for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-clone"></span>`fn clone(&self) -> DyldCacheMappingInfo<E>` — [`DyldCacheMappingInfo`](#dyldcachemappinginfo)

##### `impl CloneToUninit for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheMappingInfo<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldCacheMappingInfo<E>`

##### `impl ToOwned for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcachemappinginfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcachemappinginfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcachemappinginfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheMappingInfo<E>`

- <span id="dyldcachemappinginfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcachemappinginfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldCacheMappingAndSlideInfo<E: Endian>`

```rust
struct DyldCacheMappingAndSlideInfo<E: Endian> {
    pub address: crate::endian::U64<E>,
    pub size: crate::endian::U64<E>,
    pub file_offset: crate::endian::U64<E>,
    pub slide_info_file_offset: crate::endian::U64<E>,
    pub slide_info_file_size: crate::endian::U64<E>,
    pub flags: crate::endian::U64<E>,
    pub max_prot: crate::endian::U32<E>,
    pub init_prot: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:477-486`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L477-L486)*

Corresponds to struct dyld_cache_mapping_and_slide_info from dyld_cache_format.h.

#### Implementations

- <span id="machodyldcachemappingandslideinfo-slide"></span>`fn slide<'data, R: ReadRef<'data>>(&self, endian: E, data: R) -> Result<DyldCacheSlideInfo<'data, E>>` — [`Result`](../index.md#result), [`DyldCacheSlideInfo`](../read/macho/index.md#dyldcacheslideinfo)

  Return the (optional) array of slide information structs

#### Trait Implementations

##### `impl Any for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-clone"></span>`fn clone(&self) -> DyldCacheMappingAndSlideInfo<E>` — [`DyldCacheMappingAndSlideInfo`](#dyldcachemappingandslideinfo)

##### `impl CloneToUninit for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheMappingAndSlideInfo<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldCacheMappingAndSlideInfo<E>`

##### `impl ToOwned for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcachemappingandslideinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcachemappingandslideinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcachemappingandslideinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheMappingAndSlideInfo<E>`

- <span id="dyldcachemappingandslideinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcachemappingandslideinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldCacheImageInfo<E: Endian>`

```rust
struct DyldCacheImageInfo<E: Endian> {
    pub address: crate::endian::U64<E>,
    pub mod_time: crate::endian::U64<E>,
    pub inode: crate::endian::U64<E>,
    pub path_file_offset: crate::endian::U32<E>,
    pub pad: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:491-497`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L491-L497)*

Corresponds to struct dyld_cache_image_info from dyld_cache_format.h.

#### Implementations

- <span id="machodyldcacheimageinfo-path"></span>`fn path<'data, R: ReadRef<'data>>(&self, endian: E, data: R) -> Result<&'data [u8]>` — [`Result`](../index.md#result)

  The file system path of this image.

  

  `data` should be the main cache file, not the subcache containing the image.

#### Trait Implementations

##### `impl Any for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-clone"></span>`fn clone(&self) -> DyldCacheImageInfo<E>` — [`DyldCacheImageInfo`](#dyldcacheimageinfo)

##### `impl CloneToUninit for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheImageInfo<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldCacheImageInfo<E>`

##### `impl ToOwned for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcacheimageinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcacheimageinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheimageinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheImageInfo<E>`

- <span id="dyldcacheimageinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheimageinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldCacheSlideInfo2<E: Endian>`

```rust
struct DyldCacheSlideInfo2<E: Endian> {
    pub version: crate::endian::U32<E>,
    pub page_size: crate::endian::U32<E>,
    pub page_starts_offset: crate::endian::U32<E>,
    pub page_starts_count: crate::endian::U32<E>,
    pub page_extras_offset: crate::endian::U32<E>,
    pub page_extras_count: crate::endian::U32<E>,
    pub delta_mask: crate::endian::U64<E>,
    pub value_add: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:502-511`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L502-L511)*

Corresponds to struct dyld_cache_slide_info2 from dyld_cache_format.h.

#### Trait Implementations

##### `impl Any for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-clone"></span>`fn clone(&self) -> DyldCacheSlideInfo2<E>` — [`DyldCacheSlideInfo2`](#dyldcacheslideinfo2)

##### `impl CloneToUninit for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheSlideInfo2<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldCacheSlideInfo2<E>`

##### `impl ToOwned for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcacheslideinfo2-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcacheslideinfo2-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheslideinfo2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheSlideInfo2<E>`

- <span id="dyldcacheslideinfo2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheslideinfo2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldCacheSlideInfo3<E: Endian>`

```rust
struct DyldCacheSlideInfo3<E: Endian> {
    pub version: crate::endian::U32<E>,
    pub page_size: crate::endian::U32<E>,
    pub page_starts_count: crate::endian::U32<E>,
    reserved1: [u8; 4],
    pub auth_value_add: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:524-530`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L524-L530)*

Corresponds to struct dyld_cache_slide_info3 from dyld_cache_format.h.

#### Trait Implementations

##### `impl Any for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-clone"></span>`fn clone(&self) -> DyldCacheSlideInfo3<E>` — [`DyldCacheSlideInfo3`](#dyldcacheslideinfo3)

##### `impl CloneToUninit for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheSlideInfo3<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldCacheSlideInfo3<E>`

##### `impl ToOwned for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcacheslideinfo3-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcacheslideinfo3-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheslideinfo3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheSlideInfo3<E>`

- <span id="dyldcacheslideinfo3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheslideinfo3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldCacheSlidePointer3`

```rust
struct DyldCacheSlidePointer3(u64);
```

*Defined in [`object-0.37.3/src/macho.rs:537`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L537)*

Corresponds to union dyld_cache_slide_pointer3 from dyld_cache_format.h.

#### Implementations

- <span id="dyldcacheslidepointer3-is-auth"></span>`fn is_auth(&self) -> bool`

  Whether the pointer is authenticated.

- <span id="dyldcacheslidepointer3-target"></span>`fn target(&self) -> u64`

  The target of the pointer.

  

  Only valid if `is_auth` is false.

- <span id="dyldcacheslidepointer3-high8"></span>`fn high8(&self) -> u64`

  The high 8 bits of the pointer.

  

  Only valid if `is_auth` is false.

- <span id="dyldcacheslidepointer3-runtime-offset"></span>`fn runtime_offset(&self) -> u64`

  The target of the pointer as an offset from the start of the shared cache.

  

  Only valid if `is_auth` is true.

- <span id="dyldcacheslidepointer3-diversity"></span>`fn diversity(&self) -> u16`

  The diversity value for authentication.

  

  Only valid if `is_auth` is true.

- <span id="dyldcacheslidepointer3-addr-div"></span>`fn addr_div(&self) -> bool`

  Whether to use address diversity for authentication.

  

  Only valid if `is_auth` is true.

- <span id="dyldcacheslidepointer3-key"></span>`fn key(&self) -> u8`

  The key for authentication.

  

  Only valid if `is_auth` is true.

- <span id="dyldcacheslidepointer3-next"></span>`fn next(&self) -> u64`

  The offset to the next slide pointer in 8-byte units.

  

  0 if no next slide pointer.

#### Trait Implementations

##### `impl Any for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-clone"></span>`fn clone(&self) -> DyldCacheSlidePointer3` — [`DyldCacheSlidePointer3`](#dyldcacheslidepointer3)

##### `impl CloneToUninit for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DyldCacheSlidePointer3`

##### `impl Debug for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcacheslidepointer3-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcacheslidepointer3-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheslidepointer3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheSlidePointer3`

- <span id="dyldcacheslidepointer3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheslidepointer3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldCacheSlideInfo5<E: Endian>`

```rust
struct DyldCacheSlideInfo5<E: Endian> {
    pub version: crate::endian::U32<E>,
    pub page_size: crate::endian::U32<E>,
    pub page_starts_count: crate::endian::U32<E>,
    reserved1: [u8; 4],
    pub value_add: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:598-604`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L598-L604)*

Corresponds to struct dyld_cache_slide_info5 from dyld_cache_format.h.

#### Trait Implementations

##### `impl Any for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-clone"></span>`fn clone(&self) -> DyldCacheSlideInfo5<E>` — [`DyldCacheSlideInfo5`](#dyldcacheslideinfo5)

##### `impl CloneToUninit for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheSlideInfo5<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldCacheSlideInfo5<E>`

##### `impl ToOwned for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcacheslideinfo5-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcacheslideinfo5-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheslideinfo5-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheSlideInfo5<E>`

- <span id="dyldcacheslideinfo5-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheslideinfo5-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldCacheSlidePointer5`

```rust
struct DyldCacheSlidePointer5(u64);
```

*Defined in [`object-0.37.3/src/macho.rs:611`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L611)*

Corresponds to struct dyld_cache_slide_pointer5 from dyld_cache_format.h.

#### Implementations

- <span id="dyldcacheslidepointer5-is-auth"></span>`fn is_auth(&self) -> bool`

  Whether the pointer is authenticated.

- <span id="dyldcacheslidepointer5-runtime-offset"></span>`fn runtime_offset(&self) -> u64`

  The target of the pointer as an offset from the start of the shared cache.

- <span id="dyldcacheslidepointer5-high8"></span>`fn high8(&self) -> u64`

  The high 8 bits of the pointer.

  

  Only valid if `is_auth` is false.

- <span id="dyldcacheslidepointer5-diversity"></span>`fn diversity(&self) -> u16`

  The diversity value for authentication.

  

  Only valid if `is_auth` is true.

- <span id="dyldcacheslidepointer5-addr-div"></span>`fn addr_div(&self) -> bool`

  Whether to use address diversity for authentication.

  

  Only valid if `is_auth` is true.

- <span id="dyldcacheslidepointer5-key-is-data"></span>`fn key_is_data(&self) -> bool`

  Whether the key is IA or DA.

  

  Only valid if `is_auth` is true.

- <span id="dyldcacheslidepointer5-next"></span>`fn next(&self) -> u64`

  The offset to the next slide pointer in 8-byte units.

  

  0 if no next slide pointer.

#### Trait Implementations

##### `impl Any for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-clone"></span>`fn clone(&self) -> DyldCacheSlidePointer5` — [`DyldCacheSlidePointer5`](#dyldcacheslidepointer5)

##### `impl CloneToUninit for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DyldCacheSlidePointer5`

##### `impl Debug for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcacheslidepointer5-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcacheslidepointer5-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheslidepointer5-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheSlidePointer5`

- <span id="dyldcacheslidepointer5-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheslidepointer5-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldSubCacheEntryV1<E: Endian>`

```rust
struct DyldSubCacheEntryV1<E: Endian> {
    pub uuid: [u8; 16],
    pub cache_vm_offset: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:665-670`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L665-L670)*

Added in dyld-940, which shipped with macOS 12 / iOS 15.
Originally called `dyld_subcache_entry`, renamed to `dyld_subcache_entry_v1`
in dyld-1042.1.

#### Fields

- **`uuid`**: `[u8; 16]`

  The UUID of this subcache.

- **`cache_vm_offset`**: `crate::endian::U64<E>`

  The offset of this subcache from the main cache base address.

#### Trait Implementations

##### `impl Any for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-clone"></span>`fn clone(&self) -> DyldSubCacheEntryV1<E>` — [`DyldSubCacheEntryV1`](#dyldsubcacheentryv1)

##### `impl CloneToUninit for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldSubCacheEntryV1<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldSubCacheEntryV1<E>`

##### `impl ToOwned for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldsubcacheentryv1-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldsubcacheentryv1-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldsubcacheentryv1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldSubCacheEntryV1<E>`

- <span id="dyldsubcacheentryv1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldsubcacheentryv1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldSubCacheEntryV2<E: Endian>`

```rust
struct DyldSubCacheEntryV2<E: Endian> {
    pub uuid: [u8; 16],
    pub cache_vm_offset: crate::endian::U64<E>,
    pub file_suffix: [u8; 32],
}
```

*Defined in [`object-0.37.3/src/macho.rs:676-683`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L676-L683)*

Added in dyld-1042.1, which shipped with macOS 13 / iOS 16.
Called `dyld_subcache_entry` as of dyld-1042.1.

#### Fields

- **`uuid`**: `[u8; 16]`

  The UUID of this subcache.

- **`cache_vm_offset`**: `crate::endian::U64<E>`

  The offset of this subcache from the main cache base address.

- **`file_suffix`**: `[u8; 32]`

  The file name suffix of the subCache file, e.g. ".25.data" or ".03.development".

#### Trait Implementations

##### `impl Any for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-clone"></span>`fn clone(&self) -> DyldSubCacheEntryV2<E>` — [`DyldSubCacheEntryV2`](#dyldsubcacheentryv2)

##### `impl CloneToUninit for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldSubCacheEntryV2<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldSubCacheEntryV2<E>`

##### `impl ToOwned for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldsubcacheentryv2-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldsubcacheentryv2-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldsubcacheentryv2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldSubCacheEntryV2<E>`

- <span id="dyldsubcacheentryv2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldsubcacheentryv2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FatHeader`

```rust
struct FatHeader {
    pub magic: crate::endian::U32<crate::endian::BigEndian>,
    pub nfat_arch: crate::endian::U32<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:708-713`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L708-L713)*

#### Fields

- **`magic`**: `crate::endian::U32<crate::endian::BigEndian>`

  FAT_MAGIC or FAT_MAGIC_64

- **`nfat_arch`**: `crate::endian::U32<crate::endian::BigEndian>`

  number of structs that follow

#### Trait Implementations

##### `impl Any for FatHeader`

- <span id="fatheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FatHeader`

- <span id="fatheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FatHeader`

- <span id="fatheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FatHeader`

- <span id="fatheader-clone"></span>`fn clone(&self) -> FatHeader` — [`FatHeader`](#fatheader)

##### `impl CloneToUninit for FatHeader`

- <span id="fatheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FatHeader`

##### `impl Debug for FatHeader`

- <span id="fatheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FatHeader`

- <span id="fatheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FatHeader`

- <span id="fatheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pod for FatHeader`

##### `impl ToOwned for FatHeader`

- <span id="fatheader-toowned-type-owned"></span>`type Owned = T`

- <span id="fatheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fatheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FatHeader`

- <span id="fatheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fatheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FatHeader`

- <span id="fatheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fatheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FatArch32`

```rust
struct FatArch32 {
    pub cputype: crate::endian::U32<crate::endian::BigEndian>,
    pub cpusubtype: crate::endian::U32<crate::endian::BigEndian>,
    pub offset: crate::endian::U32<crate::endian::BigEndian>,
    pub size: crate::endian::U32<crate::endian::BigEndian>,
    pub align: crate::endian::U32<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:717-728`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L717-L728)*

#### Fields

- **`cputype`**: `crate::endian::U32<crate::endian::BigEndian>`

  cpu specifier (int)

- **`cpusubtype`**: `crate::endian::U32<crate::endian::BigEndian>`

  machine specifier (int)

- **`offset`**: `crate::endian::U32<crate::endian::BigEndian>`

  file offset to this object file

- **`size`**: `crate::endian::U32<crate::endian::BigEndian>`

  size of this object file

- **`align`**: `crate::endian::U32<crate::endian::BigEndian>`

  alignment as a power of 2

#### Trait Implementations

##### `impl Any for FatArch32`

- <span id="fatarch32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FatArch32`

- <span id="fatarch32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FatArch32`

- <span id="fatarch32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FatArch32`

- <span id="fatarch32-clone"></span>`fn clone(&self) -> FatArch32` — [`FatArch32`](#fatarch32)

##### `impl CloneToUninit for FatArch32`

- <span id="fatarch32-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FatArch32`

##### `impl Debug for FatArch32`

- <span id="fatarch32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FatArch for FatArch32`

- <span id="fatarch32-fatarch-type-word"></span>`type Word = u32`

- <span id="fatarch32-fatarch-const-magic"></span>`const MAGIC: u32`

- <span id="fatarch32-fatarch-cputype"></span>`fn cputype(&self) -> u32`

- <span id="fatarch32-fatarch-cpusubtype"></span>`fn cpusubtype(&self) -> u32`

- <span id="fatarch32-fatarch-offset"></span>`fn offset(&self) -> <Self as >::Word` — [`FatArch`](../read/macho/index.md#fatarch)

- <span id="fatarch32-fatarch-size"></span>`fn size(&self) -> <Self as >::Word` — [`FatArch`](../read/macho/index.md#fatarch)

- <span id="fatarch32-fatarch-align"></span>`fn align(&self) -> u32`

##### `impl<T> From for FatArch32`

- <span id="fatarch32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FatArch32`

- <span id="fatarch32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pod for FatArch32`

##### `impl ToOwned for FatArch32`

- <span id="fatarch32-toowned-type-owned"></span>`type Owned = T`

- <span id="fatarch32-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fatarch32-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FatArch32`

- <span id="fatarch32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fatarch32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FatArch32`

- <span id="fatarch32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fatarch32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FatArch64`

```rust
struct FatArch64 {
    pub cputype: crate::endian::U32<crate::endian::BigEndian>,
    pub cpusubtype: crate::endian::U32<crate::endian::BigEndian>,
    pub offset: crate::endian::U64<crate::endian::BigEndian>,
    pub size: crate::endian::U64<crate::endian::BigEndian>,
    pub align: crate::endian::U32<crate::endian::BigEndian>,
    pub reserved: crate::endian::U32<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:743-756`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L743-L756)*

#### Fields

- **`cputype`**: `crate::endian::U32<crate::endian::BigEndian>`

  cpu specifier (int)

- **`cpusubtype`**: `crate::endian::U32<crate::endian::BigEndian>`

  machine specifier (int)

- **`offset`**: `crate::endian::U64<crate::endian::BigEndian>`

  file offset to this object file

- **`size`**: `crate::endian::U64<crate::endian::BigEndian>`

  size of this object file

- **`align`**: `crate::endian::U32<crate::endian::BigEndian>`

  alignment as a power of 2

- **`reserved`**: `crate::endian::U32<crate::endian::BigEndian>`

  reserved

#### Trait Implementations

##### `impl Any for FatArch64`

- <span id="fatarch64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FatArch64`

- <span id="fatarch64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FatArch64`

- <span id="fatarch64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FatArch64`

- <span id="fatarch64-clone"></span>`fn clone(&self) -> FatArch64` — [`FatArch64`](#fatarch64)

##### `impl CloneToUninit for FatArch64`

- <span id="fatarch64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FatArch64`

##### `impl Debug for FatArch64`

- <span id="fatarch64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FatArch for FatArch64`

- <span id="fatarch64-fatarch-type-word"></span>`type Word = u64`

- <span id="fatarch64-fatarch-const-magic"></span>`const MAGIC: u32`

- <span id="fatarch64-fatarch-cputype"></span>`fn cputype(&self) -> u32`

- <span id="fatarch64-fatarch-cpusubtype"></span>`fn cpusubtype(&self) -> u32`

- <span id="fatarch64-fatarch-offset"></span>`fn offset(&self) -> <Self as >::Word` — [`FatArch`](../read/macho/index.md#fatarch)

- <span id="fatarch64-fatarch-size"></span>`fn size(&self) -> <Self as >::Word` — [`FatArch`](../read/macho/index.md#fatarch)

- <span id="fatarch64-fatarch-align"></span>`fn align(&self) -> u32`

##### `impl<T> From for FatArch64`

- <span id="fatarch64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FatArch64`

- <span id="fatarch64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pod for FatArch64`

##### `impl ToOwned for FatArch64`

- <span id="fatarch64-toowned-type-owned"></span>`type Owned = T`

- <span id="fatarch64-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fatarch64-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FatArch64`

- <span id="fatarch64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fatarch64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FatArch64`

- <span id="fatarch64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fatarch64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachHeader32<E: Endian>`

```rust
struct MachHeader32<E: Endian> {
    pub magic: crate::endian::U32<crate::endian::BigEndian>,
    pub cputype: crate::endian::U32<E>,
    pub cpusubtype: crate::endian::U32<E>,
    pub filetype: crate::endian::U32<E>,
    pub ncmds: crate::endian::U32<E>,
    pub sizeofcmds: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:765-780`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L765-L780)*

The 32-bit mach header.

Appears at the very beginning of the object file for 32-bit architectures.

#### Fields

- **`magic`**: `crate::endian::U32<crate::endian::BigEndian>`

  mach magic number identifier

- **`cputype`**: `crate::endian::U32<E>`

  cpu specifier

- **`cpusubtype`**: `crate::endian::U32<E>`

  machine specifier

- **`filetype`**: `crate::endian::U32<E>`

  type of file

- **`ncmds`**: `crate::endian::U32<E>`

  number of load commands

- **`sizeofcmds`**: `crate::endian::U32<E>`

  the size of all the load commands

- **`flags`**: `crate::endian::U32<E>`

  flags

#### Trait Implementations

##### `impl Any for MachHeader32<E>`

- <span id="machheader32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachHeader32<E>`

- <span id="machheader32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachHeader32<E>`

- <span id="machheader32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for MachHeader32<E>`

- <span id="machheader32-clone"></span>`fn clone(&self) -> MachHeader32<E>` — [`MachHeader32`](#machheader32)

##### `impl CloneToUninit for MachHeader32<E>`

- <span id="machheader32-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for MachHeader32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for MachHeader32<E>`

- <span id="machheader32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachHeader32<E>`

- <span id="machheader32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachHeader32<E>`

- <span id="machheader32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Endian: endian::Endian> MachHeader for macho::MachHeader32<Endian>`

- <span id="machomachheader32-machheader-type-word"></span>`type Word = u32`

- <span id="machomachheader32-machheader-type-endian"></span>`type Endian = Endian`

- <span id="machomachheader32-machheader-type-segment"></span>`type Segment = SegmentCommand32<Endian>`

- <span id="machomachheader32-machheader-type-section"></span>`type Section = Section32<Endian>`

- <span id="machomachheader32-machheader-type-nlist"></span>`type Nlist = Nlist32<Endian>`

- <span id="machomachheader32-machheader-is-type-64"></span>`fn is_type_64(&self) -> bool`

- <span id="machomachheader32-machheader-is-big-endian"></span>`fn is_big_endian(&self) -> bool`

- <span id="machomachheader32-machheader-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="machomachheader32-machheader-magic"></span>`fn magic(&self) -> u32`

- <span id="machomachheader32-machheader-cputype"></span>`fn cputype(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader32-machheader-cpusubtype"></span>`fn cpusubtype(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader32-machheader-filetype"></span>`fn filetype(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader32-machheader-ncmds"></span>`fn ncmds(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader32-machheader-sizeofcmds"></span>`fn sizeofcmds(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader32-machheader-flags"></span>`fn flags(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

##### `impl<E: Endian> Pod for MachHeader32<E>`

##### `impl ToOwned for MachHeader32<E>`

- <span id="machheader32-toowned-type-owned"></span>`type Owned = T`

- <span id="machheader32-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="machheader32-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MachHeader32<E>`

- <span id="machheader32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machheader32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachHeader32<E>`

- <span id="machheader32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machheader32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachHeader64<E: Endian>`

```rust
struct MachHeader64<E: Endian> {
    pub magic: crate::endian::U32<crate::endian::BigEndian>,
    pub cputype: crate::endian::U32<E>,
    pub cpusubtype: crate::endian::U32<E>,
    pub filetype: crate::endian::U32<E>,
    pub ncmds: crate::endian::U32<E>,
    pub sizeofcmds: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
    pub reserved: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:793-810`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L793-L810)*

The 64-bit mach header.

Appears at the very beginning of object files for 64-bit architectures.

#### Fields

- **`magic`**: `crate::endian::U32<crate::endian::BigEndian>`

  mach magic number identifier

- **`cputype`**: `crate::endian::U32<E>`

  cpu specifier

- **`cpusubtype`**: `crate::endian::U32<E>`

  machine specifier

- **`filetype`**: `crate::endian::U32<E>`

  type of file

- **`ncmds`**: `crate::endian::U32<E>`

  number of load commands

- **`sizeofcmds`**: `crate::endian::U32<E>`

  the size of all the load commands

- **`flags`**: `crate::endian::U32<E>`

  flags

- **`reserved`**: `crate::endian::U32<E>`

  reserved

#### Trait Implementations

##### `impl Any for MachHeader64<E>`

- <span id="machheader64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachHeader64<E>`

- <span id="machheader64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachHeader64<E>`

- <span id="machheader64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for MachHeader64<E>`

- <span id="machheader64-clone"></span>`fn clone(&self) -> MachHeader64<E>` — [`MachHeader64`](#machheader64)

##### `impl CloneToUninit for MachHeader64<E>`

- <span id="machheader64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for MachHeader64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for MachHeader64<E>`

- <span id="machheader64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachHeader64<E>`

- <span id="machheader64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachHeader64<E>`

- <span id="machheader64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Endian: endian::Endian> MachHeader for macho::MachHeader64<Endian>`

- <span id="machomachheader64-machheader-type-word"></span>`type Word = u64`

- <span id="machomachheader64-machheader-type-endian"></span>`type Endian = Endian`

- <span id="machomachheader64-machheader-type-segment"></span>`type Segment = SegmentCommand64<Endian>`

- <span id="machomachheader64-machheader-type-section"></span>`type Section = Section64<Endian>`

- <span id="machomachheader64-machheader-type-nlist"></span>`type Nlist = Nlist64<Endian>`

- <span id="machomachheader64-machheader-is-type-64"></span>`fn is_type_64(&self) -> bool`

- <span id="machomachheader64-machheader-is-big-endian"></span>`fn is_big_endian(&self) -> bool`

- <span id="machomachheader64-machheader-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="machomachheader64-machheader-magic"></span>`fn magic(&self) -> u32`

- <span id="machomachheader64-machheader-cputype"></span>`fn cputype(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader64-machheader-cpusubtype"></span>`fn cpusubtype(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader64-machheader-filetype"></span>`fn filetype(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader64-machheader-ncmds"></span>`fn ncmds(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader64-machheader-sizeofcmds"></span>`fn sizeofcmds(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

- <span id="machomachheader64-machheader-flags"></span>`fn flags(&self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md#machheader)

##### `impl<E: Endian> Pod for MachHeader64<E>`

##### `impl ToOwned for MachHeader64<E>`

- <span id="machheader64-toowned-type-owned"></span>`type Owned = T`

- <span id="machheader64-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="machheader64-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MachHeader64<E>`

- <span id="machheader64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machheader64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachHeader64<E>`

- <span id="machheader64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machheader64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LoadCommand<E: Endian>`

```rust
struct LoadCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:947-954`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L947-L954)*

Common fields at the start of every load command.

The load commands directly follow the mach_header.  The total size of all
of the commands is given by the sizeofcmds field in the mach_header.  All
load commands must have as their first two fields `cmd` and `cmdsize`.  The `cmd`
field is filled in with a constant for that command type.  Each command type
has a structure specifically for it.  The `cmdsize` field is the size in bytes
of the particular load command structure plus anything that follows it that
is a part of the load command (i.e. section structures, strings, etc.).  To
advance to the next load command the `cmdsize` can be added to the offset or
pointer of the current load command.  The `cmdsize` for 32-bit architectures
MUST be a multiple of 4 bytes and for 64-bit architectures MUST be a multiple
of 8 bytes (these are forever the maximum alignment of any load commands).
The padded bytes must be zero.  All tables in the object file must also
follow these rules so the file can be memory mapped.  Otherwise the pointers
to these tables will not work well or at all on some machines.  With all
padding zeroed like objects will compare byte for byte.

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  Type of load command.
  
  One of the `LC_*` constants.

- **`cmdsize`**: `crate::endian::U32<E>`

  Total size of command in bytes.

#### Trait Implementations

##### `impl Any for LoadCommand<E>`

- <span id="loadcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LoadCommand<E>`

- <span id="loadcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LoadCommand<E>`

- <span id="loadcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for LoadCommand<E>`

- <span id="loadcommand-clone"></span>`fn clone(&self) -> LoadCommand<E>` — [`LoadCommand`](#loadcommand)

##### `impl CloneToUninit for LoadCommand<E>`

- <span id="loadcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for LoadCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for LoadCommand<E>`

- <span id="loadcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LoadCommand<E>`

- <span id="loadcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LoadCommand<E>`

- <span id="loadcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for LoadCommand<E>`

##### `impl ToOwned for LoadCommand<E>`

- <span id="loadcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="loadcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="loadcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LoadCommand<E>`

- <span id="loadcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loadcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LoadCommand<E>`

- <span id="loadcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loadcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LcStr<E: Endian>`

```rust
struct LcStr<E: Endian> {
    pub offset: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1087-1090`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1087-L1090)*

A variable length string in a load command.

The strings are stored just after the load command structure and
the offset is from the start of the load command structure.  The size
of the string is reflected in the `cmdsize` field of the load command.
Once again any padded bytes to bring the `cmdsize` field to a multiple
of 4 bytes must be zero.

#### Fields

- **`offset`**: `crate::endian::U32<E>`

  offset to the string

#### Trait Implementations

##### `impl Any for LcStr<E>`

- <span id="lcstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LcStr<E>`

- <span id="lcstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LcStr<E>`

- <span id="lcstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for LcStr<E>`

- <span id="lcstr-clone"></span>`fn clone(&self) -> LcStr<E>` — [`LcStr`](#lcstr)

##### `impl CloneToUninit for LcStr<E>`

- <span id="lcstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for LcStr<E>`

##### `impl<E: fmt::Debug + Endian> Debug for LcStr<E>`

- <span id="lcstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LcStr<E>`

- <span id="lcstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LcStr<E>`

- <span id="lcstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for LcStr<E>`

##### `impl ToOwned for LcStr<E>`

- <span id="lcstr-toowned-type-owned"></span>`type Owned = T`

- <span id="lcstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lcstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LcStr<E>`

- <span id="lcstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lcstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LcStr<E>`

- <span id="lcstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lcstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentCommand32<E: Endian>`

```rust
struct SegmentCommand32<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub segname: [u8; 16],
    pub vmaddr: crate::endian::U32<E>,
    pub vmsize: crate::endian::U32<E>,
    pub fileoff: crate::endian::U32<E>,
    pub filesize: crate::endian::U32<E>,
    pub maxprot: crate::endian::U32<E>,
    pub initprot: crate::endian::U32<E>,
    pub nsects: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1106-1129`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1106-L1129)*

32-bit segment load command.

The segment load command indicates that a part of this file is to be
mapped into the task's address space.  The size of this segment in memory,
vmsize, maybe equal to or larger than the amount to map from this file,
filesize.  The file is mapped starting at fileoff to the beginning of
the segment in memory, vmaddr.  The rest of the memory of the segment,
if any, is allocated zero fill on demand.  The segment's maximum virtual
memory protection and initial virtual memory protection are specified
by the maxprot and initprot fields.  If the segment has sections then the
`Section32` structures directly follow the segment command and their size is
reflected in `cmdsize`.

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SEGMENT

- **`cmdsize`**: `crate::endian::U32<E>`

  includes sizeof section structs

- **`segname`**: `[u8; 16]`

  segment name

- **`vmaddr`**: `crate::endian::U32<E>`

  memory address of this segment

- **`vmsize`**: `crate::endian::U32<E>`

  memory size of this segment

- **`fileoff`**: `crate::endian::U32<E>`

  file offset of this segment

- **`filesize`**: `crate::endian::U32<E>`

  amount to map from the file

- **`maxprot`**: `crate::endian::U32<E>`

  maximum VM protection

- **`initprot`**: `crate::endian::U32<E>`

  initial VM protection

- **`nsects`**: `crate::endian::U32<E>`

  number of sections in segment

- **`flags`**: `crate::endian::U32<E>`

  flags

#### Trait Implementations

##### `impl Any for SegmentCommand32<E>`

- <span id="segmentcommand32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentCommand32<E>`

- <span id="segmentcommand32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentCommand32<E>`

- <span id="segmentcommand32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SegmentCommand32<E>`

- <span id="segmentcommand32-clone"></span>`fn clone(&self) -> SegmentCommand32<E>` — [`SegmentCommand32`](#segmentcommand32)

##### `impl CloneToUninit for SegmentCommand32<E>`

- <span id="segmentcommand32-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SegmentCommand32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SegmentCommand32<E>`

- <span id="segmentcommand32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SegmentCommand32<E>`

- <span id="segmentcommand32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SegmentCommand32<E>`

- <span id="segmentcommand32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SegmentCommand32<E>`

##### `impl<Endian: endian::Endian> Segment for macho::SegmentCommand32<Endian>`

- <span id="machosegmentcommand32-segment-type-word"></span>`type Word = u32`

- <span id="machosegmentcommand32-segment-type-endian"></span>`type Endian = Endian`

- <span id="machosegmentcommand32-segment-type-section"></span>`type Section = Section32<<SegmentCommand32<Endian> as Segment>::Endian>`

- <span id="machosegmentcommand32-segment-from-command"></span>`fn from_command(command: LoadCommandData<'_, <Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>` — [`LoadCommandData`](../read/macho/index.md#loadcommanddata), [`Segment`](../read/macho/index.md#segment), [`Result`](../index.md#result)

- <span id="machosegmentcommand32-segment-cmd"></span>`fn cmd(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-cmdsize"></span>`fn cmdsize(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-segname"></span>`fn segname(&self) -> &[u8; 16]`

- <span id="machosegmentcommand32-segment-vmaddr"></span>`fn vmaddr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-vmsize"></span>`fn vmsize(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-fileoff"></span>`fn fileoff(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-filesize"></span>`fn filesize(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-maxprot"></span>`fn maxprot(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-initprot"></span>`fn initprot(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-nsects"></span>`fn nsects(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand32-segment-flags"></span>`fn flags(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

##### `impl ToOwned for SegmentCommand32<E>`

- <span id="segmentcommand32-toowned-type-owned"></span>`type Owned = T`

- <span id="segmentcommand32-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="segmentcommand32-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SegmentCommand32<E>`

- <span id="segmentcommand32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentcommand32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentCommand32<E>`

- <span id="segmentcommand32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentcommand32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentCommand64<E: Endian>`

```rust
struct SegmentCommand64<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub segname: [u8; 16],
    pub vmaddr: crate::endian::U64<E>,
    pub vmsize: crate::endian::U64<E>,
    pub fileoff: crate::endian::U64<E>,
    pub filesize: crate::endian::U64<E>,
    pub maxprot: crate::endian::U32<E>,
    pub initprot: crate::endian::U32<E>,
    pub nsects: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1139-1162`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1139-L1162)*

64-bit segment load command.

The 64-bit segment load command indicates that a part of this file is to be
mapped into a 64-bit task's address space.  If the 64-bit segment has
sections then `Section64` structures directly follow the 64-bit segment
command and their size is reflected in `cmdsize`.

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SEGMENT_64

- **`cmdsize`**: `crate::endian::U32<E>`

  includes sizeof section_64 structs

- **`segname`**: `[u8; 16]`

  segment name

- **`vmaddr`**: `crate::endian::U64<E>`

  memory address of this segment

- **`vmsize`**: `crate::endian::U64<E>`

  memory size of this segment

- **`fileoff`**: `crate::endian::U64<E>`

  file offset of this segment

- **`filesize`**: `crate::endian::U64<E>`

  amount to map from the file

- **`maxprot`**: `crate::endian::U32<E>`

  maximum VM protection

- **`initprot`**: `crate::endian::U32<E>`

  initial VM protection

- **`nsects`**: `crate::endian::U32<E>`

  number of sections in segment

- **`flags`**: `crate::endian::U32<E>`

  flags

#### Trait Implementations

##### `impl Any for SegmentCommand64<E>`

- <span id="segmentcommand64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentCommand64<E>`

- <span id="segmentcommand64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentCommand64<E>`

- <span id="segmentcommand64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SegmentCommand64<E>`

- <span id="segmentcommand64-clone"></span>`fn clone(&self) -> SegmentCommand64<E>` — [`SegmentCommand64`](#segmentcommand64)

##### `impl CloneToUninit for SegmentCommand64<E>`

- <span id="segmentcommand64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SegmentCommand64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SegmentCommand64<E>`

- <span id="segmentcommand64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SegmentCommand64<E>`

- <span id="segmentcommand64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SegmentCommand64<E>`

- <span id="segmentcommand64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SegmentCommand64<E>`

##### `impl<Endian: endian::Endian> Segment for macho::SegmentCommand64<Endian>`

- <span id="machosegmentcommand64-segment-type-word"></span>`type Word = u64`

- <span id="machosegmentcommand64-segment-type-endian"></span>`type Endian = Endian`

- <span id="machosegmentcommand64-segment-type-section"></span>`type Section = Section64<<SegmentCommand64<Endian> as Segment>::Endian>`

- <span id="machosegmentcommand64-segment-from-command"></span>`fn from_command(command: LoadCommandData<'_, <Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>` — [`LoadCommandData`](../read/macho/index.md#loadcommanddata), [`Segment`](../read/macho/index.md#segment), [`Result`](../index.md#result)

- <span id="machosegmentcommand64-segment-cmd"></span>`fn cmd(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-cmdsize"></span>`fn cmdsize(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-segname"></span>`fn segname(&self) -> &[u8; 16]`

- <span id="machosegmentcommand64-segment-vmaddr"></span>`fn vmaddr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-vmsize"></span>`fn vmsize(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-fileoff"></span>`fn fileoff(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-filesize"></span>`fn filesize(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-maxprot"></span>`fn maxprot(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-initprot"></span>`fn initprot(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-nsects"></span>`fn nsects(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

- <span id="machosegmentcommand64-segment-flags"></span>`fn flags(&self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md#segment)

##### `impl ToOwned for SegmentCommand64<E>`

- <span id="segmentcommand64-toowned-type-owned"></span>`type Owned = T`

- <span id="segmentcommand64-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="segmentcommand64-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SegmentCommand64<E>`

- <span id="segmentcommand64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentcommand64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentCommand64<E>`

- <span id="segmentcommand64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentcommand64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Section32<E: Endian>`

```rust
struct Section32<E: Endian> {
    pub sectname: [u8; 16],
    pub segname: [u8; 16],
    pub addr: crate::endian::U32<E>,
    pub size: crate::endian::U32<E>,
    pub offset: crate::endian::U32<E>,
    pub align: crate::endian::U32<E>,
    pub reloff: crate::endian::U32<E>,
    pub nreloc: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
    pub reserved1: crate::endian::U32<E>,
    pub reserved2: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1206-1229`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1206-L1229)*

32-bit section.

#### Fields

- **`sectname`**: `[u8; 16]`

  name of this section

- **`segname`**: `[u8; 16]`

  segment this section goes in

- **`addr`**: `crate::endian::U32<E>`

  memory address of this section

- **`size`**: `crate::endian::U32<E>`

  size in bytes of this section

- **`offset`**: `crate::endian::U32<E>`

  file offset of this section

- **`align`**: `crate::endian::U32<E>`

  section alignment (power of 2)

- **`reloff`**: `crate::endian::U32<E>`

  file offset of relocation entries

- **`nreloc`**: `crate::endian::U32<E>`

  number of relocation entries

- **`flags`**: `crate::endian::U32<E>`

  flags (section type and attributes)

- **`reserved1`**: `crate::endian::U32<E>`

  reserved (for offset or index)

- **`reserved2`**: `crate::endian::U32<E>`

  reserved (for count or sizeof)

#### Trait Implementations

##### `impl Any for Section32<E>`

- <span id="section32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Section32<E>`

- <span id="section32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Section32<E>`

- <span id="section32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for Section32<E>`

- <span id="section32-clone"></span>`fn clone(&self) -> Section32<E>` — [`Section32`](#section32)

##### `impl CloneToUninit for Section32<E>`

- <span id="section32-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for Section32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Section32<E>`

- <span id="section32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Section32<E>`

- <span id="section32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Section32<E>`

- <span id="section32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for Section32<E>`

##### `impl<Endian: endian::Endian> Section for macho::Section32<Endian>`

- <span id="machosection32-section-type-word"></span>`type Word = u32`

- <span id="machosection32-section-type-endian"></span>`type Endian = Endian`

- <span id="machosection32-section-sectname"></span>`fn sectname(&self) -> &[u8; 16]`

- <span id="machosection32-section-segname"></span>`fn segname(&self) -> &[u8; 16]`

- <span id="machosection32-section-addr"></span>`fn addr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Section`](../read/macho/index.md#section)

- <span id="machosection32-section-size"></span>`fn size(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Section`](../read/macho/index.md#section)

- <span id="machosection32-section-offset"></span>`fn offset(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

- <span id="machosection32-section-align"></span>`fn align(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

- <span id="machosection32-section-reloff"></span>`fn reloff(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

- <span id="machosection32-section-nreloc"></span>`fn nreloc(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

- <span id="machosection32-section-flags"></span>`fn flags(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

##### `impl ToOwned for Section32<E>`

- <span id="section32-toowned-type-owned"></span>`type Owned = T`

- <span id="section32-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="section32-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Section32<E>`

- <span id="section32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="section32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Section32<E>`

- <span id="section32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="section32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Section64<E: Endian>`

```rust
struct Section64<E: Endian> {
    pub sectname: [u8; 16],
    pub segname: [u8; 16],
    pub addr: crate::endian::U64<E>,
    pub size: crate::endian::U64<E>,
    pub offset: crate::endian::U32<E>,
    pub align: crate::endian::U32<E>,
    pub reloff: crate::endian::U32<E>,
    pub nreloc: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
    pub reserved1: crate::endian::U32<E>,
    pub reserved2: crate::endian::U32<E>,
    pub reserved3: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1234-1259`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1234-L1259)*

64-bit section.

#### Fields

- **`sectname`**: `[u8; 16]`

  name of this section

- **`segname`**: `[u8; 16]`

  segment this section goes in

- **`addr`**: `crate::endian::U64<E>`

  memory address of this section

- **`size`**: `crate::endian::U64<E>`

  size in bytes of this section

- **`offset`**: `crate::endian::U32<E>`

  file offset of this section

- **`align`**: `crate::endian::U32<E>`

  section alignment (power of 2)

- **`reloff`**: `crate::endian::U32<E>`

  file offset of relocation entries

- **`nreloc`**: `crate::endian::U32<E>`

  number of relocation entries

- **`flags`**: `crate::endian::U32<E>`

  flags (section type and attributes)

- **`reserved1`**: `crate::endian::U32<E>`

  reserved (for offset or index)

- **`reserved2`**: `crate::endian::U32<E>`

  reserved (for count or sizeof)

- **`reserved3`**: `crate::endian::U32<E>`

  reserved

#### Trait Implementations

##### `impl Any for Section64<E>`

- <span id="section64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Section64<E>`

- <span id="section64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Section64<E>`

- <span id="section64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for Section64<E>`

- <span id="section64-clone"></span>`fn clone(&self) -> Section64<E>` — [`Section64`](#section64)

##### `impl CloneToUninit for Section64<E>`

- <span id="section64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for Section64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Section64<E>`

- <span id="section64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Section64<E>`

- <span id="section64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Section64<E>`

- <span id="section64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for Section64<E>`

##### `impl<Endian: endian::Endian> Section for macho::Section64<Endian>`

- <span id="machosection64-section-type-word"></span>`type Word = u64`

- <span id="machosection64-section-type-endian"></span>`type Endian = Endian`

- <span id="machosection64-section-sectname"></span>`fn sectname(&self) -> &[u8; 16]`

- <span id="machosection64-section-segname"></span>`fn segname(&self) -> &[u8; 16]`

- <span id="machosection64-section-addr"></span>`fn addr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Section`](../read/macho/index.md#section)

- <span id="machosection64-section-size"></span>`fn size(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Section`](../read/macho/index.md#section)

- <span id="machosection64-section-offset"></span>`fn offset(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

- <span id="machosection64-section-align"></span>`fn align(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

- <span id="machosection64-section-reloff"></span>`fn reloff(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

- <span id="machosection64-section-nreloc"></span>`fn nreloc(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

- <span id="machosection64-section-flags"></span>`fn flags(&self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md#section)

##### `impl ToOwned for Section64<E>`

- <span id="section64-toowned-type-owned"></span>`type Owned = T`

- <span id="section64-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="section64-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Section64<E>`

- <span id="section64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="section64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Section64<E>`

- <span id="section64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="section64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fvmlib<E: Endian>`

```rust
struct Fvmlib<E: Endian> {
    pub name: LcStr<E>,
    pub minor_version: crate::endian::U32<E>,
    pub header_addr: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1448-1455`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1448-L1455)*

#### Fields

- **`name`**: `LcStr<E>`

  library's target pathname

- **`minor_version`**: `crate::endian::U32<E>`

  library's minor version number

- **`header_addr`**: `crate::endian::U32<E>`

  library's header address

#### Trait Implementations

##### `impl Any for Fvmlib<E>`

- <span id="fvmlib-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fvmlib<E>`

- <span id="fvmlib-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fvmlib<E>`

- <span id="fvmlib-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for Fvmlib<E>`

- <span id="fvmlib-clone"></span>`fn clone(&self) -> Fvmlib<E>` — [`Fvmlib`](#fvmlib)

##### `impl CloneToUninit for Fvmlib<E>`

- <span id="fvmlib-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for Fvmlib<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Fvmlib<E>`

- <span id="fvmlib-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Fvmlib<E>`

- <span id="fvmlib-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fvmlib<E>`

- <span id="fvmlib-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for Fvmlib<E>`

##### `impl ToOwned for Fvmlib<E>`

- <span id="fvmlib-toowned-type-owned"></span>`type Owned = T`

- <span id="fvmlib-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fvmlib-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Fvmlib<E>`

- <span id="fvmlib-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fvmlib-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fvmlib<E>`

- <span id="fvmlib-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fvmlib-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FvmlibCommand<E: Endian>`

```rust
struct FvmlibCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub fvmlib: Fvmlib<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1466-1473`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1466-L1473)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_IDFVMLIB or LC_LOADFVMLIB

- **`cmdsize`**: `crate::endian::U32<E>`

  includes pathname string

- **`fvmlib`**: `Fvmlib<E>`

  the library identification

#### Trait Implementations

##### `impl Any for FvmlibCommand<E>`

- <span id="fvmlibcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FvmlibCommand<E>`

- <span id="fvmlibcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FvmlibCommand<E>`

- <span id="fvmlibcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for FvmlibCommand<E>`

- <span id="fvmlibcommand-clone"></span>`fn clone(&self) -> FvmlibCommand<E>` — [`FvmlibCommand`](#fvmlibcommand)

##### `impl CloneToUninit for FvmlibCommand<E>`

- <span id="fvmlibcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for FvmlibCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for FvmlibCommand<E>`

- <span id="fvmlibcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FvmlibCommand<E>`

- <span id="fvmlibcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FvmlibCommand<E>`

- <span id="fvmlibcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for FvmlibCommand<E>`

##### `impl ToOwned for FvmlibCommand<E>`

- <span id="fvmlibcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="fvmlibcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fvmlibcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FvmlibCommand<E>`

- <span id="fvmlibcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fvmlibcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FvmlibCommand<E>`

- <span id="fvmlibcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fvmlibcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Dylib<E: Endian>`

```rust
struct Dylib<E: Endian> {
    pub name: LcStr<E>,
    pub timestamp: crate::endian::U32<E>,
    pub current_version: crate::endian::U32<E>,
    pub compatibility_version: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1486-1495`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1486-L1495)*

#### Fields

- **`name`**: `LcStr<E>`

  library's path name

- **`timestamp`**: `crate::endian::U32<E>`

  library's build time stamp

- **`current_version`**: `crate::endian::U32<E>`

  library's current version number

- **`compatibility_version`**: `crate::endian::U32<E>`

  library's compatibility vers number

#### Trait Implementations

##### `impl Any for Dylib<E>`

- <span id="dylib-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dylib<E>`

- <span id="dylib-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dylib<E>`

- <span id="dylib-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for Dylib<E>`

- <span id="dylib-clone"></span>`fn clone(&self) -> Dylib<E>` — [`Dylib`](#dylib)

##### `impl CloneToUninit for Dylib<E>`

- <span id="dylib-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for Dylib<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Dylib<E>`

- <span id="dylib-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Dylib<E>`

- <span id="dylib-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Dylib<E>`

- <span id="dylib-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for Dylib<E>`

##### `impl ToOwned for Dylib<E>`

- <span id="dylib-toowned-type-owned"></span>`type Owned = T`

- <span id="dylib-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dylib-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Dylib<E>`

- <span id="dylib-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dylib-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dylib<E>`

- <span id="dylib-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dylib-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DylibCommand<E: Endian>`

```rust
struct DylibCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub dylib: Dylib<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1506-1513`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1506-L1513)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ID_DYLIB, LC_LOAD_{,WEAK_}DYLIB, LC_REEXPORT_DYLIB

- **`cmdsize`**: `crate::endian::U32<E>`

  includes pathname string

- **`dylib`**: `Dylib<E>`

  the library identification

#### Trait Implementations

##### `impl Any for DylibCommand<E>`

- <span id="dylibcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DylibCommand<E>`

- <span id="dylibcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DylibCommand<E>`

- <span id="dylibcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DylibCommand<E>`

- <span id="dylibcommand-clone"></span>`fn clone(&self) -> DylibCommand<E>` — [`DylibCommand`](#dylibcommand)

##### `impl CloneToUninit for DylibCommand<E>`

- <span id="dylibcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DylibCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DylibCommand<E>`

- <span id="dylibcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DylibCommand<E>`

- <span id="dylibcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DylibCommand<E>`

- <span id="dylibcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DylibCommand<E>`

##### `impl ToOwned for DylibCommand<E>`

- <span id="dylibcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="dylibcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dylibcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DylibCommand<E>`

- <span id="dylibcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dylibcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DylibCommand<E>`

- <span id="dylibcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dylibcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SubFrameworkCommand<E: Endian>`

```rust
struct SubFrameworkCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub umbrella: LcStr<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1527-1534`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1527-L1534)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SUB_FRAMEWORK

- **`cmdsize`**: `crate::endian::U32<E>`

  includes umbrella string

- **`umbrella`**: `LcStr<E>`

  the umbrella framework name

#### Trait Implementations

##### `impl Any for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-clone"></span>`fn clone(&self) -> SubFrameworkCommand<E>` — [`SubFrameworkCommand`](#subframeworkcommand)

##### `impl CloneToUninit for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SubFrameworkCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SubFrameworkCommand<E>`

##### `impl ToOwned for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="subframeworkcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="subframeworkcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="subframeworkcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SubFrameworkCommand<E>`

- <span id="subframeworkcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="subframeworkcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SubClientCommand<E: Endian>`

```rust
struct SubClientCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub client: LcStr<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1547-1554`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1547-L1554)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SUB_CLIENT

- **`cmdsize`**: `crate::endian::U32<E>`

  includes client string

- **`client`**: `LcStr<E>`

  the client name

#### Trait Implementations

##### `impl Any for SubClientCommand<E>`

- <span id="subclientcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SubClientCommand<E>`

- <span id="subclientcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SubClientCommand<E>`

- <span id="subclientcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SubClientCommand<E>`

- <span id="subclientcommand-clone"></span>`fn clone(&self) -> SubClientCommand<E>` — [`SubClientCommand`](#subclientcommand)

##### `impl CloneToUninit for SubClientCommand<E>`

- <span id="subclientcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SubClientCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SubClientCommand<E>`

- <span id="subclientcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SubClientCommand<E>`

- <span id="subclientcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SubClientCommand<E>`

- <span id="subclientcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SubClientCommand<E>`

##### `impl ToOwned for SubClientCommand<E>`

- <span id="subclientcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="subclientcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="subclientcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SubClientCommand<E>`

- <span id="subclientcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="subclientcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SubClientCommand<E>`

- <span id="subclientcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="subclientcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SubUmbrellaCommand<E: Endian>`

```rust
struct SubUmbrellaCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub sub_umbrella: LcStr<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1571-1578`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1571-L1578)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SUB_UMBRELLA

- **`cmdsize`**: `crate::endian::U32<E>`

  includes sub_umbrella string

- **`sub_umbrella`**: `LcStr<E>`

  the sub_umbrella framework name

#### Trait Implementations

##### `impl Any for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-clone"></span>`fn clone(&self) -> SubUmbrellaCommand<E>` — [`SubUmbrellaCommand`](#subumbrellacommand)

##### `impl CloneToUninit for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SubUmbrellaCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SubUmbrellaCommand<E>`

##### `impl ToOwned for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-toowned-type-owned"></span>`type Owned = T`

- <span id="subumbrellacommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="subumbrellacommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="subumbrellacommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SubUmbrellaCommand<E>`

- <span id="subumbrellacommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="subumbrellacommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SubLibraryCommand<E: Endian>`

```rust
struct SubLibraryCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub sub_library: LcStr<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1597-1604`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1597-L1604)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SUB_LIBRARY

- **`cmdsize`**: `crate::endian::U32<E>`

  includes sub_library string

- **`sub_library`**: `LcStr<E>`

  the sub_library name

#### Trait Implementations

##### `impl Any for SubLibraryCommand<E>`

- <span id="sublibrarycommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SubLibraryCommand<E>`

- <span id="sublibrarycommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SubLibraryCommand<E>`

- <span id="sublibrarycommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SubLibraryCommand<E>`

- <span id="sublibrarycommand-clone"></span>`fn clone(&self) -> SubLibraryCommand<E>` — [`SubLibraryCommand`](#sublibrarycommand)

##### `impl CloneToUninit for SubLibraryCommand<E>`

- <span id="sublibrarycommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SubLibraryCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SubLibraryCommand<E>`

- <span id="sublibrarycommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SubLibraryCommand<E>`

- <span id="sublibrarycommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SubLibraryCommand<E>`

- <span id="sublibrarycommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SubLibraryCommand<E>`

##### `impl ToOwned for SubLibraryCommand<E>`

- <span id="sublibrarycommand-toowned-type-owned"></span>`type Owned = T`

- <span id="sublibrarycommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sublibrarycommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SubLibraryCommand<E>`

- <span id="sublibrarycommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sublibrarycommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SubLibraryCommand<E>`

- <span id="sublibrarycommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sublibrarycommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PreboundDylibCommand<E: Endian>`

```rust
struct PreboundDylibCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub name: LcStr<E>,
    pub nmodules: crate::endian::U32<E>,
    pub linked_modules: LcStr<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1617-1628`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1617-L1628)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_PREBOUND_DYLIB

- **`cmdsize`**: `crate::endian::U32<E>`

  includes strings

- **`name`**: `LcStr<E>`

  library's path name

- **`nmodules`**: `crate::endian::U32<E>`

  number of modules in library

- **`linked_modules`**: `LcStr<E>`

  bit vector of linked modules

#### Trait Implementations

##### `impl Any for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-clone"></span>`fn clone(&self) -> PreboundDylibCommand<E>` — [`PreboundDylibCommand`](#prebounddylibcommand)

##### `impl CloneToUninit for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for PreboundDylibCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for PreboundDylibCommand<E>`

##### `impl ToOwned for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="prebounddylibcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="prebounddylibcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="prebounddylibcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PreboundDylibCommand<E>`

- <span id="prebounddylibcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="prebounddylibcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DylinkerCommand<E: Endian>`

```rust
struct DylinkerCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub name: LcStr<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1640-1647`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1640-L1647)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ID_DYLINKER, LC_LOAD_DYLINKER or LC_DYLD_ENVIRONMENT

- **`cmdsize`**: `crate::endian::U32<E>`

  includes pathname string

- **`name`**: `LcStr<E>`

  dynamic linker's path name

#### Trait Implementations

##### `impl Any for DylinkerCommand<E>`

- <span id="dylinkercommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DylinkerCommand<E>`

- <span id="dylinkercommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DylinkerCommand<E>`

- <span id="dylinkercommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DylinkerCommand<E>`

- <span id="dylinkercommand-clone"></span>`fn clone(&self) -> DylinkerCommand<E>` — [`DylinkerCommand`](#dylinkercommand)

##### `impl CloneToUninit for DylinkerCommand<E>`

- <span id="dylinkercommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DylinkerCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DylinkerCommand<E>`

- <span id="dylinkercommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DylinkerCommand<E>`

- <span id="dylinkercommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DylinkerCommand<E>`

- <span id="dylinkercommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DylinkerCommand<E>`

##### `impl ToOwned for DylinkerCommand<E>`

- <span id="dylinkercommand-toowned-type-owned"></span>`type Owned = T`

- <span id="dylinkercommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dylinkercommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DylinkerCommand<E>`

- <span id="dylinkercommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dylinkercommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DylinkerCommand<E>`

- <span id="dylinkercommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dylinkercommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThreadCommand<E: Endian>`

```rust
struct ThreadCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1672-1681`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1672-L1681)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_THREAD or  LC_UNIXTHREAD

- **`cmdsize`**: `crate::endian::U32<E>`

  total size of this command

#### Trait Implementations

##### `impl Any for ThreadCommand<E>`

- <span id="threadcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThreadCommand<E>`

- <span id="threadcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThreadCommand<E>`

- <span id="threadcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for ThreadCommand<E>`

- <span id="threadcommand-clone"></span>`fn clone(&self) -> ThreadCommand<E>` — [`ThreadCommand`](#threadcommand)

##### `impl CloneToUninit for ThreadCommand<E>`

- <span id="threadcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for ThreadCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for ThreadCommand<E>`

- <span id="threadcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ThreadCommand<E>`

- <span id="threadcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThreadCommand<E>`

- <span id="threadcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for ThreadCommand<E>`

##### `impl ToOwned for ThreadCommand<E>`

- <span id="threadcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="threadcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="threadcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ThreadCommand<E>`

- <span id="threadcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="threadcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThreadCommand<E>`

- <span id="threadcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="threadcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RoutinesCommand32<E: Endian>`

```rust
struct RoutinesCommand32<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub init_address: crate::endian::U32<E>,
    pub init_module: crate::endian::U32<E>,
    pub reserved1: crate::endian::U32<E>,
    pub reserved2: crate::endian::U32<E>,
    pub reserved3: crate::endian::U32<E>,
    pub reserved4: crate::endian::U32<E>,
    pub reserved5: crate::endian::U32<E>,
    pub reserved6: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1693-1709`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1693-L1709)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ROUTINES

- **`cmdsize`**: `crate::endian::U32<E>`

  total size of this command

- **`init_address`**: `crate::endian::U32<E>`

  address of initialization routine

- **`init_module`**: `crate::endian::U32<E>`

  index into the module table that the init routine is defined in

#### Trait Implementations

##### `impl Any for RoutinesCommand32<E>`

- <span id="routinescommand32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RoutinesCommand32<E>`

- <span id="routinescommand32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RoutinesCommand32<E>`

- <span id="routinescommand32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for RoutinesCommand32<E>`

- <span id="routinescommand32-clone"></span>`fn clone(&self) -> RoutinesCommand32<E>` — [`RoutinesCommand32`](#routinescommand32)

##### `impl CloneToUninit for RoutinesCommand32<E>`

- <span id="routinescommand32-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for RoutinesCommand32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for RoutinesCommand32<E>`

- <span id="routinescommand32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RoutinesCommand32<E>`

- <span id="routinescommand32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RoutinesCommand32<E>`

- <span id="routinescommand32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for RoutinesCommand32<E>`

##### `impl ToOwned for RoutinesCommand32<E>`

- <span id="routinescommand32-toowned-type-owned"></span>`type Owned = T`

- <span id="routinescommand32-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="routinescommand32-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RoutinesCommand32<E>`

- <span id="routinescommand32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="routinescommand32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RoutinesCommand32<E>`

- <span id="routinescommand32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="routinescommand32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RoutinesCommand64<E: Endian>`

```rust
struct RoutinesCommand64<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub init_address: crate::endian::U64<E>,
    pub init_module: crate::endian::U64<E>,
    pub reserved1: crate::endian::U64<E>,
    pub reserved2: crate::endian::U64<E>,
    pub reserved3: crate::endian::U64<E>,
    pub reserved4: crate::endian::U64<E>,
    pub reserved5: crate::endian::U64<E>,
    pub reserved6: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1716-1732`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1716-L1732)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ROUTINES_64

- **`cmdsize`**: `crate::endian::U32<E>`

  total size of this command

- **`init_address`**: `crate::endian::U64<E>`

  address of initialization routine

- **`init_module`**: `crate::endian::U64<E>`

  index into the module table that the init routine is defined in

#### Trait Implementations

##### `impl Any for RoutinesCommand64<E>`

- <span id="routinescommand64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RoutinesCommand64<E>`

- <span id="routinescommand64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RoutinesCommand64<E>`

- <span id="routinescommand64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for RoutinesCommand64<E>`

- <span id="routinescommand64-clone"></span>`fn clone(&self) -> RoutinesCommand64<E>` — [`RoutinesCommand64`](#routinescommand64)

##### `impl CloneToUninit for RoutinesCommand64<E>`

- <span id="routinescommand64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for RoutinesCommand64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for RoutinesCommand64<E>`

- <span id="routinescommand64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RoutinesCommand64<E>`

- <span id="routinescommand64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RoutinesCommand64<E>`

- <span id="routinescommand64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for RoutinesCommand64<E>`

##### `impl ToOwned for RoutinesCommand64<E>`

- <span id="routinescommand64-toowned-type-owned"></span>`type Owned = T`

- <span id="routinescommand64-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="routinescommand64-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RoutinesCommand64<E>`

- <span id="routinescommand64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="routinescommand64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RoutinesCommand64<E>`

- <span id="routinescommand64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="routinescommand64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymtabCommand<E: Endian>`

```rust
struct SymtabCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub symoff: crate::endian::U32<E>,
    pub nsyms: crate::endian::U32<E>,
    pub stroff: crate::endian::U32<E>,
    pub strsize: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1741-1754`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1741-L1754)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SYMTAB

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct SymtabCommand)

- **`symoff`**: `crate::endian::U32<E>`

  symbol table offset

- **`nsyms`**: `crate::endian::U32<E>`

  number of symbol table entries

- **`stroff`**: `crate::endian::U32<E>`

  string table offset

- **`strsize`**: `crate::endian::U32<E>`

  string table size in bytes

#### Implementations

- <span id="machosymtabcommand-symbols"></span>`fn symbols<'data, Mach: MachHeader<Endian = E>, R: ReadRef<'data>>(&self, endian: E, data: R) -> Result<SymbolTable<'data, Mach, R>>` — [`Result`](../index.md#result), [`SymbolTable`](../read/macho/index.md#symboltable)

  Return the symbol table that this command references.

#### Trait Implementations

##### `impl Any for SymtabCommand<E>`

- <span id="symtabcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymtabCommand<E>`

- <span id="symtabcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymtabCommand<E>`

- <span id="symtabcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SymtabCommand<E>`

- <span id="symtabcommand-clone"></span>`fn clone(&self) -> SymtabCommand<E>` — [`SymtabCommand`](#symtabcommand)

##### `impl CloneToUninit for SymtabCommand<E>`

- <span id="symtabcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SymtabCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SymtabCommand<E>`

- <span id="symtabcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymtabCommand<E>`

- <span id="symtabcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymtabCommand<E>`

- <span id="symtabcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SymtabCommand<E>`

##### `impl ToOwned for SymtabCommand<E>`

- <span id="symtabcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="symtabcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symtabcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymtabCommand<E>`

- <span id="symtabcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symtabcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymtabCommand<E>`

- <span id="symtabcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symtabcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DysymtabCommand<E: Endian>`

```rust
struct DysymtabCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub ilocalsym: crate::endian::U32<E>,
    pub nlocalsym: crate::endian::U32<E>,
    pub iextdefsym: crate::endian::U32<E>,
    pub nextdefsym: crate::endian::U32<E>,
    pub iundefsym: crate::endian::U32<E>,
    pub nundefsym: crate::endian::U32<E>,
    pub tocoff: crate::endian::U32<E>,
    pub ntoc: crate::endian::U32<E>,
    pub modtaboff: crate::endian::U32<E>,
    pub nmodtab: crate::endian::U32<E>,
    pub extrefsymoff: crate::endian::U32<E>,
    pub nextrefsyms: crate::endian::U32<E>,
    pub indirectsymoff: crate::endian::U32<E>,
    pub nindirectsyms: crate::endian::U32<E>,
    pub extreloff: crate::endian::U32<E>,
    pub nextrel: crate::endian::U32<E>,
    pub locreloff: crate::endian::U32<E>,
    pub nlocrel: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1798-1931`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1798-L1931)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_DYSYMTAB

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct DysymtabCommand)

- **`ilocalsym`**: `crate::endian::U32<E>`

  index to local symbols

- **`nlocalsym`**: `crate::endian::U32<E>`

  number of local symbols

- **`iextdefsym`**: `crate::endian::U32<E>`

  index to externally defined symbols

- **`nextdefsym`**: `crate::endian::U32<E>`

  number of externally defined symbols

- **`iundefsym`**: `crate::endian::U32<E>`

  index to undefined symbols

- **`nundefsym`**: `crate::endian::U32<E>`

  number of undefined symbols

- **`tocoff`**: `crate::endian::U32<E>`

  file offset to table of contents

- **`ntoc`**: `crate::endian::U32<E>`

  number of entries in table of contents

- **`modtaboff`**: `crate::endian::U32<E>`

  file offset to module table

- **`nmodtab`**: `crate::endian::U32<E>`

  number of module table entries

- **`extrefsymoff`**: `crate::endian::U32<E>`

  offset to referenced symbol table

- **`nextrefsyms`**: `crate::endian::U32<E>`

  number of referenced symbol table entries

- **`indirectsymoff`**: `crate::endian::U32<E>`

  file offset to the indirect symbol table

- **`nindirectsyms`**: `crate::endian::U32<E>`

  number of indirect symbol table entries

- **`extreloff`**: `crate::endian::U32<E>`

  offset to external relocation entries

- **`nextrel`**: `crate::endian::U32<E>`

  number of external relocation entries

- **`locreloff`**: `crate::endian::U32<E>`

  offset to local relocation entries

- **`nlocrel`**: `crate::endian::U32<E>`

  number of local relocation entries

#### Trait Implementations

##### `impl Any for DysymtabCommand<E>`

- <span id="dysymtabcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DysymtabCommand<E>`

- <span id="dysymtabcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DysymtabCommand<E>`

- <span id="dysymtabcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DysymtabCommand<E>`

- <span id="dysymtabcommand-clone"></span>`fn clone(&self) -> DysymtabCommand<E>` — [`DysymtabCommand`](#dysymtabcommand)

##### `impl CloneToUninit for DysymtabCommand<E>`

- <span id="dysymtabcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DysymtabCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DysymtabCommand<E>`

- <span id="dysymtabcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DysymtabCommand<E>`

- <span id="dysymtabcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DysymtabCommand<E>`

- <span id="dysymtabcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DysymtabCommand<E>`

##### `impl ToOwned for DysymtabCommand<E>`

- <span id="dysymtabcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="dysymtabcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dysymtabcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DysymtabCommand<E>`

- <span id="dysymtabcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dysymtabcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DysymtabCommand<E>`

- <span id="dysymtabcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dysymtabcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DylibTableOfContents<E: Endian>`

```rust
struct DylibTableOfContents<E: Endian> {
    pub symbol_index: crate::endian::U32<E>,
    pub module_index: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1946-1951`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1946-L1951)*

#### Fields

- **`symbol_index`**: `crate::endian::U32<E>`

  the defined external symbol (index into the symbol table)

- **`module_index`**: `crate::endian::U32<E>`

  index into the module table this symbol is defined in

#### Trait Implementations

##### `impl Any for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-clone"></span>`fn clone(&self) -> DylibTableOfContents<E>` — [`DylibTableOfContents`](#dylibtableofcontents)

##### `impl CloneToUninit for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DylibTableOfContents<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DylibTableOfContents<E>`

##### `impl ToOwned for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-toowned-type-owned"></span>`type Owned = T`

- <span id="dylibtableofcontents-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dylibtableofcontents-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dylibtableofcontents-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DylibTableOfContents<E>`

- <span id="dylibtableofcontents-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dylibtableofcontents-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DylibModule32<E: Endian>`

```rust
struct DylibModule32<E: Endian> {
    pub module_name: crate::endian::U32<E>,
    pub iextdefsym: crate::endian::U32<E>,
    pub nextdefsym: crate::endian::U32<E>,
    pub irefsym: crate::endian::U32<E>,
    pub nrefsym: crate::endian::U32<E>,
    pub ilocalsym: crate::endian::U32<E>,
    pub nlocalsym: crate::endian::U32<E>,
    pub iextrel: crate::endian::U32<E>,
    pub nextrel: crate::endian::U32<E>,
    pub iinit_iterm: crate::endian::U32<E>,
    pub ninit_nterm: crate::endian::U32<E>,
    pub objc_module_info_addr: crate::endian::U32<E>,
    pub objc_module_info_size: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1956-1987`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1956-L1987)*

#### Fields

- **`module_name`**: `crate::endian::U32<E>`

  the module name (index into string table)

- **`iextdefsym`**: `crate::endian::U32<E>`

  index into externally defined symbols

- **`nextdefsym`**: `crate::endian::U32<E>`

  number of externally defined symbols

- **`irefsym`**: `crate::endian::U32<E>`

  index into reference symbol table

- **`nrefsym`**: `crate::endian::U32<E>`

  number of reference symbol table entries

- **`ilocalsym`**: `crate::endian::U32<E>`

  index into symbols for local symbols

- **`nlocalsym`**: `crate::endian::U32<E>`

  number of local symbols

- **`iextrel`**: `crate::endian::U32<E>`

  index into external relocation entries

- **`nextrel`**: `crate::endian::U32<E>`

  number of external relocation entries

- **`iinit_iterm`**: `crate::endian::U32<E>`

  low 16 bits are the index into the init section, high 16 bits are the index into the term section

- **`ninit_nterm`**: `crate::endian::U32<E>`

  low 16 bits are the number of init section entries, high 16 bits are the number of term section entries

- **`objc_module_info_addr`**: `crate::endian::U32<E>`

  for this module address of the start of the (__OBJC,__module_info) section

- **`objc_module_info_size`**: `crate::endian::U32<E>`

  for this module size of the (__OBJC,__module_info) section

#### Trait Implementations

##### `impl Any for DylibModule32<E>`

- <span id="dylibmodule32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DylibModule32<E>`

- <span id="dylibmodule32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DylibModule32<E>`

- <span id="dylibmodule32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DylibModule32<E>`

- <span id="dylibmodule32-clone"></span>`fn clone(&self) -> DylibModule32<E>` — [`DylibModule32`](#dylibmodule32)

##### `impl CloneToUninit for DylibModule32<E>`

- <span id="dylibmodule32-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DylibModule32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DylibModule32<E>`

- <span id="dylibmodule32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DylibModule32<E>`

- <span id="dylibmodule32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DylibModule32<E>`

- <span id="dylibmodule32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DylibModule32<E>`

##### `impl ToOwned for DylibModule32<E>`

- <span id="dylibmodule32-toowned-type-owned"></span>`type Owned = T`

- <span id="dylibmodule32-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dylibmodule32-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DylibModule32<E>`

- <span id="dylibmodule32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dylibmodule32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DylibModule32<E>`

- <span id="dylibmodule32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dylibmodule32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DylibModule64<E: Endian>`

```rust
struct DylibModule64<E: Endian> {
    pub module_name: crate::endian::U32<E>,
    pub iextdefsym: crate::endian::U32<E>,
    pub nextdefsym: crate::endian::U32<E>,
    pub irefsym: crate::endian::U32<E>,
    pub nrefsym: crate::endian::U32<E>,
    pub ilocalsym: crate::endian::U32<E>,
    pub nlocalsym: crate::endian::U32<E>,
    pub iextrel: crate::endian::U32<E>,
    pub nextrel: crate::endian::U32<E>,
    pub iinit_iterm: crate::endian::U32<E>,
    pub ninit_nterm: crate::endian::U32<E>,
    pub objc_module_info_size: crate::endian::U32<E>,
    pub objc_module_info_addr: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:1992-2023`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1992-L2023)*

#### Fields

- **`module_name`**: `crate::endian::U32<E>`

  the module name (index into string table)

- **`iextdefsym`**: `crate::endian::U32<E>`

  index into externally defined symbols

- **`nextdefsym`**: `crate::endian::U32<E>`

  number of externally defined symbols

- **`irefsym`**: `crate::endian::U32<E>`

  index into reference symbol table

- **`nrefsym`**: `crate::endian::U32<E>`

  number of reference symbol table entries

- **`ilocalsym`**: `crate::endian::U32<E>`

  index into symbols for local symbols

- **`nlocalsym`**: `crate::endian::U32<E>`

  number of local symbols

- **`iextrel`**: `crate::endian::U32<E>`

  index into external relocation entries

- **`nextrel`**: `crate::endian::U32<E>`

  number of external relocation entries

- **`iinit_iterm`**: `crate::endian::U32<E>`

  low 16 bits are the index into the init section, high 16 bits are the index into the term section

- **`ninit_nterm`**: `crate::endian::U32<E>`

  low 16 bits are the number of init section entries, high 16 bits are the number of term section entries

- **`objc_module_info_size`**: `crate::endian::U32<E>`

  for this module size of the (__OBJC,__module_info) section

- **`objc_module_info_addr`**: `crate::endian::U64<E>`

  for this module address of the start of the (__OBJC,__module_info) section

#### Trait Implementations

##### `impl Any for DylibModule64<E>`

- <span id="dylibmodule64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DylibModule64<E>`

- <span id="dylibmodule64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DylibModule64<E>`

- <span id="dylibmodule64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DylibModule64<E>`

- <span id="dylibmodule64-clone"></span>`fn clone(&self) -> DylibModule64<E>` — [`DylibModule64`](#dylibmodule64)

##### `impl CloneToUninit for DylibModule64<E>`

- <span id="dylibmodule64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DylibModule64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DylibModule64<E>`

- <span id="dylibmodule64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DylibModule64<E>`

- <span id="dylibmodule64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DylibModule64<E>`

- <span id="dylibmodule64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DylibModule64<E>`

##### `impl ToOwned for DylibModule64<E>`

- <span id="dylibmodule64-toowned-type-owned"></span>`type Owned = T`

- <span id="dylibmodule64-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dylibmodule64-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DylibModule64<E>`

- <span id="dylibmodule64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dylibmodule64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DylibModule64<E>`

- <span id="dylibmodule64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dylibmodule64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DylibReference<E: Endian>`

```rust
struct DylibReference<E: Endian> {
    pub bitfield: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2035-2041`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2035-L2041)*

#### Trait Implementations

##### `impl Any for DylibReference<E>`

- <span id="dylibreference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DylibReference<E>`

- <span id="dylibreference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DylibReference<E>`

- <span id="dylibreference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DylibReference<E>`

- <span id="dylibreference-clone"></span>`fn clone(&self) -> DylibReference<E>` — [`DylibReference`](#dylibreference)

##### `impl CloneToUninit for DylibReference<E>`

- <span id="dylibreference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DylibReference<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DylibReference<E>`

- <span id="dylibreference-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DylibReference<E>`

- <span id="dylibreference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DylibReference<E>`

- <span id="dylibreference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DylibReference<E>`

##### `impl ToOwned for DylibReference<E>`

- <span id="dylibreference-toowned-type-owned"></span>`type Owned = T`

- <span id="dylibreference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dylibreference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DylibReference<E>`

- <span id="dylibreference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dylibreference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DylibReference<E>`

- <span id="dylibreference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dylibreference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TwolevelHintsCommand<E: Endian>`

```rust
struct TwolevelHintsCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub offset: crate::endian::U32<E>,
    pub nhints: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2049-2058`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2049-L2058)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_TWOLEVEL_HINTS

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct TwolevelHintsCommand)

- **`offset`**: `crate::endian::U32<E>`

  offset to the hint table

- **`nhints`**: `crate::endian::U32<E>`

  number of hints in the hint table

#### Trait Implementations

##### `impl Any for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-clone"></span>`fn clone(&self) -> TwolevelHintsCommand<E>` — [`TwolevelHintsCommand`](#twolevelhintscommand)

##### `impl CloneToUninit for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for TwolevelHintsCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for TwolevelHintsCommand<E>`

##### `impl ToOwned for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-toowned-type-owned"></span>`type Owned = T`

- <span id="twolevelhintscommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="twolevelhintscommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="twolevelhintscommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TwolevelHintsCommand<E>`

- <span id="twolevelhintscommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="twolevelhintscommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TwolevelHint<E: Endian>`

```rust
struct TwolevelHint<E: Endian> {
    pub bitfield: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2078-2085`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2078-L2085)*

#### Trait Implementations

##### `impl Any for TwolevelHint<E>`

- <span id="twolevelhint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TwolevelHint<E>`

- <span id="twolevelhint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TwolevelHint<E>`

- <span id="twolevelhint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for TwolevelHint<E>`

- <span id="twolevelhint-clone"></span>`fn clone(&self) -> TwolevelHint<E>` — [`TwolevelHint`](#twolevelhint)

##### `impl CloneToUninit for TwolevelHint<E>`

- <span id="twolevelhint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for TwolevelHint<E>`

##### `impl<E: fmt::Debug + Endian> Debug for TwolevelHint<E>`

- <span id="twolevelhint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TwolevelHint<E>`

- <span id="twolevelhint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TwolevelHint<E>`

- <span id="twolevelhint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for TwolevelHint<E>`

##### `impl ToOwned for TwolevelHint<E>`

- <span id="twolevelhint-toowned-type-owned"></span>`type Owned = T`

- <span id="twolevelhint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="twolevelhint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TwolevelHint<E>`

- <span id="twolevelhint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="twolevelhint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TwolevelHint<E>`

- <span id="twolevelhint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="twolevelhint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PrebindCksumCommand<E: Endian>`

```rust
struct PrebindCksumCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub cksum: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2099-2106`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2099-L2106)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_PREBIND_CKSUM

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct PrebindCksumCommand)

- **`cksum`**: `crate::endian::U32<E>`

  the check sum or zero

#### Trait Implementations

##### `impl Any for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-clone"></span>`fn clone(&self) -> PrebindCksumCommand<E>` — [`PrebindCksumCommand`](#prebindcksumcommand)

##### `impl CloneToUninit for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for PrebindCksumCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for PrebindCksumCommand<E>`

##### `impl ToOwned for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="prebindcksumcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="prebindcksumcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="prebindcksumcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PrebindCksumCommand<E>`

- <span id="prebindcksumcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="prebindcksumcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UuidCommand<E: Endian>`

```rust
struct UuidCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub uuid: [u8; 16],
}
```

*Defined in [`object-0.37.3/src/macho.rs:2114-2121`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2114-L2121)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_UUID

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct UuidCommand)

- **`uuid`**: `[u8; 16]`

  the 128-bit uuid

#### Trait Implementations

##### `impl Any for UuidCommand<E>`

- <span id="uuidcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UuidCommand<E>`

- <span id="uuidcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UuidCommand<E>`

- <span id="uuidcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for UuidCommand<E>`

- <span id="uuidcommand-clone"></span>`fn clone(&self) -> UuidCommand<E>` — [`UuidCommand`](#uuidcommand)

##### `impl CloneToUninit for UuidCommand<E>`

- <span id="uuidcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for UuidCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for UuidCommand<E>`

- <span id="uuidcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UuidCommand<E>`

- <span id="uuidcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UuidCommand<E>`

- <span id="uuidcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for UuidCommand<E>`

##### `impl ToOwned for UuidCommand<E>`

- <span id="uuidcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="uuidcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="uuidcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UuidCommand<E>`

- <span id="uuidcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uuidcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UuidCommand<E>`

- <span id="uuidcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uuidcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RpathCommand<E: Endian>`

```rust
struct RpathCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub path: LcStr<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2129-2136`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2129-L2136)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_RPATH

- **`cmdsize`**: `crate::endian::U32<E>`

  includes string

- **`path`**: `LcStr<E>`

  path to add to run path

#### Trait Implementations

##### `impl Any for RpathCommand<E>`

- <span id="rpathcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RpathCommand<E>`

- <span id="rpathcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RpathCommand<E>`

- <span id="rpathcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for RpathCommand<E>`

- <span id="rpathcommand-clone"></span>`fn clone(&self) -> RpathCommand<E>` — [`RpathCommand`](#rpathcommand)

##### `impl CloneToUninit for RpathCommand<E>`

- <span id="rpathcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for RpathCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for RpathCommand<E>`

- <span id="rpathcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RpathCommand<E>`

- <span id="rpathcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RpathCommand<E>`

- <span id="rpathcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for RpathCommand<E>`

##### `impl ToOwned for RpathCommand<E>`

- <span id="rpathcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="rpathcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rpathcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RpathCommand<E>`

- <span id="rpathcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rpathcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RpathCommand<E>`

- <span id="rpathcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rpathcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LinkeditDataCommand<E: Endian>`

```rust
struct LinkeditDataCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub dataoff: crate::endian::U32<E>,
    pub datasize: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2144-2155`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2144-L2155)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  `LC_CODE_SIGNATURE`, `LC_SEGMENT_SPLIT_INFO`, `LC_FUNCTION_STARTS`,
  `LC_DATA_IN_CODE`, `LC_DYLIB_CODE_SIGN_DRS`, `LC_LINKER_OPTIMIZATION_HINT`,
  `LC_DYLD_EXPORTS_TRIE`, or `LC_DYLD_CHAINED_FIXUPS`.

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct LinkeditDataCommand)

- **`dataoff`**: `crate::endian::U32<E>`

  file offset of data in __LINKEDIT segment

- **`datasize`**: `crate::endian::U32<E>`

  file size of data in __LINKEDIT segment

#### Trait Implementations

##### `impl Any for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-clone"></span>`fn clone(&self) -> LinkeditDataCommand<E>` — [`LinkeditDataCommand`](#linkeditdatacommand)

##### `impl CloneToUninit for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for LinkeditDataCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for LinkeditDataCommand<E>`

##### `impl ToOwned for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-toowned-type-owned"></span>`type Owned = T`

- <span id="linkeditdatacommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linkeditdatacommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linkeditdatacommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LinkeditDataCommand<E>`

- <span id="linkeditdatacommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linkeditdatacommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FilesetEntryCommand<E: Endian>`

```rust
struct FilesetEntryCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub vmaddr: crate::endian::U64<E>,
    pub fileoff: crate::endian::U64<E>,
    pub entry_id: LcStr<E>,
    pub reserved: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2159-2172`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2159-L2172)*

#### Fields

- **`cmdsize`**: `crate::endian::U32<E>`

  includes id string

- **`vmaddr`**: `crate::endian::U64<E>`

  memory address of the dylib

- **`fileoff`**: `crate::endian::U64<E>`

  file offset of the dylib

- **`entry_id`**: `LcStr<E>`

  contained entry id

- **`reserved`**: `crate::endian::U32<E>`

  entry_id is 32-bits long, so this is the reserved padding

#### Trait Implementations

##### `impl Any for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-clone"></span>`fn clone(&self) -> FilesetEntryCommand<E>` — [`FilesetEntryCommand`](#filesetentrycommand)

##### `impl CloneToUninit for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for FilesetEntryCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for FilesetEntryCommand<E>`

##### `impl ToOwned for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-toowned-type-owned"></span>`type Owned = T`

- <span id="filesetentrycommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="filesetentrycommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filesetentrycommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilesetEntryCommand<E>`

- <span id="filesetentrycommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filesetentrycommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EncryptionInfoCommand32<E: Endian>`

```rust
struct EncryptionInfoCommand32<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub cryptoff: crate::endian::U32<E>,
    pub cryptsize: crate::endian::U32<E>,
    pub cryptid: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2180-2191`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2180-L2191)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ENCRYPTION_INFO

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct EncryptionInfoCommand32)

- **`cryptoff`**: `crate::endian::U32<E>`

  file offset of encrypted range

- **`cryptsize`**: `crate::endian::U32<E>`

  file size of encrypted range

- **`cryptid`**: `crate::endian::U32<E>`

  which enryption system, 0 means not-encrypted yet

#### Trait Implementations

##### `impl Any for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-clone"></span>`fn clone(&self) -> EncryptionInfoCommand32<E>` — [`EncryptionInfoCommand32`](#encryptioninfocommand32)

##### `impl CloneToUninit for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for EncryptionInfoCommand32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for EncryptionInfoCommand32<E>`

##### `impl ToOwned for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-toowned-type-owned"></span>`type Owned = T`

- <span id="encryptioninfocommand32-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="encryptioninfocommand32-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="encryptioninfocommand32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EncryptionInfoCommand32<E>`

- <span id="encryptioninfocommand32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="encryptioninfocommand32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EncryptionInfoCommand64<E: Endian>`

```rust
struct EncryptionInfoCommand64<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub cryptoff: crate::endian::U32<E>,
    pub cryptsize: crate::endian::U32<E>,
    pub cryptid: crate::endian::U32<E>,
    pub pad: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2199-2212`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2199-L2212)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ENCRYPTION_INFO_64

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct EncryptionInfoCommand64)

- **`cryptoff`**: `crate::endian::U32<E>`

  file offset of encrypted range

- **`cryptsize`**: `crate::endian::U32<E>`

  file size of encrypted range

- **`cryptid`**: `crate::endian::U32<E>`

  which enryption system, 0 means not-encrypted yet

- **`pad`**: `crate::endian::U32<E>`

  padding to make this struct's size a multiple of 8 bytes

#### Trait Implementations

##### `impl Any for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-clone"></span>`fn clone(&self) -> EncryptionInfoCommand64<E>` — [`EncryptionInfoCommand64`](#encryptioninfocommand64)

##### `impl CloneToUninit for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for EncryptionInfoCommand64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for EncryptionInfoCommand64<E>`

##### `impl ToOwned for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-toowned-type-owned"></span>`type Owned = T`

- <span id="encryptioninfocommand64-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="encryptioninfocommand64-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="encryptioninfocommand64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EncryptionInfoCommand64<E>`

- <span id="encryptioninfocommand64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="encryptioninfocommand64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VersionMinCommand<E: Endian>`

```rust
struct VersionMinCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub version: crate::endian::U32<E>,
    pub sdk: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2220-2229`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2220-L2229)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_VERSION_MIN_MACOSX or LC_VERSION_MIN_IPHONEOS or LC_VERSION_MIN_WATCHOS or LC_VERSION_MIN_TVOS

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct VersionMinCommand)

- **`version`**: `crate::endian::U32<E>`

  X.Y.Z is encoded in nibbles xxxx.yy.zz

- **`sdk`**: `crate::endian::U32<E>`

  X.Y.Z is encoded in nibbles xxxx.yy.zz

#### Trait Implementations

##### `impl Any for VersionMinCommand<E>`

- <span id="versionmincommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VersionMinCommand<E>`

- <span id="versionmincommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VersionMinCommand<E>`

- <span id="versionmincommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for VersionMinCommand<E>`

- <span id="versionmincommand-clone"></span>`fn clone(&self) -> VersionMinCommand<E>` — [`VersionMinCommand`](#versionmincommand)

##### `impl CloneToUninit for VersionMinCommand<E>`

- <span id="versionmincommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for VersionMinCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for VersionMinCommand<E>`

- <span id="versionmincommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VersionMinCommand<E>`

- <span id="versionmincommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VersionMinCommand<E>`

- <span id="versionmincommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for VersionMinCommand<E>`

##### `impl ToOwned for VersionMinCommand<E>`

- <span id="versionmincommand-toowned-type-owned"></span>`type Owned = T`

- <span id="versionmincommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="versionmincommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VersionMinCommand<E>`

- <span id="versionmincommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="versionmincommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VersionMinCommand<E>`

- <span id="versionmincommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="versionmincommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BuildVersionCommand<E: Endian>`

```rust
struct BuildVersionCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub platform: crate::endian::U32<E>,
    pub minos: crate::endian::U32<E>,
    pub sdk: crate::endian::U32<E>,
    pub ntools: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2238-2251`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2238-L2251)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_BUILD_VERSION

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct BuildVersionCommand) plus ntools * sizeof(struct BuildToolVersion)

- **`platform`**: `crate::endian::U32<E>`

  platform

- **`minos`**: `crate::endian::U32<E>`

  X.Y.Z is encoded in nibbles xxxx.yy.zz

- **`sdk`**: `crate::endian::U32<E>`

  X.Y.Z is encoded in nibbles xxxx.yy.zz

- **`ntools`**: `crate::endian::U32<E>`

  number of tool entries following this

#### Trait Implementations

##### `impl Any for BuildVersionCommand<E>`

- <span id="buildversioncommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildVersionCommand<E>`

- <span id="buildversioncommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildVersionCommand<E>`

- <span id="buildversioncommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for BuildVersionCommand<E>`

- <span id="buildversioncommand-clone"></span>`fn clone(&self) -> BuildVersionCommand<E>` — [`BuildVersionCommand`](#buildversioncommand)

##### `impl CloneToUninit for BuildVersionCommand<E>`

- <span id="buildversioncommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for BuildVersionCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for BuildVersionCommand<E>`

- <span id="buildversioncommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BuildVersionCommand<E>`

- <span id="buildversioncommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildVersionCommand<E>`

- <span id="buildversioncommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for BuildVersionCommand<E>`

##### `impl ToOwned for BuildVersionCommand<E>`

- <span id="buildversioncommand-toowned-type-owned"></span>`type Owned = T`

- <span id="buildversioncommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="buildversioncommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BuildVersionCommand<E>`

- <span id="buildversioncommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buildversioncommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildVersionCommand<E>`

- <span id="buildversioncommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buildversioncommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BuildToolVersion<E: Endian>`

```rust
struct BuildToolVersion<E: Endian> {
    pub tool: crate::endian::U32<E>,
    pub version: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2255-2260`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2255-L2260)*

#### Fields

- **`tool`**: `crate::endian::U32<E>`

  enum for the tool

- **`version`**: `crate::endian::U32<E>`

  version number of the tool

#### Trait Implementations

##### `impl Any for BuildToolVersion<E>`

- <span id="buildtoolversion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildToolVersion<E>`

- <span id="buildtoolversion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildToolVersion<E>`

- <span id="buildtoolversion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for BuildToolVersion<E>`

- <span id="buildtoolversion-clone"></span>`fn clone(&self) -> BuildToolVersion<E>` — [`BuildToolVersion`](#buildtoolversion)

##### `impl CloneToUninit for BuildToolVersion<E>`

- <span id="buildtoolversion-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for BuildToolVersion<E>`

##### `impl<E: fmt::Debug + Endian> Debug for BuildToolVersion<E>`

- <span id="buildtoolversion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BuildToolVersion<E>`

- <span id="buildtoolversion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildToolVersion<E>`

- <span id="buildtoolversion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for BuildToolVersion<E>`

##### `impl ToOwned for BuildToolVersion<E>`

- <span id="buildtoolversion-toowned-type-owned"></span>`type Owned = T`

- <span id="buildtoolversion-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="buildtoolversion-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BuildToolVersion<E>`

- <span id="buildtoolversion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buildtoolversion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildToolVersion<E>`

- <span id="buildtoolversion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buildtoolversion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DyldInfoCommand<E: Endian>`

```rust
struct DyldInfoCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub rebase_off: crate::endian::U32<E>,
    pub rebase_size: crate::endian::U32<E>,
    pub bind_off: crate::endian::U32<E>,
    pub bind_size: crate::endian::U32<E>,
    pub weak_bind_off: crate::endian::U32<E>,
    pub weak_bind_size: crate::endian::U32<E>,
    pub lazy_bind_off: crate::endian::U32<E>,
    pub lazy_bind_size: crate::endian::U32<E>,
    pub export_off: crate::endian::U32<E>,
    pub export_size: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2291-2404`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2291-L2404)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_DYLD_INFO or LC_DYLD_INFO_ONLY

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct DyldInfoCommand)

- **`rebase_off`**: `crate::endian::U32<E>`

  file offset to rebase info

- **`rebase_size`**: `crate::endian::U32<E>`

  size of rebase info

- **`bind_off`**: `crate::endian::U32<E>`

  file offset to binding info

- **`bind_size`**: `crate::endian::U32<E>`

  size of binding info

- **`weak_bind_off`**: `crate::endian::U32<E>`

  file offset to weak binding info

- **`weak_bind_size`**: `crate::endian::U32<E>`

  size of weak binding info

- **`lazy_bind_off`**: `crate::endian::U32<E>`

  file offset to lazy binding info

- **`lazy_bind_size`**: `crate::endian::U32<E>`

  size of lazy binding infs

- **`export_off`**: `crate::endian::U32<E>`

  file offset to lazy binding info

- **`export_size`**: `crate::endian::U32<E>`

  size of lazy binding infs

#### Trait Implementations

##### `impl Any for DyldInfoCommand<E>`

- <span id="dyldinfocommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldInfoCommand<E>`

- <span id="dyldinfocommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldInfoCommand<E>`

- <span id="dyldinfocommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldInfoCommand<E>`

- <span id="dyldinfocommand-clone"></span>`fn clone(&self) -> DyldInfoCommand<E>` — [`DyldInfoCommand`](#dyldinfocommand)

##### `impl CloneToUninit for DyldInfoCommand<E>`

- <span id="dyldinfocommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldInfoCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldInfoCommand<E>`

- <span id="dyldinfocommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldInfoCommand<E>`

- <span id="dyldinfocommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldInfoCommand<E>`

- <span id="dyldinfocommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DyldInfoCommand<E>`

##### `impl ToOwned for DyldInfoCommand<E>`

- <span id="dyldinfocommand-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldinfocommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldinfocommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldInfoCommand<E>`

- <span id="dyldinfocommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldinfocommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldInfoCommand<E>`

- <span id="dyldinfocommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldinfocommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LinkerOptionCommand<E: Endian>`

```rust
struct LinkerOptionCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub count: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2476-2484`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2476-L2484)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_LINKER_OPTION only used in MH_OBJECT filetypes

- **`count`**: `crate::endian::U32<E>`

  number of strings

#### Trait Implementations

##### `impl Any for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-clone"></span>`fn clone(&self) -> LinkerOptionCommand<E>` — [`LinkerOptionCommand`](#linkeroptioncommand)

##### `impl CloneToUninit for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for LinkerOptionCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for LinkerOptionCommand<E>`

##### `impl ToOwned for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-toowned-type-owned"></span>`type Owned = T`

- <span id="linkeroptioncommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linkeroptioncommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linkeroptioncommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LinkerOptionCommand<E>`

- <span id="linkeroptioncommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linkeroptioncommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymsegCommand<E: Endian>`

```rust
struct SymsegCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub offset: crate::endian::U32<E>,
    pub size: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2497-2506`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2497-L2506)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SYMSEG

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct SymsegCommand)

- **`offset`**: `crate::endian::U32<E>`

  symbol segment offset

- **`size`**: `crate::endian::U32<E>`

  symbol segment size in bytes

#### Trait Implementations

##### `impl Any for SymsegCommand<E>`

- <span id="symsegcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymsegCommand<E>`

- <span id="symsegcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymsegCommand<E>`

- <span id="symsegcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SymsegCommand<E>`

- <span id="symsegcommand-clone"></span>`fn clone(&self) -> SymsegCommand<E>` — [`SymsegCommand`](#symsegcommand)

##### `impl CloneToUninit for SymsegCommand<E>`

- <span id="symsegcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SymsegCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SymsegCommand<E>`

- <span id="symsegcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymsegCommand<E>`

- <span id="symsegcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymsegCommand<E>`

- <span id="symsegcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SymsegCommand<E>`

##### `impl ToOwned for SymsegCommand<E>`

- <span id="symsegcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="symsegcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symsegcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymsegCommand<E>`

- <span id="symsegcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symsegcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymsegCommand<E>`

- <span id="symsegcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symsegcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IdentCommand<E: Endian>`

```rust
struct IdentCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2516-2521`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2516-L2521)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_IDENT

- **`cmdsize`**: `crate::endian::U32<E>`

  strings that follow this command

#### Trait Implementations

##### `impl Any for IdentCommand<E>`

- <span id="identcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IdentCommand<E>`

- <span id="identcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IdentCommand<E>`

- <span id="identcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for IdentCommand<E>`

- <span id="identcommand-clone"></span>`fn clone(&self) -> IdentCommand<E>` — [`IdentCommand`](#identcommand)

##### `impl CloneToUninit for IdentCommand<E>`

- <span id="identcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for IdentCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for IdentCommand<E>`

- <span id="identcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IdentCommand<E>`

- <span id="identcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IdentCommand<E>`

- <span id="identcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for IdentCommand<E>`

##### `impl ToOwned for IdentCommand<E>`

- <span id="identcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="identcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="identcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IdentCommand<E>`

- <span id="identcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="identcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IdentCommand<E>`

- <span id="identcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="identcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FvmfileCommand<E: Endian>`

```rust
struct FvmfileCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub name: LcStr<E>,
    pub header_addr: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2531-2540`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2531-L2540)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_FVMFILE

- **`cmdsize`**: `crate::endian::U32<E>`

  includes pathname string

- **`name`**: `LcStr<E>`

  files pathname

- **`header_addr`**: `crate::endian::U32<E>`

  files virtual address

#### Trait Implementations

##### `impl Any for FvmfileCommand<E>`

- <span id="fvmfilecommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FvmfileCommand<E>`

- <span id="fvmfilecommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FvmfileCommand<E>`

- <span id="fvmfilecommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for FvmfileCommand<E>`

- <span id="fvmfilecommand-clone"></span>`fn clone(&self) -> FvmfileCommand<E>` — [`FvmfileCommand`](#fvmfilecommand)

##### `impl CloneToUninit for FvmfileCommand<E>`

- <span id="fvmfilecommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for FvmfileCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for FvmfileCommand<E>`

- <span id="fvmfilecommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FvmfileCommand<E>`

- <span id="fvmfilecommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FvmfileCommand<E>`

- <span id="fvmfilecommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for FvmfileCommand<E>`

##### `impl ToOwned for FvmfileCommand<E>`

- <span id="fvmfilecommand-toowned-type-owned"></span>`type Owned = T`

- <span id="fvmfilecommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fvmfilecommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FvmfileCommand<E>`

- <span id="fvmfilecommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fvmfilecommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FvmfileCommand<E>`

- <span id="fvmfilecommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fvmfilecommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EntryPointCommand<E: Endian>`

```rust
struct EntryPointCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub entryoff: crate::endian::U64<E>,
    pub stacksize: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2550-2559`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2550-L2559)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_MAIN only used in MH_EXECUTE filetypes

- **`cmdsize`**: `crate::endian::U32<E>`

  24

- **`entryoff`**: `crate::endian::U64<E>`

  file (__TEXT) offset of main()

- **`stacksize`**: `crate::endian::U64<E>`

  if not zero, initial stack size

#### Trait Implementations

##### `impl Any for EntryPointCommand<E>`

- <span id="entrypointcommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EntryPointCommand<E>`

- <span id="entrypointcommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EntryPointCommand<E>`

- <span id="entrypointcommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for EntryPointCommand<E>`

- <span id="entrypointcommand-clone"></span>`fn clone(&self) -> EntryPointCommand<E>` — [`EntryPointCommand`](#entrypointcommand)

##### `impl CloneToUninit for EntryPointCommand<E>`

- <span id="entrypointcommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for EntryPointCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for EntryPointCommand<E>`

- <span id="entrypointcommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EntryPointCommand<E>`

- <span id="entrypointcommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EntryPointCommand<E>`

- <span id="entrypointcommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for EntryPointCommand<E>`

##### `impl ToOwned for EntryPointCommand<E>`

- <span id="entrypointcommand-toowned-type-owned"></span>`type Owned = T`

- <span id="entrypointcommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="entrypointcommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EntryPointCommand<E>`

- <span id="entrypointcommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entrypointcommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EntryPointCommand<E>`

- <span id="entrypointcommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entrypointcommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SourceVersionCommand<E: Endian>`

```rust
struct SourceVersionCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub version: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2567-2574`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2567-L2574)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SOURCE_VERSION

- **`cmdsize`**: `crate::endian::U32<E>`

  16

- **`version`**: `crate::endian::U64<E>`

  A.B.C.D.E packed as a24.b10.c10.d10.e10

#### Trait Implementations

##### `impl Any for SourceVersionCommand<E>`

- <span id="sourceversioncommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceVersionCommand<E>`

- <span id="sourceversioncommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceVersionCommand<E>`

- <span id="sourceversioncommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for SourceVersionCommand<E>`

- <span id="sourceversioncommand-clone"></span>`fn clone(&self) -> SourceVersionCommand<E>` — [`SourceVersionCommand`](#sourceversioncommand)

##### `impl CloneToUninit for SourceVersionCommand<E>`

- <span id="sourceversioncommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for SourceVersionCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SourceVersionCommand<E>`

- <span id="sourceversioncommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SourceVersionCommand<E>`

- <span id="sourceversioncommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SourceVersionCommand<E>`

- <span id="sourceversioncommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for SourceVersionCommand<E>`

##### `impl ToOwned for SourceVersionCommand<E>`

- <span id="sourceversioncommand-toowned-type-owned"></span>`type Owned = T`

- <span id="sourceversioncommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sourceversioncommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SourceVersionCommand<E>`

- <span id="sourceversioncommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourceversioncommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceVersionCommand<E>`

- <span id="sourceversioncommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourceversioncommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataInCodeEntry<E: Endian>`

```rust
struct DataInCodeEntry<E: Endian> {
    pub offset: crate::endian::U32<E>,
    pub length: crate::endian::U16<E>,
    pub kind: crate::endian::U16<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2583-2590`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2583-L2590)*

#### Fields

- **`offset`**: `crate::endian::U32<E>`

  from mach_header to start of data range

- **`length`**: `crate::endian::U16<E>`

  number of bytes in data range

- **`kind`**: `crate::endian::U16<E>`

  a DICE_KIND_* value

#### Trait Implementations

##### `impl Any for DataInCodeEntry<E>`

- <span id="dataincodeentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataInCodeEntry<E>`

- <span id="dataincodeentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataInCodeEntry<E>`

- <span id="dataincodeentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DataInCodeEntry<E>`

- <span id="dataincodeentry-clone"></span>`fn clone(&self) -> DataInCodeEntry<E>` — [`DataInCodeEntry`](#dataincodeentry)

##### `impl CloneToUninit for DataInCodeEntry<E>`

- <span id="dataincodeentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DataInCodeEntry<E>`

##### `impl<E: fmt::Debug + Endian> Debug for DataInCodeEntry<E>`

- <span id="dataincodeentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DataInCodeEntry<E>`

- <span id="dataincodeentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DataInCodeEntry<E>`

- <span id="dataincodeentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for DataInCodeEntry<E>`

##### `impl ToOwned for DataInCodeEntry<E>`

- <span id="dataincodeentry-toowned-type-owned"></span>`type Owned = T`

- <span id="dataincodeentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dataincodeentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataInCodeEntry<E>`

- <span id="dataincodeentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dataincodeentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataInCodeEntry<E>`

- <span id="dataincodeentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dataincodeentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NoteCommand<E: Endian>`

```rust
struct NoteCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub data_owner: [u8; 16],
    pub offset: crate::endian::U64<E>,
    pub size: crate::endian::U64<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2618-2629`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2618-L2629)*

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_NOTE

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct NoteCommand)

- **`data_owner`**: `[u8; 16]`

  owner name for this LC_NOTE

- **`offset`**: `crate::endian::U64<E>`

  file offset of this data

- **`size`**: `crate::endian::U64<E>`

  length of data region

#### Trait Implementations

##### `impl Any for NoteCommand<E>`

- <span id="notecommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoteCommand<E>`

- <span id="notecommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoteCommand<E>`

- <span id="notecommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for NoteCommand<E>`

- <span id="notecommand-clone"></span>`fn clone(&self) -> NoteCommand<E>` — [`NoteCommand`](#notecommand)

##### `impl CloneToUninit for NoteCommand<E>`

- <span id="notecommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for NoteCommand<E>`

##### `impl<E: fmt::Debug + Endian> Debug for NoteCommand<E>`

- <span id="notecommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NoteCommand<E>`

- <span id="notecommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoteCommand<E>`

- <span id="notecommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for NoteCommand<E>`

##### `impl ToOwned for NoteCommand<E>`

- <span id="notecommand-toowned-type-owned"></span>`type Owned = T`

- <span id="notecommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="notecommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NoteCommand<E>`

- <span id="notecommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="notecommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoteCommand<E>`

- <span id="notecommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="notecommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Nlist32<E: Endian>`

```rust
struct Nlist32<E: Endian> {
    pub n_strx: crate::endian::U32<E>,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: crate::endian::U16<E>,
    pub n_value: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2635-2646`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2635-L2646)*

#### Fields

- **`n_strx`**: `crate::endian::U32<E>`

  index into the string table

- **`n_type`**: `u8`

  type flag, see below

- **`n_sect`**: `u8`

  section number or NO_SECT

- **`n_desc`**: `crate::endian::U16<E>`

  see <mach-o/stab.h>

- **`n_value`**: `crate::endian::U32<E>`

  value of this symbol (or stab offset)

#### Trait Implementations

##### `impl Any for Nlist32<E>`

- <span id="nlist32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Nlist32<E>`

- <span id="nlist32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Nlist32<E>`

- <span id="nlist32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for Nlist32<E>`

- <span id="nlist32-clone"></span>`fn clone(&self) -> Nlist32<E>` — [`Nlist32`](#nlist32)

##### `impl CloneToUninit for Nlist32<E>`

- <span id="nlist32-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for Nlist32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Nlist32<E>`

- <span id="nlist32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Nlist32<E>`

- <span id="nlist32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Nlist32<E>`

- <span id="nlist32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Endian: endian::Endian> Nlist for macho::Nlist32<Endian>`

- <span id="machonlist32-nlist-type-word"></span>`type Word = u32`

- <span id="machonlist32-nlist-type-endian"></span>`type Endian = Endian`

- <span id="machonlist32-nlist-n-strx"></span>`fn n_strx(&self, endian: <Self as >::Endian) -> u32` — [`Nlist`](../read/macho/index.md#nlist)

- <span id="machonlist32-nlist-n-type"></span>`fn n_type(&self) -> u8`

- <span id="machonlist32-nlist-n-sect"></span>`fn n_sect(&self) -> u8`

- <span id="machonlist32-nlist-n-desc"></span>`fn n_desc(&self, endian: <Self as >::Endian) -> u16` — [`Nlist`](../read/macho/index.md#nlist)

- <span id="machonlist32-nlist-n-value"></span>`fn n_value(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Nlist`](../read/macho/index.md#nlist)

##### `impl<E: Endian> Pod for Nlist32<E>`

##### `impl ToOwned for Nlist32<E>`

- <span id="nlist32-toowned-type-owned"></span>`type Owned = T`

- <span id="nlist32-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nlist32-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Nlist32<E>`

- <span id="nlist32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nlist32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Nlist32<E>`

- <span id="nlist32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nlist32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Nlist64<E: Endian>`

```rust
struct Nlist64<E: Endian> {
    pub n_strx: crate::endian::U32<E>,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: crate::endian::U16<E>,
    pub n_value: crate::endian::U64Bytes<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2653-2665`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2653-L2665)*

#### Fields

- **`n_strx`**: `crate::endian::U32<E>`

  index into the string table

- **`n_type`**: `u8`

  type flag, see below

- **`n_sect`**: `u8`

  section number or NO_SECT

- **`n_desc`**: `crate::endian::U16<E>`

  see <mach-o/stab.h>

- **`n_value`**: `crate::endian::U64Bytes<E>`

  value of this symbol (or stab offset)

#### Trait Implementations

##### `impl Any for Nlist64<E>`

- <span id="nlist64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Nlist64<E>`

- <span id="nlist64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Nlist64<E>`

- <span id="nlist64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for Nlist64<E>`

- <span id="nlist64-clone"></span>`fn clone(&self) -> Nlist64<E>` — [`Nlist64`](#nlist64)

##### `impl CloneToUninit for Nlist64<E>`

- <span id="nlist64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for Nlist64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Nlist64<E>`

- <span id="nlist64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Nlist64<E>`

- <span id="nlist64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Nlist64<E>`

- <span id="nlist64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Endian: endian::Endian> Nlist for macho::Nlist64<Endian>`

- <span id="machonlist64-nlist-type-word"></span>`type Word = u64`

- <span id="machonlist64-nlist-type-endian"></span>`type Endian = Endian`

- <span id="machonlist64-nlist-n-strx"></span>`fn n_strx(&self, endian: <Self as >::Endian) -> u32` — [`Nlist`](../read/macho/index.md#nlist)

- <span id="machonlist64-nlist-n-type"></span>`fn n_type(&self) -> u8`

- <span id="machonlist64-nlist-n-sect"></span>`fn n_sect(&self) -> u8`

- <span id="machonlist64-nlist-n-desc"></span>`fn n_desc(&self, endian: <Self as >::Endian) -> u16` — [`Nlist`](../read/macho/index.md#nlist)

- <span id="machonlist64-nlist-n-value"></span>`fn n_value(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Nlist`](../read/macho/index.md#nlist)

##### `impl<E: Endian> Pod for Nlist64<E>`

##### `impl ToOwned for Nlist64<E>`

- <span id="nlist64-toowned-type-owned"></span>`type Owned = T`

- <span id="nlist64-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nlist64-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Nlist64<E>`

- <span id="nlist64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nlist64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Nlist64<E>`

- <span id="nlist64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nlist64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Relocation<E: Endian>`

```rust
struct Relocation<E: Endian> {
    pub r_word0: crate::endian::U32<E>,
    pub r_word1: crate::endian::U32<E>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:2996-2999`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2996-L2999)*

A relocation entry.

Mach-O relocations have plain and scattered variants, with the
meaning of the fields depending on the variant.

This type provides functions for determining whether the relocation
is scattered, and for accessing the fields of each variant.

#### Implementations

- <span id="relocation-r-scattered"></span>`fn r_scattered(self, endian: E, cputype: u32) -> bool`

  Determine whether this is a scattered relocation.

- <span id="relocation-info"></span>`fn info(self, endian: E) -> RelocationInfo` — [`RelocationInfo`](#relocationinfo)

  Return the fields of a plain relocation.

- <span id="relocation-scattered-info"></span>`fn scattered_info(self, endian: E) -> ScatteredRelocationInfo` — [`ScatteredRelocationInfo`](#scatteredrelocationinfo)

  Return the fields of a scattered relocation.

#### Trait Implementations

##### `impl Any for Relocation<E>`

- <span id="relocation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Relocation<E>`

- <span id="relocation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Relocation<E>`

- <span id="relocation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for Relocation<E>`

- <span id="relocation-clone"></span>`fn clone(&self) -> Relocation<E>` — [`Relocation`](#relocation)

##### `impl CloneToUninit for Relocation<E>`

- <span id="relocation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for Relocation<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Relocation<E>`

- <span id="relocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Relocation<E>`

- <span id="relocation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Relocation<E>`

- <span id="relocation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: Endian> Pod for Relocation<E>`

##### `impl ToOwned for Relocation<E>`

- <span id="relocation-toowned-type-owned"></span>`type Owned = T`

- <span id="relocation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Relocation<E>`

- <span id="relocation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Relocation<E>`

- <span id="relocation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationInfo`

```rust
struct RelocationInfo {
    pub r_address: u32,
    pub r_symbolnum: u32,
    pub r_pcrel: bool,
    pub r_length: u8,
    pub r_extern: bool,
    pub r_type: u8,
}
```

*Defined in [`object-0.37.3/src/macho.rs:3061-3074`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3061-L3074)*

#### Fields

- **`r_address`**: `u32`

  offset in the section to what is being relocated

- **`r_symbolnum`**: `u32`

  symbol index if r_extern == 1 or section ordinal if r_extern == 0

- **`r_pcrel`**: `bool`

  was relocated pc relative already

- **`r_length`**: `u8`

  0=byte, 1=word, 2=long, 3=quad

- **`r_extern`**: `bool`

  does not include value of sym referenced

- **`r_type`**: `u8`

  if not 0, machine specific relocation type

#### Implementations

- <span id="relocationinfo-relocation"></span>`fn relocation<E: Endian>(self, endian: E) -> Relocation<E>` — [`Relocation`](#relocation)

  Combine the fields into a `Relocation`.

#### Trait Implementations

##### `impl Any for RelocationInfo`

- <span id="relocationinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationInfo`

- <span id="relocationinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationInfo`

- <span id="relocationinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationInfo`

- <span id="relocationinfo-clone"></span>`fn clone(&self) -> RelocationInfo` — [`RelocationInfo`](#relocationinfo)

##### `impl CloneToUninit for RelocationInfo`

- <span id="relocationinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationInfo`

##### `impl Debug for RelocationInfo`

- <span id="relocationinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RelocationInfo`

- <span id="relocationinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationInfo`

- <span id="relocationinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RelocationInfo`

- <span id="relocationinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationInfo`

- <span id="relocationinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationInfo`

- <span id="relocationinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScatteredRelocationInfo`

```rust
struct ScatteredRelocationInfo {
    pub r_address: u32,
    pub r_type: u8,
    pub r_length: u8,
    pub r_pcrel: bool,
    pub r_value: u32,
}
```

*Defined in [`object-0.37.3/src/macho.rs:3170-3181`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3170-L3181)*

#### Fields

- **`r_address`**: `u32`

  offset in the section to what is being relocated

- **`r_type`**: `u8`

  if not 0, machine specific relocation type

- **`r_length`**: `u8`

  0=byte, 1=word, 2=long, 3=quad

- **`r_pcrel`**: `bool`

  was relocated pc relative already

- **`r_value`**: `u32`

  the value the item to be relocated is referring to (without any offset added)

#### Implementations

- <span id="scatteredrelocationinfo-relocation"></span>`fn relocation<E: Endian>(self, endian: E) -> Relocation<E>` — [`Relocation`](#relocation)

  Combine the fields into a `Relocation`.

#### Trait Implementations

##### `impl Any for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-clone"></span>`fn clone(&self) -> ScatteredRelocationInfo` — [`ScatteredRelocationInfo`](#scatteredrelocationinfo)

##### `impl CloneToUninit for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ScatteredRelocationInfo`

##### `impl Debug for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="scatteredrelocationinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="scatteredrelocationinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scatteredrelocationinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScatteredRelocationInfo`

- <span id="scatteredrelocationinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scatteredrelocationinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `PtrauthKey`

```rust
enum PtrauthKey {
    IA,
    IB,
    DA,
    DB,
}
```

*Defined in [`object-0.37.3/src/macho.rs:293-302`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L293-L302)*

The key used to sign a pointer for authentication.

The variant values correspond to the values used in the
`ptrauth_key` enum in `ptrauth.h`.

#### Variants

- **`IA`**

  Instruction key A.

- **`IB`**

  Instruction key B.

- **`DA`**

  Data key A.

- **`DB`**

  Data key B.

#### Trait Implementations

##### `impl Any for PtrauthKey`

- <span id="ptrauthkey-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PtrauthKey`

- <span id="ptrauthkey-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PtrauthKey`

- <span id="ptrauthkey-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PtrauthKey`

- <span id="ptrauthkey-clone"></span>`fn clone(&self) -> PtrauthKey` — [`PtrauthKey`](#ptrauthkey)

##### `impl CloneToUninit for PtrauthKey`

- <span id="ptrauthkey-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PtrauthKey`

##### `impl Debug for PtrauthKey`

- <span id="ptrauthkey-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PtrauthKey`

##### `impl<T> From for PtrauthKey`

- <span id="ptrauthkey-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PtrauthKey`

- <span id="ptrauthkey-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PtrauthKey`

- <span id="ptrauthkey-partialeq-eq"></span>`fn eq(&self, other: &PtrauthKey) -> bool` — [`PtrauthKey`](#ptrauthkey)

##### `impl StructuralPartialEq for PtrauthKey`

##### `impl ToOwned for PtrauthKey`

- <span id="ptrauthkey-toowned-type-owned"></span>`type Owned = T`

- <span id="ptrauthkey-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ptrauthkey-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PtrauthKey`

- <span id="ptrauthkey-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ptrauthkey-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PtrauthKey`

- <span id="ptrauthkey-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ptrauthkey-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `cpu_subtype_intel`

```rust
const fn cpu_subtype_intel(f: u32, m: u32) -> u32
```

*Defined in [`object-0.37.3/src/macho.rs:126-128`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L126-L128)*

### `cpu_subtype_intel_family`

```rust
const fn cpu_subtype_intel_family(x: u32) -> u32
```

*Defined in [`object-0.37.3/src/macho.rs:153-155`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L153-L155)*

### `cpu_subtype_intel_model`

```rust
const fn cpu_subtype_intel_model(x: u32) -> u32
```

*Defined in [`object-0.37.3/src/macho.rs:159-161`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L159-L161)*

## Constants

### `CPU_ARCH_MASK`
```rust
const CPU_ARCH_MASK: u32 = 4_278_190_080u32;
```

*Defined in [`object-0.37.3/src/macho.rs:20`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L20)*

mask for architecture bits

### `CPU_ARCH_ABI64`
```rust
const CPU_ARCH_ABI64: u32 = 16_777_216u32;
```

*Defined in [`object-0.37.3/src/macho.rs:22`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L22)*

64 bit ABI

### `CPU_ARCH_ABI64_32`
```rust
const CPU_ARCH_ABI64_32: u32 = 33_554_432u32;
```

*Defined in [`object-0.37.3/src/macho.rs:24`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L24)*

ABI for 64-bit hardware with 32-bit types; LP32

### `CPU_TYPE_ANY`
```rust
const CPU_TYPE_ANY: u32 = 4_294_967_295u32;
```

*Defined in [`object-0.37.3/src/macho.rs:30`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L30)*

### `CPU_TYPE_VAX`
```rust
const CPU_TYPE_VAX: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:32`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L32)*

### `CPU_TYPE_MC680X0`
```rust
const CPU_TYPE_MC680X0: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:33`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L33)*

### `CPU_TYPE_X86`
```rust
const CPU_TYPE_X86: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:34`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L34)*

### `CPU_TYPE_X86_64`
```rust
const CPU_TYPE_X86_64: u32 = 16_777_223u32;
```

*Defined in [`object-0.37.3/src/macho.rs:35`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L35)*

### `CPU_TYPE_MIPS`
```rust
const CPU_TYPE_MIPS: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:36`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L36)*

### `CPU_TYPE_MC98000`
```rust
const CPU_TYPE_MC98000: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:37`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L37)*

### `CPU_TYPE_HPPA`
```rust
const CPU_TYPE_HPPA: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:38`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L38)*

### `CPU_TYPE_ARM`
```rust
const CPU_TYPE_ARM: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/macho.rs:39`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L39)*

### `CPU_TYPE_ARM64`
```rust
const CPU_TYPE_ARM64: u32 = 16_777_228u32;
```

*Defined in [`object-0.37.3/src/macho.rs:40`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L40)*

### `CPU_TYPE_ARM64_32`
```rust
const CPU_TYPE_ARM64_32: u32 = 33_554_444u32;
```

*Defined in [`object-0.37.3/src/macho.rs:41`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L41)*

### `CPU_TYPE_MC88000`
```rust
const CPU_TYPE_MC88000: u32 = 13u32;
```

*Defined in [`object-0.37.3/src/macho.rs:42`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L42)*

### `CPU_TYPE_SPARC`
```rust
const CPU_TYPE_SPARC: u32 = 14u32;
```

*Defined in [`object-0.37.3/src/macho.rs:43`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L43)*

### `CPU_TYPE_I860`
```rust
const CPU_TYPE_I860: u32 = 15u32;
```

*Defined in [`object-0.37.3/src/macho.rs:44`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L44)*

### `CPU_TYPE_ALPHA`
```rust
const CPU_TYPE_ALPHA: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/macho.rs:45`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L45)*

### `CPU_TYPE_POWERPC`
```rust
const CPU_TYPE_POWERPC: u32 = 18u32;
```

*Defined in [`object-0.37.3/src/macho.rs:46`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L46)*

### `CPU_TYPE_POWERPC64`
```rust
const CPU_TYPE_POWERPC64: u32 = 16_777_234u32;
```

*Defined in [`object-0.37.3/src/macho.rs:47`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L47)*

### `CPU_SUBTYPE_MASK`
```rust
const CPU_SUBTYPE_MASK: u32 = 4_278_190_080u32;
```

*Defined in [`object-0.37.3/src/macho.rs:53`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L53)*

mask for feature flags

### `CPU_SUBTYPE_LIB64`
```rust
const CPU_SUBTYPE_LIB64: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/macho.rs:55`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L55)*

64 bit libraries

### `CPU_SUBTYPE_PTRAUTH_ABI`
```rust
const CPU_SUBTYPE_PTRAUTH_ABI: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/macho.rs:57`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L57)*

pointer authentication with versioned ABI

### `CPU_SUBTYPE_ANY`
```rust
const CPU_SUBTYPE_ANY: u32 = 4_294_967_295u32;
```

*Defined in [`object-0.37.3/src/macho.rs:62`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L62)*

When selecting a slice, ANY will pick the slice with the best
grading for the selected cpu_type_t, unlike the "ALL" subtypes,
which are the slices that can run on any hardware for that cpu type.

### `CPU_SUBTYPE_MULTIPLE`
```rust
const CPU_SUBTYPE_MULTIPLE: u32 = 4_294_967_295u32;
```

*Defined in [`object-0.37.3/src/macho.rs:75`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L75)*

### `CPU_SUBTYPE_LITTLE_ENDIAN`
```rust
const CPU_SUBTYPE_LITTLE_ENDIAN: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:76`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L76)*

### `CPU_SUBTYPE_BIG_ENDIAN`
```rust
const CPU_SUBTYPE_BIG_ENDIAN: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:77`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L77)*

### `CPU_SUBTYPE_VAX_ALL`
```rust
const CPU_SUBTYPE_VAX_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:84`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L84)*

### `CPU_SUBTYPE_VAX780`
```rust
const CPU_SUBTYPE_VAX780: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:85`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L85)*

### `CPU_SUBTYPE_VAX785`
```rust
const CPU_SUBTYPE_VAX785: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:86`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L86)*

### `CPU_SUBTYPE_VAX750`
```rust
const CPU_SUBTYPE_VAX750: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:87`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L87)*

### `CPU_SUBTYPE_VAX730`
```rust
const CPU_SUBTYPE_VAX730: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:88`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L88)*

### `CPU_SUBTYPE_UVAXI`
```rust
const CPU_SUBTYPE_UVAXI: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:89`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L89)*

### `CPU_SUBTYPE_UVAXII`
```rust
const CPU_SUBTYPE_UVAXII: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:90`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L90)*

### `CPU_SUBTYPE_VAX8200`
```rust
const CPU_SUBTYPE_VAX8200: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:91`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L91)*

### `CPU_SUBTYPE_VAX8500`
```rust
const CPU_SUBTYPE_VAX8500: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:92`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L92)*

### `CPU_SUBTYPE_VAX8600`
```rust
const CPU_SUBTYPE_VAX8600: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/macho.rs:93`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L93)*

### `CPU_SUBTYPE_VAX8650`
```rust
const CPU_SUBTYPE_VAX8650: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:94`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L94)*

### `CPU_SUBTYPE_VAX8800`
```rust
const CPU_SUBTYPE_VAX8800: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:95`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L95)*

### `CPU_SUBTYPE_UVAXIII`
```rust
const CPU_SUBTYPE_UVAXIII: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/macho.rs:96`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L96)*

### `CPU_SUBTYPE_MC680X0_ALL`
```rust
const CPU_SUBTYPE_MC680X0_ALL: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:115`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L115)*

### `CPU_SUBTYPE_MC68030`
```rust
const CPU_SUBTYPE_MC68030: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:117`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L117)*

### `CPU_SUBTYPE_MC68040`
```rust
const CPU_SUBTYPE_MC68040: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:118`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L118)*

### `CPU_SUBTYPE_MC68030_ONLY`
```rust
const CPU_SUBTYPE_MC68030_ONLY: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:119`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L119)*

### `CPU_SUBTYPE_I386_ALL`
```rust
const CPU_SUBTYPE_I386_ALL: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:130`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L130)*

### `CPU_SUBTYPE_386`
```rust
const CPU_SUBTYPE_386: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:131`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L131)*

### `CPU_SUBTYPE_486`
```rust
const CPU_SUBTYPE_486: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:132`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L132)*

### `CPU_SUBTYPE_486SX`
```rust
const CPU_SUBTYPE_486SX: u32 = 132u32;
```

*Defined in [`object-0.37.3/src/macho.rs:133`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L133)*

### `CPU_SUBTYPE_586`
```rust
const CPU_SUBTYPE_586: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:134`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L134)*

### `CPU_SUBTYPE_PENT`
```rust
const CPU_SUBTYPE_PENT: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:135`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L135)*

### `CPU_SUBTYPE_PENTPRO`
```rust
const CPU_SUBTYPE_PENTPRO: u32 = 22u32;
```

*Defined in [`object-0.37.3/src/macho.rs:136`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L136)*

### `CPU_SUBTYPE_PENTII_M3`
```rust
const CPU_SUBTYPE_PENTII_M3: u32 = 54u32;
```

*Defined in [`object-0.37.3/src/macho.rs:137`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L137)*

### `CPU_SUBTYPE_PENTII_M5`
```rust
const CPU_SUBTYPE_PENTII_M5: u32 = 86u32;
```

*Defined in [`object-0.37.3/src/macho.rs:138`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L138)*

### `CPU_SUBTYPE_CELERON`
```rust
const CPU_SUBTYPE_CELERON: u32 = 103u32;
```

*Defined in [`object-0.37.3/src/macho.rs:139`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L139)*

### `CPU_SUBTYPE_CELERON_MOBILE`
```rust
const CPU_SUBTYPE_CELERON_MOBILE: u32 = 119u32;
```

*Defined in [`object-0.37.3/src/macho.rs:140`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L140)*

### `CPU_SUBTYPE_PENTIUM_3`
```rust
const CPU_SUBTYPE_PENTIUM_3: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:141`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L141)*

### `CPU_SUBTYPE_PENTIUM_3_M`
```rust
const CPU_SUBTYPE_PENTIUM_3_M: u32 = 24u32;
```

*Defined in [`object-0.37.3/src/macho.rs:142`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L142)*

### `CPU_SUBTYPE_PENTIUM_3_XEON`
```rust
const CPU_SUBTYPE_PENTIUM_3_XEON: u32 = 40u32;
```

*Defined in [`object-0.37.3/src/macho.rs:143`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L143)*

### `CPU_SUBTYPE_PENTIUM_M`
```rust
const CPU_SUBTYPE_PENTIUM_M: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/macho.rs:144`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L144)*

### `CPU_SUBTYPE_PENTIUM_4`
```rust
const CPU_SUBTYPE_PENTIUM_4: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:145`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L145)*

### `CPU_SUBTYPE_PENTIUM_4_M`
```rust
const CPU_SUBTYPE_PENTIUM_4_M: u32 = 26u32;
```

*Defined in [`object-0.37.3/src/macho.rs:146`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L146)*

### `CPU_SUBTYPE_ITANIUM`
```rust
const CPU_SUBTYPE_ITANIUM: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:147`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L147)*

### `CPU_SUBTYPE_ITANIUM_2`
```rust
const CPU_SUBTYPE_ITANIUM_2: u32 = 27u32;
```

*Defined in [`object-0.37.3/src/macho.rs:148`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L148)*

### `CPU_SUBTYPE_XEON`
```rust
const CPU_SUBTYPE_XEON: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/macho.rs:149`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L149)*

### `CPU_SUBTYPE_XEON_MP`
```rust
const CPU_SUBTYPE_XEON_MP: u32 = 28u32;
```

*Defined in [`object-0.37.3/src/macho.rs:150`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L150)*

### `CPU_SUBTYPE_INTEL_FAMILY_MAX`
```rust
const CPU_SUBTYPE_INTEL_FAMILY_MAX: u32 = 15u32;
```

*Defined in [`object-0.37.3/src/macho.rs:156`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L156)*

### `CPU_SUBTYPE_INTEL_MODEL_ALL`
```rust
const CPU_SUBTYPE_INTEL_MODEL_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:162`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L162)*

### `CPU_SUBTYPE_X86_ALL`
```rust
const CPU_SUBTYPE_X86_ALL: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:168`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L168)*

### `CPU_SUBTYPE_X86_64_ALL`
```rust
const CPU_SUBTYPE_X86_64_ALL: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:169`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L169)*

### `CPU_SUBTYPE_X86_ARCH1`
```rust
const CPU_SUBTYPE_X86_ARCH1: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:170`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L170)*

### `CPU_SUBTYPE_X86_64_H`
```rust
const CPU_SUBTYPE_X86_64_H: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:172`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L172)*

Haswell feature subset

### `CPU_SUBTYPE_MIPS_ALL`
```rust
const CPU_SUBTYPE_MIPS_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:178`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L178)*

### `CPU_SUBTYPE_MIPS_R2300`
```rust
const CPU_SUBTYPE_MIPS_R2300: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:179`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L179)*

### `CPU_SUBTYPE_MIPS_R2600`
```rust
const CPU_SUBTYPE_MIPS_R2600: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:180`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L180)*

### `CPU_SUBTYPE_MIPS_R2800`
```rust
const CPU_SUBTYPE_MIPS_R2800: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:181`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L181)*

### `CPU_SUBTYPE_MIPS_R2000A`
```rust
const CPU_SUBTYPE_MIPS_R2000A: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:183`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L183)*

pmax

### `CPU_SUBTYPE_MIPS_R2000`
```rust
const CPU_SUBTYPE_MIPS_R2000: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:184`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L184)*

### `CPU_SUBTYPE_MIPS_R3000A`
```rust
const CPU_SUBTYPE_MIPS_R3000A: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:186`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L186)*

3max

### `CPU_SUBTYPE_MIPS_R3000`
```rust
const CPU_SUBTYPE_MIPS_R3000: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:187`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L187)*

### `CPU_SUBTYPE_MC98000_ALL`
```rust
const CPU_SUBTYPE_MC98000_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:192`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L192)*

### `CPU_SUBTYPE_MC98601`
```rust
const CPU_SUBTYPE_MC98601: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:193`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L193)*

### `CPU_SUBTYPE_HPPA_ALL`
```rust
const CPU_SUBTYPE_HPPA_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:200`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L200)*

### `CPU_SUBTYPE_HPPA_7100LC`
```rust
const CPU_SUBTYPE_HPPA_7100LC: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:201`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L201)*

### `CPU_SUBTYPE_MC88000_ALL`
```rust
const CPU_SUBTYPE_MC88000_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:206`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L206)*

### `CPU_SUBTYPE_MC88100`
```rust
const CPU_SUBTYPE_MC88100: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:207`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L207)*

### `CPU_SUBTYPE_MC88110`
```rust
const CPU_SUBTYPE_MC88110: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:208`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L208)*

### `CPU_SUBTYPE_SPARC_ALL`
```rust
const CPU_SUBTYPE_SPARC_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:213`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L213)*

### `CPU_SUBTYPE_I860_ALL`
```rust
const CPU_SUBTYPE_I860_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:218`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L218)*

### `CPU_SUBTYPE_I860_860`
```rust
const CPU_SUBTYPE_I860_860: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:219`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L219)*

### `CPU_SUBTYPE_POWERPC_ALL`
```rust
const CPU_SUBTYPE_POWERPC_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:224`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L224)*

### `CPU_SUBTYPE_POWERPC_601`
```rust
const CPU_SUBTYPE_POWERPC_601: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:225`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L225)*

### `CPU_SUBTYPE_POWERPC_602`
```rust
const CPU_SUBTYPE_POWERPC_602: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:226`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L226)*

### `CPU_SUBTYPE_POWERPC_603`
```rust
const CPU_SUBTYPE_POWERPC_603: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:227`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L227)*

### `CPU_SUBTYPE_POWERPC_603E`
```rust
const CPU_SUBTYPE_POWERPC_603E: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:228`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L228)*

### `CPU_SUBTYPE_POWERPC_603EV`
```rust
const CPU_SUBTYPE_POWERPC_603EV: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:229`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L229)*

### `CPU_SUBTYPE_POWERPC_604`
```rust
const CPU_SUBTYPE_POWERPC_604: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:230`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L230)*

### `CPU_SUBTYPE_POWERPC_604E`
```rust
const CPU_SUBTYPE_POWERPC_604E: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:231`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L231)*

### `CPU_SUBTYPE_POWERPC_620`
```rust
const CPU_SUBTYPE_POWERPC_620: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:232`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L232)*

### `CPU_SUBTYPE_POWERPC_750`
```rust
const CPU_SUBTYPE_POWERPC_750: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/macho.rs:233`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L233)*

### `CPU_SUBTYPE_POWERPC_7400`
```rust
const CPU_SUBTYPE_POWERPC_7400: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:234`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L234)*

### `CPU_SUBTYPE_POWERPC_7450`
```rust
const CPU_SUBTYPE_POWERPC_7450: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:235`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L235)*

### `CPU_SUBTYPE_POWERPC_970`
```rust
const CPU_SUBTYPE_POWERPC_970: u32 = 100u32;
```

*Defined in [`object-0.37.3/src/macho.rs:236`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L236)*

### `CPU_SUBTYPE_ARM_ALL`
```rust
const CPU_SUBTYPE_ARM_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:241`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L241)*

### `CPU_SUBTYPE_ARM_V4T`
```rust
const CPU_SUBTYPE_ARM_V4T: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:242`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L242)*

### `CPU_SUBTYPE_ARM_V6`
```rust
const CPU_SUBTYPE_ARM_V6: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:243`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L243)*

### `CPU_SUBTYPE_ARM_V5TEJ`
```rust
const CPU_SUBTYPE_ARM_V5TEJ: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:244`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L244)*

### `CPU_SUBTYPE_ARM_XSCALE`
```rust
const CPU_SUBTYPE_ARM_XSCALE: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:245`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L245)*

### `CPU_SUBTYPE_ARM_V7`
```rust
const CPU_SUBTYPE_ARM_V7: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/macho.rs:247`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L247)*

ARMv7-A and ARMv7-R

### `CPU_SUBTYPE_ARM_V7F`
```rust
const CPU_SUBTYPE_ARM_V7F: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:249`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L249)*

Cortex A9

### `CPU_SUBTYPE_ARM_V7S`
```rust
const CPU_SUBTYPE_ARM_V7S: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:251`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L251)*

Swift

### `CPU_SUBTYPE_ARM_V7K`
```rust
const CPU_SUBTYPE_ARM_V7K: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/macho.rs:252`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L252)*

### `CPU_SUBTYPE_ARM_V8`
```rust
const CPU_SUBTYPE_ARM_V8: u32 = 13u32;
```

*Defined in [`object-0.37.3/src/macho.rs:253`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L253)*

### `CPU_SUBTYPE_ARM_V6M`
```rust
const CPU_SUBTYPE_ARM_V6M: u32 = 14u32;
```

*Defined in [`object-0.37.3/src/macho.rs:255`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L255)*

Not meant to be run under xnu

### `CPU_SUBTYPE_ARM_V7M`
```rust
const CPU_SUBTYPE_ARM_V7M: u32 = 15u32;
```

*Defined in [`object-0.37.3/src/macho.rs:257`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L257)*

Not meant to be run under xnu

### `CPU_SUBTYPE_ARM_V7EM`
```rust
const CPU_SUBTYPE_ARM_V7EM: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/macho.rs:259`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L259)*

Not meant to be run under xnu

### `CPU_SUBTYPE_ARM_V8M`
```rust
const CPU_SUBTYPE_ARM_V8M: u32 = 17u32;
```

*Defined in [`object-0.37.3/src/macho.rs:261`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L261)*

Not meant to be run under xnu

### `CPU_SUBTYPE_ARM64_ALL`
```rust
const CPU_SUBTYPE_ARM64_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:266`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L266)*

### `CPU_SUBTYPE_ARM64_V8`
```rust
const CPU_SUBTYPE_ARM64_V8: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:267`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L267)*

### `CPU_SUBTYPE_ARM64E`
```rust
const CPU_SUBTYPE_ARM64E: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:268`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L268)*

### `CPU_SUBTYPE_ARM64_32_ALL`
```rust
const CPU_SUBTYPE_ARM64_32_ALL: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:273`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L273)*

### `CPU_SUBTYPE_ARM64_32_V8`
```rust
const CPU_SUBTYPE_ARM64_32_V8: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:274`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L274)*

### `VM_PROT_READ`
```rust
const VM_PROT_READ: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:279`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L279)*

read permission

### `VM_PROT_WRITE`
```rust
const VM_PROT_WRITE: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:281`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L281)*

write permission

### `VM_PROT_EXECUTE`
```rust
const VM_PROT_EXECUTE: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:283`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L283)*

execute permission

### `DYLD_CACHE_MAPPING_AUTH_DATA`
```rust
const DYLD_CACHE_MAPPING_AUTH_DATA: u64 = 1u64;
```

*Defined in [`object-0.37.3/src/macho.rs:468`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L468)*

### `DYLD_CACHE_MAPPING_DIRTY_DATA`
```rust
const DYLD_CACHE_MAPPING_DIRTY_DATA: u64 = 2u64;
```

*Defined in [`object-0.37.3/src/macho.rs:469`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L469)*

### `DYLD_CACHE_MAPPING_CONST_DATA`
```rust
const DYLD_CACHE_MAPPING_CONST_DATA: u64 = 4u64;
```

*Defined in [`object-0.37.3/src/macho.rs:470`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L470)*

### `DYLD_CACHE_MAPPING_TEXT_STUBS`
```rust
const DYLD_CACHE_MAPPING_TEXT_STUBS: u64 = 8u64;
```

*Defined in [`object-0.37.3/src/macho.rs:471`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L471)*

### `DYLD_CACHE_DYNAMIC_CONFIG_DATA`
```rust
const DYLD_CACHE_DYNAMIC_CONFIG_DATA: u64 = 16u64;
```

*Defined in [`object-0.37.3/src/macho.rs:472`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L472)*

### `DYLD_CACHE_SLIDE_PAGE_ATTRS`
```rust
const DYLD_CACHE_SLIDE_PAGE_ATTRS: u16 = 49_152u16;
```

*Defined in [`object-0.37.3/src/macho.rs:513`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L513)*

### `DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA`
```rust
const DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/macho.rs:515`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L515)*

### `DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE`
```rust
const DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE: u16 = 16_384u16;
```

*Defined in [`object-0.37.3/src/macho.rs:517`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L517)*

### `DYLD_CACHE_SLIDE_PAGE_ATTR_END`
```rust
const DYLD_CACHE_SLIDE_PAGE_ATTR_END: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/macho.rs:519`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L519)*

### `DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE`
```rust
const DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE: u16 = 65_535u16;
```

*Defined in [`object-0.37.3/src/macho.rs:533`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L533)*

Page has no rebasing.

### `DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE`
```rust
const DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE: u16 = 65_535u16;
```

*Defined in [`object-0.37.3/src/macho.rs:607`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L607)*

Page has no rebasing.

### `FAT_MAGIC`
```rust
const FAT_MAGIC: u32 = 3_405_691_582u32;
```

*Defined in [`object-0.37.3/src/macho.rs:702`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L702)*

### `FAT_CIGAM`
```rust
const FAT_CIGAM: u32 = 3_199_925_962u32;
```

*Defined in [`object-0.37.3/src/macho.rs:704`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L704)*

NXSwapLong(FAT_MAGIC)

### `FAT_MAGIC_64`
```rust
const FAT_MAGIC_64: u32 = 3_405_691_583u32;
```

*Defined in [`object-0.37.3/src/macho.rs:737`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L737)*

### `FAT_CIGAM_64`
```rust
const FAT_CIGAM_64: u32 = 3_216_703_178u32;
```

*Defined in [`object-0.37.3/src/macho.rs:739`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L739)*

NXSwapLong(FAT_MAGIC_64)

### `MH_MAGIC`
```rust
const MH_MAGIC: u32 = 4_277_009_102u32;
```

*Defined in [`object-0.37.3/src/macho.rs:784`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L784)*

the mach magic number

### `MH_CIGAM`
```rust
const MH_CIGAM: u32 = 3_472_551_422u32;
```

*Defined in [`object-0.37.3/src/macho.rs:786`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L786)*

NXSwapInt(MH_MAGIC)

### `MH_MAGIC_64`
```rust
const MH_MAGIC_64: u32 = 4_277_009_103u32;
```

*Defined in [`object-0.37.3/src/macho.rs:814`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L814)*

the 64-bit mach magic number

### `MH_CIGAM_64`
```rust
const MH_CIGAM_64: u32 = 3_489_328_638u32;
```

*Defined in [`object-0.37.3/src/macho.rs:816`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L816)*

NXSwapInt(MH_MAGIC_64)

### `MH_OBJECT`
```rust
const MH_OBJECT: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:842`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L842)*

relocatable object file

### `MH_EXECUTE`
```rust
const MH_EXECUTE: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:844`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L844)*

demand paged executable file

### `MH_FVMLIB`
```rust
const MH_FVMLIB: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:846`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L846)*

fixed VM shared library file

### `MH_CORE`
```rust
const MH_CORE: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:848`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L848)*

core file

### `MH_PRELOAD`
```rust
const MH_PRELOAD: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:850`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L850)*

preloaded executable file

### `MH_DYLIB`
```rust
const MH_DYLIB: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:852`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L852)*

dynamically bound shared library

### `MH_DYLINKER`
```rust
const MH_DYLINKER: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:854`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L854)*

dynamic link editor

### `MH_BUNDLE`
```rust
const MH_BUNDLE: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:856`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L856)*

dynamically bound bundle file

### `MH_DYLIB_STUB`
```rust
const MH_DYLIB_STUB: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/macho.rs:858`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L858)*

shared library stub for static linking only, no section contents

### `MH_DSYM`
```rust
const MH_DSYM: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:860`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L860)*

companion file with only debug sections

### `MH_KEXT_BUNDLE`
```rust
const MH_KEXT_BUNDLE: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:862`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L862)*

x86_64 kexts

### `MH_FILESET`
```rust
const MH_FILESET: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/macho.rs:864`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L864)*

set of mach-o's

### `MH_NOUNDEFS`
```rust
const MH_NOUNDEFS: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:868`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L868)*

the object file has no undefined references

### `MH_INCRLINK`
```rust
const MH_INCRLINK: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:870`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L870)*

the object file is the output of an incremental link against a base file and can't be link edited again

### `MH_DYLDLINK`
```rust
const MH_DYLDLINK: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:872`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L872)*

the object file is input for the dynamic linker and can't be statically link edited again

### `MH_BINDATLOAD`
```rust
const MH_BINDATLOAD: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:874`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L874)*

the object file's undefined references are bound by the dynamic linker when loaded.

### `MH_PREBOUND`
```rust
const MH_PREBOUND: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/macho.rs:876`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L876)*

the file has its dynamic undefined references prebound.

### `MH_SPLIT_SEGS`
```rust
const MH_SPLIT_SEGS: u32 = 32u32;
```

*Defined in [`object-0.37.3/src/macho.rs:878`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L878)*

the file has its read-only and read-write segments split

### `MH_LAZY_INIT`
```rust
const MH_LAZY_INIT: u32 = 64u32;
```

*Defined in [`object-0.37.3/src/macho.rs:880`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L880)*

the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete)

### `MH_TWOLEVEL`
```rust
const MH_TWOLEVEL: u32 = 128u32;
```

*Defined in [`object-0.37.3/src/macho.rs:882`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L882)*

the image is using two-level name space bindings

### `MH_FORCE_FLAT`
```rust
const MH_FORCE_FLAT: u32 = 256u32;
```

*Defined in [`object-0.37.3/src/macho.rs:884`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L884)*

the executable is forcing all images to use flat name space bindings

### `MH_NOMULTIDEFS`
```rust
const MH_NOMULTIDEFS: u32 = 512u32;
```

*Defined in [`object-0.37.3/src/macho.rs:886`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L886)*

this umbrella guarantees no multiple definitions of symbols in its sub-images so the two-level namespace hints can always be used.

### `MH_NOFIXPREBINDING`
```rust
const MH_NOFIXPREBINDING: u32 = 1_024u32;
```

*Defined in [`object-0.37.3/src/macho.rs:888`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L888)*

do not have dyld notify the prebinding agent about this executable

### `MH_PREBINDABLE`
```rust
const MH_PREBINDABLE: u32 = 2_048u32;
```

*Defined in [`object-0.37.3/src/macho.rs:890`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L890)*

the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.

### `MH_ALLMODSBOUND`
```rust
const MH_ALLMODSBOUND: u32 = 4_096u32;
```

*Defined in [`object-0.37.3/src/macho.rs:892`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L892)*

indicates that this binary binds to all two-level namespace modules of its dependent libraries. only used when MH_PREBINDABLE and MH_TWOLEVEL are both set.

### `MH_SUBSECTIONS_VIA_SYMBOLS`
```rust
const MH_SUBSECTIONS_VIA_SYMBOLS: u32 = 8_192u32;
```

*Defined in [`object-0.37.3/src/macho.rs:894`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L894)*

safe to divide up the sections into sub-sections via symbols for dead code stripping

### `MH_CANONICAL`
```rust
const MH_CANONICAL: u32 = 16_384u32;
```

*Defined in [`object-0.37.3/src/macho.rs:896`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L896)*

the binary has been canonicalized via the unprebind operation

### `MH_WEAK_DEFINES`
```rust
const MH_WEAK_DEFINES: u32 = 32_768u32;
```

*Defined in [`object-0.37.3/src/macho.rs:898`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L898)*

the final linked image contains external weak symbols

### `MH_BINDS_TO_WEAK`
```rust
const MH_BINDS_TO_WEAK: u32 = 65_536u32;
```

*Defined in [`object-0.37.3/src/macho.rs:900`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L900)*

the final linked image uses weak symbols

### `MH_ALLOW_STACK_EXECUTION`
```rust
const MH_ALLOW_STACK_EXECUTION: u32 = 131_072u32;
```

*Defined in [`object-0.37.3/src/macho.rs:902`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L902)*

When this bit is set, all stacks in the task will be given stack execution privilege.  Only used in MH_EXECUTE filetypes.

### `MH_ROOT_SAFE`
```rust
const MH_ROOT_SAFE: u32 = 262_144u32;
```

*Defined in [`object-0.37.3/src/macho.rs:904`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L904)*

When this bit is set, the binary declares it is safe for use in processes with uid zero

### `MH_SETUID_SAFE`
```rust
const MH_SETUID_SAFE: u32 = 524_288u32;
```

*Defined in [`object-0.37.3/src/macho.rs:906`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L906)*

When this bit is set, the binary declares it is safe for use in processes when issetugid() is true

### `MH_NO_REEXPORTED_DYLIBS`
```rust
const MH_NO_REEXPORTED_DYLIBS: u32 = 1_048_576u32;
```

*Defined in [`object-0.37.3/src/macho.rs:908`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L908)*

When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported

### `MH_PIE`
```rust
const MH_PIE: u32 = 2_097_152u32;
```

*Defined in [`object-0.37.3/src/macho.rs:910`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L910)*

When this bit is set, the OS will load the main executable at a random address.  Only used in MH_EXECUTE filetypes.

### `MH_DEAD_STRIPPABLE_DYLIB`
```rust
const MH_DEAD_STRIPPABLE_DYLIB: u32 = 4_194_304u32;
```

*Defined in [`object-0.37.3/src/macho.rs:912`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L912)*

Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being referenced from the dylib.

### `MH_HAS_TLV_DESCRIPTORS`
```rust
const MH_HAS_TLV_DESCRIPTORS: u32 = 8_388_608u32;
```

*Defined in [`object-0.37.3/src/macho.rs:914`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L914)*

Contains a section of type S_THREAD_LOCAL_VARIABLES

### `MH_NO_HEAP_EXECUTION`
```rust
const MH_NO_HEAP_EXECUTION: u32 = 16_777_216u32;
```

*Defined in [`object-0.37.3/src/macho.rs:916`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L916)*

When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes.

### `MH_APP_EXTENSION_SAFE`
```rust
const MH_APP_EXTENSION_SAFE: u32 = 33_554_432u32;
```

*Defined in [`object-0.37.3/src/macho.rs:918`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L918)*

The code was linked for use in an application extension.

### `MH_NLIST_OUTOFSYNC_WITH_DYLDINFO`
```rust
const MH_NLIST_OUTOFSYNC_WITH_DYLDINFO: u32 = 67_108_864u32;
```

*Defined in [`object-0.37.3/src/macho.rs:920`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L920)*

The external symbols listed in the nlist symbol table do not include all the symbols listed in the dyld info.

### `MH_SIM_SUPPORT`
```rust
const MH_SIM_SUPPORT: u32 = 134_217_728u32;
```

*Defined in [`object-0.37.3/src/macho.rs:923`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L923)*

Allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with
the platforms macOS, iOSMac, iOSSimulator, tvOSSimulator and watchOSSimulator.

### `MH_DYLIB_IN_CACHE`
```rust
const MH_DYLIB_IN_CACHE: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/macho.rs:926`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L926)*

Only for use on dylibs. When this bit is set, the dylib is part of the dyld
shared cache, rather than loose in the filesystem.

### `LC_REQ_DYLD`
```rust
const LC_REQ_DYLD: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/macho.rs:965`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L965)*

### `LC_SEGMENT`
```rust
const LC_SEGMENT: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:969`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L969)*

segment of this file to be mapped

### `LC_SYMTAB`
```rust
const LC_SYMTAB: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:971`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L971)*

link-edit stab symbol table info

### `LC_SYMSEG`
```rust
const LC_SYMSEG: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:973`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L973)*

link-edit gdb symbol table info (obsolete)

### `LC_THREAD`
```rust
const LC_THREAD: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:975`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L975)*

thread

### `LC_UNIXTHREAD`
```rust
const LC_UNIXTHREAD: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:977`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L977)*

unix thread (includes a stack)

### `LC_LOADFVMLIB`
```rust
const LC_LOADFVMLIB: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:979`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L979)*

load a specified fixed VM shared library

### `LC_IDFVMLIB`
```rust
const LC_IDFVMLIB: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:981`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L981)*

fixed VM shared library identification

### `LC_IDENT`
```rust
const LC_IDENT: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:983`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L983)*

object identification info (obsolete)

### `LC_FVMFILE`
```rust
const LC_FVMFILE: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/macho.rs:985`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L985)*

fixed VM file inclusion (internal use)

### `LC_PREPAGE`
```rust
const LC_PREPAGE: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:987`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L987)*

prepage command (internal use)

### `LC_DYSYMTAB`
```rust
const LC_DYSYMTAB: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:989`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L989)*

dynamic link-edit symbol table info

### `LC_LOAD_DYLIB`
```rust
const LC_LOAD_DYLIB: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/macho.rs:991`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L991)*

load a dynamically linked shared library

### `LC_ID_DYLIB`
```rust
const LC_ID_DYLIB: u32 = 13u32;
```

*Defined in [`object-0.37.3/src/macho.rs:993`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L993)*

dynamically linked shared lib ident

### `LC_LOAD_DYLINKER`
```rust
const LC_LOAD_DYLINKER: u32 = 14u32;
```

*Defined in [`object-0.37.3/src/macho.rs:995`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L995)*

load a dynamic linker

### `LC_ID_DYLINKER`
```rust
const LC_ID_DYLINKER: u32 = 15u32;
```

*Defined in [`object-0.37.3/src/macho.rs:997`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L997)*

dynamic linker identification

### `LC_PREBOUND_DYLIB`
```rust
const LC_PREBOUND_DYLIB: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/macho.rs:999`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L999)*

modules prebound for a dynamically linked shared library

### `LC_ROUTINES`
```rust
const LC_ROUTINES: u32 = 17u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1001`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1001)*

image routines

### `LC_SUB_FRAMEWORK`
```rust
const LC_SUB_FRAMEWORK: u32 = 18u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1003`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1003)*

sub framework

### `LC_SUB_UMBRELLA`
```rust
const LC_SUB_UMBRELLA: u32 = 19u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1005`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1005)*

sub umbrella

### `LC_SUB_CLIENT`
```rust
const LC_SUB_CLIENT: u32 = 20u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1007`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1007)*

sub client

### `LC_SUB_LIBRARY`
```rust
const LC_SUB_LIBRARY: u32 = 21u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1009`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1009)*

sub library

### `LC_TWOLEVEL_HINTS`
```rust
const LC_TWOLEVEL_HINTS: u32 = 22u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1011`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1011)*

two-level namespace lookup hints

### `LC_PREBIND_CKSUM`
```rust
const LC_PREBIND_CKSUM: u32 = 23u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1013`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1013)*

prebind checksum

### `LC_LOAD_WEAK_DYLIB`
```rust
const LC_LOAD_WEAK_DYLIB: u32 = 2_147_483_672u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1016`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1016)*

load a dynamically linked shared library that is allowed to be missing
(all symbols are weak imported).

### `LC_SEGMENT_64`
```rust
const LC_SEGMENT_64: u32 = 25u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1018`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1018)*

64-bit segment of this file to be mapped

### `LC_ROUTINES_64`
```rust
const LC_ROUTINES_64: u32 = 26u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1020`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1020)*

64-bit image routines

### `LC_UUID`
```rust
const LC_UUID: u32 = 27u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1022`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1022)*

the uuid

### `LC_RPATH`
```rust
const LC_RPATH: u32 = 2_147_483_676u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1024`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1024)*

runpath additions

### `LC_CODE_SIGNATURE`
```rust
const LC_CODE_SIGNATURE: u32 = 29u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1026`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1026)*

local of code signature

### `LC_SEGMENT_SPLIT_INFO`
```rust
const LC_SEGMENT_SPLIT_INFO: u32 = 30u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1028`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1028)*

local of info to split segments

### `LC_REEXPORT_DYLIB`
```rust
const LC_REEXPORT_DYLIB: u32 = 2_147_483_679u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1030`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1030)*

load and re-export dylib

### `LC_LAZY_LOAD_DYLIB`
```rust
const LC_LAZY_LOAD_DYLIB: u32 = 32u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1032`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1032)*

delay load of dylib until first use

### `LC_ENCRYPTION_INFO`
```rust
const LC_ENCRYPTION_INFO: u32 = 33u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1034`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1034)*

encrypted segment information

### `LC_DYLD_INFO`
```rust
const LC_DYLD_INFO: u32 = 34u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1036`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1036)*

compressed dyld information

### `LC_DYLD_INFO_ONLY`
```rust
const LC_DYLD_INFO_ONLY: u32 = 2_147_483_682u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1038`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1038)*

compressed dyld information only

### `LC_LOAD_UPWARD_DYLIB`
```rust
const LC_LOAD_UPWARD_DYLIB: u32 = 2_147_483_683u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1040`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1040)*

load upward dylib

### `LC_VERSION_MIN_MACOSX`
```rust
const LC_VERSION_MIN_MACOSX: u32 = 36u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1042`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1042)*

build for MacOSX min OS version

### `LC_VERSION_MIN_IPHONEOS`
```rust
const LC_VERSION_MIN_IPHONEOS: u32 = 37u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1044`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1044)*

build for iPhoneOS min OS version

### `LC_FUNCTION_STARTS`
```rust
const LC_FUNCTION_STARTS: u32 = 38u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1046`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1046)*

compressed table of function start addresses

### `LC_DYLD_ENVIRONMENT`
```rust
const LC_DYLD_ENVIRONMENT: u32 = 39u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1048`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1048)*

string for dyld to treat like environment variable

### `LC_MAIN`
```rust
const LC_MAIN: u32 = 2_147_483_688u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1050`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1050)*

replacement for LC_UNIXTHREAD

### `LC_DATA_IN_CODE`
```rust
const LC_DATA_IN_CODE: u32 = 41u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1052`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1052)*

table of non-instructions in __text

### `LC_SOURCE_VERSION`
```rust
const LC_SOURCE_VERSION: u32 = 42u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1054`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1054)*

source version used to build binary

### `LC_DYLIB_CODE_SIGN_DRS`
```rust
const LC_DYLIB_CODE_SIGN_DRS: u32 = 43u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1056`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1056)*

Code signing DRs copied from linked dylibs

### `LC_ENCRYPTION_INFO_64`
```rust
const LC_ENCRYPTION_INFO_64: u32 = 44u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1058`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1058)*

64-bit encrypted segment information

### `LC_LINKER_OPTION`
```rust
const LC_LINKER_OPTION: u32 = 45u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1060`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1060)*

linker options in MH_OBJECT files

### `LC_LINKER_OPTIMIZATION_HINT`
```rust
const LC_LINKER_OPTIMIZATION_HINT: u32 = 46u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1062`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1062)*

optimization hints in MH_OBJECT files

### `LC_VERSION_MIN_TVOS`
```rust
const LC_VERSION_MIN_TVOS: u32 = 47u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1064`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1064)*

build for AppleTV min OS version

### `LC_VERSION_MIN_WATCHOS`
```rust
const LC_VERSION_MIN_WATCHOS: u32 = 48u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1066`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1066)*

build for Watch min OS version

### `LC_NOTE`
```rust
const LC_NOTE: u32 = 49u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1068`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1068)*

arbitrary data included within a Mach-O file

### `LC_BUILD_VERSION`
```rust
const LC_BUILD_VERSION: u32 = 50u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1070`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1070)*

build for platform min OS version

### `LC_DYLD_EXPORTS_TRIE`
```rust
const LC_DYLD_EXPORTS_TRIE: u32 = 2_147_483_699u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1072`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1072)*

used with `LinkeditDataCommand`, payload is trie

### `LC_DYLD_CHAINED_FIXUPS`
```rust
const LC_DYLD_CHAINED_FIXUPS: u32 = 2_147_483_700u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1074`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1074)*

used with `LinkeditDataCommand`

### `LC_FILESET_ENTRY`
```rust
const LC_FILESET_ENTRY: u32 = 2_147_483_701u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1076`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1076)*

used with `FilesetEntryCommand`

### `SG_HIGHVM`
```rust
const SG_HIGHVM: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1166`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1166)*

the file contents for this segment is for the high part of the VM space, the low part is zero filled (for stacks in core files)

### `SG_FVMLIB`
```rust
const SG_FVMLIB: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1168`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1168)*

this segment is the VM that is allocated by a fixed VM library, for overlap checking in the link editor

### `SG_NORELOC`
```rust
const SG_NORELOC: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1170`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1170)*

this segment has nothing that was relocated in it and nothing relocated to it, that is it maybe safely replaced without relocation

### `SG_PROTECTED_VERSION_1`
```rust
const SG_PROTECTED_VERSION_1: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1172`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1172)*

This segment is protected.  If the segment starts at file offset 0, the first page of the segment is not protected.  All other pages of the segment are protected.

### `SG_READ_ONLY`
```rust
const SG_READ_ONLY: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1174`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1174)*

This segment is made read-only after fixups

### `SECTION_TYPE`
```rust
const SECTION_TYPE: u32 = 255u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1268`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1268)*

256 section types

### `SECTION_ATTRIBUTES`
```rust
const SECTION_ATTRIBUTES: u32 = 4_294_967_040u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1270`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1270)*

24 section attributes

### `S_REGULAR`
```rust
const S_REGULAR: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1274`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1274)*

regular section

### `S_ZEROFILL`
```rust
const S_ZEROFILL: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1276`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1276)*

zero fill on demand section

### `S_CSTRING_LITERALS`
```rust
const S_CSTRING_LITERALS: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1278`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1278)*

section with only literal C strings

### `S_4BYTE_LITERALS`
```rust
const S_4BYTE_LITERALS: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1280`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1280)*

section with only 4 byte literals

### `S_8BYTE_LITERALS`
```rust
const S_8BYTE_LITERALS: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1282`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1282)*

section with only 8 byte literals

### `S_LITERAL_POINTERS`
```rust
const S_LITERAL_POINTERS: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1284`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1284)*

section with only pointers to literals

### `S_NON_LAZY_SYMBOL_POINTERS`
```rust
const S_NON_LAZY_SYMBOL_POINTERS: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1298`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1298)*

section with only non-lazy symbol pointers

### `S_LAZY_SYMBOL_POINTERS`
```rust
const S_LAZY_SYMBOL_POINTERS: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1300`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1300)*

section with only lazy symbol pointers

### `S_SYMBOL_STUBS`
```rust
const S_SYMBOL_STUBS: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1302`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1302)*

section with only symbol stubs, byte size of stub in the reserved2 field

### `S_MOD_INIT_FUNC_POINTERS`
```rust
const S_MOD_INIT_FUNC_POINTERS: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1304`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1304)*

section with only function pointers for initialization

### `S_MOD_TERM_FUNC_POINTERS`
```rust
const S_MOD_TERM_FUNC_POINTERS: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1306`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1306)*

section with only function pointers for termination

### `S_COALESCED`
```rust
const S_COALESCED: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1308`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1308)*

section contains symbols that are to be coalesced

### `S_GB_ZEROFILL`
```rust
const S_GB_ZEROFILL: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1310`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1310)*

zero fill on demand section (that can be larger than 4 gigabytes)

### `S_INTERPOSING`
```rust
const S_INTERPOSING: u32 = 13u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1312`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1312)*

section with only pairs of function pointers for interposing

### `S_16BYTE_LITERALS`
```rust
const S_16BYTE_LITERALS: u32 = 14u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1314`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1314)*

section with only 16 byte literals

### `S_DTRACE_DOF`
```rust
const S_DTRACE_DOF: u32 = 15u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1316`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1316)*

section contains DTrace Object Format

### `S_LAZY_DYLIB_SYMBOL_POINTERS`
```rust
const S_LAZY_DYLIB_SYMBOL_POINTERS: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1318`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1318)*

section with only lazy symbol pointers to lazy loaded dylibs

### `S_THREAD_LOCAL_REGULAR`
```rust
const S_THREAD_LOCAL_REGULAR: u32 = 17u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1323`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1323)*

template of initial values for TLVs

### `S_THREAD_LOCAL_ZEROFILL`
```rust
const S_THREAD_LOCAL_ZEROFILL: u32 = 18u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1325`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1325)*

template of initial values for TLVs

### `S_THREAD_LOCAL_VARIABLES`
```rust
const S_THREAD_LOCAL_VARIABLES: u32 = 19u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1327`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1327)*

TLV descriptors

### `S_THREAD_LOCAL_VARIABLE_POINTERS`
```rust
const S_THREAD_LOCAL_VARIABLE_POINTERS: u32 = 20u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1329`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1329)*

pointers to TLV descriptors

### `S_THREAD_LOCAL_INIT_FUNCTION_POINTERS`
```rust
const S_THREAD_LOCAL_INIT_FUNCTION_POINTERS: u32 = 21u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1331`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1331)*

functions to call to initialize TLV values

### `S_INIT_FUNC_OFFSETS`
```rust
const S_INIT_FUNC_OFFSETS: u32 = 22u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1333`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1333)*

32-bit offsets to initializers

### `SECTION_ATTRIBUTES_USR`
```rust
const SECTION_ATTRIBUTES_USR: u32 = 4_278_190_080u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1340`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1340)*

User setable attributes

### `S_ATTR_PURE_INSTRUCTIONS`
```rust
const S_ATTR_PURE_INSTRUCTIONS: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1342`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1342)*

section contains only true machine instructions

### `S_ATTR_NO_TOC`
```rust
const S_ATTR_NO_TOC: u32 = 1_073_741_824u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1344`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1344)*

section contains coalesced symbols that are not to be in a ranlib table of contents

### `S_ATTR_STRIP_STATIC_SYMS`
```rust
const S_ATTR_STRIP_STATIC_SYMS: u32 = 536_870_912u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1346`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1346)*

ok to strip static symbols in this section in files with the MH_DYLDLINK flag

### `S_ATTR_NO_DEAD_STRIP`
```rust
const S_ATTR_NO_DEAD_STRIP: u32 = 268_435_456u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1348`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1348)*

no dead stripping

### `S_ATTR_LIVE_SUPPORT`
```rust
const S_ATTR_LIVE_SUPPORT: u32 = 134_217_728u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1350`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1350)*

blocks are live if they reference live blocks

### `S_ATTR_SELF_MODIFYING_CODE`
```rust
const S_ATTR_SELF_MODIFYING_CODE: u32 = 67_108_864u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1352`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1352)*

Used with i386 code stubs written on by dyld

### `S_ATTR_DEBUG`
```rust
const S_ATTR_DEBUG: u32 = 33_554_432u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1363`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1363)*

a debug section

### `SECTION_ATTRIBUTES_SYS`
```rust
const SECTION_ATTRIBUTES_SYS: u32 = 16_776_960u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1365`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1365)*

system setable attributes

### `S_ATTR_SOME_INSTRUCTIONS`
```rust
const S_ATTR_SOME_INSTRUCTIONS: u32 = 1_024u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1367`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1367)*

section contains some machine instructions

### `S_ATTR_EXT_RELOC`
```rust
const S_ATTR_EXT_RELOC: u32 = 512u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1369`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1369)*

section has external relocation entries

### `S_ATTR_LOC_RELOC`
```rust
const S_ATTR_LOC_RELOC: u32 = 256u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1371`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1371)*

section has local relocation entries

### `SEG_PAGEZERO`
```rust
const SEG_PAGEZERO: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1390`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1390)*

the pagezero segment which has no protections and catches NULL references for MH_EXECUTE files

### `SEG_TEXT`
```rust
const SEG_TEXT: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1393`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1393)*

the tradition UNIX text segment

### `SECT_TEXT`
```rust
const SECT_TEXT: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1395`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1395)*

the real text part of the text section no headers, and no padding

### `SECT_FVMLIB_INIT0`
```rust
const SECT_FVMLIB_INIT0: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1397`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1397)*

the fvmlib initialization section

### `SECT_FVMLIB_INIT1`
```rust
const SECT_FVMLIB_INIT1: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1399`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1399)*

the section following the fvmlib initialization section

### `SEG_DATA`
```rust
const SEG_DATA: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1402`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1402)*

the tradition UNIX data segment

### `SECT_DATA`
```rust
const SECT_DATA: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1404`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1404)*

the real initialized data section no padding, no bss overlap

### `SECT_BSS`
```rust
const SECT_BSS: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1406`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1406)*

the real uninitialized data section no padding

### `SECT_COMMON`
```rust
const SECT_COMMON: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1408`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1408)*

the section common symbols are allocated in by the link editor

### `SEG_OBJC`
```rust
const SEG_OBJC: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1411`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1411)*

objective-C runtime segment

### `SECT_OBJC_SYMBOLS`
```rust
const SECT_OBJC_SYMBOLS: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1413`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1413)*

symbol table

### `SECT_OBJC_MODULES`
```rust
const SECT_OBJC_MODULES: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1415`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1415)*

module information

### `SECT_OBJC_STRINGS`
```rust
const SECT_OBJC_STRINGS: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1417`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1417)*

string table

### `SECT_OBJC_REFS`
```rust
const SECT_OBJC_REFS: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1419`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1419)*

string table

### `SEG_ICON`
```rust
const SEG_ICON: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1422`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1422)*

the icon segment

### `SECT_ICON_HEADER`
```rust
const SECT_ICON_HEADER: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1424`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1424)*

the icon headers

### `SECT_ICON_TIFF`
```rust
const SECT_ICON_TIFF: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1426`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1426)*

the icons in tiff format

### `SEG_LINKEDIT`
```rust
const SEG_LINKEDIT: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1429`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1429)*

the segment containing all structs created and maintained by the link editor.  Created with -seglinkedit option to ld(1) for MH_EXECUTE and FVMLIB file types only

### `SEG_LINKINFO`
```rust
const SEG_LINKINFO: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1432`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1432)*

the segment overlapping with linkedit containing linking information

### `SEG_UNIXSTACK`
```rust
const SEG_UNIXSTACK: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1435`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1435)*

the unix stack segment

### `SEG_IMPORT`
```rust
const SEG_IMPORT: &str;
```

*Defined in [`object-0.37.3/src/macho.rs:1438`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1438)*

the segment for the self (dyld) modifying code stubs that has read, write and execute permissions

### `INDIRECT_SYMBOL_LOCAL`
```rust
const INDIRECT_SYMBOL_LOCAL: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1940`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1940)*

### `INDIRECT_SYMBOL_ABS`
```rust
const INDIRECT_SYMBOL_ABS: u32 = 1_073_741_824u32;
```

*Defined in [`object-0.37.3/src/macho.rs:1941`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L1941)*

### `PLATFORM_MACOS`
```rust
const PLATFORM_MACOS: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2263`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2263)*

### `PLATFORM_IOS`
```rust
const PLATFORM_IOS: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2264`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2264)*

### `PLATFORM_TVOS`
```rust
const PLATFORM_TVOS: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2265`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2265)*

### `PLATFORM_WATCHOS`
```rust
const PLATFORM_WATCHOS: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2266`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2266)*

### `PLATFORM_BRIDGEOS`
```rust
const PLATFORM_BRIDGEOS: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2267`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2267)*

### `PLATFORM_MACCATALYST`
```rust
const PLATFORM_MACCATALYST: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2268`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2268)*

### `PLATFORM_IOSSIMULATOR`
```rust
const PLATFORM_IOSSIMULATOR: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2269`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2269)*

### `PLATFORM_TVOSSIMULATOR`
```rust
const PLATFORM_TVOSSIMULATOR: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2270`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2270)*

### `PLATFORM_WATCHOSSIMULATOR`
```rust
const PLATFORM_WATCHOSSIMULATOR: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2271`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2271)*

### `PLATFORM_DRIVERKIT`
```rust
const PLATFORM_DRIVERKIT: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2272`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2272)*

### `PLATFORM_XROS`
```rust
const PLATFORM_XROS: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2273`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2273)*

### `PLATFORM_XROSSIMULATOR`
```rust
const PLATFORM_XROSSIMULATOR: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2274`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2274)*

### `TOOL_CLANG`
```rust
const TOOL_CLANG: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2277`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2277)*

### `TOOL_SWIFT`
```rust
const TOOL_SWIFT: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2278`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2278)*

### `TOOL_LD`
```rust
const TOOL_LD: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2279`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2279)*

### `REBASE_TYPE_POINTER`
```rust
const REBASE_TYPE_POINTER: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2409`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2409)*

### `REBASE_TYPE_TEXT_ABSOLUTE32`
```rust
const REBASE_TYPE_TEXT_ABSOLUTE32: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2410`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2410)*

### `REBASE_TYPE_TEXT_PCREL32`
```rust
const REBASE_TYPE_TEXT_PCREL32: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2411`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2411)*

### `REBASE_OPCODE_MASK`
```rust
const REBASE_OPCODE_MASK: u8 = 240u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2413`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2413)*

### `REBASE_IMMEDIATE_MASK`
```rust
const REBASE_IMMEDIATE_MASK: u8 = 15u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2414`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2414)*

### `REBASE_OPCODE_DONE`
```rust
const REBASE_OPCODE_DONE: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2415`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2415)*

### `REBASE_OPCODE_SET_TYPE_IMM`
```rust
const REBASE_OPCODE_SET_TYPE_IMM: u8 = 16u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2416`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2416)*

### `REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`
```rust
const REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB: u8 = 32u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2417`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2417)*

### `REBASE_OPCODE_ADD_ADDR_ULEB`
```rust
const REBASE_OPCODE_ADD_ADDR_ULEB: u8 = 48u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2418`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2418)*

### `REBASE_OPCODE_ADD_ADDR_IMM_SCALED`
```rust
const REBASE_OPCODE_ADD_ADDR_IMM_SCALED: u8 = 64u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2419`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2419)*

### `REBASE_OPCODE_DO_REBASE_IMM_TIMES`
```rust
const REBASE_OPCODE_DO_REBASE_IMM_TIMES: u8 = 80u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2420`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2420)*

### `REBASE_OPCODE_DO_REBASE_ULEB_TIMES`
```rust
const REBASE_OPCODE_DO_REBASE_ULEB_TIMES: u8 = 96u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2421`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2421)*

### `REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB`
```rust
const REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB: u8 = 112u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2422`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2422)*

### `REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB`
```rust
const REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB: u8 = 128u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2423`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2423)*

### `BIND_TYPE_POINTER`
```rust
const BIND_TYPE_POINTER: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2428`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2428)*

### `BIND_TYPE_TEXT_ABSOLUTE32`
```rust
const BIND_TYPE_TEXT_ABSOLUTE32: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2429`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2429)*

### `BIND_TYPE_TEXT_PCREL32`
```rust
const BIND_TYPE_TEXT_PCREL32: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2430`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2430)*

### `BIND_SPECIAL_DYLIB_SELF`
```rust
const BIND_SPECIAL_DYLIB_SELF: i8 = 0i8;
```

*Defined in [`object-0.37.3/src/macho.rs:2432`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2432)*

### `BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE`
```rust
const BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE: i8 = -1i8;
```

*Defined in [`object-0.37.3/src/macho.rs:2433`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2433)*

### `BIND_SPECIAL_DYLIB_FLAT_LOOKUP`
```rust
const BIND_SPECIAL_DYLIB_FLAT_LOOKUP: i8 = -2i8;
```

*Defined in [`object-0.37.3/src/macho.rs:2434`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2434)*

### `BIND_SPECIAL_DYLIB_WEAK_LOOKUP`
```rust
const BIND_SPECIAL_DYLIB_WEAK_LOOKUP: i8 = -3i8;
```

*Defined in [`object-0.37.3/src/macho.rs:2435`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2435)*

### `BIND_SYMBOL_FLAGS_WEAK_IMPORT`
```rust
const BIND_SYMBOL_FLAGS_WEAK_IMPORT: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2437`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2437)*

### `BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION`
```rust
const BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2438`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2438)*

### `BIND_OPCODE_MASK`
```rust
const BIND_OPCODE_MASK: u8 = 240u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2440`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2440)*

### `BIND_IMMEDIATE_MASK`
```rust
const BIND_IMMEDIATE_MASK: u8 = 15u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2441`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2441)*

### `BIND_OPCODE_DONE`
```rust
const BIND_OPCODE_DONE: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2442`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2442)*

### `BIND_OPCODE_SET_DYLIB_ORDINAL_IMM`
```rust
const BIND_OPCODE_SET_DYLIB_ORDINAL_IMM: u8 = 16u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2443`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2443)*

### `BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB`
```rust
const BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB: u8 = 32u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2444`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2444)*

### `BIND_OPCODE_SET_DYLIB_SPECIAL_IMM`
```rust
const BIND_OPCODE_SET_DYLIB_SPECIAL_IMM: u8 = 48u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2445`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2445)*

### `BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM`
```rust
const BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM: u8 = 64u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2446`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2446)*

### `BIND_OPCODE_SET_TYPE_IMM`
```rust
const BIND_OPCODE_SET_TYPE_IMM: u8 = 80u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2447`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2447)*

### `BIND_OPCODE_SET_ADDEND_SLEB`
```rust
const BIND_OPCODE_SET_ADDEND_SLEB: u8 = 96u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2448`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2448)*

### `BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`
```rust
const BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB: u8 = 112u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2449`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2449)*

### `BIND_OPCODE_ADD_ADDR_ULEB`
```rust
const BIND_OPCODE_ADD_ADDR_ULEB: u8 = 128u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2450`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2450)*

### `BIND_OPCODE_DO_BIND`
```rust
const BIND_OPCODE_DO_BIND: u8 = 144u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2451`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2451)*

### `BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB`
```rust
const BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB: u8 = 160u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2452`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2452)*

### `BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED`
```rust
const BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED: u8 = 176u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2453`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2453)*

### `BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB`
```rust
const BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB: u8 = 192u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2454`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2454)*

### `BIND_OPCODE_THREADED`
```rust
const BIND_OPCODE_THREADED: u8 = 208u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2455`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2455)*

### `BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB`
```rust
const BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2456`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2456)*

### `BIND_SUBOPCODE_THREADED_APPLY`
```rust
const BIND_SUBOPCODE_THREADED_APPLY: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2457`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2457)*

### `EXPORT_SYMBOL_FLAGS_KIND_MASK`
```rust
const EXPORT_SYMBOL_FLAGS_KIND_MASK: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2463`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2463)*

### `EXPORT_SYMBOL_FLAGS_KIND_REGULAR`
```rust
const EXPORT_SYMBOL_FLAGS_KIND_REGULAR: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2464`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2464)*

### `EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL`
```rust
const EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2465`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2465)*

### `EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE`
```rust
const EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2466`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2466)*

### `EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION`
```rust
const EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2467`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2467)*

### `EXPORT_SYMBOL_FLAGS_REEXPORT`
```rust
const EXPORT_SYMBOL_FLAGS_REEXPORT: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2468`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2468)*

### `EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER`
```rust
const EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2469`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2469)*

### `DICE_KIND_DATA`
```rust
const DICE_KIND_DATA: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2591`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2591)*

### `DICE_KIND_JUMP_TABLE8`
```rust
const DICE_KIND_JUMP_TABLE8: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2592`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2592)*

### `DICE_KIND_JUMP_TABLE16`
```rust
const DICE_KIND_JUMP_TABLE16: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2593`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2593)*

### `DICE_KIND_JUMP_TABLE32`
```rust
const DICE_KIND_JUMP_TABLE32: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2594`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2594)*

### `DICE_KIND_ABS_JUMP_TABLE32`
```rust
const DICE_KIND_ABS_JUMP_TABLE32: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/macho.rs:2595`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2595)*

### `N_STAB`
```rust
const N_STAB: u8 = 224u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2683`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2683)*

if any of these bits set, a symbolic debugging entry

### `N_PEXT`
```rust
const N_PEXT: u8 = 16u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2685`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2685)*

private external symbol bit

### `N_TYPE`
```rust
const N_TYPE: u8 = 14u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2687`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2687)*

mask for the type bits

### `N_EXT`
```rust
const N_EXT: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2689`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2689)*

external symbol bit, set for external symbols

### `N_UNDF`
```rust
const N_UNDF: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2702`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2702)*

undefined, n_sect == NO_SECT

### `N_ABS`
```rust
const N_ABS: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2704`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2704)*

absolute, n_sect == NO_SECT

### `N_SECT`
```rust
const N_SECT: u8 = 14u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2706`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2706)*

defined in section number n_sect

### `N_PBUD`
```rust
const N_PBUD: u8 = 12u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2708`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2708)*

prebound undefined (defined in a dylib)

### `N_INDR`
```rust
const N_INDR: u8 = 10u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2710`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2710)*

indirect

### `NO_SECT`
```rust
const NO_SECT: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2732`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2732)*

symbol is not in any section

### `MAX_SECT`
```rust
const MAX_SECT: u8 = 255u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2734`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2734)*

1 thru 255 inclusive

### `REFERENCE_TYPE`
```rust
const REFERENCE_TYPE: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2771`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2771)*

### `REFERENCE_FLAG_UNDEFINED_NON_LAZY`
```rust
const REFERENCE_FLAG_UNDEFINED_NON_LAZY: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2773`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2773)*

### `REFERENCE_FLAG_UNDEFINED_LAZY`
```rust
const REFERENCE_FLAG_UNDEFINED_LAZY: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2774`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2774)*

### `REFERENCE_FLAG_DEFINED`
```rust
const REFERENCE_FLAG_DEFINED: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2775`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2775)*

### `REFERENCE_FLAG_PRIVATE_DEFINED`
```rust
const REFERENCE_FLAG_PRIVATE_DEFINED: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2776`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2776)*

### `REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY`
```rust
const REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2777`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2777)*

### `REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY`
```rust
const REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2778`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2778)*

### `REFERENCED_DYNAMICALLY`
```rust
const REFERENCED_DYNAMICALLY: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2786`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2786)*

### `SELF_LIBRARY_ORDINAL`
```rust
const SELF_LIBRARY_ORDINAL: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2820`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2820)*

### `MAX_LIBRARY_ORDINAL`
```rust
const MAX_LIBRARY_ORDINAL: u8 = 253u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2821`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2821)*

### `DYNAMIC_LOOKUP_ORDINAL`
```rust
const DYNAMIC_LOOKUP_ORDINAL: u8 = 254u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2822`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2822)*

### `EXECUTABLE_ORDINAL`
```rust
const EXECUTABLE_ORDINAL: u8 = 255u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2823`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2823)*

### `N_NO_DEAD_STRIP`
```rust
const N_NO_DEAD_STRIP: u16 = 32u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2836`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2836)*

symbol is not to be dead stripped

### `N_DESC_DISCARDED`
```rust
const N_DESC_DISCARDED: u16 = 32u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2844`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2844)*

symbol is discarded

### `N_WEAK_REF`
```rust
const N_WEAK_REF: u16 = 64u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2852`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2852)*

symbol is weak referenced

### `N_WEAK_DEF`
```rust
const N_WEAK_DEF: u16 = 128u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2861`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2861)*

coalesced symbol is a weak definition

### `N_REF_TO_WEAK`
```rust
const N_REF_TO_WEAK: u16 = 128u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2868`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2868)*

reference to a weak symbol

### `N_ARM_THUMB_DEF`
```rust
const N_ARM_THUMB_DEF: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2875`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2875)*

symbol is a Thumb function (ARM)

### `N_SYMBOL_RESOLVER`
```rust
const N_SYMBOL_RESOLVER: u16 = 256u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2883`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2883)*

### `N_ALT_ENTRY`
```rust
const N_ALT_ENTRY: u16 = 512u16;
```

*Defined in [`object-0.37.3/src/macho.rs:2889`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2889)*

### `N_GSYM`
```rust
const N_GSYM: u8 = 32u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2919`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2919)*

global symbol: name,,NO_SECT,type,0

### `N_FNAME`
```rust
const N_FNAME: u8 = 34u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2921`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2921)*

procedure name (f77 kludge): name,,NO_SECT,0,0

### `N_FUN`
```rust
const N_FUN: u8 = 36u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2923`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2923)*

procedure: name,,n_sect,linenumber,address

### `N_STSYM`
```rust
const N_STSYM: u8 = 38u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2925`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2925)*

static symbol: name,,n_sect,type,address

### `N_LCSYM`
```rust
const N_LCSYM: u8 = 40u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2927`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2927)*

.lcomm symbol: name,,n_sect,type,address

### `N_BNSYM`
```rust
const N_BNSYM: u8 = 46u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2929`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2929)*

begin nsect sym: 0,,n_sect,0,address

### `N_AST`
```rust
const N_AST: u8 = 50u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2931`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2931)*

AST file path: name,,NO_SECT,0,0

### `N_OPT`
```rust
const N_OPT: u8 = 60u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2933`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2933)*

emitted with gcc2_compiled and in gcc source

### `N_RSYM`
```rust
const N_RSYM: u8 = 64u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2935`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2935)*

register sym: name,,NO_SECT,type,register

### `N_SLINE`
```rust
const N_SLINE: u8 = 68u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2937`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2937)*

src line: 0,,n_sect,linenumber,address

### `N_ENSYM`
```rust
const N_ENSYM: u8 = 78u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2939`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2939)*

end nsect sym: 0,,n_sect,0,address

### `N_SSYM`
```rust
const N_SSYM: u8 = 96u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2941`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2941)*

structure elt: name,,NO_SECT,type,struct_offset

### `N_SO`
```rust
const N_SO: u8 = 100u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2943`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2943)*

source file name: name,,n_sect,0,address

### `N_OSO`
```rust
const N_OSO: u8 = 102u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2945`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2945)*

object file name: name,,0,0,st_mtime

### `N_LSYM`
```rust
const N_LSYM: u8 = 128u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2947`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2947)*

local sym: name,,NO_SECT,type,offset

### `N_BINCL`
```rust
const N_BINCL: u8 = 130u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2949`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2949)*

include file beginning: name,,NO_SECT,0,sum

### `N_SOL`
```rust
const N_SOL: u8 = 132u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2951`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2951)*

#included file name: name,,n_sect,0,address

### `N_PARAMS`
```rust
const N_PARAMS: u8 = 134u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2953`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2953)*

compiler parameters: name,,NO_SECT,0,0

### `N_VERSION`
```rust
const N_VERSION: u8 = 136u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2955`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2955)*

compiler version: name,,NO_SECT,0,0

### `N_OLEVEL`
```rust
const N_OLEVEL: u8 = 138u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2957`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2957)*

compiler -O level: name,,NO_SECT,0,0

### `N_PSYM`
```rust
const N_PSYM: u8 = 160u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2959`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2959)*

parameter: name,,NO_SECT,type,offset

### `N_EINCL`
```rust
const N_EINCL: u8 = 162u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2961`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2961)*

include file end: name,,NO_SECT,0,0

### `N_ENTRY`
```rust
const N_ENTRY: u8 = 164u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2963`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2963)*

alternate entry: name,,n_sect,linenumber,address

### `N_LBRAC`
```rust
const N_LBRAC: u8 = 192u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2965`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2965)*

left bracket: 0,,NO_SECT,nesting level,address

### `N_EXCL`
```rust
const N_EXCL: u8 = 194u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2967`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2967)*

deleted include file: name,,NO_SECT,0,sum

### `N_RBRAC`
```rust
const N_RBRAC: u8 = 224u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2969`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2969)*

right bracket: 0,,NO_SECT,nesting level,address

### `N_BCOMM`
```rust
const N_BCOMM: u8 = 226u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2971`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2971)*

begin common: name,,NO_SECT,0,0

### `N_ECOMM`
```rust
const N_ECOMM: u8 = 228u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2973`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2973)*

end common: name,,n_sect,0,0

### `N_ECOML`
```rust
const N_ECOML: u8 = 232u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2975`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2975)*

end common (local name): 0,,n_sect,0,address

### `N_LENG`
```rust
const N_LENG: u8 = 254u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2977`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2977)*

second stab entry with length information

### `N_PC`
```rust
const N_PC: u8 = 48u8;
```

*Defined in [`object-0.37.3/src/macho.rs:2983`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L2983)*

global pascal symbol: name,,NO_SECT,subtype,line

### `R_ABS`
```rust
const R_ABS: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3101`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3101)*

absolute relocation type for Mach-O files

### `R_SCATTERED`
```rust
const R_SCATTERED: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/macho.rs:3167`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3167)*

Bit set in `Relocation::r_word0` for scattered relocations.

### `GENERIC_RELOC_VANILLA`
```rust
const GENERIC_RELOC_VANILLA: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3218`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3218)*

generic relocation as described above

### `GENERIC_RELOC_PAIR`
```rust
const GENERIC_RELOC_PAIR: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3220`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3220)*

Only follows a GENERIC_RELOC_SECTDIFF

### `GENERIC_RELOC_SECTDIFF`
```rust
const GENERIC_RELOC_SECTDIFF: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3221`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3221)*

### `GENERIC_RELOC_PB_LA_PTR`
```rust
const GENERIC_RELOC_PB_LA_PTR: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3223`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3223)*

prebound lazy pointer

### `GENERIC_RELOC_LOCAL_SECTDIFF`
```rust
const GENERIC_RELOC_LOCAL_SECTDIFF: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3224`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3224)*

### `GENERIC_RELOC_TLV`
```rust
const GENERIC_RELOC_TLV: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3226`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3226)*

thread local variables

### `ARM_RELOC_VANILLA`
```rust
const ARM_RELOC_VANILLA: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3239`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3239)*

generic relocation as described above

### `ARM_RELOC_PAIR`
```rust
const ARM_RELOC_PAIR: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3241`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3241)*

the second relocation entry of a pair

### `ARM_RELOC_SECTDIFF`
```rust
const ARM_RELOC_SECTDIFF: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3243`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3243)*

a PAIR follows with subtract symbol value

### `ARM_RELOC_LOCAL_SECTDIFF`
```rust
const ARM_RELOC_LOCAL_SECTDIFF: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3245`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3245)*

like ARM_RELOC_SECTDIFF, but the symbol referenced was local.

### `ARM_RELOC_PB_LA_PTR`
```rust
const ARM_RELOC_PB_LA_PTR: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3247`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3247)*

prebound lazy pointer

### `ARM_RELOC_BR24`
```rust
const ARM_RELOC_BR24: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3249`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3249)*

24 bit branch displacement (to a word address)

### `ARM_THUMB_RELOC_BR22`
```rust
const ARM_THUMB_RELOC_BR22: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3251`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3251)*

22 bit branch displacement (to a half-word address)

### `ARM_THUMB_32BIT_BRANCH`
```rust
const ARM_THUMB_32BIT_BRANCH: u8 = 7u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3253`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3253)*

obsolete - a thumb 32-bit branch instruction possibly needing page-spanning branch workaround

### `ARM_RELOC_HALF`
```rust
const ARM_RELOC_HALF: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3268`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3268)*

### `ARM_RELOC_HALF_SECTDIFF`
```rust
const ARM_RELOC_HALF_SECTDIFF: u8 = 9u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3269`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3269)*

### `ARM64_RELOC_UNSIGNED`
```rust
const ARM64_RELOC_UNSIGNED: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3277`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3277)*

for pointers

### `ARM64_RELOC_SUBTRACTOR`
```rust
const ARM64_RELOC_SUBTRACTOR: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3279`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3279)*

must be followed by a ARM64_RELOC_UNSIGNED

### `ARM64_RELOC_BRANCH26`
```rust
const ARM64_RELOC_BRANCH26: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3281`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3281)*

a B/BL instruction with 26-bit displacement

### `ARM64_RELOC_PAGE21`
```rust
const ARM64_RELOC_PAGE21: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3283`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3283)*

pc-rel distance to page of target

### `ARM64_RELOC_PAGEOFF12`
```rust
const ARM64_RELOC_PAGEOFF12: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3285`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3285)*

offset within page, scaled by r_length

### `ARM64_RELOC_GOT_LOAD_PAGE21`
```rust
const ARM64_RELOC_GOT_LOAD_PAGE21: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3287`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3287)*

pc-rel distance to page of GOT slot

### `ARM64_RELOC_GOT_LOAD_PAGEOFF12`
```rust
const ARM64_RELOC_GOT_LOAD_PAGEOFF12: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3289`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3289)*

offset within page of GOT slot, scaled by r_length

### `ARM64_RELOC_POINTER_TO_GOT`
```rust
const ARM64_RELOC_POINTER_TO_GOT: u8 = 7u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3291`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3291)*

for pointers to GOT slots

### `ARM64_RELOC_TLVP_LOAD_PAGE21`
```rust
const ARM64_RELOC_TLVP_LOAD_PAGE21: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3293`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3293)*

pc-rel distance to page of TLVP slot

### `ARM64_RELOC_TLVP_LOAD_PAGEOFF12`
```rust
const ARM64_RELOC_TLVP_LOAD_PAGEOFF12: u8 = 9u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3295`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3295)*

offset within page of TLVP slot, scaled by r_length

### `ARM64_RELOC_ADDEND`
```rust
const ARM64_RELOC_ADDEND: u8 = 10u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3297`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3297)*

must be followed by PAGE21 or PAGEOFF12

### `ARM64_RELOC_AUTHENTICATED_POINTER`
```rust
const ARM64_RELOC_AUTHENTICATED_POINTER: u8 = 11u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3318`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3318)*

### `PPC_RELOC_VANILLA`
```rust
const PPC_RELOC_VANILLA: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3338`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3338)*

generic relocation as described above

### `PPC_RELOC_PAIR`
```rust
const PPC_RELOC_PAIR: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3340`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3340)*

the second relocation entry of a pair

### `PPC_RELOC_BR14`
```rust
const PPC_RELOC_BR14: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3342`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3342)*

14 bit branch displacement (to a word address)

### `PPC_RELOC_BR24`
```rust
const PPC_RELOC_BR24: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3344`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3344)*

24 bit branch displacement (to a word address)

### `PPC_RELOC_HI16`
```rust
const PPC_RELOC_HI16: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3346`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3346)*

a PAIR follows with the low half

### `PPC_RELOC_LO16`
```rust
const PPC_RELOC_LO16: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3348`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3348)*

a PAIR follows with the high half

### `PPC_RELOC_HA16`
```rust
const PPC_RELOC_HA16: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3352`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3352)*

Same as the RELOC_HI16 except the low 16 bits and the high 16 bits are added together
with the low 16 bits sign extended first.  This means if bit 15 of the low 16 bits is
set the high 16 bits stored in the instruction will be adjusted.

### `PPC_RELOC_LO14`
```rust
const PPC_RELOC_LO14: u8 = 7u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3355`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3355)*

Same as the LO16 except that the low 2 bits are not stored in the instruction and are
always zero.  This is used in double word load/store instructions.

### `PPC_RELOC_SECTDIFF`
```rust
const PPC_RELOC_SECTDIFF: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3357`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3357)*

a PAIR follows with subtract symbol value

### `PPC_RELOC_PB_LA_PTR`
```rust
const PPC_RELOC_PB_LA_PTR: u8 = 9u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3359`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3359)*

prebound lazy pointer

### `PPC_RELOC_HI16_SECTDIFF`
```rust
const PPC_RELOC_HI16_SECTDIFF: u8 = 10u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3361`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3361)*

section difference forms of above.  a PAIR

### `PPC_RELOC_LO16_SECTDIFF`
```rust
const PPC_RELOC_LO16_SECTDIFF: u8 = 11u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3363`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3363)*

follows these with subtract symbol value

### `PPC_RELOC_HA16_SECTDIFF`
```rust
const PPC_RELOC_HA16_SECTDIFF: u8 = 12u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3364`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3364)*

### `PPC_RELOC_JBSR`
```rust
const PPC_RELOC_JBSR: u8 = 13u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3365`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3365)*

### `PPC_RELOC_LO14_SECTDIFF`
```rust
const PPC_RELOC_LO14_SECTDIFF: u8 = 14u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3366`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3366)*

### `PPC_RELOC_LOCAL_SECTDIFF`
```rust
const PPC_RELOC_LOCAL_SECTDIFF: u8 = 15u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3368`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3368)*

like PPC_RELOC_SECTDIFF, but the symbol referenced was local.

### `X86_64_RELOC_UNSIGNED`
```rust
const X86_64_RELOC_UNSIGNED: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3523`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3523)*

for absolute addresses

### `X86_64_RELOC_SIGNED`
```rust
const X86_64_RELOC_SIGNED: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3525`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3525)*

for signed 32-bit displacement

### `X86_64_RELOC_BRANCH`
```rust
const X86_64_RELOC_BRANCH: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3527`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3527)*

a CALL/JMP instruction with 32-bit displacement

### `X86_64_RELOC_GOT_LOAD`
```rust
const X86_64_RELOC_GOT_LOAD: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3529`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3529)*

a MOVQ load of a GOT entry

### `X86_64_RELOC_GOT`
```rust
const X86_64_RELOC_GOT: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3531`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3531)*

other GOT references

### `X86_64_RELOC_SUBTRACTOR`
```rust
const X86_64_RELOC_SUBTRACTOR: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3533`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3533)*

must be followed by a X86_64_RELOC_UNSIGNED

### `X86_64_RELOC_SIGNED_1`
```rust
const X86_64_RELOC_SIGNED_1: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3535`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3535)*

for signed 32-bit displacement with a -1 addend

### `X86_64_RELOC_SIGNED_2`
```rust
const X86_64_RELOC_SIGNED_2: u8 = 7u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3537`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3537)*

for signed 32-bit displacement with a -2 addend

### `X86_64_RELOC_SIGNED_4`
```rust
const X86_64_RELOC_SIGNED_4: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3539`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3539)*

for signed 32-bit displacement with a -4 addend

### `X86_64_RELOC_TLV`
```rust
const X86_64_RELOC_TLV: u8 = 9u8;
```

*Defined in [`object-0.37.3/src/macho.rs:3541`](../../../.source_1765633015/object-0.37.3/src/macho.rs#L3541)*

for thread local variables

